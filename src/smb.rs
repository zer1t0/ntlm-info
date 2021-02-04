use crate::auth::extract_ntlm_challenge;
use crate::auth::new_spnego_init2;
use crate::challenge::Challenge;
use crate::challenge::Host;
use ntlm::ChallengeMsg;
use smb::smb1::negotiate::SMB_DIA_NT_LM;
use smb::smb1::negotiate::SMB_DIA_SMB_2_002;
use smb::smb1::negotiate::SMB_DIA_SMB_2_QUESTION;
use smb::smb2::negotiate::SMB2_DIA_202;
use smb::smb2::negotiate::SMB2_DIA_210;
use smb::smb2::negotiate::SMB2_DIA_300;
use smb::smb2::Smb2NegResp;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::time::Duration;

use smb::smb1;
use smb::smb2;

use smb2::negotiate::{Smb2NegReq, SMB2_GLOBAL_CAP_ENCRYPTION};
use smb2::session_setup::{
    Smb2SessionSetupReq, SMB2_NEGOTIATE_SIGNING_ENABLED,
};

use smb1::header::flags::{
    SMB_FLAGS_CANONICALIZED_PATHS, SMB_FLAGS_CASE_INSENSITIVE,
};
use smb1::header::{
    SMB_FLAGS2_EXTENDED_SECURITY, SMB_FLAGS2_LONG_NAMES, SMB_FLAGS2_NT_STATUS,
};

use smb::net::{
    send_recv_negotiate2, send_recv_negotiate_msg, send_recv_session_setup2,
};
use smb1::Smb1NegReq;

use crate::dns;
use smb::SmbNegResp;

#[derive(Clone, Debug, Copy)]
pub struct SmbOptions {
    pub timeout: Duration,
    pub port: u16,
}

pub fn fetch_ntlm_challenge_smb(
    host: String,
    options: SmbOptions,
) -> Result<Challenge, String> {
    let host = match host.parse::<IpAddr>() {
        Ok(ip) => Host::new(ip, None),
        Err(_) => {
            let ip = dns::resolve_hostname(&host)?;
            Host::new(ip, Some(host))
        }
    };

    let target_address = SocketAddr::new(host.ip, options.port);
    return Ok(Challenge::new(
        host.into(),
        challenge_smb(&target_address, options.timeout)?,
    ));
}

pub fn challenge_smb(
    addr: &SocketAddr,
    timeout: Duration,
) -> Result<ChallengeMsg, String> {
    let mut stream = TcpStream::connect_timeout(addr, timeout)
        .map_err(|e| format!("Error connecting with '{}': {}", addr, e))?;

    stream
        .set_read_timeout(Some(timeout))
        .expect("Invalid timeout to SMB stream");

    let _ = smb_negotiate(&mut stream).map_err(|e| {
        format!("Error in SMB negotiation with '{}': {}", addr, e)
    })?;

    // let neg_resp = match neg_resp {
    //    SmbNegResp::V2(packet) => packet,
    // };

    smb_negotiate2(&mut stream).map_err(|e| {
        format!("Error in SMB2 negotiation with '{}': {}", addr, e)
    })?;

    return smb_session_setup2(&mut stream);
}

fn smb_negotiate(stream: &mut TcpStream) -> smb::Result<SmbNegResp> {
    let mut neg_req = Smb1NegReq::new();
    neg_req.header.flags =
        SMB_FLAGS_CANONICALIZED_PATHS | SMB_FLAGS_CASE_INSENSITIVE;
    neg_req.header.flags2 = SMB_FLAGS2_NT_STATUS
        | SMB_FLAGS2_LONG_NAMES
        | SMB_FLAGS2_EXTENDED_SECURITY;

    neg_req.body.dialects = vec![
        SMB_DIA_NT_LM.to_string(),
        SMB_DIA_SMB_2_002.to_string(),
        SMB_DIA_SMB_2_QUESTION.to_string(),
    ];

    return send_recv_negotiate_msg(stream, &neg_req);
}

fn smb_negotiate2(stream: &mut TcpStream) -> smb::Result<Smb2NegResp> {
    let mut neg2_req = Smb2NegReq::new();
    neg2_req.header.message_id = 1;

    neg2_req.body.security_mode =
        smb2::negotiate::SMB2_NEGOTIATE_SIGNING_ENABLED;
    neg2_req.body.capabilities = SMB2_GLOBAL_CAP_ENCRYPTION;
    neg2_req.body.client_guid = [
        0x4f, 0x49, 0x7a, 0x4d, 0x6c, 0x4d, 0x4f, 0x43, 0x59, 0x77, 0x69, 0x42,
        0x69, 0x54, 0x67, 0x76,
    ];

    neg2_req.body.dialects = vec![SMB2_DIA_202, SMB2_DIA_210, SMB2_DIA_300];

    return send_recv_negotiate2(stream, &neg2_req);
}

fn smb_session_setup2(stream: &mut TcpStream) -> Result<ChallengeMsg, String> {
    let mut sess_req = Smb2SessionSetupReq::new();
    sess_req.header.message_id = 2;
    sess_req.body.security_mode = SMB2_NEGOTIATE_SIGNING_ENABLED;

    sess_req.body.buffer = new_spnego_init2().build();

    let setup_resp = send_recv_session_setup2(stream, &sess_req)
        .map_err(|_| format!("Error in SMB2 session setup"))?;

    return extract_ntlm_challenge(&setup_resp.body.buffer);
}
