use num_bigint::BigUint;
pub mod ecdh;
pub mod ecdsa;

pub struct EllipticCurve1{
     a:BigUint,
     b:BigUint,
     p:BigUint,

}
#[derive(PartialEq,Clone,Debug)]
pub enum point{
    coor(BigUint,BigUint),
    identity 
}
impl EllipticCurve1 {
    pub fn add(&self,P:&point,Q:&point) ->point{
        assert!(self.is_pt_on_curve(P),"point P is not on the curtodo!()ve");
        assert!(self.is_pt_on_curve(Q),"point Q is not on the curve");
        assert!(P!=Q,"points P and  Q should not be equal");
        match (P,Q) {
            (point::identity,Q) =>Q.clone(),
            (P,point::identity) =>P.clone(),
            (point::coor(x1,y1),(point::coor(x2,y2))) =>{
                /*m = (y2-y1)/(x2-x1) 
                x3 = m^2 - x1-x2
                y3 = m*(x1-x3)-y1*/
                let y1plusy2 = finitefield::add(y1, y2, &self.p);
                if x1 == x2&& y1plusy2 ==BigUint::from(0u32) {return point::identity}
                let y2minusy1 = finitefield::subtract(y2, y1, &self.p);
                let x2minusx1 = finitefield::subtract(x2, x1, &self.p);
                let m = finitefield::devision(&y2minusy1, &x2minusx1, &self.p);
                let m2 = m.modpow(&BigUint::from(2u32), &self.p);
                let minusof_x1plusx2 = finitefield::inv_add((&finitefield::add(x1, x2, &self.p)),&self.p);
                let x3 = finitefield::add(&m2, &minusof_x1plusx2, &self.p);
                let x1minusx3 = finitefield::subtract(x1, &x3, &self.p);
                let m_into_x1minusx3 = finitefield::multiplication(&m, &x1minusx3, &self.p);
                let y3 =finitefield::subtract(&m_into_x1minusx3, y1, &self.p);
                return point::coor(x3,y3);

            }
            
        }
        /* */
    }
    pub fn pt_doubling(&self,P:&point)->point{
        /*x3 = m2-2x1, y3 = m(x1-x3)-y1,where m = (3x1^(2)+a/2y1)*/
        match P {
            point::identity =>point::identity,
            point::coor(x1,y1) =>{
                if *y1 ==BigUint::from(0u32){return point::identity;}
                let x1square= x1.modpow(&BigUint::from(2u32), &self.p);
                let _3x1square = finitefield::multiplication(&BigUint::from(3u32), &x1square, &self.p);
                let _3x1squareplusa = finitefield::add(&_3x1square, &self.a, &self.p);
                let _2y1 = finitefield::multiplication(&BigUint::from(2u32), y1, &self.p);
                let m = finitefield::devision(&_3x1squareplusa, &_2y1, &self.p);
                let _2x1 = finitefield::multiplication(&BigUint::from(2u32), x1,&self.p);
                let m2 = m.modpow(&BigUint::from(2u32), &self.p);
                let x3 = finitefield::subtract(&m2, &_2x1, &self.p);
                let x1minusx3 = finitefield::subtract(x1, &x3, &self.p);
                let m_into_x1minusx3 = finitefield::multiplication(&m, &x1minusx3, &self.p);
                let y3 = finitefield::subtract(&m_into_x1minusx3, &y1, &self.p);
                return point::coor(x3, y3)
            }
        }
        
    }
    pub fn scalar_multiplication(&self,n:&BigUint,P:&point)-> point{
        /*This function is going to take a number 'n' and a point 'P' on the elliptic curve and 
        returns n*P point on the elliptic curve  */
        /*Double and Add algoritm */
        let k = n.bits();
        let mut t = point::identity;
        for i in 0..(n.bits()){
            t = self.pt_doubling(&t);
            if n.bit(k-i-1){
                t = self.add(&t, P);
            }
            


        }
        t
       
    }
    







    // fn doubling(P:&point) ->point{}
    // fn multiply(P:&point,k:BigUint) ->point{}
    pub fn is_pt_on_curve(&self,P:&point)->bool{
        /*y2 = x3+a*x+b */
        match P {
            point::coor(x,y ) =>{
                let y2 = y.modpow(&BigUint::from(2u32),&self.p);
                let x3 = x.modpow(&BigUint::from(3u32), &self.p);
                let ax =finitefield::multiplication(&self.a, x, &self.p);
                let x3plusax = finitefield::add(&x3, &ax, &self.p);
                y2 == finitefield::add(&x3plusax, &self.b, &self.p)
                //y2 == x3+ax+&self.b
            }
            point::identity =>true
        }
        
    }
    
    
}
pub struct finitefield{}
impl finitefield {
    fn add(c:&BigUint,d:&BigUint,p:&BigUint)->BigUint{
        let r = c+d ;
        r %p
          /*c+d = r mod p */
    }
    fn multiplication(c:&BigUint,d:&BigUint,p:&BigUint)->BigUint{
        let r = c*d ;
        r%p
         /*a*b mod p = c */
    }
    fn inv_add(c:&BigUint,p:&BigUint) ->BigUint{
        let r = c%p;
        p-r   /*-c mod p */
    }
    fn inv_mul(c:&BigUint,p:&BigUint) ->BigUint{
         c.modpow(&(p-BigUint::from(2u32)), &p) /* c^(-1) mod p */
    }
    fn subtract(c:&BigUint,d:&BigUint,p:&BigUint) ->BigUint{
        let inv_d= finitefield::inv_add(d,p);
        finitefield::add(c, &inv_d, p)
    }
    fn devision(c:&BigUint,d:&BigUint,p:&BigUint) ->BigUint{
        let inv_d = finitefield::inv_mul(d, p);
        finitefield::multiplication(c, &inv_d, p)
    }


    
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_add1(){
        let c = BigUint::from(5u32);
        let d = BigUint::from(6u32);
        let p = BigUint::from(7u32);
        let r = finitefield::add(&c, &d, &p);
        assert_eq!(r,BigUint::from(4u32));
    }
    #[test]
    fn test_multiplication2(){
        let c = BigUint::from(5u32);
        let d = BigUint::from(6u32);
        let p = BigUint::from(7u32);
        let r = finitefield::multiplication(&c, &d, &p);
        assert_eq!(r,BigUint::from(2u32));
    }
    #[test]
    fn test_inverse_add(){
        let c = BigUint::from(11u32);
        let p = BigUint::from(7u32);
        let r = finitefield::inv_add(&c, &p);
        assert_eq!(r,BigUint::from(3u32));
    }
    #[test]
    fn test_inverse_multiply(){
        let c = BigUint::from(4u32);
        let p = BigUint::from(11u32);
        let r = finitefield::inv_mul(&c, &p);
        assert_eq!(r,BigUint::from(3u32));
    }
    #[test]
    fn test_devide(){
        
        let c = BigUint::from(5u32);
        let d = BigUint::from(6u32);
        let p = BigUint::from(7u32);
        let r = finitefield::devision(&c, &d, &p);
        assert_eq!(r,BigUint::from(2u32));
        }
    #[test]
    fn test_elliptic_pt_addition(){
        /*y2 = x3+2x+2 mod 17 */
        let ec = EllipticCurve1{a:BigUint::from(2u32),b:BigUint::from(2u32),p:BigUint::from(17u32)};
        let P1 = point::coor(BigUint::from(3u32),BigUint::from(1u32));
        let P2 = point::coor(BigUint::from(5u32),BigUint::from(1u32));
        let P1plusP2 = point::coor(BigUint::from(9u32),BigUint::from(16u32));

        let result = ec.add(&P2, &P1);
        assert_eq!(result,P1plusP2);
    }
    #[test]
    fn test_elliptic_pt_addition_identity(){
        /*y2 = x3+2x+2 mod 17 */
        let ec = EllipticCurve1{a:BigUint::from(2u32),b:BigUint::from(2u32),p:BigUint::from(17u32)};
        let P1 = point::coor(BigUint::from(3u32),BigUint::from(1u32));
        let P2 = point::identity;
        let P1plusP2 = P1.clone();

        let result = ec.add(&P1, &P2);
        assert_eq!(result,P1plusP2);
    }
    #[test]
    fn test_elliptic_pt_addition2(){
        /*y2 = x3+2x+2 mod 17 */
        let ec = EllipticCurve1{a:BigUint::from(2u32),b:BigUint::from(2u32),p:BigUint::from(17u32)};
        let P1 = point::coor(BigUint::from(3u32),BigUint::from(1u32));
        let P2 = point::coor(BigUint::from(3u32),BigUint::from(16u32));
        let P1plusP2 = point::identity;

        let result = ec.add(&P2, &P1);
        assert_eq!(result,P1plusP2);
    }
    #[test]
    fn test_elliptic_pt_doubling(){
        let ec = EllipticCurve1{a:BigUint::from(2u32),b:BigUint::from(2u32),p:BigUint::from(17u32)};
        let P1 = point::coor(BigUint::from(5u32),BigUint::from(1u32));
        let _2P1 = point::coor(BigUint::from(6u32), BigUint::from(3u32));
        let result = ec.pt_doubling(&P1);
        assert_eq!(result,_2P1);

    }
    #[test]
    fn test_elliptic_pt_doubling2(){
        /*2(5,1) = (6,3) */
        let ec = EllipticCurve1{a:BigUint::from(2u32),b:BigUint::from(2u32),p:BigUint::from(17u32)};
        let P1 = point::coor(BigUint::from(5u32),BigUint::from(1u32));
        let _2P1 = point::coor(BigUint::from(6u32),BigUint::from(3u32));
        let result = ec.pt_doubling(&P1);
        assert_eq!(result,_2P1);

    }

    #[test]
    fn test_elliptic_scalar_multiplication(){
        let ec = EllipticCurve1{a:BigUint::from(2u32),b:BigUint::from(2u32),p:BigUint::from(17u32)};
        let P1 = point::coor(BigUint::from(5u32),BigUint::from(1u32));
        let P2 = point::identity;
        /*19(5,1)= identity */

        let result = ec.scalar_multiplication(&BigUint::from(19u32),&P1);
        assert_eq!(result,P2);

    }
    #[test]
    fn test_elliptic_scalar_multiplication2(){
        let ec = EllipticCurve1{a:BigUint::from(2u32),b:BigUint::from(2u32),p:BigUint::from(17u32)};
        let P1 = point::coor(BigUint::from(5u32),BigUint::from(1u32));
        let P2 = point::coor(BigUint::from(6u32), BigUint::from(3u32));
        /*2(5,1)= (6,3) */

        let result = ec.scalar_multiplication(&BigUint::from(2u32),&P1);
        assert_eq!(result,P2);

    }
    #[test]
    fn test_elliptic_scalar_multiplication3(){
        let ec = EllipticCurve1{a:BigUint::from(2u32),b:BigUint::from(2u32),p:BigUint::from(17u32)};
        let P1 = point::coor(BigUint::from(5u32),BigUint::from(1u32));
        let P2 = point::coor(BigUint::from(7u32), BigUint::from(11u32));
        /*10(5,1)= (7,11) */

        let result = ec.scalar_multiplication(&BigUint::from(10u32),&P1);
        assert_eq!(result,P2);

    }
    #[test]
    fn test_elliptic_scalar_multiplication4(){
        let ec = EllipticCurve1{a:BigUint::from(2u32),b:BigUint::from(2u32),p:BigUint::from(17u32)};
        let P1 = point::coor(BigUint::from(5u32),BigUint::from(1u32));
        let P2 = point::coor(BigUint::from(10u32), BigUint::from(11u32));
        /*16(5,1)= (10,11) */

        let result = ec.scalar_multiplication(&BigUint::from(16u32),&P1);
        assert_eq!(result,P2);

    }
    #[test]
    fn test_ec_secp26k1(){
        /*y2 =x^3+7 
       p= 0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f
       G = (0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798, 0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8)
       n = 0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141
       a = 0x0000000000000000000000000000000000000000000000000000000000000000
       b = 0x0000000000000000000000000000000000000000000000000000000000000007   */
       let p = BigUint::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",16).unwrap();
       let n = BigUint::parse_bytes(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 
       16).unwrap();
       let gx = BigUint::parse_bytes(b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        16).unwrap();
       let gy = BigUint::parse_bytes(b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 
       16).unwrap();
       let ec = EllipticCurve1{a:BigUint::from(0u32),b:BigUint::from(7u32),p};
       let g =point::coor(gx, gy);
       let res = ec.scalar_multiplication(&n, &g);
       assert_eq!(res,point::identity);



    }





    

    }








