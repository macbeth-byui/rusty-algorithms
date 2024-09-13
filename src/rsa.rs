



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test1_euclid_small() {
//         let a = BigInt::from(8);
//         let b = BigInt::from(12);

//         let (gcd, i, j) = euclid(&a, &b);
//         assert_eq!(gcd, BigInt::from(4));
//         assert_eq!(i, BigInt::from(-1));
//         assert_eq!(j, BigInt::from(1));
//     }

//     #[test]
//     fn test2_euclid_coprime_small() {
//         let a = BigInt::from(5);
//         let b = BigInt::from(72);

//         let (gcd, i, j) = euclid(&a, &b);
//         assert_eq!(gcd, BigInt::from(1));
//         assert_eq!(i, BigInt::from(29));
//         assert_eq!(j, BigInt::from(-2)); 
//     }

//     #[test]
//     fn test3_euclid_coprime_big() {
//         let a = BigInt::from(65537);
//         let b = ("87178291199".parse::<BigInt>().unwrap() - 1) * 
//                         ("22815088913".parse::<BigInt>().unwrap() - 1);
        
//         let (gcd, i, j) = euclid(&a, &b);
//         assert_eq!(gcd, BigInt::from(1));
//         assert_eq!(i, "-691197798001282429727".parse::<BigInt>().unwrap());
//         assert_eq!(j, BigInt::from(22775)); 
//     }

//     #[test]
//     fn test4_mod_expo() {
//         let a = BigInt::from(3);
//         let b = BigInt::from(50);
//         let n = BigInt::from(5);
//         let result = mod_expo(&a, &b, &n);
//         assert_eq!(result, BigInt::from(4));
//     }

//     #[test]
//     fn test5_rand_rel_prime() {
//         let phi = ("87178291199".parse::<BigInt>().unwrap() - 1) * 
//                           ("22815088913".parse::<BigInt>().unwrap() - 1);
//         let e = rand_rel_prime(&phi);
//         let (gcd, _, _) = euclid(&e, &phi);
//         assert_eq!(gcd, BigInt::one());
//     }

//     #[test]
//     fn test6_encrypt_decrypt() {
//         let p = "87178291199".parse::<BigInt>().unwrap();
//         let q = "22815088913".parse::<BigInt>().unwrap();
//         let keys = gen_keys(&p, &q);
//         let value = BigInt::from(42);
//         let encrypted = encrypt(&value, &keys);
//         let decrypted = decyprt(&encrypted, &keys);
//         assert_eq!(decrypted, value);
//     }

//     #[test]
//     fn test7_encrypt_decrypt() {
//         let p = "203956878356401977405765866929034577280193993314348263094772646453283062722701277632936616063144088173312372882677123879538709400158306567338328279154499698366071906766440037074217117805690872792848149112022286332144876183376326512083574821647933992961249917319836219304274280243803104015000563790123".parse::<BigInt>().unwrap();
//         let q = "531872289054204184185084734375133399408303613982130856645299464930952178606045848877129147820387996428175564228204785846141207532462936339834139412401975338705794646595487324365194792822189473092273993580587964571659678084484152603881094176995594813302284232006001752128168901293560051833646881436219".parse::<BigInt>().unwrap();
//         let keys = gen_keys(&p, &q);
//         let value = BigInt::from(42);
//         let encrypted = encrypt(&value, &keys);
//         let decrypted = decyprt(&encrypted, &keys);
//         assert_eq!(decrypted, value);
//     }

//     #[test]
//     fn test8_encrypt_decrypt() {
//         let p = "203956878356401977405765866929034577280193993314348263094772646453283062722701277632936616063144088173312372882677123879538709400158306567338328279154499698366071906766440037074217117805690872792848149112022286332144876183376326512083574821647933992961249917319836219304274280243803104015000563790123".parse::<BigInt>().unwrap();
//         let q = "531872289054204184185084734375133399408303613982130856645299464930952178606045848877129147820387996428175564228204785846141207532462936339834139412401975338705794646595487324365194792822189473092273993580587964571659678084484152603881094176995594813302284232006001752128168901293560051833646881436219".parse::<BigInt>().unwrap();
//         let keys = gen_keys(&p, &q);
//         let value = BigInt::from(42);
//         let encrypted = encrypt(&value, &keys);
//         let decrypted = decyprt(&encrypted, &keys);
//         assert_eq!(decrypted, value);
//     }

// }
