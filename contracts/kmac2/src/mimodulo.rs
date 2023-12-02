//mimodulo.rs rev Nov 26, 2023
use soroban_sdk::{Env, String};
//comm 
pub fn plus_two(x: u32) -> u32 {
    x + 4
}
//comm nov 09 cambio a 6x6 
//comm pub fn put_coordinates() -> [(usize, usize); 6] {
//comm     let mut coordinates: [(usize, usize); 6] = [(0, 0); 6];

    pub fn put_coordinates() -> [(usize, usize); 36] {
        let mut coordinates: [(usize, usize); 36] = [(0, 0); 36];
        //comm ojo es de 6x6 la mat por lo tato son 36 valores
        let mut idx = 0;
        
        
        //coordinates[idx] = (0,0);idx = idx+1;
        coordinates[idx] = (0,2);idx = idx+1;
        
        coordinates[idx] = (0,3);idx = idx+1;
        coordinates[idx] = (0,4);idx = idx+1;
        coordinates[idx] = (0,5);idx = idx+1;
        //
        coordinates[idx] = (1,0); idx = idx+1;
        coordinates[idx] = (1,1); idx = idx+1;
        coordinates[idx] = (1,3); idx = idx+1;
        coordinates[idx] = (1,4); idx = idx+1;
        coordinates[idx] = (1,5); idx = idx+1;
        coordinates[idx] = (2,0); idx = idx+1;
        coordinates[idx] = (2,1); idx = idx+1;
        coordinates[idx] = (2,2); idx = idx+1;
        
        //selcofblnd  ==(2,4)==17 coordinates[idx] = (2,4); idx = idx+1;
        coordinates[idx] = (2,5); idx = idx+1;
        coordinates[idx] = (3,1); idx = idx+1;
        coordinates[idx] = (3,3); idx = idx+1;
        coordinates[idx] = (3,4); idx = idx+1;
        coordinates[idx] = (3,5); idx = idx+1;
        coordinates[idx] = (4,0); idx = idx+1;
        coordinates[idx] = (4,1); idx = idx+1;
        coordinates[idx] = (4,2); idx = idx+1;
        coordinates[idx] = (4,3); idx = idx+1;
        coordinates[idx] = (4,4); idx = idx+1;
        coordinates[idx] = (4,5); idx = idx+1;
        coordinates[idx] = (5,0); idx = idx+1;
        coordinates[idx] = (5,1); idx = idx+1;
        coordinates[idx] = (5,2); idx = idx+1;
        coordinates[idx] = (5,3); idx = idx+1;
        coordinates[idx] = (5,4); idx = idx+1;
        coordinates[idx] = (5,5); 
        coordinates
        
    }

    // pub fn plus_two(x: u32) -> u32 {
    //     x + 4
    // }
//function "msj_t_num_usr". Edit this function for adding new user transaction.
//the number returned must be the location of the transaction in the matrix
//NOT the funtion number
pub fn msj_t_num_usr(env:&Env ,strmsj: &String)-> i32 {
    //let selcofblnd = "selcofblnd";
    let sselcofblnd = String::from_slice(&env, "selcofblnd");
    //let buyerpay = "buyerpay";
    let sbuyerpay = String::from_slice(&env,"buyerpay");
    let sretactiv = String::from_slice(&env,"retactiv");
    let serr01 = String::from_slice(&env,"err01");
    if *strmsj==sselcofblnd {
        return 17;
    }
    if *strmsj==sbuyerpay {
        return 30;
    }
    if *strmsj==sretactiv {
        return 33;
    }
    if *strmsj==serr01{
        return 34;
    }
    return 9999;
}

pub fn function7(env:&Env )-> String {
    //fn savemsg (env:&Env ,nummsj: usize, msg1: &mut String,msg2: &mut String, 
    //let env = Env::default();
    let selcofblnd = "SELECTED COFFEE BLEND!";
    let sselcofblnd = String::from_slice(&env, selcofblnd);
    
    return sselcofblnd;
}
pub fn function8(env:&Env )-> String {
    let sf8 = String::from_slice(&env, "BUYER PAYED");
    //payment done. It suppose the vending machine delivers the coffee right
    //in an more real simulation, these functions must return an abnormal result,
    //when for instance, the coffee not fill the cup.
    return sf8;
}
pub fn function9(env:&Env )-> String {
    let sf8 = String::from_slice(&env, "RET ACTIV");
    return sf8;
}
pub fn function10(env:&Env )-> String {
    let sf8 = String::from_slice(&env, "ERR01");
    return sf8;
}
pub fn function11(env:&Env )-> String {
    let sf8 = String::from_slice(&env, "F11 DUMMY");
    return sf8;
}
pub fn function12(env:&Env )-> String {
    let sf8 = String::from_slice(&env, "F12 DUMMY");
    return sf8;
}
pub fn function13(env:&Env )-> String {
    let sf8 = String::from_slice(&env, "F13 DUMMY");
    return sf8;
}
pub fn function14(env:&Env )-> String {
    let sf8 = String::from_slice(&env, "F14 DUMMY");
    return sf8;
}
pub fn function15(env:&Env )-> String {
    let sf8 = String::from_slice(&env, "F15 DUMMY");
    return sf8;
}
//matrix of the core functions
//(a) IMPORTANT NOTE: the values between matrixnum[0][0] until
//matrixnum[3][2]=6; MUST NOT BE MODIFIED!
//(b) The user could add new trx from matrixnum[2][4]
//(c) the numbers are NOT ARBITRARY.It must follow 1,2 3, etc. These are the "functions numbering"
    pub fn put_corefunc() -> [[i32; 6]; 6] {
        //cell <> 0 must be indicated where the FUNCTION REALLY EXISTS!
        let mut matrixnum: [[i32; 6]; 6] = [[0; 6]; 6];
        let  number = 0;//means not function
        for row in 0..5 { //6 cols 6 filas
        for col in 0..5 {
            matrixnum[row][col]=number;
        }
    }
    matrixnum[0][0]=1;
    matrixnum[0][1]=2;
    matrixnum[1][2]=3;
    matrixnum[2][3]=4;
    matrixnum[3][0]=5;
    matrixnum[3][2]=6;
    //FROM HERE begins the user  functions
    matrixnum[2][4]=7;
    //f7==(2,4)==selcofblnd selected coffee blend==function number 7
    matrixnum[4][5]=8;
    //f8==(4,5)==buyerpay==function number 8
    matrixnum[5][2]=9;
    //f9==(5,2)==retactiv==function number 9 "return to activity", after deliver coffee
    //END the user  functions
    matrixnum
    }

