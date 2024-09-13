



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test1_partition_position_middle() {
//         let mut data = vec![3, 8, 1, 9, 2, 0, 5, 7, 6, 4];
//         let result = partition(&mut data);
//         assert_eq!(data, vec![3, 1, 2, 0, 4, 9, 5, 7, 6, 8]);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn test2_partition_position_start() {
//         let mut data = vec![3, 8, 1, 4, 2, 9, 5, 7, 6, 0];
//         let result = partition(&mut data);
//         assert_eq!(data, vec![0, 8, 1, 4, 2, 9, 5, 7, 6, 3]);
//         assert_eq!(result, 0);
//     }

//     #[test]
//     fn test3_partition_position_end() {
//         let mut data = vec![3, 8, 1, 4, 2, 0, 5, 7, 6, 9];
//         let result = partition(&mut data);
//         assert_eq!(data, vec![3, 8, 1, 4, 2, 0, 5, 7, 6, 9]);
//         assert_eq!(result, 9);
//     }

//     #[test]
//     fn test4_rand_pivot() {
//         let mut data = vec![1, 2, 3, 4, 5];
//         rand_pivot(&mut data);
//         assert!(data.contains(&1));
//         assert!(data.contains(&2));
//         assert!(data.contains(&3));
//         assert!(data.contains(&4));
//         assert!(data.contains(&5));
//         assert_eq!(data.len(), 5);
//     }

//     #[test]
//     fn test5_sort() {
//         let mut data = vec![3, 5, 2, 6, 1, 4];
//         sort(&mut data);
//         assert_eq!(data, vec![1, 2, 3, 4, 5, 6]);
//     }

//     #[test]
//     fn test6_already_sorted() {
//         let mut data = vec![1, 2, 3, 4, 5, 6];
//         sort(&mut data);
//         assert_eq!(data, vec![1, 2, 3, 4, 5, 6]);
//     }

//     #[test]
//     fn test7_empty_sort() {
//         let mut data: Vec<i32> = vec![];
//         sort(&mut data);
//         assert_eq!(data.len(), 0);
//     }




// }
