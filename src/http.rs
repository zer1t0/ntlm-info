use crate::auth;
use crate::challenge::Challenge;
use ntlm::ChallengeMsg;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use std::time::Duration;

const AUTH_HEADER: &'static str = "www-authenticate";

#[derive(Clone, Debug, Copy)]
pub struct HttpOptions {
    pub timeout: Duration,
}

pub fn challenge_http(
    url: &str,
    options: HttpOptions,
) -> Result<Challenge, String> {
    let client = Client::new();
    let neg_b64 = base64::encode(auth::new_ntlm_negotiate().build());

    let resp = client
        .get(url)
        .timeout(options.timeout)
        .header("Authorization", format!("NTLM {}", neg_b64))
        .send()
        .map_err(|e| format!("Error requesting {}: {}", url, e))?;

    let ntlm_challenge = extract_challenge(&resp)?;

    return Ok(Challenge::new(url.into(), ntlm_challenge));
}

fn extract_challenge(resp: &Response) -> Result<ChallengeMsg, String> {
    let auth_header = resp.headers().get(AUTH_HEADER).ok_or(format!(
        "No NTLM challenge in HTTP response (no {} header)",
        AUTH_HEADER
    ))?;

    let auth_header = auth_header
        .to_str()
        .map_err(|_| format!("Error decoding NTLM challenge"))?;

    if !auth_header.contains("NTLM") {
        return Err(format!(
            "No NTLM challenge in HTTP response (Not supported)"
        ));
    }

    let parts: Vec<&str> = auth_header.split(" ").collect();

    if parts.len() < 2 {
        return Err(format!("Error decoding NTLM challenge"));
    }

    let challenge_b64 = parts[1];

    let challenge_raw = base64::decode(challenge_b64)
        .map_err(|_| format!("Error decoding NTLM challenge"))?;

    let challenge = ChallengeMsg::parse(&challenge_raw)
        .map_err(|_| format!("Error decoding NTLM challenge"))?;

    return Ok(challenge);
}
