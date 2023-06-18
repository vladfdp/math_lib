use crate::traits::{Field, Pow};
use std::ops::{Mul,Add, Sub, Neg};
use crate::poly::poly_ff::Polyff;
use crate::traits::{One, Zero, Inv, Card};

#[derive(Debug, PartialEq, Clone)]
pub struct FieldExtension<T:Field>{
    pub nb: Polyff<T>,
    pub poly: Polyff<T>
}

impl<T:Field> Add for FieldExtension<T>{
    type Output = FieldExtension<T>;

    fn add(self, rhs: FieldExtension<T>) -> FieldExtension<T>{
        if self.poly != rhs.poly {
            panic!("operands don't belong to the same Field");        
        }
        FieldExtension {
            nb: (self.nb + rhs.nb) % self.poly.clone(),
            poly: self.poly
        }
    }
}

impl<T:Field> Sub for FieldExtension<T>{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<T:Field> Mul for FieldExtension<T>{
    type Output = FieldExtension<T>;

    fn mul(self, rhs: FieldExtension<T>) -> FieldExtension<T>{
        if self.poly != rhs.poly {
            panic!("operands don't belong to the same Field");
        }
        FieldExtension{
            nb: (self.nb * rhs.nb) % self.poly.clone(),
            poly: self.poly
        }
    }
}

impl<T:Field> Mul<i32> for FieldExtension<T>{
    type Output = FieldExtension<T>;

    fn mul(self, rhs: i32) -> FieldExtension<T>{
        FieldExtension{
            nb: self.nb * rhs,
            poly: self.poly
        }
    }
}

impl<T:Field> Neg for FieldExtension<T>{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1
    }
    
}

impl<T:Field> One for FieldExtension<T>{
    fn one(&self)->FieldExtension<T>{
        FieldExtension{nb:self.nb.one(), 
        poly: self.poly.clone()}
    }
}

impl<T:Field> Zero for FieldExtension<T>{
    fn zero(&self)->FieldExtension<T>{ 
        FieldExtension{nb:self.nb.zero(), 
            poly: self.poly.clone()}
    }
    fn is_zero(&self)-> bool {
        self.nb.is_zero()
    }
}

impl<T:Field> Inv for FieldExtension<T>{
    fn inv(&self)->FieldExtension<T>{
        self.pow((self.get_card()-2).try_into().unwrap())
    }
}

impl<T:Field> Card for FieldExtension<T> {
    fn get_card(&self)-> usize {
        self.nb.coeff[0].get_card().pow(self.poly.get_deg().try_into().unwrap())
    }
}

impl<T:Field> FieldExtension<T>  {
    pub fn new_from_vec(nb:Vec<T>, poly:Vec<T>)->FieldExtension<T>{
        FieldExtension { nb: Polyff { coeff: nb } %  Polyff { coeff: poly.clone() } , 
        poly: Polyff { coeff: poly } }
    }


}