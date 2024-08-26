use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chacha20poly1305::{
  aead::{Aead, OsRng},
  AeadCore, ChaCha20Poly1305, KeyInit,
};
use chrono::{DateTime, Utc};
use core::{fmt, str};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
const KEY: &[u8] = b"0123456789abcdefghijklmnopqrstuv";

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
  name: String,
  #[serde(rename = "privateAge")]
  age: u8,
  date_of_birth: DateTime<Utc>,
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  skills: Vec<String>,
  state: WorkState,
  #[serde(serialize_with = "b64_encode", deserialize_with = "b64_decode")]
  data: Vec<u8>,
  #[serde_as(as = "DisplayFromStr")]
  sensitive: SensitiveData,
  #[serde_as(as = "Vec<DisplayFromStr>")]
  url: Vec<http::Uri>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "details")]
enum WorkState {
  Working(String),
  OnLeave(DateTime<Utc>),
  Terminated,
}
#[derive(Debug)]
struct SensitiveData(String);

fn main() -> Result<()> {
  let user = User {
    name: "John Doe".to_string(),
    age: 28,
    date_of_birth: DateTime::parse_from_rfc3339("1996-01-01T00:00:00Z")?.into(),
    skills: vec!["Rust".to_string(), "C++".to_string()],
    state: WorkState::Working("Software Engineer".to_string()),
    data: vec![1, 2, 3],
    sensitive: SensitiveData::new("secret data".to_string()),
    url: vec!["https://www.example.com".parse()?],
  };
  let json = serde_json::to_string(&user)?;
  println!("{}", json);

  let user1: User = serde_json::from_str(&json)?;
  println!("{:?}", user1);
  println!("{:?}", user1.url.first());
  Ok(())
}

fn b64_encode<S>(v: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
  S: serde::Serializer,
{
  let encoded = URL_SAFE_NO_PAD.encode(v);
  serializer.serialize_str(&encoded)
}

fn b64_decode<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
  D: serde::Deserializer<'de>,
{
  let encoded: String = String::deserialize(deserializer)?;
  let decoded = URL_SAFE_NO_PAD.decode(encoded.as_bytes()).map_err(serde::de::Error::custom)?;
  Ok(decoded)
}

impl SensitiveData {
  fn new(data: impl Into<String>) -> Self {
    Self(data.into())
  }
}

impl fmt::Display for SensitiveData {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let encrypted = encrypt(self.0.as_bytes());
    write!(f, "{}", encrypted)
  }
}

impl str::FromStr for SensitiveData {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self> {
    let decrypted = decrypt(s)?;
    let decrypted = String::from_utf8(decrypted)?;
    Ok(Self(decrypted))
  }
}

fn encrypt(data: &[u8]) -> String {
  let cipher = ChaCha20Poly1305::new(KEY.into());
  let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
  let ciphertext = cipher.encrypt(&nonce, data).expect("Failed to encrypt data");
  let nonce_ciphertext: Vec<u8> = nonce.into_iter().chain(ciphertext).collect();

  URL_SAFE_NO_PAD.encode(nonce_ciphertext)
}

fn decrypt(encoded: &str) -> Result<Vec<u8>> {
  let decoded = URL_SAFE_NO_PAD.decode(encoded.as_bytes())?;
  let cipher = ChaCha20Poly1305::new(KEY.into());
  let nonce = decoded[..12].into();
  let ciphertext = &decoded[12..];
  let plaintext = cipher.decrypt(nonce, ciphertext).expect("Failed to decrypt data");
  Ok(plaintext)
}
