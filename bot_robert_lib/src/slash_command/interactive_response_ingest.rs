use crate::slash_command::transfer::{SlackInteractionPayloadTransfer, SlackFormPayloadTransfer, SlackBlockActionsPayloadTransfer};
use crate::slash_command::{SlackPayloadType, SlackBlockActions, SlackResponse};
use crate::slash_command::converter::*;
use std::error::Error;
use crate::slash_command::handlers::command_handler::CommandHandler;
use crate::slash_command::handlers::joke_add_handler::JokeAddHandler;
use crate::slash_command::handlers::fixed_response_handler::FixedResponseHandler;

/// process a slack action. producing a slack response & return url indicating where to send the response
pub fn process_action(response: &SlackFormPayloadTransfer) -> Result<(SlackResponse, Option<String>), Box<dyn Error>>{
    let slack_interactions_payload = serde_json::from_str::<SlackInteractionPayloadTransfer>(&response.payload)?;

    return if slack_interactions_payload.payload_type == SlackPayloadType::BlockActions.to_string() {
        let response = handle_block_actions_response(&response.payload)?;
        Ok(response)
    }
    else {
        Ok((SlackResponse::from_string("Ya... I'm going to need you to repeat that."), None))
    }
}

fn handle_block_actions_response(payload: &String) -> Result<(SlackResponse, Option<String>), Box<dyn Error>>{
    let slack_block_actions_transfer = serde_json::from_str::<SlackBlockActionsPayloadTransfer>(payload)?;
    let slack_block_actions = slack_block_actions_to_model::convert(&slack_block_actions_transfer);

    let (res, url) = process_block_action(slack_block_actions);
    Ok((res, Some(url)))
}

fn process_block_action(slack_block_actions: SlackBlockActions) -> (SlackResponse, String) {
    let command_processors = get_block_action_processors();

    for action in &slack_block_actions.actions {
        for processor in &command_processors {
            if processor.can_handle_command(&action.action_id) {
                return (processor.handle_block_actions_command(&slack_block_actions).unwrap_or(SlackResponse::from_string("eh?")), slack_block_actions.response_url)
            }
        }
    }

    return (SlackResponse::from_string("I must be drinking to much... I can't seem to remember how to process this?!?!"), slack_block_actions.response_url)
}


/// get action processor list
fn get_block_action_processors() -> Vec<Box<dyn CommandHandler>> {
    vec!(
        Box::new(JokeAddHandler::new()),
        Box::new(FixedResponseHandler::new(&String::from("cancel-action"), &SlackResponse::from_string_ephemeral("Ok then. :fast_parrot:"))),
    )
}