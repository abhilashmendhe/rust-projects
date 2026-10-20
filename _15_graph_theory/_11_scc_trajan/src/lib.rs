use std::collections::VecDeque;

use _1_graph::graph::Graph;

pub fn dfs(
    g: &mut Graph,
    at: usize,
    id: &mut i64,
    scc_count: &mut usize,
    ids: &mut Vec<i64>,
    low: &mut Vec<i64>,
    on_stack: &mut Vec<bool>,
    stack: &mut VecDeque<usize>,
) {
    stack.push_back(at);
    on_stack[at] = true;
    *id += 1;
    ids[at] = *id;
    low[at] = *id;

    for edge in &g.vertices[at].edges {
        let to = edge.get_dest_id() as usize;
        if ids[at] == -1 {
            return dfs(g, to as usize, id, scc_count, ids, low, on_stack, stack);
        }
        if on_stack[to] {
            low[at] = low[to].min(low[at]);
        }
    }
    if ids[at] == low[at] {
        while let Some(i) = stack.pop_back() {
            on_stack[i] = false;
            low[i] = ids[i];
            if i == at {
                break;
            }
        }

        *scc_count += 1;
    }
}

pub fn scc(g: &mut Graph) -> Vec<i64> {
    // let unvisited= -1;
    let n = g.vertices.len();
    let mut id = 0;
    let mut scc_count = 0;

    let mut ids = vec![-1; n];
    let mut low = vec![0; n];
    let mut on_stack = vec![false; n];
    let mut stack = VecDeque::<usize>::new();

    for i in 0..n {
        if ids[i] == -1 {
            dfs(
                g,
                i,
                &mut id,
                &mut scc_count,
                &mut ids,
                &mut low,
                &mut on_stack,
                &mut stack,
            );
        }
    }
    println!("{}", scc_count);
    low
}
