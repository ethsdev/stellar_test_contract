#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String, log};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Inputs
    let component_type = String::from_str(&env, "solarpsolarpsolarpsolarpsolarpsolarpsolarpsolarpsolarpsolarp");
    let production_quantity = 100;
    let production_date = String::from_str(&env, "2025-04-24");
    let facility_id = String::from_str(&env, "FAC123");
    let doc_hash = String::from_str(&env, "abc123def456");

    let batch_id = client.submit_data(
        &component_type,
        &production_quantity,
        &production_date,
        &facility_id,
        &doc_hash,
    );

    log!(&env, "batch_id:", batch_id);

    let timestamp = client.get_timestamp(&batch_id);

    log!(&env, "timestamp:", timestamp);
    
}
