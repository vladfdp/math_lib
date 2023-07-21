use crate::zn::Zn;
use crate::poly::poly_ff::Polyff;
use crate::traits::Pow;

pub fn is_prime(n:i32)->bool{ //this is slow, will implement miller-rabin or something


    if n < 2 { return false}

    let lhs = Polyff::new_from_vec(Zn::from_vec(vec![1,1],n)).pow(n.try_into().unwrap()); //(x + 1)^n

    let mut vec = vec![1];
    let mut zeros = vec![0;(n-1).try_into().unwrap()];
    vec.append(&mut zeros);
    vec.push(1);

    let rhs = Polyff::new_from_vec(Zn::from_vec(vec,n)); //x^n + 1

    lhs == rhs

}

pub fn gcd(a:i32,b:i32)->i32{
    if b == 0{
        a
    }else{
        gcd(b, a % b)
    }
}