use std::fs;

use actix_web::{App, HttpResponse, HttpServer, Responder, Result, web};
use rc5_block_cipher_rust::rc5::{decrypt, encrypt};
use serde::{Deserialize, Serialize};

const ROUNDS: usize = 12;
const BLOCK_BYTES: usize = 8;

#[actix_web::get("/ping")]
pub async fn ping() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().body("pong"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(encrypt_file)
            .service(decrypt_file)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Deserialize)]
struct EncryptRequest {
    passphrase: String,
    text: String,
    filename: String,
}

#[derive(Deserialize)]
struct DecryptRequest {
    passphrase: String,
    filename: String,
}

#[derive(Serialize)]
struct DecryptResponse {
    text: String,
}

#[actix_web::post("/encrypt")]
async fn encrypt_file(req: web::Json<EncryptRequest>) -> Result<impl Responder> {
    let key = req.passphrase.as_bytes();
    let mut data = req.text.as_bytes().to_vec();

    let pad_len = (BLOCK_BYTES - (data.len() % BLOCK_BYTES)) % BLOCK_BYTES;
    data.extend(vec![0u8; pad_len]);

    let mut cipher = Vec::with_capacity(data.len());
    for chunk in data.chunks(BLOCK_BYTES) {
        let a = u32::from_be_bytes(chunk[0..4].try_into().unwrap());
        let b = u32::from_be_bytes(chunk[4..8].try_into().unwrap());
        let [c0, c1] = encrypt([a, b], key, ROUNDS);
        cipher.extend(&c0.to_be_bytes());
        cipher.extend(&c1.to_be_bytes());
    }

    fs::write(&req.filename, &cipher).map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().body("Encrypted and written to file"))
}

#[actix_web::post("/decrypt")]
async fn decrypt_file(req: web::Json<DecryptRequest>) -> Result<impl Responder> {
    let key = req.passphrase.as_bytes();
    let cipher = fs::read(&req.filename).map_err(|e| actix_web::error::ErrorNotFound(e))?;

    let mut plain = Vec::with_capacity(cipher.len());
    for chunk in cipher.chunks(BLOCK_BYTES) {
        let a = u32::from_be_bytes(chunk[0..4].try_into().unwrap());
        let b = u32::from_be_bytes(chunk[4..8].try_into().unwrap());
        let [p0, p1] = decrypt([a, b], key, ROUNDS);
        plain.extend(&p0.to_be_bytes());
        plain.extend(&p1.to_be_bytes());
    }

    while plain.last() == Some(&0u8) {
        plain.pop();
    }

    let text = String::from_utf8(plain).map_err(|e| actix_web::error::ErrorBadRequest(e))?;
    Ok(web::Json(DecryptResponse { text }))
}
