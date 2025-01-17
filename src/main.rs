use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{serve, Json, Router};
use ed25519_dalek::{Signature, Verifier, VerifyingKey, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use reqwest::multipart::{Form, Part};
use reqwest::Client;
use serde_json::{json, Value};
use tokio::net::TcpListener;

const PUBLIC_KEY: &str = "7d2b7d9084b7dafe913a2b86f88e12f42d99dfacb49420c47cb51455edfa6dcd";

const AGENTS: &[(&str, &str)] = &[
    ("Astra", "astra"),
    ("Breach", "breach"),
    ("Brimstone", "brimstone"),
    ("Chamber", "chamber"),
    ("Clove", "clove"),
    ("Cypher", "cypher"),
    ("Deadlock", "deadlock"),
    ("Fade", "fade"),
    ("Gekko", "gekko"),
    ("Harbor", "harbor"),
    ("Iso", "iso"),
    ("Jett", "jett"),
    ("KAY/O", "kayo"),
    ("Killjoy", "killjoy"),
    ("Neon", "neon"),
    ("Omen", "omen"),
    ("Phoenix", "phoenix"),
    ("Raze", "raze"),
    ("Reyna", "reyna"),
    ("Sage", "sage"),
    ("Skye", "skye"),
    ("Sova", "sova"),
    ("Tejo", "tejo"),
    ("Viper", "viper"),
    ("Vyse", "vyse"),
    ("Yoru", "yoru"),
];

const WEAPONS: &[(&str, &str)] = &[
    ("Ares", "ares"),
    ("Bucky", "bucky"),
    ("Bulldog", "bulldog"),
    ("Classic", "classic"),
    ("Frenzy", "frenzy"),
    ("Ghost", "ghost"),
    ("Guardian", "guardian"),
    ("Judge", "judge"),
    ("Knife", "knife"),
    ("Marshal", "marshal"),
    ("Odin", "odin"),
    ("Operator", "operator"),
    ("Outlaw", "outlaw"),
    ("Phantom", "phantom"),
    ("Sheriff", "sheriff"),
    ("Shorty", "shorty"),
    ("Spectre", "spectre"),
    ("Stinger", "stinger"),
    ("Vandal", "vandal"),
];

const ARMOR: &[(&str, &str)] = &[
    ("Light Armor", "light"),
    ("Regen Heavy Armor", "heavy"),
    ("Regen Shield", "regen"),
];

async fn handle_interaction(interaction: Value) {
    let application_id = interaction["application_id"].as_str().unwrap();
    let interaction_token = interaction["token"].as_str().unwrap();

    let (_agent_name, file_name) = AGENTS.choose(&mut thread_rng()).unwrap();
    let file_path = format!("/home/stnwtr/Downloads/paul/{}.png", file_name);

    let url = format!(
        "https://discord.com/api/v10/webhooks/{}/{}/messages/@original",
        application_id, interaction_token
    );
    let json_message = r#"{"attachments":[{"id":0}]}"#;
    let form = Form::new()
        .part(
            "payload_json",
            Part::text(json_message)
                .mime_str("application/json")
                .unwrap(),
        )
        .part(
            "files[0]",
            Part::file(file_path).await.unwrap().file_name("reyna.png"),
        );

    Client::new()
        .patch(url)
        .multipart(form)
        .send()
        .await
        .unwrap();
}

async fn handler(header_map: HeaderMap, body: String) -> Response {
    let signature = header_map.get("X-Signature-Ed25519").unwrap();
    let timestamp = header_map.get("X-Signature-Timestamp").unwrap();

    if !verify(signature, timestamp, &body) {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let body: Value = serde_json::from_str(&body).unwrap();

    if body["type"] == 1 {
        return Json(json!({"type": 1})).into_response();
    }

    tokio::spawn(async move { handle_interaction(body).await });

    Json(json!({"type": 5, "data": {"content": "test-content"}})).into_response()
}

fn verify(signature: &HeaderValue, timestamp: &HeaderValue, body: &str) -> bool {
    let key: [u8; PUBLIC_KEY_LENGTH] = hex::decode(PUBLIC_KEY).unwrap().try_into().unwrap();
    let verifier = VerifyingKey::from_bytes(&key).unwrap();
    let signature: [u8; SIGNATURE_LENGTH] = hex::decode(signature).unwrap().try_into().unwrap();
    let message = [timestamp.as_bytes(), body.as_bytes()].concat();
    verifier
        .verify(&message, &Signature::from_bytes(&signature))
        .is_ok()
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8787").await.unwrap();

    let router = Router::new().route("/", post(handler));

    serve(listener, router).await.unwrap();
}
