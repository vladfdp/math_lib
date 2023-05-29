use std::ops::{Mul,Add};
use crate::traits::{One,Zero};

#[derive(Debug, PartialEq, Clone)]
pub struct Zn{ //ring Z/nZ, if n prime we get a field
    pub nb: u32,
    pub n:u32
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

impl Mul<u32> for Zn{
    type Output = Zn;

    fn mul(self, scalar:u32 ) -> Zn{
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

impl Zn{

    



    // fn pow(&self, n: u32) -> Zn { //using fast exponentiation
    //     let mut x = self.copy();
    //     let mut k = n;
    //     let mut ans = Zn::one(self.n);
    //     while k > 0{
    //         if k % 2 == 1{
    //             ans = ans * &x;
    //         }

    //         x = &x * &x;
    //         k = k/2;
    //     }
    //     ans
    // }

    // pub fn inv(&self) -> Zn{  //p must be prime for this to work
    //     self.pow(self.n-2)
    // }
}

