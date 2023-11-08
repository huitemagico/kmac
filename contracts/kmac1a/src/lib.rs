#![no_std]
// rev. 28102023 20:41
// rev 31102023 11:57 ... auth aqui
//comm rev 31102023 11:57 tratando de meter auth aqui
//comm 01112023 receive 2 parameters more, same as auth example :-|
//comm rev Nov02: desde ahora los comentarios van en ingles :-)
// 01112023 19:33 
// (a)receive 2 parameters more, same as auth example 
// see runk1.sh for more details
// (b)Nothing more about auth. 
// Pending issue, dealing with adm and normal user
// rev 06112023 new structure at github repo. kmac1==deliverable 1 version
//comm (user: Address, value: u32)

// 01112023 receive 2 parameters more, same as auth example :-|
//comm BETA3 of the echo2. Realmente es el ALFA de KMAC :-D
//comm prueba conectar un modulo externo:
//comm gracias a https://github.com/fernandodavidmartinez/soroban-safe-maths-mod/blob/main/src/lib.rs
mod mimodulo;
//use core::simd::i16x32;
//#![no_std]
//
//comm use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, String, Symbol, Vec,log};
use soroban_sdk::{contract, contractimpl,contracttype, Address, symbol_short, vec, Env, String, Symbol, Vec,log,IntoVal};
//use soroban_sdk::{contract, contractimpl, token, Address, Env, IntoVal}
//----use-----------
//comm auth:use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};=> agrego contracttype
//comm auth luego tiene en conctract type..
#[contracttype]
//----contracttype----
pub enum DataKey {
    Kstad(Address), //Counter--> Kreator Stored Address
}
// https://docs.rs/soroban-sdk/20.0.0-rc2/soroban_sdk/attr.contracttype.html

//comm
//comm------- External FUnctions related statements--------------------------------------
//comm .........the external functions must have the follwing:
//comm .........In the main: USE, CALL, pub trait
//comm............(1) plus_two----------------:
//comm ...........use for plus_two OK
use mimodulo::plus_two;
//commuse mimodulo::print_str;
//
//comm...........calling to plus_two ok (see instruction Number ...
//comm...........count = plus_two (count);
//comm...........trait
pub trait Suma2{
    fn plus_two(&self, env: Env,x: u32) -> u32;
  }
//comm.........plus_two end----------------:
//
//comm use mimodulo::put_coordinates;



pub trait PutMat{
//comm ub fn put_coordinates() -> [(usize, usize); 9] {
//comm  pub fn put_coordinates() -> [(usize, usize); 9]
    fn put_coordinates (&self, env: Env) -> [(usize, usize); 9];
  }
//comm  pub trait PrStr{
    
//comm     fn print_str (&self, env: Env,x: u32) -> u32;
//comm   }
  
//
//comm------- External FUnctions related statements END--------------------------------------
//
const COUNTER: Symbol = symbol_short!("COUNTER");
const OLD_MSG: Symbol = symbol_short!("OLD_MSG");
const KSTADR: Symbol = symbol_short!("KSTADR");

mod test;

#[contract]
pub struct Echo2Contract;
// (3) #contract  pub struct ---------
// #[contract]
// pub struct IncrementContract;

#[contractimpl]
impl Echo2Contract {
//comm  kmac out : for "print" the matrix values return them...................................1...2....3...4...5...6
//comm  kmac in  : adding new parameter to the program: the "trx"
//comm               obligado..{.echo2.......}. {kmac.....} 
//comm  kmac trx : (1) receive from caller
//comm  kmacauth: agrego parametros del auth:
//comm Nov02:use runk1.sh see leame.txt
//comm Nov02:test call is: client.echo2    (&user_1, &5,  &first_message, &first_trx);
//comm pub fn increment(env: Env, user: Address, value: u32) -> u32 {
       pub fn echo2    (env: Env,  user: Address, value: u32, message: String, sender:String ) -> 
       (  u32,u32, u32, u32, Vec<String>, bool,i32,i32,i32,i32,i32,i32) {
 // 
//comm test logs BEGIN
 //commlet env = Env::default();

//comm log!(&env, "a logggggggggggggggggggggggggggggggg entry");
//comm fin test logs END
//
//comm test current_contract_adress--------------------------------------- BEGIN 
//comm    let env = Env::default();
    let current_contract_add = env.current_contract_address();
    log!(&env, "current_contract_add: {}", current_contract_add);
//comm test current_contract_adress--------------------------------------- END
//
// //comm Nov02 test address from parameter--------------------------------------- BEGIN 
// //let key = DataKey::Counter(user.clone()); //from auth lib
// //comm    let env = Env::default();
// //let add_from_par = user;

// let cloneuser0=  user.clone();
// log!(&env, "cloneuser0: {}", cloneuser0);
// let cloneuser=  user.clone();
// log!(&env, "cloneuser: {}", cloneuser);
// //
// let cloneuser2= cloneuser.clone();
// log!(&env, "cloneuser2: {}", cloneuser2);
// //comm devuelve:
// //commAddress(
// //comm Account(
// //comm AccountId(
// //comm        PublicKeyTypeEd25519(
// //comm            Uint256(95bb799dceedb4a0e9fb38c9a8dc99f538b331094aa6ee732faa599618037b1b),
// //comm test address from parameter--------------------------------------------- END

//comm nota: (????) ver doc notasfromprojects y proyecto errors en /home/aiglesiasvera/aivwork/sorowork/errors
//
//comm test register_contract ---------------------------------------------BEGIN 
//comm : falla. solo funciona en test
//comm let contract_id = env.register_contract(None, Echo2Contract);
//comm test register_contract ---------------------------------------------END
//
//comm transaction trx (2): basic validation ------------------------------BEGIN
// if trx == resetmessage {
//     old_message = String::from_slice(&env, "ResetMessageStored");
// }
//comm echo2 actions depending on OLD_MSG -----------------------------------END

//comm PENDIENTE
//comm transaction trx (2): basic validation ------------------------------END

//comm kmac test trx en duro BEGIN
    let stringtrx = String::from_slice(&env, "13");  
//comm kmac test trx en duro END
//
 //      
 //comm kmac standard default init values for the matrix 2x3--------------BEGIN
 //comm kmac alfa: in this version, the matrix has to init every call
 //comm kmac PENDIENTE : perhaps matrix save and restore? no creo.
    log!(&env, "logg standard default init values for the matrix");
    let string11 = String::from_slice(&env, "11");
    let string21 = String::from_slice(&env, "21");
    let string12 = String::from_slice(&env, "12");
    let string22 = String::from_slice(&env, "22");
    let string13 = String::from_slice(&env, "13");
    let string23 = String::from_slice(&env, "23");
    let string00 = String::from_slice(&env, "00");
//comm kmac standard default init values for the matrix 2x3--------------END    
//
//comm kmac the external (user) module kmac usr mod init mat has the duty
//comm for setting (re-setting) some cells depending on the flow of FSM, after
//
//comm ----STRING  m a t _ d e f _ s t d _ i n i t-------------------------BEGIN
//comm     STRING Matrix definition and standard init filling  
//comm --Example(dos filas y tres columnas)
//comm...............................columnas....filas
  let  mut  matrix: [[String; 3]; 2] = [[
     string11,string12,string13],[string21,string22,string23 ]];
//comm STRING Matrix definition and standard init filling--------------- END
//
//comm ----i32 NUMBER  m a t _ d e f _ s t d _ i n i t-------------------------BEGIN
//comm  matriz de numeros. Inicializados 12 3 etc fila a fila.
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
//comm ----i32 NUMBER  m a t _ d e f _ s t d _ i n i t----------------------END
//comm    // for row in matrixnum.iter_mut() {
//comm     for cell in row.iter_mut() {
//comm         *cell = number;
//comm         number += 1;
//comm     }
//comm }
//comm let algo= matrixnum[0][1];
//comm --------------p u t _ c o o r d i n a t e s ---------------------BEGIN 
//comm the external function "put_coordinates" init the vector coordinates corresponding to cell (i,j) 
//comm      where NO exists transaction 
let coordinates = mimodulo::put_coordinates();
log!(&env, "logging when the coordinates put the values in matrixnum ");
//comm --------------p u t _ c o o r d i n a t e s ---------------------END
//comm let i32_value: i32 = usize_value as i32;
//comm let mut value = 5;
//
//comm ---------s e t t i n g _ m a t r i x  --------------------------------BEGIN
//comm The lib.rs(main module) "fn" "setting_matrix", does the setting of the cells
//comm (by the coordinates) where NO exists transaction.
//comm for convention, the value 9999 in cell (i:0...,j:0..) means that the trx (i,j) does not exist.
//comm PENDING: The setting_matrix ¿would be external?..¿or only the put_ccordinates?
//comm PENDING Decide how big is the hard coded matrix dimensions (6x6)??
//comm beta 2x3
for row in 0..2{
    for col in 0..3 {
        for col1 in 0..6 {
            let mut value = row as i32;
           //comm log!(&env, "logg row value ", value , symbol_short!("another"));
            let mut value = col as i32;
            //comm log!(&env, "logg col value ", value , symbol_short!("another"));
            let mut value = col1 as i32;
            //comm log!(&env, "logg col1 value {}", value );
            //comm log!(&env, "current_contract_add: {}", current_contract_add);
            //comm log!(&env, "logg value row {} col {} coordinates idx {}",row,symbol_short!("another"));
            let (coordx,coordy)= coordinates[col1];
            if coordx==row && coordy==col {
                log!(&env, "logg ");
                matrixnum[row][col]=9999;

            }
        }

    }
}
//comm ---------s e t t i n g _ m a t r i x  --------------------------------BEGIN
//comm TEST update cell :---------------------------------------- BEGIN  
//comm para probar como update una cel de la matrix 
let mut fila2: usize = 0;
let mut colu2: usize = 0;
matrix[fila2][colu2] = string00;
//comm si lo de arriba funciona... pèro el string00 queda "atrapado" y ya no lo puedo ocupar :-/
//comm TEST update cell :---------------------------------------- END
//
//comm esto no anda no le gusta y no se como llamar a la macro println
//comm let pi = 3.14159;
//comm std::println!("The value of pi is approximately {:.2}", pi);
//
//comm ---ejemplo llamado a funcion pasando la matriz
//comm let  fila: usize;
//comm let mut colu: usize;
//comm let (fila, colu)= find_string_in_matrix(&matrix, &stringtrx) ;
//comm fn find2 ------------------------------------------------ BEGIN  
// the function "find2" has to find the parameter "transaction" () in   the matrix
let (fila, colu)= find2(&matrix, stringtrx) ;
//
//comm  test clone BEGIN    
   let hello = "Hello"; // &str implements Clone
   assert_eq!("Hello", hello.clone());
//comm let s1: &str = clone(two_dimensional_array [1][1]);      
//comm test clone END        
//
//comm test asignacion a String BEGIN (y luego cambiarlo)  
        let s1: &str = "World";
        //let mut stringbool1= String::from_slice(&env, "trueley22");
        //let mut stringbool1;
//comm test asignacion a String END
 //comm test play with storage -------------------------------------------------------BEGIN       

         let mystorage = env.storage();
         let mut key = symbol_short!("key99");
         mystorage.persistent().set(&key, &1);
         assert_eq!(mystorage.persistent().has(&key), true);
         assert_eq!(mystorage.persistent().get::<_, i32>(&key), Some(1));
         //comm 0611 check if exsts this key------BEGIN
         let mut booleantype :bool=true;
         booleantype=mystorage.persistent().has(&key);
         log!(&env, "booleantype when there is the key: %s", booleantype);
         //comm ver 447 ahi sehace este chequeo
        //  //comm check if exsts this key KSTADR -------------------------KSTADR
        //  //let mut key = symbol_short!("KSTADR");
        //  booleantype=mystorage.persistent().has(&KSTADR);
        //  log!(&env, "booleantype when for KSTADR existence:", booleantype);
        //  //comm booleantype=findkey (env);
        //  //
        //  let mut stringbool1= String::from_slice(&env, "trueleyy");
    // if booleantype { //== true  {
    //     let stringbool1  = String::from_slice(&env, "VERDADERITO");
    // }
//comm test play with storage -----------------------------------------------------------END     
//
//comm test cambiando valor de String segun boolean --------------BEGIN
//comm const TRULY: Symbol = symbol_short!("TRULY");
//comm const FALSY: Symbol = symbol_short!("FALSY");
  
   // booleantype = mystorage.persistent().has(&key);
 //comm   asigno a stringbool que va a contener true or false segun el valor del booelano
 //comm   ESTO NO ESTA FUNCIONANDO *****************
    // let mut stringbool1= String::from_slice(&env, "trueleyy");
    // if booleantype { //== true  {
    //     let stringbool1  = String::from_slice(&env, "VERDADERITO");
    // }
//comm test cambiando valor de String segun boolean --------------END
//
//comm kmac : guardando el mensaje recibido con una key "kmac"--- BEGIN
//comm PENDIENTE 
//comm  guardando el mensaje recibido con una key "kmac" -------- END
//
//comm ---------------e c h o 2   b l o c k ----------- --------------------------------------------BEGIN 
//comm echo2 variables and kmac variables
//comm this is the alfa version of init variables that exist in the core---
        let _old_message = "nomessage";
        
//comm  hardcode the kreator messages  -----------------------------------------------------BEGIN
// the kreator messages are hardcoded in the core of kmac
// only the kreator can send the following messages:
        let savekadmessage = String::from_slice(&env, "savekad");
        let resetmessage = String::from_slice(&env, "reset"); 
        let resetkad = String::from_slice(&env, "resetkad"); 
//comm  hardcode the kreator messages  -----------------------------------------------------END
//
//comm  administrative tasks:-------------------------------BEGIN
// (a)    save for work message received
        let ln1: u32;
        ln1 = message.len();  //length of message received
//comm  administrative tasks:-------------------------------END
        
//
//comm echo2 get COUNTER to internal variable BEGIN
        let mut count: u32 = env.storage().persistent().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
//comm echo2 get COUNTER END        
//comm echo2 internal counter update BEGIN
        count += 1;
//comm echo2 internal counter update END        
//
//

//comm echo2 Get the old message BEGIN
        let mut old_message = env
            .storage()
            .persistent()
            .get(&OLD_MSG)
            .unwrap_or(String::from_slice(&env, "NoOldMessage0"));
//comm echo2 Get the old message END
//
// //comm kmac Get the kreator address ------------********------------------------------------BEGIN
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

// //comm kmac Get the kreator address **************************------------------------------END
//
//comm ANALYSIS OF THE MESSAGES (1)-------------------------------------------------BEGIN
//comm echo2 actions depending on OLD_MSG -----------------------------------BEGIN
//comm esto es para echo2. PENDIENTE: pasar todo el analisis de los messages en un solo lugar
        if message == resetmessage {
            old_message = String::from_slice(&env, "ResetMessageStored");
        }
//comm echo2 actions depending on OLD_MSG -----------------------------------END
//comm echo2 Save new message OLD_MSG and COUNTER in storage -----BEGIN
//comm esto es una herencia de echo2. PENDIENTE revisar si es util mantenerlo
        env.storage().persistent().set(&OLD_MSG, &message);
        env.storage().persistent().set(&COUNTER, &count);
//comm echo2 Save new message OLD_MSG and COUNTER in storage -----END        
//comm ANALYSIS OF THE MESSAGES (1)-------------------------------------------------END
//
//comm e c h o 2   b l o c k---------------------------------------------------------------------- END
//
//
//comm  test funcion EXTERNA en modulo mimodulo que suma 2  BEGIN 
        count = plus_two (count);
//comm  test funcion EXTERNA en modulo mimodulo que suma 2  END
//
//comm test no funcionando? asignacion a variables BEGIN
        let fila1: u32;
        let colu1: u32;
        fila1=fila as u32;
        colu1=colu as u32;
//comm  let num_rows:i32= 2;//matrixnum.len();
//comm  let num_cols:i32= 3;//matrixnum[0].len();
        let somevalue1:i32=matrixnum[0][0];//100;
        let somevalue2:i32=matrixnum[0][1];//num_rows;//matrixnum[0][2];
        let somevalue3:i32=matrixnum[0][2];//num_cols;//matrixnum[0][3];
        let somevalue4:i32=matrixnum[1][0];
        let somevalue5:i32=matrixnum[1][1];
        let somevalue6:i32=matrixnum[1][2];
//comm test no funcionando? asignacion a variables END
//comm=====================================================================================================
//comm -----------k m a c   b l o c k  -------------------------------------------------------------- BEGIN
//comm=====================================================================================================
//comm 0.1 para probar el "Kreator address", la logica es:
//comm (a)
//comm    Via CLI se crea una address (identities.sh) asociada al "Kreator"
//comm   Esta address va a ser la "address del Kreator".
//comm (b)
//comm   Se crea un "invoke" especial, que debiera ser unico en el ciclo de vida de la maquina(*).
//comm   Este invoke, identifica al source (que es el firmante) como "Kreator", y se le pasa al programa
//comm   la address del Kreator, con una trx "set_Kreator_address", que queda asociada al Kreator, mediante grabado en el storage.
//comm  (c) este invoke llama al punto central de entrada del kmac (revisable, podria llamar directamente a uan funcion de
//comm  categoria "Kreator")
//comm    El kmac reconoce el firmante, el user=Kreator, y la trx...y llama a la "set_Kreator_address" la que 
//comm asocia el "rol de Kreator" a la "Kreator address".
//comm    ---PENDIENTE: que pasa si la address admin se pierde (posible solucion, una trx "cold reset")??
//comm    beta version: kmac no se preocupa de si ya existe una Kreator address en el storage. 
//comm    ¿validacion: si el "rol de Kreator" ya existe...=> error, y debiera el llamante o llamar al cold reset..
//        o conformarse con trx de buyer.
//comm    Nota: segun lo comprendido, al identificar al "source" esto equivale a agregar la  "firma" en el message.
//comm
//comm mucho sentido.
//comm 1. validar trx, contra las trx validas desde la matrix
//comm ...si la trx no es valida...genera msgrespuesta error y se va a final_return
//comm 2. recuperar el estado anterior. Si no existe "default"??
//comm 3. si la trx es valida segun el estado anterior fn(matrix)
//comm...estado nuevo <--- nuevo estado (matrix, trx)
//comm...llama a la funcion =fn(trx).
//comm...la fn externa devuelve una respuesta (ver detalle de fn externas)
//comm...graba estado_nuevo, estado_anterior, trx, respuesta 
//comm trx, msgrespuestafinal, estado_anterior, estado_nuevo, fnrespuesta
//comm 
//comm version message BEGIN
        let msg = "kmac1a=deliverable1=beta 1.1 Nov072023-2047";
        //
        let sout = String::from_slice(&env, msg);
        let rstkadm = "rstkadm";
        let srstkadm = String::from_slice(&env, rstkadm);
//comm version message END
//
//comm ANALYSIS OF THE MESSAGES (2)-------------------------------------------------BEGIN
//let brwsvkadmsg = &savekadmessage;
let clonemsg=  message.clone();
log!(&env, "clonemsg: {}", clonemsg);
let mut stringbool1= String::from_slice(&env, "trueley22");
let mut stringbool2= String::from_slice(&env, "trueley22");
//let csavekadm=savekadmessage.clone();
//let brwmsg = &message;
let strtrue  = String::from_slice(&env, "KeyExists");
let strfalse  = String::from_slice(&env, "KeyNoExists");
let strtrue2  = String::from_slice(&env, "bad trx!(err panic) KeyExists");
let strfalse2  = String::from_slice(&env, "trx accepted,KeyNoExists");
let strtrue3  = String::from_slice(&env, "bad trx!(err panic) KeyExists");
let strfalse3  = String::from_slice(&env, "trx accepted,KeyNoExists");
//
let cloneuser=  user.clone();
log!(&env, "cloneuser: {}", cloneuser);

let cloneuser2= cloneuser.clone();
log!(&env, "cloneuser2: {}", cloneuser2);
//
if clonemsg==srstkadm {
    log!(&env, "reset key kreator");
    if is_initialized(&env) {
        //panic!("contract has been already initialized");
        stringbool2  =strtrue2;
    }else {stringbool2  =strfalse2;
    //comm lo apropiado seria remove la actual y setear... pero no lo veo necesario
    //comm por lo tanto, el reset viene siendo igual que el set...o viceversa.
    env.storage().persistent().set(&KSTADR, &cloneuser2);
}
        
}

if clonemsg==savekadmessage {
    //comm VALIDATE THE KREATOR FIRST!! PENDING 
    //comm here we are NOT sure that is the kreator the sender.. ONLY that the message is savekadmessage, so:
    //comm (1) --------------------------------------------setting work variables:BEGIN 
//comm
//
//comm verifica BEGIN si la key ya existe. Si existe, debe abortar. No se acepta reescribir la clave
//let mut key = symbol_short!("KSTADR");

// let strtrue  = String::from_slice(&env, "KeyExists");
// let strfalse  = String::from_slice(&env, "KeyNoExists");
// let strtrue2  = String::from_slice(&env, "KeyExists");
// let strfalse2  = String::from_slice(&env, "KeyNoExists");
         booleantype=mystorage.persistent().has(&KSTADR);
         log!(&env, "booleantype when for KSTADR existence:", booleantype);
         //let mut stringbool1= String::from_slice(&env, "trueleyy");
    if booleantype { //== true  {
        //let stringbool1  = String::from_slice(&env, "VERDADERITO");
        stringbool1  =strtrue;
     } else {stringbool1  =strfalse;}
//comm lo verifico doble :-/ paratest de la funcion
if is_initialized(&env) {
    //panic!("contract has been already initialized");
    stringbool2  =strtrue3;
}else {stringbool2  =strfalse3;}
//comm verifica END













    let cloneuser0=  user.clone();
    log!(&env, "cloneuser0: {}", cloneuser0);
    // let cloneuser=  user.clone();
    // log!(&env, "cloneuser: {}", cloneuser);
    // let cloneuser2= cloneuser.clone();
    // log!(&env, "cloneuser2: {}", cloneuser2);
    // is the sender == Kreator?
    // user.require_auth(); <----this is not useful because buyer can send kreator messages
    //comm (2.1)-------------------- Get the kreator address ----------------------------------------BEGIN
    let kreatorstoredaddress: Address;
    kreatorstoredaddress = env
    .storage()
    .persistent()
    .get(&KSTADR)
    .unwrap_or     (cloneuser);
    let clonestored=  kreatorstoredaddress.clone();
//comm ya que las trx de administracion solo las puede enviar el kreator, lo sig debiera bastar:
user.require_auth(); //pero no funciona...el buyer manda la trx y pasa soplado.
//comm por lo tanto destapo el require_auth_for_args siguiente:
 //cloneuser2.require_auth_for_args(
 //(user.clone(),kreatorstoredaddress ).into_val(&env),);
    //
    //comm (1) ---------------------------------------------setting work variables:END
//comm (2) -----------------------------------------------------verify if its SET or RESET :BEGIN 
    // log the new or old (?) kreator address...depend on the existence check of KSTADR...
     log!(&env, "new or old (?) kreator address(from stored)", clonestored);
     log!(&env, "booleantype when for KSTADR existence:", booleantype);
     log!(&env, "new (parameter) kreator address", cloneuser0);
     if clonestored==cloneuser0{
         booleantype=true;
         log!(&env, "OLD AND NEW ARE THE SAME!");
     }else {
         log!(&env, "KSTADR RESETED!!!");
     }
    //comm (2.1)-------------------- Get the kreator address ----------------------------------------END
    //comm (2) -----------------------------------------------------verify if its SET or RESET :END
    //comm (3)-------------------------SET THE KREATOR ADDRESS----BEGIN
    //comm esto no es necesario si no hubo RESET...o sea si OLD AND NEW were equal
    env.storage().persistent().set(&KSTADR, &cloneuser2);
    //comm (3)-------------------------SET THE KREATOR ADDRESS ---END
    }
    //comm=====================================================================================================
    //comm -----savekadmessage  b l o c k  -------------------------------------------------------------- END
    //comm=====================================================================================================

    //_ => println!("Other"),
    //_ => {
            //log!(&env, "message received not recognized!!: {}", clonemsg) ;
            else {
            log!(&env, "message received not recognized!!: ") ;
         }
//}

//comm echo2 actions depending on OLD_MSG -----------------------------------END
//comm echo2 Save new message OLD_MSG and COUNTER in storage -----BEGIN
//comm esto es una herencia de echo2. PENDIENTE revisar si es util mantenerlo
env.storage().persistent().set(&OLD_MSG, &message);
env.storage().persistent().set(&COUNTER, &count);
//comm echo2 Save new message OLD_MSG and COUNTER in storage -----END        
//comm ANALYSIS OF THE MESSAGES (1)-------------------------------------------------END
//============================================================
//
//comm end return BEGIN
//comm    pub fn echo2(env: Env, message: String, trx:String ) -> (  u32,u32, u32, u32, Vec<String>, bool,i32,i32,i32,i32,i32,i32) {
//comm  kmac adding the trx string last place of the Vec:(after stringbool1)
        return (ln1, count, fila1, colu1,vec![&env,  sout, old_message, message, stringbool1, stringbool2],booleantype,
        matrixnum[0][0],matrixnum[0][1],matrixnum[0][2],matrixnum[1][0],matrixnum[1][1],matrixnum[1][2]);
//
//comm end return END
//
//comm no usado  BEGIN
//comm        //    somevalue1,somevalue2,somevalue3,somevalue4,somevalue5,somevalue6);
//comm        //
//comm        // fn plus_one(x: u32) -> u32 {
//comm        //     x + 1
//comm        // }

//comm test register contract version alfa: en el main

//comm pub fn my_method(env: Env) {
//comm     let current_contract_id = env.current_contract_id();
//comm     // Now you can use current_contract_id
 
//comm }
//comm no usado  END

fn is_initialized(env: &Env) -> bool {
    //env.storage().instance().has(&DataKey::Init)
    env.storage().persistent().has(&KSTADR)
    
    
}

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

         //comm after find2 begin
         //commfn findkey(env: Env) -> (bool) {
         //comm findkey in storage -------------------------------------------------------BEGIN       

         //commlet mystorage = env.storage();
         //let mut key = symbol_short!(keystr);
         //assert_eq!(mystorage.persistent().has(&keystr), true);
         //commlet mut booleantype :bool=true;
         //const KSTADR: Symbol = symbol_short!("KSTADR");
         //commlet mut key = symbol_short!("KSTADR");
         //commbooleantype=mystorage.persistent().has(&key);
         //commlog!(&env, "booleantype ask if there is the key: %s", booleantype);
         //commbooleantype
         //comm}
         //comm findkey in storage -------------------------------------------------------END 
         //comm after findkey  end
          
        
    }  
    }

