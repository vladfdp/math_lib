
use std::ops::{Add,Mul};

pub trait One{ //Trait to get the multiplicative identity of the ring
    fn one(&self)-> Self;
}

impl One for i32{
    fn one(&self)-> Self {
        1
    }
}

pub trait Zero{
    fn zero(&self)-> Self;

    fn is_zero(&self)-> bool;
}
impl Zero for i32 {
    fn zero(&self)-> Self {
        0
    }
    fn is_zero(&self)-> bool {
        self == &0
    }
}


pub trait Pow: Clone + One + Mul + Mul<Output = Self>{
    fn pow(&self, n:i32)-> Self;
}
impl<T: Clone + One + Mul + Mul<Output = T>> Pow for T {
    fn pow(&self, n:i32)-> T{
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


pub trait Ring: Pow + Mul<i32, Output = Self> + Zero + Add  +Add<Output = Self> {}
impl<T: Pow + Mul<i32, Output = T> + Zero + Add  + Add<Output = T>> Ring for T {}