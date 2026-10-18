use _1_graph::{edges::etrait::EdgeTrait, graph::Graph};

pub fn bellman_ford(g: &Graph, edge_list: Vec<Box<dyn EdgeTrait>>) -> Vec<i64> {
    let _e = edge_list.len();
    let v = g.vertices.len();
    let s = 0;
    let mut d = vec![i64::MAX; v];

    d[s] = 0;

    // Relax each edge V-1 times
    for _ in 0..v - 1 {
        for edge in &edge_list {
            let src_id = edge.get_from_id();
            let dst_id = edge.get_dest_id();
            let e_weight = edge.get_weight().unwrap();
            if d[src_id as usize] + e_weight < d[dst_id as usize] {
                d[dst_id as usize] = d[src_id as usize] + e_weight;
            }
        }
    }
    // println!("{:?}", d);
    // Now again do the process to find nodes caught in a -ve cycles
    for _ in 0..v - 1 {
        for edge in &edge_list {
            let src_id = edge.get_from_id();
            let dst_id = edge.get_dest_id();
            let e_weight = edge.get_weight().unwrap();
            // println!("{:?}", d[src_id as usize].checked_add(e_weight));
            if let Some(sum_value) = d[src_id as usize].checked_add(e_weight) {
                if sum_value < d[dst_id as usize] {
                    d[dst_id as usize] = i64::MIN;
                }
            } else {
                d[dst_id as usize] = i64::MIN;
            }
        }
    }
    // println!("{:?}", d);
    d
}
