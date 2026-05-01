# Stripping progress / handoff

This doc is a snapshot of what's been done and what's left, designed so a fresh Claude session can pick up cold. Last updated 2026-05-01 (post-cleanup-53).

---

## Goal

Personal warp fork (`GarethCott/warp`) that boots straight to a terminal with **no agent, no auth, no cloud, no telemetry**, side-by-side with the official `warp-oss` install. Approach is **"neuter, not strip"** primarily, with iterative cleanup of dead code.

## What's done

### Phase A — Neuter (PR #1, merged)
- `http_client::Client::execute` blocks any request to `*.warp.dev` / `*.warpdotdev.com` with a fake 503
- `skip_login` enabled by default → boots authenticated as a local Test user
- Telemetry sender (`send_batch_messages_to_rudder`) returns `Ok(())` immediately
- AI / Drive top menus removed
- Settings sidebar trimmed to 8 non-cloud entries
- Right panel hidden (`has_right_region = false`)
- `is_any_ai_enabled()` returns `false` unconditionally (gates ~265 UI sites)
- Command palette filters out the `WarpAi` binding group + Drive + conversation sources
- README, NOTICE, CHANGELOG rewritten for fork identity + AGPL §5 compliance

### Phase B — Tier 1 + Tier 3 strip (PRs #2, #3, merged)
- Deleted leaf crates: `voice_input`, `warp_web_event_bus`, `managed_secrets_wasm`, `firebase`
- Inlined two firebase types into `app/src/auth/firebase_types.rs` (bridge for Tier 4 cleanup later)

### Phase C — Tier 4 attempt (abandoned)
- Tried bulk-deleting agent crates + `app/src/ai/` subtree
- Hit ~1000 compile errors that didn't converge
- **Lesson:** the agent code is interleaved with core terminal logic in `app/src/lib.rs`, `app/src/terminal/view.rs`, `app/src/workspace/view.rs`, etc. Bulk delete is not viable.
- Branch `strip-tier-4` was deleted from both local and remote.

### Phase D — Iterative cleanup (PRs #4 through #32, all merged)
- Removed all dead `voice_input` cfg blocks from non-test, non-agent-tree code
- Deleted entire voice transcriber subsystem (3 files)
- Deleted `VoiceWidget` from settings UI
- Removed 18 orphan feature definitions from `app/Cargo.toml`
- Replaced ~70 `is_any_ai_enabled()` callsites with the constant `false` across ~12 files
- Deleted dead `make_new_ai_menu` / `make_new_drive_menu` builders in `app_menus.rs`
- Trimmed agent/cloud features from `[default]` features list
- Dropped `ai_enabled: bool` plumbing from `right_panel.rs` review-terminal helpers (cleanup-23)
- Removed dead `else if false` agent-tutorial branch in `root_view.rs` (cleanup-24)
- Removed dead `let _ = WarpDriveSettings/AISettings` neuter suppressions + their imports in command palette (cleanup-25)
- Collapsed `EditorView::render_controls` after AI/voice paths went dead, deleted `render_at_context_menu_button`, dropped `at_context_menu_button_mouse_handle` field (cleanup-26: -86 net lines)
- Deleted dead Agent + Cloud Oz items in `unified_new_session_menu_items` (cleanup-27)
- Collapsed `Workspace::send_feedback` to URL-only fallback, deleted `is_feedback_skill_available` helper, dropped slash-command + binding-override callers (cleanup-28)
- Deleted entire `TerminalViewZeroStateBlock` subsystem: 410-line file, two enum variants, dead removal loop, dead insertion site (cleanup-29: -481 net lines)
- Collapsed `should_render_use_agent_footer` tail to literal `false` after warpify + CLI-agent live branches (cleanup-30)
- Collapsed AI prompt-history path in up-arrow suggestions, dropped `terminal_view_id` parameter + `include_prompts` field, marked `HistoryInputSuggestion::AIQuery` as `#[allow(dead_code)]` for follow-up (cleanup-31)
- Deleted `TerminalInputMessageBar` subsystem (467-line file, function, field, mod, all imports) (cleanup-32: -518 net lines)
- Deleted dead Ask-AI block toolbelt button (constructor, field, layout/paint/dispatch sites, mouse state) (cleanup-33: -94 net lines)
- Deleted dead "Add as context" selection tooltip in code editor + `Editor::is_selecting` (cleanup-34: -115 net lines)
- Dropped `/update-tab-config` skill button + its full event chain across 7 files (footer action/event, local code editor event, code view event, pane group event, workspace handler) (cleanup-35: -108 net lines)
- Removed dead AI Autofill subsystem from workflow editor: render block, `WorkflowAction::AiAssist`, `issue_request`/`display_upgrade_error`/`populate_missing_field_with_suggestion`/`is_ai_assist_button_disabled` methods, `AiAssistState` enum, fields, mouse-state handles, constants (cleanup-36: -286 net lines)
- Deleted AgentAssisted environment modal subsystem: 774-line modal file + 364-line companion tests file, modal field/handle/open helper in `environments_page.rs`, `OpenAgentAssistedCreateModal` action variant, empty-state "Launch agent" button row, `SettingsPageEvent::AgentAssistedEnvironmentModalToggled` variant + 3 dispatch arms, `pane_with_open_agent_assisted_environment_modal` field + init + cleanup blocks + tab-level render block, the matching arm in environment_management_pane (cleanup-37: -1298 net lines)
- Removed orphan terminal-zero-state + input-message-bar settings: `show_terminal_zero_state_block` field + `should_show_zero_state_block()` getter, `show_terminal_input_message_bar` field + `is_terminal_input_message_bar_enabled()` accessor, `SHOW_TERMINAL_INPUT_MESSAGE_LINE_FLAG` constant + workspace flag insertion, two `FeaturesPageAction` variants + their telemetry/handler arms, two AgentView-gated widget pushes, the `ToggleShowTerminalInputMessageLine` binding pair, and both widget structs/impls (cleanup-38: -188 net lines)
- Dropped 3 dead AI/voice context flag writers in `Workspace::keymap_context` (`IS_ANY_AI_ENABLED`, `IS_ACTIVE_AI_ENABLED`, `IS_VOICE_INPUT_ENABLED`) — all of their gates ultimately reach `is_any_ai_enabled()` which is `false` (cleanup-39: -13 net lines)
- Dropped 7 more dead AI context flag writers in the same fn (`AI_INPUT_AUTODETECTION_FLAG`, `NLD_IN_TERMINAL_FLAG`, `INTELLIGENT_AUTOSUGGESTIONS_FLAG`, `PROMPT_SUGGESTIONS_FLAG`, `CODE_SUGGESTIONS_FLAG`, `NATURAL_LANGUAGE_AUTOSUGGESTIONS_FLAG`, `SHARED_BLOCK_TITLE_GENERATION_FLAG`) (cleanup-40: -27 net lines)
- Removed unreachable `CloudConversationStorageWidget` in privacy_page (struct + impl + push + action variant + handler + helper method) (cleanup-41: -133 net lines)
- Deleted dead `ExecutionProfileView`: 829-line file + the `profile_views` field, `create_profile_views`/`refresh_profile_views` methods, and 3 callsites in `ai_page.rs` (cleanup-42: -875 net lines)
- Deleted the entire AI Settings page: 6918-line `ai_page.rs` + 93-line `ai_page_tests.rs`, the `mod ai_page` declaration, every cross-file reference to `AISettingsPageView`/`AISettingsPageAction`/`AISettingsPageEvent`/`AISubpage` (mod.rs + settings_page.rs), the `SettingsPageViewHandle::AI` variant + 2 dispatch arms, the `SettingsAction::AI` variant + dispatch, the entire `handle_ai_page_event` dispatcher, the `cli_agent_settings_widget_id` re-export, the `OpenCodingAgentSettings` action body in agent_input_footer, plus 4 settings_page.rs helper functions only used by ai_page.rs (`render_full_pane_width_ai_button`, `render_custom_size_header`, `render_body_item_label_with_icon`, `render_settings_info_banner`). 4 ai/* subtree items annotated `#[allow(dead_code)]` (cleanup-43: -7286 net lines)
- Deleted unreachable Warp Drive settings page (`warp_drive_page.rs`, 254 lines) + plumbing in mod.rs/settings_page.rs (cleanup-44: -290 net lines)
- Deleted unreachable Platform / OzCloudAPIKeys settings page: `platform_page.rs` (726 lines) + the entire `platform/` subdir (`create_api_key_modal.rs` 731 lines, `expire_api_key_button.rs` 128 lines, mod.rs), plus mod.rs/settings_page.rs plumbing. AuthClient's API-key trait methods annotated `#[allow(dead_code)]` (cleanup-45: -1625 net lines)
- Deleted unreachable Teams settings page: `teams_page.rs` (4097 lines) + 4 companion files (`tab_menu.rs` 58 lines, `transfer_ownership_confirmation_modal.rs` 154 lines, `clickable_text_input.rs` 183 lines, `cloud_action_confirmation_dialog.rs` 225 lines). Removed cascade in mod.rs/settings_page.rs/root_view.rs (2 fns + 2 action registrations)/uri/mod.rs (URI handler)/workspace/view.rs (helper method)/server/telemetry/events.rs (`TeamsInviteOption` import + `ChangedInviteViewOption` variant + 3 dispatch arms). Annotated 12 orphan items in workspaces/server/word_block_editor/ai with `#[allow(dead_code)]` (cleanup-46: -4827 net lines)
- Deleted unreachable Referrals settings page: `referrals_page.rs` (1132 lines) + the 4 entry-point buttons that fed it (settings main page `EarnRewardsWidget`, resource center invite button, user-menu "Invite a friend", macOS app menu "Refer a friend"). Removed `WorkspaceAction::ShowReferralSettingsPage` variant + handler, 2 keybindings, `CustomAction::ReferAFriend` variant + its `is_undoable` arm, and a chain of now-unused imports/constants (`SEND_SVG_PATH`, `REFERRAL_CTA`, `SECTION_SPACING_BOTTOM`) (cleanup-47: -1371 net lines)
- Deleted unreachable BillingAndUsage settings page: `billing_and_usage_page.rs` (3697 lines) + companion subdir (`overage_limit_modal.rs` 357 lines, `usage_history_entry.rs` 225 lines, `usage_history_model.rs` 139 lines), plus `admin_actions.rs` (36 lines) and `tab_selector.rs` (99 lines) which only existed to support the billing page. Extracted `create_discount_badge` into a new 22-line `view_components/discount_badge.rs` so the two terminal modals (`buy_credits_banner`, `enable_auto_reload_modal`) keep compiling. Annotated 3 orphan items (cleanup-48: -4717 net lines)
- Deleted unreachable Shared Blocks settings page: `show_blocks_view.rs` (812 lines) + plumbing in mod.rs/settings_page.rs/workspace/mod.rs/app_menus.rs/util/bindings.rs (cleanup-49: -847 net lines)
- Sweep PR: removed all `#[allow(dead_code)]` annotations introduced by cleanups 43, 45, 46, 48 by deleting the items themselves (`AIExecutionProfilesModel::create_profile`, `total_current_workspace_bonus_credits_remaining`, `refresh_duration_to_string`, `UpdateManager::remove_team_objects`, `WordBlockEditorView::{num_chips, with_validator, add_word}`, `ChipEditorState`, `TeamUpdateManagerEvent` + 6 `TeamUpdateManager` methods, `DisplayMode::Settings` + collapsed irrefutable branches, 6 `BlocklistAIPermissions` methods, `AgentToolbarInlineEditor` + impls + action enum, 4 `AuthClient` trait methods + impls). Cleaned up newly-unused imports across all touched files (cleanup-50: -630 net lines)
- Deleted unreachable MCP Servers settings page: 587-line `mcp_servers_page.rs` + 60-line tests + entire `mcp_servers/` subdir (8 files, 5325 lines: edit_page 951, installation_modal 640, list_page 1826, server_card 1098, update_modal 487, destructive_mcp_confirmation_dialog 193, style 62, mod 68). Removed cascade across mod.rs/settings_page.rs/root_view.rs (open_mcp_settings_in_{new,existing}_window fns + 2 action registrations + OpenMCPSettingsArgs struct + URI handler)/workspace/view.rs (4 dispatch arms + open_mcp_servers_page method + WorkspaceAction::OpenMCPServerCollection)/workspace/mod.rs (binding)/util/bindings.rs (CustomAction)/pane_group/mod.rs (Event variant)/pane_group/pane/terminal_pane.rs (propagation arm)/terminal/view.rs (Event variant + 2 emitters → neuter)/drive/index.rs (DriveIndexAction + DriveIndexEvent variants + 3 dispatch sites)/drive/panel.rs (DrivePanelEvent variant + helper + dispatch)/drive/items/{mcp_server,mcp_server_collection}.rs (click_action → None). Annotated 8 orphan items in ai/mcp/ subtree (cleanup-51: -6217 net lines)
- Partial sweep PR for cleanup-51's annotations: deleted prettify_json + 5 MCP manager helper methods that had no remaining callers; restored 2 gallery methods that turned out to still be needed by templatable_manager/native.rs (cleanup-52: -61 net lines)
- Dropped dead `if false`/`&& false` branches across the codebase (5 in workspace/view.rs, 2 in command_palette/zero_state.rs, 3 in command_search/view.rs, 1 in features_page.rs). Deleted the orphan AI search modules whose only callers were the dead branches: `command_search/warp_ai.rs` (271 lines), `command_search/ai_queries/` subdir (3 files, 258 lines), `search/ai_queries/` subdir (2 files, 25 lines). Deleted the `render_ai_assistant_warm_welcome` method (~95 lines) + its mouse-state field. Dropped the unused `ai_client` field on CommandSearchView + its constructor parameter (cleanup-53: -810 net lines)

**Total stripped: ~36000+ lines of dead code, 45 files entirely deleted, 18 dead feature flags removed.**

### Pending follow-ups
- `HistoryInputSuggestion::AIQuery` variant + 6 match arms in `input_suggestions.rs` can be removed. Blocked on cleaning up test files (`input_suggestions_test.rs`, `input_test.rs`) that still construct the variant. Per the existing convention test files are out of scope, but here removing the variant breaks `cargo test`, so this needs deliberate test surgery.

---

## Current state

- Master is clean: `cargo check --workspace` → zero errors, zero warnings
- App builds & runs: `cargo run --bin warp-oss`
- Bundle ID `dev.warp.WarpOss`, data dir `~/Library/Application Support/dev.warp.WarpOss/` — fully isolated from the official Warp install
- 50+ commits beyond upstream

---

## What's left to strip

### Easy / next tier (hours, not days)

**1. AI Settings UI surgery — DONE in cleanups 42 + 43**
The page (`ai_page.rs`, 6918 lines) and its companion editor (`execution_profile_view.rs`, 829 lines) have been deleted entirely. What remains in the AI cluster is the model layer in `app/src/settings/ai.rs` (~3000 lines, 22 `is_any_ai_enabled` calls). That model is still consumed by code outside ai_page (`workspace/view.rs`, `welcome_palette/view.rs`, `mcp_servers/list_page.rs`, the dead-on-arrival ai/ subtree, etc.), so removing it would cascade widely. Treat it as a Tier-4-style operation, not an iso target.

**2. Remaining `is_any_ai_enabled` callsites**
Inside agent subtree (`app/src/ai/`) and test files. Already gated by `is_any_ai_enabled = false` plus the agent code is dead-on-arrival. Touch only as part of bigger surgery.

**3. Dead `// neuter:` markers and `let _ = ...` suppressions**
Some are now redundant since their gated code was deleted. Low priority cleanup.

**4. AI feature flags still declared in `[features]`**
Many `agent_*`, `ai_*`, `cloud_*`, `mcp_*` features are still declared as `feature_name = []`. They gate code in agent subtree (we keep that as inert dead code). Removing the declarations would create `unexpected_cfg` warnings. Trade-off, not pure win.

### Medium / harder

**5. The agent subtree itself (`app/src/ai/`, `app/src/notebooks/`, `app/src/drive/`)**
This is the original Tier 4 work that we proved doesn't converge in any reasonable session count. The agent is interleaved into:
- `terminal/input.rs` (~600 lines of agent input handling)
- `workspace/view.rs` (agent workspace state)
- `lib.rs` (agent init, agent singletons)
- Plus 30+ files importing types from `app/src/ai/`

Realistic path is **rewrite, not strip** — see "Path C" below.

**6. Cloud auth flow in `app/src/server/server_api/auth.rs` + related**
The Firebase token-refresh flow is dead at runtime (skip_login bypasses it) but still compiled. ~1000 lines. Removing requires cleaning up the `TokenProvider` trait + its callers.

**7. Cloud-flavored crates still present**
- `crates/managed_secrets/` (warp_managed_secrets package)
- `crates/isolation_platform/` (warp_isolation_platform — agent docker sandbox)
- `crates/graphql/` (warp_graphql)
- `crates/warp_graphql_schema/`
- `crates/warp_server_client/`
- `crates/onboarding/`
- `crates/computer_use/`
- `crates/ai/`

These would all be removable in a Tier 4-style operation. Same convergence problem as #5.

### "Path C" alternative (multi-week project, not multi-session)

If you want to truly strip everything: rewrite `app/src/lib.rs` from scratch (~50-100 lines) using the warp crates (`warpui`, `warp_terminal`, `command`, `editor`, etc.) as a library. Skip the existing `app/src/`. Result: minimal binary that opens a window with a PTY and renders blocks.

---

## Workflow / how we work

Every change goes through this loop:

```bash
# 1. Branch off master
git checkout master && git pull origin master
git checkout -b strip-cleanup-NN

# 2. Make edits, verify
cargo check --workspace  # must be clean before committing

# 3. Commit + push
git add -A && git commit -m "strip(cleanup-NN): ..."
git push -u origin strip-cleanup-NN

# 4. PR + rebase merge
gh pr create --repo GarethCott/warp --title "..." --base master --head strip-cleanup-NN --body "..."
gh pr merge --repo GarethCott/warp <branch-name> --rebase --delete-branch

# 5. Sync master
git checkout master && git pull origin master
```

**Never commit directly to master.** Use a branch every time.

`gh repo set-default GarethCott/warp` is set so PRs go to the fork, not upstream.

---

## Common pitfalls

1. **Don't bulk-delete agent crates / app/src/ai.** Hits a wall. Already proven in the abandoned `strip-tier-4` branch.
2. **Some features are required.** `traces` is consumed by `record_trace_event!` macros from `warpui_core`. Don't remove it from `app/Cargo.toml`.
3. **Ignore is_any_ai_enabled refs in tests** (`*_test.rs`, `*_tests.rs`). Tests aren't compiled by default `cargo check`. Don't burn time on them.
4. **Replace, don't delete.** When simplifying a method that has callers, replace its body with `false` / `return Ok(())` rather than deleting the method (which would break callers).

---

## Quick sanity checks for a fresh session

```bash
# verify clean build
cargo check --workspace

# count lines stripped this session vs upstream
git log --oneline upstream/master..master | wc -l

# count remaining is_any_ai_enabled callsites
grep -rn "is_any_ai_enabled" app/src --include="*.rs" | grep -vE "/ai/|_test\.rs|_tests\.rs" | wc -l
```

---

## How to resume

1. Read this file (you're here).
2. Read `CHANGELOG.md` — full record of cleanup PRs.
3. Read `NOTICE.md` — AGPL §5 modification notice.
4. Glance at `README.md` for fork identity.
5. Pick a target from "What's left to strip" above.
6. Follow the workflow loop.

If you want a single concrete next step: all the unreachable settings pages are gone. Remaining iso targets are tougher:
- The reachable pages still in the sidebar (`features_page.rs` ~7000 lines, `appearance_page.rs` 5186 lines, `code_page.rs` 2462 lines, `keybindings.rs`) can only be chipped at by removing AI-only widgets one at a time.
- `environments_page.rs` is still here (3500 lines via `update_environment_form.rs`) but reachable via `EnvironmentManagementPane`, used by app_state persistence schema, agent_input_footer, and root_view — Tier-4 territory.
- The dead-on-arrival `app/src/ai/`, `app/src/notebooks/`, `app/src/drive/` subtrees still won't converge as a Tier-4 strip.
- The cloud crates in `crates/`: managed_secrets, isolation_platform, graphql, warp_graphql_schema, warp_server_client, onboarding, computer_use, ai. Each is wide but tractable.

Smaller wins: search the codebase for `if false`/`&& false`/`if true`/`&& true` for more dead branches; look for `#[allow(dead_code)]` annotations and verify they're still needed; search `is_any_ai_enabled()` callsites outside `/ai/` for any new orphans created by recent strips.

If those feel too risky: there's plenty of small dead-code cleanup left around the codebase. Run `cargo check --workspace 2>&1 | grep "warning"` and fix what comes up. Or grep for `// strip(neuter):` and `let _ = ` and tidy the suppressions.

Whatever you pick: small PRs, rebase-merge, never commit to master.
