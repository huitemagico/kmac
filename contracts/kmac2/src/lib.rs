#![no_std]
// kmac2 rev november 22 2023 12:08
// see https://github.com/huitemagico/kmac/wiki#allow-list-implementation
mod mimodulo;
use soroban_sdk::{contract, contractimpl,contracttype, Address, symbol_short, vec, Env, String, Symbol, Vec,log,BytesN};
#[contracttype]
pub enum DataKey {
    Kstad(Address), //Counter--> Kreator Stored Address
}
pub enum DataKey2 {
    UsrCnt,
    UsrAdr(BytesN<32>),
    UsrNme,
}

use mimodulo::plus_two;
pub trait Suma2{
    fn plus_two(&self, env: Env,x: u32) -> u32;
  }
pub trait PutMat{
    fn put_coordinates (&self, env: Env) -> [(usize, usize); 9];
  }
pub trait InitMat{
     fn put_corefunc (&self, env: Env) -> [[i32; 6]; 6];
}
//
const COUNTER: Symbol = symbol_short!("COUNTER");
const OLD_MSG: Symbol = symbol_short!("OLD_MSG");
const KSTADR: Symbol = symbol_short!("KSTADR");
const B1STAD: Symbol = symbol_short!("B1STAD"); 
const MCSTAT: Symbol = symbol_short!("MCSTAT"); //state of the machine !!

mod test;

#[contract]
pub struct KmacContract;

#[contractimpl]
impl KmacContract {

       pub fn kmac    (env: Env,  user: Address, buyer: Address, value: u32, message: String, sender:String ) -> 
       //(  u32,u32, u32, u32, Vec<String>, bool,i32,i32,i32,i32,i32,i32) {
        (  u32,u32, u32, u32, Vec<String>, Vec<i32>) {
 // 
//let my_vector: Vec<i32> = vec![0; 12];
//let coordinates: [(usize, usize); 36] // s
//    let mut coordinates: [(usize, usize); 36] = [(0, 0); 36];
let mut  veci32:           [i32; 20]= [0;20]; // s
let mut iveci32:usize = 0;
// let  anystring0= String::from_slice(&env, "0");
// let  anystring1= String::from_slice(&env, "0");
// let  anystring2= String::from_slice(&env, "0");
// let  anystring3= String::from_slice(&env, "0");
// let  anystring4= String::from_slice(&env, "0");
// let  anystring5= String::from_slice(&env, "0");
// let  anystring6= String::from_slice(&env, "0");
// let  anystring7= String::from_slice(&env, "0");
// let  anystring8= String::from_slice(&env, "0");
// let  anystring9= String::from_slice(&env, "0");
// let mut array: [String; 3] = [anystring0,anystring1,anystring2];
//let  mut  arrstr: [[String; 10]] = [[anystring0,anystring1,anystring2 ,anystring3, anystring4,anystring5,anystring6,anystring7,anystring8,anystring9]];
let scldrst =String::from_slice(&env, "cldrst");
let rstkadm = "rstkadm";
let srstkadm = String::from_slice(&env, rstkadm);
let svb1adrms = String::from_slice(&env, "svb1adr");
let savekadmessage = String::from_slice(&env, "savekad");
let resetmessage = String::from_slice(&env, "reset"); 
let clonemsg=  message.clone();
//
let mut nummsg=0;
if clonemsg == scldrst      {nummsg=1;} 
if clonemsg==srstkadm       {nummsg=2;} 
if clonemsg==svb1adrms      {nummsg=9;}
//if clonemsg==savekadmessage {nummsg=99;}
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
//|    C   |        |        | 15     |  16(4) |        |        |   2    |
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


log!(&env, "logg standard default init values for the matrix number");
//let mut matrixnum: [[i32; 3]; 2] = [[0; 3]; 2];
let mut matrixnum: [[i32; 6]; 6] = [[0; 6]; 6];
    let mut number = 1;
    // let num_rows = matrixnum.len();
    // let num_cols = matrixnum[0].len();
    for row in 0..6 { //6 cols 6 filas
        for col in 0..6 {
            matrixnum[row][col]=number;
            number += 1;
        }
    }
    let str = "123";
   
let coordinates = mimodulo::put_coordinates();
log!(&env, "logging when the coordinates put the values in matrixnum ");
let mut value: i32;
for row in 0..5{
    for col in 0..5 {
        for col1 in 0..35 {
            // value = row as i32;
            // log!(&env, "logg row value ", value , symbol_short!("another"));
            // value = col as i32;
            // log!(&env, "logg col value ", value , symbol_short!("another"));
            // value = col1 as i32;
            let (coordx,coordy)= coordinates[col1];
            if coordx==row && coordy==col {
                // log!(&env, "logg ");
                // log!(&env, "logg {}",value);
                matrixnum[row][col]=9999;

            }
        }
        if matrixnum[row][col]!=9999{
            if  iveci32<19 {veci32[iveci32] = row as i32;iveci32=iveci32+1;}
            if  iveci32<19 {veci32[iveci32] = col as i32;iveci32=iveci32+1;}
            if  iveci32<19 {veci32[iveci32] = matrixnum [row][col];iveci32=iveci32+1;}

        }
        

    }
}
if  iveci32<19 {veci32[iveci32] = 88888;iveci32=iveci32+1;}
log!(&env, "CORE init functions definition for the matrix function");
let corefuncmat: [[i32; 6]; 6] ;
corefuncmat = mimodulo::put_corefunc();
// some logging:
let mut filita:i32;
let mut cmnita:i32;
for row in 0..5 { //6 cols 6 filas
    for col in 0..5 {
        if corefuncmat [row][col] !=0 {
            filita=row as i32;
            cmnita=col as i32;
            log!(&env, "corefuncmat value row {} col {} value {}",filita,cmnita,corefuncmat [row][col] );
            if  iveci32<19 {veci32[iveci32] = corefuncmat [row][col];iveci32=iveci32+1;}

        }
        
        
    }
}
if  iveci32<19 {veci32[iveci32] = 77777;}
//
let current_contract_add = env.current_contract_address();
log!(&env, "current_contract_add: {}", current_contract_add);
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
        
//
// (a)    save for work message received
        let ln1: u32;
        ln1 = message.len();  //length of message received
        
//
        let mut count: u32 = env.storage().persistent().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        count += 1;
//
//

        let mut old_message = env
            .storage()
            .persistent()
            .get(&OLD_MSG)
            .unwrap_or(String::from_slice(&env, "NoOldMessage0"));
//
// let mut kreatorstoredaddress: Address = env
// .storage()
// .persistent()
// .get(&KSTADR)
// .unwrap_or     (cloneuser);
// // log the new or old (?) kreator address...depend on the existence check of KSTADR...
// log!(&env, "new or old (?) kreator address(from stored)", kreatorstoredaddress);
// log!(&env, "booleantype when for KSTADR existence:", booleantype);
// log!(&env, "new (parameter) kreator address", cloneuser0);
// if kreatorstoredaddress==cloneuser0{
//     booleantype=true;
//     log!(&env, "OLD AND NEW ARE THE SAME!");
// }else {
//     log!(&env, "KSTADR RESETED!!!");
// }

//
        if message == resetmessage {
            old_message = String::from_slice(&env, "ResetMessageStored");
        }
        env.storage().persistent().set(&OLD_MSG, &message);
        env.storage().persistent().set(&COUNTER, &count);
//
//
//
        count = plus_two (count);
//
//        o conformarse con trx de buyer.


        let rstkadm = "rstkadm";//reset key administrator
        let srstkadm = String::from_slice(&env, rstkadm);
        //let srstkadm1 = String::from_slice(&env, rstkadm);
        let scldrst =String::from_slice(&env, "cldrst");
        //let scldrst2 =String::from_slice(&env, "cldrst");
//let brwsvkadmsg = &savekadmessage;
log!(&env, "clonemsg: {}", clonemsg);
let mut stringbool1= String::from_slice(&env, "sbool1");
let mut stringbool2= String::from_slice(&env, "trueley22");
let stringbool3= String::from_slice(&env, "trueley22");
let mut stringbool4= String::from_slice(&env, "trueley22");
let stringbool5= String::from_slice(&env, "trueley22");
let mut stringbool6= String::from_slice(&env, "trueley22");
let stringbool7= String::from_slice(&env, "trueley22");
//let csavekadm=savekadmessage.clone();
//let brwmsg = &message;
let strtrue  = String::from_slice(&env, "KeyExists");
let strfalse  = String::from_slice(&env, "KeyNoExists");
let strtrue3  = String::from_slice(&env, "bad trx!(err panic) KeyExists");
let strfalse3  = String::from_slice(&env, "trx accepted,KeyNoExists");
let kstradrex=String::from_slice(&env, "(srstkadm)KSTADR existed!-panic?");
let kstradre2=String::from_slice(&env, "(savekadmessage)KSTADR existed!-panic?");

let moldnewsm=String::from_slice(&env, "(savekad)stored==new (bad flow of init?)");//old and new are the same
let moldnewsm2=String::from_slice(&env, "(savekad)stored==new (bad flow of init?)");//old and new are the same
//
let cloneuser=  user.clone();

log!(&env, "cloneuser: {}", cloneuser);
let cloneuser2= cloneuser.clone();
log!(&env, "cloneuser2: {}", cloneuser2);
let cloneuser3=  cloneuser2.clone();
let cloneuser4=  cloneuser3.clone();
let cloneuser5=  cloneuser4.clone();
let cloneuser6=  cloneuser5.clone();
let cloneuser7=  cloneuser6.clone();
let cloneuser8=  cloneuser7.clone();


let crow:usize ;
let ccol:usize ;
(crow,ccol)= findcoor(matrixnum,nummsg);
//
let fnnumber:i32; 
fnnumber= corefuncmat [crow][ccol];
// some logging:
let  rwval= crow as i32;
let  clval= ccol as i32;
log!(&env, "corefuncmat returns fn number {} with row {} col {}",fnnumber, rwval, clval);
// version kmac1b: PENDING call the functions parametrically depending on the function matrix
// version kmac1b: the functions are called directly depending on the message
// PENDING coldreset must include clean de Kreator Key. 
// Probably this cause the abort when call
// require_auth at message"savekadmessage" (nummsg=2) (??)
// PENDING clean the block if-else replacing for match code and dynamic call of functions
// for documentation of the kmac1b version see the documentation at wiki 
// 

//

match nummsg {
    1 =>{  
        log!(&env, "match nummsg: (1) cldrst call coldreset.");
        (stringbool1, stringbool2)=coldreset (&env, scldrst);
        log!(&env, "match nummsg: (1) cldrst ret {} {}", stringbool1, stringbool2);
        }

    2 =>{  //srstkadm==rstkadm==nummsg=2
        log!(&env, "match nummsg: (2) rstkadm call reset.");
        (stringbool1, stringbool2)=reset_key (&env, kstradrex,strfalse,cloneuser );
        log!(&env, "match nummsg: (2) rstkadm call reset.", stringbool1, stringbool2);
        }
    99 => { 
        log!(&env, "savekadmessage");
    //core function for "savekadmessage"
        //user.require_auth();//aqui abort..auth existing value
        let  boolty :bool;
        let  boolty2 :bool;
        (stringbool1, stringbool2, stringbool4, stringbool6, boolty, boolty2)= 
       savekadd (&env, kstradre2, strtrue,strtrue3,strfalse,strfalse3,user, cloneuser, moldnewsm,
       cloneuser2);
       log!(&env, "stringbool1 {}",stringbool1);
       log!(&env, "stringbool2 {}",stringbool2);
       log!(&env, "stringbool4 {}",stringbool4);
       log!(&env, "boolty {}",boolty);
       log!(&env, "boolty2 {}",boolty2);
    
         }
    9=> {  log!(&env, "svb1adrms");
              stringbool1=set_keyb   (&env , buyer);
        }
    
    _=> {log!(&env, "no se ???");}
        }

//============================================================

  let mut colors = ["red", "green", "blue"];
  colors[0]="green";
  let s = String::from_slice(&env, colors[0]);
//  let s1=s.clone();
//  let s2=s.clone();
//  let s3=s.clone();
//  let idxarr=0;
 

let mensaje1:String;
mensaje1=clonemsg;
        // return (ln1, count, fila1, colu1,vec![&env,  sout, old_message, message, stringbool1, stringbool2,s],booleantype,
        // matrixnum[0][0],matrixnum[0][1],matrixnum[0][2],matrixnum[1][0],matrixnum[1][1],matrixnum[1][2]);
//-----------------------------------------------------------------------------------------------        
//              |coldreset          | reset_key         |
//ln1,          |   
//count,        |
//fila1, 
//colu1,
//vec!
//[sout, 
//old_message, 
//message, 
//stringbool1, |scldrst ="cldrst"   |KSTADR(existed/no)|
//stringbool2, | statecoldrst = "A" | stateb ="B"      |
//stringbool3  |                    |                  |
//
//stringbool4  |                    |
//,stringbool5 |                    |
//,stringbool6 |                    |
//,stringbool7,|                    |
//s,           |                    |
//mensaje1],   |                    |
//booleantype, |                    |
//matrnum[0][0]|                    |
//matrnum[0][1]|                    |
//matrnum[0][2]|                    |
//matrnum[1][0]|                    |
//matrnum[1][1]|                    |
//matrnum[1][2]|;                   |
//
 let str = "123";
 let num: i32 = str.parse().unwrap();
 log!(&env, "string to  number {}", num);

let colu1: u32=9999;
let fila1: u32=9999;
//   if  iveci32<11 {veci32[iveci32] = 100;iveci32=iveci32+1;}
//   if  iveci32<11 {veci32[iveci32] = 200;iveci32=iveci32+1;}
//   if  iveci32<11 {veci32[iveci32] = 300;}
  
let msg = "kmac2=deliverable2='beta' 2.0 Nov162023-1928-"; //version message
let sout = String::from_slice(&env, msg);
//let mut sout = String::from_slice(&env, &array[0]);
//sout=array[0];
 return (ln1, count, fila1, colu1,vec![&env,  sout, old_message, message, stringbool1, stringbool2,stringbool3,stringbool4,stringbool5,stringbool6,stringbool7,s,mensaje1],
     //veci32
     vec![&env, veci32[0], veci32[1], veci32[2],veci32[3], veci32[4], veci32[5],veci32[6], veci32[7], veci32[8],veci32[9],
     veci32[10], veci32[11], veci32[12],veci32[13], veci32[14], veci32[15],veci32[16], veci32[17], veci32[18],veci32[19]]
//    booleantype,
  //  matrixnum[0][0],matrixnum[0][1],matrixnum[0][2],matrixnum[1][0],matrixnum[1][1],matrixnum[1][2]

);   
    

fn set_keyb   (env:&Env , user:Address)->String{
//svb1adrms block BEGIN =="svb1adr"==9
let statec: String;
  statec = String::from_slice(&env, "C");
  let setkeybret = String::from_slice(&env, "set_keyb ok");
  log!(&env, "save the buyer ad (that i have no!");
  env.storage().persistent().set(&B1STAD, &user);
  //env.storage().persistent().set(&MCSTAT, &stateb);
  env.storage().persistent().set(&MCSTAT, &statec);
  return setkeybret;


//   n reset_key (env:&Env ,kstradrex:String, strfalse2:String, user:Address)->(String, String){
//     //set the KSTR key, return "KSTR already existed" or "KSTR no existed before"
//     //local
//     let stringbool3: String;//::from_slice(&env, "trueleyy");
//     let stateb: String;
//     stateb = String::from_slice(&env, "B");
//     log!(&env, "reset key kreator");
//     if is_initialized(&env) { // verifica si existia &KSTADR
//         stringbool3=kstradrex;//(savekadmessage)KSTADR existed!-panic?");
        
//     }else 
//      {
//         stringbool3  =strfalse2;
//     }
//         env.storage().persistent().set(&KSTADR, &user);
        
//         env.storage().persistent().set(&MCSTAT, &stateb);
     
//      return (stringbool3,stateb);
// }//reset_key


}

// set key buyer end
fn reset_key (env:&Env ,kstradrex:String, strfalse2:String, user:Address)->(String, String){
    //set the KSTR key, return "KSTR already existed" or "KSTR no existed before"
    //local
    let stringbool3: String;//::from_slice(&env, "trueleyy");
    let stateb: String;
    stateb = String::from_slice(&env, "B");
    log!(&env, "reset key kreator");
    if is_initialized(&env) { // verifica si existia &KSTADR
        stringbool3=kstradrex;//(savekadmessage)KSTADR existed!-panic?");
        
    }else 
     {
        stringbool3  =strfalse2;
    }
        env.storage().persistent().set(&KSTADR, &user);
        
        env.storage().persistent().set(&MCSTAT, &stateb);
     
     return (stringbool3,stateb);
}//reset_key
//
fn coldreset (env:&Env,scldrst:String)->(String, String)
{
    //local variables
    let stringbool7: String;
   log!(&env, "scldrs!!: ");
   stringbool7=scldrst;
   let statecoldrst = String::from_slice(&env, "A");
   env.storage().persistent().set(&MCSTAT, &statecoldrst);
   return (stringbool7, statecoldrst);
}
                                  // }
//
fn cmpaddr (env:&Env,newaddr:Address) -> (String,bool){
    //local variables
    let booleantype :bool;
    let mut mkstrnoex= String::from_slice(&env, "stored KSTADR didnt existed");
    let mut mkstradeq= String::from_slice(&env, "new KSTADR == stored KSTADR (??)");
    let mut mkstradneq= String::from_slice(&env, "new KSTADR <> stored KSTADR (??)");
    let mut meqadr= String::from_slice(&env, "..");
    let kstoredadd: Address;
    if is_initialized(&env) { 
         kstoredadd = env
            .storage()
            .persistent()
            .get(&KSTADR)
            .unwrap();
        let clonestored=  kstoredadd.clone();
        // comparo:
        if clonestored==newaddr
        {
            //old and new are the same
             booleantype=true;
             log!(&env, "stored KSTADR== new");
             meqadr=mkstradeq; 
        } else 
           {
               log!(&env, "stored KSTADR <> new");
               booleantype=false;
               meqadr=mkstradneq;
            }
        
    }else 
        {
            log!(&env, "stored KSTADR didnt existed.");
            booleantype=false;
            meqadr=mkstrnoex;
        }

     
    return (meqadr, booleantype);
}


//-> stringbool1, stringbool2, stringbool4,
fn savekadd (env: &Env, kstradre2:String, strtrue: String,strtrue3: String, 
    strfalse:String,strfalse3:String, user:Address, cloneuser:Address, moldnewsm:String,
    cloneuser2:Address)
  ->   (String, String, String, String, bool, bool)
 {
    //local variables
    let booleantype :bool;
    let booleantype2 :bool=true;
    let stringbool1: String;
    let mut stringbool4= String::from_slice(&env, "KSTADR no exists previously");

    let mystorage = env.storage();
    booleantype=mystorage.persistent().has(&KSTADR);
    log!(&env, "savekadd : KSTADR existence:", booleantype);
    if booleantype { //== true  {
        stringbool4=kstradre2; 
        stringbool1  =strtrue; 
        log!(&env, "savekadd :KSTADR exists!stringbool4{}", stringbool4);
        log!(&env, "savekadd :KSTADR exists!stringbool1{}", stringbool1);
    } else {stringbool1  =strfalse; //redundary? msg Means:..."KeyNoExists");
            log!(&env, "savekadd :KSTADR NO exists!stringbool1{}", stringbool1);
           }
    //com 1411 tapo lo sig
    // if is_initialized(&env) 
    //    {
    //     //panic!("contract has been already initialized");
    //         stringbool2  =strtrue3;
    //         log!(&env, "savekadd :is_init!stringbool2{}", stringbool2);
    //     }else {stringbool2  =strfalse3;
    //         log!(&env, "savekadd :is_init NOT!stringbool2{}", stringbool2);
    //     }
    //com 1411 tapo lo sig FIN

    let cloneuser0=  user.clone();
    log!(&env, "cloneuser0: {}", cloneuser0);
    // is the sender == Kreator?
    // user.require_auth(); <----this is not useful because buyer can send kreator messages
    //com 1411 tapo lo sig:
    // let kreatorstoredaddress: Address;
    // kreatorstoredaddress = env
    //               .storage()
    //               .persistent()
    //               .get(&KSTADR)
    //               .unwrap_or     (cloneuser);
    // let clonestored=  kreatorstoredaddress.clone();
    //com 1411 tapo lo sig:FIN
    
    //cloneuser2.require_auth_for_args(
    // log the new or old (?) kreator address...depend on the existence check of KSTADR...
    // log!(&env, "savekadd new or old (?) kreator address(from stored)", clonestored);
    // log!(&env, "savekadd booleantype when for KSTADR existence:", booleantype);
    // log!(&env, "savekadd new (parameter) kreator address", cloneuser0);
    // if clonestored==cloneuser0
    //  {
    //     booleantype2=true;
    //     log!(&env, "savekadd OLD AND NEW ARE THE SAME!");
    //     stringbool6=moldnewsm; //old and new are the same
    //  }else 
    //     {
    //         log!(&env, "savekadd KSTADR SETED!!!");
    //         stringbool6=msg6;
    //         booleantype2=false;
            
    //     }  
    
    // env.storage().persistent().set(&KSTADR, &cloneuser2);
let stringbool6= String::from_slice(&env, "******NOSIRVE");
let stringbool2= String::from_slice(&env, "******NOSIRVE");


    return ( stringbool1,stringbool2,stringbool4, stringbool6, booleantype, booleantype2);
 }         // savekadmessage  b l o c k END
  
fn is_initialized(env: &Env) -> bool {
    //env.storage().instance().has(&DataKey::Init)
    env.storage().persistent().has(&KSTADR)
    
    
}
//
fn find2(matrix: &[[String; 3]; 2], target: String) -> (usize, usize) {
    let mut fila1: usize = 0;
    let mut colu1: usize = 0;
    let mut row = 0;
    while row < matrix.len() {
        let mut col = 0;
        while col < matrix[0].len() {
              if matrix[row][col] == target  {
                fila1=row;
                colu1=col;
    }
            col += 1;
        }
        row += 1;
    }
    return (fila1,colu1);   
} 

//fn finditrav (matrix: [[i32; 6]; 6], index: i32) -> (usize){} 
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




