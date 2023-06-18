mod zn;
mod matrix;
mod traits;
mod poly;
mod field_extension;




#[cfg(test)]
mod tests {


    //use std::convert::TryInto;
    //use std::ops::{Add,Mul};


    


    mod zn_tests{

        use crate::zn::Zn;
        use crate::traits::Pow;
        use crate::traits::Inv;

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
    fn zn_inv() {
        let a = Zn { nb: 5, n: 7 };
        assert_eq!(a.inv(), Zn { nb: 3, n: 7 });

        let a = Zn { nb: 6, n: 13 };
        assert_eq!(a.inv(), Zn { nb: 11, n: 13 });
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
    #[test]
    fn poly_of_poly(){

        use crate::poly::poly_zx::PolyZx;

        let poly_a = PolyZx{coeff:vec![1,2,1]};
        let poly_b = PolyZx{coeff:vec![4,1,7]};
        assert_eq!(poly_a.eval(poly_b), PolyZx{coeff:vec![25,10,71,14,49]});

    }

    }

    mod poly_ring_test{
        use crate::traits::{Zero, One};
        use crate::poly::poly_ring::Poly;
        use crate::zn::Zn;
        use crate::matrix::Matrix;

        #[test]
        fn poly_ring_eval(){
    
            let poly = Poly{coeff:Zn::from_vec(vec![4,1,7], 13)};
    
            let a = Zn{ nb: 3, n:13};
    
            assert_eq!(poly.eval(a),Zn{nb: 5,n:13});
    
            let matrix_a = Matrix::new(vec![3,0,1,1],2);
            let matrix_b = Matrix::new(vec![2,4,0,1],2);
    
            let poly: Poly<Matrix> = Poly { coeff: vec![matrix_a.clone(),matrix_b.clone()] };
    
            let matrix = Matrix::new(vec![1,0,2,3],2);
    
            assert_eq!(poly.eval(matrix.clone()), matrix_a + (matrix * matrix_b));
        }

        #[test]
        fn poly_ring_add() {


            let poly_a = Poly{coeff: Zn::from_vec(vec![5,1,7], 13)};
            let poly_b = Poly{coeff: Zn::from_vec(vec![11,1,6], 13)};

            let expected_result = Poly{coeff:Zn::from_vec(vec![3,2], 13)};

            assert_eq!(poly_a + poly_b, expected_result);

            let poly_a = Poly{coeff: Zn::from_vec(vec![5,1,7], 13)};
            let poly_b = Poly{coeff: Zn::from_vec(vec![8,12,6], 13)};

            let expected_result = Poly{coeff:Zn::from_vec(vec![0], 13)};

            assert_eq!(poly_a + poly_b, expected_result);

            let poly_a: Poly<i32> = Poly{coeff: vec![5,1,7]};
            let poly_b: Poly<i32> = Poly{coeff: vec![-5,-1,-7]};


            assert!((poly_a + poly_b).is_zero());

        }
    
        #[test]
    fn poly_ring_mul() {
        let poly_a:Poly<i32> = Poly{coeff:vec![1,0,1]};
        let poly_b:Poly<i32> = Poly{coeff:vec![3,2]};
        let expected_result:Poly<i32> = Poly{coeff:vec![3,2,3,2]};

        assert_eq!(poly_a * poly_b , expected_result);

        let poly_a:Poly<i32> = Poly{coeff:vec![0,0,3,2,5]};
        let poly_b:Poly<i32> = Poly{coeff:vec![4,1,7]};
        let expected_result:Poly<i32> = Poly{coeff:vec![0,0,12,11,43,19,35]};

        assert_eq!(poly_a * poly_b , expected_result);
    }


        #[test]
        fn poly_ring_of_poly(){

    
            let poly_a = Poly{coeff:vec![Poly{coeff:vec![1]},Poly{coeff:vec![2]},Poly{coeff:vec![1]}]};
            let poly_b = Poly{coeff:vec![4,1,7]};
            assert_eq!(poly_a.eval(poly_b), Poly{coeff:vec![25,10,71,14,49]});
    
        }
    }

    mod poly_ff_test{

        use crate::poly::poly_ff::Polyff;
        use crate::zn::Zn;

        #[test]
        fn poly_ff_eval(){
    
            let poly = Polyff{coeff:Zn::from_vec(vec![4,1,7], 13)};
    
            let a = Zn{ nb: 3, n:13};
    
            assert_eq!(poly.eval(a),Zn{nb: 5,n:13});
        }

        #[test]
        fn poly_ff_add() {


            let poly_a = Polyff{coeff: Zn::from_vec(vec![5,1,7], 13)};
            let poly_b = Polyff{coeff: Zn::from_vec(vec![11,1,6], 13)};

            let expected_result = Polyff{coeff:Zn::from_vec(vec![3,2], 13)};

            assert_eq!(poly_a + poly_b, expected_result);

            let poly_a = Polyff{coeff: Zn::from_vec(vec![5,1,7], 13)};
            let poly_b = Polyff{coeff: Zn::from_vec(vec![8,12,6], 13)};

            let expected_result = Polyff{coeff:Zn::from_vec(vec![0], 13)};

            assert_eq!(poly_a + poly_b, expected_result);

        }
    
        #[test]
    fn poly_ff_mul() {
        
        let poly_a = Polyff{coeff: Zn::from_vec(vec![4,1,7], 13)};
        let poly_b = Polyff{coeff: Zn::from_vec(vec![11,1,6], 13)};


        let expected_result = Polyff{coeff:Zn::from_vec(vec![5,2,11,0,3], 13)};

        assert_eq!(poly_a * poly_b, expected_result);
    }

    #[test]
    fn poly_ff_rem(){

        let poly_a = Polyff{coeff: Zn::from_vec(vec![4,1,7], 13)};
        let poly_b = Polyff{coeff: Zn::from_vec(vec![11,1,6], 13)};


        let expected_result = Polyff{coeff:Zn::from_vec(vec![2,2], 13)};


        assert_eq!(poly_a % poly_b, expected_result);

        let poly_a = Polyff{coeff: Zn::from_vec(vec![4,1,7], 13)};
        let poly_b = Polyff{coeff: Zn::from_vec(vec![11,1,1,1], 13)};


        let expected_result = Polyff{coeff:Zn::from_vec(vec![4,1,7], 13)};


        assert_eq!(poly_a % poly_b, expected_result);

        let poly_a = Polyff{coeff: Zn::from_vec(vec![4,1,7,5,1], 13)};
        let poly_b = Polyff{coeff: Zn::from_vec(vec![1,0,1], 13)};


        let expected_result = Polyff{coeff:Zn::from_vec(vec![11,9], 13)};


        assert_eq!(poly_a % poly_b, expected_result);

    }


    }
 

    mod fe_test{
        use crate::{field_extension::FieldExtension, zn::Zn, traits::{Inv, Card, One, Zero, Pow}, poly::poly_ff::Polyff};


        #[test]
        fn fe_new_test(){
    
            let x = FieldExtension::new_from_vec(
                Zn::from_vec(vec![1,5,3,2], 3),
                Zn::from_vec(vec![1,1,2],3)
                );
    
            assert_eq!(Zn::into_i32(x.nb.coeff),vec![0]);

            let x = FieldExtension::new_from_vec(
                Zn::from_vec(vec![1,5,4,2], 3),
                Zn::from_vec(vec![1,1,2],3)
                );
    
            assert_eq!(Zn::into_i32(x.nb.coeff),vec![1,1]);
        }

        #[test]
        fn fe_add_same_field(){
            let x = FieldExtension::new_from_vec(
                Zn::from_vec(vec![2,4], 5),
                Zn::from_vec(vec![1,1,2],5)
                );
    

            let y = FieldExtension::new_from_vec(
                Zn::from_vec(vec![1,1], 5),
                Zn::from_vec(vec![1,1,2],5)
                );
    
            assert_eq!(Zn::into_i32((x + y).nb.coeff),vec![3]);

        }

        #[test]
        fn fe_mul_same_field(){
            let x = FieldExtension::new_from_vec(
                Zn::from_vec(vec![2,4], 5),
                Zn::from_vec(vec![1,1,2],5)
                );
    

            let y = FieldExtension::new_from_vec(
                Zn::from_vec(vec![1,1], 5),
                Zn::from_vec(vec![1,1,2],5)
                );
    
            assert_eq!(Zn::into_i32((x * y).nb.coeff),vec![0,4]);

        }

        #[test]
        fn fe_inv(){
            let x = FieldExtension::new_from_vec(
                Zn::from_vec(vec![2,4], 5),
                Zn::from_vec(vec![1,1,2],5)
            );
            println!("{}",x.get_card());

            let y = x.inv();

            assert_eq!(Zn::into_i32((x * y).nb.coeff),vec![1]);
        }

        #[test]
        fn fe_of_fe(){

            let alpha = FieldExtension::new_from_vec( //alpha is such that alpha^2 == alpha + 1
                Zn::from_vec(vec![0,1], 2),
                Zn::from_vec(vec![1,1,1],2)
            );

            assert_eq!(alpha.pow(3),alpha.one());

            let a2:FieldExtension<FieldExtension<Zn>> = FieldExtension::new_from_vec(
                vec![alpha.zero(),alpha.one()],
                vec![alpha.one(), alpha.clone(), alpha.one()]
            );


            assert_eq!(a2.pow(15).nb.coeff, a2.one().nb.coeff)

        }
        
    }
    
    
    mod ec_test{
        
    }
    
    
    use crate::poly::poly_ff::Polyff;
    use crate::zn::Zn;
    use crate::traits::{Pow,Inv};



    #[test]
    fn test(){

       
    }



    
}
