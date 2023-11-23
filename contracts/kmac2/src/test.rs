#![cfg(test)]
//use std::println;
//
use soroban_sdk::{
    String,
    testutils::Address as _, 
    Address, Env, 
};

extern crate std;

use crate::{KmacContract, KmacContractClient};
//comm ....................................
#[test]
fn test() {
    //kmac alfa 251023 agrego el parametro "trx" conforme a lo qu eespera el contract
    //kmac beta Nov 08, 2023 . "Only kmac"
    let env = Env::default();



       let contract_id = env.register_contract(None, KmacContract);

let user_1 = Address::random(&env);
std::println!("user_1 from let user_1 = Address::random(&env);{:#?}", user_1);

       let client = KmacContractClient::new    (&env, &contract_id);


    // init first message
    let first_message = String::from_slice(&env, "reset");
    // init default trx
    let first_trx = String::from_slice(&env, "ab");
    //
    //
    // calling contract
    //fn set_keyb   (env:&Env , user:Address)
    let echo_response_tupla = 
              client.kmac        (&user_1,  &user_1,       &5,         &first_message,  &first_trx);
             
    
    let expected_echo_response = 5;

    assert_eq!(echo_response_tupla.0, expected_echo_response);
}