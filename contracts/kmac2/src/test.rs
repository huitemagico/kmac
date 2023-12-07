#![cfg(test)]
//test module for KMAC
//version for kmac2.0 
//please see 
//use std::println;
//

use soroban_sdk::{
    String,
    testutils::Address as _, 
    Address, Env, 
};

extern crate std;

use crate::{KmacContract, KmacContractClient};
//comm testing the corefn module:
//use crate::corefn;
//use crate::mimodulo;
use crate::kmacusermod;
//comm use crate::corefn::my_function;
//comm ....................................
#[test]
fn test() {

    //comm 
    //comm 
    let env = Env::default();



       let contract_id = env.register_contract(None, KmacContract);

let user_1 = Address::random(&env);
std::println!("user_1 from let user_1 = Address::random(&env);{:#?}", user_1);

       let client = KmacContractClient::new    (&env, &contract_id);


    
    // init default trx
    let first_trx = String::from_slice(&env, "ab");
    //
    // init first message
    std::println!("Testing core messages: Testing Message cldrst==1 ");
    let cldrst = String::from_slice(&env, "cldrst");
    let mut echo_response_tupla = 
              client.kmac        (&user_1,  &user_1,       &5,         &cldrst,  &first_trx);
    let expected_echo_response = get_value_trx1();
    assert_eq!(echo_response_tupla.1.get(0), expected_echo_response);
    std::println!("Testing Message cldrst==1 successfully ");





      
   //let current_state=corefn::xget_curr_st(&env);
   //let current_state=corefn::xget_curr_st();
//    let numerito=corefn::hello6789(&env);
//    std::println!("Numerito ok {}",  numerito);   

   
   //let init_stat=corefn::xis_init_stat(&env);
   let expected_state_trx1:Option <String>;
   
//    expected_state_trx1 = get_state_trx1(&env);
//    //expected_state_trx1a=expected_state_trx1
//    std::println!("Testing state after message ==1 ");
//    assert_eq!(echo_response_tupla.0.get(0), expected_state_trx1);
//    std::println!("Testing state after message ==1 Ok {} ","MCSTAT not existed");
//    //comm no se como poner la variable arriba! 
//
// calling contract with message rstkadm ==2
    std::println!("Testing core messages: Testing Message rstkadm==2 ");
    let rstkadm = String::from_slice(&env, "rstkadm");
    let echo_response_tupla = 
              client.kmac        (&user_1,  &user_1,       &5,         &rstkadm,  &first_trx);
    let expected_echo_response = get_value_trx2();
    assert_eq!(echo_response_tupla.1.get(0), expected_echo_response);
    std::println!("Testing Message rstkadm==2 successfully ");

// calling contract with message rstkadm ==2
std::println!("Testing core messages: Testing Message rstkadm==2 ");
let rstkadm = String::from_slice(&env, "rstkadm");
let echo_response_tupla = 
          client.kmac        (&user_1,  &user_1,       &5,         &rstkadm,  &first_trx);
let expected_echo_response = get_value_trx2();
assert_eq!(echo_response_tupla.1.get(0), expected_echo_response);
std::println!("Testing Message rstkadm==2 successfully ");
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

    
    fn _get_state_trx1 (env:&Env)  ->Option <String> {
        //
       let nostatems:String;
       nostatems  = String::from_slice(&env, "MCSTAT not existed");
       let my_option_string: Option<String> = Some(nostatems); //
       my_option_string
    }
    
    fn get_value_trx1() -> Option<i32> {
        Some(1)
    }

    fn get_value_trx2() -> Option<i32> {
        Some(2)
    }
}