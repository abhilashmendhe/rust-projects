use _1_graph::graph::Graph;

/**
 * In kruskal MST.
 * 
 * First get the sorted(ascending order) edges by weight.
 * 
 * Then start picking by the first edge and compute union-find
 */

fn find(mut node: usize, parent: &mut [usize]) -> usize {
    
    while node != parent[node] {
        // println!("n: {}", node);
        parent[node] = parent[parent[node]];
        node = parent[node];
    }
    node
}

pub fn compute_disjoint_set(g: &Graph, edges: Vec<(i64, u32, u32)>) {
    
    let v_len = g.vertices.len();
    let mut parent = (0..g.vertices.len()).collect::<Vec<_>>();
    let mut size = vec![1_usize; g.vertices.len()];
    let mut mst_edges = vec![];
    println!("Before -> {:?}", parent);
    println!("Before -> {:?}", size);
    let mut iter = 0;
    for (we, from, to) in edges {
        if mst_edges.len() >= v_len - 1 {
            break;
        }
        iter += 1;
        let r1 = find(from as usize, &mut parent);
        let r2 = find(to as usize, &mut parent);
        if r1 != r2 {
            let r1_size = size[r1];
            let r2_size = size[r2];
            if r1_size >= r2_size {
                parent[r2] = r1;
                size[r1] += size[r2];
            } else {
                parent[r1] = r2;
                size[r2] += size[r1];
            }
            mst_edges.push((we, from, to));
        }
    }
    println!("After -> {:?}", parent);
    println!("After -> {:?}", size);
    println!("Total iter: {}", iter);
    println!("{:?}", mst_edges);
}