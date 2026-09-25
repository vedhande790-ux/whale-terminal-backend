use axum::{
    extract::{Query, State},
    response::Json,
};
use rand::Rng;
use serde::Deserialize;
use std::sync::Arc;

use crate::AppState;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct SupabaseLicense {
    key:        String,
    device_id:  Option<String>,
    expires_at: Option<String>,
    created_at: Option<String>,
}

fn supabase_base() -> String {
    let raw = std::env::var("SUPABASE_URL").expect("SUPABASE_URL not set");
    let s = raw.trim_end_matches('/');
    let s = s.strip_suffix("/rest/v1").unwrap_or(s);
    let s = s.trim_end_matches('/');
    let base = format!("{}/rest/v1", s);
    println!("[supabase] base = {}", base);
    base
}

fn supabase_key() -> String {
    std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY not set")
}

fn url_encode(s: &str) -> String {
    s.chars().map(|c| match c {
        'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
        _ => format!("%{:02X}", c as u32),
    }).collect()
}

async fn db_insert_license(
    client: &reqwest::Client,
    key: &str,
    expires_at: Option<&str>,
) -> Result<(), String> {
    let url = format!("{}/licenses", supabase_base());
    let body = serde_json::json!({
        "key": key,
        "device_id": serde_json::Value::Null,
        "expires_at": expires_at,
    });

    println!("[supabase] POST {}", url);
    println!("[supabase] body = {}", body);

    let res = client
        .post(&url)
        .header("apikey", supabase_key())
        .header("Authorization", format!("Bearer {}", supabase_key()))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=minimal")
        .body(body.to_string())
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = res.status();
    let text = res.text().await.unwrap_or_default();
    println!("[supabase] insert response {} — {}", status, text);

    if status.is_success() {
        Ok(())
    } else {
        Err(format!("insert failed: {} — {}", status, text))
    }
}

async fn db_lookup_license(
    client: &reqwest::Client,
    key: &str,
) -> Result<Option<SupabaseLicense>, String> {
    let variants = vec![
        key.to_string(),
        key.to_uppercase(),
        key.to_lowercase(),
    ];

    for variant in &variants {
        let url = format!(
            "{}/licenses?key=eq.{}&limit=1",
            supabase_base(),
            urlencoding::encode(variant)
        );

        println!("[supabase] GET {}", url);

        let res = client
            .get(&url)
            .header("apikey", supabase_key())
            .header("Authorization", format!("Bearer {}", supabase_key()))
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        println!("[supabase] lookup response {} — {}", status, text);

        if !status.is_success() {
            return Err(format!("lookup failed: {} — {}", status, text));
        }

        let rows: Vec<SupabaseLicense> = serde_json::from_str(&text)
            .map_err(|e| format!("parse error: {e} — raw: {text}"))?;

        if let Some(row) = rows.into_iter().next() {
            return Ok(Some(row));
        }
    }

    Ok(None)
}

#[derive(Deserialize)]
pub struct ValidateParams { key: String, device: String }

pub async fn h_validate(
    State(s): State<Arc<AppState>>,
    Query(p): Query<ValidateParams>,
    _headers: axum::http::HeaderMap,
) -> Json<serde_json::Value> {
    let key = p.key.trim().to_string();
    if key.len() < 8 {
        return Json(serde_json::json!({"valid":false,"status":"INVALID","expires":"","warning":""}));
    }
    match db_lookup_license(&s.http_client, &key).await {
        Ok(Some(rec)) => {
            let expired = rec.expires_at.as_deref().map(|exp| {
                if exp == "LIFETIME" { return false; }
                chrono::NaiveDate::parse_from_str(exp, "%Y-%m-%d")
                    .map(|d| d < chrono::Utc::now().naive_utc().date())
                    .unwrap_or(true)
            }).unwrap_or(false);
            if expired {
                Json(serde_json::json!({"valid":false,"status":"EXPIRED","expires":rec.expires_at,"warning":""}))
            } else {
                Json(serde_json::json!({"valid":true,"status":"ACTIVE","expires":rec.expires_at,"warning":""}))
            }
        }
        Ok(None) => Json(serde_json::json!({"valid":false,"status":"INVALID","expires":"","warning":""})),
        Err(e) => {
            eprintln!("[license] validate error: {e}");
            Json(serde_json::json!({"valid":false,"status":"ERROR","expires":"","warning":""}))
        }
    }
}

pub async fn h_payment_info() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "networks": [
            { "name": "USDT TRC20 ", "address": "TJ82R2Yqq11KNUudbYj4JPCPggeEseztKi" },
            { "name": "USDT ERC20 ", "address": "0x8d2bc6fc63f04464016e382ed3670c1dec8f746e" }
        ],
        "plans": [
            { "name": "Monthly", "price_usd": 35,  "label": "$35 / month" },
            { "name": "Yearly",  "price_usd": 380, "label": "$380 / year — Best value (save $40)" }
        ],
        "instructions": "HOW TO GET ACCESS:\n\
1. Send USDT to one of the addresses above\n\
2. Copy your transaction hash (TX ID)\n\
3. Message me on Telegram with:\n\
   - TX hash\n\
   - Screenshot of the transaction\n\
4. You will receive your license key within 5–30 minutes after confirmation\n\
\n\
IMPORTANT:\n\
- Send the exact amount\n\
- Make sure you use the correct network (TRC20 or ERC20)\n\
- Double-check the address before sending",
        "telegram": "@Rust0xDev",
        "support": "Need help? Message me on Telegram — I usually respond within minutes."
    }))
}

#[derive(Deserialize)]
pub struct GenKeyParams { secret: String, expires: Option<String> }

pub async fn h_gen_key(
    State(s): State<Arc<AppState>>,
    Query(p): Query<GenKeyParams>,
) -> Json<serde_json::Value> {
    let admin_secret = std::env::var("ADMIN_SECRET")
        .unwrap_or_else(|_| "CHANGE_THIS_BEFORE_USE".into());
    if p.secret != admin_secret {
        return Json(serde_json::json!({"error":"unauthorized"}));
    }
    let suffix: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(6).map(char::from).collect::<String>().to_uppercase();
    let today  = chrono::Utc::now().format("%Y-%m-%d");
    let key    = format!("Whale-PRO-{today}-{suffix}");
    let expires = p.expires.as_deref();

    match db_insert_license(&s.http_client, &key, expires).await {
        Ok(()) => {
            println!("[admin] key={key} expires={:?}", expires);
            Json(serde_json::json!({"key": key, "expires": expires}))
        }
        Err(e) => {
            eprintln!("[admin] insert error: {e}");
            Json(serde_json::json!({"error": "failed to store key", "detail": e}))
        }
    }
}
