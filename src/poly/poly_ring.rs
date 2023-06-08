use crate::traits::Ring;
use std::ops::{Mul,Add};
use std::convert::TryInto;

pub struct Poly<T:Ring>{ //Z/nZ[X] polynomial
    pub coeff: Vec<T>
}

impl<T> Poly<T>
where
    T:Ring
{
    pub fn get_deg(&self)->usize{self.coeff.len()-1}

    pub fn eval(&self,x:T)-> T {
        let mut ans:T = x.zero();
        for i in 0..self.coeff.len(){
            ans = ans + (x.pow(i.try_into().unwrap()) * self.coeff[i].clone());
        }
        ans


    }

    pub fn rm_trailing_zeros(mut self)-> Poly<T> {
        while self.coeff.last().unwrap().is_zero(){
            self.coeff.pop();
        }
        self
    }
}