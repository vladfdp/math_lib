use std::ops::{Mul,Add, Sub, Neg};
use crate::traits::{Zero,Ring};

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<T:Ring>{ 
    pub coeff: Vec<T>,
    pub n:usize
}


impl<T:Ring> Vector<T>{
    
    pub fn new(coeff:Vec<T>)->Vector<T>{
        Vector{
            coeff: coeff.clone(),
            n:coeff.len()
        }
    }

    pub fn get(&self, index:usize)->T{
        if self.n < index {
            panic!("cannot get element {} as vector is size {}",index ,self.n);
        }
        self.coeff[index].clone()
    }
}


impl<T:Ring> Add for Vector<T>{
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Vector<T>{
        if self.n != rhs.n {
            panic!("Vectors are not the same size {} and {}",self.n,rhs.n);
        }
        Vector::new(
        self.coeff.iter()
        .zip(rhs.coeff.iter())
        .map(|(x,y)|x.clone() + y.clone() )
        .collect())
    }
}

impl<T:Ring> Sub for Vector<T>{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<T:Ring> Mul for Vector<T>{
    type Output = T;

    fn mul(self, rhs: Vector<T>) -> T{
        if self.n != rhs.n {
            panic!("Vectors are not the same size {} and {}",self.n,rhs.n);
        }
        let tmp = self.coeff.iter()
            .zip(rhs.coeff.iter())
            .map(|(x,y)|x.clone() * y.clone() );
        let mut ans =self.coeff[0].zero();
        for x in tmp {
            ans = ans + x;
        }
        ans
    }

    
}

impl<T:Ring> Mul<i32> for Vector<T>{
    type Output = Vector<T>;

    fn mul(self,scalar:i32)-> Vector<T>{
        Vector::new(
            self.coeff.iter().map(|x| x.clone() * scalar).collect()
        )
    }

}



impl<T:Ring> Zero for Vector<T>{
    fn zero(&self)-> Vector<T>{
        Vector::new( vec![ self.coeff[0].zero() ; self.n])
    }
    fn is_zero(&self)-> bool {
        for x in &self.coeff{
            if !x.is_zero(){
                return false;
            } 
        }
        true
    }
}

impl<T:Ring> Neg for Vector<T>{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1
    }
    
}