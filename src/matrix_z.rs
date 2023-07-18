use std::ops::{Mul,Add, Sub, Neg};
use crate::{traits::{One,Zero}, vector_z::VectorZ};

#[derive(Debug, PartialEq, Clone)]
pub struct MatrixZ{ //coeff should be of len n*n 
    pub coeff: Vec<i32>,
    pub n:usize
}

impl MatrixZ{                // get an element of the Matrix, a line or a column or create new Matrix
    pub fn get(&self,i:usize,j:usize) -> i32{
        if self.n < i || self.n < j{
            panic!("cannot get element at ({},{}) as matrix is size {}",i ,j ,self.n);
        }
        self.coeff[ i*self.n + j]
    }

    pub fn get_lin(&self,i:usize)->Vec<i32>{
        if self.n < i {
            panic!("cannot get line {} as matrix is size {}",i ,self.n);
        }
        let mut ans: Vec<i32> = Vec::new();
        for j in 0..self.n{
            ans.push(self.get(i, j));
        }    
        ans
    }

    pub fn get_col(&self,j:usize)->Vec<i32>{
        if self.n < j {
            panic!("cannot get column {} as matrix is size {}",j ,self.n);
        }
        let mut ans: Vec<i32> = Vec::new();
        for i in 0..self.n{
            ans.push(self.get(i, j));
        }
        ans
    }

    pub fn new(coeff:Vec<i32>, n:usize) -> MatrixZ{
        if n*n !=coeff.len() {
            panic!("coeff is not the right length, is {} and should be {}",coeff.len(),n*n);
        }
        MatrixZ{
            coeff,
            n
        }
    }

}

impl Add for MatrixZ{
    type Output = MatrixZ;

    fn add(self, rhs: MatrixZ) -> MatrixZ{
        if self.n != rhs.n {
            panic!("Matrices are not the same size {} and {}",self.n,rhs.n);
        }
        MatrixZ::new(
        self.coeff.iter()
        .zip(rhs.coeff.iter())
        .map(|(x,y)|x+y).collect(),
        self.n)
    }
}

impl Sub for MatrixZ{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul for MatrixZ{
    type Output = MatrixZ;

    fn mul(self, rhs: MatrixZ) -> MatrixZ{
        if self.n != rhs.n {
            panic!("Matrices are not the same size {} and {}",self.n,rhs.n);
        }
        let mut ans = Vec::new();
        for i in 0..self.n{
            for j in 0..self.n{

            ans.push(self.get_lin(i).iter()
            .zip(rhs.get_col(j).iter())
            .map(|(x, y)| x*y)
            .sum());
           }
        }
        MatrixZ::new(ans, self.n)
    }

    
}

impl Mul<i32> for MatrixZ{
    type Output = MatrixZ;

    fn mul(self,scalar:i32)-> MatrixZ{
        MatrixZ::new(
            self.coeff.iter().map(|x| scalar * x).collect(),
            self.n
        )
    }

}

impl Mul<VectorZ> for MatrixZ{
    type Output = VectorZ;

    fn mul(self, rhs: VectorZ) -> Self::Output {
        if self.n != rhs.n {
            panic!("Matrix is not the same size as Vector, {} and {}",self.n,rhs.n);
        }
        let mut ans = Vec::new();
        for i in 0..self.n{
            ans.push(VectorZ::new(self.get_lin(i)) * rhs.clone()) //pushes the dot product of the i-th line and the vector
        }
        VectorZ::new(ans)
    }
}


impl One for MatrixZ{
    fn one(&self)->MatrixZ{
        let mut one:MatrixZ = MatrixZ::new( vec![0; self.n * self.n], self.n);
        for i in 0..self.n{
            one.coeff[ i + i* self.n] = 1;
        }
        one
    }
}

impl Zero for MatrixZ{
    fn zero(&self)-> MatrixZ{
        MatrixZ::new( vec![0; self.n * self.n], self.n)
    }
    fn is_zero(&self)-> bool {
        for x in &self.coeff{
            if x != &0{
                return false;
            } 
        }
        true
    }
}

impl Neg for MatrixZ{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1
    }
    
}