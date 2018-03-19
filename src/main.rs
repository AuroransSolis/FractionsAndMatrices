mod fracs;
use fracs::*;
mod mats;
use mats::*;

fn main() {
    let v1 = vec![16, -14, 6, 1, -13, 3, -12, -9, 10, -19, 2, -7, -4, 4, 5, -3];
    let mut m1 = Matrix::from_i32_vec(4, v1, TSOpts{try: true, print: true}).unwrap();
    let v2 = vec![-12, 20, -18, -2, 11, -17, -20, 10, 7, -9, -10, -11, 0, 16, 15, -14];
    let mut m2 = Matrix::from_i32_vec(4, v2, TSOpts{try: true, print: true}).unwrap();
    println!("Now beginning to do stuff.\n");
    match m1.div(m2, true, true) {
        Ok(mat) => print!("{}", mat),
        Err(e) => print!("{}", e)
    }
    //println!("{}", m1);
}