use std::fmt;
use std::cmp;

use Fracs;

#[derive(Clone)]
pub struct Matrix {
    pub dimension: (usize, usize),
    pub matrix: Vec<Vec<Fracs::Frac>>
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut matr = String::from("");
        let mut longest_in_column: Vec<usize> = Vec::with_capacity(self.dimension.1);
        for _ in 0..self.dimension.1 {
            longest_in_column.push(0);
        }
        for a in 0..self.dimension.0 {
            for b in 0..self.dimension.1 {
                if self.matrix[a][b].as_string().len() > longest_in_column[b] {
                    longest_in_column[b] = self.matrix[a][b].as_string().len();
                }
            }
        }
        for a in 0..self.dimension.0 {
            let mut line = String::from("");
            if a == 0 {
                line = format!("⎡ {}", line);
            } else if a == self.dimension.0 - 1 {
                line = format!("⎣ {}", line);
            } else {
                line = format!("⎢ {}", line);
            }
            for b in 0..self.dimension.1 {
                let mut spacer_left = String::from("");
                let elem_string = self.matrix[a][b].as_string();
                for _ in 0..longest_in_column[b] - elem_string.len() {
                    spacer_left = format!("{}{}", spacer_left, " ");
                }
                if b == self.dimension.1 - 1 {
                    line = format!("{}{}{}", line, spacer_left, elem_string);
                } else {
                    line = format!("{}{}{}, ", line, spacer_left, elem_string);
                }
            }
            if a == 0 {
                line = format!("{} ⎤", line);
            } else if a == self.dimension.0 - 1 {
                line = format!("{} ⎦", line);
            } else {
                line = format!("{} ⎥", line);
            }
            if a == self.dimension.0 - 1 {
                matr = format!("{}{}", matr, line);
            } else {
                matr = format!("{}{}\n", matr, line);
            }
        }
        write!(f, "{}", matr)
    }
}

enum RowOps {
    add((usize, usize)),
    sub((usize, usize)),
    mul((usize, Fracs::Frac)),
    div((usize, Fracs::Frac)),
    swap_rows((usize, usize)),
}


impl Matrix {
    pub fn from_dimension(dim: (usize, usize)) -> Matrix {
        let mut mat: Vec<Vec<Fracs::Frac>> = Vec::with_capacity(dim.0);
        for _ in 0..dim.0 {
            let mut row: Vec<Fracs::Frac> = Vec::with_capacity(dim.1);
            for _ in 0..dim.1 {
                row.push(Fracs::Frac::from(0));
            }
            mat.push(row);
        }
        Matrix {
            dimension: dim,
            matrix: mat
        }
    }

    pub fn from_vecs(vecs: Vec<Vec<Fracs::Frac>>) -> Result<Matrix, String> {
        for a in 0..vecs.len() - 1 {
            for b in a..vecs.len() {
                if vecs[a].len() != vecs[b].len() {
                    return Err(String::from("One or more vecs in input vec had unequal length."));
                }
            }
        }
        let ret = Matrix {
            dimension: (vecs.len(), vecs[0].len()),
            matrix: vecs
        };
        Ok(ret)
    }

    pub fn add(&mut self, other: Matrix) -> Result<&mut Matrix, String> {
        if self.dimension.0 != other.dimension.0 || self.dimension.1 != other.dimension.1 {
            return Err(String::from("Matrices are not of the same dimension - unable to perform addition."));
        }
        for i in 0..self.dimension.0 {
            for j in 0..self.dimension.1 {
                self.matrix[i][j] = self.matrix[i][j].add(other.matrix[i][j]);
            }
        }
        Ok(self)
    }

    pub fn sub(&mut self, other: Matrix) -> Result<&mut Matrix, String> {
        if self.dimension.0 != other.dimension.0 || self.dimension.1 != other.dimension.1 {
            return Err(String::from("Matrices are not of the same dimension - unable to perform subtraction."));
        }
        for i in 0..self.dimension.0 {
            for j in 0..self.dimension.1 {
                self.matrix[i][j] = self.matrix[i][j].sub(other.matrix[i][j]);
            }
        }
        Ok(self)
    }

    pub fn mul(&mut self, other: Matrix) -> Result<Matrix, String> {
        if self.dimension.1 != other.dimension.0 {
            return Err(String::from("Matrices do not have matching b, c dimensions for a, b x c, d."));
        }
        let mut ret = Matrix::from_dimension((self.dimension.0, other.dimension.1));
        for a in 0..self.matrix[0].len() {
            let mut total = Fracs::Frac::from(0);
            for b in 0..other.matrix.len() {
                let new = self.matrix[a][b].mul(other.matrix[b][a]);
                total = total.add(new);
            }
            ret.matrix[a].push(total);
        }
        Ok(ret)
    }

    fn row_op(&mut self, op: RowOps) {
        match op {
            RowOps::add(tup) => {
                for b in 0..self.dimension.1 {
                    self.matrix[tup.0][b] = self.matrix[tup.0][b].add(self.matrix[tup.1][b]);
                }
                //println!("{}", self);
            },
            RowOps::sub(tup) => {
                for b in 0..self.dimension.1 {
                    self.matrix[tup.0][b] = self.matrix[tup.0][b].sub(self.matrix[tup.1][b]);
                }
                //println!("{}", self);
            },
            RowOps::mul(tup) => {
                for b in 0..self.dimension.1 {
                    self.matrix[tup.0][b] = self.matrix[tup.0][b].mul(tup.1);
                }
                //println!("{}", self);
            },
            RowOps::div(tup) => {
                for b in 0..self.dimension.1 {
                    self.matrix[tup.0][b] = self.matrix[tup.0][b].div(tup.1);
                }
                //println!("{}", self);
            },
            RowOps::swap_rows(tup) => {
                self.matrix.swap(tup.0, tup.1);
                //println!("{}", self);
            }
        }
    }

    pub fn row_ops_add(&mut self, target_row: usize, tool: usize) {
        //println!("ROA | target: {}, tool: {}", target_row, tool);
        self.row_op(RowOps::add((target_row, tool)))
    }

    pub fn row_ops_sub(&mut self, target_row: usize, tool: usize) {
        //println!("ROS | target: {}, tool: {}", target_row, tool);
        self.row_op(RowOps::sub((target_row, tool)))
    }

    pub fn row_ops_mul(&mut self, target_row: usize, amt: Fracs::Frac) {
        //println!("ROM | target: {}, amt: {}", target_row, amt);
        self.row_op(RowOps::mul((target_row, amt)))
    }

    pub fn row_ops_div(&mut self, target_row: usize, amt: Fracs::Frac) {
        //println!("ROD | target: {}, amt: {}", target_row, amt);
        self.row_op(RowOps::div((target_row, amt)))
    }

    pub fn row_ops_swap(&mut self, row1: usize, row2: usize) {
        //println!("ROSw | row1: {}, row2: {}", row1, row2);
        self.row_op(RowOps::swap_rows((row1, row2)))
    }

    pub fn row_echelon_form(&mut self, print_steps: bool) {
        let max = cmp::min(self.dimension.0, self.dimension.1);
        for a in 0..max {
            for b in 0..a + 1 {
                let amt1 = self.clone().matrix[a][b];
                if b < a {
                    let tst = self.clone().matrix[b][b];
                    if tst.num != 1 || tst.den != 1 {
                        self.row_ops_div(a, amt1);
                        continue;
                    }
                    let mut sign = String::from("");
                    let mut neg = false;
                    match amt1.num > 0 {
                        true => {
                            self.row_ops_mul(b, amt1);
                            self.row_ops_sub(a, b);
                            self.row_ops_div(b, amt1);
                            sign = String::from("-");
                        },
                        false => {
                            let mut tmpamt = amt1;
                            tmpamt.num *= -1;
                            self.row_ops_mul(b, tmpamt);
                            self.row_ops_add(a, b);
                            self.row_ops_div(b, tmpamt);
                            sign = String::from("+");
                            neg = true;
                        }
                    }
                    if print_steps {
                        print!("R{} {} ({}) * R{} → R{0}\n{}\n\n", a + 1, sign, {
                            if neg {
                                amt1.negative().try_simplify()
                            } else {
                                amt1
                            }
                        }, b + 1, self);
                    }
                    continue;
                }
                if b == a /*&& (amt1.num != 1 && amt1.den == 1)*/ {
                    if amt1.num == 0 {
                        let mut other: i32 = -1;
                        for i in (0..max).filter(|&i| i != a) {
                            if self.clone().matrix[i][b].num != 0 {
                                other = i as i32;
                                break;
                            }
                        }
                        if other == -1 {
                            continue;
                        }
                        let other = other as usize;
                        let mut add = true;
                        let amt2 = self.clone().matrix[other][b];
                        match amt2.num > 0 {
                            true => {
                                self.row_ops_add(b, other);
                            }
                            false => {
                                add = false;
                                self.row_ops_sub(b, other);
                            }
                        }
                        let sign = match add {
                            true => String::from("+"),
                            false => String::from("-")
                        };
                        if print_steps {
                            print!("R{} {} R{} → R{0}\n{}\n\n", a + 1, sign, other + 1, self);
                        }
                        let foo = self.clone().matrix[a][b];
                        let foo = foo.inverse();
                        if amt1.num != 1 && amt1.den != 1 {
                            self.row_ops_div(a, amt1);
                            if print_steps {
                                print!("({}) * R{} → R{1}\n{}\n\n", foo, a + 1, self);
                            }
                        }
                        continue;
                    }
                    self.row_ops_div(a, amt1);
                    let amt1 = amt1.inverse();
                    if print_steps {
                        print!("({}) * R{} → R{1}\n{}\n\n", amt1, a + 1, self);
                    }
                    continue;
                }
            }
        }
    }

    pub fn reduced_row_echelon_form(&mut self, print_steps: bool) {
        self.row_echelon_form(print_steps);
        let max = cmp::min(self.dimension.0, self.dimension.1);
        for a in 0..max - 1 {
            for b in a + 1..max {
                let amt = self.clone().matrix[a][b];
                if amt.num != 1 || amt.den != 1 {
                    let coef = match self.clone().matrix[b][b] {
                        Fracs::Frac{num: 1, den: 1} => amt,
                        _ => amt.div(self.clone().matrix[b][b])
                    };
                    self.row_ops_mul(b, coef);
                    self.row_ops_sub(a, b);
                    self.row_ops_div(b, coef);
                    if print_steps {
                        print!("R{} - ({}) * R{} → R{0}\n{}\n", a + 1, coef, b + 1, self);
                    }
                }
            }
        }
    }

    pub fn div(mut self, other: Matrix) -> Result<Matrix, String> {
        Err(String::from("Error! You tried to use code the dev was too lazy to implement."))
    }
}