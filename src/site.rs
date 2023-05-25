use crate::models::*;
use axum::{
    response::Redirect,
    routing::{get, post},
    Form, Json, Router,
};
use projects::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use strum::VariantNames;

pub fn get_tags(projects: &Vec<Project>) -> BTreeSet<String> {
    projects
        .iter()
        .filter(|p| !p.tags.is_empty())
        .map(|p| p.tags.clone())
        .flatten()
        .collect()
}

#[derive(Debug, Clone, Deserialize)]
struct ContactForm {
    name: String,
    contact: String,
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Embed {
    title: String,
    description: String,
    color: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WebhookMessage {
    username: String,
    content: Option<String>,
    embeds: Vec<Embed>,
}

fn escape_code(s: &str) -> String {
    s.replace("`", "`​")
}

async fn contact_form(Json(form): Json<ContactForm>) {
    println!("Contact Form: {:?}", &form);
    let msg = WebhookMessage {
        username: "Website Message".into(),
        content: None,
        embeds: vec![Embed {
            title: "Contact Form".into(),
            description: format!(
                "From ``{}``\nContact: ``{}``\n```\n{}\n```",
                escape_code(&form.name),
                escape_code(&form.contact),
                escape_code(&form.content)
            ),
            color: 0x55ff77,
        }],
    };

    let client = reqwest::Client::new();
    let req = client
        .post(&std::env::var("WEBHOOK_URL").expect("WEBHOOK_URL is not set"))
        .json(&msg)
        .send()
        .await;

    match req {
        Ok(_) => {}
        Err(_) => eprintln!("ERROROROROROR: Unable to send msg to discord"),
    }
}

pub fn router(projects: Vec<Project>) -> Router {
    let tags = get_tags(&projects);
    Router::new()
        .route(
            "/",
            get(|| async { Redirect::temporary("https://funnyboyroks.com") }),
        )
        .route("/contact", post(contact_form))
        .route("/projects", get(|| async { Json(projects) }))
        .route(
            "/projects/langs",
            get(|| async { Json(Language::VARIANTS) }),
        )
        .route("/projects/tags", get(|| async { Json(tags) }))
}
