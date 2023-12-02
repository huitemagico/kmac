#![no_std]
// kmac2 rev December 02 2023 1057
// see https://github.com/huitemagico/kmac/wiki
mod mimodulo;
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
pub trait ExtFunc{
    fn extfunc1 (&self, env: Env) -> [[i32; 6]; 6];
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
  

let mut stringbool1= String::from_slice(&env, "sbool1");
//let mut stringbool1ad:&String;
//let mut stringbool2ad:&String;

let mut stringbool2= String::from_slice(&env, "trueley22");
let stringbool3= String::from_slice(&env, "trueley22");
let stringbool4= String::from_slice(&env, "trueley22");
let stringbool5= String::from_slice(&env, "trueley22");
let stringbool6= String::from_slice(&env, "trueley22");
let stringbool7= String::from_slice(&env, "trueley22");
//
let testmsjguardado1 = "???";
let mut smsg1= String::from_slice(&env, testmsjguardado1);
//

let mut smsg2= String::from_slice(&env, testmsjguardado1);

let mut smsg3= String::from_slice(&env, testmsjguardado1);
let mut smsg4= String::from_slice(&env, testmsjguardado1);
let mut smsg5= String::from_slice(&env, testmsjguardado1);
let mut smsg6= String::from_slice(&env, testmsjguardado1);
let mut smsg7= String::from_slice(&env, testmsjguardado1);


let mut  veci32:           [i32; 20]= [0;20];
let mut iveci32:usize = 0;
let mut inummsg:usize = 0;

let scldrst =String::from_slice(&env, "cldrst");
let rstkadm = "rstkadm";
let srstkadm = String::from_slice(&env, rstkadm);
let svb1adrms = String::from_slice(&env, "svb1adr");
let selcofblnd = String::from_slice(&env, "selcofblnd");
let resetmessage = String::from_slice(&env, "reset"); 
let clonemsg=  message.clone();
//
let mut nummsg=msjtonum (&clonemsg, &scldrst,&srstkadm,&svb1adrms,&selcofblnd);
if nummsg ==9999 {
    nummsg=mimodulo::msj_t_num_usr (&env,&clonemsg);
    //usr function "convert string message to number"
}
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




log!(&env, "logg standard default init values for the matrix number");
let mut matrixnum: [[i32; 6]; 6] = [[0; 6]; 6];
    let mut number = 1;
    for row in 0..6 { 
        for col in 0..6 {
            matrixnum[row][col]=number;
            number += 1;
        }
    }

let coordinates = mimodulo::put_coordinates();
log!(&env, "logging when the coordinates put the values in matrixnum ");

 for row in 0..5{
    for col in 0..5 {
        for col1 in 0..35 {
            let (coordx,coordy)= coordinates[col1];
            if coordx==row && coordy==col {
                matrixnum[row][col]=9999;

            }
        }
        

    }
}
//

    inummsg=inummsg+1;
    if inummsg <8{
        let matrixnuminit  = String::from_slice(&env, "MATRIXNUM INIT end.");
        savemsg (&env, inummsg, &mut smsg1, &mut smsg2,
            &mut smsg3,
            &mut smsg4,
            &mut smsg5,
            &mut smsg6,
            &mut smsg7,
            matrixnuminit);
        
    }


log!(&env, "CORE init functions definition for the matrix function");
let corefuncmat: [[i32; 6]; 6] ;
corefuncmat = mimodulo::put_corefunc();
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
    inummsg=inummsg+1;
    if inummsg <8{
        let corefuncmatend  = String::from_slice(&env, "COREFUNCMAT end.");
        savemsg (&env, inummsg, &mut smsg1, &mut smsg2,
            &mut smsg3,
            &mut smsg4,
            &mut smsg5,
            &mut smsg6,
            &mut smsg7,
            corefuncmatend);
        
    }
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
// (a)    save for work message received
        let ln1: u32;
        ln1 = message.len();  //length of message received
        
        let mut count: u32 = env.storage().persistent().get(&COUNTER).unwrap_or(0); 
        count += 1;
        let mut old_message = env
            .storage()
            .persistent()
            .get(&OLD_MSG)
            .unwrap_or(String::from_slice(&env, "NoOldMessage0"));

        if message == resetmessage {
            old_message = String::from_slice(&env, "ResetMessageStored");
        }
        env.storage().persistent().set(&OLD_MSG, &message);
        env.storage().persistent().set(&COUNTER, &count);
        count = plus_two (count);
//



//let brwsvkadmsg = &savekadmessage;





let strfalse  = String::from_slice(&env, "KeyNoExists");
let kstradrex=String::from_slice(&env, "(srstkadm)KSTADR existed!-panic?");
let cloneuser=  user.clone();

log!(&env, "cloneuser: {}", cloneuser);
let cloneuser2= cloneuser.clone();
log!(&env, "cloneuser2: {}", cloneuser2);


let crow:usize ;
let ccol:usize ;
(crow,ccol)= findcoor(matrixnum,nummsg);
//
let fnnumber:i32; 
fnnumber= corefuncmat [crow][ccol];
let  rwval= crow as i32;
let  clval= ccol as i32;
log!(&env, "corefuncmat returns fn number {} with row {} col {}",fnnumber, rwval, clval);

        inummsg=inummsg+1;
        if inummsg <8{
        let getfnnumend  = String::from_slice(&env, "GET FN NUM END");
        savemsg (&env, inummsg, &mut smsg1, &mut smsg2,
            &mut smsg3,
            &mut smsg4,
            &mut smsg5,
            &mut smsg6,
            &mut smsg7,
            getfnnumend);
        
    }
let mut crstatec = String::from_slice(&env, "X");
let nostatecond = String::from_slice(&env, "X");
if is_init_stat(&env) { 
        crstatec =get_curr_st(&env);
        //stringbool1=set_keyb   (&env , clonebuyer);
   }
let  initstatefmsg : String;//::from_slice(&env, "X");
let  endstatefmsg ;//= String::from_slice(&env, "X");
initstatefmsg=get_str_st( &env ,crow) ;
endstatefmsg=get_str_st( &env ,ccol) ;
//
if crstatec ==nostatecond {
    inummsg=inummsg+1;
    if inummsg <8{
        let nostatems  = String::from_slice(&env, "is_initStat:no existed MCSTAT");
        savemsg (&env, inummsg, &mut smsg1, &mut smsg2,
            &mut smsg3,
            &mut smsg4,
            &mut smsg5,
            &mut smsg6,
            &mut smsg7,
            nostatems);
        
    }
    
}
let str_trx_bad  = String::from_slice(&env, "Trx DECLINED");
let  compbool:bool;
if fnnumber==1 {compbool = true;}
else  {compbool= cmp_curr_prop (&crstatec, &initstatefmsg);}


let fnname:String;
fnname=n_to_str (&env,fnnumber);
 inummsg=inummsg+1;
 if inummsg <8
  {
     savemsg (&env, inummsg, &mut smsg1, &mut smsg2,&mut smsg3,
     &mut smsg4,&mut smsg5,&mut smsg6,&mut smsg7,
     fnname  );}
match fnnumber {
    1=> {
        log!(&env, "match nummsg: (1) cldrst call coldreset.");
        
        let result=coldreset (&env);
        match result {
            Ok(code) => {log!(&env, "ok {} Ok", code)},
            Err(code) => log!(&env, "Error {}", code),
        }
    }
    2 =>{  
        log!(&env, "match nummsg: (2) rstkadm call reset.");
       
        if  compbool{
              (stringbool1, stringbool2)=reset_key (&env, kstradrex,strfalse,cloneuser );
              log!(&env, "match nummsg: (2) rstkadm call reset.", stringbool1, stringbool2);
              inummsg=inummsg+1;
              if inummsg <8{
                 let str_rstkadm_st  = String::from_slice(&env, "Trx rstkadm ok");
                 savemsg (&env, inummsg, &mut smsg1, &mut smsg2,
                 &mut smsg3,
                 &mut smsg4,
                 &mut smsg5,
                 &mut smsg6,
                 &mut smsg7,
                 str_rstkadm_st);
        
                }
                
            }else {
                inummsg=inummsg+1;
                if inummsg <8{
                   savemsg (&env, inummsg, &mut smsg1, &mut smsg2,
                   &mut smsg3,
                   &mut smsg4,
                   &mut smsg5,
                   &mut smsg6,
                   &mut smsg7,
                   str_trx_bad);
          
                   }
                }
        }
    3=> {    
            if  compbool{
                  log!(&env, "svb1adrms");
                  let clonebuyer=  buyer.clone();
                  stringbool1=set_keyb   (&env , clonebuyer);
                  inummsg=inummsg+1;
                  if inummsg <8{
                     let str_rstkadm_st  = String::from_slice(&env, "Trx svb1adrms ok");
                     savemsg (&env, inummsg, &mut smsg1, &mut smsg2,
                     &mut smsg3,
                     &mut smsg4,
                     &mut smsg5,
                     &mut smsg6,
                     &mut smsg7,
                     str_rstkadm_st);
                     }
            }else {
                inummsg=inummsg+1;
                if inummsg <8{
                   savemsg (&env, inummsg, &mut smsg1, &mut smsg2,
                   &mut smsg3,
                   &mut smsg4,
                   &mut smsg5,
                   &mut smsg6,
                   &mut smsg7,
                   str_trx_bad);
                }
            }
   
        }
    4=> {     log!(&env, "maintenance");
              stringbool1=dummyfunc   (&env , &user);
        }
    5=> {     log!(&env, "offline");
              stringbool1=dummyfunc   (&env , &user);
        }
    6=> {     log!(&env, "reactivation");
              stringbool1=dummyfunc   (&env , &user);
        }
    7=> {     
              inummsg=inummsg+1;
              if inummsg <8
               {
                     let str_f7  = String::from_slice(&env, "Function 7");
                     savemsg (&env, inummsg, &mut smsg1, &mut smsg2,
                     &mut smsg3,
                     &mut smsg4,
                     &mut smsg5,
                     &mut smsg6,
                     &mut smsg7,
                     str_f7);
        
                }
                if  compbool{
                    let kstatee = String::from_slice(&env, "E");
                    let result=set_state (&env,&kstatee);
                    match result {
                        Ok(code) => log!(&env, "ok {} Ok", code),
                        Err(code) => log!(&env, "Error {}", code),
                    }
                      inummsg=inummsg+1;
                      if inummsg <8{
                         let str_rstkadm_st  = String::from_slice(&env, "Trx svb1adrms ok");
                         savemsg (&env, inummsg, &mut smsg1, &mut smsg2,
                         &mut smsg3,
                         &mut smsg4,
                         &mut smsg5,
                         &mut smsg6,
                         &mut smsg7,
                         str_rstkadm_st);
                
                        }
                }else
                    {
                        inummsg=inummsg+1;
                        if inummsg <8{
                              savemsg (&env, inummsg, &mut smsg1, &mut smsg2,
                              &mut smsg3,
                              &mut smsg4,
                              &mut smsg5,
                              &mut smsg6,
                              &mut smsg7,
                              str_trx_bad);
                  
                        }
                    }
           
        }
       
        8=> {
            if  compbool{
              mimodulo::function8(&env);
              let result=trx_proc (&env, &endstatefmsg)  ;
              match result {
                  Ok(code) => log!(&env, "ok {} Ok", code),
                  Err(code) => log!(&env, "Error {}", code),
              }
              inummsg=inummsg+1;
              if inummsg <8{
                   let fn8ok  = String::from_slice(&env, "Function 8 ok");
                   savemsg (&env, inummsg, &mut smsg1, &mut smsg2,&mut smsg3,
                   &mut smsg4,&mut smsg5,&mut smsg6,&mut smsg7, fn8ok);
                    }
          }else
              {   
                  inummsg=inummsg+1;
                  if inummsg <8{savemsg (&env, inummsg, &mut smsg1, &mut smsg2,&mut smsg3,
                        &mut smsg4,&mut smsg5,&mut smsg6,&mut smsg7,str_trx_bad);}
               }
     
  }
  9=> {
   
      if  compbool{
          mimodulo::function9(&env);
          let result=trx_proc (&env, &endstatefmsg)  ;
          match result {
              Ok(code) => log!(&env, "ok {} Ok", code),
              Err(code) => log!(&env, "Error {}", code),
          }
          inummsg=inummsg+1;
          if inummsg <8{
               let fnok  = String::from_slice(&env, "Function 9 ok");
               savemsg (&env, inummsg, &mut smsg1, &mut smsg2,&mut smsg3,
               &mut smsg4,&mut smsg5,&mut smsg6,&mut smsg7, fnok);
                }
      }else
          {   
              inummsg=inummsg+1;
              if inummsg <8{savemsg (&env, inummsg, &mut smsg1, &mut smsg2,&mut smsg3,
                    &mut smsg4,&mut smsg5,&mut smsg6,&mut smsg7,str_trx_bad);}
           }
 
}
10=> {
   
    if  compbool{
        mimodulo::function10(&env);
        let result=trx_proc (&env, &endstatefmsg)  ;
        match result {
            Ok(code) => log!(&env, "ok {} Ok", code),
            Err(code) => log!(&env, "Error {}", code),
        }
        inummsg=inummsg+1;
        if inummsg <8{
             let fnok  = String::from_slice(&env, "Function 10 ok");
             savemsg (&env, inummsg, &mut smsg1, &mut smsg2,&mut smsg3,
             &mut smsg4,&mut smsg5,&mut smsg6,&mut smsg7, fnok);
              }
    }else
        {   
            inummsg=inummsg+1;
            if inummsg <8{savemsg (&env, inummsg, &mut smsg1, &mut smsg2,&mut smsg3,
                  &mut smsg4,&mut smsg5,&mut smsg6,&mut smsg7,str_trx_bad);}
         }

}
    _=> {
        log!(&env, "no se ???");}
    
} //match


//============================================================



 

let mensaje1:String;
mensaje1=clonemsg;
 let str = "123";
 let num: i32 = str.parse().unwrap();
 log!(&env, "string to  number {}", num);

let colu1: u32=9999;
let fila1: u32=9999;
     let messages = ["message1", "message2", "message3"];
     if let Some(message) = get_message(&messages, 1) {
         log!(&env, "Message: {}", message);
     } else {
         log!(&env, "Invalid index");
     }
let mut colors = ["red", "green", "blue"];
   colors[0]="green";
   let s = String::from_slice(&env, colors[0]);

let msg = "kmac2=deliverable2='beta' 2.0 December 02-2023-1155"; //version message
let sout = String::from_slice(&env, msg);
 return (ln1, count, fila1, colu1,vec![&env, 
    smsg1, smsg2,
    smsg3, smsg4,
    smsg5, smsg6,
    smsg7, 
    sout, old_message, message, stringbool1, stringbool2,stringbool3,stringbool4,
    stringbool5,stringbool6,stringbool7,s,mensaje1],
     //veci32
     vec![&env, veci32[0], veci32[1], veci32[2],veci32[3], veci32[4], veci32[5],veci32[6], veci32[7], veci32[8],veci32[9],
     veci32[10], veci32[11], veci32[12],veci32[13], veci32[14], veci32[15],veci32[16], veci32[17], veci32[18],veci32[19]]

);  
      

fn trx_proc( env:&Env , endstatefmsg:&String)->  Result<u32, ()> {
      let result=set_state (&env,&endstatefmsg);
      match result {
          Ok(code) => log!(&env, "ok {} Ok", code),
          Err(code) => log!(&env, "Error {}", code),
      }
      return Ok(0);     
 
}



fn cmp_curr_prop (r_crstatec:&String, r_initstatefmsg:&String)->bool{
    
    return *r_crstatec == *r_initstatefmsg;
}



fn get_str_st( env:&Env ,index: usize) -> String {
    let sta: String;
       sta = String::from_slice(&env, "A");
       
       let stb = String::from_slice(&env, "B");
       let stc = String::from_slice(&env, "C");
       let std = String::from_slice(&env, "D");
       let ste = String::from_slice(&env, "E");
       let stf = String::from_slice(&env, "F");
       let stz:String;
       let str = String::from_slice(&env, "R");
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

fn get_message<'a>(messages: &'a [&str], index: usize) -> Option<&'a str> {
    if index < messages.len() {
        Some(messages[index])
    } else {
        None
    }
}


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
fn n_to_str(env:&Env,n: i32) -> String {
    let indx:usize =n as usize;
    let stb: String;
    const STR_ARRAY: [&str; 16] = ["Fn 0", "Fn 1", "Fn 2", "Fn 3", "Fn 4", "Fn 5", "Fn 6", "Fn 7", "Fn 8", "Fn 9", "Fn 10", "Fn 11","Fn 12","Fn 13","Fn 14","Fn 15"];  
    if  n<=15 
    { stb = String::from_slice(&env, STR_ARRAY[indx as usize]);
    }
    else { stb = String::from_slice(&env, "0");}
    return stb;


} 
fn msjtonum(msg:&String, scldrst:&String,srstkadm:&String,svb1adrms:&String, selcofblnd:&String) -> i32  {
    let mut nummsg: i32 = 9999;

    if msg == scldrst      {nummsg=1;} 
    if msg==srstkadm       {nummsg=2;} 
    if msg==svb1adrms      {nummsg=9;}
    if msg==selcofblnd      {nummsg=17;}
    return nummsg;

}
fn dummyfunc   (env:&Env , user:&Address)->String{
       let retdummy: String;
       retdummy = String::from_slice(&env, "dummyok");
       log!(&env, "user en dummy {}", user);
       return retdummy;
       
    
    
    }
    
fn set_keyb   (env:&Env , user:Address)->String{
let statec: String;
  statec = String::from_slice(&env, "C");
  let setkeybret = String::from_slice(&env, "set_keyb ok");
  log!(&env, "save the buyer ad (that i have no!");
  env.storage().persistent().set(&B1STAD, &user);
  //env.storage().persistent().set(&MCSTAT, &stateb);
  env.storage().persistent().set(&MCSTAT, &statec);
  return setkeybret;
}
//
fn ch_state_mach (env:&Env )->  Result<u32, ()>{
    let stateb: String;
    stateb = String::from_slice(&env, "B");
    
        env.storage().persistent().set(&MCSTAT, &stateb);
     
   return Ok(0);
}
fn reset_key (env:&Env ,kstradrex:String, strfalse2:String, user:Address)->(String, String){
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
}
fn set_state (env:&Env, newstate:&String)->  Result<u32, ()>
{ 
   env.storage().persistent().set(&MCSTAT, newstate);
   return Ok(0);
}
//
fn coldreset (env:&Env)->  Result<u32, ()>
{
    
   let statecoldrst = String::from_slice(&env, "A");
   env.storage().persistent().set(&MCSTAT, &statecoldrst);
   return Ok(0);
}
    fn get_curr_st (env:&Env) -> String {
   let mut crstatec = String::from_slice(&env, "X");
   if is_init_stat(&env) { 
    crstatec = env
            .storage()
            .persistent()
            .get(&MCSTAT)
            .unwrap();
    }
    return crstatec
}
fn cmpaddr (env:&Env,newaddr:Address) -> (String,bool){
    let booleantype :bool;
    let  mkstrnoex= String::from_slice(&env, "stored KSTADR didnt existed");
    let  mkstradeq= String::from_slice(&env, "new KSTADR == stored KSTADR (??)");
    let  mkstradneq= String::from_slice(&env, "new KSTADR <> stored KSTADR (??)");
    let  meqadr:String;
    let kstoredadd: Address;
    if is_initialized(&env) { 
         kstoredadd = env
            .storage()
            .persistent()
            .get(&KSTADR)
            .unwrap();
        let clonestored=  kstoredadd.clone();
        if clonestored==newaddr
        {
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


  
fn is_initialized(env: &Env) -> bool {
    //env.storage().instance().has(&DataKey::Init)
    env.storage().persistent().has(&KSTADR)
    
    
}
fn is_init_stat(env: &Env) -> bool {
    env.storage().persistent().has(&MCSTAT)
    
    
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
