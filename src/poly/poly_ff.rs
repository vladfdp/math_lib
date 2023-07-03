use crate::traits::Field;
use std::ops::{Mul,Add, Sub,Rem, Neg, Div};
use crate::traits::{One,Zero,Inv};
use std::convert::TryInto;

#[derive(Debug, PartialEq, Clone)]
pub struct Polyff<T:Field>{ //Polynomial over a field
    pub coeff: Vec<T>
}

impl<T:Field> Polyff<T>{
    pub fn get_deg(&self)->usize{self.coeff.len()-1}

    pub fn eval(&self,x:T)-> T {
        let mut ans:T = x.zero();
        for i in 0..self.coeff.len(){
            ans = ans + (x.pow(i.try_into().unwrap()) * self.coeff[i].clone());
        }
        ans


    }

    pub fn rm_trailing_zeros(mut self)-> Polyff<T> {
        while self.coeff.last().unwrap().is_zero() && self.coeff.len() > 1{
            self.coeff.pop();
        }
        self
    }

    pub fn times_x_to_the(&self,n:usize)->Polyff<T>{
        let mut ans = self.coeff.clone();
        let mut zeros = vec![self.coeff[0].zero();n];
        zeros.append(&mut ans);
        Polyff{ coeff:zeros}
    }

    pub fn new_from_vec(coeff:Vec<T>) -> Polyff<T>{
        Polyff{
            coeff
        }.rm_trailing_zeros()
    }
}

impl<T:Field> Add for Polyff<T>{
    type Output = Polyff<T>;

    fn add(self, rhs:Self)-> Self{

        let mut a = self.coeff;
        let mut b = rhs.coeff;

        while a.len()>b.len() { //we make a and b the same size
            b.push(b[0].zero());
        }
        while a.len()<b.len() {
            a.push(a[0].zero());
        }

        Polyff{ coeff: 
            a.iter()
            .zip(b.iter())
            .map(|(x,y)|x.clone() + y.clone()).collect()
        }.rm_trailing_zeros()
    }
}

impl<T:Field> Sub for Polyff<T>{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<T:Field> Mul for Polyff<T> {
    type Output = Polyff<T>;
    fn mul(self, rhs: Polyff<T>) -> Polyff<T> {
        let mut ans = vec![ self.coeff[0].zero() ;self.coeff.len()+rhs.coeff.len()-1];

        for i in 0..self.coeff.len(){
            for j in 0..rhs.coeff.len(){
                ans[i+j] = ans[i+j].clone()
                + self.coeff[i].clone() * rhs.coeff[j].clone();   
            }
        }
        Polyff{ coeff: ans }
    }
    
}

impl<T:Field> Mul<T> for Polyff<T> {
    type Output = Polyff<T>;
    fn mul(self, rhs: T) -> Polyff<T> {
        Polyff{ coeff: self.coeff.iter()
            .map(|x| x.clone() * rhs.clone())
            .collect() }
    }
    
}

impl<T:Field> Mul<i32> for Polyff<T> {
    type Output = Polyff<T>;
    fn mul(self, rhs: i32) -> Polyff<T> {
        Polyff{ coeff: self.coeff.iter()
            .map(|x| x.clone() * rhs)
            .collect() }
    }
    
}

impl<T:Field> Zero for Polyff<T>  {
    fn zero(&self)->Polyff<T>{
        Polyff { coeff: vec![self.coeff[0].zero()]}
    }

    fn is_zero(&self)-> bool {
        self.coeff.len() == 1 && self.coeff[0].is_zero()
    }
}

impl<T:Field> One for Polyff<T>  {
    fn one(&self)->Polyff<T>{
        Polyff { coeff: vec![self.coeff[0].one()]}
    }
}

impl<T:Field> Div for Polyff<T>{
    type Output = Polyff<T>;

    fn div(self, rhs: Self) -> Self::Output {

        if rhs.get_deg() > self.get_deg(){
            return self.zero();
        }
        if rhs.is_zero(){
            panic!("Can't divide by zero");
        }
        
        let mut ans = vec![self.coeff[0].zero(); self.get_deg() - rhs.get_deg() + 1];
        let mut a = self;
        let b = rhs;
        let pivot = b.coeff.last().unwrap().inv();

        
        while a.get_deg() >= b.get_deg() && !a.is_zero(){

            let x = pivot.clone() * a.coeff.last().unwrap().clone();
            ans[a.get_deg() - b.get_deg()] = x.clone();
            a = a.clone() - (b.times_x_to_the(a.get_deg() - b.get_deg()) * x );
            a = a.rm_trailing_zeros();

        }
        Polyff{
            coeff:ans
        }
    }
    
}

impl<T:Field> Rem for Polyff<T>{
    type Output = Polyff<T>;

    fn rem(self, rhs: Self) -> Self::Output {

        if rhs.get_deg() > self.get_deg(){
            return self;
        }
        if rhs.is_zero(){
            panic!("Can't divide by zero");
        }
        
        let mut a = self;
        let b = rhs;
        let pivot = b.coeff.last().unwrap().inv();
        
        while a.get_deg() >= b.get_deg() && !a.is_zero(){
            a = a.clone() - (b.times_x_to_the(a.get_deg() - b.get_deg()) * (pivot.clone() * a.coeff.last().unwrap().clone()) );
            a = a.rm_trailing_zeros();
        }
        a
    }
    
}

impl<T:Field> Neg for Polyff<T>{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1
    }
    
}