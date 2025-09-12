use bitvmx_client::bitcoin::{secp256k1, Address, Network, PublicKey, ScriptBuf, XOnlyPublicKey};
use bitvmx_client::protocol_builder::scripts::{self, ProtocolScript};

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
    Ok(pubkey.inner.into())
}
