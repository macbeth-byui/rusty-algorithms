

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test1_no_negative_cycle() {
//         let mut graph = Graph::new(5);
//         let _ = graph.add_edge(0, 1, 6.0);
//         let _ = graph.add_edge(0, 3, 7.0);
//         let _ = graph.add_edge(1, 2, 5.0);
//         let _ = graph.add_edge(1, 3, 8.0);
//         let _ = graph.add_edge(1, 4, -4.0);
//         let _ = graph.add_edge(2, 1, -2.0);
//         let _ = graph.add_edge(3, 2, -3.0);
//         let _ = graph.add_edge(3, 4, 9.0);
//         let _ = graph.add_edge(4, 0, 2.0);
//         let _ = graph.add_edge(4, 2, 7.0);

//         let result = shortest_path(&graph, 0);
//         assert!(result.is_ok());
//         let (dist,pred) = result.unwrap();
//         assert_eq!(dist, vec![0.0, 2.0, 4.0, 7.0, -2.0]);
//         assert_eq!(pred, vec![None, Some(2), Some(3), Some(0), Some(1)]);
//     }

//     #[test]
//     fn test2_negative_cycle() {
//         let mut graph = Graph::new(5);
//         let _ = graph.add_edge(0, 1, 6.0);
//         let _ = graph.add_edge(0, 3, 7.0);
//         let _ = graph.add_edge(1, 2, 5.0);
//         let _ = graph.add_edge(1, 3, -1.0);
//         let _ = graph.add_edge(1, 4, -4.0);
//         let _ = graph.add_edge(2, 1, -2.0);
//         let _ = graph.add_edge(3, 2, -3.0);
//         let _ = graph.add_edge(3, 4, 9.0);
//         let _ = graph.add_edge(4, 0, 2.0);
//         let _ = graph.add_edge(4, 2, 7.0);

//         let result = shortest_path(&graph, 0);
//         assert_eq!(result, Err(GraphError::NegativeCycle));
//     }
// }