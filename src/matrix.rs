use std::ops::{Mul,Add};
use crate::traits::{One,Zero};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix{ //would be nice to let coefficients be part of any ring
    coeff: Vec<i32>,
    n:usize
}

impl Matrix{                // get an element of the Matrix, a line or a column or create new Matrix
    pub fn get(&self,i:usize,j:usize) -> i32{
        self.coeff[ i*self.n + j]
    }

    pub fn get_lin(&self,i:usize)->Vec<i32>{
        if self.n < i {
            panic!("cannot get line {} as matrix is size {}",i ,self.n);
        }
        let mut ans: Vec<i32> = Vec::new();
        for k in 0..self.n{
            ans.push(self.coeff[i*self.n + k]);
        }    
        ans
    }

    pub fn get_col(&self,j:usize)->Vec<i32>{
        if self.n < j {
            panic!("cannot get column {} as matrix is size {}",j ,self.n);
        }
        let mut ans: Vec<i32> = Vec::new();
        for k in 0..self.n{
            ans.push(self.coeff[k*self.n + j]);
        }
        ans
    }

    pub fn new(coeff:Vec<i32>, n:usize) -> Matrix{
        if n*n !=coeff.len() {
            panic!("coeff is not the right length, is {} and should be {}",coeff.len(),n*n);
        }
        Matrix{
            coeff,
            n
        }
    }

}

impl Add for Matrix{
    type Output = Matrix;

    fn add(self, other: Matrix) -> Matrix{
        if self.n != other.n {
            panic!("Matrices are not the same size {} and {}",self.n,other.n);
        }
        Matrix::new(
        self.coeff.iter()
        .zip(other.coeff.iter())
        .map(|(x,y)|x+y).collect(),
        self.n)
    }
}

impl Mul for Matrix{
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix{
        if self.n != other.n {
            panic!("Matrices are not the same size {} and {}",self.n,other.n);
        }
        let mut ans = Vec::new();
        for i in 0..self.n{
            for j in 0..self.n{

            ans.push(self.get_lin(i).iter()
            .zip(other.get_col(j).iter())
            .map(|(x, y)| x*y)
            .sum());
           }
        }
        Matrix::new(ans, self.n)
    }

    
}

impl Mul<i32> for Matrix{
    type Output = Matrix;

    fn mul(self,scalar:i32)-> Matrix{
        Matrix::new(
            self.coeff.iter().map(|x| scalar * x).collect(),
            self.n
        )
    }

}


impl One for Matrix{
    fn one(&self)->Matrix{
        let mut one:Matrix = Matrix::new( vec![0; self.n * self.n], self.n);
        for i in 0..self.n{
            one.coeff[ i + i* self.n] = 1;
        }
        one
    }
}

impl Zero for Matrix{
    fn zero(&self)-> Matrix{
        Matrix::new( vec![0; self.n * self.n], self.n)
    }
}

