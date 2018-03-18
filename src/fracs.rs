use std::fmt;

#[derive(Clone, Copy)]
pub struct Frac {
    pub num: i32,
    pub den: i32
}

pub enum CmpRes {
    Eq,
    Lt,
    Gt
}

impl PartialEq for CmpRes {
    fn eq(&self, other: &CmpRes) -> bool {
        match self {
            &CmpRes::Eq => {
                match other {
                    &CmpRes::Eq => true,
                    _ => false
                }
            },
            &CmpRes::Lt => {
                match other {
                    &CmpRes::Lt => true,
                    _ => false
                }
            },
            &CmpRes::Gt => {
                match other {
                    &CmpRes::Gt => true,
                    _ => false
                }
            }
        }
    }
}

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.den == 1 {
            return write!(f, "{}", self.num);
        }
        write!(f, "{} / {}", self.num, self.den)
    }
}

impl From<i32> for Frac {
    fn from(num: i32) -> Self {
        Frac {
            num: num,
            den: 1
        }.try_simplify()
    }
}

impl Frac {
    pub fn new(num: i32, den: i32) -> Self {
        if (num == 0 && den == 0) || den == 0 {
            panic!("Tried to create a fraction with a numerator or denominator of zero. Use a scalar for zero.");
        }
        Frac {
            num: num,
            den: den
        }.try_simplify()
    }

    // Not particularly relevant in this module, mostly in formatting in the matrix module
    pub fn as_string(&self) -> String {
        if self.den == 1 {
            return format!("{}", self.num);
        }
        format!("{} / {}", self.num, self.den)
    }

    pub fn inverse(mut self) -> Frac {
        let temp = self.num;
        self.num = self.den;
        self.den = temp;
        self.try_simplify()
    }

    pub fn negative(mut self) -> Frac {
        self.num *= -1;
        self
    }

    // Probably the most overused function in this module :^)
    pub fn try_simplify(mut self) -> Frac {
        if self.num == 0 {
            if self.den != 1 {
                self.den = 1;
            }
            return self;
        }
        if self.den < 0 && self.num >= 0 { // Keep the negative in the numerator
            self.den *= -1;
            self.num *= -1;
        }
        if self.num < 0 && self.den < 0 { // Simplify to positives
            self.num *= -1;
            self.den *= -1;
        }
        if self.num % self.den == 0 {
            self.num /= self.den;
            self.den /= self.den;
        }
        // Test if the numerator and denominator are coprime; if not, divide by gcd
        let a = match self.num < 0 {
            true => (0 - self.num) as u32,
            false => self.num as u32
        };
        let b = match self.den < 0 {
            true => (0 - self.den) as u32,
            false => self.den as u32
        };
        let test_gcd = get_gcd(a, b) as i32;
        if test_gcd > 1 {
            self.num /= test_gcd;
            self.den /= test_gcd;
        }
        self
    }

    pub fn add(mut self, other: Frac) -> Frac {
        if self.den == other.den || (0 - self.den == other.den && other.num < 0) { // ez case
            self.num += other.num;
            self.try_simplify()
        } else {
            let a = match self.den < 0 {
                true => (0 - self.den) as u32,
                false => self.den as u32
            };
            let b = match other.den < 0 {
                true => (0 - other.den) as u32,
                false => other.den as u32
            };
            let lcm = get_lcm(a, b) as i32;
            let self_mult = lcm / a as i32;
            let other_mult = lcm / b as i32;
            self.num *= self_mult;
            self.num += other.num * other_mult;
            self.den = lcm;
            self.try_simplify()
        }
    }

    pub fn mul(mut self, other: Frac) -> Frac {
        self.num *= other.num;
        self.den *= other.den;
        self.try_simplify()
    }

    pub fn div(self, other: Frac) -> Frac {
        self.mul(other.inverse()).try_simplify()
    }

    pub fn sub(mut self, other: Frac) -> Frac {
        if self.den == other.den { // ez case
            self.num -= other.num;
            return self.try_simplify();
        } else {
            let a = match self.den < 0 {
                true => (0 - self.den) as u32,
                false => self.den as u32
            };
            let b = match other.den < 0 {
                true => (0 - other.den) as u32,
                false => other.den as u32
            };
            let lcm = get_lcm(a, b) as i32;
            let self_mult = lcm / a as i32;
            let other_mult = lcm / b as i32;
            self.num *= self_mult;
            self.num -= other.num * other_mult;
            self.den = lcm;
            self.try_simplify()
        }
    }

    pub fn cmp(&self, other: &Frac) -> CmpRes {
        if self.den == other.den {
            if self.num > other.num {
                return CmpRes::Gt;
            } else if self.num == other.num {
                return CmpRes::Eq;
            } else {
                return CmpRes::Lt;
            }
        }
        // Compare numerators for equal denominators
        let a = match self.den < 0 {
            true => (0 - self.den) as u32,
            false => self.den as u32
        };
        let b = match other.den < 0 {
            true => (0 - other.den) as u32,
            false => other.den as u32
        };
        let lcm = get_lcm(a, b) as i32;
        let self_lcm = self.num * lcm / self.den;
        let other_lcm = other.num * lcm / other.den;
        if self_lcm < other_lcm {
            CmpRes::Lt
        } else if self_lcm == other_lcm {
            CmpRes::Eq
        } else {
            CmpRes::Gt
        }
    }
}

// Using Euclid's Algorithm
fn get_gcd(mut a: u32, mut b: u32) -> u32 {
    loop {
        if a > b {
            a -= b;
        } else if b > a {
            b -= a;
        } else if b % a == 0 {
            return b;
        } else if a % b == 0 {
            return a;
        }
    }
}

// Neat trick here: lcm = a * b / gcd
fn get_lcm(a: u32, b: u32) -> u32 {
    let gcd = get_gcd(a, b);
    a * b / gcd
}