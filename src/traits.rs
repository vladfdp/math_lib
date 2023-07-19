
use std::ops::{Add, Sub, Mul, Neg, Div};
use std::fmt::Debug;
use std::cmp::PartialEq;
use std::iter::Sum;

pub trait One{ //Trait to get the multiplicative identity of the ring
    fn one(&self)-> Self;
}

// impl One for i32{ //makes i32 a Ring
//     fn one(&self)-> Self {
//         1
//     }
// }

pub trait Zero{ //get the zero of the ring
    fn zero(&self)-> Self;

    fn is_zero(&self)-> bool;
}

// impl Zero for i32 { //makes i32 a Ring
//     fn zero(&self)-> Self {
//         0
//     }
//     fn is_zero(&self)-> bool {
//         self == &0
//     }
// }


pub trait Pow: Clone + One + Mul + Mul<Output = Self>{
    fn pow(&self, n:u32)-> Self;
}
impl<T: Clone + One + Mul + Mul<Output = Self>> Pow for T {
    fn pow(&self, n:u32)-> T{
        let mut x = self.clone();
        let mut k = n;
        let mut ans = x.one();
        while k > 0{
            if k % 2 == 1{
                ans = ans * x.clone();
            }
            x = x.clone() * x;
            k /= 2;
            }
        ans
    }
}


pub trait Ring: PartialEq + Debug + Pow + Mul<i32, Output = Self> + Zero + Add + Sub<Output = Self> +Add<Output= Self> + Neg<Output = Self>{}
impl<T: PartialEq + Debug + Pow + Mul<i32, Output = T> + Zero + Add + Sub<Output = Self> + Add<Output = T> + Neg<Output = Self>> Ring for T {}

pub trait Inv{
    fn inv(&self)->Self;
}

pub trait Card {
    fn get_card(&self)-> i32;
    fn get_char(&self)-> i32;
}



pub trait Field: Ring + Inv + Div<Output = Self> + Card{}
impl<T: Ring + Inv + Div<Output = Self> + Card > Field for T {}
