use ec_generic::{elliptic_curve::EllipticCurve, Point, finite_fields};
use num_bigint::{BigUint,RandBigInt};
use rand::{self, Rng};
use sha256::{digest,try_digest};
// pub mod lib;
// use lib::{EllipticCurve1,point};
pub struct ECDSA{
    ec_curve:EllipticCurve,
    a_gen:Point,
    q_order:BigUint,
}
impl ECDSA{
    //generate (d, d*A) = (secret key, publickey)
    fn generate_keypair(&self)->(BigUint,Point){
        let private_key = self.generate_privatekey();
        let public_key = self.generate_publickey(private_key.clone());
        (private_key,public_key) 
    }
    
    fn generate_privatekey(&self)->BigUint{
        let private_key = self.generate_positive_random_number_lessthan(self.q_order.clone());
        return private_key
        
    }
    fn generate_publickey(&self,private_key:BigUint)->Point{
        self.ec_curve.scalar_mul(&self.a_gen, &private_key).unwrap()
       
        
    }
    // outputs a random number (0,n)
    fn generate_positive_random_number_lessthan(&self,max:BigUint)->BigUint{
       let mut rng = rand::thread_rng;
       rng().gen_biguint_range(&BigUint::from(1u32),&max)
    }


    //Input:H(m),d(secet_key),random_number-k
    //Output:(x_r,s) Where x_r is the x-cordinate of the R=(x_r,y_r)
    //where R = k*A, A-generator of the elliptic curve 
    //s= (H(m)+x_r.d)k^(-1)
    
    pub fn sign(&self,
        hash:BigUint,
        private_key:BigUint,
        k_random:BigUint)->(BigUint,BigUint){
        assert!(hash < self.q_order,"hash is bigger than the EC group");
        assert!(private_key< self.q_order, "private_key is bigger than the EC group");
        assert!(k_random <self.q_order,"k_random is bigger than the EC group");
        let R:Point = self.ec_curve.scalar_mul(&self.a_gen,&k_random).unwrap();
        if let Point::Coor(r,_) = R{
            let t = finite_fields::FiniteField::mult(&r,&private_key,&self.q_order);
            let l = finite_fields::FiniteField::add(&hash,&t.unwrap(),&self.q_order);
            let k_inverse = finite_fields::FiniteField::inv_mult_prime(&k_random,&self.q_order);
            let s = finite_fields::FiniteField::mult(&l.unwrap(),&k_inverse.unwrap(),&self.q_order);
            return (r,s.unwrap())
        }
        panic!("The random point R shouldn't be the identity");

        }
        pub fn generate_hash_lessthan(&self,message:&str,max:&BigUint) ->BigUint{
            let digest = digest(message);
            let hash_bytes = hex::decode(&digest).expect("couldn't convert hash to vec<u8>");
            let hash = BigUint::from_bytes_be(&hash_bytes);
            let hash = hash.modpow(&BigUint::from(1u32), &(max - BigUint::from(1u32)));
            let hash = hash+BigUint::from(1u32);
            hash

        }
        //u1 = s^(-1)*H(m) mod q
        //u1 = s^(-1)*x_r mod q
        // P = u1 A+u2*B mod q = (xp,yp)
        //if r = xp then verfified


        
    pub fn verification(&self,hash:BigUint,
        public_key:Point,
        signature:(BigUint,BigUint))->bool  {
            assert!(hash <self.q_order,"hash is bigger than the order of the EC Group");
            let (r,s) = signature;
            let s_inverse = s.modpow(&(self.q_order.clone() -BigUint::from(2u32)), &self.q_order);
            let u1 = finite_fields::FiniteField::mult(&s_inverse, &hash, &self.q_order).unwrap();
            let u2 = finite_fields::FiniteField::mult(&s_inverse, &r, &self.q_order).unwrap();
            let u1timesA = self.ec_curve.scalar_mul(&self.a_gen,&u1).unwrap();
            let u2timesB = self.ec_curve.scalar_mul(&public_key,&u2).unwrap();
            let u1timesAplusu2timesB = self.ec_curve.add(&u1timesA, &u2timesB).unwrap();
            if let Point::Coor(xp,_ ) = u1timesAplusu2timesB{
                return xp ==r;
            }
            // let (x,y) = u1timesAplusu2timesB;
            //assert_eq!(x,r)
            panic!("point u1timesAplusu2timesB = u1 A+u2*B can't be the identity");

        
            
        }


}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sign_verify(){
        let ec_curve = EllipticCurve{
            a:BigUint::from(2u32),
            b:BigUint::from(2u32),
            p:BigUint::from(17u32),
        };
        let q_order = BigUint::from(19u32);
        let a_gen = Point::Coor(BigUint::from(5u32),BigUint::from(1u32));
        let ecdsa = ECDSA{ec_curve,a_gen,q_order};
        let private_key = BigUint::from(7u32);
        let public_key = ecdsa.generate_publickey(private_key.clone());
        //let hash = BigUint::from(10u32);
        let k_random = BigUint::from(18u32);
        let message = "Manish send 1BTC to Sai";
        let hash = ecdsa.generate_hash_lessthan(message,&ecdsa.q_order);
        let signature = ecdsa.sign(hash.clone(),private_key,k_random);
        //println!("{:?}",signature);
        
        let verify_result = ecdsa.verification(hash, public_key, signature);
        assert!(verify_result,"verification result is false");



    }
    #[test]
    fn test_sign_verify_tampermessage(){
        let ec_curve = EllipticCurve{
            a:BigUint::from(2u32),
            b:BigUint::from(2u32),
            p:BigUint::from(17u32),
        };
        let q_order = BigUint::from(19u32);
        let a_gen = Point::Coor(BigUint::from(5u32),BigUint::from(1u32));
        let ecdsa = ECDSA{ec_curve,a_gen,q_order};
        let private_key = BigUint::from(7u32);
        let public_key = ecdsa.generate_publickey(private_key.clone());
        //let hash = BigUint::from(10u32);
        let k_random = BigUint::from(18u32);
        let message = "Manish send 1BTC to Sai";
        let hash = ecdsa.generate_hash_lessthan(message,&ecdsa.q_order);
        let signature = ecdsa.sign(hash.clone(),private_key,k_random);
        //println!("{:?}",signature);
        let message = "Manish send 2BTC to Sai";
        let hash = ecdsa.generate_hash_lessthan(message,&ecdsa.q_order);
        let verify_result = ecdsa.verification(hash, public_key, signature);
        assert!(!verify_result,"verification should fail when message is tempered");
        


    }
    #[test]
    fn test_sign_verify_tamper_sign(){
        let ec_curve = EllipticCurve{
            a:BigUint::from(2u32),
            b:BigUint::from(2u32),
            p:BigUint::from(17u32),
        };
        let q_order = BigUint::from(19u32);
        let a_gen = Point::Coor(BigUint::from(5u32),BigUint::from(1u32));
        let ecdsa = ECDSA{ec_curve,a_gen,q_order};
        let private_key = BigUint::from(7u32);
        let public_key = ecdsa.generate_publickey(private_key.clone());
        //let hash = BigUint::from(10u32);
        let k_random = BigUint::from(18u32);
        let message = "Manish send 1BTC to Sai";
        let hash = ecdsa.generate_hash_lessthan(message,&ecdsa.q_order);
        let signature = ecdsa.sign(hash.clone(),private_key,k_random);
        //println!("{:?}",signature);
        let (r,s) = signature;
        let tempered_sign = ((r+BigUint::from(1u32)).modpow(&BigUint::from(1u32), &ecdsa.q_order),s) ;
       
        let verify_result = ecdsa.verification(hash, public_key, tempered_sign);
        assert!(!verify_result,"verification should fail when signature is tempered");
        


    }
    #[test]
    fn test_secp256_sign_verify(){
       let p = BigUint::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",16).unwrap();
       let q_order = BigUint::parse_bytes(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 
       16).unwrap();
       let gx = BigUint::parse_bytes(b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        16).unwrap();
       let gy = BigUint::parse_bytes(b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 
       16).unwrap();
       let ec_curve = EllipticCurve{a:BigUint::from(0u32),b:BigUint::from(7u32),p};
       let a_gen =Point::Coor(gx, gy);
       let ecdsa = ECDSA{
        ec_curve,
        a_gen,
        q_order,
       };
       let private_key = BigUint::parse_bytes(b"483ada7726a3c4655da4fbfc0e110a8fd189748a68094199c2cde8fb1c044b0", 
       16).unwrap();
       let public_key = ecdsa.generate_publickey(private_key.clone());
       //let hash = BigUint::from(10u32);
       let k_random = BigUint::parse_bytes(b"79be667098dab0ac95d03295ce870a17029bfcdb2dce28d959f2815b16f81798",
       16).unwrap();
       let message = "Manish send 1BTC to Sai";
       let hash = ecdsa.generate_hash_lessthan(message,&ecdsa.q_order);
       let signature = ecdsa.sign(hash.clone(),private_key,k_random);
       //println!("{:?}",signature);
       
       let verify_result = ecdsa.verification(hash, public_key, signature);
       assert!(verify_result,"verification result is false");


       
    }
    #[test]
    fn test_secp256_sign_verification_temper_message(){
       let p = BigUint::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",16).unwrap();
       let q_order = BigUint::parse_bytes(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 
       16).unwrap();
       let gx = BigUint::parse_bytes(b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        16).unwrap();
       let gy = BigUint::parse_bytes(b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 
       16).unwrap();
       let ec_curve = EllipticCurve{a:BigUint::from(0u32),b:BigUint::from(7u32),p};
       let a_gen =Point::Coor(gx, gy);
       let ecdsa = ECDSA{
        ec_curve,
        a_gen,
        q_order,
       };
       let private_key = BigUint::parse_bytes(b"483ada7726a3c4655da4fbfc0e110a8fd189748a68094199c2cde8fb1c044b0", 
       16).unwrap();
       let public_key = ecdsa.generate_publickey(private_key.clone());
       //let hash = BigUint::from(10u32);
       let k_random = BigUint::parse_bytes(b"79be667098dab0ac95d03295ce870a17029bfcdb2dce28d959f2815b16f81798",
       16).unwrap();
       let message = "Manish send 1BTC to Sai";
       let hash = ecdsa.generate_hash_lessthan(message,&ecdsa.q_order);
       let signature = ecdsa.sign(hash.clone(),private_key,k_random);
       //println!("{:?}",signature);
       let message = "Manish send 2BTC to Sai";
       let hash = ecdsa.generate_hash_lessthan(message,&ecdsa.q_order);
       
       let verify_result = ecdsa.verification(hash, public_key, signature);
       assert!(!verify_result,"verification should have failed due to tempered message");


       
    }
    #[test]
    fn test_secp256_sign_verification_temper_sigature(){
       let p = BigUint::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",16).unwrap();
       let q_order = BigUint::parse_bytes(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 
       16).unwrap();
       let gx = BigUint::parse_bytes(b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        16).unwrap();
       let gy = BigUint::parse_bytes(b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 
       16).unwrap();
       let ec_curve = EllipticCurve{a:BigUint::from(0u32),b:BigUint::from(7u32),p};
       let a_gen =Point::Coor(gx, gy);
       let ecdsa = ECDSA{
        ec_curve,
        a_gen,
        q_order,
       };
       let private_key = BigUint::parse_bytes(b"483ada7726a3c4655da4fbfc0e110a8fd189748a68094199c2cde8fb1c044b0", 
       16).unwrap();
       let public_key = ecdsa.generate_publickey(private_key.clone());
       //let hash = BigUint::from(10u32);
       let k_random = BigUint::parse_bytes(b"79be667098dab0ac95d03295ce870a17029bfcdb2dce28d959f2815b16f81798",
       16).unwrap();
       let message = "Manish send 1BTC to Sai";
       let hash = ecdsa.generate_hash_lessthan(message,&ecdsa.q_order);
       let signature = ecdsa.sign(hash.clone(),private_key,k_random);
       //println!("{:?}",signature);
       let (r,s) = signature;
       let tempered_sign = ((r+BigUint::from(1u32)).modpow(&BigUint::from(1u32), &ecdsa.q_order),s) ;

       let verify_result = ecdsa.verification(hash, public_key, tempered_sign);
       assert!(!verify_result,"verification should have failed due to tempered signature");


       
    }
    









}
fn main(){}



