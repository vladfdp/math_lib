use crate::traits::Ring;
use std::ops::{Mul,Add};
use std::convert::TryInto;


#[derive(Debug, PartialEq, Clone)]
pub struct PolyNx{ //N[X] polynomial, they can be evaluated on any ring
    pub coeff: Vec<u32>
}

impl PolyNx{
    pub fn get_deg(&self)->usize{self.coeff.len()-1}

    pub fn eval<T:Ring>(&self,x:T)-> T {
        let mut ans:T = x.zero();
        for i in 0..self.coeff.len(){
            ans = ans + (x.pow(i.try_into().unwrap()) * self.coeff[i]);
        }
        ans


    }

    pub fn rm_trailing_zeros(mut self)-> PolyNx{
        while self.coeff.last().unwrap() == &0{
            self.coeff.pop();
        }
        self
    }
}


impl Add for PolyNx{
    type Output = PolyNx;

    

    fn add(self, other:PolyNx)-> PolyNx{

        let mut a = self.coeff;
        let mut b = other.coeff;

        while a.len()>b.len() { //we make a and b the same size
            b.push(0);
        }
        while a.len()<b.len() {
            a.push(0);
        }

        PolyNx{ coeff: 
            a.iter()
            .zip(b.iter())
            .map(|(x,y)|x+y).collect()
        }
    }
}

impl Mul for PolyNx{
    type Output = PolyNx;

    

    fn mul(self, other:PolyNx)-> PolyNx{

        let mut ans = vec![0;self.coeff.len()+other.coeff.len()-1];

        for i in 0..self.coeff.len(){
            for j in 0..other.coeff.len(){
                ans[i+j] += self.coeff[i] * other.coeff[j];
                
            }
        }

        PolyNx{ coeff: ans }
    }
}