use crate::traits::Ring;
use std::ops::{Mul,Add};
use crate::traits::{One,Zero};
use std::convert::TryInto;


#[derive(Debug, PartialEq, Clone)]
pub struct PolyZx{ //Z[X] polynomial, they can be evaluated on any ring
    pub coeff: Vec<i32>
}


impl PolyZx{
    pub fn get_deg(&self)->usize{self.coeff.len()-1}

    pub fn eval<T:Ring>(&self,x:T)-> T {
        let mut ans:T = x.zero();
        for i in 0..self.coeff.len(){
            ans = ans + (x.pow(i.try_into().unwrap()) * self.coeff[i]);
        }
        ans


    }

    pub fn rm_trailing_zeros(mut self)-> PolyZx{
        while self.coeff.last().unwrap() == &0{
            self.coeff.pop();
        }
        self
    }
}


impl Add for PolyZx{
    type Output = PolyZx;

    

    fn add(self, other:PolyZx)-> PolyZx{

        let mut a = self.coeff;
        let mut b = other.coeff;

        while a.len()>b.len() { //we make a and b the same size
            b.push(0);
        }
        while a.len()<b.len() {
            a.push(0);
        }

        PolyZx{ coeff: 
            a.iter()
            .zip(b.iter())
            .map(|(x,y)|x+y).collect()
        }.rm_trailing_zeros()
    }
}

impl Mul for PolyZx{
    type Output = PolyZx;

    

    fn mul(self, other:PolyZx)-> PolyZx{

        let mut ans = vec![0;self.coeff.len()+other.coeff.len()-1];

        for i in 0..self.coeff.len(){
            for j in 0..other.coeff.len(){
                ans[i+j] += self.coeff[i] * other.coeff[j];   
            }
        }
        PolyZx{ coeff: ans }
    }
}

impl Mul<i32> for PolyZx{
    type Output = PolyZx;

    fn mul(self,scalar:i32)-> PolyZx{
        PolyZx{
            coeff: self.coeff.iter().map(|x| scalar * x).collect()
        }
    }

}

impl Zero for PolyZx {
    fn zero(&self)-> PolyZx{
        PolyZx{coeff:vec![0]}
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

impl One for PolyZx {
    fn one(&self)-> PolyZx{
        PolyZx{coeff:vec![1]}
    }
    
}
