use std::env;

use serde::{Deserialize, Serialize};
use serde_json::json;
use serenity::http::Http;
use serenity::model::webhook::Webhook;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web, post};
use dotenv::dotenv;

#[derive(Debug, Deserialize, Serialize)]
struct RanChatData {
    pub em_type : i32,
    pub sz_name : String,
    pub sz_chat_msg : String,
}

#[get("/api/healthcheck")]
async fn health_check() -> impl Responder {
    const MESSAGE: &str = "PUTANG INA MO GUMAGANA AKO";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[post("/api/sendmsg")]
async fn send_msg(
    body: web::Json<RanChatData>
) -> impl Responder {
    println!("Message: {:?}", body.0);
    ran_chat_hook(body.0).await;
    HttpResponse::Ok().json(json!({"status": "success","message": "ok"}))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host: String = env::var("HOST_IP").expect("Set HOST_IP environment variable.");
    let port: u16 = env::var("HOST_PORT")
        .expect("Set HOST_PORT environment variable.")
        .parse()
        .expect("Invalid port number.");

    println!("🚀 Discord Hook Server started successfully");

    HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(send_msg)
    })
    .bind((host.clone(), port))?
    .run()
    .await

}

async fn ran_chat_hook(chat_data: RanChatData) {
    // You don't need a token when you are only dealing with webhooks.
    let http = Http::new("");
    let webhook = Webhook::from_url(&http, "https://discord.com/api/webhooks/1146037187033452574/tetEkHhMfSb7fag-te4GnxlFVmoYj3Gd8aPHj1UFuIQR8IvtWYEmboViVyReKleUy1hd").await.expect("Replace the webhook with your own");
    let avatar_url = "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQXzNJjFMvQdH2vjVLBMeyR4GaukOUGFPg1YojPPkaELxMH8Bk1UBgRqfaIl88P101uqrA&usqp=CAU";
    match chat_data.em_type {
        // CHAT_TYPE_GLOBAL
        1 => {
            webhook
                .execute(&http, false, |w| 
                    w
                    .username(chat_data.sz_name)
                    .content(chat_data.sz_chat_msg)
                    .avatar_url(avatar_url)
                )
                .await
                .expect("Could not execute webhook.");
        }
        // CHAT_TYPE_NORMA
        2 => {
            webhook
                .execute(&http, false, |w| 
                    w
                    .username(chat_data.sz_name)
                    .content(chat_data.sz_chat_msg)
                    .avatar_url(avatar_url)
                )
                .await
                .expect("Could not execute webhook.");
        }
        // CHAT_TYPE_CTRL_GLOBAL
        8 => {
            webhook
                .execute(&http, false, |w| 
                    w
                    .username(chat_data.sz_name)
                    .content(chat_data.sz_chat_msg)
                    .avatar_url(avatar_url)
                )
                .await
                .expect("Could not execute webhook.");
        }
        // CHAT_TYPE_CTRL_GLOBAL2
        9 => {
            webhook
                .execute(&http, false, |w| 
                    w
                    .username(chat_data.sz_name)
                    .content(chat_data.sz_chat_msg)
                    .avatar_url(avatar_url)
                )
                .await
                .expect("Could not execute webhook.");
        }
        _ => {
            webhook
                .execute(&http, false, |w| 
                    w
                    .username(chat_data.sz_name)
                    .content(chat_data.sz_chat_msg)
                )
                .await
                .expect("Could not execute webhook.");
        }
    }
}
