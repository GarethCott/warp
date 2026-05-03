use warpui::{AppContext, EntityId, SingletonEntity};

use crate::ai::agent::conversation::AIConversationId;
use crate::ai::agent_conversations_model::AgentConversationsModel;
use crate::ai::blocklist::history_model::BlocklistAIHistoryModel;

/// Delete a conversation from the blocklist, local storage, and the cloud.
pub fn delete_conversation(
    conversation_id: AIConversationId,
    terminal_view_id: Option<EntityId>,
    ctx: &mut AppContext,
) {
    let server_conversation_token = get_server_conversation_token(&conversation_id, ctx);

    BlocklistAIHistoryModel::handle(ctx).update(ctx, |history, model_ctx| {
        history.delete_conversation(conversation_id, terminal_view_id, model_ctx);

        if server_conversation_token.is_none() {
            log::info!(
                "No server conversation token found for conversation to delete: {conversation_id}"
            );
        }
    });

    // Make sure the agent conversations model is up to date.
    AgentConversationsModel::handle(ctx).update(ctx, |model, ctx| {
        model.sync_conversations(ctx);
    });
}

/// Remove a conversation from the blocklist and optionally from the cloud.
///
/// This is similar to `delete_conversation`, but it is only used for ephemeral/empty conversations
/// (i.e. conversations from closed empty agent mode views, or passive code diffs).
pub fn remove_conversation(
    conversation_id: AIConversationId,
    terminal_view_id: EntityId,
    _delete_from_cloud: bool,
    ctx: &mut AppContext,
) {
    BlocklistAIHistoryModel::handle(ctx).update(ctx, |history, model_ctx| {
        history.remove_conversation(conversation_id, terminal_view_id, model_ctx);
    });
}

/// Get the server conversation token for a historical or live conversation.
fn get_server_conversation_token(
    conversation_id: &AIConversationId,
    ctx: &AppContext,
) -> Option<String> {
    let history = BlocklistAIHistoryModel::as_ref(ctx);

    history
        .conversation(conversation_id)
        .and_then(|c| {
            c.server_conversation_token()
                .map(|t| t.as_str().to_string())
        })
        .or_else(|| {
            history
                .get_conversation_metadata(conversation_id)
                .and_then(|m| m.server_conversation_token.as_ref())
                .map(|t| t.as_str().to_string())
        })
}
