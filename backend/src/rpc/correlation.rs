use bitvmx_client::types::{IncomingBitVMXApiMessages, OutgoingBitVMXApiMessages};
use uuid::Uuid;

/// Convert the transaction name to a correlation ID
pub fn tx_name_to_correlation_id(program_id: &Uuid, name: &str) -> String {
    format!("{program_id}_{name}")
}

/// Convert the message to send to BitVMX to a correlation ID
pub fn request_to_correlation_id(
    message: &IncomingBitVMXApiMessages,
) -> Result<String, anyhow::Error> {
    // Serialize the message
    match message {
        IncomingBitVMXApiMessages::Ping() => Ok("ping".to_string()),
        IncomingBitVMXApiMessages::SetVar(uuid, _key, _value) => Ok(uuid.to_string()),
        IncomingBitVMXApiMessages::SetWitness(uuid, _address, _witness) => Ok(uuid.to_string()),
        IncomingBitVMXApiMessages::SetFundingUtxo(utxo) => {
            Ok(format!("set_funding_utxo_{}_{}", utxo.txid, utxo.vout))
        }
        IncomingBitVMXApiMessages::GetVar(uuid, _key) => Ok(uuid.to_string()),
        IncomingBitVMXApiMessages::GetWitness(uuid, _address) => Ok(uuid.to_string()),
        IncomingBitVMXApiMessages::GetCommInfo() => Ok("get_comm_info".to_string()),
        IncomingBitVMXApiMessages::GetTransaction(uuid, _txid) => Ok(uuid.to_string()),
        IncomingBitVMXApiMessages::GetTransactionInfoByName(uuid, _name) => Ok(uuid.to_string()),
        IncomingBitVMXApiMessages::GetHashedMessage(uuid, _name, _vout, _leaf) => {
            Ok(uuid.to_string())
        }
        IncomingBitVMXApiMessages::Setup(uuid, _program_type, _participants, _leader_idx) => {
            Ok(uuid.to_string())
        }
        IncomingBitVMXApiMessages::SubscribeToTransaction(uuid, _txid) => Ok(uuid.to_string()),
        IncomingBitVMXApiMessages::SubscribeUTXO() => Ok("subscribe_utxo".to_string()),
        IncomingBitVMXApiMessages::SubscribeToRskPegin() => Ok("subscribe_rsk_pegin".to_string()),
        IncomingBitVMXApiMessages::GetSPVProof(_txid) => Ok(format!("spv_proof_{_txid}")),
        IncomingBitVMXApiMessages::DispatchTransaction(uuid, _transaction) => Ok(uuid.to_string()),
        IncomingBitVMXApiMessages::DispatchTransactionName(uuid, name) => {
            Ok(tx_name_to_correlation_id(uuid, name))
        }
        IncomingBitVMXApiMessages::SetupKey(uuid, _addresses, _operator_key, _funding_key) => {
            Ok(uuid.to_string())
        }
        IncomingBitVMXApiMessages::GetAggregatedPubkey(uuid) => Ok(uuid.to_string()),
        IncomingBitVMXApiMessages::GetProtocolVisualization(uuid) => {
            Ok(format!("protocol_visualization_{uuid}"))
        }
        IncomingBitVMXApiMessages::GetKeyPair(uuid) => Ok(uuid.to_string()),
        IncomingBitVMXApiMessages::GetPubKey(uuid, _new_key) => Ok(uuid.to_string()),
        IncomingBitVMXApiMessages::SignMessage(uuid, _payload_to_sign, _public_key_to_use) => {
            Ok(uuid.to_string())
        }
        IncomingBitVMXApiMessages::GenerateZKP(uuid, _payload_to_sign, _name) => {
            Ok(uuid.to_string())
        }
        IncomingBitVMXApiMessages::ProofReady(uuid) => Ok(uuid.to_string()),
        IncomingBitVMXApiMessages::GetZKPExecutionResult(uuid) => Ok(uuid.to_string()),
        IncomingBitVMXApiMessages::Encrypt(uuid, _payload_to_encrypt, _public_key_to_use) => {
            Ok(uuid.to_string())
        }
        IncomingBitVMXApiMessages::Decrypt(uuid, _payload_to_decrypt) => Ok(uuid.to_string()),
        IncomingBitVMXApiMessages::GetFundingBalance(uuid) => Ok(uuid.to_string()),
        IncomingBitVMXApiMessages::GetFundingAddress(uuid) => Ok(uuid.to_string()),
        IncomingBitVMXApiMessages::SendFunds(uuid, _destination, _fee) => Ok(uuid.to_string()),
        _ => Err(anyhow::anyhow!(
            "unhandled request message type: {:?}",
            message
        )),
    }
}

/// Convert the response received from BitVMX to a correlation ID
pub fn response_to_correlation_id(
    response: &OutgoingBitVMXApiMessages,
) -> Result<String, anyhow::Error> {
    match response {
        OutgoingBitVMXApiMessages::Pong() => Ok("ping".to_string()),
        OutgoingBitVMXApiMessages::Transaction(uuid, _transaction_status, name) => match name {
            Some(name) => Ok(tx_name_to_correlation_id(uuid, name)),
            None => Ok(uuid.to_string()),
        },
        OutgoingBitVMXApiMessages::PeginTransactionFound(_txid, _transaction_status) => {
            Ok("rsk_pegin".to_string())
        }
        OutgoingBitVMXApiMessages::SpendingUTXOTransactionFound(
            uuid,
            _txid,
            _vout,
            _transaction_status,
        ) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::SetupCompleted(uuid) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::AggregatedPubkey(uuid, _aggregated_pubkey) => {
            Ok(uuid.to_string())
        }
        OutgoingBitVMXApiMessages::AggregatedPubkeyNotReady(uuid) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::ProtocolVisualization(uuid, _visualization) => {
            Ok(format!("protocol_visualization_{uuid}"))
        }
        OutgoingBitVMXApiMessages::TransactionInfo(uuid, _name, _transaction) => {
            Ok(uuid.to_string())
        }
        OutgoingBitVMXApiMessages::ZKPResult(uuid, _zkp_result, _zkp_proof) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::CommInfo(_p2p_address) => Ok("get_comm_info".to_string()),
        OutgoingBitVMXApiMessages::KeyPair(uuid, _private_key, _public_key) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::PubKey(uuid, _pub_key) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::SignedMessage(
            uuid,
            _signature_r,
            _signature_s,
            _recovery_id,
        ) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::Variable(uuid, _key, _value) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::Witness(uuid, _key, _witness) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::NotFound(uuid, _key) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::HashedMessage(uuid, _name, _vout, _leaf, _) => {
            Ok(uuid.to_string())
        }
        OutgoingBitVMXApiMessages::ProofReady(uuid) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::ProofNotReady(uuid) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::ProofGenerationError(uuid, _error) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::SPVProof(txid, _spv_proof) => Ok(format!("spv_proof_{txid}")),
        OutgoingBitVMXApiMessages::Encrypted(uuid, _encrypted_message) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::Decrypted(uuid, _decrypted_message) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::FundingAddress(uuid, _address) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::FundingBalance(uuid, _balance) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::FundsSent(uuid, _txid) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::WalletNotReady(uuid) => Ok(uuid.to_string()),
        OutgoingBitVMXApiMessages::WalletError(uuid, _error) => Ok(uuid.to_string()),
        _ => Err(anyhow::anyhow!(
            "unhandled response message type: {:?}",
            response
        )),
    }
}
