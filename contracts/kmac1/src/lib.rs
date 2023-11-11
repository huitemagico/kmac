#![no_std]
// kmac1b rev november 08 2023 18:08
// kmac1b version:
// 1. basic role management for BUYERS users.
// see https://github.com/huitemagico/kmac/wiki#allow-list-implementation
mod mimodulo;
use soroban_sdk::{contract, contractimpl,contracttype, Address, symbol_short, vec, Env, String, Symbol, Vec,log,IntoVal,BytesN};
#[contracttype]
pub enum DataKey {
    Kstad(Address), //Counter--> Kreator Stored Address
}
pub enum DataKey2 {
    UsrCnt,
    UsrAdr(BytesN<32>),
    UsrNme,
}

// impl StringArray {
//     // Method to add a string to the array
//     fn add_string(&mut self, string: String) {
//         self.strings.push(string);
//     }

//     // Method to retrieve the entire array
//     fn get_array(&self) -> &Vec<String> {
//         &self.strings
//     }
// }
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
// (3) #contract  pub struct ---------
// #[contract]
// pub struct IncrementContract;

#[contractimpl]
impl KmacContract {
       pub fn kmac    (env: Env,  user: Address, buyer: Address, value: u32, message: String, sender:String ) -> 
       (  u32,u32, u32, u32, Vec<String>, bool,i32,i32,i32,i32,i32,i32) {
 // 

//
    let current_contract_add = env.current_contract_address();
    log!(&env, "current_contract_add: {}", current_contract_add);
//
// //let key = DataKey::Counter(user.clone()); //from auth lib
// //let add_from_par = user;

// let cloneuser0=  user.clone();
// log!(&env, "cloneuser0: {}", cloneuser0);
// let cloneuser=  user.clone();
// log!(&env, "cloneuser: {}", cloneuser);
// //
// let cloneuser2= cloneuser.clone();
// log!(&env, "cloneuser2: {}", cloneuser2);

//
//
// if trx == resetmessage {
//     old_message = String::from_slice(&env, "ResetMessageStored");
// }


    let stringtrx = String::from_slice(&env, "13");  
//
 //      
    log!(&env, "logg standard default init values for the matrix");
    let string11 = String::from_slice(&env, "11");
    let string21 = String::from_slice(&env, "21");
    let string12 = String::from_slice(&env, "12");
    let string22 = String::from_slice(&env, "22");
    let string13 = String::from_slice(&env, "13");
    let string23 = String::from_slice(&env, "23");
    let string00 = String::from_slice(&env, "00");
//
//

//let  mut  msjtos: [[String; 10]] ;
  let  mut  matrix: [[String; 3]; 2] = [[
     string11,string12,string13],[string21,string22,string23 ]];
//
log!(&env, "logg standard default init values for the matrix number");
//let mut matrixnum: [[i32; 3]; 2] = [[0; 3]; 2];
let mut matrixnum: [[i32; 6]; 6] = [[0; 6]; 6];
    let mut number = 1;
    // let num_rows = matrixnum.len();
    // let num_cols = matrixnum[0].len();
    for row in 0..5 { //6 cols 6 filas
        for col in 0..5 {
            matrixnum[row][col]=number;
            number += 1;
        }
    }
let coordinates = mimodulo::put_coordinates();
log!(&env, "logging when the coordinates put the values in matrixnum ");
//
for row in 0..2{
    for col in 0..3 {
        for col1 in 0..6 {
            let mut value = row as i32;
            let mut value = col as i32;
            let mut value = col1 as i32;
            let (coordx,coordy)= coordinates[col1];
            if coordx==row && coordy==col {
                log!(&env, "logg ");
                matrixnum[row][col]=9999;

            }
        }

    }
}
let mut fila2: usize = 0;
let mut colu2: usize = 0;
matrix[fila2][colu2] = string00;
//
//
// the function "find2" has to find the parameter "transaction" () in   the matrix
let (fila, colu)= find2(&matrix, stringtrx) ;
//
log!(&env, "CORE init functions definition for the matrix function");
//let mut corefuncmat: [[i32; 6]; 6] = [[0; 6]; 6];
//let mut corefuncmat: [[i32; 6]; 6] = [[0; 6]; 6];
let mut corefuncmat: [[i32; 6]; 6] ;
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

        }
        
        
    }
}

   let hello = "Hello"; // &str implements Clone
   assert_eq!("Hello", hello.clone());
//
        let s1: &str = "World";
        //let mut stringbool1= String::from_slice(&env, "trueley22");
        //let mut stringbool1;

         let mystorage = env.storage();
         let mut key = symbol_short!("key99");
         mystorage.persistent().set(&key, &1);
         assert_eq!(mystorage.persistent().has(&key), true);
         assert_eq!(mystorage.persistent().get::<_, i32>(&key), Some(1));
         let mut booleantype :bool=true;
         booleantype=mystorage.persistent().has(&key);
         log!(&env, "booleantype when there is the key: %s", booleantype);
        //  //let mut key = symbol_short!("KSTADR");
        //  booleantype=mystorage.persistent().has(&KSTADR);
        //  log!(&env, "booleantype when for KSTADR existence:", booleantype);
        //  //
        //  let mut stringbool1= String::from_slice(&env, "trueleyy");
    // if booleantype { //== true  {
    //     let stringbool1  = String::from_slice(&env, "VERDADERITO");
    // }
//
  
   // booleantype = mystorage.persistent().has(&key);
    // let mut stringbool1= String::from_slice(&env, "trueleyy");
    // if booleantype { //== true  {
    //     let stringbool1  = String::from_slice(&env, "VERDADERITO");
    // }
//
//
        let _old_message = "nomessage";
        
// the kreator messages are hardcoded in the core of kmac
// only the kreator can send the following messages:
        let svb1adrms = String::from_slice(&env, "svb1adr");//save buyer 1 address
        let savekadmessage = String::from_slice(&env, "savekad");
        let savekadmessage1 = String::from_slice(&env, "savekad");
        let resetmessage = String::from_slice(&env, "reset"); 
        let resetkad = String::from_slice(&env, "resetkad"); 
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
        let fila1: u32;
        let colu1: u32;
        fila1=fila as u32;
        colu1=colu as u32;
        let somevalue1:i32=matrixnum[0][0];//100;
        let somevalue2:i32=matrixnum[0][1];//num_rows;//matrixnum[0][2];
        let somevalue3:i32=matrixnum[0][2];//num_cols;//matrixnum[0][3];
        let somevalue4:i32=matrixnum[1][0];
        let somevalue5:i32=matrixnum[1][1];
        let somevalue6:i32=matrixnum[1][2];
//        o conformarse con trx de buyer.

        let msg = "kmac1b=deliverable1='prod' 2.0 Nov102023-"; //version message
        let sout = String::from_slice(&env, msg);
        let rstkadm = "rstkadm";
        let srstkadm = String::from_slice(&env, rstkadm);
        let srstkadm1 = String::from_slice(&env, rstkadm);
        let scldrst =String::from_slice(&env, "cldrst");
        let scldrst2 =String::from_slice(&env, "cldrst");
//let brwsvkadmsg = &savekadmessage;
let clonemsg=  message.clone();
log!(&env, "clonemsg: {}", clonemsg);
let mut stringbool1= String::from_slice(&env, "trueley22");
let mut stringbool2= String::from_slice(&env, "trueley22");
let mut stringbool3= String::from_slice(&env, "trueley22");
let mut stringbool4= String::from_slice(&env, "trueley22");
let mut stringbool5= String::from_slice(&env, "trueley22");
let mut stringbool6= String::from_slice(&env, "trueley22");
let mut stringbool7= String::from_slice(&env, "trueley22");
//let csavekadm=savekadmessage.clone();
//let brwmsg = &message;
let strtrue  = String::from_slice(&env, "KeyExists");
let strtrue4  = String::from_slice(&env, "KeyExists");
let strfalse  = String::from_slice(&env, "KeyNoExists");
let strtrue2  = String::from_slice(&env, "(rstkadm):KSTADR existed! (or prog BUG??");
let strfalse2  = String::from_slice(&env, "trx accepted,KeyNoExists");
let strtrue3  = String::from_slice(&env, "bad trx!(err panic) KeyExists");
let strtrue33  = String::from_slice(&env, "bad trx!(err panic) KeyExists");
let strfalse3  = String::from_slice(&env, "trx accepted,KeyNoExists");
let strfalse33  = String::from_slice(&env, "trx accepted,KeyNoExists");
let strfalse4  = String::from_slice(&env, "KeyNoExists");
let kstradrex=String::from_slice(&env, "(srstkadm)KSTADR existed!-panic?");
let kstradre2=String::from_slice(&env, "(savekadmessage)KSTADR existed!-panic?");
let kstradre22=String::from_slice(&env, "(savekadmessage)KSTADR existed!-panic?");

let msgnotrec=String::from_slice(&env, "(message?)trx not recognized");
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


// match clonemsg{
//     KRST =>{
//         log!(&env, "rstkadm");
//         stringbool2=srstkadm1;

//     }
//     savekadmessage1 =>  {
//         log!(&env, "savekadmessage");
//         stringbool3=srstkadm1;
//     }
//     svb1adrms =>{
//         log!(&env, "svb1adrms");
//     }
//     scldrst =>{
//         log!(&env, "scldrst");
//     }
//     _ => {
//         log!(&env, "no se ???");
//     }
// }
let mut nummsg=0;
if clonemsg==srstkadm       {nummsg=99;} //reset not used yet
if clonemsg==savekadmessage {nummsg=2;}//set k address on the storage               ==2==(0,2)==a->b
if clonemsg==svb1adrms      {nummsg=9;}//set by address=>"activation of the machine"==9==(1,2)==(B-C)
if clonemsg == scldrst      {nummsg=1;} //cold reset segun matrix                   ==1==(0,0)==(A->A)(D->A)
let mut crow:usize ;
let mut ccol:usize ;
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
    2 => { log!(&env, "savekadmessage");
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
    9=> {  log!(&env, "svb1adrms");}
    
    _=> {log!(&env, "no se ???");}
        }


//
if clonemsg==srstkadm 
  {
    log!(&env, "reset key kreator");
    if is_initialized(&env) { // verifica si existia &KSTADR
        stringbool3=kstradrex;//(savekadmessage)KSTADR existed!-panic?");
        stringbool2  =strtrue2;
    }else 
     {
        stringbool2  =strfalse2;
        env.storage().persistent().set(&KSTADR, &cloneuser8);
     }
        
  } else {
           if clonemsg==savekadmessage //savekad 
           {
               booleantype=mystorage.persistent().has(&KSTADR);
               log!(&env, "booleantype when for KSTADR existence:", booleantype);
               //let mut stringbool1= String::from_slice(&env, "trueleyy");
               if booleantype { //== true  {
                   stringbool4=kstradre22;
                   stringbool1  =strtrue4;
               } else {stringbool1  =strfalse4;}
               if is_initialized(&env) 
                   {
                      //panic!("contract has been already initialized");
                      stringbool2  =strtrue33;
                   }else {stringbool2  =strfalse33;}
                let cloneuser0=  cloneuser3;
                log!(&env, "cloneuser0: {}", cloneuser0);
                // is the sender == Kreator?
                // user.require_auth(); <----this is not useful because buyer can send kreator messages
                let kreatorstoredaddress: Address;
                kreatorstoredaddress = env
                  .storage()
                  .persistent()
                  .get(&KSTADR)
                  .unwrap_or     (cloneuser4);
                let clonestored=  kreatorstoredaddress.clone();
                cloneuser5.require_auth(); //pero no funciona...el buyer manda la trx y pasa soplado... a menos que estemos en un bloque de K!
                //cloneuser2.require_auth_for_args(
                // log the new or old (?) kreator address...depend on the existence check of KSTADR...
                log!(&env, "new or old (?) kreator address(from stored)", clonestored);
                log!(&env, "booleantype when for KSTADR existence:", booleantype);
                log!(&env, "new (parameter) kreator address", cloneuser0);
                if clonestored==cloneuser0
                    {
                       booleantype=true;
                       log!(&env, "OLD AND NEW ARE THE SAME!");
                    }else 
                         {
                            log!(&env, "KSTADR RESETED!!!");
                         }  //Get the kreator address-END-verify if its SET or RESET :END
                stringbool6=moldnewsm2;
                env.storage().persistent().set(&KSTADR, &cloneuser7);
            }         // savekadmessage  b l o c k END
            else 
                    {
                         if clonemsg==svb1adrms 
                         {  //svb1adrms block BEGIN
                             log!(&env, "save the buyer ad (that i have no!");
                             env.storage().persistent().set(&B1STAD, &cloneuser6);
                         } 
                        else {
                                if clonemsg == scldrst2
                                   { 
                                      log!(&env, "scldrs!!: ");
                                      stringbool7=scldrst2;
                                      let statecoldrst = String::from_slice(&env, "A");
                                      env.storage().persistent().set(&MCSTAT, &statecoldrst);
                                   }else 
                                       {    
                                            log!(&env, "message received not recognized!!: ") ;
                                            stringbool5=msgnotrec;
                                            //let msgnotrec=String::from_slice(&env, "(message?)trx not recognized");
                                       }
                            }
                    }
            }        
  //      }
     
//}

env.storage().persistent().set(&OLD_MSG, &message);
env.storage().persistent().set(&COUNTER, &count);
//============================================================
//
//let borrowed_slice: &str = &my_string;
//let cloneuser2= cloneuser.clone();
// let strtrue  = String::from_slice(&env, "KeyExists");
//let msjito=String::from_slice(&env, &arrmsj[0]);

// let vecmsj = [ "0", "1", "2", "3"];
// let msj11 = String::from_slice(&env, "11");
// // let mut arrmsj:[String;3]=[string11,string12,string13];
// //arrmsj[0]=msj11;
// let env = Env::default();
// let msg = "a message";
 let mut colors = ["red", "green", "blue"];
 colors[0]="azulblu";
 let s = String::from_slice(&env, colors[0]);
 let s1=s.clone();
 let s2=s.clone();
 let s3=s.clone();
 let idxarr=0;
 
 //let mensaje2=String::from_slice(&env,"mensaje2");
 let mensaje3=String::from_slice(&env,"mensaje3");
 let mensaje4=String::from_slice(&env,"mensaje4");

let mensaje1:String;
mensaje1=clonemsg;
        // return (ln1, count, fila1, colu1,vec![&env,  sout, old_message, message, stringbool1, stringbool2,s],booleantype,
        // matrixnum[0][0],matrixnum[0][1],matrixnum[0][2],matrixnum[1][0],matrixnum[1][1],matrixnum[1][2]);
//
return (ln1, count, fila1, colu1,vec![&env,  sout, old_message, message, stringbool1, stringbool2,stringbool3,stringbool4,stringbool5,stringbool6,stringbool7,s,mensaje1],booleantype,
    matrixnum[0][0],matrixnum[0][1],matrixnum[0][2],matrixnum[1][0],matrixnum[1][1],matrixnum[1][2]);


//


 



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
//-> stringbool1, stringbool2, stringbool4,
fn savekadd (env: &Env, kstradre2:String, strtrue: String,strtrue3: String, 
    strfalse:String,strfalse3:String, user:Address, cloneuser:Address, moldnewsm:String,
    cloneuser2:Address)
  ->   (String, String, String, String, bool, bool)
 {
    //local variables
    let mut booleantype :bool;
    let mut booleantype2 :bool=true;
    let stringbool1: String;//::from_slice(&env, "trueleyy");
    let stringbool2: String;//::from_slice(&env, "trueleyy");
    //let mensaje4=String::from_slice(&env,"mensaje4");
    let mut stringbool4= String::from_slice(&env, "KSTADR no exists previously");
    let mut msg6= String::from_slice(&env, "KSTADR no exstd prev. (re)setted");
    let stringbool6: String;//::from_slice(&env, "trueleyy");

    let mystorage = env.storage();
    booleantype=mystorage.persistent().has(&KSTADR);
    log!(&env, "savekadd : KSTADR existence:", booleantype);
    if booleantype { //== true  {
        stringbool4=kstradre2; //redundary?msg....(savekadmessage)KSTADR existed!-panic?");
        stringbool1  =strtrue; //redundary? msg.Meaning:"KSTADR already exists"... "KeyExists");
        log!(&env, "savekadd :KSTADR exists!stringbool4{}", stringbool4);
        log!(&env, "savekadd :KSTADR exists!stringbool1{}", stringbool1);
    } else {stringbool1  =strfalse; //redundary? msg Means:..."KeyNoExists");
            log!(&env, "savekadd :KSTADR NO exists!stringbool1{}", stringbool1);
           }
    if is_initialized(&env) 
       {
        //panic!("contract has been already initialized");
            stringbool2  =strtrue3;
            log!(&env, "savekadd :is_init!stringbool2{}", stringbool2);
        }else {stringbool2  =strfalse3;
            log!(&env, "savekadd :is_init NOT!stringbool2{}", stringbool2);
        }

    let cloneuser0=  user.clone();
    log!(&env, "cloneuser0: {}", cloneuser0);
    // is the sender == Kreator?
    // user.require_auth(); <----this is not useful because buyer can send kreator messages
    let kreatorstoredaddress: Address;
    kreatorstoredaddress = env
                  .storage()
                  .persistent()
                  .get(&KSTADR)
                  .unwrap_or     (cloneuser);
    let clonestored=  kreatorstoredaddress.clone();
    
    //cloneuser2.require_auth_for_args(
    // log the new or old (?) kreator address...depend on the existence check of KSTADR...
    log!(&env, "savekadd new or old (?) kreator address(from stored)", clonestored);
    log!(&env, "savekadd booleantype when for KSTADR existence:", booleantype);
    log!(&env, "savekadd new (parameter) kreator address", cloneuser0);
    if clonestored==cloneuser0
     {
        booleantype2=true;
        log!(&env, "savekadd OLD AND NEW ARE THE SAME!");
        stringbool6=moldnewsm; //old and new are the same
     }else 
        {
            log!(&env, "savekadd KSTADR SETED!!!");
            stringbool6=msg6;
            booleantype2=false;
            
        }  
    
    env.storage().persistent().set(&KSTADR, &cloneuser2);

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
    let mut nullrow: usize=0; let mut nullcol: usize=0;
    for (row_index, row) in matrix.iter().enumerate() {
        if let Some(col_index) = row.iter().position(|&x| x == index) {
            nullrow=row_index;
            nullcol=col_index;
            return (row_index, col_index);
        }
    }
 //   None
 return (nullrow, nullcol);
}

//
//let mut matrixnum: [[i32; 6]; 6] = [[0; 6]; 6];
// fn findcoor(matrixnum: [[i32; 6]; 6] = [[0; 6]; 6], target: i32) -> (usize, usize) {
//     let mut fila1: usize = 0;
//     let mut colu1: usize = 0;
//     let mut row = 0;
//     while row < matrix.len() {
//         let mut col = 0;
//         while col < matrix[0].len() {
//               if matrix[row][col] == target  {
//                 fila1=row;
//                 colu1=col;
//     }
//             col += 1;
//         }
//         row += 1;
//     }
//     return (fila1,colu1);   
// } 

        // fn find2(matrix: &[[String; 3]; 2], target: String) -> (usize, usize) {
        //     let mut fila1: usize = 0;
        //     let mut colu1: usize = 0;
        //     let mut row = 0;
        //     while row < matrix.len() {
        //         let mut col = 0;
        //         while col < matrix[0].len() {
        //               if matrix[row][col] == target  {
        //                 fila1=row;
        //                 colu1=col;
        //     }
        //             col += 1;
        //         }
        //         row += 1;
        //     }
        //     return (fila1,colu1);   
        // } 


         //let mut key = symbol_short!(keystr);
         //assert_eq!(mystorage.persistent().has(&keystr), true);
         //const KSTADR: Symbol = symbol_short!("KSTADR");
          
        
    }  
    }

