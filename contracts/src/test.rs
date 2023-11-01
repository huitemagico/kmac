#![cfg(test)]
use super::*;
use soroban_sdk::String;

#[test]
fn test() {
    //kmac alfa 251023 agrego el parametro "trx" conforme a lo qu eespera el contract
    let env = Env::default();
    let contract_id = env.register_contract(None, Echo2Contract);
    let client = Echo2ContractClient::new(&env, &contract_id);

    // init first message
    let first_message = String::from_slice(&env, "reset");
    // init default trx
    let first_trx = String::from_slice(&env, "ab");
    //[5,170,["echo2 v.1.1 27/08/2023","ResetMessageStored","reset"]]

    //
    // calling contract
    let echo_response_tupla = client.echo2(&first_message, &first_trx);
    let expected_echo_response = 5;

    assert_eq!(echo_response_tupla.0, expected_echo_response);
}