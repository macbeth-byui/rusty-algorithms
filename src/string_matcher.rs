


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test1_build_fsm1() {
//         let fsm = build_fsm("AAC", "ACGT");
//         assert_eq!(fsm, vec![
//             vec![('A', 1), ('C', 0), ('G', 0), ('T', 0)].into_iter().collect(),
//             vec![('A', 2), ('C', 0), ('G', 0), ('T', 0)].into_iter().collect(),
//             vec![('A', 2), ('C', 3), ('G', 0), ('T', 0)].into_iter().collect(),
//             vec![('A', 1), ('C', 0), ('G', 0), ('T', 0)].into_iter().collect(),
//         ]);
//     }

//     #[test]
//     fn test2_build_fsm1() {
//         let fsm = build_fsm("CBCBA", "ABC");
//         assert_eq!(fsm, vec![
//             vec![('A', 0), ('B', 0), ('C', 1)].into_iter().collect(),
//             vec![('A', 0), ('B', 2), ('C', 1)].into_iter().collect(),
//             vec![('A', 0), ('B', 0), ('C', 3)].into_iter().collect(),
//             vec![('A', 0), ('B', 4), ('C', 1)].into_iter().collect(),        
//             vec![('A', 5), ('B', 0), ('C', 3)].into_iter().collect(),        
//             vec![('A', 0), ('B', 0), ('C', 1)].into_iter().collect(),        
//         ]);
//     }

//     #[test]
//     fn test3_match1() {
//         let fsm = build_fsm("AAC", "ACGT");
//         let results = match_pattern("GTAACAGTAAACG", &fsm);
//         assert!(results.is_ok());
//         assert_eq!(results.unwrap(), vec![4, 11]);
//     }

//     #[test]
//     fn test4_match2() {
//         let fsm = build_fsm("AA", "ACGT");
//         let results = match_pattern("GTAACAGTAAACG", &fsm);
//         assert!(results.is_ok());
//         assert_eq!(results.unwrap(), vec![3, 9, 10]);
//     }

//     #[test]
//     fn test5_match3() {
//         let fsm = build_fsm("CBC", "ABC");
//         let results = match_pattern("ABCBCABCBCBC", &fsm);
//         assert!(results.is_ok());
//         assert_eq!(results.unwrap(), vec![4, 9, 11]);
//     }

//     #[test]
//     fn test6_no_matches() {
//         let fsm = build_fsm("AACT", "ACGT");
//         let results = match_pattern("GTAACAGTAAACG", &fsm);
//         assert!(results.is_ok());
//         assert_eq!(results.unwrap(), vec![]);
//     }
// }