use std::ops::{Mul,Add, Sub, Neg, Div};
use crate::{traits::{One, Zero, Pow, Inv, Card}, field_extension::FieldExtension};

#[derive(Debug, PartialEq, Clone)]
pub struct Zn{ //ring Z/nZ, if n prime we get a field
    pub nb: i32,
    pub n:i32
}

impl Add for Zn{
    type Output = Zn;

    fn add(self, rhs: Zn) -> Zn{
        if self.n != rhs.n {
            panic!("operands don't belong to the same ring Z{} and Z{}",self.n,rhs.n);        
        }
        Zn {
            nb: (self.nb + rhs.nb) % self.n,
            n: self.n
        }
    }
}

impl Mul for Zn{
    type Output = Zn;

    fn mul(self, rhs: Zn) -> Zn{
        if self.n != rhs.n {
            panic!("operands don't belong to the same ring Z{} and Z{}",self.n,rhs.n);
        }
        Zn {
            nb: (self.nb * rhs.nb) % self.n,
            n: self.n
        }
    }
}

impl Mul<i32> for Zn{
    type Output = Zn;

    fn mul(self, scalar:i32 ) -> Zn{
        let mut nb = (self.nb * scalar) % self.n;
        if scalar < 0{
            nb += self.n;
        }

        Zn {
            nb,
            n: self.n
        }
    }
}

impl One for Zn{
    fn one(&self)->Zn{ Zn{nb:1, n: self.n}}
}

impl Zero for Zn{
    fn zero(&self)->Zn{ Zn{nb:0, n: self.n}}
    fn is_zero(&self)-> bool {
        self.nb == 0
    }
}

impl Neg for Zn{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1
    }
    
}

impl Sub for Zn{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Inv for Zn{ //Extended euclidean algorithm from wikipedia.org/wiki/Extended_Euclidean_algorithm
    fn inv(&self)->Zn{
        
        let mut t = 0;
        let mut newt = 1;
        let mut r = self.n;    
        let mut newr = self.nb;
    
        while newr != 0 {
            let quotient = r / newr;

            let tmp = t;
            t = newt;
            newt = tmp - (quotient * newt);

            let tmp = r;
            r = newr;
            newr = tmp - (quotient * newr);
        }
    
        if t < 0 {
            t += self.n;
        }
        
        Zn::new(t, self.n)
        
    }
}

impl Div for Zn{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl Card for Zn {
    fn get_card(&self)-> i32 {
        self.n
    }
    fn get_char(&self)-> i32 {
        self.n
    }
}

impl Zn {

    pub fn from_vec(vec: Vec<i32>,n:i32)-> Vec<Zn>{ // get a vec of Zn from a vec<i32> and n
        let mut ans:Vec<Zn> = Vec::new();
        for i in  vec{
            ans.push(Zn::new(i, n));
        }
        ans
        
    }

    pub fn into_i32(vec: Vec<Zn>)-> Vec<i32> {  // get a vec of int from a vec<Zn>
        let mut ans:Vec<i32> = Vec::new();
        for i in  vec{
            ans.push(i.nb);
        }
        ans
    }

    pub fn new(x:i32,n:i32)-> Zn{
        let mut nb = x % n;
        if nb < 0{
            nb += n;
        }

        Zn {
            nb,
            n
        }
    }

    pub fn legendre_symbol(&self)->i32{
        let ls = self.pow( ((self.n - 1)/ 2).try_into().unwrap() ).nb;
        if ls > 1 {
            return ls - self.n;
        }
        ls
    }

    pub fn is_square(&self) -> bool{
        self.legendre_symbol() >= 0
    }

    pub fn sqrt(a:Zn)->(Zn,Zn){  //using cipolla's algorithm
        if !a.is_square(){
            panic!("{} is not a square mod {}",a.nb,a.n);
        }

        let mut t = Zn::new(2, a.n);

        while (t.pow(2) - a.clone()).is_square() {
            t.nb += 1;
        }


        let x: FieldExtension<Zn> = FieldExtension::new_from_vec(vec![t.clone() ,t.one()], vec![a.clone() - t.pow(2), t.zero(), t.one()]);

        let ans: FieldExtension<Zn> = x.pow(((a.n + 1)/ 2).try_into().unwrap());

        let y = ans.nb.coeff[0].clone();

        (y.clone(),-y)


    }


}


