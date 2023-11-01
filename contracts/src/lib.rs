#![no_std]
// rev. 28102023 20:41
mod mimodulo;
//use core::simd::i16x32;

use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, String, Symbol, Vec,log};
//use soroban_sdk::{log, Env};


use mimodulo::plus_two;
//
pub trait Suma2{
    fn plus_two(&self, env: Env,x: u32) -> u32;
  }
//



pub trait PutMat{
    fn put_coordinates (&self, env: Env) -> [(usize, usize); 9];
  }
    
  
//
//
const COUNTER: Symbol = symbol_short!("COUNTER");
const OLD_MSG: Symbol = symbol_short!("OLD_MSG");

mod test;

#[contract]
pub struct Echo2Contract;

#[contractimpl]
impl Echo2Contract {
    pub fn echo2(env: Env, message: String, trx:String ) -> (  u32,u32, u32, u32, Vec<String>, bool,i32,i32,i32,i32,i32,i32) {
 // 

log!(&env, "a logggggggggggggggggggggggggggggggg entry");
    let current_contract_add = env.current_contract_address();
    log!(&env, "current_contract_add: {}", current_contract_add);
//
//

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

  let        mut           matrix: [[String; 3]; 2] = [[
     string11,string12,string13],[string21,string22,string23 ]];
//
log!(&env, "logg standard default init values for the matrix number");
let mut matrixnum: [[i32; 3]; 2] = [[0; 3]; 2];
    let mut number = 1;
    // let num_rows = matrixnum.len();
    // let num_cols = matrixnum[0].len();

    for row in 0..2 {
        for col in 0..3 {
            matrixnum[row][col]=number;
            number += 1;
        }
    }

//
let coordinates = mimodulo::put_coordinates();
log!(&env, "logging when the coordinates put the values in matrixnum ");
for row in 0..2{
    for col in 0..3 {
        for col1 in 0..6 {
            let mut value = row as i32;
            log!(&env, "logg row value ", value , symbol_short!("another"));
            let mut value = col as i32;
            log!(&env, "logg col value ", value , symbol_short!("another"));
            let mut value = col1 as i32;
            log!(&env, "logg col1 value {}", value );
            log!(&env, "current_contract_add: {}", current_contract_add);

            


            let (coordx,coordy)= coordinates[col1];
            if coordx==row && coordy==col {
                log!(&env, "logg ");
                matrixnum[row][col]=9999;

            }
        }
        
    }
}
//
//
let mut fila2: usize = 0;
let mut colu2: usize = 0;
matrix[fila2][colu2] = string00;
//
//
let (fila, colu)= find2(&matrix, stringtrx) ;
//
   let hello = "Hello"; // &str implements Clone
   assert_eq!("Hello", hello.clone());
//
        let s1: &str = "World";
        let mut stringbool1= String::from_slice(&env, "trueley22");

        let mystorage = env.storage();
        let key = symbol_short!("key");
        mystorage.persistent().set(&key, &1);
        assert_eq!(mystorage.persistent().has(&key), true);
        assert_eq!(mystorage.persistent().get::<_, i32>(&key), Some(1));
//
    let booleantype :bool;
    booleantype = mystorage.persistent().has(&key);
    let mut stringbool1= String::from_slice(&env, "trueleyy");
    if booleantype { //== true  {
        let stringbool1  = String::from_slice(&env, "VERDADERITO");
    }
//
//
        let _old_message = "nomessage";
        let resetmessage = String::from_slice(&env, "reset");
//
        let ln1: u32;
        ln1 = message.len(); 
        //length of message received
        // get the data named "
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
        if message == resetmessage {
            old_message = String::from_slice(&env, "ResetMessageStored");
        }
        //


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
        let msg = "kmac alfa 1.0 25/10/2023 1520";
        //
        let sout = String::from_slice(&env, msg);
//
        return (ln1, count, fila1, colu1,vec![&env,  sout, old_message, message, stringbool1],booleantype,
        matrixnum[0][0],matrixnum[0][1],matrixnum[0][2],matrixnum[1][0],matrixnum[1][1],matrixnum[1][2]);
//
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
           
          
        
    }  
    }

