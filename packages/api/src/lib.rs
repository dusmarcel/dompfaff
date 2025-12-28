//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;
//use reqwest;

/// Echo the user input on the server.
// #[post("/api/echo")]
// pub async fn echo(input: String) -> Result<String, ServerFnError> {
//     Ok(input)
// }

#[get("/api/res")]
pub async fn res() -> Result<String, ServerFnError> {
    Ok("bar".to_string())
    //let response = reqwest::get("https://mi.aufentha.lt/api/v1/timelines/public").await;
    //response
}
