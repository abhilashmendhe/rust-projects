use std::fmt::Display;

use crate::{edge::Edge, vertex::Vertex};

#[derive(Debug)]
pub struct Graph {
    pub vertices: Vec<Vertex>
}

impl Graph {
    pub fn new() -> Self {
        // let mut vertices = Vertex::new(state_id, state_name)
        Self {
            vertices: vec![]
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {

        let f: bool = self.vertices.contains(&vertex);
        if !f {
            self.vertices.push(vertex);
        } 
    }

    pub fn add_edge_by_id(&mut self, id1: u32, id2: u32, weight: i32) {

        let mut v1 = None;
        let mut v2 = None;

        for v in self.vertices.iter_mut() {
            if v.get_id() == id1 {
                v1 = Some(v);
            } else if v.get_id() == id2 {
                v2 = Some(v);
            }
            if v1.is_some() && v2.is_some() {
                break; // Exit early when both are found
            }
        }

        if let (Some(v1), Some(v2)) = (v1, v2) {
            let e1 = Edge::new(id1, weight);
            let e2 = Edge::new(id2, weight);

            if !v1.edge_list.contains(&e2) && !v2.edge_list.contains(&e1) {
                v1.add_edge(e2);
                v2.add_edge(e1);
            } else {
                println!("Edge already exists between {} and {}", id1, id2);
            }
        }
        // let mut pos_indexes = vec![];

        // for (ind, v) in self.vertices.iter().enumerate() {
        //     if v.get_id() == id1 || v.get_id() == id2 {
        //         pos_indexes.push(ind);
        //     }
        // }
        // // println!("{:?}",vertices_exists);
        // if pos_indexes.len() > 1 {

        //     let (v1, v2) = self.vertices.split_at_mut(*pos_indexes.get(1).unwrap());
        //     let v1 = &mut v1[*pos_indexes.get(0).unwrap()];
        //     let v2 = &mut v2[0];
            
        //     let e1 = Edge::new(id1, weight);
        //     let v_e1 = v1.edge_list.contains(&e1);

        //     let e2 = Edge::new(id2, weight);
        //     let v_e2 = v2.edge_list.contains(&e2);
        //     // println!()
        //     if !v_e1 && !v_e2{
        //         v1.add_edge(e2);
        //         v2.add_edge(e1);
        //     } else {
        //         println!("Edge already exists between {} and {} vertices", id1, id2);
        //     }
            
        // } else {
        //     println!("One or none of the vertices exists!");
        // }
        
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in &self.vertices {
            println!("{}", v.get_id());
        }
        write!(f, "{}", "")
    }
}