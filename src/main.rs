#![allow(dead_code)]

mod fracs;
use fracs::*;
mod mats;
use mats::*;

fn main() {
    /*let v2 = vec![16, -14, 6, 1, -13, 3, -12, -9, 10, -19, 2, -7, -4, 4, 5, -3];
    let m1 = Matrix::from_i32_vec(4, v2, TSOpts{try: true, print: true}).unwrap();
    let v1 = vec![-12, 20, -18, -2, 11, -17, -20, 10, 7, -9, -10, -11, 0, 16, 15, -14];
    let m2 = Matrix::from_i32_vec(4, v1, TSOpts{try: true, print: true}).unwrap();
    println!("Now beginning to do stuff.\n");
    print!("m1:\n\n{}\n\n", m1);
    print!("m2:\n\n{}\n\n", m2);
    let mut add = m1.clone();
    match add.add(m2.clone(), false) {
        Ok(mat) => print!("m1 + m2:\n\n{}\n\n", mat),
        Err(e) => println!("Error: {}\n", e)
    }
    let mut sub = m1.clone();
    match sub.sub(m2.clone(), false) {
        Ok(mat) => print!("m1 - m2:\n\n{}\n\n", mat),
        Err(e) => println!("Error: {}\n", e)
    }
    let mut mul = m1.clone();
    match mul.mul(m2.clone(), false) {
        Ok(mat) => print!("m1 * m2:\n\n{}\n\n", mat),
        Err(e) => println!("Error: {}\n", e)
    }
    let mut div = m1.clone();
    match div.div(m2.clone(), false, false) {
        Ok(mat) => print!("m1 / m2 (really m1 * m2^(-1)):\n\n{}\n\n", mat),
        Err(e) => println!("Error: {}\n", e)
    }
    let mut m1_rowef = m1.clone();
    m1_rowef.row_echelon_form(false);
    print!("REF(m1):\n\n{}\n\n", m1_rowef);
    let mut m2_rowef = m2.clone();
    m2_rowef.row_echelon_form(false);
    print!("REF(m2):\n\n{}\n\n", m2_rowef);
    let mut m1_rref = m1.clone();
    m1_rref.reduced_row_echelon_form(false);
    print!("RREF(m1):\n\n{}\n\n", m1_rref);
    let mut m2_rref = m2.clone();
    m2_rref.reduced_row_echelon_form(false);
    print!("RREF(m2):\n\n{}\n\n", m2_rref);*/
    let v1 = vec![11, -14, 5, 0, 2, -20, 18, -2, -10, 3, -19, 17, 9, -15, -5, -9, 16, 19,
                  -17, 8, 7, 1, 20, 13, -11];
    let m1_res = Matrix::from_i32_vec(5, v1, TSOpts{try: false, print: false});
    let mut m1 = match m1_res {
        Ok(matr) => matr,
        Err(e) => panic!(format!("Error! {}", e))
    };
    m1.reduced_row_echelon_form(true);
}