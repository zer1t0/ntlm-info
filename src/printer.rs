use crate::challenge::Challenge;
use log::info;
use ntlm::ChallengeMsg;
use serde::{Deserialize, Serialize};
use std::fs::File;

pub struct Output {
    challenges: Vec<Challenge>,
    out_file: Option<String>,
}

impl Output {
    pub fn new(out_file: Option<String>) -> Self {
        return Self {
            challenges: Vec::new(),
            out_file,
        };
    }
}

impl Output {
    pub fn add(&mut self, ch: Challenge) {
        print_challenge(&ch);
        self.challenges.push(ch);
    }

    pub fn finish(&self) -> Result<(), String> {
        if let Some(out_file) = &self.out_file {
            save_challenges(out_file, &self.challenges)?;
        }

        return Ok(());
    }
}

#[derive(Serialize, Deserialize)]
struct JsonChallenge {
    pub target: String,
    pub nb_computer: Option<String>,
    pub nb_domain: Option<String>,
    pub dns_computer: Option<String>,
    pub dns_domain: Option<String>,
    pub dns_tree: Option<String>,
    pub version: Option<String>,
    pub os_names: Option<Vec<String>>,
}

impl JsonChallenge {
    fn from_challenge(ch: &Challenge) -> Self {
        Self {
            target: ch.target.to_string(),
            nb_computer: ch.challenge.nb_computer_name().map(|v| v.clone()),
            nb_domain: ch.challenge.nb_domain_name().map(|v| v.clone()),
            dns_computer: ch.challenge.dns_computer_name().map(|v| v.clone()),
            dns_domain: ch.challenge.dns_domain_name().map(|v| v.clone()),
            dns_tree: ch.challenge.dns_tree_name().map(|v| v.clone()),
            version: ch
                .challenge
                .version
                .as_ref()
                .map(|v| format!("{}.{}.{}", v.major, v.minor, v.build)),
            os_names: ch
                .challenge
                .version
                .as_ref()
                .map(|v| v.os_names().iter().map(|s| s.to_string()).collect()),
        }
    }
}

fn save_challenges(out_file: &str, chs: &Vec<Challenge>) -> Result<(), String> {
    let mut json_chs = Vec::new();
    for ch in chs {
        json_chs.push(JsonChallenge::from_challenge(&ch));
    }

    let file = File::create(out_file)
        .map_err(|e| format!("Error opening '{}': {}", out_file, e))?;
    serde_json::to_writer(&file, &json_chs).map_err(|e| {
        format!("Error writing JSON data in '{}': {}", out_file, e)
    })?;

    info!("Save challenges in '{}'", out_file);

    return Ok(());
}

pub fn print_challenge(ch: &Challenge) {
    println!(
        "\nTarget: {}\n{}",
        ch.target,
        ntlm_challenge_to_string(&ch.challenge)
    );
}

fn ntlm_challenge_to_string(nt_ch: &ChallengeMsg) -> String {
    let mut msg = Vec::new();
    if let Some(nb_cn) = nt_ch.nb_computer_name() {
        msg.push(format!("NbComputer: {}", nb_cn));
    }

    if let Some(nb_dn) = nt_ch.nb_domain_name() {
        msg.push(format!("NbDomain: {}", nb_dn));
    }

    if let Some(dns_cn) = nt_ch.dns_computer_name() {
        msg.push(format!("DnsComputer: {}", dns_cn));
    }

    if let Some(dns_dn) = nt_ch.dns_domain_name() {
        msg.push(format!("DnsDomain: {}", dns_dn));
    }

    if let Some(dns_fn) = nt_ch.dns_tree_name() {
        msg.push(format!("DnsTree: {}", dns_fn));
    }

    if let Some(version) = &nt_ch.version {
        msg.push(format!(
            "Version: {}.{}.{}",
            version.major, version.minor, version.build
        ));

        let names = version.os_names();
        if names.len() > 0 {
            msg.push(format!("OS: {}", names.join(" | ")))
        }
    }

    return msg.join("\n");
}
