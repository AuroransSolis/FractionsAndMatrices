#![allow(dead_code, non_snake_case_crates)]

use std::fmt;

#[derive(Clone, Copy)]
pub struct Frac {
    pub num: i32,
    pub den: i32
}

impl PartialEq for Frac {
    fn eq(&self, other: &Frac) -> bool {
        match self.cmp(other) {
            CmpRes::Eq => true,
            _ => false
        }
    }
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
        }
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
            let lcm = get_lcm(self.den, other.den) as i32;
            let self_mult = lcm / self.den;
            let other_mult = lcm / other.den;
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

    pub fn mul_no_ts(mut self, other: Frac) -> Frac {
        self.num *= other.num;
        self.den *= other.den;
        self
    }

    pub fn div(self, other: Frac) -> Frac {
        self.mul(other.inverse()).try_simplify()
    }

    pub fn sub(mut self, other: Frac) -> Frac {
        if self.den == other.den { // ez case
            self.num -= other.num;
            return self.try_simplify();
        } else {
            let lcm = get_lcm(self.den, other.den);
            let self_mult = lcm / self.den;
            let other_mult = lcm / other.den;
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
        let lcm = get_lcm(self.den, other.den);
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

// Not using Euclid's Algorithm anymore because it's really slow >:v
pub fn get_gcd(mut a: u32, mut b: u32) -> u32 {
    loop {
        if b == 0 {
            return a;
        } else {
            let c = b;
            b = a % b;
            a = c;
        }
    }
}

// Neat trick here: lcm = a * b / gcd
pub fn get_lcm(a: i32, b: i32) -> i32 {
    let ayy = match a < 0 {
        true => (0 - a) as u32,
        false => a as u32
    };
    let bee = match b < 0 {
        true => (0 - b) as u32,
        false => b as u32
    };
    let gcd = get_gcd(ayy, bee);
    (ayy * bee / gcd) as i32
}