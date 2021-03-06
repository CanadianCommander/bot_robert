use crate::transfer::SlackBlockActionsActionTransfer;
use crate::model::*;
use crate::converter::*;

pub fn convert(transfer: &SlackBlockActionsActionTransfer) -> SlackBlockActionsAction {
    SlackBlockActionsAction {
        action_id: transfer.action_id.clone(),
        action_type: SlackActionType::from(transfer.action_type.clone()),
        block_id: transfer.block_id.clone(),
        value: transfer.value.clone(),
        text: transfer.text.as_ref().map(|text| slack_text_transfer_to_model::convert(text)),
        selected_options: transfer.selected_options.as_ref().map(|selected_options| selected_options.iter().map(|opt| slack_action_option_to_model::convert(opt)).collect())
    }
}