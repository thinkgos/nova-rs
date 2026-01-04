use chrono::Duration;
use serde::{Deserialize, Serialize};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredClaims<T> {
    /// the `iss` (Issuer) claim. See https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.1
    #[serde(skip_serializing_if = "Option::is_none")]
    iss: Option<String>,
    /// the `sub` (Subject) claim. See https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.2
    sub: String,
    /// the `aud` (Audience) claim. See https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.3
    #[serde(skip_serializing_if = "Option::is_none")]
    aud: Option<String>,
    /// the `exp` (Expiration Time) claim. See https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.4
    exp: usize,
    /// the `nbf` (Not Before) claim. See https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.5
    #[serde(skip_serializing_if = "Option::is_none")]
    nbf: Option<usize>,
    /// the `iat` (Issued At) claim. See https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.6
    iat: usize,
    /// the `jti` (JWT ID) claim. See https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.7
    #[serde(skip_serializing_if = "Option::is_none")]
    jti: Option<String>,
    /// meta data
    meta: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// 用户名
    username: String,
    /// 组织ID
    org_id: String,
    /// 敏感信息
    sensitive: String,
}

fn main() -> Result<(), anyhow::Error> {
    let now = chrono::Utc::now();
    let exp = (now + Duration::hours(24)).timestamp() as usize;
    let iat = now.timestamp() as usize;

    let claims = RegisteredClaims {
        iss: None,
        sub: "1".to_string(),
        aud: None,
        exp,
        nbf: None,
        iat,
        jti: None,
        meta: Metadata {
            username: "admin".to_string(),
            org_id: "11111".to_string(),
            sensitive: "sensitive".to_string(),
        },
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )?;
    println!("claims: {}", token);

    let validation = Validation::new(Algorithm::HS256);

    // `token` is a struct with 2 fields: `header` and `claims` where `claims` is your own struct.
    let token_data = decode::<RegisteredClaims<Metadata>>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &validation,
    )?;
    println!("claims: {:?}", token_data.claims);

    Ok(())
}
