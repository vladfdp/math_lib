use std::ops::{Mul,Add, Sub, Neg};
use crate::traits::{One,Zero};
use crate::traits::Ring;
use crate::vector_ring::Vector;

#[derive(Debug, PartialEq, Clone)]
pub struct MatrixRing<T:Ring>{ //coefficients are part of any ring
    coeff: Vec<T>,
    n:usize
}

impl<T:Ring> MatrixRing<T>{    // get an element of the Matrix, a line or a column or create new Matrix
    pub fn get(&self,i:usize,j:usize) -> T{
        self.coeff[ i*self.n + j].clone()
    }

    pub fn get_lin(&self,i:usize)->Vec<T>{
        if self.n < i {
            panic!("cannot get line {} as matrix is size {}",i ,self.n);
        }
        let mut ans: Vec<T> = Vec::new();
        for j in 0..self.n{
            ans.push(self.get(i, j));
        }    
        ans
    }

    pub fn get_col(&self,j:usize)->Vec<T>{
        if self.n < j {
            panic!("cannot get column {} as matrix is size {}",j ,self.n);
        }
        let mut ans: Vec<T> = Vec::new();
        for i in 0..self.n{
            ans.push(self.get(i, j));
        }
        ans
    }

    pub fn new(coeff:Vec<T>, n:usize) -> MatrixRing<T>{
        if n*n !=coeff.len() {
            panic!("coeff is not the right length, is {} and should be {}",coeff.len(), n*n);
        }
        MatrixRing{
            coeff,
            n
        }
    }

}

impl<T:Ring> Add for MatrixRing<T>{
    type Output = MatrixRing<T>;

    fn add(self, rhs: MatrixRing<T>) -> MatrixRing<T>{
        if self.n != rhs.n {
            panic!("Matrices are not the same size {} and {}",self.n,rhs.n);
        }
        MatrixRing::new(
        self.coeff.iter()
        .zip(rhs.coeff.iter())
        .map(|(x,y)|x.clone() + y.clone() ).collect(),
        self.n)
    }
}

impl<T:Ring> Sub for MatrixRing<T>{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<T:Ring> Mul for MatrixRing<T>{
    type Output = MatrixRing<T>;

    fn mul(self, rhs: MatrixRing<T>) -> MatrixRing<T>{
        if self.n != rhs.n {
            panic!("Matrices are not the same size {} and {}",self.n,rhs.n);
        }
        let mut ans = Vec::new();
        for i in 0..self.n{
            for j in 0..self.n{
                let mut sum = self.coeff[0].zero();
                for (x,y) in self.get_lin(i).iter()
                .zip(rhs.get_col(j).iter()){
                    sum = sum + (x.clone() * y.clone());
                }

            ans.push(sum);
           }
        }
        MatrixRing::new(ans, self.n)
    }

    
}

impl<T:Ring> Mul<Vector<T>> for MatrixRing<T>{
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        if self.n != rhs.n {
            panic!("Matrix is not the same size as Vector, {} and {}",self.n,rhs.n);
        }
        let mut ans = Vec::new();
        for i in 0..self.n{
            ans.push(Vector::new(self.get_lin(i)) * rhs.clone()) //pushes the dot product of the i-th line and the vector
        }
        Vector::new(ans)
    }
}

impl<T:Ring> Mul<i32> for MatrixRing<T>{
    type Output = MatrixRing<T>;

    fn mul(self,scalar:i32)-> MatrixRing<T>{
        MatrixRing::new(
            self.coeff.iter().map(|x| x.clone() * scalar).collect(),
            self.n
        )
    }

}

impl<T:Ring> Mul<T> for MatrixRing<T>{
    type Output = MatrixRing<T>;

    fn mul(self,scalar:T)-> MatrixRing<T>{
        MatrixRing::new(
            self.coeff.iter().map(|x| scalar.clone() * x.clone()).collect(),
            self.n
        )
    }

}


impl<T:Ring> One for MatrixRing<T>{
    fn one(&self)->MatrixRing<T>{
        let mut one = MatrixRing::new( vec![self.coeff[0].zero(); self.n * self.n], self.n);
        for i in 0..self.n{
            one.coeff[ i + i* self.n] = self.coeff[0].one();
        }
        one
    }
}

impl<T:Ring> Zero for MatrixRing<T>{
    fn zero(&self)-> MatrixRing<T>{
        MatrixRing::new( vec![self.coeff[0].zero(); self.n * self.n], self.n)
    }
    fn is_zero(&self)-> bool {
        for x in &self.coeff{
            if !x.is_zero(){
                return false;
            } 
        }
        true
    }
}

impl<T:Ring> Neg for MatrixRing<T>{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1
    }
    
}