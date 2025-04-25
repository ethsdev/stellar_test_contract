#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype,
    panic_with_error, 
    xdr::ToXdr, Bytes, BytesN, Env, String, crypto::Hash, 
};

#[contracttype]
pub struct Section45X {
    batch_id: String,
    timestamp: u64,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractErrors {
    ComponentTypeLengthError = 0,
    ProductionDateLengthError = 1,
    FacilityIdLengthError = 2,
    DocHashLengthError = 3,
}

const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

fn bytesn_to_hex_string(env: &Env, bytes: BytesN<32>) -> String {
    let raw = bytes.to_array();
    let mut hex_bytes = [0u8; 64];

    for (i, byte) in raw.iter().enumerate() {
        hex_bytes[i * 2] = HEX_CHARS[(byte >> 4) as usize];
        hex_bytes[i * 2 + 1] = HEX_CHARS[(byte & 0x0F) as usize];
    }

    String::from_bytes(env, &hex_bytes)
}

fn hash_to_bytes_n(hash: Hash<32>) -> BytesN<32> {
    hash.into()
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn submit_data(
        env: &Env,
        component_type: String,
        production_quantity: i32,
        production_date: String,
        facility_id: String,
        doc_hash: String,
    ) -> String {
        let mut bytes_for_hash = Bytes::new(env);
        let timestamp = env.ledger().timestamp();

        // ---------- component_type ----------- //
        let component_type_len = component_type.len() as usize;
        if component_type_len > 64 {
            panic_with_error!(env, &ContractErrors::ComponentTypeLengthError);
        }
        let mut component_type_slice: [u8; 64] = [0; 64];
        component_type.copy_into_slice(&mut component_type_slice[..component_type_len]);
        let component_type_as_byte =
            Bytes::from_slice(&env, &component_type_slice[0..component_type_len]);

        // ---------- production_quantity ----------- //
        let production_quantity_as_bytes = production_quantity.to_xdr(env);

        // ---------- production_date ----------- //
        let production_date_len = production_date.len() as usize;
        if production_date_len > 20 {
            panic_with_error!(env, &ContractErrors::ProductionDateLengthError);
        }
        let mut production_date_slice: [u8; 20] = [0; 20];
        production_date.copy_into_slice(&mut production_date_slice[..production_date_len]);
        let production_date_as_byte =
            Bytes::from_slice(&env, &production_date_slice[0..production_date_len]);

        // ---------- facility_id ----------- //
        let facility_id_len = facility_id.len() as usize;
        if facility_id_len > 64 {
            panic_with_error!(env, &ContractErrors::FacilityIdLengthError);
        }
        let mut facility_id_slice: [u8; 64] = [0; 64];
        facility_id.copy_into_slice(&mut facility_id_slice[..facility_id_len]);
        let facility_id_as_byte = Bytes::from_slice(&env, &facility_id_slice[0..facility_id_len]);

        // ---------- doc_hash ----------- //
        let doc_hash_len = doc_hash.len() as usize;
        if doc_hash_len > 70 {
            panic_with_error!(env, &ContractErrors::DocHashLengthError);
        }
        let mut doc_hash_slice: [u8; 70] = [0; 70];
        doc_hash.copy_into_slice(&mut doc_hash_slice[..doc_hash_len]);
        let doc_hash_as_byte = Bytes::from_slice(&env, &doc_hash_slice[0..doc_hash_len]);

        bytes_for_hash.append(&component_type_as_byte);
        bytes_for_hash.append(&production_quantity_as_bytes);
        bytes_for_hash.append(&production_date_as_byte);
        bytes_for_hash.append(&facility_id_as_byte);
        bytes_for_hash.append(&doc_hash_as_byte);

        let batch_id = env.crypto().sha256(&bytes_for_hash);

        let batch_id_as_str = bytesn_to_hex_string(env, hash_to_bytes_n(batch_id.clone()));

        env.storage().persistent().set(&batch_id_as_str, &timestamp);

        batch_id_as_str
    }

    pub fn get_timestamp(env: &Env, batch_id: String) -> u64 {
        env.storage().persistent().get(&batch_id).unwrap_or(0)
    }
}

mod test;
