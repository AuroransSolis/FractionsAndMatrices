#![allow(dead_code)]

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
        let matr = self.to_string();
        write!(f, "{}", matr)
    }
}

pub struct TSOpts {
    pub try: bool,
    pub print: bool
}

impl From<(bool, bool)> for TSOpts {
    fn from(tup: (bool, bool)) -> Self {
        TSOpts {
            try: tup.0,
            print: tup.1
        }
    }
}

impl Matrix {
    pub fn to_string(&self) -> String {
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
        matr
    }

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

    pub fn from_vecs(vecs: Vec<Vec<fracs::Frac>>, try_simplify_opts: TSOpts) -> Result<Matrix, String> {
        for a in 0..vecs.len() - 1 {
            for b in a..vecs.len() {
                if vecs[a].len() != vecs[b].len() {
                    return Err(String::from("One or more vecs in input vec had unequal length."));
                }
            }
        }
        let mut ret = Matrix {
            dimension: (vecs.len(), vecs[0].len()),
            matrix: vecs
        };
        print!("Constructed matrix:\n{}\n\n", ret);
        if try_simplify_opts.try {
            ret.try_simplify_matrix(try_simplify_opts.print);
        }
        Ok(ret)
    }

    pub fn from_i32_vec(width: usize, vec: Vec<i32>, try_simplify_opts: TSOpts) -> Result<Matrix, String> {
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
        let mut ret = Matrix {
            dimension: (vec.len() / width, width),
            matrix: matr
        };
        print!("Constructed matrix:\n\n{}\n\n\n", ret);
        if try_simplify_opts.try {
            ret.try_simplify_matrix(try_simplify_opts.print);
        }
        Ok(ret)
    }
}

pub mod format {
    use std::fmt;
    use mats::Matrix;

    pub enum Separator {
        Plus,
        Minus,
        Times,
        Divide,
        Space
    }

    impl fmt::Display for Separator {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                &Separator::Plus => write!(f, "+"),
                &Separator::Minus => write!(f, "-"),
                &Separator::Times => write!(f, "*"),
                &Separator::Divide => write!(f, "/"),
                &Separator::Space => write!(f, " ")
            }
        }
    }

    pub fn add_mat_to_string(string: String, matr: &Matrix, separator: Separator) -> String {
        let mut lines_vec = string.lines().map(|line| String::from(line)).collect::<Vec<String>>();
        let mut mat_vec = matr.to_string().lines().map(|line| String::from(line)).collect::<Vec<String>>();
        let (mut top_gap, mut height_comp_state) = (0, -2);
        if lines_vec.len() > mat_vec.len() {
            height_comp_state = -1;
        } else if lines_vec.len() == mat_vec.len() {
            height_comp_state = 0;
        } else {
            height_comp_state = 1;
        }
        if height_comp_state == 0 {
            for i in 0..lines_vec.len() {
                if i == lines_vec.len() / 2 {
                    lines_vec[i] = format!("{} {} {}", lines_vec[i], separator, mat_vec[i]);
                } else {
                    lines_vec[i] = format!("{}   {}", lines_vec[i], mat_vec[i]);
                }
            }
            let mut ret = String::from("");
            for i in 0..lines_vec.len() - 1 {
                ret = format!("{}{}\n", ret, lines_vec[i]);
            }
            ret = format!("{}{}", ret, lines_vec[lines_vec.len() - 1]);
            return ret;
        }
        let mut ws_max = 0;
        let mut max = 0;
        if height_comp_state == -1 {
            ws_max = lines_vec.len() - mat_vec.len();
            max = lines_vec.len();
        } else {
            ws_max = mat_vec.len() - lines_vec.len();
            max = mat_vec.len();
        }
        for a in (0..ws_max).filter(|&a| a & 1 == 0) {
            top_gap += 1;
        }
        let mut new_lines: Vec<String> = Vec::new();
        match height_comp_state > 0 {
            true => {
                for i in top_gap..max {
                    if i == mat_vec.len() / 2 {
                        mat_vec[i] = format!("{} {} {}", mat_vec[i], separator, lines_vec[i - top_gap]);
                    } else {
                        mat_vec[i] = format!("{}   {}", mat_vec[i], lines_vec[i - top_gap]);
                    }
                }
                new_lines = mat_vec;
            },
            false => {
                for i in top_gap..max {
                    if i == lines_vec.len() / 2 {
                        lines_vec[i] = format!("{} {} {}", lines_vec[i], separator, mat_vec[i - top_gap]);
                    } else {
                        lines_vec[i] = format!("{}   {}", lines_vec[i], mat_vec[i - top_gap]);
                    }
                }
                new_lines = lines_vec;
            }
        }
        let mut ret = String::from("");
        for i in 0..new_lines.len() - 1 {
            ret = format!("{}{}\n", ret, new_lines[i]);
        }
        ret = format!("{}{}", ret, new_lines[new_lines.len() - 1]);
        ret
    }
}

pub mod operations {
    use fracs;
    use mats::Matrix;
    use mats::format::*;

    impl Matrix {
        pub fn add(&mut self, other: Matrix, print_action: bool) -> Result<&mut Matrix, String> {
            if self.dimension.0 != other.dimension.0 || self.dimension.1 != other.dimension.1 {
                return Err(String::from("Matrices are not of the same dimension - unable to perform addition."));
            }
            if print_action {
                println!("{}\n", add_mat_to_string(self.to_string(), &other, Separator::Plus));
            }
            for i in 0..self.dimension.0 {
                for j in 0..self.dimension.1 {
                    self.matrix[i][j] = self.matrix[i][j].add(other.matrix[i][j]);
                }
            }
            Ok(self)
        }

        pub fn sub(&mut self, other: Matrix, print_action: bool) -> Result<&mut Matrix, String> {
            if self.dimension.0 != other.dimension.0 || self.dimension.1 != other.dimension.1 {
                return Err(String::from("Matrices are not of the same dimension - unable to perform subtraction."));
            }
            if print_action {
                println!("{}\n", add_mat_to_string(self.to_string(), &other, Separator::Minus));
            }
            for i in 0..self.dimension.0 {
                for j in 0..self.dimension.1 {
                    self.matrix[i][j] = self.matrix[i][j].sub(other.matrix[i][j]);
                }
            }
            Ok(self)
        }

        pub fn mul(&mut self, other: Matrix, print_action: bool) -> Result<Matrix, String> {
            if self.dimension.1 != other.dimension.0 {
                return Err(String::from("Matrices do not have matching b, c dimensions for a, b x c, d."));
            }
            if print_action {
                println!("{}\n", add_mat_to_string(self.to_string(), &other, Separator::Times));
            }
            let mut ret = Matrix::from_dimension((self.dimension.0, other.dimension.1));
            for a in 0..self.dimension.0 {
                for o in 0..other.dimension.1 {
                    let mut total = fracs::Frac::from(0);
                    let other_column = (0..other.dimension.0).map(|i| other.matrix[i][o]).collect::<Vec<fracs::Frac>>();
                    for b in 0..self.dimension.1 {
                        let new = self.matrix[a][b].mul(other_column[b]);
                        total = total.add(new);
                    }
                    ret.matrix[a][o] = total;
                }
            }
            Ok(ret)
        }

        // "Divide" by multiplying by the inverse of the other matrix
        pub fn div(&mut self, other: Matrix, print_action: bool, print_inverse_steps: bool) -> Result<Matrix, String> {
            if !(self.dimension.1 == other.dimension.0 && other.dimension.0 == other.dimension.1) {
                return Err(String::from("Unable to do division with these two matrices. The divisor must be a square matrix,\
            and the dividend's number of columns must be the same as that of both dimensions in the divisor."));
            }
            if print_action {
                println!("{}\n", add_mat_to_string(self.to_string(), &other, Separator::Divide));
            }
            let rehto = other.inverse(print_inverse_steps);
            match rehto {
                Err(e) => Err(e),
                Ok(xirtam) => {
                    if print_action {
                        println!("{}\n", add_mat_to_string(self.to_string(), &xirtam, Separator::Times));
                    }
                    let res = self.mul(xirtam, false);
                    match res {
                        Err(e) => Err(e),
                        Ok(result) => Ok(result)
                    }
                }
            }
        }
    }
}

pub mod transforms {
    use std::cmp;
    use fracs;
    use mats::Matrix;
    use mats::format::*;

    enum RowOps {
        Add((usize, usize)),
        Sub((usize, usize)),
        Mul((usize, fracs::Frac)),
        Div((usize, fracs::Frac)),
        SwapRows((usize, usize)),
        TrySimplify((usize, bool))
    }

    impl Matrix {
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
                },
                RowOps::TrySimplify(tup) => {
                    self.try_simplify(tup.0, tup.1);
                }
            }
        }

        pub fn try_simplify_matrix(&mut self, print_steps: bool) {
            println!("Attempting to simplify matrix.\n");
            let mut did_simplification = false;
            for row in 0..self.matrix.len() {
                match self.try_simplify(row, print_steps) {
                    true => did_simplification = true,
                    _ => {}
                }
            }
            if did_simplification {
                println!("\nWas able to simplify. New matrix:\n\n{}\n", self);
            }
            if !did_simplification {
                println!("\nWas unable to simplify.\n");
            }
        }

        pub fn try_simplify(&mut self, row: usize, print_steps: bool) -> bool {
            if self.matrix[row].len() > 1 {
                let row_vec: &Vec<fracs::Frac> = &self.clone().matrix[row];
                let (mut zero_count, mut plus_minus_one_count, mut neg_count, mut non_zeros) = (0, 0, 0, Vec::new());
                for i in 0..row_vec.len() {
                    let tst = row_vec[i];
                    match tst.cmp(&fracs::Frac::from(0)) {
                        fracs::CmpRes::Eq => zero_count += 1,
                        fracs::CmpRes::Lt => {
                            neg_count += 1;
                            match tst.eq(&fracs::Frac::from(-1)) {
                                true => plus_minus_one_count += 1,
                                false => non_zeros.push(tst)
                            }
                        }
                        fracs::CmpRes::Gt => {
                            match tst.eq(&fracs::Frac::from(1)) {
                                true => plus_minus_one_count += 1,
                                false => non_zeros.push(tst)
                            }
                        }
                    }
                }
                if neg_count == self.matrix[row].len() {
                    for b in 0..neg_count {
                        self.matrix[row][b] = self.matrix[row][b].negative();
                    }
                    if print_steps {
                        print!("(-1) * R{} → R{0}\n{}\n\n", row, self);
                    }
                }
                if non_zeros.len() > 1 {
                    if plus_minus_one_count == 0 {
                        let first_frac = non_zeros[0];
                        let second_frac = non_zeros[1];
                        let mut num_gcd = fracs::get_gcd(first_frac.num as u32, second_frac.num as u32);
                        let mut den_gcd = fracs::get_gcd(first_frac.den as u32, second_frac.den as u32);
                        if num_gcd == 1 && den_gcd == 1 {
                            return false;
                        }
                        if non_zeros.len() > 2 {
                            for i in 2..non_zeros.len() {
                                let next = non_zeros[i];
                                num_gcd = fracs::get_gcd(num_gcd, next.num as u32);
                                den_gcd = fracs::get_gcd(den_gcd, next.den as u32);
                                if num_gcd == 1 && den_gcd == 1 {
                                    return false;
                                }
                            }
                        }
                        let sorta_gcd = fracs::Frac::new(num_gcd as i32, den_gcd as i32);
                        for b in 0..row_vec.len() {
                            self.matrix[row][b] = self.matrix[row][b].div(sorta_gcd);
                        }
                        if print_steps {
                            print!("({}) * R{} → R{1}\n{}\n\n", sorta_gcd.inverse(), row, self);
                        }
                        return true;
                    }
                }
            }
            false
        }

        // Wrapper functions for convenience
        pub fn row_ops_add(&mut self, target_row: usize, tool: usize) {
            self.row_op(RowOps::Add((target_row, tool)));
        }

        pub fn row_ops_sub(&mut self, target_row: usize, tool: usize) {
            self.row_op(RowOps::Sub((target_row, tool)));
        }

        pub fn row_ops_mul(&mut self, target_row: usize, amt: fracs::Frac) {
            self.row_op(RowOps::Mul((target_row, amt)));
        }

        pub fn row_ops_div(&mut self, target_row: usize, amt: fracs::Frac) {
            self.row_op(RowOps::Div((target_row, amt)));
        }

        pub fn row_ops_swap(&mut self, row1: usize, row2: usize) {
            self.row_op(RowOps::SwapRows((row1, row2)));
        }

        pub fn row_ops_try_simplify(&mut self, row: usize, print_steps: bool) {
            self.row_op(RowOps::TrySimplify((row, print_steps)));
        }

        pub fn row_echelon_form(&mut self, print_steps: bool) {
            if print_steps {
                println!("------- Starting REF -------\n");
            }
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
            if print_steps {
                println!("------- Completed REF, starting RREF -------\n");
            }
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
        pub fn inverse(&self, print_steps: bool) -> Result<Matrix, String> {
            let mut slef = self.clone();
            if slef.dimension.0 != slef.dimension.1 {
                return Err(String::from("Matrix must be square in dimension to calculate the inverse."));
            }
            let mut unit = Matrix::from_dimension((slef.dimension.0, slef.dimension.1));
            for a in 0..unit.dimension.0 {
                unit.matrix[a][a] = fracs::Frac::from(1);
            }
            if print_steps {
                print!("Setup at start of inverse calculation:\n{}\n\n", add_mat_to_string(slef.to_string(), &unit, Separator::Space));
            }
            let max = cmp::min(slef.dimension.0, slef.dimension.1);
            for a in 0..max {
                for b in 0..a + 1 { // Keep tested values "below" or on the diagonal line
                    let amt1 = slef.clone().matrix[a][b]; // Current value
                    if b < a { // "Under" the diagonal line
                        if amt1.num == 0 {
                            continue;
                        }
                        let mut sign = String::from("");
                        let mut neg = false;
                        match amt1.num > 0 {
                            true => {
                                slef.row_ops_mul(b, amt1);
                                unit.row_ops_mul(b, amt1);
                                slef.row_ops_sub(a, b);
                                unit.row_ops_sub(a, b);
                                slef.row_ops_div(b, amt1);
                                unit.row_ops_div(b, amt1);
                                sign = String::from("-");
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
                            }, b + 1, add_mat_to_string(slef.to_string(), &unit, Separator::Space));
                        }
                        continue;
                    }
                    if b == a { // On the diagonal line
                        if amt1.num == 0 {
                            let mut other: i32 = -1;
                            // Find row beneath current one with a value in the columnn that the current
                            // row's missing
                            for i in (b..max).filter(|&i| i != a) {
                                if slef.clone().matrix[i][b].num != 0 {
                                    other = i as i32;
                                    break;
                                }
                            }
                            if other == -1 { // It's okay if there isn't one - just move on
                                continue;
                            }
                            let other = other as usize;
                            let mut add = true;
                            let amt2 = slef.clone().matrix[other][b]; // Get second value
                            match amt2.num > 0 {
                                true => {
                                    slef.row_ops_add(b, other); // Get value in zero element
                                    unit.row_ops_add(b, other);
                                }
                                false => {
                                    add = false;
                                    slef.row_ops_sub(b, other); // Get value in zero element
                                    unit.row_ops_sub(b, other);
                                }
                            }
                            let sign = match add {
                                true => String::from("+"),
                                false => String::from("-")
                            };
                            if print_steps {
                                print!("R{} {} R{} → R{0}\n{}\n\n", a + 1, sign, other + 1,
                                       add_mat_to_string(slef.to_string(), &unit, Separator::Space));
                            }
                            let amt1 = slef.clone().matrix[a][b]; // Refresh current value
                            if amt1.num != 1 {
                                slef.row_ops_div(a, amt1);
                                if print_steps {
                                    let foo = amt1.clone().inverse();
                                    print!("({}) * R{} → R{1}\n{}\n\n", foo, a + 1,
                                           add_mat_to_string(slef.to_string(), &unit, Separator::Space));
                                }
                            }
                            continue;
                        }
                        slef.row_ops_div(a, amt1); // Divide by slef
                        unit.row_ops_div(a, amt1);
                        if print_steps {
                            let amt1 = amt1.inverse();
                            print!("({}) * R{} → R{1}\n{}\n\n", amt1, a + 1,
                                   add_mat_to_string(slef.to_string(), &unit, Separator::Space));
                        }
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
                        if print_steps {
                            print!("R{} - ({}) * R{} → R{0}\n{}\n\n", a + 1, amt, b + 1,
                                   add_mat_to_string(slef.to_string(), &unit, Separator::Space))
                        }
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
    }
}

pub mod tests {
    use std::cmp;
    use fracs;
    use mats::Matrix;

    impl Matrix {
        pub fn is_linearly_independent(&self) -> bool {
            let mut tst = self.clone();
            tst.row_echelon_form(false);
            let max = cmp::min(self.dimension.0, self.dimension.1);
            for a in 0..max {
                for b in 0..a + 1 {
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
}