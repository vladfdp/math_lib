
struct Zp{ //ring Z/pZ, if p prime we get a field
    nb: u32,
    p:u32
}

struct Matrix{ //would be nice to let coefficients be part of any ring
    coeff: Vec<u32>,
    size:usize
}

impl std::ops::Add for &Zp{
    type Output = Zp;

    fn add(self, other: &Zp) -> Zp{
        if self.p != other.p {
            panic!("operands don't belong to the same field F{} and F{}",self.p,other.p);        
        }
        Zp {
            nb: (self.nb + other.nb) % self.p,
            p: self.p
        }
    }


}

impl std::ops::Mul for &Zp{
    type Output = Zp;

    fn mul(self, other: &Zp) -> Zp{
        if self.p != other.p {
            panic!("operands don't belong to the same field F{} and F{}",self.p,other.p);
        }
        Zp {
            nb: (self.nb * other.nb) % self.p,
            p: self.p
        }

    }
}


impl Zp{

    fn copy(&self) -> Zp { //there is probably a better way to do this
        Zp{
            nb: self.nb,
            p:self.p
        }
    }



    fn pow(&self, n: u32) -> Zp { //using fast exponentiation
        let mut x = self.copy();
        let mut k = n;
        let mut ans = Zp{nb:1, p: self.p};
        while k > 0{
            if k % 2 == 1{
                ans = &ans * &x;
            }

            x = &x * &x;
            k = k/2;
        }
        ans
    }

    fn inv(&self) -> Zp{  //p must be prime for this to work
        self.pow(self.p-2)
    }
}

impl Matrix{                // get an element of the Matrix, a line or a column or create new Matrix
    fn get(&self,i:usize,j:usize) -> u32{
        self.coeff[ i*self.size + j]
    }

    fn get_lin(&self,i:usize)->Vec<u32>{
        if self.size < i {
            panic!("cannot get line {} as matrix is size {}",i ,self.size);
        }
        let mut ans: Vec<u32> = Vec::new();
        for k in 0..self.size{
            ans.push(self.coeff[i*self.size + k]);
        }    
        ans
    }

    fn get_col(&self,j:usize)->Vec<u32>{
        if self.size < j {
            panic!("cannot get column {} as matrix is size {}",j ,self.size);
        }
        let mut ans: Vec<u32> = Vec::new();
        for k in 0..self.size{
            ans.push(self.coeff[k*self.size + j]);
        }
        ans
    }

    fn new(coeff:Vec<u32>, size:usize) -> Matrix{
        if size*size !=coeff.len() {
            panic!("coeff is not the right length, is {} and should be {}",coeff.len(),size*size);
        }
        Matrix{
            coeff,
            size
        }
    }

}

impl std::ops::Add for &Matrix{
    type Output = Matrix;

    fn add(self, other: &Matrix) -> Matrix{
        if self.size != other.size {
            panic!("Matrices are not the same size {} and {}",self.size,other.size);
        }
        let mut ans = Vec::new();
        for i in 0..(self.size*self.size){
            ans.push(self.coeff[i] + other.coeff[i])
        }
        Matrix::new(ans, self.size)

    }

}

impl std::ops::Mul for &Matrix{
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Matrix{
        if self.size != other.size {
            panic!("Matrices are not the same size {} and {}",self.size,other.size);
        }
        let mut ans = Vec::new();
        for i in 0..self.size{
            for j in 0..self.size{

            ans.push(self.get_lin(i).iter()
            .zip(other.get_col(j).iter())
            .map(|(x, y)| x*y)
            .sum());
           }
        }
        Matrix::new(ans, self.size)
    }
}

fn main(){
    // let a = Zp{nb:4,p:7};
    // let b = Zp{nb:5,p:7};
    // let c = &a + &b;
    // let d = &a * &b;
    // let e = a.pow(3);
    // let f = a.inv();
    // let k = 3;
    // println!("{}+{} is {}", a.nb, b.nb, c.nb);
    // println!("{}x{} is {}", a.nb, b.nb, d.nb);
    // println!("{}^{} is {}", a.nb, k, e.nb);
    // println!("{}^-1 is {}", a.nb, f.nb);

    let A = Matrix::new(vec![2,4,6,3],2);
    let B = Matrix::new(vec![1,5,8,1],2);
    let col1 = A.get_col(1);
    let lin0 = A.get_lin(0);
    let C = &A + &B;
    let D = &A * &B;
    println!("the second column of A is {:?}",col1);
    println!("the first line of A is {:?}",lin0);
    println!("A+B is {:?}",C.coeff);
    println!("A*B is {:?}",D.coeff);

}


    

    
    