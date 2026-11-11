//! Supabase Authentication & Loopback Callback Engine for Smart-Filee.
//!
//! Provides publishable-key guarded Supabase Auth REST client and the local HTTP loopback
//! server on `127.0.0.1:8765` for one-time token_hash email confirmation.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

pub const DEFAULT_SUPABASE_URL: &str = "https://fyuzbacxqbrqkmeubwdb.supabase.co";
pub const DEFAULT_SUPABASE_PUBLISHABLE_KEY: &str = "sb_publishable_7G5QJEkOp2Tgx-3RSAh3kA_POSeexNE";

/// Validated configuration for the Supabase Auth client.
#[derive(Debug, Clone)]
pub struct SupabaseConfig {
    pub url: String,
    pub publishable_key: String,
}

impl SupabaseConfig {
    /// Validates and constructs `SupabaseConfig`. Strictly forbids secret / service_role keys.
    pub fn new(url: &str, publishable_key: &str) -> anyhow::Result<Self> {
        let trimmed_url = url.trim().trim_end_matches('/');
        let trimmed_key = publishable_key.trim();

        if trimmed_url.is_empty() || trimmed_key.is_empty() {
            anyhow::bail!(
                "Supabase is not configured. Set SUPABASE_URL and SUPABASE_PUBLISHABLE_KEY in .env."
            );
        }

        if !trimmed_url.starts_with("https://") || !trimmed_url.contains(".supabase.co") {
            anyhow::bail!("SUPABASE_URL must be the HTTPS URL for your Supabase project.");
        }

        let key_lower = trimmed_key.to_ascii_lowercase();
        if trimmed_key.starts_with("sb_secret_") || key_lower.contains("service_role") {
            anyhow::bail!(
                "A secret/service_role key was detected. Use only the Supabase publishable key."
            );
        }

        Ok(Self {
            url: trimmed_url.to_string(),
            publishable_key: trimmed_key.to_string(),
        })
    }

    /// Loads configuration from `SUPABASE_URL` and `SUPABASE_PUBLISHABLE_KEY` environment variables.
    pub fn from_env() -> anyhow::Result<Self> {
        let url = std::env::var("SUPABASE_URL").unwrap_or_default();
        let key = std::env::var("SUPABASE_PUBLISHABLE_KEY")
            .or_else(|_| std::env::var("SUPABASE_ANON_KEY"))
            .unwrap_or_default();
        Self::new(&url, &key)
    }
}

/// User profile details returned by Supabase Auth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub avatar_url: String,
    pub role: String,
}

/// Active user session. Kept strictly in memory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthSession {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub user: AuthUser,
}

/// Result received by the local email confirmation loopback listener.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailConfirmationResult {
    pub token_hash: Option<String>,
    pub otp_type: Option<String>,
    pub error: Option<String>,
}

/// Synchronous REST client for Supabase Auth and database operations.
#[derive(Debug)]
pub struct SupabaseClient {
    config: SupabaseConfig,
    client: reqwest::blocking::Client,
}

impl SupabaseClient {
    pub fn new(config: SupabaseConfig) -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .unwrap_or_default();

        Self { config, client }
    }

    /// Authenticates with email and password via GoTrue `/auth/v1/token?grant_type=password`.
    pub fn sign_in_with_password(
        &self,
        email: &str,
        password: &str,
    ) -> anyhow::Result<AuthSession> {
        let endpoint = format!("{}/auth/v1/token?grant_type=password", self.config.url);
        let mut body = HashMap::new();
        body.insert("email", email);
        body.insert("password", password);

        let resp = self
            .client
            .post(&endpoint)
            .header("apikey", &self.config.publishable_key)
            .json(&body)
            .send()?;

        if !resp.status().is_success() {
            let err_text = resp.text().unwrap_or_default();
            anyhow::bail!("Sign in failed: {err_text}");
        }

        let json_val: serde_json::Value = resp.json()?;
        let access_token = json_val["access_token"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing access_token"))?
            .to_string();

        let refresh_token = json_val["refresh_token"].as_str().map(ToString::to_string);
        let user_val = &json_val["user"];

        let user = AuthUser {
            id: user_val["id"].as_str().unwrap_or_default().to_string(),
            email: user_val["email"].as_str().unwrap_or_default().to_string(),
            display_name: user_val["user_metadata"]["display_name"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            avatar_url: user_val["user_metadata"]["avatar_url"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            role: "user".to_string(),
        };

        Ok(AuthSession {
            access_token,
            refresh_token,
            user,
        })
    }

    /// Registers a new user with email and password via GoTrue `/auth/v1/signup`.
    pub fn sign_up_with_password(&self, email: &str, password: &str) -> anyhow::Result<AuthUser> {
        let endpoint = format!("{}/auth/v1/signup", self.config.url);
        let mut body = HashMap::new();
        body.insert("email", email);
        body.insert("password", password);

        let resp = self
            .client
            .post(&endpoint)
            .header("apikey", &self.config.publishable_key)
            .json(&body)
            .send()?;

        if !resp.status().is_success() {
            let err_text = resp.text().unwrap_or_default();
            anyhow::bail!("Sign up failed: {err_text}");
        }

        let json_val: serde_json::Value = resp.json()?;
        let user_val = if json_val.get("user").is_some() {
            &json_val["user"]
        } else {
            &json_val
        };

        Ok(AuthUser {
            id: user_val["id"].as_str().unwrap_or_default().to_string(),
            email: user_val["email"].as_str().unwrap_or_default().to_string(),
            display_name: user_val["user_metadata"]["display_name"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            avatar_url: user_val["user_metadata"]["avatar_url"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            role: "user".to_string(),
        })
    }

    /// Updates the current user's profile display name.
    pub fn update_profile(&self, access_token: &str, display_name: &str) -> anyhow::Result<()> {
        let endpoint = format!("{}/auth/v1/user", self.config.url);
        let mut meta = HashMap::new();
        meta.insert("display_name", display_name);
        let mut body = HashMap::new();
        body.insert("data", meta);

        let resp = self
            .client
            .put(&endpoint)
            .header("apikey", &self.config.publishable_key)
            .header("Authorization", format!("Bearer {access_token}"))
            .json(&body)
            .send()?;

        if !resp.status().is_success() {
            let err_text = resp.text().unwrap_or_default();
            anyhow::bail!("Update profile failed: {err_text}");
        }

        Ok(())
    }

    /// Verifies single-use `token_hash` from email confirmation callback.
    pub fn verify_otp(&self, token_hash: &str, otp_type: &str) -> anyhow::Result<AuthSession> {
        let endpoint = format!("{}/auth/v1/verify", self.config.url);
        let mut body = HashMap::new();
        body.insert("token_hash", token_hash);
        body.insert("type", otp_type);

        let resp = self
            .client
            .post(&endpoint)
            .header("apikey", &self.config.publishable_key)
            .json(&body)
            .send()?;

        if !resp.status().is_success() {
            let err_text = resp.text().unwrap_or_default();
            anyhow::bail!("OTP verification failed: {err_text}");
        }

        let json_val: serde_json::Value = resp.json()?;
        let access_token = json_val["access_token"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let refresh_token = json_val["refresh_token"].as_str().map(ToString::to_string);
        let user_val = &json_val["user"];

        let user = AuthUser {
            id: user_val["id"].as_str().unwrap_or_default().to_string(),
            email: user_val["email"].as_str().unwrap_or_default().to_string(),
            display_name: user_val["user_metadata"]["display_name"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            avatar_url: user_val["user_metadata"]["avatar_url"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            role: "user".to_string(),
        };

        Ok(AuthSession {
            access_token,
            refresh_token,
            user,
        })
    }
}

/// Local HTTP loopback server on `127.0.0.1:8765` for email verification redirect.
#[derive(Debug)]
pub struct EmailConfirmationListener {
    state: String,
    result: Arc<Mutex<Option<EmailConfirmationResult>>>,
    running: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl EmailConfirmationListener {
    pub const HOST: &'static str = "127.0.0.1";
    pub const PORT: u16 = 8765;

    pub fn start() -> anyhow::Result<Self> {
        // Generate pseudo-random urlsafe state
        let random_bytes: [u8; 16] = std::array::from_fn(|_| {
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_or(0, |d| d.as_nanos());
            (now & 0xFF) as u8
        });
        let state = format!("{:x}", u128::from_ne_bytes(random_bytes));

        let listener = TcpListener::bind(format!("{}:{}", Self::HOST, Self::PORT))?;
        listener.set_nonblocking(true)?;

        let result = Arc::new(Mutex::new(None));
        let running = Arc::new(AtomicBool::new(true));

        let result_clone = Arc::clone(&result);
        let running_clone = Arc::clone(&running);
        let expected_path = format!("/auth/callback/{state}");

        let handle = thread::spawn(move || {
            while running_clone.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let mut buffer = [0u8; 4096];
                        if let Ok(n) = stream.read(&mut buffer) {
                            let req = String::from_utf8_lossy(&buffer[..n]);
                            if let Some(first_line) = req.lines().next() {
                                let parts: Vec<&str> = first_line.split_whitespace().collect();
                                if parts.len() >= 2 && parts[0] == "GET" {
                                    let target_url = parts[1];
                                    let (path, query) = match target_url.split_once('?') {
                                        Some((p, q)) => (p, q),
                                        None => (target_url, ""),
                                    };

                                    if path == expected_path {
                                        let mut token_hash = None;
                                        let mut otp_type = None;
                                        let mut error = None;

                                        for pair in query.split('&') {
                                            if let Some((k, v)) = pair.split_once('=') {
                                                match k {
                                                    "token_hash" => {
                                                        token_hash = Some(v.to_string());
                                                    }
                                                    "type" => otp_type = Some(v.to_string()),
                                                    "error_description" | "error" => {
                                                        error = Some(v.to_string());
                                                    }
                                                    _ => {}
                                                }
                                            }
                                        }

                                        let parsed_res = EmailConfirmationResult {
                                            token_hash,
                                            otp_type,
                                            error,
                                        };

                                        let mut lock = result_clone.lock().unwrap();
                                        if lock.is_none() {
                                            *lock = Some(parsed_res);
                                        }

                                        let html = "<!doctype html><html><head><meta charset='utf-8'><title>Smart-Filee</title></head><body><h2>Email confirmation received</h2><p>Return to Smart File Manager Pro.</p></body></html>";
                                        let resp = format!(
                                            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                                            html.len(),
                                            html
                                        );
                                        let _ = stream.write_all(resp.as_bytes());
                                        let _ = stream.flush();
                                    } else {
                                        let resp = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                                        let _ = stream.write_all(resp.as_bytes());
                                        let _ = stream.flush();
                                    }
                                }
                            }
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(20));
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(Self {
            state,
            result,
            running,
            handle: Some(handle),
        })
    }

    /// Constructs the redirect URL that Supabase confirmation emails should open.
    pub fn redirect_url(&self) -> String {
        format!(
            "http://{}:{}/auth/callback/{}",
            Self::HOST,
            Self::PORT,
            self.state
        )
    }

    /// Retrieves and clears the received confirmation result.
    pub fn take_result(&self) -> Option<EmailConfirmationResult> {
        let mut lock = self.result.lock().unwrap();
        lock.take()
    }

    /// Shuts down the local HTTP listener.
    pub fn close(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for EmailConfirmationListener {
    fn drop(&mut self) {
        self.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supabase_config_safety_checks() {
        assert!(
            SupabaseConfig::new(DEFAULT_SUPABASE_URL, DEFAULT_SUPABASE_PUBLISHABLE_KEY).is_ok()
        );

        // Secret key rejection
        assert!(SupabaseConfig::new(DEFAULT_SUPABASE_URL, "sb_secret_abcdef123456").is_err());

        // Non-HTTPS rejection
        assert!(SupabaseConfig::new(
            "http://fyuzbacxqbrqkmeubwdb.supabase.co",
            DEFAULT_SUPABASE_PUBLISHABLE_KEY
        )
        .is_err());
    }

    #[test]
    fn test_email_confirmation_listener_flow() {
        let mut listener = EmailConfirmationListener::start().unwrap();
        let redirect = listener.redirect_url();

        let client = reqwest::blocking::Client::new();

        // 1. Valid request to correct state path
        let test_url = format!("{redirect}?token_hash=test_token_123&type=email");
        let resp = client.get(&test_url).send().unwrap();
        assert_eq!(resp.status(), 200);
        let body = resp.text().unwrap();
        assert!(body.contains("Email confirmation received"));

        let res = listener.take_result().expect("Result should be captured");
        assert_eq!(res.token_hash.as_deref(), Some("test_token_123"));
        assert_eq!(res.otp_type.as_deref(), Some("email"));

        // Second read must be None
        assert!(listener.take_result().is_none());

        // 2. Request to wrong state path
        let wrong_url = "http://127.0.0.1:8765/auth/callback/wrong-state?token_hash=bad";
        let resp_wrong = client.get(wrong_url).send().unwrap();
        assert_eq!(resp_wrong.status(), 404);

        listener.close();
    }
}
