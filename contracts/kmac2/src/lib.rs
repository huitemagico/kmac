#![no_std]
/// kmac2.0 delivery stage 2 rev December 19, 2023
/// kmac implement an finite state machine template.
/// For how to use see https://github.com/huitemagico/kmac/wiki
/// The lib.rs is the main program. Uses an external module called kmacusermod.rs
/// where the main functions are:
///  ext fn"put_coordinates".
/// fn msj_t_num_usr
/// fns function7 to function15 these is for user logic
/// fn put_corefunc()
/// The explanation of these functions are at http://
/// 
mod kmacusermod;
use soroban_sdk::{contract, contractimpl,contracttype, symbol_short,Address,  vec, Env, String, Symbol, Vec,log,BytesN};
#[contracttype]
pub enum DataKey {
    Kstad(Address), //Counter--> Kreator Stored Address
}
pub enum DataKey2 {
    UsrCnt,
    UsrAdr(BytesN<32>),
    UsrNme,
}
pub trait PutMat{
    fn put_coordinates (&self, env: Env) -> [(usize, usize); 9];
  }
pub trait InitMat{
     fn put_corefunc (&self, env: Env) -> [[i32; 6]; 6];
}
//com  fn extfunc1 (&self, env: Env) -> [[i32; 6]; 6];

#[repr(u32)]
pub enum B1Error {
    NoExist=1,
    NoMatch=2,
}
const COUNTER: Symbol = symbol_short!("COUNTER");
const OLD_MSG: Symbol = symbol_short!("OLD_MSG");
const KSTADR: Symbol = symbol_short!("KSTADR");
const B1STAD: Symbol = symbol_short!("B1STAD"); 
const MCSTAT: Symbol = symbol_short!("MCSTAT"); 
//state of the machine !!

mod test;

#[contract]
pub struct KmacContract;

#[contractimpl]
impl KmacContract {
//let response_tupla =
//               client.kmac        (&user_1,  &user_1,       &5,         &cldrst,  &first_trx);
       pub fn kmac    (env: Env,  user: Address, buyer: Address, _value: u32, message: String, _sender:String ) -> 
       //(  u32,u32, u32, u32, Vec<String>, bool,i32,i32,i32,i32,i32,i32) {
        (  
             Vec<String>, Vec<i32>) {
  


env.events()
            .publish((COUNTER, symbol_short!("buy")), buyer.clone());
let gral_trace :bool=false;

//let mut stringbool1ad:&String;
//let mut stringbool2ad:&String;

let stringbool2= String::from_str(&env, ".");
let stringbool3= String::from_str(&env, ".");
let stringbool4= String::from_str(&env, ".");
let stringbool5= String::from_str(&env, ".");
let stringbool6= String::from_str(&env, ".");
let stringbool7= String::from_str(&env, ".");
//
let testmsjguardado1 = "???";
let mut smsg1= String::from_str(&env, testmsjguardado1);
//

let mut smsg2= String::from_str(&env, testmsjguardado1);

let mut smsg3= String::from_str(&env, testmsjguardado1);
let mut smsg4= String::from_str(&env, testmsjguardado1);
let mut smsg5= String::from_str(&env, testmsjguardado1);
let mut smsg6= String::from_str(&env, testmsjguardado1);
let mut smsg7= String::from_str(&env, testmsjguardado1);


let mut  veci32:           [i32; 20]= [0;20];
let mut iveci32:usize = 0;
let mut inummsg:usize = 0;

let scldrst =String::from_str(&env, "cldrst");
let rstkadm = "rstkadm";
let srstkadm = String::from_str(&env, rstkadm);
let svb1adrms = String::from_str(&env, "svb1adr");
let selcofblnd = String::from_str(&env, "selcofblnd");
let resetmessage = String::from_str(&env, "reset"); 
let clonemsg=  message.clone();
//
//NOTE: the "veci32[]-tracer" of the result panel is FIXED. This means that the code of the
//     core is NOT possible change, (unless you know what that means)
let mut nummsg=msjtonum (&clonemsg, &scldrst,&srstkadm,&svb1adrms,&selcofblnd);
if nummsg ==9999 {
    nummsg=kmacusermod::msj_t_num_usr (&env,&clonemsg);
    //usr function "convert string message to number"
}
let stringbool1= String::from_str(&env, "numm msg");  
if  iveci32<19 {veci32[iveci32] = nummsg;iveci32=iveci32+1;}
//
//|--------|--------|--------|--------|--------|--------|--------|
//|--------|--------|--------|--------|--------|--------|--------|--------|
//|        | A      | B      |  C     | D      | E      |        |        |
//|--------|--------|--------|--------|--------|--------|--------|--------|
//|        | 0      | 1      | 2      | 3      | 4      |  5     |        |
//|--------|--------|--------|--------|--------|--------|--------|--------|
//|--------|--------|--------|--------|--------|--------|--------|--------|
//|    A   | 1 (1)  | 2 (2)  |        |        |        |        |   0    |
//|--------|--------|--------|--------|--------|--------|--------|--------|
//|    B   |        |        | 9(3)   |        |        |        |   1    |
//|--------|--------|--------|--------|--------|--------|--------|--------|
//|    C   |        |        | 15     |  16(4) |  17()  |        |   2    |
//|--------|--------|--------|--------|--------|--------|--------|--------|
//|    D   | 19(5)  |        | 21 (6) |        |        |        |   3    |
//|--------|--------|--------|--------|--------|--------|--------|--------|
//|    E   |        |        |        |        |        |        |   4    |
//|--------|--------|--------|--------|--------|--------|--------|--------|
//|        |        |        |        |        |        |        |   5    |
//|--------|--------|--------|--------|--------|--------|--------|--------|
//|        |        |        |        |        |        |        |        |
//|--------|--------|--------|--------|--------|--------|--------|--------|
//|        |        |        |        |        |        |        |        |
//|--------|--------|--------|--------|--------|--------|--------|--------|



// ---------MATRIXNUM (set_matrix_num) fill matrix with consecutive numbers ----------------BEGIN
//  matrixnum: matriz de numeros. Inicializados 12 3 etc fila a fila.
// This block is in mainlib.rs(main) .
// Function: this block set the cells with 1,2,3 consecutive integers.
// That means "EXISTS TRX"

log!(&env, "logg standard default init values for the matrix number");
//
let mut matrixnum: [[i32; 6]; 6] = [[0; 6]; 6];
    let mut number = 1;
    for row in 0..6 { 
        for col in 0..6 {
            matrixnum[row][col]=number;
            number += 1;
        }
    }
//
//---------MATRIXNUM (step1)END fill matrix_num with consecutive numbers------------END
//   
//---------matrixnum (step 2) put 99999 WHERE NO EXISTS trx------------------------------BEGIN
//After the step above (set_matrix_num), the main program call the ext fn"put_coordinates".
//This function init coordinates corresponding cell (i,j)  where NO exists transaction 
//The convention is that the value  9999 in cell (i:0...,j:0..) means that the trx (i,j) does not exist.
//

let coordinates = kmacusermod::put_coordinates();
log!(&env, "logging when the coordinates put the values in matrixnum ");

 for row in 0..5{
    for col in 0..5 {
        for col1 in 0..35 {
            //log!(&env, "current_contract_add: {}", current_contract_add);
            let (coordx,coordy)= coordinates[col1];
            if coordx==row && coordy==col {
                matrixnum[row][col]=9999;

            }
        }
        

    }
}
//
    if gral_trace {
    inummsg=inummsg+1;
    if inummsg <8{
        let matrixnuminit  = String::from_str(&env, "MATRIXNUM INIT end.");
        savemsg (&env, inummsg, &mut smsg1, &mut smsg2,
            &mut smsg3,
            &mut smsg4,
            &mut smsg5,
            &mut smsg6,
            &mut smsg7,
            matrixnuminit);
        
    }
    }


log!(&env, "CORE init functions definition for the matrix function");
let corefuncmat: [[i32; 6]; 6] ;
corefuncmat = kmacusermod::put_corefunc();
let mut filita:i32;
let mut cmnita:i32;
for row in 0..5 { 
    for col in 0..5 {
        if corefuncmat [row][col] !=0 {
            filita=row as i32;
            cmnita=col as i32;
            log!(&env, "corefuncmat value row {} col {} value {}",filita,cmnita,corefuncmat [row][col] );
            if  iveci32<19 {veci32[iveci32] = filita;iveci32=iveci32+1;}
            if  iveci32<19 {veci32[iveci32] = cmnita;iveci32=iveci32+1;}
            if  iveci32<19 {veci32[iveci32] = corefuncmat [row][col];iveci32=iveci32+1;}

        }
        
        
    }
}
if  iveci32<19 {veci32[iveci32] = 77777;}
    if gral_trace {
    inummsg=inummsg+1;
    if inummsg <8{
        let corefuncmatend  = String::from_str(&env, "COREFUNCMAT end.");
        savemsg (&env, inummsg, &mut smsg1, &mut smsg2,
            &mut smsg3,
            &mut smsg4,
            &mut smsg5,
            &mut smsg6,
            &mut smsg7,
            corefuncmatend);
        
    }
    }
//

//

         let mystorage = env.storage();
         let key = symbol_short!("key99");
         mystorage.persistent().set(&key, &1);
         assert_eq!(mystorage.persistent().has(&key), true);
         assert_eq!(mystorage.persistent().get::<_, i32>(&key), Some(1));
         let booleantype :bool;
         booleantype=mystorage.persistent().has(&key);
         log!(&env, "booleantype when there is the key: %s", booleantype);
      
//
//
        let _old_message = "nomessage";
// (a)    save for work message received
        let ln1: u32;
        ln1 = message.len();  //length of message received
       if  iveci32<11 {veci32[iveci32] = ln1 as i32;iveci32=iveci32+1;}
        
        let mut count: u32 = env.storage().persistent().get(&COUNTER).unwrap_or(0); 
env.events()
            .publish((COUNTER, symbol_short!("increment")), count);
        count += 1;
        if  iveci32<11 {veci32[iveci32] = ln1 as i32;iveci32=iveci32+1;}
        let mut old_message = env
            .storage()
            .persistent()
            .get(&OLD_MSG)
            .unwrap_or(String::from_str(&env, "NoOldMessage0"));

        if message == resetmessage {
            old_message = String::from_str(&env, "ResetMessageStored");
        }
        env.storage().persistent().set(&OLD_MSG, &message);
        env.storage().persistent().set(&COUNTER, &count);
//


//let strfalse  = String::from_str(&env, "KeyNoExists");
//let kstradrex=String::from_str(&env, "(srstkadm)KSTADR existed!-panic?");
let cloneuser=  user.clone();

log!(&env, "cloneuser: {}", cloneuser);
let cloneuser2= cloneuser.clone();
log!(&env, "cloneuser2: {}", cloneuser2);
let cloneuser3=  cloneuser2.clone();

//
let crow:usize ;
let ccol:usize ;
(crow,ccol)= findcoor(matrixnum,nummsg);
//
let fnnumber:i32; 
fnnumber= corefuncmat [crow][ccol];
let  rwval= crow as i32;
let  clval= ccol as i32;
log!(&env, "corefuncmat returns fn number {} with row {} col {}",fnnumber, rwval, clval);
        if gral_trace {
        inummsg=inummsg+1;
        if inummsg <8{
        let getfnnumend  = String::from_str(&env, "GET FN NUM END");
        savemsg (&env, inummsg, &mut smsg1, &mut smsg2,
            &mut smsg3,
            &mut smsg4,
            &mut smsg5,
            &mut smsg6,
            &mut smsg7,
            getfnnumend);
        
    }
    }
let mut crstatec = String::from_str(&env, "X");
if is_init_stat(&env) { 
        crstatec =get_curr_st(&env);
        //stringbool1=set_keyb   (&env , clonebuyer);
   }
let curr_state=crstatec;
let mut curr_state2=curr_state.clone ();
inummsg=inummsg+1;
if inummsg <8{
    //let nostatems  = String::from_str(&env, "MCSTAT not existed");
    savemsg (&env, inummsg, &mut smsg1, &mut smsg2,
        &mut smsg3,
        &mut smsg4,
        &mut smsg5,
        &mut smsg6,
        &mut smsg7,
        curr_state);
    
}

let  initstatefmsg : String;//::from_str(&env, "X");
let  endstatefmsg ;//= String::from_str(&env, "X");
initstatefmsg=get_str_st( &env ,crow) ;
endstatefmsg=get_str_st( &env ,ccol) ;
//let str_trx_bad  = String::from_str(&env, "Trx DECLINED");
let  is_in_flow:bool;
let st_ad_cndu:u32;
let current_contract_add = env.current_contract_address();
log!(&env, "current_contract_add: {}", current_contract_add);
let _code:u32;
//-------------------------------------------------------------------------------
// Allow list beta step 1 : is the allowed buyer?---------------------BEGIN
// Ask if the address of the buyer is really a buyer address stored
// This code really can go after knowing if is useful knows this
// condition, in this case if the trx is svb1ad. But for the 
// moment I think is better test it always.
// st_ad_cndu: stored address condition for user parameter
st_ad_cndu=
cmp_addr_b (&env,user);
let  mkstrnoex= String::from_str(&env, "stored B1STAD didnt existed");
let  mkstradeq= String::from_str(&env, "input user =stored B1STAD");
let  mkstradneq= String::from_str(&env, "input user <>stored B1STAD");
match st_ad_cndu {
    //inp buy ad !=stored B1STAD
    1=>{
       inummsg=inummsg+1;
       if inummsg <8
      { savemsg (&env, inummsg, &mut smsg1, &mut smsg2,&mut smsg3, &mut smsg4,&mut smsg5,&mut smsg6,
       &mut smsg7, mkstradneq  );}
       inummsg=inummsg+1;
        }

    2=> {
        //inp buy ad =stored B1STAD
        inummsg=inummsg+1;
       if inummsg <8
      { savemsg (&env, inummsg, &mut smsg1, &mut smsg2,&mut smsg3, &mut smsg4,&mut smsg5,&mut smsg6,
       &mut smsg7, mkstradeq  );}
    }
    3=> {
        //stored B1STAD didnt existed
        inummsg=inummsg+1;
       if inummsg <8
      { savemsg (&env, inummsg, &mut smsg1, &mut smsg2,&mut smsg3, &mut smsg4,&mut smsg5,&mut smsg6,
       &mut smsg7, mkstrnoex  );}
    }
    _=> {
        log!(&env, " ???");
        }

}
// The former analysys only for logging at this moment.
// Allow list beta step 1 : is the allowed buyer?------------------------------------END 
//--------------------------------------------------------------------------------------------------
//----Set the condition: (Stored Addres vs User Address): KSTRAD--st_ad_cndk----------------------BEGIN
let st_ad_cndk:i32;
st_ad_cndk=
cmp_addr_k (&env,&cloneuser3);
let  mkadmnoex= String::from_str(&env, "stored KSTADR didnt existed");
let  mkadmeq= String::from_str(&env, "input user =stored KSTADR");
let  mkadmneq= String::from_str(&env, "input user <>stored KSTADR");
match st_ad_cndk {
    //inp buy ad !=stored B1STAD
    1=>{
       inummsg=inummsg+1;
       if inummsg <8
      { savemsg (&env, inummsg, &mut smsg1, &mut smsg2,&mut smsg3, &mut smsg4,&mut smsg5,&mut smsg6,
       &mut smsg7, mkadmneq  );}
       inummsg=inummsg+1;
        }

    2=> {
        //inp buy ad =stored B1STAD
        inummsg=inummsg+1;
       if inummsg <8
      { savemsg (&env, inummsg, &mut smsg1, &mut smsg2,&mut smsg3, &mut smsg4,&mut smsg5,&mut smsg6,
       &mut smsg7, mkadmeq  );}
    }
    3=> {
        //stored B1STAD didnt existed
        inummsg=inummsg+1;
       if inummsg <8
      { savemsg (&env, inummsg, &mut smsg1, &mut smsg2,&mut smsg3, &mut smsg4,&mut smsg5,&mut smsg6,
       &mut smsg7, mkadmnoex  );}
    }
    _=> {
        log!(&env, " ???");
        }

}
//----Set the condition: (Stored Addres vs User Address): KSTRAD--st_ad_cndk----------------------END
//-----------------------------------------------------------------------------------------------
// What kind of caller related-trx is this? -----------------------------------------------------BEGIN
// There are 3 types of trx: 
//(1) Admin 'supervisor': no matter the current state, always proceed. No matter the user address.
//(2) Admin 'flow': have to respect the FSM states. The user, must be the Admin address
//(3) user : have to respect the   FSM states. The user, must be the Buyer Address
//The following function determines the trx kind type of user
//let kndt:i32=kmacusermod::kndtrx(&env,&fnnumber);
let kndt:i32=kmacusermod::kndtrx(&fnnumber);
// Here: kndt is the (1,2,3) condition 
//
// (I)  is_in_flow condition= Fun(kndt); This is for SPV-admin trx:
//: Set the FSM states condition have to respect--------------------BEGIN
// "is_in_flow" flag means the type of user-trx is.
match kndt  {
    //1 admin super, always proceed
    1=>{is_in_flow=true;}
    //2 admin flow FSM States
    2=>{is_in_flow= cmp_curr_prop (&curr_state2, &initstatefmsg);}
    //3 admin flow FSM States
    3=>{is_in_flow= cmp_curr_prop (&curr_state2, &initstatefmsg);}
    //abnormal situation, bad fnnumber ?
    _=> {is_in_flow= cmp_curr_prop (&curr_state2, &initstatefmsg);}
    //not handled error here
}
if is_in_flow != true {
    let  trx_not_inflow= String::from_str(&env, "Trx NOT in FLOW!");
    savemsg(&env, inummsg, &mut smsg1, &mut smsg2, &mut smsg3,
                   &mut smsg4, &mut smsg5, &mut smsg6, &mut smsg7,
                   trx_not_inflow  );
}
//Here kndt; and is_in_flow 
// (I)  Set the FSM states condition have to respect--------------------END AQUI VOY
// (II) Analysys of the appropiate address of the "user" parameter related to trx (allow list)-------BEGIN
// If the trx is admin type, and the trx is NOT the cldrst, the user address must be ==stored admin address
// If the trx is user type, the user address must be ==stored user address
//
// (II) Analysys of the appropiate address of the "user" parameter related to trx (allow list)-------END
// if fnnumber==1 {is_in_flow = true;}
// else  {is_in_flow= cmp_curr_prop (&curr_state2, &initstatefmsg);}






let fnname:String;
fnname=n_to_str (&env,fnnumber);
 inummsg=inummsg+1;
 if inummsg <8
  {
     savemsg (&env, inummsg, &mut smsg1, &mut smsg2,&mut smsg3,
     &mut smsg4,&mut smsg5,&mut smsg6,&mut smsg7,
     fnname  );}
// Here I have the following situation:----------------------------------------------BEGIN
//-----------------------------------------------------------------------------------
// (I)  kndt : kind of trx
//    (1) admin 'supervisor': no matter the current state, always proceed. No matter the user address.
//    (2) Admin 'flow': have to respect the FSM states. The user, must be the Admin address
//    (3) user : have to respect the   FSM states. The user, must be the Buyer Address
//-----------------------------------------------------------------------------------
// (II) is_in_flow :bool: true=> the trx "is in flow" i.e. ok  false=> trx "is not in the flow" (=> abort trx)
//-----------------------------------------------------------------------------------
//st_ad_cndu: stored address condition for the user:
//   1=> the user's address is not equal to the stored buyer's address
//   (note that this is not a priori an error; it's only a condition. It will be an error if the transaction pretends to be a user transaction).
//   2=> the user's address is equal to the stored buyer's address
//   (note that this is not a priori a successful transaction. It will be OK if the transaction is a user transaction. It will be an error
//   if the transaction is an Admin transaction).
//   3=> stored buyer's address does not exist.
//   In summary,'st_ad_cndu' is a condition. It will be an error or success depending on the rest of the context.
//-----------------------------------------------------------------------------------
// (III) st_ad_cndk : stored address condition for the admin
//    1=>input user <>stored KSTADR
//    2=>input user =stored KSTADR
//    3=>stored KSTADR didnt existed
//-----------------------------------------------------------------------------------
// Here I have the following situation:----------------------------------------------END
//call funtions admin or user ------------------------------BEGIN
//depending on kind of trx (admins /user) call two different functions
//reason is only structure. The core design could treat them as one only functions.
// NOTE:Design decision: the validation of correctness of user address for admin fn its made before all.
// So, if trx is admin trx, validate now if its correct his address:
           //
           //
//let fn_adm_val:bool; //is admin function valide?
//let is_admin_fn:bool; //is an admin function?
//res_ad_cndk : the admin address condition is ok?

//if is_admin_fnfn(&fnnumber){
if kndt ==1 || kndt==2 {
    let res_ad_cndk:bool;
    //val_ad_cndk :validate admin condition of (his) key related to the function . See NOTE:Design decision
    res_ad_cndk= val_ad_cndk (&fnnumber, &st_ad_cndk);
    if res_ad_cndk ==false {
        //fn_adm_val=false;
        //address condition key (for) Admin Message
        let ad_cndka= String::from_str(&env, "Admin bad condition Address?");
        inummsg=inummsg+1; if inummsg <8{savemsg (&env, inummsg, &mut smsg1, &mut smsg2,&mut smsg3,
            &mut smsg4,&mut smsg5,&mut smsg6, &mut smsg7,ad_cndka);}    
    }else {
        let _ad_cndks2= String::from_str(&env, "Admin cnd OK. Valid admin fn");
    }
}
//here I know that, if the function is admin function, can be valid or not (fn_adm_val)
//the former conditions have being logged
//usr step:
let fn_usr_val:bool; //is user function valide?
//let is_usr_fn:bool; //is an usr function?
//st_usr_cndk : stored usr address cnd (for) the Key

//if is_usr_fnfn(&fnnumber){
if kndt ==3 {
    //let res_usr_cndk:bool;
    //validate if the stored address condition is valid for user buyer
    //st_ad_cndu: stored address condition for the user:
    //res_usr_cndk= val_usr_cndk (&fnnumber, &st_ad_cndu);
    fn_usr_val= val_usr_cndk ( &st_ad_cndu);
    //really ALL the user function must be signed by the buyer...but could be useful in the future versions having parametric values
    if fn_usr_val ==false {
        ////address condition key (for) User Message
        let ad_cndku= String::from_str(&env, "User bad cnd Add. Not valid user fn");
        inummsg=inummsg+1; if inummsg <8{savemsg (&env, inummsg, &mut smsg1, &mut smsg2,&mut smsg3,
            &mut smsg4,&mut smsg5,&mut smsg6, &mut smsg7,ad_cndku);}    
    }else {
        let _ad_cndks2= String::from_str(&env, "User Addr cnd OK.");
       // fn_usr_val=true;
    }
}



//if the fn is userfn, answer must be true
//result of function of admin
let res_fn_ad:i32;
//result of function of user 
//let res_fn_us:i32;
if fnnumber ==1 || fnnumber ==2 || fnnumber ==3 {
//if kndt==1  {
    //These are ADMIN-CORE ... NOT ADMIN-FLOW functions

    //fn admin_fn(env:&Env ,kndt:&i32,fnnumber:&i32,st_ad_cndk:&i32,is_in_flow:&bool, usr_ad_fn:&Address, buyer_ad:&Address)->i32{
    res_fn_ad=admin_fn(&env,&kndt,&fnnumber,&st_ad_cndk,&is_in_flow,&cloneuser3,&buyer);
    let ms_fnad_bad= String::from_str(&env, "Admin Function Error");
    let ms_fnad_ok= String::from_str(&env, "Admin Function OK");
    if res_fn_ad > 0{
        inummsg=inummsg+1; if inummsg <8{ savemsg (&env, inummsg, &mut smsg1, &mut smsg2, &mut smsg3,&mut smsg4, &mut smsg5,
              &mut smsg6,&mut smsg7,ms_fnad_ok);}
    }
    else {
        inummsg=inummsg+1; if inummsg <8{ savemsg (&env, inummsg, &mut smsg1, &mut smsg2, &mut smsg3,&mut smsg4, &mut smsg5,
            &mut smsg6,&mut smsg7,ms_fnad_bad);}
    }
}else
//  ...user functions ... USER or ADMIN-FLOW!
{
    let ms_usfn_bad= String::from_str(&env, "USER Function Error");


    //really ALL the user function must be signed by the buyer...but could be useful in the future versions having parametric values
    //fn_usr_val should be at this step already init. but the compiler
    //says no... pending analysys.
    let fn_usr_val1:bool;
    let  ms_uf_procok :String   ;//= String::from_str(&env, "USER Function Proc OK");
    let  ms_uf_proc_fail: String;//= String::from_str(&env, "USER Function Proc FAIL");
    //here is possible user function (3) or admin.flow...
    if kndt ==3 {
        fn_usr_val1= val_usr_cndk ( &st_ad_cndu);
        ms_uf_procok = String::from_str(&env, "USER Function Proc OK");
        ms_uf_proc_fail= String::from_str(&env, "USER Function Proc FAIL");
    }else {
        //res_ad_cndk= val_ad_cndk (&fnnumber, &st_ad_cndk);
        //this is already setted... bt not inthis scope.. :-/
        fn_usr_val1= val_ad_cndk (&fnnumber, &st_ad_cndk);
        ms_uf_procok = String::from_str(&env, "ADMIN FLOW Function Proc OK");
        ms_uf_proc_fail= String::from_str(&env, "ADMIN FLOW Function Proc FAIL");

    }

    if fn_usr_val1==true && is_in_flow==true {
        let us_b:bool;
        let us_msg:String;
        //fn user_fn(env:&Env ,nextstate:&i32,fnnumber:&i32, usr_ad_fn:&Address, buyer_ad:&Address)->(String, bool){

         (us_msg, us_b)=   user_fn(&env,&endstatefmsg, &fnnumber);
         if us_b {
             //let ms_uf_procok= String::from_str(&env, "USER Function Proc OK");
             inummsg=inummsg+1; if inummsg <8{ savemsg (&env, inummsg, &mut smsg1, &mut smsg2, &mut smsg3,&mut smsg4, &mut smsg5,
                                                        &mut smsg6,&mut smsg7,us_msg);}
             inummsg=inummsg+1; if inummsg <8{ savemsg (&env, inummsg, &mut smsg1, &mut smsg2, &mut smsg3,&mut smsg4, &mut smsg5,
                                                        &mut smsg6,&mut smsg7,ms_uf_procok);}
         }else {
             //let ms_uf_proc_fail= String::from_str(&env, "USER Function Proc FAIL");
             inummsg=inummsg+1; if inummsg <8{ savemsg (&env, inummsg, &mut smsg1, &mut smsg2, &mut smsg3,&mut smsg4, &mut smsg5,
                                                        &mut smsg6,&mut smsg7,ms_uf_proc_fail);}
         }
        //,&cloneuser3,&buyer
    }else {
        inummsg=inummsg+1; if inummsg <8{ savemsg (&env, inummsg, &mut smsg1, &mut smsg2, &mut smsg3,&mut smsg4, &mut smsg5,
            &mut smsg6,&mut smsg7,ms_usfn_bad);}
    }
    
    
}
//these two could return samemmessage return.BUT having different error conditions...each one with his own error.

//LOGG the return message


//
//res_fnm=admin_fn(&env,&kndt,&fnnumber,&st_ad_cndk,is_in_flow);
//fn trx_proc( env:&Env , endstatefmsg:&String)->  Result<u32, ()> {

curr_state2=get_curr_st(&env);
 inummsg=inummsg+1; if inummsg <8{savemsg (&env, inummsg, &mut smsg1, &mut smsg2,&mut smsg3,
    &mut smsg4,&mut smsg5,&mut smsg6, &mut smsg7,curr_state2);}
 let str = "123";
 let num: i32 = str.parse().unwrap();
 log!(&env, "string to  number {}", num);

let mut colors = ["red", "green", "blue"];
   colors[0]="green";

let msg = "kmac v3.0 = final deliverable2= December 19 20:49:-2023"; //version message
let sout = String::from_str(&env, msg);

 return (
    vec![&env, 
    smsg1, smsg2,
    smsg3, smsg4,
    smsg5, smsg6,
    smsg7, 
    sout, old_message, message
    , stringbool1, stringbool2,stringbool3,stringbool4,
    stringbool5,stringbool6,stringbool7
    ],
     //veci32
     vec![&env, veci32[0], veci32[1], veci32[2],veci32[3], veci32[4], veci32[5],veci32[6], veci32[7], veci32[8],veci32[9],
     veci32[10], veci32[11], veci32[12],veci32[13], veci32[14], veci32[15],veci32[16], veci32[17], veci32[18],veci32[19]]

);
//}
//fn val_usr_cndkn : set the condition true or false for the buyer trx, based on the function number
// and the stored address condition of the buyer address


//val_ad_cndk :validate admin condition of (his) key related to the function 
//res_ad_cndk= val_ad_cndk (&fnnumber, &st_ad_cndk);
fn val_ad_cndk(fnnumber: &i32,st_ad_cndu: &i32)->bool {
//See NOTE:Design decision
// Design decision: the validation of correctness of user address for admin fn its made before all.
// So, if trx is admin trx, validate now if its correct his address:
//Note (b) According to the procedure, restoring k address must be after cold reset. See diagram at wiki.
//Coldreset doesnt touch the admin address, only set the state of the machine.
// Then the only possible reason for having an address stored is after cold reset (b), and
// having the same identity of the contract. This happens always after succesive coldreset without a new deploy.
// Note (a) This is an strange situation normally must not be happen. Even there is no problem with the logic,
//I think is better marking as error and analyze the situation.
match *fnnumber {
    1=>{
        if *st_ad_cndu==1 {return true;}
        else {
            if *st_ad_cndu==2 {return true;}
            else {
                if *st_ad_cndu==3 {return true;}
                else {return false;}// internal error!!
            }
        }
    }//1
    2=>{
        if *st_ad_cndu==1 {return false;}
        else {
            if *st_ad_cndu==2 {return true;}
            else {
                if *st_ad_cndu==3 {return true;}
                else {return false;}// internal error!!
            }
        }
    }
    3=>{
        if *st_ad_cndu==1 {return false;}
        else {
            if *st_ad_cndu==2 {return true;}
            else {
                if *st_ad_cndu==3 {return false;}
                else {return false;}// internal error!!
            }
        }
    }
    _=> {
        //by definition, every ADMIN function must be with the ADMIN address
        //INCLUDED trx of 'admin flow' i.e. those defined by the builder-user
        if *st_ad_cndu==1 {return false;}
        else {
            if *st_ad_cndu==2 {return true;}
            else {
                if *st_ad_cndu==3 {return false;}
                else {return false;}// internal error!!
            }
        }
    }
}
// if *fnnumber==1 && *st_ad_cndu==1 {return true;}
// if *fnnumber==1 && *st_ad_cndu==2 {return true;}
//if *fnnumber==1 && *st_ad_cndu==3 {return true;}
// if *fnnumber==2 && *st_ad_cndu==1 {return false;}
// if *fnnumber==2 && *st_ad_cndu==2 {return true;}
// if *fnnumber==2 && *st_ad_cndu==3 {return false;}
// if *fnnumber==3 && *st_ad_cndu==1 {return false;}
// if *fnnumber==3 && *st_ad_cndu==2 {return true;}
// if *fnnumber==3 && *st_ad_cndu==3 {return false;}

}


fn val_usr_cndk ( st_ad_cndu: &u32)->bool{
    //really ALL the user function must be signed by the buyer...but could be useful in the future versions having parametric values
    //so, for now, the unique condition is that input address must be == buyer stored address 
    if *st_ad_cndu ==2 
    {return true;}
    else {return false;}
}


// fn is_admin_fnfn(fnnumber:&i32)->bool {
//     //by core definition  (bypassing enetually usersetting) the (1,2,3) ARE admin functions
//     if *fnnumber ==1 || *fnnumber ==2 || *fnnumber ==3 {
//         return true;
// }else {
//     return false; }
// }
// fn is_usr_fnfn(fnnumber:&i32)->bool {
//     //must be external
//     //
//     if *fnnumber !=1 && *fnnumber !=2 && *fnnumber !=3 {
//         return true;
// }else {
//     return false; }
// }
//



fn user_fn(env:&Env ,nextstate:&String,fnnumber:&i32)->(String, bool){
    //usr_ad_fn:&Address,
    //,  buyer_ad:&Address
    //1,2,3 function number defined by design are "admin core".
    // Rest of functions could be admin flow, or user.
    //let fn_msg= String::from_str(&env, "function code but trx_proc Error! (internal)");
    let fn_msg :String;
    let fn_err= String::from_str(&env, "function code but trx_proc Error! (internal)");
    let cc:bool;
    //let resbool:bool;
    //call the functions that have been created (eventually) by the "builder" and the "user"
    //see explanations on kmac wiki
    //the user must not edit lib.rs
    //every "user" (or admin flow) functions must return the same answer.
    match *fnnumber   {
       4=>{
        (fn_msg, cc)= kmacusermod::function4(&env);
       }
       5=>{
        (fn_msg, cc)= kmacusermod::function5(&env);
       }
        6=>{
            (fn_msg, cc)= kmacusermod::function6(&env);
        }
        7=>{
            (fn_msg, cc)= kmacusermod::function7(&env);
        }
        8=>{
            (fn_msg, cc)= kmacusermod::function8(&env);
        }
        9=>{
            (fn_msg, cc)= kmacusermod::function9(&env);
        }
        10=>{
            (fn_msg, cc)= kmacusermod::function10(&env);
        }
        11=>{
            (fn_msg, cc)= kmacusermod::function11(&env);
        }
        12=>{
            (fn_msg, cc)= kmacusermod::function12(&env);
        }
       _=> {
        //internal error impossible here!
           fn_msg= String::from_str(&env, "function code but trx_proc Error! (internal)");
           return (fn_msg,false);
        }
    }//match
    //y cada funcion debiera responder un cc y un mensaje
    if cc==true {
       let resbool=trx_proc1 (&env, &nextstate)  ;
       if resbool==true {
        return (fn_msg,true);
       }
       else {
        return (fn_err,false);
       }
    } else {
        return (fn_msg,false);
    }
}
        
//}



fn admin_fn(env:&Env ,_kndt:&i32,fnnumber:&i32,st_ad_cndk:&i32,is_in_flow:&bool, usr_ad_fn:&Address, buyer_ad:&Address)->i32{
    //admin_fn : work for admin functions
    //let res:i32;
    match *fnnumber 
        {
        1=> {
        //admin cldrst BY DEFINITION OF CORE
        let cldrstres:bool;
        cldrstres=coldreset (&env);
        if cldrstres ==true {
            return 100;
         }else {
            return -1;
               }
        }//1 case
          
        2=>{
           //admin rstkadm BY DEFINITION OF CORE(*)
            if *is_in_flow {
                if *st_ad_cndk == 2 || *st_ad_cndk == 3 {
                    //by definition trx 2 must be admin and flow type
                    //let strx2_m1:String;
                    //let strx2_m2: String;
                    //needed parameters: K adress
                    let resbl: bool;
                    resbl = reset_key2(&env, usr_ad_fn);
                    // set the KSTR key AND set the state.
                    if resbl == true {
                        return 101;
                    } else {
                        return -2;
                    }
                } else {
                    //admin trx not consistent with stored address FALTA MESSAGE
                    return -3;
                }
            }//is in flow
            else {
                //not in flow
                return -4;
            }
        
        }//case 2
             
        3=> {    
            //function 3: admin svb1add
            let resbl2:bool;
            resbl2=set_keyb2   (&env , buyer_ad);
            if resbl2==true {
                return 102;
             }else {
                return -3;
             }
                       
        } //case 3
        _=> {
            //internal error impossible here!FALTA MESSAGE
            return -100;
        }
        
    }//match 
    
    }//fn admin_fn 
    
    



// fn trx_proc( env:&Env , endstatefmsg:&String)->  Result<u32, ()> {
//       let result=set_state (&env,&endstatefmsg);
//       match result {
//           Ok(code) => log!(&env, "ok {} Ok", code),
//           Err(code) => log!(&env, "Error {}", code),
//       }
//       return Ok(0);     
 
// }
           fn trx_proc1( env:&Env , endstatefmsg:&String)->  bool {
               let answ:bool;
               let result=set_state (&env,&endstatefmsg);
               match result {
                   Ok(code) => {log!(&env, "ok {} Ok", code);
                   answ=true;},
                   Err(code) => {log!(&env, "Error {}", code);
                   answ=false;},
               }
               return answ;

           }



fn cmp_curr_prop (r_crstatec:&String, r_initstatefmsg:&String)->bool{
    
    return *r_crstatec == *r_initstatefmsg;
}



fn get_str_st( env:&Env ,index: usize) -> String {
    let sta: String;
       sta = String::from_str(&env, "A");
       
       let stb = String::from_str(&env, "B");
       let stc = String::from_str(&env, "C");
       let std = String::from_str(&env, "D");
       let ste = String::from_str(&env, "E");
       let stf = String::from_str(&env, "F");
       let stz:String;
       let str = String::from_str(&env, "R");
    match  index {
        0 => {
            stz=sta;
        }
        1 => {
            stz=stb;
        }
        2 => {
            stz=stc;
        }
        3 => {
            stz=std;
        }
        4 => {
            stz=ste;
        }
        5 => {
            stz=stf;
        }
        _=> {stz=str;}
      }
      stz
}

// fn get_message<'a>(messages: &'a [&str], index: usize) -> Option<&'a str> {
//     if index < messages.len() {
//         Some(messages[index])
//     } else {
//         None
//     }
// }


// fn savemsg -------------------------------------------BEGIN
//This 'savemsg' function manages a set of 'receivers' that are filled according to the nummsj.
// In other words, it is a rustic implementation of something similar to storing in an array,
// which I haven't been able to program. :-/
// The receivers are placed as is on the "dashboard" line.
// Objective: a handmade implementation of a log or trace.
// There are 7 receivers, arbitrarily.
// The caller is responsible for incrementing the index.
// see https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90technical-description#savemsg
fn savemsg (env:&Env ,nummsj: usize, msg1: &mut String,msg2: &mut String, 
    msg3: &mut String, 
    msg4: &mut String, 
    msg5: &mut String, 
    msg6: &mut String, 
    msg7: &mut String, 
    strfuente: String)->usize {

    match  nummsj {
      1 => {
        *msg1=strfuente;
      }
      2 => {
        *msg2=strfuente;
      }
      3 => {
        *msg3=strfuente;
      }
      4 => {
        *msg4=strfuente;
      }
      5 => {
        *msg5=strfuente;
      }  
      6 => {
        *msg6=strfuente;
      }
      7 => {
        *msg7=strfuente;
      }
      _=> {log!(&env, "no se ???");}
    }
    

return 0
}
// fn savemsg -------------------------------------------END
fn n_to_str(env:&Env,n: i32) -> String {
    let indx:usize =n as usize;
    let stb: String;
    const STR_ARRAY: [&str; 16] = ["Fn 0", "Fn 1", "Fn 2", "Fn 3", "Fn 4", "Fn 5", "Fn 6", "Fn 7", "Fn 8", "Fn 9", "Fn 10", "Fn 11","Fn 12","Fn 13","Fn 14","Fn 15"];  
    if  n<=15 
    { stb = String::from_str(&env, STR_ARRAY[indx as usize]);
    }
    else { stb = String::from_str(&env, "0");}
    return stb;


} 
//fn msjtonum translate the message txt to nummsg
//see https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90technical-description#function-msjtonum
fn msjtonum(msg:&String, scldrst:&String,srstkadm:&String,svb1adrms:&String, selcofblnd:&String) -> i32  {
    let mut nummsg: i32 = 9999;
    //9999 It means that the message could not be translated. It is associated with an internal message equivalent to
    // "I proceed to be able to transmit the condition without having to skip in a tremendous if :-/
    //
    if msg == scldrst      {nummsg=1;} 
    if msg==srstkadm       {nummsg=2;} 
    if msg==svb1adrms      {nummsg=9;}
    if msg==selcofblnd      {nummsg=17;}
    return nummsg;

}
// fn dummyfunc   (env:&Env , user:&Address)->String{
//        let retdummy: String;
//        retdummy = String::from_str(&env, "dummyok");
//        log!(&env, "user en dummy {}", user);
//        return retdummy;
       
    
    
//     }
    
//fn cmp_addr_b (env:&Env,newaddr:Address) -> Result<(), B1Error>{
    //fn cmp_addr_k (env:&Env,newaddr:Address) -> u32{
fn cmp_addr_k (env:&Env,newaddr:&Address) -> i32{
        let kstoredadd: Address;
        if is_initialized(&env) { 
             kstoredadd = env
                .storage()
                .persistent()
                .get(&KSTADR)
                .unwrap();
            let clonestored=  kstoredadd.clone();
        
    let addequal:bool;
    //addequal=clonestored.eq(&newaddr);
    addequal=clonestored.eq(newaddr);
            if addequal ==false
            {
                   log!(&env, "stored KSTADR <> inputaddress");
                  return 1;
            } else {
                return 2;
            }
            
        }else 
            {
                log!(&env, "stored KSTADR didnt existed.");
                return 3;
            }
       
    }
    





// function compare address of business user: (stored address (if exist) with (input address))
//fn cmp_addr_b (env:&Env,newaddr:Address) -> Result<(), B1Error>{
    fn cmp_addr_b (env:&Env,newaddr:Address) -> u32{
    let kstoredadd: Address;
    if is_init_b1ad(&env) { 
         kstoredadd = env
            .storage()
            .persistent()
            .get(&B1STAD)
            .unwrap();
        let clonestored=  kstoredadd.clone();
    
let addequal:bool;
addequal=clonestored.eq(&newaddr);
        if addequal ==false
        {
               log!(&env, "stored B1STAD <> inputaddress");
              return 1;
        } else {
            return 2;
        }
        
    }else 
        {
            log!(&env, "stored B1STAD didnt existed.");
            return 3;
        }
   
}

fn set_keyb2   (env:&Env , user:&Address)->bool{
    //same discuss about TWO functions :-/ and work more about the return value
      let statec: String;
      statec = String::from_str(&env, "C");
      env.storage().persistent().set(&B1STAD, user);
      env.storage().persistent().set(&MCSTAT, &statec);
      return true;
    }

// fn set_keyb   (env:&Env , user:Address)->String{
// let statec: String;
//   statec = String::from_str(&env, "C");
//   let setkeybret = String::from_str(&env, "set_keyb ok");
//   log!(&env, "save the buyer ad (that i have no!");
//   env.storage().persistent().set(&B1STAD, &user);
//   //env.storage().persistent().set(&MCSTAT, &stateb);
//   env.storage().persistent().set(&MCSTAT, &statec);
//   return setkeybret;
// }
//
// fn ch_state_mach (env:&Env )->  Result<u32, ()>{
//     let stateb: String;
//     stateb = String::from_str(&env, "B");
    
//         env.storage().persistent().set(&MCSTAT, &stateb);
     
//    return Ok(0);
// }


fn reset_key2 (env:&Env ,user:&Address)->bool{
// set the KSTR key AND set the state.This function assumes that the condition if exist previously or not
//is already done.
// Discuss if this function ONLY has to set the key...and not set the state ... hmm
//Discuss the kind of return: what happens if storage fail?
    let stateb: String;
    stateb = String::from_str(&env, "B");
    env.storage().persistent().set(&KSTADR, user);
    env.storage().persistent().set(&MCSTAT, &stateb);
    return true;
}

// #[allow(dead_code)]
// fn reset_key (env:&Env ,kstradrex:String, strfalse2:String, user:Address)->(String, String){
//     let stringbool3: String;//::from_str(&env, "trueleyy");
//     let stateb: String;
//     stateb = String::from_str(&env, "B");
//     log!(&env, "reset key kreator");
//     if is_initialized(&env) { // verifica si existia &KSTADR
//         stringbool3=kstradrex;//(savekadmessage)KSTADR existed!-panic?");
        
//     }else 
//      {
//         stringbool3  =strfalse2;
//     }
//         env.storage().persistent().set(&KSTADR, &user);
//         env.storage().persistent().set(&MCSTAT, &stateb);
//         return (stringbool3,stateb);
// }
fn set_state (env:&Env, newstate:&String)->  Result<u32, ()>
{ 
   env.storage().persistent().set(&MCSTAT, newstate);
   return Ok(0);
}
//
//fn coldreset (env:&Env)->  Result<u32, ()>
fn coldreset (env:&Env)-> bool
{
   let statecoldrst = String::from_str(&env, "A");
   env.storage().persistent().set(&MCSTAT, &statecoldrst);
   return true;
}
   fn get_curr_st (env:&Env) -> String {
   let mut crstatec = String::from_str(&env, "X");
   if is_init_stat(&env) { 
    crstatec = env
            .storage()
            .persistent()
            .get(&MCSTAT)
            .unwrap();
    }
    return crstatec
}
// fn cmpaddr (env:&Env,newaddr:Address) -> (String,bool){
//     let booleantype :bool;
//     let  mkstrnoex= String::from_str(&env, "stored KSTADR didnt existed");
//     let  mkstradeq= String::from_str(&env, "new KSTADR == stored KSTADR (??)");
//     let  mkstradneq= String::from_str(&env, "new KSTADR <> stored KSTADR (??)");
//     let  meqadr:String;
//     let kstoredadd: Address;
//     if is_initialized(&env) { 
//          kstoredadd = env
//             .storage()
//             .persistent()
//             .get(&KSTADR)
//             .unwrap();
//         let clonestored=  kstoredadd.clone();
//         if clonestored==newaddr
//         {
//              booleantype=true;
//              log!(&env, "stored KSTADR== new");
//              meqadr=mkstradeq; 
//         } else 
//            {
//                log!(&env, "stored KSTADR <> new");
//                booleantype=false;
//                meqadr=mkstradneq;
//             }
        
//     }else 
//         {
//             log!(&env, "stored KSTADR didnt existed.");
//             booleantype=false;
//             meqadr=mkstrnoex;
//         }

     
//     return (meqadr, booleantype);
// }
//storage function, query if a key exist
//Next version, its necessary having only one function(?)
//See 
//
fn is_init_b1ad(env: &Env) -> bool {
    env.storage().persistent().has(&B1STAD)
    
    
}
  
fn is_initialized(env: &Env) -> bool {
    env.storage().persistent().has(&KSTADR)
  }
fn is_init_stat(env: &Env) -> bool {
    env.storage().persistent().has(&MCSTAT)
    
    
}


fn findcoor(matrix: [[i32; 6]; 6], index: i32) -> (usize, usize){
    let nullrow: usize=0; let nullcol: usize=0;
    for (row_index, row) in matrix.iter().enumerate() {
        if let Some(col_index) = row.iter().position(|&x| x == index) {
            //nullrow=row_index;
            //nullcol=col_index;
            return (row_index, col_index);
        }
    }
  return (nullrow, nullcol);
}

}

 


}
