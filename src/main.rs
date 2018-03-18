mod Fracs;
use Fracs::*;
mod Mats;
use Mats::*;

fn main() {
    let ayy = Frac::from(-5);
    let bee = Frac::from(8);
    let see = Frac::new(-3, 5);
    let dee = Frac::new(7, 3);
    println!("a = {}", ayy);
    println!("b = {}", bee);
    println!("c = {}", see);
    println!("d = {}", dee);
    println!("a * c = {}", see.mul(ayy));
    println!("c * d = {}", dee.mul(see));
    println!("b / d = {}", bee.div(dee));
    println!("a + b = {}", ayy.add(bee));
    let mut matrix = Matrix::from_dimension((3, 3));
    for a in 0..matrix.dimension.0 {
        for b in 0..matrix.dimension.1 {
            //println!("Adding to matrix: {}", Frac::from((3 * a + b + 1) as i32));
            matrix.matrix[a][b] = Frac::from((3 * a + b + 1) as i32);
        }
    }
    matrix.matrix[1][1] = Frac::from(15);
    println!("Start:\n{}\n", matrix);
    matrix.row_echelon_form(true);
    //println!("\n{}\n", matrix);
    let r1 = vec![Frac::from(6), Frac::from(25), Frac::from(58), Frac::from(-54), Frac::from(5)];
    let r2 = vec![ Frac::from(-52), Frac::from(-41), Frac::from(30), Frac::from(23), Frac::from(28)];
    let r3 = vec![Frac::from(55), Frac::from(-74), Frac::from(-70), Frac::from(-56), Frac::from(17)];
    let r4 = vec![Frac::from(11), Frac::from(81), Frac::from(9), Frac::from(-10), Frac::from(-53)];
    let new_matr_data = vec![r1, r2, r3, r4];
    let new_matr = Matrix::from_vecs(new_matr_data);
    match new_matr {
        Ok(mut mat) => {
            println!("\n{}\n", mat);
            mat.reduced_row_echelon_form(true);
            println!("\n{}\n", mat);
        },
        Err(e) => println!("{}", e)
    }
}