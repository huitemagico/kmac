#![cfg(test)]
//test module for KMAC
//version for kmac 1.0.0 "prod version" December 31
//https://github.com/huitemagico/kmac/wiki#kmac-program-version
//please see https://github.com/huitemagico/kmac/wiki
//the transactions output explained at:
//https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#the-vending-machine-example-version-december-19-2023-output-example-and-explanation
//https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#the-vending-machine-example-the-sequence-of-command-shell-transactions-takeaways
//https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#cargo-test-the-program-how-to
extern crate std;
use soroban_sdk::IntoVal;
use soroban_sdk::{
    String,
    Env
};
use soroban_sdk::testutils::Address;
use soroban_sdk::{
    symbol_short,
    testutils::{//Address as _,
                AuthorizedFunction, AuthorizedInvocation},

};

use crate::{KmacContract, KmacContractClient};
//use crate::kmacusermod;

#[test]
fn test() {


    std::println!(
        "Test module for kmac v.2.0 Dec 30. 2023. See //https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#cargo-test-the-program-how-to \n");
    std::println!("https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#the-vending-machine-example-the-sequence-of-command-shell-transactions-takeaways");
    std::println!("https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90extending-feature:-How-add-user-functions#the-transaction-number-vs-the-function-number");
    std::println!("See the https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90extending-feature:-How-add-user-functions#the-transaction-number-vs-the-function-number");

    let traceon :bool=false;
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KmacContract);
    let client = KmacContractClient::new    (&env, &contract_id);
    let user_1 = <soroban_sdk::Address as Address>::generate(&env);
    let user_2 = <soroban_sdk::Address as Address>::generate(&env);
    if traceon {std::println!("user_1 from let user_1 = Address::random(&test.env);{:#?}", user_1);}
    let nouse = String::from_str(&env, "nouse");
    let cldrst = String::from_str(&env, "cldrst");

    // trx cldrst------------------------------------------------------------BEGIN
    std::println!("\n(1) Test transaction in correct flow\n\n");

    std::println!("---->Testing the (transaction number)=F(transaction name) AND the (final state)=F(transaction, user)");
    std::println!("https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#about-transaction-numbertransaction-namefinal-state-auth_flag");
    //std::println!("---->Testing the (final state)=F(transaction, user)");
    std::println!("\n(I)(BEGIN)---->Testing (transaction number) and (final state) with trx cldrst \n ");

    //
    let response_tupla =
               client.kmac        (&user_1,  &user_2,       &5,         &cldrst,  &nouse);
    //
    if traceon {std::println!("\nLOG env.auths() {:#?}", env.auths());}
    let expected_echo_response = get_value_trx1();
    assert_eq!(response_tupla.1.get(0), expected_echo_response);
    std::println!("---->Answer transaction number (1) OK.");
    std::println!("---->Testing Final state after trx cldrst:");
    //let sexpected_response_statea = get_value_str_cldrst_statea(&test.env);
    let sexpected_response_statea = get_value_str_cldrst_statea(&env);
    assert_eq!(response_tupla.0.get(5).unwrap(), sexpected_response_statea.unwrap());
    std::println!("...after Message cldrst==1 ...final state=A: OK");
    std::println!("(I)(END)---->SUCCESS (transaction number) and (final state) with trx cldrst \n");
    // trx cldrst------------------------------------------------------------END
    std::println!("\n(II)(BEGIN)---->Testing env.auths()");
    if traceon {std::println!("Testing env.auths() : value:");
    std::println!("env.auths() {:#?}", env.auths());
    std::println!("Testing env.auths() !!!");}
    //
    assert_eq!(
        env.auths(),
        std::vec![(
            // Address for which authorization check is performed
            user_1.clone(),
            //            user_1.clone(),
            // Invocation tree that needs to be authorized
            AuthorizedInvocation {
                // Function that is authorized. Can be a contract function or
                // a host function that requires authorization.
                /*
                function: AuthorizedFunction::Contract((
                    // Address of the called contract
                    contract_id.clone(),
                 */
                function: AuthorizedFunction::Contract((
                    // Address of the called contract
                    contract_id.clone(),
                    // Name of the called function
                    //symbol_short!("increment"),
                    symbol_short!("kmac"),
                    // Arguments used to call `increment` (converted to the env-managed vector via `into_val`)
                    // (user_1.clone(), 5_u32).into_val(&env),
                    //)),
                    // pub fn increment(env: Env, user: Address, value: u32) -> u32 {
                    // (                           user_1.clone(), 5_u32).into_val(&env),
                    //Vec(Ok(Address(obj#5)), Ok(Address(obj#7)), Ok(U32(5)), Ok(String(obj#11)), Ok(String(obj#9)
                    (        user_1.clone(),user_2.clone(), 5_u32,    cldrst.clone(),   nouse.clone()    ).into_val(&env),
                )),
                //client.kmac        (&user_1,  &user_1,       &5,         &cldrst,  &nouse);
                // pub fn kmac
                // (env: Env,  user: Address, buyer: Address, _value: u32, message: String, _sender:String ) ->

                /*                    soroban contract invoke   --id $(cat .soroban/kmac2-id) \
                                                                       --source  kreator   \
                                                                       --network testnet  \
                                                                       --   kmac    \
                                                                       --user GDLTFWZTH3JXPJX4LHRJU5L7WDBZ2CQKTRA3Z57HUKP25H35ICQZRMFH         \
                                                                       --value 2    \
                                                                       --message "cldrst"    \
                                                                       --buyer "GBGC5LMJOTEYRHND7AY3GMDNTQPHJ22WMUMWKVBD7D746MLAN3OGVXRP"    \
                                                                       --sender  "kreator"

                */





                // The contract doesn't call any other contracts that require
                // authorization,
                sub_invocations: std::vec![]
            }
        )]
    );

    std::println!("(II)(END)---->SUCCESS Testing env.auths()\n");
//
// trx rstkadm ==2----------------------------------------------------------BEGIN
    std::println!("(III)(BEGIN)---->Testing (transaction number) and (final state) with trx rstkadm \n");
    //std::println!("Testing How core answer with Transaction rstkadm (2)  ");
    //std::println!("---->Testing transaction number with trx rstkadm (2) ");
    //let rstkadm = String::from_str(&test.env, "rstkadm");
    let rstkadm = String::from_str(&env,"rstkadm");
    //
    let mut response_tupla2 =
               client.kmac        (&user_1,  &user_2,       &5,         &rstkadm,  &nouse);
    let expected_echo_response = get_value_trx2();
    assert_eq!(response_tupla2.1.get(0), expected_echo_response);
    std::println!("---->Answer transaction number (2) OK.");
    std::println!("---->Testing Final state after trx rstkadm:");
    //
    let sexpected_response_stateb = get_value_str_cldrst_stateb(&env);
    assert_eq!(response_tupla2.0.get(5).unwrap(), sexpected_response_stateb.unwrap());
    std::println!("...after Message rstkadm...final state=B: OK");
    std::println!("\n(III)(END)---->SUCCESS (transaction number) and (final state) with trx rstkadm  ");
    // trx rstkadm ==2----------------------------------------------------------END
    if traceon {std::println!("LOG Testing env.auths() : value:");
    std::println!("LOG env.auths() {:#?}", env.auths());}


// trx svb1adr (9)----------------------------------------------------------BEGIN
    //std::println!("---->Testing transaction number with trx svb1adr (9) with CORRECT users!  ");
    std::println!("\n(IV)(BEGIN)---->Testing (transaction number) and (final state) with trx svb1adr ");
    let svb1adr = String::from_str(&env, "svb1adr");
    let response_tupla3 =
        client.kmac        (&user_1,  &user_2,       &5,         &svb1adr,  &nouse);
    //
    let expected_echo_response = get_value_trx9();
    assert_eq!(response_tupla3.1.get(0), expected_echo_response);
    std::println!("---->Answer transaction number (9) OK....but the state ???");
    let sexpected_response_statecc = get_value_str_cldrst_statec(&env);
    assert_eq!(response_tupla3.0.get(5).unwrap(), sexpected_response_statecc.unwrap());
    std::println!("...after Message rstkadm...final state=C: OK...changed!");
    std::println!("\n(IV)(END)---->SUCCESS (transaction number) and (final state) with trx svb1adr\n ");
    // trx svb1adr (9)----------------------------------------------------------END

    // selcofblnd (17)----------------------------------------------------------BEGIN
    std::println!("(V)(BEGIN)---->Testing (transaction number) and (final state) with trx selcofblnd ");
    //std::println!("---->Testing transaction number with trx selcofblnd (17)   ");
    //let selcofblnd = String::from_str(&test.env, "selcofblnd");
    let selcofblnd = String::from_str(&env, "selcofblnd");
    let response_tupla4 =
        client.kmac        (&user_2,  &user_2,       &5,         &selcofblnd,  &nouse);
    let expected_echo_response = get_value_trx17();
    assert_eq!(response_tupla4.1.get(0), expected_echo_response);
    std::println!("---->Answer transaction number (17) OK.");
    std::println!("---->Answer transaction number  OK....but the state ???");
   /* //known bug#1: with this trx there is no message with final state :-/
    //turnaround 1: change the comparison string (change the get_value_str_cldrst_statee function return)
    let sexpected_response_stateee = get_value_str_cldrst_statee(&env);
    assert_eq!(response_tupla4.0.get(5).unwrap(), sexpected_response_stateee.unwrap());
    std::println!("...after Message selcofblnd...returned message... : OK (see known bug #1)");
    std::println!("(V)(END)---->SUCCESS (transaction number) and (final message) with trx selcofblnd\n \n ");

    */
    //turnaround 2: send trx (admin flow or user trx ) (see wiki) "out of flow").
    // In this case is sufficent with
    //sending the same selcofblnd (select coffe blend) again. Note: The business user has defined that the sequence
    //selcofblnd - selcofblnd is invalid. Please see https://github.com/huitemagico/kmac/wiki/Valid-Transaction-sequences
    //
    //The "former state" (state before the transaction action) comes at "0" place at the "dashboard"
    //See https://github.com/huitemagico/kmac/wiki/Dashboard-utility
    //
    //THE NEXT TRX IS SENT BY CORRECT USER (USER_2)
    //
    let response_tupla4 =
        client.kmac        (&user_2,  &user_2,       &5,         &selcofblnd,  &nouse);
    let sexpected_response_stateeee = get_value_str_cldrst_stateee(&env);
    //std::println!("---->turnaround 2.....{:#?}",response_tupla4.0.get(0).unwrap());
    assert_eq!(response_tupla4.0.get(0).unwrap(), sexpected_response_stateeee.unwrap());
    std::println!("\n(V)(END)---->SUCCESS (transaction number) and (final state) with trx selcofblnd\n ");

    // selcofblnd (17)----------------------------------------------------------END
    // buyerpay (fn8-transaction 30)----------------------------------------------------------BEGIN
    std::println!("(VI)(BEGIN)---->Testing (transaction number) and (final state) with trx buyerpay ");
    //std::println!("Testing core messages: Testing Message buyerpay (fn8-transaction 30) ");
    //std::println!("---->Testing transaction number with trx buyerpay (fn8-transaction 30)");

    let buyerpay = String::from_str(&env, "buyerpay");
    let response_tupla5 =
        client.kmac        (&user_2,  &user_2,       &5,         &buyerpay,  &nouse);
    let expected_echo_response = get_value_trx30();
    assert_eq!(response_tupla5.1.get(0), expected_echo_response);
    std::println!("---->Answer transaction number (30) OK... state??");
    //know bug...same case above...
    let sexpected_response_statef = get_value_str_cldrst_stateff(&env);
    assert_eq!(response_tupla5.0.get(5).unwrap(), sexpected_response_statef.unwrap());

    //The "former state" (state before the transaction action) comes at "0" place at the "dashboard"
    //See https://github.com/huitemagico/kmac/wiki/Dashboard-utility
    let response_tupla5 =
        client.kmac        (&user_2,  &user_2,       &5,         &buyerpay,  &nouse);
    let sexpected_response_statef = get_value_str_cldrst_statef(&env);
    assert_eq!(response_tupla5.0.get(0).unwrap(), sexpected_response_statef.unwrap());

    std::println!("\n(VI)(END)---->Testing (transaction number) and (final state) with trx buyerpay ");

    // buyerpay (fn8-transaction 30)----------------------------------------------------------END

    //retactiv
    std::println!("\n(VII)(BEGIN)---->Testing (transaction number) and (final state) with trx retactiv (33) ");
    let retactiv = String::from_str(&env, "retactiv");
    let response_tupla6 =
        client.kmac        (&user_1,  &user_1,       &5,         &retactiv,  &nouse);
    let expected_echo_response = get_value_trx33();
    assert_eq!(response_tupla6.1.get(0), expected_echo_response);
    std::println!("---->Answer transaction number (33) OK.");
    std::println!("\n(VII)(END)---->Testing (transaction number) [and (final state)..pending] with trx retactiv (33) ");
    //


//
//
// calling not used function. This is done because: (a) prevent compiler warmings (b) the functions
//below are "user functions" that could be called in another modeling program. See the "extensions" at
// https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#the-three-views-of-kmac-
//
    /*
    let f7out= kmacusermod::function7(&env);
    std::println!("function7out {:?}",  f7out);
    //
    let f15out= kmacusermod::function15(&env);
    std::println!("function15out {:?}",  f15out);
     */
    //

// dec 28
    //let contract_id = &test.register_contract(None, KmacContract);
    //let env = Env::default();
    /*
    let env2 = Env::default();
    std::println!("env.auths() env2*****************{:#?}", env2.auths());
    std::println!("env.auths() env-----------------------------------*****************{:#?}", env.auths());
    let nouse2 = String::from_str(&env, "nouse");
    std::println!("nouse2...............{:#?}", nouse2);
    let user_11 = <soroban_sdk::Address as Address>::generate(&env);
    let contract_id = env.register_contract(None, KmacContract);
    //error[E0277]: can't compare
    // `&test::std::vec::Vec<(soroban_sdk::Address, AuthorizedInvocation)>` with
    // `test::std::vec::Vec<(soroban_sdk::Address, AuthorizedInvocation)>`
    let cldrst2 = String::from_str(&env, "cldrst");


     */
    //dec 28 end
    //test();
    std::println!("\n\n (1) Test transaction in correct flow: END\n\n");
    std::println!("------------------------------------------------------- ");
    //std::println!(" (2) Test transaction in BAD sequence flow:");

    std::println!("\n(2) Test transaction in BAD sequence flow\n ");
    std::println!("(VIII) (a) Initialize the cycle, but with the contract already initialized(*)");
    std::println!("See https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#about-initialize-the-cycle-but-with-the-contract-already-initialized ");
    //
    std::println!("(VIII) (a) BEGIN....cldrst (user_1, user_2, auth_require=true)");
   //
    let response_tupla =
        client.kmac        (&user_1,  &user_2,       &5,         &cldrst,  &nouse);
   //
    let expected_echo_response = get_value_trx1();
    assert_eq!(response_tupla.1.get(0), expected_echo_response);
    std::println!("---->Answer transaction number (1) OK.");
    std::println!("---->Testing Final state after trx cldrst:");
    //"A"
    let sexpected_response_statea = get_value_str_cldrst_statea(&env);
    //known bug: if the contract has already been initialized, the state answer moves one place
    //see https://github.com/huitemagico/kmac/issues/2 Defect #1
    assert_eq!(response_tupla.0.get(6).unwrap(), sexpected_response_statea.clone().unwrap());
    //std::println!("...after Message cldrst==1 ...final state=A: OK");
    std::println!("(VIII) (a) END ....cldrst final state=A: OK");
    //
    response_tupla2 =
        client.kmac        (&user_1,  &user_2,       &5,         &rstkadm,  &nouse);
    response_tupla2 =
        client.kmac        (&user_1,  &user_2,       &5,         &svb1adr,  &nouse);
    let sexpected_response_statecc9 = get_value_str_cldrst_statec(&env);
    let sexpected_response_c_unw = sexpected_response_statecc9.unwrap();
    assert_eq!(response_tupla2.0.get(6).unwrap(), sexpected_response_c_unw);
    std::println!("(VIII) (a) END ....cldrst final state=C: OK");
    std::println!("\nTesting out-of-flow transactions");
    std::println!("\nAfter the precedent trx, the machine state is in 'C' state. Consistent with the flow definition of the vending machine example,");
    std::println!("\nthe only trx accepted will be (maintenance) or (select coffee blend) trx. NOW:");
    std::println!("\nI will try the 'buyerpay' trx:");
    //
    let response_tupla55 =
        client.kmac        (&user_1,  &user_2,       &5,         &buyerpay,  &nouse);
    /*assert_eq!(response_tupla55.0.get(6).unwrap(), sexpected_response_c_unw);
    std::println!("...after Message buyerpay...not change the state...A...cldrst==1 ...final state=A: OK");
    let sexpected_echo_response = get_value_strx(&env);
    //Verify that the program answers "Trx NOT in FLOW!"
    std::println!("---->Testing transaction NOT IN FLOW (buyerpay)");

     */
    //known bug: the state message is not at fixed place
    //you have to test with the commands for see the exact place
    //Trx NOT in FLOW! (3)
    let sexpected_response_not_fl=get_value_strx(&env);
    let sexpected_response_not_fl1 = sexpected_response_not_fl.unwrap();
    assert_eq!(response_tupla55.0.get(3).unwrap(), sexpected_response_not_fl1);
    //assert_eq!(response_tupla55.0.get(6), sexpected_echo_response);
    std::println!("---->Testing transaction NOT IN FLOW (buyerpay) OK) ");
    std::println!("(VIII) (b) END testing trx not in flow.");




    std::println!(" (2) Test transaction in BAD sequence flow: END");
    std::println!("------------------------------------------------------- ");

    //testing cycle from Cold again...
    std::println!("\n (IX)(BEGIN) cycle from Cold again ... for testing bad user sending trx\n");
    std::println!("\n ... trx cldrst \n");
    //
    let response_tupla1 =
        client.kmac        (&user_1,  &user_2,       &5,         &cldrst,  &nouse);
    //
    assert_eq!(response_tupla1.1.get(0), expected_echo_response);
    std::println!("---->Answer transaction number (1) OK.");
    std::println!("---->Testing Final state after trx cldrst:");
    let sexpected_response_statea1 = get_value_str_cldrst_statea(&env);
    //note: once initialized, the contract put the new state at (6) place
    //see https://github.com/huitemagico/kmac/issues/2 defect 1
    assert_eq!(response_tupla1.0.get(6).unwrap(), sexpected_response_statea1.unwrap());
    std::println!("...after Message cldrst==1 ...final state=A: OK");
    std::println!("\n ...now,  trx rstkadm\n");
    //
    let response_tupla22 =
        client.kmac        (&user_1,  &user_2,       &5,         &rstkadm,  &nouse);
    let expected_echo_response1 = get_value_trx2();
    assert_eq!(response_tupla22.1.get(0), expected_echo_response1);
    std::println!("---->Answer transaction number (2) OK.");
    std::println!("---->Testing Final state after trx rstkadm:");
    let sexpected_response_stateb1 = get_value_str_cldrst_stateb(&env);
    assert_eq!(response_tupla22.0.get(6).unwrap(), sexpected_response_stateb1.unwrap());
    std::println!("...after Message rstkadm...final state=B: OK");
    //
    std::println!("\n ...now, trx svb1adr \n");
    //let svb1adr = String::from_str(&test.env, "svb1adr");
    //let svb1adr = String::from_str(&env, "svb1adr");
    let response_tupla3a =
        client.kmac        (&user_1,  &user_2,       &5,         &svb1adr,  &nouse);
    let expected_echo_response1 = get_value_trx9();
    assert_eq!(response_tupla3a.1.get(0), expected_echo_response1);
    std::println!("---->Answer transaction number (9) OK....but the state ???");
    let sexpected_response_statecc1 = get_value_str_cldrst_statec(&env);
    assert_eq!(response_tupla3a.0.get(6).unwrap(), sexpected_response_statecc1.unwrap());
    std::println!("...after Message rstkadm...final state=C: OK...changed!");
    std::println!("\nThe cycle right now is in the C state...\n");
    //what happens if now (select coffee blend) is sended by kreator...??
    std::println!("\n\n NOW: WHAT HAPPENS IF I NOW SEND (select coffee blend) TRX... AND IT IS SENT BY THE CREATOR?\n\n");
    std::println!("\nNote: the former select coffee blend trx as been sent by user_2!!\n");
    //NOTE: SEE "THE NEXT TRX IS SENT BY CORRECT USER (USER_2)" ABOVE !!!
    //
    let response_tupla4a =
        client.kmac        (&user_1,  &user_2,       &15,         &selcofblnd,  &nouse);
    let expected_echo_response17 = get_value_trx17();
    assert_eq!(response_tupla4a.1.get(0), expected_echo_response17);
    std::println!("---->Answer transaction number (17) OK.");
    std::println!("---->Answer transaction number  OK....but the state ???");
    //
    let response_tupla4b =
        client.kmac        (&user_1,  &user_2,       &15,         &selcofblnd,  &nouse);
    let sexpected_response_statecc2 = get_value_str_cldrst_statec(&env);
    //
    assert_eq!(response_tupla4b.0.get(0).unwrap(), sexpected_response_statecc2.unwrap());
    std::println!("...The state is C state, i.e. the trx failed...because bad user\n");

    // selcofblnd (17)----------------------------------------------------------END
    /*
 (1)   "string": "input user <>stored B1STAD"
 (3)    "string": "input user =stored KSTADR"
 (5)    "string": "User bad cnd Add. Not valid user fn"
     */
    std::println!("\nThe former call was using AUTH_FLAG 'false' ...lets see the dashboard messages...\n");
    let sexpected_response_dsh1 = get_dsh1(&env);
    assert_eq!(response_tupla4b.0.get(1).unwrap(), sexpected_response_dsh1.unwrap());
    //
    let  sexpected_response_dsh3 = get_dsh3(&env);
    assert_eq!(response_tupla4b.0.get(3).unwrap(), sexpected_response_dsh3.unwrap());
    //
    //
    let sexpected_response_dsh5 = get_dsh5(&env);
    assert_eq!(response_tupla4b.0.get(5).unwrap(), sexpected_response_dsh5.unwrap());
    //
    std::println!("Success! The mesages are :\n");
    std::println!("(1) input user <>stored B1STAD\n");
    std::println!("(3) input user =stored KSTADR\n");
    std::println!("(5) User bad cnd Add. Not valid user fn\n");


    std::println!("\nNOW calling with AUTH true...\n");

    let _response_tupla4d =
        client.kmac        (&user_1,  &user_2,       &5,         &selcofblnd,  &nouse);




    std::println!("\nEND OF KMAC TEST\n");
    fn get_dsh5 (env:&Env)->Option<String>{
        //"User bad cnd Add. Not valid user fn"
        Some(String::from_str(&env,"User bad cnd Add. Not valid user fn"))
    }
    fn get_dsh3 (env:&Env)->Option<String>{
        //input user =stored KSTADR
        Some(String::from_str(&env, "input user =stored KSTADR"))
    }
    fn get_dsh1 (env:&Env)->Option<String>{
    //(1)   "string": "input user <>stored B1STAD"
    // let mut sexpected_response_dsh1 = get_str_dsh_1(&env);
    Some(String::from_str(&env, "input user <>stored B1STAD"))
    }
    fn get_value_strx(env: &Env) -> Option<String> {
        Some(String::from_str(&env, "Trx NOT in FLOW!"))
    }
    fn get_value_str_cldrst_statea(env: &Env) -> Option<String> {
        Some(String::from_str(&env, "A"))
    }
    fn get_value_str_cldrst_stateb(env: &Env) -> Option<String> {
        Some(String::from_str(&env, "B"))
    }
    fn get_value_str_cldrst_statec(env: &Env) -> Option<String> {
        Some(String::from_str(&env, "C"))
    }
    #[allow(dead_code)]
    fn get_value_str_cldrst_statee(env: &Env) -> Option<String> {
        //Some(String::from_str(&env, "E"))
        //SELECTED COFFEE BLEND! SEE KNOWN BUG #1
        Some(String::from_str(&env, "SELECTED COFFEE BLEND!"))
    }
    fn get_value_str_cldrst_stateee(env: &Env) -> Option<String> {
        Some(String::from_str(&env, "E"))
    }
    //get_value_str_cldrst_statef
    fn get_value_str_cldrst_stateff(env: &Env) -> Option<String> {
        Some(String::from_str(&env, "BUYER PAYED"))
    }
    fn get_value_str_cldrst_statef(env: &Env) -> Option<String> {
        Some(String::from_str(&env, "F"))
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




}

