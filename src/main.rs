mod args;
mod auth;
mod challenge;
mod dns;
mod http;
mod printer;
mod readin;
mod smb;

use crate::args::HttpArgs;
use crate::args::SmbArgs;
use crate::challenge::Challenge;
use crate::http::challenge_http;
use crate::http::HttpOptions;
use crate::printer::Output;
use crate::smb::fetch_ntlm_challenge_smb;
use crate::smb::SmbOptions;
use args::Args;
use ipnet::IpNet;
use log::{error, warn};
use readin::read_inputs;
use std::thread;
use stderrlog;

use std::sync::mpsc::{channel, Receiver, Sender};
use threadpool::ThreadPool;

fn init_log(verbosity: usize) {
    stderrlog::new()
        .module(module_path!())
        .verbosity(verbosity)
        .init()
        .unwrap();
}

fn main() {
    let args = Args::parse_args();

    match args {
        Args::Smb(a) => smb_main(a),
        Args::Http(a) => http_main(a),
    }
}

fn http_main(args: HttpArgs) {
    init_log(args.verbosity);

    let options = HttpOptions {
        timeout: args.timeout,
    };

    let out = Output::new(args.json);
    let pool = ThreadPool::new(args.workers);
    let (sc, rc) = channel();

    let out_thread = thread::spawn(move || {
        handle_output(out, rc);
    });

    for url in read_inputs(args.urls, true, true) {
        let sc = sc.clone();
        pool.execute(move || match challenge_http(&url, options) {
            Ok(challenge) => {
                sc.send(challenge).expect("Error sending HTTP challenge");
            }
            Err(err) => {
                warn!("{}", err);
            }
        });
    }

    drop(sc);
    out_thread.join().expect("Error joining output thread");
}

fn smb_main(args: SmbArgs) {
    init_log(args.verbosity);

    let options = SmbOptions {
        timeout: args.timeout,
        port: 445,
    };

    let out = Output::new(args.json);
    let pool = ThreadPool::new(args.workers);
    let (sc, rc) = channel();

    let out_thread = thread::spawn(move || {
        handle_output(out, rc);
    });

    for target in read_inputs(args.targets, true, true) {
        match target.parse::<IpNet>() {
            Ok(net) => {
                for ip in net.hosts() {
                    let sc = sc.clone();
                    pool.execute(move || {
                        smb_do(ip.to_string(), options, sc);
                    });
                }
            }
            Err(_) => {
                let sc = sc.clone();
                pool.execute(move || {
                    smb_do(target, options, sc);
                });
            }
        };
    }

    drop(sc);
    out_thread.join().expect("Error joining output thread");
}

fn smb_do(host: String, options: SmbOptions, sender: Sender<Challenge>) {
    match fetch_ntlm_challenge_smb(host, options) {
        Ok(challenge) => {
            sender.send(challenge).expect("Error sending SMB challenge");
        }
        Err(err) => {
            warn!("{}", err);
        }
    }
}

fn handle_output(mut out: Output, rc: Receiver<Challenge>) {
    loop {
        match rc.recv() {
            Ok(ch) => out.add(ch),
            Err(_) => {
                break;
            }
        }
    }

    if let Err(err) = out.finish() {
        error!("{}", err);
    }
}
