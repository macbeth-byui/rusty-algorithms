


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test1_orientation_convex() {
//         let a = Point {x:0.0, y:0.0};
//         let b = Point {x:4.0, y:0.0};
//         let c = Point {x:3.0, y:1.0};
//         let result = orientation(&a, &b, &c);
//         assert_eq!(result, Angle::Convex)
//     }

//     #[test]
//     fn test2_orientation_concave() {
//         let a = Point {x:4.0, y:0.0};
//         let b = Point {x:3.0, y:1.0};
//         let c = Point {x:8.0, y:8.0};
//         let result = orientation(&a, &b, &c);
//         assert_eq!(result, Angle::Concave)
//     }

//     #[test]
//     fn test3_orientation_colinear() {
//         let a = Point {x:0.0, y:0.0};
//         let b = Point {x:1.0, y:1.0};
//         let c = Point {x:8.0, y:8.0};
//         let result = orientation(&a, &b, &c);
//         assert_eq!(result, Angle::Colinear)
//     }

//     #[test]
//     fn test4_get_angle() {
//         let a = Point {x:1.0, y:2.0};
//         let b = Point {x:4.0, y:7.0};
//         let result = get_angle(&a, &b);
//         assert!((result - 1.030).abs() <= TOLERANCE)
//     }

//     #[test]
//     fn test5_get_dist() {
//         let a = Point {x:1.0, y:2.0};
//         let b = Point {x:4.0, y:7.0};
//         let result = get_dist(&a, &b);
//         assert!((result - 5.831).abs() <= TOLERANCE)
//     }

//     #[test]
//     fn test6_gen_hull() {
//         let points = vec![
//             Point {x:0.0, y:0.0},
//             Point {x:4.0, y:0.0},
//             Point {x:3.0, y:1.0},
//             Point {x:1.0, y:1.0},
//             Point {x:8.0, y:8.0},
//             Point {x:3.0, y:6.0},
//             Point {x:1.0, y:4.0},
//             Point {x:1.0, y:3.0},
//             Point {x:0.0, y:4.0},
//             Point {x:0.0, y:2.0},
//             Point {x:5.5, y:7.0},
//         ];
//         let hull = gen_hull(&points);
//         assert!(hull.is_some());
//         assert_eq!(hull.unwrap(), vec![
//             Point {x:0.0, y:0.0},
//             Point {x:4.0, y:0.0},
//             Point {x:8.0, y:8.0},
//             Point {x:3.0, y:6.0},
//             Point {x:0.0, y:4.0},
//             Point {x:0.0, y:0.0},
//         ]);
//     }

//     #[test]
//     fn test7_gen_hull_too_small() {
//         let points = vec![
//             Point {x:0.0, y:0.0},
//             Point {x:4.0, y:0.0},
//         ];
//         let hull = gen_hull(&points);
//         assert!(hull.is_none());
//     }

//     #[test]
//     fn test8_gen_hull_too_small_1() {
//         let points = vec![
//             Point {x:0.0, y:0.0},
//         ];
//         let hull = gen_hull(&points);
//         assert!(hull.is_none());
//     }

//     #[test]
//     fn test9_gen_hull_too_small_0() {
//         let points = vec![];
//         let hull = gen_hull(&points);
//         assert!(hull.is_none());
//     }

//     #[test]
//     fn test10_gen_hull_all_colinear() {
//         let points = vec![
//             Point {x:0.0, y:0.0},
//             Point {x:1.0, y:1.0},
//             Point {x:2.0, y:2.0},
//             Point {x:3.0, y:3.0},
//             Point {x:4.0, y:4.0},
//         ];
//         let hull = gen_hull(&points);
//         assert!(hull.is_none());
//     }

//     #[test]
//     fn test11_gen_hull_colinear_at_start() {
//         let points = vec![
//             Point {x:0.0, y:0.0},
//             Point {x:1.0, y:0.0},
//             Point {x:2.0, y:0.0},
//             Point {x:3.0, y:0.0},
//             Point {x:4.0, y:0.0},
//             Point {x:3.0, y:1.0},
//             Point {x:1.0, y:1.0},
//             Point {x:8.0, y:8.0},
//             Point {x:3.0, y:6.0},
//             Point {x:1.0, y:4.0},
//             Point {x:1.0, y:3.0},
//             Point {x:0.0, y:4.0},
//             Point {x:0.0, y:2.0},
//             Point {x:5.5, y:7.0},
//         ];
//         let hull = gen_hull(&points);
//         assert!(hull.is_some());
//         assert_eq!(hull.unwrap(), vec![
//             Point {x:0.0, y:0.0},
//             Point {x:4.0, y:0.0},
//             Point {x:8.0, y:8.0},
//             Point {x:3.0, y:6.0},
//             Point {x:0.0, y:4.0},
//             Point {x:0.0, y:0.0},
//         ]);
//     }


// }