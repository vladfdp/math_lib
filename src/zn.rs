use std::ops::{Mul,Add};
use crate::traits::{One, Zero, Pow, Inv};

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
        let mut nb = (self.nb * scalar) % self.n;
        if scalar < 0{
            nb += self.n;
        }

        Zn {
            nb: nb,
            n: self.n
        }
    }
}

impl One for Zn{
    fn one(&self)->Zn{ Zn{nb:1, n: self.n}}
}

impl Zero for Zn{
    fn zero(&self)->Zn{ Zn{nb:0, n: self.n}}
    fn is_zero(&self)-> bool {
        self.nb == 0
    }
}

impl Inv for Zn{
    fn inv(&self)->Zn{self.pow((self.n-2).try_into().unwrap())}
}


impl Zn {
    pub fn into_Zn(vec: Vec<i32>,n:i32)-> Vec<Zn>{
        let mut ans:Vec<Zn> = Vec::new();
        for i in  vec{
            ans.push(Zn { nb: i % n, n: n });
        }
        ans
    }


}