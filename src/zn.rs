use std::ops::{Mul,Add};
use crate::traits::{One,Zero};

#[derive(Debug, PartialEq, Clone)]
pub struct Zn{ //ring Z/nZ, if n prime we get a field
    pub nb: i32,
    pub n:i32
}

impl Add for Zn{
    type Output = Zn;

    fn add(self, other: Zn) -> Zn{
        if self.n != other.n {
            panic!("operands don't belong to the same ring Z{} and Z{}",self.n,other.n);        
        }
        Zn {
            nb: (self.nb + other.nb) % self.n,
            n: self.n
        }
    }
}

impl Mul for Zn{
    type Output = Zn;

    fn mul(self, other: Zn) -> Zn{
        if self.n != other.n {
            panic!("operands don't belong to the same ring Z{} and Z{}",self.n,other.n);
        }
        Zn {
            nb: (self.nb * other.nb) % self.n,
            n: self.n
        }
    }
}

impl Mul<i32> for Zn{
    type Output = Zn;

    fn mul(self, scalar:i32 ) -> Zn{
        Zn {
            nb: (self.nb * scalar) % self.n,
            n: self.n
        }
    }
}

impl One for Zn{
    fn one(&self)->Zn{ Zn{nb:1, n: self.n}}
}

impl Zero for Zn{
    fn zero(&self)->Zn{ Zn{nb:0, n: self.n}}
}


