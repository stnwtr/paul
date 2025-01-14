fn main() {
}

async fn create_interaction_response() {
}

// // use poise::serenity_prelude as serenity;
// //
// // struct Data {} // User data, which is stored and accessible in all command invocations
// // type Error = Box<dyn std::error::Error + Send + Sync>;
// // type Context<'a> = poise::Context<'a, Data, Error>;
// //
// // /// Displays your or another user's account creation date
// // #[poise::command(slash_command, prefix_command)]
// // async fn age(
// //     ctx: Context<'_>,
// //     #[description = "Selected user"] user: Option<serenity::User>,
// // ) -> Result<(), Error> {
// //     let u = user.as_ref().unwrap_or_else(|| ctx.author());
// //     let response = format!("{}'s account was created at {}", u.name, u.created_at());
// //     ctx.say(response).await?;
// //     Ok(())
// // }
// //
// // #[tokio::main]
// // async fn main() {
// //     let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
// //     let intents = serenity::GatewayIntents::non_privileged();
// //
// //     let framework = poise::Framework::builder()
// //         .options(poise::FrameworkOptions {
// //             commands: vec![age()],
// //             ..Default::default()
// //         })
// //         .setup(|ctx, _ready, framework| {
// //             Box::pin(async move {
// //                 poise::builtins::register_globally(ctx, &framework.options().commands).await?;
// //                 Ok(Data {})
// //             })
// //         })
// //         .build();
// //
// //     let client = serenity::ClientBuilder::new(token, intents)
// //         .framework(framework)
// //         .await;
// //     client.unwrap().start().await.unwrap();
// // }
// //
// // https://discord.com/oauth2/authorize?client_id=1326298038939025559
// 
// use std::fs;
// use std::time::Duration;
// use reqwest::blocking::Body;
// use reqwest::blocking::multipart::Part;
// use reqwest::header::HeaderMap;
// use serenity::builder::*;
// use serenity::interactions_endpoint::Verifier;
// use serenity::json;
// use serenity::model::application::*;
// 
// type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
// 
// fn handle_command(interaction: CommandInteraction) -> CreateInteractionResponse {
//     let interaction_token = interaction.token.to_owned();
//     let interaction_id = interaction.id.to_string();
//     let application_id = interaction.application_id.to_string();
// 
//     std::thread::spawn(move || {
//         std::thread::sleep(std::time::Duration::from_secs(1));
//         follow_up(interaction_token.as_str(), interaction_id.as_str(), application_id.as_str());
//     });
// 
//     CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new().content("Hey"))
// }
// 
// fn handle_request(
//     mut request: tiny_http::Request,
//     body: &mut Vec<u8>,
//     verifier: &Verifier,
// ) -> Result<(), Error> {
//     println!("Received request from {:?}", request.remote_addr());
// 
//     // Read the request body (containing the interaction JSON)
//     body.clear();
//     request.as_reader().read_to_end(body)?;
// 
//     // Reject request if it fails cryptographic verification
//     // Discord rejects the interaction endpoints URL if this check is not done
//     // (This part is very specific to your HTTP server crate of choice, so serenity cannot abstract
//     // away the boilerplate)
//     let find_header =
//         |name| Some(request.headers().iter().find(|h| h.field.equiv(name))?.value.as_str());
//     let signature = find_header("X-Signature-Ed25519").ok_or("missing signature header")?;
//     let timestamp = find_header("X-Signature-Timestamp").ok_or("missing timestamp header")?;
//     if verifier.verify(signature, timestamp, body).is_err() {
//         request.respond(tiny_http::Response::empty(401))?;
//         return Ok(());
//     }
// 
//     // Build Discord response
//     let response = match json::from_slice::<Interaction>(body)? {
//         // Discord rejects the interaction endpoints URL if pings are not acknowledged
//         Interaction::Ping(_) => CreateInteractionResponse::Pong,
//         Interaction::Command(interaction) => handle_command(interaction),
//         _ => return Ok(()),
//     };
// 
//     // Send the Discord response back via HTTP
//     request.respond(
//         tiny_http::Response::from_data(json::to_vec(&response)?)
//             .with_header("Content-Type: application/json".parse::<tiny_http::Header>().unwrap()),
//     )?;
// 
//     Ok(())
// }
// 
// fn main() -> Result<(), Error> {
//     // Change this string to the Public Key value in your bot dashboard
//     let verifier =
//         Verifier::new("7d2b7d9084b7dafe913a2b86f88e12f42d99dfacb49420c47cb51455edfa6dcd");
// 
//     // Setup an HTTP server and listen for incoming interaction requests
//     // Choose any port here (but be consistent with the interactions endpoint URL in your bot
//     // dashboard)
//     let server = tiny_http::Server::http("0.0.0.0:8787")?;
//     let mut body = Vec::new();
//     loop {
//         let request = server.recv()?;
//         if let Err(e) = handle_request(request, &mut body, &verifier) {
//             eprintln!("Error while handling request: {e}");
//         }
//     }
// }
// 
// fn follow_up(interaction_token: &str, interaction_id: &str, application_id: &str) {
//     let base = "https://discord.com/api/v10";
// 
//     let patch = format!("{}/webhooks/{}/{}/messages/@original", base, application_id, interaction_token);
//     let post = format!("{}/webhooks/{}/{}", base, application_id, interaction_token);
// 
//     let create_message_body = r#"{"content": "Congrats on sending your command!", "attachments": [{"id": 0, "description": "imagea of reyna valorant", "url": "https://cdn.discordapp.com/attachments/542675523583737856/1328679245966479371/reyna.png?ex=67879479&is=678642f9&hm=36e37484a7aa7572f7242c58885dd552d63276a9f864d4d237b6bf67c4d5ad58&"}]}"#;
//     let create_message_body = r#"{"content": "Congrats on sending your command!", "attachments": [{"id": 0, "description": "imagea of reyna valorant", "url": "wtf"}]}"#;
// 
//     //
//     let file_path = "/home/stnwtr/Downloads/reyna.png";
//     let form = reqwest::blocking::multipart::Form::new()
//         .part("payload_json", Part::text(create_message_body).mime_str("application/json").unwrap())
//         .part("files[0]", Part::file(file_path).unwrap());
//     // 
// 
//     let response = reqwest::blocking::Client::new()
//         .patch(patch)
//         .multipart(form)
//         .send()
//         .unwrap();
// 
//     println!("Response: {}", response.text().unwrap());
// }
// // //
// // // // use axum_extra::TypedHeader;
// // // // use ed25519_dalek::{Signature, Verifier, VerifyingKey};
// // // // use ed25519_dalek::ed25519::Error;
// // // //
// // // // const SIGNATURE: &str = "f7f337c84bb8ad68e467c8973d994fe70775209101be308a11382c3ae46cc9eb93e935d361be5836b0908751475f68a9c1a78063bce65eafce042ba68f037902";
// // // // const TIMESTAMP: &str = "1736720526";
// // // // const PUBLIC_KEY: &str = "7d2b7d9084b7dafe913a2b86f88e12f42d99dfacb49420c47cb51455edfa6dcd";
// // // // const BODY: &str = r#"{"app_permissions":"562949953601536","application_id":"1326298038939025559","authorizing_integration_owners":{},"entitlements":[],"id":"1328126892561141800","token":"aW50ZXJhY3Rpb246MTMyODEyNjg5MjU2MTE0MTgwMDphdFcwUkpLdjhtTlVyN20xY0dRMGtIcnFJcVpER2hZMXBnQlRnRjlOOVNmeERhUTBtekdLajU4ZmdJM2h4UnZiR0R2S04weVRBV3VCdFhHN1lxWW9xOFh0Y2NWbXVnZ0JsTXdZOE1tMUZPM1NGRVBvMEt0R1BLN3ZvQWtRNTU1Ng","type":1,"user":{"avatar":"c6a249645d46209f337279cd2ca998c7","avatar_decoration_data":null,"bot":true,"clan":null,"discriminator":"0000","global_name":"Discord","id":"643945264868098049","primary_guild":null,"public_flags":1,"system":true,"username":"discord"},"version":1}"#;
// // // //
// // // // fn main() {
// // // //     let serenity_verifier = serenity::interactions_endpoint::Verifier::new(PUBLIC_KEY);
// // // //     let serenity_check = serenity_verifier.verify(SIGNATURE, TIMESTAMP, BODY.as_bytes());
// // // //
// // // //     println!("Serenity: {}", serenity_check.is_ok());
// // // //
// // // //     println!("Hex: {:?}", HelloVerifier{verifying_key: VerifyingKey::frombyttes});
// // // //
// // // //     TypedHeader
// // // // }
// // // //
// // // // fn verify() -> bool {
// // // //     let key: [u8; 32] = hex::decode(PUBLIC_KEY).unwrap().try_into().unwrap();
// // // //     let verifier = ed25519_dalek::VerifyingKey::from_bytes(&key).unwrap();
// // // //     let signature: [u8; 64] = hex::decode(SIGNATURE).unwrap().try_into().unwrap();
// // // //
// // // //     let msg = (TIMESTAMP.to_owned() + BODY).into_bytes();
// // // //
// // // //     verifier.verify(&msg, &Signature::from_bytes(&signature)).is_ok()
// // // // }
// // // //
// // // // pub struct HelloVerifier {
// // // //     pub verifying_key: VerifyingKey
// // // // }
// // // //
// // // // impl HelloVerifier {
// // // //     pub fn verify(&self, message: &str, signature: &str) -> Result<(), Error> {
// // // //         let signature: [u8; 64] = hex::decode(signature).unwrap().try_into().unwrap();
// // // //         self.verifying_key.verify(message.as_bytes(), &Signature::from_bytes(&signature))
// // // //     }
// // // // }
// // // //
// // // // // use axum::{serve, Router};
// // // // // use axum::http::{HeaderMap, StatusCode};
// // // // // use axum::response::IntoResponse;
// // // // // use axum::routing::post;
// // // // // use ed25519_dalek::{Signature, Verifier, VerifyingKey, PUBLIC_KEY_LENGTH};
// // // // // use tokio::net::TcpListener;
// // // // // use tokio::signal;
// // // // //
// // // // // fn verify(body: &str, headers: &HeaderMap, app_pk: &str) -> Result<(), StatusCode> {
// // // // //     let application_public_key: [u8; PUBLIC_KEY_LENGTH] = hex::decode(&app_pk)
// // // // //         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
// // // // //         .try_into()
// // // // //         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
// // // // //
// // // // //     let verifier = VerifyingKey::from_bytes(&application_public_key)
// // // // //         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
// // // // //
// // // // //     let timestamp = headers
// // // // //         .get("X-Signature-Timestamp")
// // // // //         .ok_or(StatusCode::BAD_REQUEST)?
// // // // //         .to_str()
// // // // //         .map_err(|_| StatusCode::BAD_REQUEST)?;
// // // // //
// // // // //     let signature_str = headers
// // // // //         .get("X-Signature-Ed25519")
// // // // //         .ok_or(StatusCode::BAD_REQUEST)?
// // // // //         .to_str()
// // // // //         .map_err(|_| StatusCode::BAD_REQUEST)?;
// // // // //
// // // // //     let signature: [u8; 64] = hex::decode(&signature_str)
// // // // //         .map_err(|_| StatusCode::BAD_REQUEST)?
// // // // //         .try_into()
// // // // //         .map_err(|_| StatusCode::BAD_REQUEST)?;
// // // // //
// // // // //     let msg = (timestamp.to_owned() + body).into_bytes();
// // // // //
// // // // //     verifier
// // // // //         .verify(&msg, &Signature::from_bytes(&signature))
// // // // //         .map_err(|_| StatusCode::UNAUTHORIZED)
// // // // // }
// // // // //
// // // // // async fn interaction_handler(header_map: HeaderMap, body: String) -> impl IntoResponse {
// // // // //     let signature = header_map.get("x-signature-ed25519");
// // // // //     let timestamp = header_map.get("x-signature-timestamp");
// // // // //
// // // // //     println!("{:?}\n{:?}\n{}", signature, timestamp, body);
// // // // //     "response"
// // // // // }
// // // // //
// // // // // #[tokio::main]
// // // // // async fn main() -> anyhow::Result<()> {
// // // // //     let app = Router::new()
// // // // //         .route("/interaction", post(interaction_handler));
// // // // //     let listener = TcpListener::bind("0.0.0.0:8080").await?;
// // // // //
// // // // //     serve(listener, app)
// // // // //         .with_graceful_shutdown(shutdown_signal())
// // // // //         .await?;
// // // // //
// // // // //     Ok(())
// // // // // }
// // // // //
// // // // // async fn shutdown_signal() {
// // // // //     let ctrl_c = async {
// // // // //         signal::ctrl_c()
// // // // //             .await
// // // // //             .expect("failed to install Ctrl+C handler");
// // // // //     };
// // // // //
// // // // //     #[cfg(unix)]
// // // // //     let terminate = async {
// // // // //         signal::unix::signal(signal::unix::SignalKind::terminate())
// // // // //             .expect("failed to install signal handler")
// // // // //             .recv()
// // // // //             .await;
// // // // //     };
// // // // //
// // // // //     #[cfg(not(unix))]
// // // // //     let terminate = std::future::pending::<()>();
// // // // //
// // // // //     tokio::select! {
// // // // //         _ = ctrl_c => {},
// // // // //         _ = terminate => {},
// // // // //     }
// // // // // }
// // // // //
// // // // // // use serenity::builder::*;
// // // // // // use serenity::interactions_endpoint::Verifier;
// // // // // // use serenity::json;
// // // // // // use serenity::model::application::*;
// // // // // //
// // // // // // type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
// // // // // //
// // // // // // fn handle_command(interaction: CommandInteraction) -> CreateInteractionResponse {
// // // // // //     CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(format!(
// // // // // //         "Hello from interactions webhook HTTP server! <@{}>",
// // // // // //         interaction.user.id
// // // // // //     )).ephemeral(true))
// // // // // // }
// // // // // //
// // // // // // fn handle_request(
// // // // // //     mut request: tiny_http::Request,
// // // // // //     body: &mut Vec<u8>,
// // // // // //     verifier: &Verifier,
// // // // // // ) -> Result<(), Error> {
// // // // // //     println!("Received request from {:?}", request.remote_addr());
// // // // // //
// // // // // //     // Read the request body (containing the interaction JSON)
// // // // // //     body.clear();
// // // // // //     request.as_reader().read_to_end(body)?;
// // // // // //
// // // // // //     // Reject request if it fails cryptographic verification
// // // // // //     // Discord rejects the interaction endpoints URL if this check is not done
// // // // // //     // (This part is very specific to your HTTP server crate of choice, so serenity cannot abstract
// // // // // //     // away the boilerplate)
// // // // // //     let find_header =
// // // // // //         |name| Some(request.headers().iter().find(|h| h.field.equiv(name))?.value.as_str());
// // // // // //     let signature = find_header("X-Signature-Ed25519").ok_or("missing signature header")?;
// // // // // //     let timestamp = find_header("X-Signature-Timestamp").ok_or("missing timestamp header")?;
// // // // // //     if verifier.verify(signature, timestamp, body).is_err() {
// // // // // //         request.respond(tiny_http::Response::empty(401))?;
// // // // // //         return Ok(());
// // // // // //     }
// // // // // //
// // // // // //     // Build Discord response
// // // // // //     let response = match json::from_slice::<Interaction>(body)? {
// // // // // //         // Discord rejects the interaction endpoints URL if pings are not acknowledged
// // // // // //         Interaction::Ping(_) => CreateInteractionResponse::Pong,
// // // // // //         Interaction::Command(interaction) => handle_command(interaction),
// // // // // //         _ => return Ok(()),
// // // // // //     };
// // // // // //
// // // // // //     // Send the Discord response back via HTTP
// // // // // //     request.respond(
// // // // // //         tiny_http::Response::from_data(json::to_vec(&response)?)
// // // // // //             .with_header("Content-Type: application/json".parse::<tiny_http::Header>().unwrap()),
// // // // // //     )?;
// // // // // //
// // // // // //     Ok(())
// // // // // // }
// // // // // //
// // // // // // fn main() -> Result<(), Error> {
// // // // // //     // Change this string to the Public Key value in your bot dashboard
// // // // // //     let verifier =
// // // // // //         Verifier::new("7d2b7d9084b7dafe913a2b86f88e12f42d99dfacb49420c47cb51455edfa6dcd");
// // // // // //
// // // // // //     // Setup an HTTP server and listen for incoming interaction requests
// // // // // //     // Choose any port here (but be consistent with the interactions endpoint URL in your bot
// // // // // //     // dashboard)
// // // // // //     let server = tiny_http::Server::http("0.0.0.0:8080")?;
// // // // // //     let mut body = Vec::new();
// // // // // //     loop {
// // // // // //         let request = server.recv()?;
// // // // // //         if let Err(e) = handle_request(request, &mut body, &verifier) {
// // // // // //             eprintln!("Error while handling request: {e}");
// // // // // //         }
// // // // // //     }
// // // // // // }
// // // // // //
// // // // // // // APP ID  = 1326298038939025559
// // // // // // // PUB KEY = 7d2b7d9084b7dafe913a2b86f88e12f42d99dfacb49420c47cb51455edfa6dcd
// // // // // // // CLI ID  = 1326298038939025559
// // // // // // // CLI SEC = sVKsFhHhaPFD0voa2En8XBvY2Bf-NzR8
// // // // // //
// // // // // // // {"token_type": "Bearer", "access_token": "VGITQ4Dl56CtD3ECJlSkIua5WKFjl9", "expires_in": 604800, "scope": "applications.commands.update identify connections guilds"
// // // // // //
// // // // // // // curl -X POST https://discord.com/api/oauth2/token \
// // // // // // //         -d client_id=1326298038939025559 \
// // // // // // //         -d client_secret=sVKsFhHhaPFD0voa2En8XBvY2Bf-NzR8 \
// // // // // // //         -d grant_type=client_credentials \
// // // // // // //         -d scope=applications.commands.update
// // // // // //
// // // // // //
// // // // // // // curl -X POST "https://discord.com/api/v10/applications/1326298038939025559/commands" \
// // // // // // //       -H "Authorization: Bearer S2RfT7iua0l9Io469xIrCfE7CzNeba" \
// // // // // // //       -v \
// // // // // // //       -d '{"name": "blep", "type": 1, "description": "Send a random adorable animal photo"}' \
// // // // // // //       -H "Content-Type: application/json"
// // // // // //
