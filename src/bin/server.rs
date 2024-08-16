use std::fs;

use actix_web::{App, HttpResponse, HttpServer, Responder, Result, web};
use rc5_block_cipher_rust::rc5::encrypt;
use serde::Deserialize;

const ROUNDS: usize = 12;
const BLOCK_BYTES: usize = 8;

#[actix_web::post("/ping")]
pub async fn ping() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().body("pong"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(ping))
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
