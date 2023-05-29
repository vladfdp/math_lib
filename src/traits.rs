
use std::ops::{Add,Mul};

pub trait One{ //Trait to get the multiplicative identity of the ring
    fn one(&self)-> Self;
}

pub trait Zero{
    fn zero(&self)-> Self;
}

pub trait Pow: Clone + One + Mul + Mul<Output = Self>{
    fn pow(&self, n:u32)-> Self;
}
impl<T: Clone + One + Mul + Mul<Output = T>> Pow for T {
    fn pow(&self, n:u32)-> T{
        let mut x = self.clone();
        let mut k = n;
        let mut ans = x.one();
        while k > 0{
            if k % 2 == 1{
                ans = ans * x.clone();
            }
            x = x.clone() * x;
            k = k/2;
            }
        ans
    }
}