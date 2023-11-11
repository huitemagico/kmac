#![cfg(test)]
//use std::println;

use super::*;
//comm use original: use soroban_sdk::String;
//comm ......agrego al use los pars desde test de auth:
// use soroban_sdk::{
//     symbol_short,
//     testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
//     Address, Env, IntoVal,
// };
use soroban_sdk::{
    String,
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal,
};
//comm agrego extern crate std para suar el println
extern crate std;
//comm agrego el crate use crate::{IncrementContract, IncrementContractClient};
use crate::{KmacContract, KmacContractClient};
//comm ....................................
#[test]
fn test() {
    //kmac alfa 251023 agrego el parametro "trx" conforme a lo qu eespera el contract
    //kmac beta Nov 08, 2023 . "Only kmac"
    let env = Env::default();
//comm kamacauth:
//comm lo de arriba igual
//comm let contract_id = env.register_contract(None, IncrementContract);
//comm let client = IncrementContractClient::new(&env, &contract_id);

//comm let user_1 = Address::random(&env);
//comm let user_2 = Address::random(&env);


       let contract_id = env.register_contract(None, KmacContract);
//comm let contract_id =          env.register_contract(None, IncrementContract);... igual

//comm en auth:
let user_1 = Address::random(&env);
std::println!("user_1 from let user_1 = Address::random(&env);{:#?}", user_1);
//comm esto no funciona std::println!("{}", user_1);
//comm el compiler sugiere el pretty print y eto esta explicado en 
//comm https://stackoverflow.com/questions/72637174/pretty-print-struct-in-rust
//assert_eq!(client.increment(&user_1, &5), 5);

       let client = KmacContractClient::new    (&env, &contract_id);
//comm let client =                          IncrementContractClient::new(&env, &contract_id); .....igual

    // init first message
    let first_message = String::from_slice(&env, "reset");
    // init default trx
    let first_trx = String::from_slice(&env, "ab");
    //
    //
    // calling contract
    //****************comm como se llama al contrato en auth??*******************
    //pub fn echo2        (env: Env,  user: Address, value: u32, message: String, sender:String ) -> 
    //(  u32,u32, u32, u32, Vec<String>, bool,i32,i32,i32,i32,i32,i32) {
    let echo_response_tupla = 
              client.kmac        (&user_1,  &user_1,       &5,         &first_message,  &first_trx);
    
    let expected_echo_response = 5;

    assert_eq!(echo_response_tupla.0, expected_echo_response);
}