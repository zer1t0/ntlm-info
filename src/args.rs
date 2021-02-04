use clap::ArgMatches;
use clap::{App, AppSettings, Arg, SubCommand};
use std::time::Duration;

const HTTP_COM: &'static str = "http";
const SMB_COM: &'static str = "smb";

fn args() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandRequired)
        .subcommand(smb_command())
        .subcommand(http_command())
}

fn smb_command() -> App<'static, 'static> {
    SubCommand::with_name(SMB_COM)
        .about("Use SMB to retrieve the NTLM challenge")
        .arg(
            Arg::with_name("target")
                .takes_value(true)
                .multiple(true)
                .value_name("host/range")
                .help("The hosts to retrieve NTLM information. If none, stdin is used."),
        )
        .arg(
            Arg::with_name("timeout")
                .long("timeout")
                .short("t")
                .help("Timeout in milliseconds")
                .takes_value(true)
                .default_value("10000")
                .value_name("millis")
                .validator(is_usize_major_than_zero),
        )
        .arg(
            Arg::with_name("json")
                .long("json")
                .short("j")
                .takes_value(true)
                .help("File to save output in json format")
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Verbosity"),
        )
        .arg(
            Arg::with_name("workers")
                .long("workers")
                .short("w")
                .help("Number of parallel workers")
                .takes_value(true)
                .default_value("1")
                .value_name("n")
                .validator(is_usize_major_than_zero),
        )
}

fn http_command() -> App<'static, 'static> {
    SubCommand::with_name(HTTP_COM)
        .about("Use HTTP to retrieve the NTLM challenge")
        .arg(
            Arg::with_name("url")
                .takes_value(true)
                .multiple(true)
                .value_name("url")
               .help("The HTTP endpoint to retrieve NTLM information. If none, stdin is used."),
        )
        .arg(
            Arg::with_name("timeout")
                .long("timeout")
                .short("t")
                .help("Timeout in milliseconds")
                .takes_value(true)
                .default_value("10000")
                .value_name("millis")
                .validator(is_usize_major_than_zero),
        )
        .arg(
            Arg::with_name("json")
                .long("json")
                .short("j")
                .takes_value(true)
                .help("File to save output in json format")
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Verbosity"),
        )
        .arg(
            Arg::with_name("workers")
                .long("workers")
                .short("w")
                .help("Number of parallel workers")
                .takes_value(true)
                .default_value("1")
                .value_name("n")
                .validator(is_usize_major_than_zero),
        )
}

fn is_usize_major_than_zero(v: String) -> Result<(), String> {
    match v.parse::<usize>() {
        Ok(uint) => {
            if uint == 0 {
                return Err(
                    "Must be a positive integer bigger than 0".to_string()
                );
            }
            Ok(())
        }
        Err(_) => Err("Must be a positive integer bigger than 0".to_string()),
    }
}

pub enum Args {
    Http(HttpArgs),
    Smb(SmbArgs),
}

impl Args {
    pub fn parse_args() -> Self {
        let matches = args().get_matches();

        match matches.subcommand_name().unwrap() {
            HTTP_COM => {
                return Self::Http(HttpArgs::parse_args(
                    matches.subcommand_matches(HTTP_COM).unwrap(),
                ))
            }
            SMB_COM => {
                return Self::Smb(SmbArgs::parse_args(
                    matches.subcommand_matches(SMB_COM).unwrap(),
                ))
            }
            _ => unreachable!("Invalid command"),
        }
    }
}

pub struct HttpArgs {
    pub json: Option<String>,
    pub timeout: Duration,
    pub urls: Vec<String>,
    pub verbosity: usize,
    pub workers: usize,
}

impl HttpArgs {
    fn parse_args(matches: &ArgMatches) -> Self {
        return Self {
            json: parse_json(matches),
            urls: parse_urls(&matches),
            timeout: parse_timeout(&matches),
            verbosity: matches.occurrences_of("verbosity") as usize,
            workers: parse_workers(matches),
        };
    }
}

pub struct SmbArgs {
    pub json: Option<String>,
    pub targets: Vec<String>,
    pub timeout: Duration,
    pub verbosity: usize,
    pub workers: usize,
}

impl SmbArgs {
    fn parse_args(matches: &ArgMatches) -> Self {
        return Self {
            json: parse_json(matches),
            targets: parse_targets(matches),
            timeout: parse_timeout(matches),
            verbosity: matches.occurrences_of("verbosity") as usize,
            workers: parse_workers(matches),
        };
    }
}

fn parse_urls(matches: &ArgMatches) -> Vec<String> {
    return parse_vec_strings(matches, "url");
}

fn parse_workers(matches: &ArgMatches) -> usize {
    return matches.value_of("workers").unwrap().parse().unwrap();
}

fn parse_timeout(matches: &ArgMatches) -> Duration {
    let timeout_secs: usize =
        matches.value_of("timeout").unwrap().parse().unwrap();

    return Duration::from_millis(timeout_secs as u64);
}

fn parse_json(matches: &ArgMatches) -> Option<String> {
    return matches.value_of("json").map(|s| s.to_string());
}

fn parse_targets(matches: &ArgMatches) -> Vec<String> {
    return parse_vec_strings(matches, "target");
}

fn parse_vec_strings(matches: &ArgMatches, name: &str) -> Vec<String> {
    match matches.values_of(name) {
        None => Vec::new(),
        Some(urls) => urls.map(|u| u.to_string()).collect(),
    }
}
