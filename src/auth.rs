use ntlm::flags as ntflag;
use ntlm::{ChallengeMsg, NegotiateMsg, Version};
use spnego::{ntlmssp_oid, NegToken};

pub fn new_ntlm_negotiate() -> NegotiateMsg {
    let mut ntlm_neg = NegotiateMsg::default();
    ntlm_neg.flags = ntflag::NTLM_NEG_56
        | ntflag::NTLM_NEG_KEY_EXCH
        | ntflag::NTLM_NEG_128
        | ntflag::NTLM_NEG_VERSION
        | ntflag::NTLM_NEG_EXTENDED_SECURITY
        | ntflag::NTLM_NEG_ALWAYS_SIGN
        | ntflag::NTLM_NEG_NTLM
        | ntflag::NTLM_NEG_LM_KEY
        | ntflag::NTLM_NEG_SIGN
        | ntflag::NTLM_REQUEST_TARGET
        | ntflag::NTLM_NEG_OEM
        | ntflag::NTLM_NEG_UNICODE;

    ntlm_neg.version = Some(Version::windows7_7601());

    return ntlm_neg;
}

pub fn new_spnego_init2() -> NegToken {
    let ntlm_neg = new_ntlm_negotiate();

    let mut init2 = spnego::NegTokenInit2::default();
    init2.mech_types = vec![ntlmssp_oid()];
    init2.mech_token = Some(ntlm_neg.build());

    return NegToken::Init2(init2);
}

pub fn extract_ntlm_challenge(raw: &[u8]) -> Result<ChallengeMsg, String> {
    let spnego_msg = NegToken::parse(raw)
        .map_err(|_| format!("Error parsing spnego response"))?;

    let spnego_resp = match spnego_msg {
        spnego::NegToken::Resp(resp) => resp,
        _ => return Err(format!("Unexpected spnego response")),
    };

    let raw_ntlm_challenge = spnego_resp
        .response_token
        .ok_or(format!("No NTLM challenge in SMB2 response"))?;

    let ntlm_challenge = ntlm::ChallengeMsg::parse(&raw_ntlm_challenge)
        .map_err(|_| format!("Error parsing NTLM challenge"))?;

    return Ok(ntlm_challenge);
}
