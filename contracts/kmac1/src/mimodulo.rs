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

    pub fn put_coordinates() -> [(usize, usize); 6] {
        let mut coordinates: [(usize, usize); 6] = [(0, 0); 6];
        //ojo es de 2x3 la mat por lo tato son 6 valores
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
        
        coordinates[idx] = (0,0);
        idx = idx+1;
        coordinates[idx] = (1,1);
        idx = idx+1;
        coordinates[idx] = (1,2);
        /*
        idx = idx+1;
        coordinates[idx] = (1,0);
        idx = idx+1;
        coordinates[idx] = (1,1);
        idx = idx+1;
        coordinates[idx] = (1,2);
         */
        


    
        coordinates
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

