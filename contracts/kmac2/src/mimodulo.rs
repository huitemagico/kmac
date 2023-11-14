//use std::fs::File;
//extern crate std;





// pub fn print_str(input: &str) {
//     //std::println!("{}", input);
//     0
// }
    

// let mut data_file = File::create("data.txt").expect("creation failed");

//     // Write contents to the file
//     data_file.write("Hello, World!".as_bytes()).expect("write failed");


pub fn plus_two(x: u32) -> u32 {
    x + 2
}
//comm nov 09 cambio a 6x6 
// pub fn put_coordinates() -> [(usize, usize); 6] {
//     let mut coordinates: [(usize, usize); 6] = [(0, 0); 6];

    pub fn put_coordinates() -> [(usize, usize); 36] {
        let mut coordinates: [(usize, usize); 36] = [(0, 0); 36];
        //ojo es de 6x6 la mat por lo tato son 36 valores
        let mut idx = 0;
        
        //let mut fila1: usize = 0;
        //must be indicated where the FLOW NO EXISTS!
        //let  row0:usize= 0;
        //let  col0:usize= 0;
        //
       // let row0: usize = 0;      
        //let  col1:usize = 0;
        //
        //let  row0:usize= 0;
        //let  col2:usize= 0;
        //
        //let  row1:usize= 0;
        //let  col0:usize= 0;
        //
        //let  row1:usize= 0;
        //let  col1:usize= 0;
        //
        //let  row1:usize= 0;
        //let  col2:usize= 0;
        //
        
        coordinates[idx] = (0,2);idx = idx+1;
        
        coordinates[idx] = (0,3);idx = idx+1;
        coordinates[idx] = (0,4);idx = idx+1;
        coordinates[idx] = (0,5);idx = idx+1;
        coordinates[idx] = (1,3); idx = idx+1;
        coordinates[idx] = (1,4); idx = idx+1;
        coordinates[idx] = (1,5); idx = idx+1;
        coordinates[idx] = (2,0); idx = idx+1;
        coordinates[idx] = (2,1); idx = idx+1;
        coordinates[idx] = (2,4); idx = idx+1;
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


//matrix of the core functions
//must not be modified!
//the numbers are ARBITRARY.
    pub fn put_corefunc() -> [[i32; 6]; 6] {
        //let mut coordinates: [(usize, usize); 36] = [(0, 0); 36];
        //let mut idx = 0;
        //cell <> 0 must be indicated where the FUNCTION REALLY EXISTS!
        let mut matrixnum: [[i32; 6]; 6] = [[0; 6]; 6];
        let  number = 0;//means not function
        for row in 0..5 { //6 cols 6 filas
        for col in 0..5 {
            matrixnum[row][col]=number;
        }
    }
    matrixnum[0][0]=11;
    matrixnum[0][1]=12;
    matrixnum[1][2]=9;
    matrixnum[2][3]=16;
    matrixnum[3][2]=21;
    matrixnum[3][0]=19;

    // 
    
    //coordinates[idx] = (0,2);idx = idx+1;
    //     coordinates[idx] = (0,2);idx = idx+1;
        
    //     coordinates[idx] = (0,3);idx = idx+1;
    //     coordinates[idx] = (0,4);idx = idx+1;
    //     coordinates[idx] = (0,5);idx = idx+1;
    //     coordinates[idx] = (1,3); idx = idx+1;
    //     coordinates[idx] = (1,4); idx = idx+1;
    //     coordinates[idx] = (1,5); idx = idx+1;
    //     coordinates[idx] = (2,0); idx = idx+1;
    //     coordinates[idx] = (2,1); idx = idx+1;
    //     coordinates[idx] = (2,4); idx = idx+1;
    //     coordinates[idx] = (2,5); idx = idx+1;
    //     coordinates[idx] = (3,1); idx = idx+1;
    //     coordinates[idx] = (3,3); idx = idx+1;
    //     coordinates[idx] = (3,4); idx = idx+1;
    //     coordinates[idx] = (3,5); idx = idx+1;
    //     coordinates[idx] = (4,0); idx = idx+1;
    //     coordinates[idx] = (4,1); idx = idx+1;
    //     coordinates[idx] = (4,2); idx = idx+1;
    //     coordinates[idx] = (4,3); idx = idx+1;
    //     coordinates[idx] = (4,4); idx = idx+1;
    //     coordinates[idx] = (4,5); idx = idx+1;
    //     coordinates[idx] = (5,0); idx = idx+1;
    //     coordinates[idx] = (5,1); idx = idx+1;
    //     coordinates[idx] = (5,2); idx = idx+1;
    //     coordinates[idx] = (5,3); idx = idx+1;
    //     coordinates[idx] = (5,4); idx = idx+1;
    //     coordinates[idx] = (5,5); 
    //     coordinates
    matrixnum
    }

// fn get_mat (mat: [[String; 3]; 2]) -> String {
        //     use soroban_sdk::{Env, String};
        //    // "hola" ESTO DA MISMATCHED TYPES EXPECTED STRTING FOUND &STR
        //    //let mut matstring= String::from_slice(&env, "trueley");
        //    //let mut matstring=mat [1][1];
        //    let s1: &str = clone(mat [1][1]);
        // }
           
          //--------------------------------columnas----filas
        //   fn find_string_in_matrix(matrix: &[[String; 3]; 2], target: &String) -> (usize, usize) {
        //     let mut fila1: usize = 0;
        //     let mut colu1: usize = 0;
        //     for (row_idx, row) in matrix.iter().enumerate() {
        //         for (col_idx, element) in row.iter().enumerate()   {
        //             if element == target {
        //                 fila1=row_idx;
        //                 colu1=col_idx;                     
        //             }                
        //         }                 
        //     }
        //     return (fila1,colu1);  
        //     }
        
          // for (row_idx, row) in matrix.iter().enumerate() {
            //     for (col_idx, element) in row.iter().enumerate()   {
            //         if element == target {
            //             fila1=row_idx;
            //             colu1=col_idx;
                        
                     
            //         }
                     
            //         }
            //     }
            
            //

/* 
pub fn Ext_Fill_Mat(matrix: &mut [[String; 3]; 2], String string00) {
    
            matrix[1][1] = string00;
        
    }
    */
    //-NO FUNCIONA -------------EJ E M P L O   M A T R I Z   D E F I N I T I O N    ---D O S ---
/* no existe to_string
let matrix: [[String; 3]; 3] = [
        ["foo".to_string(), "bar".to_string(), "baz".to_string()],
        ["qux".to_string(), "quux".to_string(), "corge".to_string()],
        ["grault".to_string(), "garply".to_string(), "waldo".to_string()],
    ];
*/
//--------------FIN EJ E M P L O   M A T R I Z   D E F I N I T I O N    ---D O S ---

 
        //--------------------------------------ejemplo como recorrer la matriz--------------------------------------
        // NO SE COMO RESCATAR UN VALOR DESDE LA MATRIZ
        //lo sig entrega mismatched type expected &str found String
        //let mut string2=     String::from_slice(&env, two_dimensional_array[1][1]);
        //veamos en https://google.github.io/comprehensive-rust/basic-syntax/string-slices.html
        //supongamos que lo que quiero es un String, al estilo de stringbool1 , erscatado desde la matriz.
        /*
        ahora bien, podria directamante ver los valores de la matriz (que son String).. y pasarlo al vector de salida
        */     

