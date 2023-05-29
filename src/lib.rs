mod zn;
mod matrix;
mod traits;




#[cfg(test)]
mod tests {

    //use std::convert::TryInto;
    //use std::ops::{Add,Mul};
    use crate::zn::Zn;
    use crate::matrix::Matrix;
    use crate::traits::Pow;

    #[test]
    fn zn_add_same_ring() {
        let a = Zn { nb: 5, n: 7 };
        let b = Zn { nb: 4, n: 7 };
        assert_eq!(a + b, Zn { nb: 2, n: 7 });

        let a = Zn { nb: 5, n: 7 };
        let b = Zn { nb: 0, n: 7 };
        assert_eq!(a + b, Zn { nb: 5, n: 7 });

        let a = Zn { nb: 10_000, n: 15_000 };
        let b = Zn { nb: 7_000, n: 15_000 };
        assert_eq!(a + b, Zn { nb: 2_000, n: 15_000 });
    }

    #[test]
    #[should_panic]
    fn zn_add_different_ring() {
        let a = Zn { nb: 3, n: 5 };
        let b = Zn { nb: 2, n: 7 };
        let _result = a + b;
    }

    #[test]
    fn zn_mul_same_ring() {
        let a = Zn { nb: 5, n: 7 };
        let b = Zn { nb: 4, n: 7 };
        assert_eq!(a * b, Zn { nb: 6, n: 7 });

        let a = Zn { nb: 5, n: 7 };
        let b = Zn { nb: 0, n: 7 };
        assert_eq!(a * b, Zn { nb: 0, n: 7 });

        let a = Zn { nb: 10_000, n: 15_000 };
        let b = Zn { nb: 7_000, n: 15_000 };
        assert_eq!(a * b, Zn { nb: 10_000, n: 15_000 });
    }

    #[test]
    #[should_panic]
    fn zn_mul_different_ring() {
        let a = Zn { nb: 3, n: 5 };
        let b = Zn { nb: 2, n: 7 };
        let _result = a * b;
    }

    #[test]
    fn zn_pow() {
       let a = Zn { nb: 5, n: 7 };
       assert_eq!(a.pow(3), Zn { nb: 6, n: 7 });

       let b = Zn { nb: 7, n: 11 };
       assert_eq!(b.pow(2), Zn { nb: 5, n: 11 });

       let c = Zn { nb: 3, n: 21 };
       assert_eq!(c.pow(3), Zn { nb: 6, n: 21 });
    }


    #[test]
    #[should_panic]
    fn matrix_new_invalid() {
        let _matrix_a = Matrix::new(vec![5, 6, 7, 8, 9, 10], 3);
    }

    #[test]
    fn matrix_add_same_size() {
        let matrix_a = Matrix::new(vec![1, 2, 3, 4], 2);
        let matrix_b = Matrix::new(vec![5, 6, 7, 8], 2);
        let expected_result = Matrix::new(vec![6, 8, 10, 12], 2);

        assert_eq!(matrix_a + matrix_b, expected_result);

        let matrix_a = Matrix::new(vec![1, 4, 3, 4, 5, 2, 9, 6, 3], 3);
        let matrix_b = Matrix::new(vec![2, 6, 1, 8, 5, 8, 2, 1, 0], 3);
        let expected_result = Matrix::new(vec![3, 10, 4, 12, 10, 10, 11, 7, 3], 3);

        assert_eq!(matrix_a + matrix_b, expected_result);
    }

    #[test]
    #[should_panic]
    fn matrix_add_diff_size() {

        let matrix_a = Matrix::new(vec![1, 2, 3, 4], 2);
        let matrix_b = Matrix::new(vec![5, 6, 7, 8, 9, 10, 11, 12, 13], 3);
        let _ = matrix_a + matrix_b;
    }

    #[test]
    fn matrix_mul_same_size() {
        let matrix_a = Matrix::new(vec![1, 2, 3, 4], 2);
        let matrix_b = Matrix::new(vec![5, 6, 7, 8], 2);
        let expected_result = Matrix::new(vec![19, 22, 43, 50], 2);

        assert_eq!(matrix_a * matrix_b, expected_result);

        let matrix_a = Matrix::new(vec![1, 4, 3, 4, 5, 2, 9, 6, 3], 3);
        let matrix_b = Matrix::new(vec![2, 6, 1, 8, 5, 8, 2, 1, 0], 3);
        let expected_result = Matrix::new(vec![40, 29, 33, 52, 51, 44, 72, 87, 57], 3);
        

        assert_eq!(matrix_a * matrix_b, expected_result);
    }

    #[test]
    #[should_panic]
    fn matrix_mul_diff_size() {

        let matrix_a = Matrix::new(vec![1, 2, 3, 4], 2);
        let matrix_b = Matrix::new(vec![5, 6, 7, 8, 9, 10, 11, 12, 13], 3);
        let _ = matrix_a * matrix_b;
    }

    #[test]
    fn matrix_pow() {
        let matrix_a = Matrix::new(vec![1, 2, 3, 4], 2);
        let matrix_a_pow_4 = Matrix::new(vec![199, 290, 435, 634], 2);
        assert_eq!(matrix_a.pow(4), matrix_a_pow_4);

        let matrix_b = Matrix::new(vec![2, 6, 1, 8, 5, 8, 2, 1, 0], 3);
        let matrix_b_pow_3 = Matrix::new(vec![552, 589, 398, 888, 885, 720, 180, 167, 148], 3);
        assert_eq!(matrix_b.pow(3), matrix_b_pow_3);
    }



    // pub fn pow<T:Foo >(base:T, n: u32) -> T { //using fast exponentiation
    //     let mut x = base.clone();
    //     let mut k = n;
    //     let mut ans = x.one();
    //     while k > 0{
    //         if k % 2 == 1{
    //             ans = ans * x.clone();
    //         }
    //         x = x.clone() * x;
    //         k = k/2;
    //         }
    //     ans
    // }
}
