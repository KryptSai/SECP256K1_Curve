use ec_generic::{elliptic_curve::EllipticCurve, Point, finite_fields};
use num_bigint::{BigUint,RandBigInt};
use rand::{self, Rng};
pub struct ecdh{
    ec_curve:EllipticCurve,
    a_gen:Point,
    q_order:BigUint,
}
impl ecdh {
     //generate (d, d*A) = (secret key, publickey)
     fn individual_comp(&self)->(Point){
        let private_key = self.generate_privatekey();
        let public_key = self.generate_publickey(&private_key);
        public_key 
    }
    #[allow(dead_code)]
    fn generate_privatekey(&self)->BigUint{
        let private_key = self.generate_positive_random_number_lessthan(self.q_order.clone());
        return private_key
        
    }
    fn generate_publickey(&self,private_key:&BigUint)->Point{
        self.ec_curve.scalar_mul(&self.a_gen, &private_key).unwrap() 
        
    }
    // outputs a random number (0,n)
    fn generate_positive_random_number_lessthan(&self,max:BigUint)->BigUint{
       let mut rng = rand::thread_rng;
       rng().gen_biguint_range(&BigUint::from(1u32),&max)
    }
    //
    fn generate_common_comp(&self,private_key:&BigUint,individual_comp:&Point) -> Point{
        let common_comp = self.ec_curve.scalar_mul(&individual_comp,&private_key).unwrap();
        return common_comp

    }

} 
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_priv_pub_key(){
        let ec_curve = EllipticCurve{
            a:BigUint::from(2u32),
            b:BigUint::from(2u32),
            p:BigUint::from(17u32),
        };
        let q_order = BigUint::from(19u32);
        let a_gen = Point::Coor(BigUint::from(5u32),BigUint::from(1u32));
        let ecdh = ecdh{ec_curve,a_gen,q_order };
        let private_key = ecdh.generate_privatekey();
        let public_key = ecdh.generate_publickey(&private_key);
        println!("{}",&private_key);
        assert!(private_key < ecdh.q_order,"priv_key_test fails");


    }
    #[test]
    fn test_common_compoenent(){
        let ec_curve = EllipticCurve{
            a:BigUint::from(2u32),
            b:BigUint::from(2u32),
            p:BigUint::from(17u32),
        };
        let q_order = BigUint::from(19u32);
        let a_gen = Point::Coor(BigUint::from(5u32),BigUint::from(1u32));
        let ecdh = ecdh{ec_curve,a_gen,q_order };
        let alice_Priv_key = ecdh.generate_privatekey();
        println!("Alice private key is {:?}", alice_Priv_key);
        let bob_priv_key = ecdh.generate_privatekey();
        println!("Bob private key is {:?}", bob_priv_key);
        let alice_pub_key = ecdh.generate_publickey(&alice_Priv_key);
        let bob_pub_key = ecdh.generate_publickey(&bob_priv_key);
        let alice_com_comp = ecdh.generate_common_comp(&alice_Priv_key,&bob_pub_key);
        println!("Alice common component is {:?}", alice_com_comp);
        let bob_com_comp = ecdh.generate_common_comp(&bob_priv_key,&alice_pub_key);
        println!("Bob common component is {:?}", bob_com_comp);
        assert_eq!(alice_com_comp, bob_com_comp);

    }
    #[test] 
    fn test_ec_common_compoenent(){
        let p = BigUint::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",16).unwrap();
        let q_order = BigUint::parse_bytes(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 
        16).unwrap();
        let gx = BigUint::parse_bytes(b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
         16).unwrap();
        let gy = BigUint::parse_bytes(b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 
        16).unwrap();
        let ec_curve = EllipticCurve{a:BigUint::from(0u32),b:BigUint::from(7u32),p};
        let a_gen =Point::Coor(gx, gy);
        let ecdh = ecdh{
            ec_curve,
            a_gen,
            q_order,
           };
        let alice_Priv_key = ecdh.generate_privatekey();
        println!("Alice private key is {:?}", alice_Priv_key);
        let bob_priv_key = ecdh.generate_privatekey();
        println!("Bob private key is {:?}", bob_priv_key);
        let alice_pub_key = ecdh.generate_publickey(&alice_Priv_key);
        let bob_pub_key = ecdh.generate_publickey(&bob_priv_key);
        let alice_com_comp = ecdh.generate_common_comp(&alice_Priv_key,&bob_pub_key);
        println!("Alice common component is {:?}", alice_com_comp);
        let bob_com_comp = ecdh.generate_common_comp(&bob_priv_key,&alice_pub_key);
        println!("Bob common component is {:?}", bob_com_comp);
        assert_eq!(alice_com_comp, bob_com_comp);

    }



}
