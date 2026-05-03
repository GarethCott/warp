//! `warp harness-support` CLI dispatch and the singleton model all subcommands run async work on.
//!
//! Subcommands:
//! - [`ping`] — fetches the current run by task ID and prints its info.
//! - [`report_artifact`] — reports an artifact (e.g. a PR) back to the Oz platform.
//!
//! strip(neuter): The whole module is dead in this fork — `run` is the only
//! public entry point and it returns an error immediately when AgentHarness is
//! gated off. The rest stays for binary-surface stability.
#![allow(dead_code, unused_imports)]
use anyhow::Result;
use warp_cli::agent::OutputFormat;
use warp_cli::harness_support::{
    FinishTaskArgs, HarnessSupportArgs, HarnessSupportCommand, NotifyUserArgs, ReportArtifactArgs,
    ReportArtifactCommand, TaskStatus,
};
use warp_cli::GlobalOptions;
use warp_core::features::FeatureFlag;
use warpui::{platform::TerminationMode, AppContext, ModelHandle, SingletonEntity};

use super::common::set_ambient_task_context_from_run_id;
use crate::ai::ambient_agents::AmbientAgentTaskId;
use crate::ai::artifacts::Artifact;
use crate::server::server_api::ServerApiProvider;

/// Run harness-support commands.
pub fn run(
    _ctx: &mut AppContext,
    _global_options: GlobalOptions,
    _args: HarnessSupportArgs,
) -> Result<()> {
    // strip(neuter): AgentHarness is gated off in this fork.
    Err(anyhow::anyhow!("This feature is not enabled"))
}

/// Fetch the current run by ID and print its info.
fn ping(
    ctx: &mut AppContext,
    runner: ModelHandle<HarnessSupportRunner>,
    task_id: AmbientAgentTaskId,
    output_format: OutputFormat,
) -> Result<()> {
    runner.update(ctx, |_, ctx| {
        let ai_client = ServerApiProvider::as_ref(ctx).get_ai_client();

        ctx.spawn(
            async move {
                let task = ai_client.get_ambient_agent_task(&task_id).await?;
                Ok(task)
            },
            move |_, result, ctx| match result {
                Ok(task) => {
                    match output_format {
                        OutputFormat::Json | OutputFormat::Ndjson => {
                            let json = serde_json::to_string(&task).unwrap_or_else(|e| {
                                serde_json::json!({"error": e.to_string()}).to_string()
                            });
                            println!("{json}");
                        }
                        OutputFormat::Pretty | OutputFormat::Text => {
                            super::ambient::print_tasks(&[task]);
                        }
                    }
                    ctx.terminate_app(TerminationMode::ForceTerminate, None);
                }
                Err(err) => {
                    super::report_fatal_error(err, ctx);
                }
            },
        );
    });

    Ok(())
}

/// Report an artifact back to the Oz platform.
fn report_artifact(
    ctx: &mut AppContext,
    runner: ModelHandle<HarnessSupportRunner>,
    args: ReportArtifactArgs,
    output_format: OutputFormat,
) -> Result<()> {
    runner.update(ctx, |_, ctx| {
        let client = ServerApiProvider::as_ref(ctx).get_harness_support_client();

        let artifact = match args.command {
            ReportArtifactCommand::PullRequest(pr_args) => Artifact::PullRequest {
                url: pr_args.url,
                branch: pr_args.branch,
                repo: None,
                number: None,
            },
        };

        ctx.spawn(
            async move { client.report_artifact(&artifact).await },
            move |_, result, ctx| match result {
                Ok(response) => {
                    match output_format {
                        OutputFormat::Json | OutputFormat::Ndjson => {
                            let json = serde_json::to_string(&response).unwrap_or_else(|e| {
                                serde_json::json!({"error": e.to_string()}).to_string()
                            });
                            println!("{json}");
                        }
                        OutputFormat::Pretty | OutputFormat::Text => {
                            println!("Artifact reported: {}", response.artifact_uid);
                        }
                    }
                    ctx.terminate_app(TerminationMode::ForceTerminate, None);
                }
                Err(err) => {
                    super::report_fatal_error(err, ctx);
                }
            },
        );
    });

    Ok(())
}

/// Send a progress notification to the task's originating platform.
fn notify_user(
    ctx: &mut AppContext,
    runner: ModelHandle<HarnessSupportRunner>,
    args: NotifyUserArgs,
    output_format: OutputFormat,
) -> Result<()> {
    runner.update(ctx, |_, ctx| {
        let client = ServerApiProvider::as_ref(ctx).get_harness_support_client();

        ctx.spawn(
            async move { client.notify_user(&args.message).await },
            move |_, result, ctx| match result {
                Ok(()) => {
                    match output_format {
                        OutputFormat::Json | OutputFormat::Ndjson => {
                            println!("{{}}");
                        }
                        OutputFormat::Pretty | OutputFormat::Text => {
                            println!("Notification sent.");
                        }
                    }
                    ctx.terminate_app(TerminationMode::ForceTerminate, None);
                }
                Err(err) => {
                    super::report_fatal_error(err, ctx);
                }
            },
        );
    });

    Ok(())
}

/// Report task completion or failure.
fn finish_task(
    ctx: &mut AppContext,
    runner: ModelHandle<HarnessSupportRunner>,
    args: FinishTaskArgs,
    output_format: OutputFormat,
) -> Result<()> {
    runner.update(ctx, |_, ctx| {
        let client = ServerApiProvider::as_ref(ctx).get_harness_support_client();

        ctx.spawn(
            async move {
                let success = args.status == TaskStatus::Success;
                client.finish_task(success, &args.summary).await
            },
            move |_, result, ctx| match result {
                Ok(()) => {
                    match output_format {
                        OutputFormat::Json | OutputFormat::Ndjson => {
                            println!("{{}}");
                        }
                        OutputFormat::Pretty | OutputFormat::Text => {
                            println!("Task finished.");
                        }
                    }
                    ctx.terminate_app(TerminationMode::ForceTerminate, None);
                }
                Err(err) => {
                    super::report_fatal_error(err, ctx);
                }
            },
        );
    });

    Ok(())
}

/// Singleton model for running async harness-support operations.
struct HarnessSupportRunner;

impl warpui::Entity for HarnessSupportRunner {
    type Event = ();
}

impl SingletonEntity for HarnessSupportRunner {}
