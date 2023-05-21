use std::ops::{Mul,Add};
struct Zn{ //ring Z/nZ, if n prime we get a field
    nb: u32,
    n:u32
}

struct Matrix{ //would be nice to let coefficients be part of any ring
    coeff: Vec<u32>,
    n:usize
}

impl Add for Zn{
    type Output = Zn;

    fn add(self, other: Zn) -> Zn{
        if self.n != other.n {
            panic!("operands don't belong to the same ring Z{} and Z{}",self.n,other.n);        
        }
        Zn {
            nb: (self.nb + other.nb) % self.n,
            n: self.n
        }
    }


}

impl Mul for Zn{
    type Output = Zn;

    fn mul(self, other: Zn) -> Zn{
        if self.n != other.n {
            panic!("operands don't belong to the same ring Z{} and Z{}",self.n,other.n);
        }
        Zn {
            nb: (self.nb * other.nb) % self.n,
            n: self.n
        }

    }
}

impl Mul<u32> for Zn{
    type Output = Zn;

    fn mul(self, scalar:u32 ) -> Zn{
        Zn {
            nb: (self.nb * scalar) % self.n,
            n: self.n
        }

    }
}

impl Clone for Zn{
    fn clone(&self) -> Zn { //I don't know what i'm doing
        Zn{
            nb: self.nb,
            n:self.n
        }
    }
}

pub trait One{ //Trait to get the multiplicative identity of the ring
    fn one(&self)-> Self;
}


impl One for Zn{
    fn one(&self)->Zn{ Zn{nb:1, n: self.n}}
}

impl Zn{

    

    fn copy(&self) -> Zn { //there is probably a better way to do this
        Zn{
            nb: self.nb,
            n:self.n
        }
    }



    // fn pow(&self, n: u32) -> Zn { //using fast exponentiation
    //     let mut x = self.copy();
    //     let mut k = n;
    //     let mut ans = Zn::one(self.n);
    //     while k > 0{
    //         if k % 2 == 1{
    //             ans = &ans * &x;
    //         }

    //         x = &x * &x;
    //         k = k/2;
    //     }
    //     ans
    // }

    // fn inv(&self) -> Zn{  //p must be prime for this to work
    //     self.pow(self.n-2)
    // }
}


impl Matrix{                // get an element of the Matrix, a line or a column or create new Matrix
    fn get(&self,i:usize,j:usize) -> u32{
        self.coeff[ i*self.n + j]
    }

    fn get_lin(&self,i:usize)->Vec<u32>{
        if self.n < i {
            panic!("cannot get line {} as matrix is size {}",i ,self.n);
        }
        let mut ans: Vec<u32> = Vec::new();
        for k in 0..self.n{
            ans.push(self.coeff[i*self.n + k]);
        }    
        ans
    }

    fn get_col(&self,j:usize)->Vec<u32>{
        if self.n < j {
            panic!("cannot get column {} as matrix is size {}",j ,self.n);
        }
        let mut ans: Vec<u32> = Vec::new();
        for k in 0..self.n{
            ans.push(self.coeff[k*self.n + j]);
        }
        ans
    }

    fn new(coeff:Vec<u32>, n:usize) -> Matrix{
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


impl Mul<u32> for Matrix{
    type Output = Matrix;

    fn mul(self,scalar:u32)-> Matrix{
        Matrix::new(
            self.coeff.iter().map(|x| scalar * x).collect(),
            self.n
        )
    }

}


impl Clone for Matrix{
    fn clone(&self) -> Matrix{
        Matrix{
            coeff: self.coeff.clone(),
            n: self.n
        }
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

pub fn pow<T:Clone + Mul + One + Mul<Output = T>>(base:T, n: u32) -> T { //using fast exponentiation
    let mut x = base.clone();
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




fn main(){
    let a = Zn{nb:4,n:7};
    // let b = Zn{nb:5,n:7};
    // let c = &a + &b;
    // let d = &a * &b;
    let e:Zn = pow(a.clone(),3);
    // let f = a.inv();
    let k = 3;
    // println!("{}+{} is {}", a.nb, b.nb, c.nb);
    // println!("{}x{} is {}", a.nb, b.nb, d.nb);
    println!("{}^{} is {}", a.nb, k, e.nb);
    // println!("{}^-1 is {}", a.nb, f.nb);

    let A = Matrix::new(vec![2,4,6,3],2);
    let B = Matrix::new(vec![1,5,8,1],2);
    let col1 = A.get_col(1);
    let lin0 = A.get_lin(0);
    let C = A.clone() + B.clone();
    let D = A.clone() * B.clone();
    let E = pow(A.clone(),4);
    let F = A.clone() * 5;
    println!("the second column of A is {:?}",col1);
    println!("the first line of A is {:?}",lin0);
    println!("A+B is {:?}",C.coeff);
    println!("A*B is {:?}",D.coeff);
    println!("A^4 is {:?}",E.coeff);
    println!("A*5 is {:?}",F.coeff)

}


    

    
    