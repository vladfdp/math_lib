mod zn;
mod matrix;
mod traits;
mod poly;




#[cfg(test)]
mod tests {


    //use std::convert::TryInto;
    //use std::ops::{Add,Mul};


    use crate::traits::Pow;
    use crate::poly::poly_ring::Poly;
    use crate::zn::Zn;
    use crate::matrix::Matrix;


    mod zn_tests{

        use crate::zn::Zn;
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
    }

    mod matrix_test{

        use crate::matrix::Matrix;
        use crate::traits::Pow;

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
    }
    
    mod poly_zx_test{


        use crate::{poly::poly_zx::PolyZx , matrix::Matrix, zn::Zn};


        #[test]
    fn poly_rm_zeros() {
        let poly = PolyZx{coeff:vec![0,0,3,2,5,0,0,0]}.rm_trailing_zeros();
        assert_eq!(poly.coeff, vec![0,0,3,2,5]);
    }

    #[test]
    fn poly_add() {
        let poly_a = PolyZx{coeff:vec![0,0,3,2,5]};
        let poly_b = PolyZx{coeff:vec![4,1,7]};
        let expected_result = PolyZx{coeff:vec![4,1,10,2,5]};

        assert_eq!(poly_a + poly_b , expected_result);

        let poly_a = PolyZx{coeff:vec![0,0,3,2,5]};
        let poly_b = PolyZx{coeff:vec![4,1,7,-2,-5]};
        let expected_result = PolyZx{coeff:vec![4,1,10]};

        assert_eq!(poly_a + poly_b , expected_result);
    }

    #[test]
    fn poly_mul() {
        let poly_a = PolyZx{coeff:vec![1,0,1]};
        let poly_b = PolyZx{coeff:vec![3,2]};
        let expected_result = PolyZx{coeff:vec![3,2,3,2]};

        assert_eq!(poly_a * poly_b , expected_result);

        let poly_a = PolyZx{coeff:vec![0,0,3,2,5]};
        let poly_b = PolyZx{coeff:vec![4,1,7]};
        let expected_result = PolyZx{coeff:vec![0,0,12,11,43,19,35]};

        assert_eq!(poly_a * poly_b , expected_result);
    }

    #[test]
    fn poly_eval(){

        let poly = PolyZx{coeff:vec![4,1,7]};

        let a = Zn{ nb: 3, n:13};

        assert_eq!(poly.eval(a),Zn{nb: 5,n:13});

        let matrix = Matrix::new(vec![1,0,2,3],2);

        assert_eq!(poly.eval(matrix), Matrix::new(vec![12,0,58,70],2));
    }

    }

    #[test]
    fn poly_of_poly(){

        use crate::poly::poly_zx::PolyZx;

        let poly_a = PolyZx{coeff:vec![1,2,1]};
        let poly_b = PolyZx{coeff:vec![4,1,7]};
        assert_eq!(poly_a.eval(poly_b), PolyZx{coeff:vec![25,10,71,14,49]});

    }
    
    #[test]
    fn poly_eval2(){

        let poly = Poly{coeff:Zn::into_Zn(vec![4,1,7], 13)};

        let a = Zn{ nb: 3, n:13};

        assert_eq!(poly.eval(a),Zn{nb: 5,n:13});

        let matrix_a = Matrix::new(vec![3,0,1,1],2);
        let matrix_b = Matrix::new(vec![2,4,0,1],2);

        let poly: Poly<Matrix> = Poly { coeff: vec![matrix_a.clone(),matrix_b.clone()] };

        let matrix = Matrix::new(vec![1,0,2,3],2);

        assert_eq!(poly.eval(matrix.clone()), matrix_a + (matrix * matrix_b));
    }

    #[test]
    fn test(){
        println!();
    }



    
}
