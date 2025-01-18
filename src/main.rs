use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{serve, Json, Router};
use ed25519_dalek::{Signature, Verifier, VerifyingKey, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};
use rand::{thread_rng, Rng};
use reqwest::multipart::{Form, Part};
use reqwest::Client;
use serde_json::{json, Value};
use tokio::net::TcpListener;

const PUBLIC_KEY: &str = "7d2b7d9084b7dafe913a2b86f88e12f42d99dfacb49420c47cb51455edfa6dcd";

const AGENTS: &[(&str, &str, &[u8])] = &[
    ("Astra", "astra.png", include_bytes!("../images/agents/astra.png")),
    ("Breach", "breach.png", include_bytes!("../images/agents/breach.png")),
    ("Brimstone", "brimstone.png", include_bytes!("../images/agents/brimstone.png")),
    ("Chamber", "chamber.png", include_bytes!("../images/agents/chamber.png")),
    ("Clove", "clove.png", include_bytes!("../images/agents/clove.png")),
    ("Cypher", "cypher.png", include_bytes!("../images/agents/cypher.png")),
    ("Deadlock", "deadlock.png", include_bytes!("../images/agents/deadlock.png")),
    ("Fade", "fade.png", include_bytes!("../images/agents/fade.png")),
    ("Gekko", "gekko.png", include_bytes!("../images/agents/gekko.png")),
    ("Harbor", "harbor.png", include_bytes!("../images/agents/harbor.png")),
    ("Iso", "iso.png", include_bytes!("../images/agents/iso.png")),
    ("Jett", "jett.png", include_bytes!("../images/agents/jett.png")),
    ("KAY/O", "kayo.png", include_bytes!("../images/agents/kayo.png")),
    ("Killjoy", "killjoy.png", include_bytes!("../images/agents/killjoy.png")),
    ("Neon", "neon.png", include_bytes!("../images/agents/neon.png")),
    ("Omen", "omen.png", include_bytes!("../images/agents/omen.png")),
    ("Phoenix", "phoenix.png", include_bytes!("../images/agents/phoenix.png")),
    ("Raze", "raze.png", include_bytes!("../images/agents/raze.png")),
    ("Reyna", "reyna.png", include_bytes!("../images/agents/reyna.png")),
    ("Sage", "sage.png", include_bytes!("../images/agents/sage.png")),
    ("Skye", "skye.png", include_bytes!("../images/agents/skye.png")),
    ("Sova", "sova.png", include_bytes!("../images/agents/sova.png")),
    ("Tejo", "tejo.png", include_bytes!("../images/agents/tejo.png")),
    ("Viper", "viper.png", include_bytes!("../images/agents/viper.png")),
    ("Vyse", "vyse.png", include_bytes!("../images/agents/vyse.png")),
    ("Yoru", "yoru.png", include_bytes!("../images/agents/yoru.png")),
];

const WEAPONS: &[(&str, &str, &[u8])] = &[
    ("Ares", "ares.png", include_bytes!("../images/weapons/ares.png")),
    ("Bucky", "bucky.png", include_bytes!("../images/weapons/bucky.png")),
    ("Bulldog", "bulldog.png", include_bytes!("../images/weapons/bulldog.png")),
    ("Classic", "classic.png", include_bytes!("../images/weapons/classic.png")),
    ("Frenzy", "frenzy.png", include_bytes!("../images/weapons/frenzy.png")),
    ("Ghost", "ghost.png", include_bytes!("../images/weapons/ghost.png")),
    ("Guardian", "guardian.png", include_bytes!("../images/weapons/guardian.png")),
    ("Judge", "judge.png", include_bytes!("../images/weapons/judge.png")),
    ("Knife", "knife.png", include_bytes!("../images/weapons/knife.png")),
    ("Marshal", "marshal.png", include_bytes!("../images/weapons/marshal.png")),
    ("Odin", "odin.png", include_bytes!("../images/weapons/odin.png")),
    ("Operator", "operator.png", include_bytes!("../images/weapons/operator.png")),
    ("Outlaw", "outlaw.png", include_bytes!("../images/weapons/outlaw.png")),
    ("Phantom", "phantom.png", include_bytes!("../images/weapons/phantom.png")),
    ("Sheriff", "sheriff.png", include_bytes!("../images/weapons/sheriff.png")),
    ("Shorty", "shorty.png", include_bytes!("../images/weapons/shorty.png")),
    ("Spectre", "spectre.png", include_bytes!("../images/weapons/spectre.png")),
    ("Stinger", "stinger.png", include_bytes!("../images/weapons/stinger.png")),
    ("Vandal", "vandal.png", include_bytes!("../images/weapons/vandal.png")),
];

const ARMOR: &[(&str, &str, &[u8])] = &[
    ("Light Armor", "light.png", include_bytes!("../images/armor/light.png")),
    ("Regen Heavy Armor", "heavy.png", include_bytes!("../images/armor/heavy.png")),
    ("Regen Shield", "regen.png", include_bytes!("../images/armor/regen.png")),
];

async fn handle_interaction(interaction: Value) {
    let command = interaction["data"]["name"].as_str().unwrap();
    let application_id = interaction["application_id"].as_str().unwrap();
    let interaction_token = interaction["token"].as_str().unwrap();
    let username = interaction["member"]["user"]["username"].as_str().unwrap();

    let (item_name, file_name, file_content) = match command {
        "agent" => AGENTS[thread_rng().gen_range(0..AGENTS.len())],
        "weapon" => WEAPONS[thread_rng().gen_range(0..WEAPONS.len())],
        "armor" => ARMOR[thread_rng().gen_range(0..ARMOR.len())],
        _ => panic!("Unknown command."),
    };

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
            Part::bytes(file_content).mime_str("image/png").unwrap().file_name(file_name),
        );

    println!("{} got {}.", username, item_name);

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
