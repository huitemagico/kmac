#![cfg(test)]
//test module for KMAC
//version for kmac2.0 version to prod December 24
//please see https://github.com/huitemagico/kmac/wiki
//the transactions output explained at:
//https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#the-vending-machine-example-version-december-19-2023-output-example-and-explanation
//https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#the-vending-machine-example-the-sequence-of-command-shell-transactions-takeaways
//https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#cargo-test-the-program-how-to

//testdec23

//use alloc::string::ToString;
extern crate std;

use soroban_sdk::{
    String,
    Env
};
use soroban_sdk::testutils::Address;

use crate::{KmacContract, KmacContractClient};
use crate::kmacusermod;
// THE TEST
pub struct KMACTest<'a> {
    env: Env,
    contract: KmacContractClient<'a>,
}

impl<'a> KMACTest<'a> {
    pub fn setup() -> Self {

        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, KmacContract);
        let contract = KmacContractClient::new    (&env, &contract_id);

        KMACTest {
            env,
            contract,
        }
    }
}

//Trx NOT in FLOW!
fn get_value_strx(env: &Env) -> Option<String> {
    Some(String::from_str(&env, "Trx NOT in FLOW!"))
}
fn get_value_str_cldrst_statea(env: &Env) -> Option<String> {
    Some(String::from_str(&env, "A"))
}
fn get_value_str_cldrst_stateb(env: &Env) -> Option<String> {
   Some(String::from_str(&env, "B"))
}
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

#[test]
fn test_main() {
    std::println!("KMAC testing:");
    std::println!("https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#the-vending-machine-example-the-sequence-of-command-shell-transactions-takeaways");
    std::println!("https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90extending-feature:-How-add-user-functions#the-transaction-number-vs-the-function-number");
    std::println!("See the https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90extending-feature:-How-add-user-functions#the-transaction-number-vs-the-function-number");
    std::println!(" (1) Test transaction in correct flow: ");
    std::println!("--------------------------------------------------- ");
    test();
    std::println!(" (1) Test transaction in correct flow: END");
    std::println!("------------------------------------------------------- ");
    std::println!(" (2) Test transaction in BAD sequence flow:");
    std::println!("------------------------------------------------------- ");
    test_call_not_in_flow();
    std::println!(" (2) Test transaction in BAD sequence flow: END");
    std::println!("------------------------------------------------------- ");
}
fn test() {
    let test = KMACTest::setup();
    let user_1 = <soroban_sdk::Address as Address>::generate(&test.env);
    std::println!("user_1 from let user_1 = Address::random(&test.env);{:#?}", user_1);
    let nouse = String::from_str(&test.env, "nouse");
    // trx cldrst------------------------------------------------------------BEGIN
    std::println!("---->Testing transaction number with trx cldrst (1) ");
    let cldrst = String::from_str(&test.env, "cldrst");
    //
    let response_tupla =
        test.contract.kmac        (&user_1,  &user_1,       &5,         &cldrst,  &nouse);
    //
    let expected_echo_response = get_value_trx1();
    assert_eq!(response_tupla.1.get(0), expected_echo_response);
    std::println!("---->Answer transaction number (1) OK.");
    std::println!("---->Testing Final state after trx cldrst:");

    let sexpected_response_statea = get_value_str_cldrst_statea(&test.env);
    assert_eq!(response_tupla.0.get(5).unwrap(), sexpected_response_statea.unwrap());
    std::println!("...after Message cldrst==1 ...final state=A: OK");
    std::println!("Testing Message cldrst==1 successfully.");
    // trx cldrst------------------------------------------------------------END
    // trx rstkadm ==2----------------------------------------------------------BEGIN
    std::println!("Testing How core answer with Transaction rstkadm (2)  ");
    std::println!("---->Testing transaction number with trx rstkadm (2) ");
    let rstkadm = String::from_str(&test.env, "rstkadm");
    let response_tupla2 =
        test.contract.kmac        (&user_1,  &user_1,       &5,         &rstkadm,  &nouse);
    let expected_echo_response = get_value_trx2();
    assert_eq!(response_tupla2.1.get(0), expected_echo_response);
    std::println!("---->Answer transaction number (2) OK.");
    std::println!("---->Testing Final state after trx rstkadm:");
    let sexpected_response_stateb = get_value_str_cldrst_stateb(&test.env);
    assert_eq!(response_tupla2.0.get(5).unwrap(), sexpected_response_stateb.unwrap());
    std::println!("...after Message rstkadm...final state=B: OK");


    // trx rstkadm ==2----------------------------------------------------------END
    // trx svb1adr (9)----------------------------------------------------------BEGIN
    std::println!("---->Testing transaction number with trx svb1adr (9)  ");
    let svb1adr = String::from_str(&test.env, "svb1adr");
    let response_tupla3 =
        test.contract.kmac        (&user_1,  &user_1,       &5,         &svb1adr,  &nouse);
    let expected_echo_response = get_value_trx9();
    assert_eq!(response_tupla3.1.get(0), expected_echo_response);
    std::println!("---->Answer transaction number (9) OK.");

    // trx svb1adr (9)----------------------------------------------------------END
    // selcofblnd (17)----------------------------------------------------------BEGIN
    std::println!("---->Testing transaction number with trx selcofblnd (17)   ");
    let selcofblnd = String::from_str(&test.env, "selcofblnd");
    let response_tupla4 =
        test.contract.kmac        (&user_1,  &user_1,       &5,         &selcofblnd,  &nouse);
    let expected_echo_response = get_value_trx17();
    assert_eq!(response_tupla4.1.get(0), expected_echo_response);
    std::println!("---->Answer transaction number (17) OK.");
    // selcofblnd (17)----------------------------------------------------------END
    // buyerpay (fn8-transaction 30)----------------------------------------------------------BEGIN
    //std::println!("Testing core messages: Testing Message buyerpay (fn8-transaction 30) ");
    std::println!("---->Testing transaction number with trx buyerpay (fn8-transaction 30)");

    let buyerpay = String::from_str(&test.env, "buyerpay");
    let response_tupla5 =
        test.contract.kmac        (&user_1,  &user_1,       &5,         &buyerpay,  &nouse);
    let expected_echo_response = get_value_trx30();
    assert_eq!(response_tupla5.1.get(0), expected_echo_response);
    std::println!("---->Answer transaction number (30) OK.");
    // buyerpay (fn8-transaction 30)----------------------------------------------------------END

    //retactiv
    std::println!("---->Testing transaction number with trx retactiv (fn9-transaction 33)");


    let retactiv = String::from_str(&test.env, "retactiv");
    let response_tupla6 =
        test.contract.kmac        (&user_1,  &user_1,       &5,         &retactiv,  &nouse);
    let expected_echo_response = get_value_trx33();
    assert_eq!(response_tupla6.1.get(0), expected_echo_response);
    std::println!("---->Answer transaction number (33) OK.");

//
//
// calling not used function. This is done because: (a) prevent compiler warmings (b) the functions
//below are "user functions" that could be called in another modeling program. See the "extensions" at
// https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#the-three-views-of-kmac-
//
    let f7out= kmacusermod::function7(&test.env);
    std::println!("function7out {:?}",  f7out);
    //
    let f15out= kmacusermod::function15(&test.env);
    std::println!("function15out {:?}",  f15out);
//



}
//#[test]
fn test_call_not_in_flow() {
    let test = KMACTest::setup();

    let user_1 = <soroban_sdk::Address as Address>::generate(&test.env);
    let buyerpay = String::from_str(&test.env, "buyerpay");
    let nouse = String::from_str(&test.env, "nouse");
    std::println!("---->Testing transaction number with trx cldrst (1) ");
    let cldrst = String::from_str(&test.env, "cldrst");
    //
    let response_tupla =
        test.contract.kmac        (&user_1,  &user_1,       &5,         &cldrst,  &nouse);
    //
    let expected_echo_response = get_value_trx1();
    assert_eq!(response_tupla.1.get(0), expected_echo_response);
    std::println!("---->Answer transaction number (1) OK.");
    std::println!("---->Testing Final state after trx cldrst:");
    //let statea = String::from_str(&test.env, "A");
    let sexpected_response_statea = get_value_str_cldrst_statea(&test.env);
    assert_eq!(response_tupla.0.get(5).unwrap(), sexpected_response_statea.clone().unwrap());
    std::println!("...after Message cldrst==1 ...final state=A: OK");

    //Testing out-of-flow transactions
    //After the precedent trx, the machine state is in 'C' state. Consistent with the flow definition of the vending machine example,
    //the only trx accepted will be (maintenance) or (select coffee blend) trx. Thats explained:
    // I will try the 'buyerpay' trx:
    //
    let response_tupla55 =
        test.contract.kmac        (&user_1,  &user_1,       &5,         &buyerpay,  &nouse);
    assert_eq!(response_tupla.0.get(5).unwrap(), sexpected_response_statea.unwrap());
    std::println!("...after Message buyerpay...not change the state...A...cldrst==1 ...final state=A: OK");
    let sexpected_echo_response = get_value_strx(&test.env);
    //Verify that the program answers "Trx NOT in FLOW!"
    std::println!("---->Testing transaction NOT IN FLOW (buyerpay)");
    assert_eq!(response_tupla55.0.get(2).unwrap(), sexpected_echo_response.unwrap());
    //assert_eq!(response_tupla55.0.get(6), sexpected_echo_response);
    std::println!("---->Testing transaction NOT IN FLOW (buyerpay) OK) ");
    //
    //
}
