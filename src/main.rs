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
    let v1 = vec![2, 2, 3, 1, -2, -3, 4, -2, -3];
    let mut m1_res = Matrix::from_i32_vec(3, v1, TSOpts{try: true, print: true});
    let mut m1 = match m1_res {
        Ok(matr) => matr,
        Err(e) => panic!(format!("Error! {}", e))
    };
    let inv = m1.inverse(true);
    if let Ok(mat) = inv {
        println!("{}", mat);
    } else {
        println!("Failed to find inverse.");
    }
}