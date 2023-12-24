#![cfg(test)]
//test module for KMAC
//version for kmac2.0 version to prod December 24
//please see https://github.com/huitemagico/kmac/wiki
//the transactions output explained at:
//https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#the-vending-machine-example-version-december-19-2023-output-example-and-explanation
//https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#the-vending-machine-example-the-sequence-of-command-shell-transactions-takeaways
//testdec23

//use alloc::string::ToString;
use soroban_sdk::{
    String,
    Env, Vec
};

extern crate std;

use soroban_sdk::testutils::Address;

use crate::{KmacContract, KmacContractClient};

//use crate::corefn;
//use crate::mimodulo;
use crate::kmacusermod;

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KmacContract);
//  let user_1 = Address::generate(&env);
    let user_1 = <soroban_sdk::Address as Address>::generate(&env);
    std::println!("user_1 from let user_1 = Address::random(&env);{:#?}", user_1);
    let client = KmacContractClient::new    (&env, &contract_id);
    // init default trx
    let nouse = String::from_str(&env, "nouse");
    // init first message
    std::println!("Testing How core answer with Transaction cldrst (1) ");
    std::println!("https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#the-vending-machine-example-the-sequence-of-command-shell-transactions-takeaways");
    let cldrst = String::from_str(&env, "cldrst");
    //
    let response_tupla =
        client.kmac        (&user_1,  &user_1,       &5,         &cldrst,  &nouse);
    //
    let expected_echo_response = get_value_trx1();
    assert_eq!(response_tupla.1.get(0), expected_echo_response);
    std::println!("Testing Message cldrst==1 successfully.");


    // calling contract with message rstkadm ==2 ------------------------------BEGIN
    std::println!("Testing How core answer with Transaction rstkadm (2)  ");
    let rstkadm = String::from_str(&env, "rstkadm");
    let response_tupla2 =
        client.kmac        (&user_1,  &user_1,       &5,         &rstkadm,  &nouse);

    let expected_echo_response = get_value_trx2();
    assert_eq!(response_tupla2.1.get(0), expected_echo_response);
    std::println!("Testing Message rstkadm==2 successfully ");
//--------------------------------------------------------------------END
//--------------------------------------------------------------------BEGIN
    std::println!("Testing core messages: Testing Message svb1adr (9) ");
    let svb1adr = String::from_str(&env, "svb1adr");
    let response_tupla3 =
        client.kmac        (&user_1,  &user_1,       &5,         &svb1adr,  &nouse);
    let expected_echo_response = get_value_trx9();
    assert_eq!(response_tupla3.1.get(0), expected_echo_response);
    std::println!("Testing Message svb1adr==9 successfully ");
    //selcofblnd
    std::println!("Testing core messages: Testing Message selcofblnd (17) ");
    let selcofblnd = String::from_str(&env, "selcofblnd");
    let response_tupla4 =
        client.kmac        (&user_1,  &user_1,       &5,         &selcofblnd,  &nouse);
    let expected_echo_response = get_value_trx17();
    assert_eq!(response_tupla4.1.get(0), expected_echo_response);
    std::println!("Testing Message svb1adr==17 successfully ");
    //buyerpay
    std::println!("Testing core messages: Testing Message buyerpay (fn8-transaction 30) ");
    std::println!("See the https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90extending-feature:-How-add-user-functions#the-transaction-number-vs-the-function-number");
    let buyerpay = String::from_str(&env, "buyerpay");
    let response_tupla5 =
        client.kmac        (&user_1,  &user_1,       &5,         &buyerpay,  &nouse);
    let expected_echo_response = get_value_trx30();
    assert_eq!(response_tupla5.1.get(0), expected_echo_response);
    std::println!("Testing Message buyerpay==30 successfully ");
    //retactiv
    std::println!("Testing core messages: Testing Message retactiv (fn9-transaction 33) ");
    std::println!("See the https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90extending-feature:-How-add-user-functions#the-transaction-number-vs-the-function-number");
    let retactiv = String::from_str(&env, "retactiv");
    let response_tupla6 =
        client.kmac        (&user_1,  &user_1,       &5,         &retactiv,  &nouse);
    let expected_echo_response = get_value_trx33();
    assert_eq!(response_tupla6.1.get(0), expected_echo_response);
    std::println!("Testing Message retactiv==33 successfully ");
//
//
// calling not used function. This is done because: (a) prevent compiler warmings (b) the functions
//below are "user functions" that could be called in another modeling program. See the "extensions" at
// https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#the-three-views-of-kmac-
//
    let f7out= kmacusermod::function7(&env);
    std::println!("function7out {:?}",  f7out);
    //
    let f15out= kmacusermod::function15(&env);
    std::println!("function7out {:?}",  f15out);
//


    fn get_value_trx1() -> Option<i32> {
        Some(1)
    }

    fn get_value_trx2() -> Option<i32> {
        Some(2)
    }
    fn get_value_trx9() -> Option<i32> {
        Some(9)
    }
    fn get_value_trx17() -> Option<i32> {
        Some(17)
    }
    fn get_value_trx30() -> Option<i32> {
        Some(30)
    }
    fn get_value_trx33() -> Option<i32> {
        Some(33)
    }

}
