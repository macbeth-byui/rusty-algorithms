




// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test1_profile() {
//         let text = "the rain in spain stays mainly in the plan";
//         let profiled_text = profile(text);
//         assert_eq!(profiled_text,
//             vec![(' ', 8), ('a', 5), ('e', 2), ('h', 2),
//                  ('i', 5), ('l', 2), ('m', 1), ('n', 6),
//                  ('p', 2), ('r', 1), ('s', 3), ('t', 3),
//                  ('y', 2)]
//         );
//     }

//     #[test]
//     fn test2_build_tree() {
//         let profiled_text = 
//             vec![(' ', 8), ('a', 5), ('e', 2), ('h', 2),
//                  ('i', 5), ('l', 2), ('m', 1), ('n', 6),
//                  ('p', 2), ('r', 1), ('s', 3), ('t', 3),
//                  ('y', 2)];
//         let tree = build_tree(&profiled_text);
//         assert!(tree.is_some());
//         assert_eq!(tree.unwrap().count, 42);
//     }


//     #[test]
//     fn test3_create_encoding() {
//         let tree = Some(Tree {count: 12, root :
//             Box::new(Node::Support(
//                 Box::new(Node::Support(
//                     Box::new(Node::Leaf('d')),
//                     Box::new(Node::Leaf('b')),
//                 )),
//                 Box::new(Node::Support(
//                     Box::new(Node::Support(
//                         Box::new(Node::Leaf('e')),
//                         Box::new(Node::Leaf('a'))
//                     )),
//                     Box::new(Node::Leaf('c'))
//                 )),
//             )) 
//         });
//         let encoding = create_encoding(&tree);
//         let expected = vec![
//             ('a',bitvec![1,0,1]),
//             ('b',bitvec![0,1]),
//             ('c',bitvec![1,1]),
//             ('d',bitvec![0,0]),
//             ('e',bitvec![1,0,0])].into_iter().collect();
//         assert_eq!(encoding, expected);
//     }

//     #[test]
//     fn test4_create_encoding_empty() {
//         let encoding = create_encoding(&None);
//         assert_eq!(encoding, HashMap::new());
//     }

//     #[test]
//     fn test5_encode() {
//         let encoding_map = vec![
//             ('a',bitvec![1,0,1]),
//             ('b',bitvec![0,1]),
//             ('c',bitvec![1,1]),
//             ('d',bitvec![0,0]),
//             ('e',bitvec![1,0,0])].into_iter().collect();
//         let text = "abcde";
//         let encoded_text = encode(text, &encoding_map);
//         assert!(encoded_text.is_some());
//         assert_eq!(encoded_text.unwrap(),bitvec![1,0,1,0,1,1,1,0,0,1,0,0]);
//     }

//     #[test]
//     fn test6_encode_invalid() {
//         let encoding_map = vec![
//             ('a',bitvec![1,0,1]),
//             ('b',bitvec![0,1]),
//             ('c',bitvec![1,1]),
//             ('d',bitvec![0,0]),
//             ('e',bitvec![1,0,0])].into_iter().collect();
//         let text = "abczde";
//         let encoded_text = encode(text, &encoding_map);
//         assert!(encoded_text.is_none());
//     }

//     #[test]
//     fn test7_decode() {
//         let tree = Some(Tree {count: 12, root :
//             Box::new(Node::Support(
//                 Box::new(Node::Support(
//                     Box::new(Node::Leaf('d')),
//                     Box::new(Node::Leaf('b')),
//                 )),
//                 Box::new(Node::Support(
//                     Box::new(Node::Support(
//                         Box::new(Node::Leaf('e')),
//                         Box::new(Node::Leaf('a'))
//                     )),
//                     Box::new(Node::Leaf('c'))
//                 )),
//             )) 
//         });
//         let encoded_text = bitvec![1,0,1,0,1,1,1,0,0,1,0,0];
//         let decoded_text = decode(&encoded_text, &tree);
//         assert!(decoded_text.is_some());
//         assert_eq!(decoded_text.unwrap(),"abcde");
//     }

//     #[test]
//     fn test8_decode_invalid() {
//         let tree = Some(Tree {count: 12, root :
//             Box::new(Node::Support(
//                 Box::new(Node::Support(
//                     Box::new(Node::Leaf('d')),
//                     Box::new(Node::Leaf('b')),
//                 )),
//                 Box::new(Node::Support(
//                     Box::new(Node::Support(
//                         Box::new(Node::Leaf('e')),
//                         Box::new(Node::Leaf('a'))
//                     )),
//                     Box::new(Node::Leaf('c'))
//                 )),
//             )) 
//         });
//         let encoded_text = bitvec![1,0,1,0,1,1,1,0,0,1,1,1];
//         let decoded_text = decode(&encoded_text, &tree);
//         assert!(decoded_text.is_none());
//     }

//     #[test]
//     fn test9_decode_invalid_single() {
//         let tree = Some(Tree {count: 12, root :
//             Box::new(Node::Leaf('a'))
//         });
//         let encoded_text = bitvec![0,0,0,0,1,0,0,0,0,0];
//         let decoded_text = decode(&encoded_text, &tree);
//         assert!(decoded_text.is_none());
//     }

//     #[test]
//     fn test10_encode_decode() {
//         let text = "the rain in spain stays mainly in the plan";
//         let profiled_text = profile(text);
//         let tree = build_tree(&profiled_text);
//         let encoding = create_encoding(&tree);
//         let encoded_bits = encode(text, &encoding);
//         assert!(encoded_bits.is_some());
//         let decoded_text = decode(&encoded_bits.unwrap(), &tree);
//         assert!(decoded_text.is_some());
//         assert_eq!(text, decoded_text.unwrap());
//     }

//     #[test]
//     fn test11_encode_decode_single() {
//         let text = "aaaaaa";
//         let profiled_text = profile(text);
//         let tree = build_tree(&profiled_text);
//         let encoding = create_encoding(&tree);
//         let encoded_bits = encode(text, &encoding);
//         assert!(encoded_bits.is_some());
//         let decoded_text = decode(&encoded_bits.unwrap(), &tree);
//         assert!(decoded_text.is_some());
//         assert_eq!(text, decoded_text.unwrap());
//     }

//     #[test]
//     fn test12_encode_decode_empty() {
//         let text = "";
//         let profiled_text = profile(text);
//         let tree = build_tree(&profiled_text);
//         let encoding = create_encoding(&tree);
//         let encoded_bits = encode(text, &encoding);
//         assert!(encoded_bits.is_some());
//         let decoded_text = decode(&encoded_bits.unwrap(), &tree);
//         assert!(decoded_text.is_some());
//         assert_eq!(text, decoded_text.unwrap());
//     }

// }


