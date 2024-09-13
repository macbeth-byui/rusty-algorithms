



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_dag_shortest_path() {
//         let mut graph = Graph::new(9);
//         let _ = graph.add_edge(0, 3, 3.0);
//         let _ = graph.add_edge(1, 3, 2.0);
//         let _ = graph.add_edge(2, 5, 5.0);
//         let _ = graph.add_edge(3, 6, 9.0);
//         let _ = graph.add_edge(5, 6, 3.0);
//         let _ = graph.add_edge(4, 8, 7.0);
//         let _ = graph.add_edge(6, 8, 7.0);
//         let _ = graph.add_edge(7, 8, 5.0);

//         let result = shortest_path(&graph, 2);
//         assert!(result.is_ok());
//         let (dist,pred) = result.unwrap();
//         assert_eq!(dist, vec![INF, INF, 0.0, INF, INF, 5.0, 8.0, INF, 15.0]);
//         assert_eq!(pred, vec![None, None, None, None, None, Some(2), Some(5), None, Some(6)]);
//     }
// }