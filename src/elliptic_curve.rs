use crate::{traits::{Field, Zero}, poly::poly_ff::Polyff};
use std::ops::{Add, Sub, Mul, Neg};

#[derive(Debug, PartialEq, Clone)]
pub struct EllipticCurve<T:Field>{ // implements elliptic curve in short weierstrass form
    a:T,
    b:T
}

#[derive(Debug, PartialEq, Clone)]
pub struct EllipticCurvePoint<T:Field>{
    pt:ECPoint<T>,
    curve: EllipticCurve<T>
}

#[derive(Debug, PartialEq, Clone)]
pub enum ECPoint<T>{
    Point{x:T,y:T},
    Infinity
}

impl<T:Field> EllipticCurve<T>{

    pub fn new(a:T,b:T)->EllipticCurve<T>{
        if (a.pow(3) * 4) + (b.pow(2) * 27) == a.zero(){
            panic!("Elliptic curve is singular");
        }
        EllipticCurve{
            a,
            b
        }
    }

    pub fn new_point(&self,x:T,y:T)->EllipticCurvePoint<T>{

        let pt = ECPoint::Point{x, y};
        if !self.is_on_curve(&pt) {
            panic!("Point is not on curve");
        }
        EllipticCurvePoint{
            pt,
            curve: self.clone()
        }
    }

    pub fn infinity(&self)->EllipticCurvePoint<T>{
        EllipticCurvePoint{
            pt:ECPoint::Infinity,
            curve:self.clone()
        }
    }

    pub fn is_on_curve(&self,pt:&ECPoint<T>)->bool{

        let poly = Polyff{coeff:vec![self.b.clone(),self.a.clone(),self.a.zero(),self.a.one()]};
        match pt {
            ECPoint::Infinity => true,
            ECPoint::Point{x,y} => y.pow(2) == poly.eval(x.clone())
        }

        

    }

}




impl<T:Field> Zero for EllipticCurvePoint<T>{
    fn zero(&self)->EllipticCurvePoint<T>{
        EllipticCurvePoint{
            pt:ECPoint::Infinity,
            curve: self.curve.clone()
        }
    }

    fn is_zero(&self)-> bool{
        self.pt == ECPoint::Infinity
    }
}

impl<T:Field> Neg for EllipticCurvePoint<T>{
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.pt {
            ECPoint::Infinity => EllipticCurvePoint { pt:  ECPoint::Infinity, curve: self.curve },
            ECPoint::Point{x ,y} => EllipticCurvePoint { pt:  ECPoint::Point{x , y: -y }, curve: self.curve }
        }
    }
    
}


impl<T:Field> Add for EllipticCurvePoint<T>{
    type Output = EllipticCurvePoint<T>;

    fn add(self, rhs:EllipticCurvePoint<T>) -> EllipticCurvePoint<T>{
        if self.curve != rhs.curve{
            panic!("points are not on the same curve");
        }
        match (self.pt.clone(),rhs.pt.clone()){
           (ECPoint::Infinity, _) => rhs,
            (_, ECPoint:: Infinity) => self,

            (ECPoint::Point{x: x1, y: y1}, ECPoint::Point{x: x2, y: y2}) => 
            EllipticCurvePoint{
                pt: ECPoint::add_ec_point(x1,y1,x2,y2,self.curve.a.clone()),
                curve: self.curve
            } 
        }
    }
}

impl<T:Field> Sub for EllipticCurvePoint<T>{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<T:Field> Mul<u32> for EllipticCurvePoint<T>{
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        let mut x = self.clone();
        let mut k = rhs;
        let mut ans = x.zero();
        while k > 0{
            if k % 2 == 1{
                ans = ans + x.clone();
            }
            x = x.clone() + x;
            k /= 2;
            }
        ans
    }
}

impl<T:Field> ECPoint<T>{

    pub fn add_ec_point(x1:T,y1:T,x2:T,y2:T,a:T) -> ECPoint<T> {
        if x1 == x2{
            if y1 == y2 {
                if y1 == y1.zero(){
                    return ECPoint::Infinity;
                }
                let m:T = (x1.clone().pow(2) * 3 + a ) / (y1.clone() * 2);
                let x = m.clone().pow(2) - x1.clone() * 2;
                let y = m * (x1 - x.clone()) - y1;
                return ECPoint::Point{ x, y};
            }
            return ECPoint::Infinity;
        }
        let m = (y1.clone() - y2) / (x1.clone() - x2.clone());
        let x = m.clone().pow(2) - x1.clone() - x2;
        let y = m * (x1 - x.clone()) - y1;
        ECPoint::Point{ x, y}
            
    }

}