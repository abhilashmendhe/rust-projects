use _1_graph::graph::Graph;

pub fn encode_tree(g: &mut Graph) -> String {

    let vertices = &g.vertices;
    let mut degree = vec![0 as i32; vertices.len()];
    let mut codes = vec!["".to_string(); vertices.len()];
    let mut leaves = vec![];

    for (ind, v) in vertices.iter().enumerate() {
        degree[ind] = v.get_edges().len() as i32;

        if v.get_edges().len() == 1 || v.get_edges().len() == 0 {
            leaves.push(ind);
            // codes[ind] = "()";
            degree[ind] = 0;
        }
    }

    let mut count = leaves.len();
    
    while count < vertices.len() {
        let mut new_leaves = vec![];
        for l in leaves {
            let nstr = format!("({})", codes[l]);
            for e in &vertices[l].edges {
                // print!("{} - ", e.get_dest_id());
                let mut pstr = format!("{}{}", &nstr, codes[e.get_dest_id() as usize]);
                if nstr > codes[e.get_dest_id() as usize] {
                    pstr = format!("{}{}",codes[e.get_dest_id() as usize], &nstr);
                }
                degree[e.get_dest_id() as usize] -= 1;
                codes[e.get_dest_id() as usize] = pstr;
                if degree[e.get_dest_id() as usize] == 1 {
                    new_leaves.push(e.get_dest_id() as usize);
                    
                }
            }
            degree[l] = 0;
        }
        count += new_leaves.len();
        leaves = new_leaves;
        
    }
    
    if codes[0] > codes[1] {
        format!("({}{})", codes[1], codes[0])
    } else {
        format!("({}{})", codes[0], codes[1])
    }
}