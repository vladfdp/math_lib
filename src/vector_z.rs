use std::ops::{Mul,Add, Sub, Neg};
use crate::traits::Zero;

#[derive(Debug, PartialEq, Clone)]
pub struct VectorZ{ 
    pub coeff: Vec<i32>,
    pub n:usize
}


impl VectorZ{
    
    pub fn new(coeff:Vec<i32>)->VectorZ{
        VectorZ{
            coeff: coeff.clone(),
            n:coeff.len()
        }
    }

    pub fn get(&self, index:usize)->i32{
        if self.n < index {
            panic!("cannot get element {} as vector is size {}",index ,self.n);
        }
        self.coeff[index]
    }
}


impl Add for VectorZ{
    type Output = VectorZ;

    fn add(self, rhs: VectorZ) -> VectorZ{
        if self.n != rhs.n {
            panic!("Vectors are not the same size {} and {}",self.n,rhs.n);
        }
        VectorZ::new(
        self.coeff.iter()
        .zip(rhs.coeff.iter())
        .map(|(x,y)|x+y)
        .collect())
    }
}

impl Sub for VectorZ{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul for VectorZ{
    type Output = i32;

    fn mul(self, rhs: VectorZ) -> i32{
        if self.n != rhs.n {
            panic!("Vectors are not the same size {} and {}",self.n,rhs.n);
        }
        self.coeff.iter()
        .zip(rhs.coeff.iter())
        .map(|(x,y)|x*y)
        .sum()
        
    }

    
}

impl Mul<i32> for VectorZ{
    type Output = VectorZ;

    fn mul(self,scalar:i32)-> VectorZ{
        VectorZ::new(
            self.coeff.iter().map(|x| scalar * x).collect()
        )
    }

}



impl Zero for VectorZ{
    fn zero(&self)-> VectorZ{
        VectorZ::new( vec![0; self.n])
    }
    fn is_zero(&self)-> bool {
        for x in &self.coeff{
            if x != &0{
                return false;
            } 
        }
        true
    }
}

impl Neg for VectorZ{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1
    }
    
}