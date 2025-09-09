use bitvmx_client::bitcoin::{secp256k1, Address, Network, PublicKey, ScriptBuf, XOnlyPublicKey};
use bitvmx_client::protocol_builder::scripts::{self, ProtocolScript};
use std::str::FromStr;

pub fn pub_key_to_p2tr(
    x_public_key: &XOnlyPublicKey,
    tap_leaves: &[ProtocolScript],
) -> Result<Address, anyhow::Error> {
    let tap_spend_info =
        scripts::build_taproot_spend_info(&secp256k1::Secp256k1::new(), x_public_key, tap_leaves)?;
    let script = ScriptBuf::new_p2tr_tweaked(tap_spend_info.output_key());
    let address = Address::from_script(&script, Network::Regtest)?;
    Ok(address)
}

pub fn pub_key_to_xonly(pubkey: &PublicKey) -> Result<XOnlyPublicKey, anyhow::Error> {
    // XOnlyPublicKey should be always even parity, compact pubkey should prefix with 02 if even or 03 if odd
    Ok(XOnlyPublicKey::from_str(
        pubkey.to_string().strip_prefix("02").unwrap(),
    )?)
}

pub fn xonly_to_pub_key(x_only_pubkey: &XOnlyPublicKey) -> Result<PublicKey, anyhow::Error> {
    // XOnlyPublicKey should be always even parity, compact pubkey should prefix with 02 if even or 03 if odd
    Ok(PublicKey::from_str(format!("02{x_only_pubkey}").as_str())?)
}
