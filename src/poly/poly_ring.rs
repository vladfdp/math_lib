use crate::traits::Ring;
use std::ops::{Mul,Add};
use crate::traits::{One,Zero};
use std::convert::TryInto;

#[derive(Debug, PartialEq, Clone)]
pub struct Poly<T:Ring>{ //Polynomial over a ring
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
        while self.coeff.last().unwrap().is_zero() && self.coeff.len() > 1{
            self.coeff.pop();
        }
        self
    }
}

impl<T:Ring> Add for Poly<T>{
    type Output = Poly<T>;

    

    fn add(self, other:Self)-> Self{

        let mut a = self.coeff;
        let mut b = other.coeff;

        while a.len()>b.len() { //we make a and b the same size
            b.push(b[0].zero());
        }
        while a.len()<b.len() {
            a.push(a[0].zero());
        }

        Poly{ coeff: 
            a.iter()
            .zip(b.iter())
            .map(|(x,y)|x.clone() + y.clone()).collect()
        }.rm_trailing_zeros()
    }
}

impl<T:Ring> Mul for Poly<T> {
    type Output = Poly<T>;
    fn mul(self, other: Poly<T>) -> Poly<T> {
        let mut ans = vec![ self.coeff[0].zero() ;self.coeff.len()+other.coeff.len()-1];

        for i in 0..self.coeff.len(){
            for j in 0..other.coeff.len(){
                ans[i+j] = ans[i+j].clone()
                + self.coeff[i].clone() * other.coeff[j].clone();   
            }
        }
        Poly{ coeff: ans }
    }
    
}

impl<T:Ring> Mul<i32> for Poly<T>{
    type Output = Poly<T>;

    fn mul(self,scalar:i32)-> Poly<T>{
        Poly{
            coeff: self.coeff.iter().map(|x| x.clone() * scalar).collect()
        }
    }

}

impl<T:Ring> Zero for Poly<T>  {
    fn zero(&self)->Poly<T>{
        Poly { coeff: vec![self.coeff[0].zero()]}
    }

    fn is_zero(&self)-> bool {
        self.coeff.len() == 1 && self.coeff[0].is_zero()
    }
}

impl<T:Ring> One for Poly<T>  {
    fn one(&self)->Poly<T>{
        Poly { coeff: vec![self.coeff[0].one()]}
    }
}