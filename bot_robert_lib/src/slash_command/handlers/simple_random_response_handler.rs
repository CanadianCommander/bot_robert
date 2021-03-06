use crate::model::*;
use async_trait::async_trait;
use crate::slash_command::handlers::command_processor::CommandProcessor;
use crate::slash_command::handlers::command_matcher::CommandMatcher;
use crate::slash_command::handlers::command_handler::CommandHandler;
use rand::Rng;


pub struct SimpleRandomResponseHandler {
    pub command_name: String,
    pub responses: Vec<SlackResponse>,
}

impl SimpleRandomResponseHandler {
    pub fn new(command: &String, responses: &Vec<SlackResponse>) -> SimpleRandomResponseHandler {
        SimpleRandomResponseHandler {
            command_name: command.clone(),
            responses: responses.clone(),
        }
    }
}

#[async_trait]
impl CommandProcessor for SimpleRandomResponseHandler {
    async fn handle_command(&self, _command: &String, _user_name: &String) -> Option<SlackResponse> {
        let rand_index = rand::thread_rng().gen_range(0..(self.responses.len()));
        Some(self.responses.get(rand_index).map(|resp| resp.clone())
            .unwrap_or(SlackResponse::from_string(&String::from("Hmm. This doesn't seem right. :/")).clone()))
    }
}

impl CommandMatcher for SimpleRandomResponseHandler {
    fn get_command_name(&self) -> String {
        self.command_name.clone()
    }
}

impl CommandHandler for SimpleRandomResponseHandler {}