# RC5-CBC File Encryptor API

A simple Actix-Web server implementing RC5 block cipher in CBC mode. It exposes REST endpoints to encrypt and decrypt text using a passphrase, writing encrypted data to disk and returning decrypted plaintext.

## Features

- **POST /encrypt**: Encrypts provided text with RC5-32/12 in CBC mode, writes ciphertext to a file.
- **POST /decrypt**: Reads ciphertext from a file, decrypts with RC5-32/12 in CBC mode, returns plaintext.
- **GET /ping**: Health check endpoint returning `pong`.

## Requirements

- Rust 1.64+
- Cargo
- `rc5_block_cipher_rust` crate (provides `encrypt`, `decrypt`) in the same workspace or installed
- Actix-Web

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/rc5-cbc-api.git
   cd rc5-cbc-api
   ```

2. Build the server:

   ```bash
   cargo build --release
   ```

## Running

Start the server on port 8080:

```bash
cargo run --release
```

You should see:

```
Server running at http://127.0.0.1:8080
```

## API Endpoints

### Health Check

```http
GET /ping
```

**Response**

- `200 OK` body: `pong`

### Encrypt Text

```http
POST /encrypt
Content-Type: application/json

{
  "passphrase": "my secret",
  "text": "Hello, World!",
  "filename": "out.bin"
}
```

**Behavior**

- Pads `text` to 8-byte blocks
- Encrypts with RC5-32/12 in CBC mode
- Writes ciphertext to `filename`

**Response**

- `200 OK` body: `Encrypted and written to file`

### Decrypt File

```http
POST /decrypt
Content-Type: application/json

{
  "passphrase": "my secret",
  "filename": "out.bin"
}
```

**Behavior**

- Reads ciphertext from `filename`
- Decrypts with RC5-32/12 in CBC mode
- Trims zero padding
- Returns plaintext

**Response**

- `200 OK` JSON:

  ```json
  { "text": "Hello, World!" }
  ```

## Examples

Encrypt via `curl`:

```bash
curl -X POST http://localhost:8080/encrypt \
  -H "Content-Type: application/json" \
  -d '{"passphrase":"secret","text":"Top Secret","filename":"cipher.bin"}'
```

Decrypt via `curl`:

```bash
curl -X POST http://localhost:8080/decrypt \
  -H "Content-Type: application/json" \
  -d '{"passphrase":"secret","filename":"cipher.bin"}'
```

## License

MIT © Your Name
