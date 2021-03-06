use crate::transfer::SlackBlockActionsStatesTransfer;
use crate::converter::slack_action_option_to_model;
use crate::model::*;

pub fn convert(transfer: &SlackBlockActionsStatesTransfer) -> Vec<SlackBlockActionsState> {
    (&transfer.values).into_iter().map(|state| -> Vec<SlackBlockActionsState> {
        let (_ , state) = state;

        return (&state.states).into_iter().map(|state| {
            let (name, state_data) = state;

            SlackBlockActionsState {
                name: name.clone(),
                state_type: SlackElementType::from(&state_data.state_type),
                value: state_data.value.clone(),
                selected_options: state_data.selected_options.as_ref().map(|options| options.iter().map(|option| slack_action_option_to_model::convert(option)).collect())
            }
        }).collect()
    }).flatten().collect()
}