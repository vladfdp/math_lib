mod zn;
mod matrix_z;
mod matrix_ring;
mod traits;
mod poly;
mod field_extension;
mod elliptic_curve;
mod num_theory;
mod vector_z;
mod vector_ring;



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

        #[test]
        fn legendre_symbol_test(){
            let a = Zn { nb: 5, n: 7 };
            assert_eq!(a.legendre_symbol(), -1);

            let b = Zn { nb: 7, n: 11 };
            assert_eq!(b.legendre_symbol(),-1);

            let c = Zn { nb: 3, n: 11 };
            assert_eq!(c.legendre_symbol(), 1);
        }

        #[test]
        fn zn_sqrt(){
            let a = Zn::new(10, 13);

            let expected_result = Zn::new(6, 13);

            assert_eq!(Zn::sqrt(a).0, expected_result);

            let a = Zn { nb: 3, n: 11 };

            let expected_result = Zn::new(5, 11);

            assert_eq!(Zn::sqrt(a).0, expected_result);
        }

        #[test]
        #[should_panic]
        fn zn_invalid_sqrt(){
            let a = Zn { nb: 5, n: 7 };
            
            let _= Zn::sqrt(a);
        }

    }

    mod matrix_z_test{

        use crate::matrix_z::MatrixZ;
        use crate::traits::Pow;

        #[test]
        #[should_panic]
        fn matrix_z_new_invalid() {
            let _matrix_z_a = MatrixZ::new(vec![5, 6, 7, 8, 9, 10], 3);
        }

        #[test]
        fn matrix_z_add_same_size() {
            let matrix_z_a = MatrixZ::new(vec![1, 2, 3, 4], 2);
            let matrix_z_b = MatrixZ::new(vec![5, 6, 7, 8], 2);
            let expected_result = MatrixZ::new(vec![6, 8, 10, 12], 2);

            assert_eq!(matrix_z_a + matrix_z_b, expected_result);

            let matrix_z_a = MatrixZ::new(vec![1, 4, 3, 4, 5, 2, 9, 6, 3], 3);
            let matrix_z_b = MatrixZ::new(vec![2, 6, 1, 8, 5, 8, 2, 1, 0], 3);
            let expected_result = MatrixZ::new(vec![3, 10, 4, 12, 10, 10, 11, 7, 3], 3);

            assert_eq!(matrix_z_a + matrix_z_b, expected_result);
        }

        #[test]
        #[should_panic]
        fn matrix_z_add_diff_size() {

            let matrix_z_a = MatrixZ::new(vec![1, 2, 3, 4], 2);
            let matrix_z_b = MatrixZ::new(vec![5, 6, 7, 8, 9, 10, 11, 12, 13], 3);
            let _ = matrix_z_a + matrix_z_b;
        }

        #[test]
        fn matrix_z_mul_same_size() {
            let matrix_z_a = MatrixZ::new(vec![1, 2, 3, 4], 2);
            let matrix_z_b = MatrixZ::new(vec![5, 6, 7, 8], 2);
            let expected_result = MatrixZ::new(vec![19, 22, 43, 50], 2);

            assert_eq!(matrix_z_a * matrix_z_b, expected_result);

            let matrix_z_a = MatrixZ::new(vec![1, 4, 3, 4, 5, 2, 9, 6, 3], 3);
            let matrix_z_b = MatrixZ::new(vec![2, 6, 1, 8, 5, 8, 2, 1, 0], 3);
            let expected_result = MatrixZ::new(vec![40, 29, 33, 52, 51, 44, 72, 87, 57], 3);
            

            assert_eq!(matrix_z_a * matrix_z_b, expected_result);
        }

        #[test]
        #[should_panic]
        fn matrix_z_mul_diff_size() {

            let matrix_z_a = MatrixZ::new(vec![1, 2, 3, 4], 2);
            let matrix_z_b = MatrixZ::new(vec![5, 6, 7, 8, 9, 10, 11, 12, 13], 3);
            let _ = matrix_z_a * matrix_z_b;
        }

        #[test]
        fn matrix_z_pow() {
            let matrix_z_a = MatrixZ::new(vec![1, 2, 3, 4], 2);
            let matrix_z_a_pow_4 = MatrixZ::new(vec![199, 290, 435, 634], 2);
            assert_eq!(matrix_z_a.pow(4), matrix_z_a_pow_4);

            let matrix_z_b = MatrixZ::new(vec![2, 6, 1, 8, 5, 8, 2, 1, 0], 3);
            let matrix_z_b_pow_3 = MatrixZ::new(vec![552, 589, 398, 888, 885, 720, 180, 167, 148], 3);
            assert_eq!(matrix_z_b.pow(3), matrix_z_b_pow_3);
        }
    }

    mod matrix_ring_test{
        use crate::{matrix_ring::MatrixRing, zn::Zn, traits::Pow};


        #[test]
        fn matrix_z_add_same_size() {
            let matrix_z_a = MatrixRing::new(Zn::from_vec(vec![1, 2, 3, 4], 13), 2);
            let matrix_z_b = MatrixRing::new(Zn::from_vec(vec![5, 6, 7, 8], 13), 2);
            let expected_result = MatrixRing::new(Zn::from_vec(vec![6, 8, 10, 12], 13), 2);

            assert_eq!(matrix_z_a + matrix_z_b, expected_result);

            let matrix_z_a = MatrixRing::new(Zn::from_vec(vec![1, 4, 3, 4, 5, 2, 9, 6, 3], 13), 3);
            let matrix_z_b = MatrixRing::new(Zn::from_vec(vec![2, 6, 1, 8, 5, 8, 2, 1, 0], 13), 3);
            let expected_result = MatrixRing::new(Zn::from_vec(vec![3, 10, 4, 12, 10, 10, 11, 7, 3], 13), 3);

            assert_eq!(matrix_z_a + matrix_z_b, expected_result);
        }

        #[test]
        #[should_panic]
        fn matrix_z_add_diff_size() {

            let matrix_z_a = MatrixRing::new(Zn::from_vec(vec![1, 2, 3, 4], 13), 2);
            let matrix_z_b = MatrixRing::new(Zn::from_vec(vec![5, 6, 7, 8, 9, 10, 11, 12, 7], 13), 3);
            let _ = matrix_z_a + matrix_z_b;
        }

        #[test]
        fn matrix_z_mul_same_size() {
            let matrix_z_a = MatrixRing::new(Zn::from_vec(vec![1, 2, 3, 4], 13), 2);
            let matrix_z_b = MatrixRing::new(Zn::from_vec(vec![5, 6, 7, 8], 13), 2);
            let expected_result = MatrixRing::new(Zn::from_vec(vec![6, 9, 4, 11], 13), 2);

            assert_eq!(matrix_z_a * matrix_z_b, expected_result);

            let matrix_z_a = MatrixRing::new(Zn::from_vec(vec![1, 4, 3, 4, 5, 2, 9, 6, 3], 13), 3);
            let matrix_z_b = MatrixRing::new(Zn::from_vec(vec![2, 6, 1, 8, 5, 8, 2, 1, 0], 13), 3);
            let expected_result = MatrixRing::new(Zn::from_vec(vec![40, 29, 33, 52, 51, 44, 72, 87, 57], 13), 3);
            

            assert_eq!(matrix_z_a * matrix_z_b, expected_result);
        }

         
        

        #[test]
        fn matrix_z_pow() {
            let matrix_z_a = MatrixRing::new(Zn::from_vec(vec![1, 2, 3, 4], 13), 2);
            let matrix_z_a_pow_4 = MatrixRing::new(Zn::from_vec(vec![199, 290, 435, 634], 13), 2);
            assert_eq!(matrix_z_a.pow(4), matrix_z_a_pow_4);

            let matrix_z_b = MatrixRing::new(Zn::from_vec(vec![2, 6, 1, 8, 5, 8, 2, 1, 0], 13), 3);
            let matrix_z_b_pow_3 = MatrixRing::new(Zn::from_vec(vec![552, 589, 398, 888, 885, 720, 180, 167, 148], 13), 3);
            assert_eq!(matrix_z_b.pow(3), matrix_z_b_pow_3);
        }

    }

    mod vector_z_test{


        use crate::vector_z::VectorZ;

        #[test]
        fn vector_z_add_same_size() {
            let vector_a = VectorZ::new(vec![1, 2, 3, 4]);
            let vector_b = VectorZ::new(vec![5, 6, 7, 8]);
            let expected_result = VectorZ::new(vec![6, 8, 10, 12]);

            assert_eq!(vector_a + vector_b, expected_result);

            let vector_a = VectorZ::new(vec![1, 4, 3, 4, 5, 2, 9, 6, 3]);
            let vector_b = VectorZ::new(vec![2, 6, 1, 8, 5, 8, 2, 1, 0]);
            let expected_result = VectorZ::new(vec![3, 10, 4, 12, 10, 10, 11, 7, 3]);

            assert_eq!(vector_a + vector_b, expected_result);
        }

        #[test]
        #[should_panic]
        fn vector_z_add_diff_size() {

            let vector_a = VectorZ::new(vec![1, 2, 3, 4]);
            let vector_b = VectorZ::new(vec![5, 6, 7, 8, 9]);
            let _ = vector_a + vector_b;
        }

        #[test]
        fn vector_dot_product() {
            let vector_a = VectorZ::new(vec![1, 2, 3, 4]);
            let vector_b = VectorZ::new(vec![5, 6, 7, 8]);
            let expected_result = 70;

            assert_eq!(vector_a * vector_b, expected_result);

            let vector_a = VectorZ::new(vec![1, 4, 3]);
            let vector_b = VectorZ::new(vec![2, 6, 1]);
            let expected_result = 29;
            

            assert_eq!(vector_a * vector_b, expected_result);
        }

        #[test]
        #[should_panic]
        fn vector_dot_prod_diff_size() {

            let vector_a = VectorZ::new(vec![1, 2, 3, 4]);
            let vector_b = VectorZ::new(vec![5, 6, 7, 8, 9, 10]);
            let _ = vector_a * vector_b;
        }
    }

    mod vector_ring_test{
        use crate::{vector_ring::Vector, zn::Zn};

        #[test]
        fn vector_ring_add_same_size() {
            let vector_a = Vector::new(Zn::from_vec(vec![1, 2, 3, 4],11));
            let vector_b = Vector::new(Zn::from_vec(vec![5, 6, 7, 8],11));
            let expected_result = Vector::new(Zn::from_vec(vec![6, 8, 10, 12], 11));

            assert_eq!(vector_a + vector_b, expected_result);

            let vector_a = Vector::new(Zn::from_vec(vec![1, 4, 3, 4, 5, 2, 9, 6, 3],5));
            let vector_b = Vector::new(Zn::from_vec(vec![2, 6, 1, 8, 5, 8, 2, 1, 0],5));
            let expected_result = Vector::new(Zn::from_vec(vec![3, 10, 4, 12, 10, 10, 11, 7, 3],5));

            assert_eq!(vector_a + vector_b, expected_result);
        }

        #[test]
        fn vector_ring_dot_product() {
            let vector_a = Vector::new(Zn::from_vec(vec![1, 2, 3, 4],13));
            let vector_b = Vector::new(Zn::from_vec(vec![5, 6, 7, 8],13));
            let expected_result = Zn::new(70, 13);

            assert_eq!(vector_a * vector_b, expected_result);

            let vector_a = Vector::new(Zn::from_vec(vec![1, 4, 3],13));
            let vector_b = Vector::new(Zn::from_vec(vec![2, 6, 1],13));
            let expected_result = Zn::new(29, 13) ;
            

            assert_eq!(vector_a * vector_b, expected_result);
        }
    }

    mod matrix_vector_mul{

        use crate::matrix_z::MatrixZ;
        use crate::vector_z::VectorZ;

        #[test]
        fn mat_vec_mul_over_z(){

            let matrix = MatrixZ::new(vec![1, 2, 3, 4], 2);
            let vector = VectorZ::new(vec![5, 6]);

            let expected_result = VectorZ::new(vec![17, 39]);

            assert_eq!(matrix * vector, expected_result);

            let matrix = MatrixZ::new(vec![5, 6, 9, 1, 0, 0, 3, 2, 2], 3);
            let vector = VectorZ::new(vec![1, 2, 4]);

            let expected_result = VectorZ::new(vec![53, 1, 15]);

            assert_eq!(matrix * vector, expected_result);
        }
    }

    mod poly_zx_test{


        use crate::{poly::poly_zx::PolyZx , matrix_z::MatrixZ, zn::Zn};


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

        let matrix_z = MatrixZ::new(vec![1,0,2,3],2);

        assert_eq!(poly.eval(matrix_z), MatrixZ::new(vec![12,0,58,70],2));
        

        let matrix = MatrixZ::new(vec![1,3,2,0], 2);

        let poly = PolyZx{coeff: vec![-6,-1,1]};

        let expected_result = MatrixZ::new(vec![0,0,0,0], 2);

        assert_eq!(poly.eval(matrix),expected_result)
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
        use crate::poly::poly_zx::PolyZx;
        use crate::traits::{Zero, One};
        use crate::poly::poly_ring::Poly;
        use crate::zn::Zn;
        use crate::matrix_z::MatrixZ;

        #[test]
        fn poly_ring_eval(){
    
            let poly = Poly{coeff:Zn::from_vec(vec![4,1,7], 13)};
    
            let a = Zn{ nb: 3, n:13};
    
            assert_eq!(poly.eval(a),Zn{nb: 5,n:13});
    
            let MatrixZ_a = MatrixZ::new(vec![3,0,1,1],2);
            let MatrixZ_b = MatrixZ::new(vec![2,4,0,1],2);
    
            let poly: Poly<MatrixZ> = Poly { coeff: vec![MatrixZ_a.clone(),MatrixZ_b.clone()] };
    
            let MatrixZ = MatrixZ::new(vec![1,0,2,3],2);
    
            assert_eq!(poly.eval(MatrixZ.clone()), MatrixZ_a + (MatrixZ * MatrixZ_b));
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

        }
    
        #[test]
    fn poly_ring_mul() {
        let poly_a = Poly{coeff: Zn::from_vec(vec![1,0,1], 13)};
        let poly_b = Poly{coeff: Zn::from_vec(vec![3,2], 13)};

        let expected_result= Poly{coeff: Zn::from_vec(vec![3,2,3,2], 13)};

        assert_eq!(poly_a * poly_b , expected_result);

        let poly_a = Poly{coeff: Zn::from_vec(vec![0,0,3,2,5], 27)};
        let poly_b = Poly{coeff: Zn::from_vec(vec![4,1,7], 27)};
        let expected_result = Poly{coeff: Zn::from_vec(vec![0,0,12,11,16,19,8], 27)};

        assert_eq!(poly_a * poly_b , expected_result);
    }


        #[test]
        fn poly_ring_of_poly(){

    
            let poly_a = Poly{coeff:vec![PolyZx{coeff:vec![1]},PolyZx{coeff:vec![2]},PolyZx{coeff:vec![1]}]};
            let poly_b = PolyZx{coeff:vec![4,1,7]};
            assert_eq!(poly_a.eval(poly_b), PolyZx{coeff:vec![25,10,71,14,49]});
    
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
            let poly_b = Polyff{coeff: Zn::from_vec(vec![1,0,1], 13)};  //1,0,1


            let expected_result = Polyff{coeff:Zn::from_vec(vec![11,9], 13)};


            assert_eq!(poly_a % poly_b, expected_result);

        }


        #[test]
        #[should_panic]
        fn poly_ff_rem_0(){

            let poly_a = Polyff{coeff: Zn::from_vec(vec![4,1,7], 13)};
            let poly_b = Polyff{coeff: Zn::from_vec(vec![0], 13)};


            let _ = poly_a % poly_b;

        }
        #[test]
        fn poly_ff_div(){

            let poly_a = Polyff{coeff: Zn::from_vec(vec![4,1,7], 13)};
            let poly_b = Polyff{coeff: Zn::from_vec(vec![11,1,6], 13)};


            let expected_result = Polyff{coeff:Zn::from_vec(vec![12], 13)};

            assert_eq!(poly_a / poly_b, expected_result);
            
            let poly_a = Polyff{coeff: Zn::from_vec(vec![4,1,7], 13)};
            let poly_b = Polyff{coeff: Zn::from_vec(vec![11,1,1,1], 13)};


            let expected_result = Polyff{coeff:Zn::from_vec(vec![0], 13)};

            assert_eq!(poly_a / poly_b, expected_result);

            let poly_a = Polyff{coeff: Zn::from_vec(vec![4,1,7,5,1], 13)};
            let poly_b = Polyff{coeff: Zn::from_vec(vec![1,0,1], 13)};  //1,0,1


            let expected_result = Polyff{coeff:Zn::from_vec(vec![6,5,1], 13)};


            assert_eq!(poly_a / poly_b, expected_result);

            let poly_a = Polyff{coeff: Zn::from_vec(vec![4,1,7,5,1], 13)};
            let poly_b = Polyff{coeff: Zn::from_vec(vec![1,0,1], 13)};  //1,0,1

            assert_eq!((poly_a.clone() / poly_b.clone()) * poly_b.clone() + (poly_a.clone() % poly_b), poly_a);


            
        }

        #[test]
        #[should_panic]
        fn poly_ff_div_0(){

            let poly_a = Polyff{coeff: Zn::from_vec(vec![4,1,7], 13)};
            let poly_b = Polyff{coeff: Zn::from_vec(vec![0], 13)};


            let _ = poly_a / poly_b;

        }

        #[test]
        fn poly_ff_gcd(){
            let poly_a = Polyff{coeff: Zn::from_vec(vec![2,10,1], 13)};
            let poly_b = Polyff{coeff: Zn::from_vec(vec![1,12,12,1], 13)};

            let expected_result = Polyff{coeff: Zn::from_vec(vec![12,1], 13)};

            assert_eq!(Polyff::GCD(poly_a, poly_b), expected_result);


            let poly_a = Polyff{coeff: Zn::from_vec(vec![2,10,1], 13)};
            let poly_b = Polyff{coeff: Zn::from_vec(vec![0], 13)};

            let expected_result = Polyff{coeff: Zn::from_vec(vec![2,10,1], 13)};

            assert_eq!(Polyff::GCD(poly_a, poly_b), expected_result);

            let poly_a = Polyff{coeff: Zn::from_vec(vec![3,6,7,1,1,21], 23)};
            let poly_b = Polyff{coeff: Zn::from_vec(vec![1,4,11], 23)};

            let expected_result = Polyff{coeff: Zn::from_vec(vec![1], 23)};

            assert_eq!(Polyff::GCD(poly_a, poly_b), expected_result);
        }

    }
 
    mod fe_test{
        use crate::{field_extension::FieldExtension,
            zn::Zn, 
            traits::{Inv, Card, One, Zero, Pow},
            poly::poly_ff::Polyff};


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

        use crate::elliptic_curve::{EllipticCurve,EllipticCurvePoint,ECPoint};
        use crate::zn::Zn;



        #[test]
        fn ec_is_on_curve(){

            let ec = EllipticCurve::new(Zn::new(8, 13),Zn::new(8, 13));

            let pt = ECPoint::Point{x:Zn::new(11,13),y:Zn::new(7, 13)};

            assert!(ec.is_on_curve(&pt));
        }

        #[test]
        fn ec_add_point(){

            let ec = EllipticCurve::new(Zn::new(1, 5),Zn::new(1, 5));

            let x = ec.new_point(Zn::new(0, 5),Zn::new(1, 5));
            let y = ec.new_point(Zn::new(4, 5),Zn::new(2, 5));

            let expected_result = ec.new_point(Zn::new(2, 5),Zn::new(1, 5));

            assert_eq!(x + y, expected_result);

            let x = ec.new_point(Zn::new(0, 5),Zn::new(1, 5));
            let y = ec.new_point(Zn::new(0, 5),Zn::new(4, 5));

            let expected_result = ec.infinity();

            assert_eq!(x + y, expected_result);

            let ec = EllipticCurve::new(Zn::new(8, 13),Zn::new(8, 13));

            let x = ec.new_point(Zn::new(4, 13),Zn::new(0, 13));

            let expected_result = ec.infinity();

            assert_eq!(x.clone() + x, expected_result);


            let x = ec.new_point(Zn::new(10, 13),Zn::new(3, 13));
            let y = ec.new_point(Zn::new(10, 13),Zn::new(10, 13));

            let expected_result = ec.infinity();

            assert_eq!(x + y, expected_result);

        }

        #[test]
        #[should_panic]
        fn ec_add_diff_curve(){
            
            let ec1 = EllipticCurve::new(Zn::new(1, 5),Zn::new(1, 5));
            let ec2 = EllipticCurve::new(Zn::new(8, 13),Zn::new(8, 13));

            let x = ec1.new_point(Zn::new(0, 5),Zn::new(1, 5));
            let y = ec2.new_point(Zn::new(11, 13),Zn::new(7, 13));

            let _= x + y;
        }

        #[test]
        fn ec_mul_scalar(){

            let ec = EllipticCurve::new(Zn::new(8, 13),Zn::new(8, 13));

            let g = ec.new_point(Zn::new(7,13), Zn::new(11, 13));

            let two_g = ec.new_point(Zn::new(8,13), Zn::new(5, 13));

            let three_g = ec.new_point(Zn::new(8,13), Zn::new(8, 13));

            let four_g = ec.new_point(Zn::new(7,13), Zn::new(2, 13));
            

            assert_eq!(g.clone() * 2, two_g);

            assert_eq!(g.clone() * 3, three_g);

            assert_eq!(g * 4, four_g);



        }


    }
    
    mod num_theory_test{

        use crate::num_theory::is_prime;

        #[test]
        fn prime_check(){
            assert!(is_prime(3));
            assert!(is_prime(13));
            assert!(is_prime(17));
            assert!(is_prime(31));
            assert!(!is_prime(16));
        }
    }
    
    use crate::poly::poly_ff::Polyff;
    use crate::poly::poly_zx::PolyZx;
    use crate::zn::Zn;
    use crate::traits::{Pow,Inv};



    use crate::matrix_z::MatrixZ;
        use crate::poly::poly_ring::Poly;

    #[test]
    fn test(){

        

        
       
    }



    
}
