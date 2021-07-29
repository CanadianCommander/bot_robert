use crate::*;
use crate::slash_command::model::Joke;
use crate::slash_command::{SlackResponse, DB_JOKE_COLLECTION};
use std::error::Error;
use futures::StreamExt;

/// get the list of jokes as slack responses
pub async fn jokes_as_slack_responses() -> Vec<SlackResponse> {

    if let Ok(mut jokes) = serde_json::from_str::<Vec<Joke>>(resources::DAD_JOKES) {
        if let Ok(mut custom_jokes) = get_custom_jokes().await {
            jokes.append(&mut custom_jokes)
        }
        jokes.into_iter().map(|joke| SlackResponse::from_string(&format!("{}\n{}", joke.setup, joke.punchline))).collect()
    }
    else {
        vec!(SlackResponse::from_string(&String::from("RIP JSON")))
    }
}

async fn get_custom_jokes() -> Result<Vec<Joke>, Box<dyn Error>> {
    let db_con = database::DATABASE_CONNECTION.lock().await;

    let joke_collection = db_con.get_collection(DB_JOKE_COLLECTION)?;
    let cursor = joke_collection.find(None, None).await?;

    let jokes: Vec<Option<Joke>> = cursor.map(|document| {
        let document = match document {
            Ok(doc) => doc,
            Err(_) => return None,
        };

        // Nothing like raw string "column names"... lol. Super easy to maintain. Need ORM.
        let setup = document.get("setup");
        let punchline = document.get("punchline");

        return if let (Some(setup), Some(punchline)) = (setup, punchline) {
            Some(Joke {setup: setup.to_string(), punchline: punchline.to_string()})
        }
        else {
            None
        }
    }).collect().await;

    // In theory both this and the last lines could be combined. futures's (rust crate) streams seem to freak out when chaining map & filter.
    // Probably just me being stupid.
    Ok(jokes.into_iter().filter(|joke| joke.is_some()).map(|joke| joke.unwrap()).collect())
}