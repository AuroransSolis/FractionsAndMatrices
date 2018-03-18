use std::fmt;
use std::cmp;

use fracs;

#[derive(Clone)]
pub struct Matrix {
    pub dimension: (usize, usize),
    pub matrix: Vec<Vec<fracs::Frac>>
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut matr = String::from(""); // Will contain string for entire matrix
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
            let mut line = String::from(""); // String for each individual line
            // Add the appropriate character for the section of the bracket at the start of each line
            if a == 0 {
                line = format!("⎡ {}", line);
            } else if a == self.dimension.0 - 1 {
                line = format!("⎣ {}", line);
            } else {
                line = format!("⎢ {}", line);
            }
            // Add spacing to line up the right side of the numbers in each column
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
            // Append appropriate end symbol for bracket section at the end of each line
            if a == 0 {
                line = format!("{} ⎤", line);
            } else if a == self.dimension.0 - 1 {
                line = format!("{} ⎦", line);
            } else {
                line = format!("{} ⎥", line);
            }
            // Add line to matrix string, add newline if it's not the last line
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
    Add((usize, usize)),
    Sub((usize, usize)),
    Mul((usize, fracs::Frac)),
    Div((usize, fracs::Frac)),
    SwapRows((usize, usize)),
}


impl Matrix {
    pub fn from_dimension(dim: (usize, usize)) -> Self {
        let mut mat: Vec<Vec<fracs::Frac>> = Vec::with_capacity(dim.0);
        for _ in 0..dim.0 {
            let mut row: Vec<fracs::Frac> = Vec::with_capacity(dim.1);
            for _ in 0..dim.1 {
                row.push(fracs::Frac::from(0));
            }
            mat.push(row);
        }
        Matrix {
            dimension: dim,
            matrix: mat
        }
    }

    pub fn from_vecs(vecs: Vec<Vec<fracs::Frac>>) -> Result<Matrix, String> {
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

    pub fn from_1d_vec(width: usize, vec: Vec<i32>) -> Result<Matrix, String> {
        if vec.len() % width != 0 {
            return Err(String::from(
                format!("Input vec len ({}) is not divisible by desired matrix width ({}).", vec.len(), width)
            ));
        }
        let mut matr: Vec<Vec<fracs::Frac>> = Vec::with_capacity(vec.len() / width);
        let mut ct = 0;
        for _ in 0..vec.len() / width {
            let mut new: Vec<fracs::Frac> = Vec::with_capacity(width);
            for _ in 0..width {
                new.push(fracs::Frac::from(vec[ct]));
                ct += 1;
            }
            matr.push(new);
        }
        let ret = Matrix {
            dimension: (vec.len() / width, width),
            matrix: matr
        };
        Ok(ret)
    }

    // Untested
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

    // Untested
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

    // Untested
    pub fn mul(&mut self, other: Matrix) -> Result<Matrix, String> {
        if self.dimension.1 != other.dimension.0 {
            return Err(String::from("Matrices do not have matching b, c dimensions for a, b x c, d."));
        }
        let mut ret = Matrix::from_dimension((self.dimension.0, other.dimension.1));
        for a in 0..self.matrix[0].len() {
            let mut total = fracs::Frac::from(0);
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
            RowOps::Add(tup) => {
                for b in 0..self.dimension.1 {
                    self.matrix[tup.0][b] = self.matrix[tup.0][b].add(self.matrix[tup.1][b]);
                }
            },
            RowOps::Sub(tup) => {
                for b in 0..self.dimension.1 {
                    self.matrix[tup.0][b] = self.matrix[tup.0][b].sub(self.matrix[tup.1][b]);
                }
            },
            RowOps::Mul(tup) => {
                for b in 0..self.dimension.1 {
                    self.matrix[tup.0][b] = self.matrix[tup.0][b].mul(tup.1);
                }
            },
            RowOps::Div(tup) => {
                for b in 0..self.dimension.1 {
                    self.matrix[tup.0][b] = self.matrix[tup.0][b].div(tup.1);
                }
            },
            RowOps::SwapRows(tup) => {
                self.matrix.swap(tup.0, tup.1);
            }
        }
    }

    // Wrapper functions for convenience
    pub fn row_ops_add(&mut self, target_row: usize, tool: usize) {
        self.row_op(RowOps::Add((target_row, tool)))
    }

    pub fn row_ops_sub(&mut self, target_row: usize, tool: usize) {
        self.row_op(RowOps::Sub((target_row, tool)))
    }

    pub fn row_ops_mul(&mut self, target_row: usize, amt: fracs::Frac) {
        self.row_op(RowOps::Mul((target_row, amt)))
    }

    pub fn row_ops_div(&mut self, target_row: usize, amt: fracs::Frac) {
        self.row_op(RowOps::Div((target_row, amt)))
    }

    pub fn row_ops_swap(&mut self, row1: usize, row2: usize) {
        self.row_op(RowOps::SwapRows((row1, row2)))
    }

    pub fn row_echelon_form(&mut self, print_steps: bool) {
        let max = cmp::min(self.dimension.0, self.dimension.1);
        for a in 0..max {
            for b in 0..a + 1 { // Keep tested values "below" or on the diagonal line
                let amt1 = self.clone().matrix[a][b]; // Current value
                if b < a { // "Under" the diagonal line
                    if amt1.num == 0 {
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
                if b == a { // On the diagonal line
                    if amt1.num == 0 {
                        let mut other: i32 = -1;
                        // Find row beneath current one with a value in the columnn that the current
                        // row's missing
                        for i in (b..max).filter(|&i| i != a) {
                            if self.clone().matrix[i][b].num != 0 {
                                other = i as i32;
                                break;
                            }
                        }
                        if other == -1 { // It's okay if there isn't one - just move on
                            continue;
                        }
                        let other = other as usize;
                        let mut add = true;
                        let amt2 = self.clone().matrix[other][b]; // Get second value
                        match amt2.num > 0 {
                            true => {
                                self.row_ops_add(b, other); // Get value in zero element
                            }
                            false => {
                                add = false;
                                self.row_ops_sub(b, other); // Get value in zero element
                            }
                        }
                        let sign = match add {
                            true => String::from("+"),
                            false => String::from("-")
                        };
                        if print_steps {
                            print!("R{} {} R{} → R{0}\n{}\n\n", a + 1, sign, other + 1, self);
                        }
                        let amt1 = self.clone().matrix[a][b]; // Refresh current value
                        if amt1.num != 1 {
                            self.row_ops_div(a, amt1);
                            if print_steps {
                                let foo = amt1.clone().inverse();
                                print!("({}) * R{} → R{1}\n{}\n\n", foo, a + 1, self);
                            }
                        }
                        continue;
                    }
                    self.row_ops_div(a, amt1); // Divide by self
                    if print_steps {
                        let amt1 = amt1.inverse();
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
        for a in (0..max - 1).rev() {
            for b in (a + 1..max).rev() {
                let amt = self.clone().matrix[a][b];
                if !amt.cmp(&fracs::Frac::from(0)).eq(&fracs::CmpRes::Eq) {
                    self.row_ops_mul(b, amt);
                    self.row_ops_sub(a, b);
                    self.row_ops_div(b, amt);
                    if print_steps {
                        print!("R{} - ({}) * R{} → R{0}\n{}\n\n", a + 1, amt, b + 1, self);
                    }
                }
            }
        }
    }

    // The inverse can be achieved by taking a matrix and transforming it into a unit matrix (RREF
    // form) and applying the transformations to a unit matrix. The resulting non-unit matrix is the
    // inverse of the original. This function combines the REF and RREF functions above and applies
    // each transformation to a unit matrix.
    pub fn inverse(&self) -> Result<Matrix, String> {
        let mut slef = self.clone();
        if slef.dimension.0 != slef.dimension.1 {
            return Err(String::from("Matrix must be square in dimension to calculate the inverse."));
        }
        let mut unit = Matrix::from_dimension((slef.dimension.0, slef.dimension.1));
        for a in 0..unit.dimension.0 {
            unit.matrix[a][a] = fracs::Frac::from(1);
        }
        let max = cmp::min(slef.dimension.0, slef.dimension.1);
        for a in 0..max {
            for b in 0..a + 1 {
                let amt1 = slef.clone().matrix[a][b];
                if b < a {
                    match amt1.num > 0 {
                        true => {
                            slef.row_ops_mul(b, amt1);
                            unit.row_ops_mul(b, amt1);
                            slef.row_ops_sub(a, b);
                            unit.row_ops_sub(a, b);
                            slef.row_ops_div(b, amt1);
                            unit.row_ops_div(b, amt1);
                        },
                        false => {
                            let mut tmpamt = amt1;
                            tmpamt.num *= -1;
                            slef.row_ops_mul(b, tmpamt);
                            unit.row_ops_mul(b, tmpamt);
                            slef.row_ops_add(a, b);
                            unit.row_ops_add(a, b);
                            slef.row_ops_div(b, tmpamt);
                            unit.row_ops_div(b, tmpamt);
                        }
                    }
                    continue;
                }
                if b == a {
                    if amt1.num == 0 {
                        let mut other: i32 = -1;
                        for i in (b..max).filter(|&i| i != a) {
                            if slef.clone().matrix[i][b].num != 0 {
                                other = i as i32;
                                break;
                            }
                        }
                        if other == -1 {
                            continue;
                        }
                        let other = other as usize;
                        let amt2 = slef.clone().matrix[other][b];
                        match amt2.num > 0 {
                            true => {
                                slef.row_ops_add(b, other);
                                unit.row_ops_add(b, other);
                            },
                            false => {
                                slef.row_ops_sub(b, other);
                                unit.row_ops_sub(b, other);
                            }
                        };
                        let amt1 = slef.clone().matrix[a][b];
                        if amt1.num != 1 && amt1.den != 1 {
                            slef.row_ops_div(a, amt1);
                            unit.row_ops_div(a, amt1);
                        }
                        continue;
                    }
                    slef.row_ops_div(a, amt1);
                    unit.row_ops_div(a, amt1);
                    continue;
                }
            }
        }
        for a in (0..max - 1).rev() {
            for b in (a + 1..max).rev() {
                let amt = slef.clone().matrix[a][b];
                if !amt.cmp(&fracs::Frac::from(0)).eq(&fracs::CmpRes::Eq) {
                    slef.row_ops_mul(b, amt);
                    unit.row_ops_mul(b, amt);
                    slef.row_ops_sub(a, b);
                    unit.row_ops_sub(a, b);
                    slef.row_ops_div(b, amt);
                    unit.row_ops_div(b, amt);
                }
            }
        }
        for a in 0..max { // Check to see if the original matrix is now a unit matrix
            for b in 0..max {
                if a != b && !slef.clone().matrix[b][a].cmp(&fracs::Frac::from(0)).eq(&fracs::CmpRes::Eq) {
                    return Err(String::from("Unable to convert matrix into unit matrix to make the inverse."));
                }
                if a == b && !slef.clone().matrix[b][a].cmp(&fracs::Frac::from(1)).eq(&fracs::CmpRes::Eq) {
                    return Err(String::from("Unable to convert matrix into unit matrix to make the inverse."));
                }
            }
        }
        Ok(unit)
    }

    // "Divide" by multiplying by the inverse of the other matrix
    pub fn div(&mut self, other: Matrix) -> Result<Matrix, String> {
        if !(self.dimension.1 == other.dimension.0 && other.dimension.0 == other.dimension.1) {
            return Err(String::from("Unable to do division with these two matrices. The divisor must be a square matrix,\
            and the dividend's number of columns must be the same as that of both dimensions in the divisor."));
        }
        let rehto = other.inverse();
        match rehto {
            Err(e) => Err(e),
            Ok(xirtam) => {
                let res = self.mul(xirtam);
                match res {
                    Err(e) => Err(e),
                    Ok(result) => Ok(result)
                }
            }
        }
    }

    pub fn is_linearly_independent(&self) -> bool {
        let mut tst = self.clone();
        tst.reduced_row_echelon_form(false);
        let max = cmp::min(self.dimension.0, self.dimension.1);
        for a in 0..max {
            for b in 0..max {
                if a != b && !tst.matrix[a][b].cmp(&fracs::Frac::from(0)).eq(&fracs::CmpRes::Eq) {
                    return false;
                } else if a == b && !tst.matrix[a][b].cmp(&fracs::Frac::from(1)).eq(&fracs::CmpRes::Eq) {
                    return false;
                }
            }
        }
        true
    }


}