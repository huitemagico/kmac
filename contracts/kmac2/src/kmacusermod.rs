//kmacusermod  rev kmac2.0 December 18, 2023
use soroban_sdk::{Env, String};
// 
pub fn _plus_two(x: u32) -> u32 {
    x + 4
}


pub fn put_coordinates() -> [(usize, usize); 36] {
    let mut coordinates: [(usize, usize); 36] = [(0, 0); 36];
    // 6x6 matrix => 36 cells
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
    let sselcofblnd = String::from_str(&env, "selcofblnd");
    //let buyerpay = "buyerpay";
    let sbuyerpay = String::from_str(&env,"buyerpay");
    let sretactiv = String::from_str(&env,"retactiv");
    let serr01 = String::from_str(&env,"err01");
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
#[allow(dead_code)]
//for VENDING MACHINE, the "builder user" define f4 "maintenance" (from state C to state D)
//
pub fn function4(env:&Env )-> (String,bool) {
    //dummy. See 
    let f4msg= String::from_str(&env, "MAINTENANCE STATE!");
    return (f4msg,true);
}
#[allow(dead_code)]

pub fn function5(env:&Env )-> (String,bool) {
    let f4msg= String::from_str(&env, "FUNCTION5!");
    return (f4msg,true);
}
#[allow(dead_code)]

pub fn function6(env:&Env )-> (String,bool) {
    let f4msg= String::from_str(&env, "FUNCTION6!");
    return (f4msg,true);
}


/*pub fn function6(env:&Env )-> (String,bool) {
    let f4msg= String::from_str(&env, "FUNCTION6!");
    return (f4msg,true);
}*/


#[allow(dead_code)]
pub fn function7(env:&Env )-> (String,bool) {
    //fn savemsg (env:&Env ,nummsj: usize, msg1: &mut String,msg2: &mut String, 
    //let env = Env::default();
    let selcofblnd = "SELECTED COFFEE BLEND!";
    let sselcofblnd = String::from_str(&env, selcofblnd);

    return (sselcofblnd,true);
}
#[allow(dead_code)]
pub fn function8(env:&Env )-> (String,bool) {
    let sf8 = String::from_str(&env, "BUYER PAYED");
    //payment done. It suppose the vending machine delivers the coffee right
    //in an more real simulation, these functions could return an abnormal result,
    //when for instance, the coffee not fill the cup.
    return (sf8,true);
}
#[allow(dead_code)]
pub fn function9(env:&Env )-> (String,bool) {
    let sf8 = String::from_str(&env, "RETURN TO ACTIVE STATE");
    return (sf8,true);
}
#[allow(dead_code)]
pub fn function10(env:&Env )-> (String,bool) {
    let sf8 = String::from_str(&env, "ERR01");
    return (sf8,true);
}
//NOTE ABOUT THE FUNCTIONS WITH GENERIC RETURN
//These are functions that could be fulfilled with extensions.
//Please see the explanation at
// https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#the-three-views-of-kmac-
//https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#c-how-to-extend-the-template
//
//
#[allow(dead_code)]
pub fn function11(env:&Env )-> (String,bool) {
    let sf8 = String::from_str(&env, "F11 DUMMY");
    return (sf8,true);
}
#[allow(dead_code)]
pub fn function12(env:&Env )-> (String,bool) {
    let sf8 = String::from_str(&env, "F12 DUMMY");
    return (sf8,true);
}
#[allow(dead_code)]
pub fn function13(env:&Env )-> (String,bool) {
    let sf8 = String::from_str(&env, "F13 DUMMY");
    return (sf8,true);
}
#[allow(dead_code)]
pub fn function14(env:&Env )-> (String,bool) {
    let sf8 = String::from_str(&env, "F14 DUMMY");
    return (sf8,true);
}
#[allow(dead_code)]
pub fn function15(env:&Env )-> (String,bool) {
    let sf8 = String::from_str(&env, "F15 DUMMY");
    return (sf8,true);
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
//
//fn kndtrx: answer the kind of message is the trx
//This is necessary for validate the buyer identity
// There are 3 types of trx:
//(1) Admin 'supervisor': no matter the current state, always proceed. No matter the user address.
//(2) Admin 'flow': have to respect the FSM states. The user, must be the Admin address
//(3) user : have to respect the   FSM states. The user, must be the Buyer Address
//The following function determines the trx kind type of user
//pub fn kndtrx(env:&Env,nummsg:&i32)-> i32{
    pub fn kndtrx(nummsg:&i32)-> i32{
    let kndtrxv:i32;
    match *nummsg {
        1=>{
            kndtrxv=1;}
        2=>{
            kndtrxv=2;}
        3=>{
            kndtrxv=2;}
        4=>{
            kndtrxv=2;}
        5=>{
            kndtrxv=2;}
        6=>{
            kndtrxv=2;}

        7=>{
            //user function
            kndtrxv=3;}

        8=>{
            //user function
            kndtrxv=3;}
        9=>{
            //admin flow function:return to active state from F state delivery coffee
            //this function is 'build' by the 'builder'. So its 'admin flow'
            kndtrxv=2;}
        _=> {
            //unknown message number supposed be user
            kndtrxv=3;}
    }
    return kndtrxv;

}