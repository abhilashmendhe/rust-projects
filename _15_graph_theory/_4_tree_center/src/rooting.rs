
use _1_graph::graph::Graph;

pub fn tree_center(g: &mut Graph) {

    // We create two arrays that stores all the vertex edge counts and leaves
    let n = g.vertices.len();
    let mut degree = vec![0 as i32; n];
    let mut leaves = vec![];

    let vertices = &g.vertices;
    for (ind, v) in vertices.iter().enumerate() {
        degree[ind] = v.get_edges().len() as i32;
        if degree[ind] == 0 || degree[ind] == 1 {
            leaves.push(ind);
            degree[ind] = 0;
        }
    }

    let mut count = leaves.len();
    

    while count < n {
    // println!("{:?}",degree);
    // println!("{:?}\n",leaves);
        let mut new_leaves = vec![];          
        for l in leaves {
            for e in &vertices[l].edges {
                // print!("{} ",e.get_dest_id());
                degree[e.get_dest_id() as usize] -= 1;
                if degree[e.get_dest_id() as usize] == 1{
                    new_leaves.push(e.get_dest_id() as usize);
                }
            }
            degree[l] = 0;
        }
        count += new_leaves.len();
        leaves = new_leaves;
    }
    // println!("{:?}",degree);
    println!("{:?}",leaves);

}


