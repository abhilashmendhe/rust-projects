use std::fmt::Display;
use crate::vertex::Vertex;
use crate::edges::edge::Edge;

#[derive(Debug, Clone)]
pub struct Graph {
    pub vertices: Vec<Vertex>,
    pub edge_type: Edge
}

impl Graph {
    pub fn new(etype: Edge) -> Graph {
        Graph { vertices: vec![], edge_type: etype }
    }

    pub fn add_vertex(&mut self, v_id: u32, v_name: String) {
        let v = Vertex::new(v_id, v_name);
        self.vertices.push(v);
    }

    pub fn add_edge(&mut self, src_id: u32, dest_id: u32, weight: Option<u32>) {

        if src_id == dest_id {
            println!("Don't want to create a self loop to a vertex!");
            return;
        }

        // Check if the source vertex exists in the graph object
        if let Some(src_vertex) = self.vertices.iter_mut().find(|v| v.v_id == src_id) {

            src_vertex.add_edge(dest_id, weight, self.edge_type.clone());
        }

        match &self.edge_type {
            Edge::DIRECTED_EDGE => {
                // Not doing anything..
            },

            Edge::UNDIRECTED_EDGE => {

                // Now adding source to destination
                if let Some(dest_vertex) = self.vertices.iter_mut().find(|v| v.v_id == dest_id) {

                    dest_vertex.add_edge(src_id, weight, self.edge_type.clone());
                }
            },
        }
    }

    pub fn delete_edge(&mut self, src_id: u32, dest_id: u32) {

        if let Some(src_vertex) = self.vertices.iter_mut().find(|v| v.v_id == src_id) {
        
            src_vertex.delete_edge(dest_id);
        }

        match &self.edge_type {
            Edge::DIRECTED_EDGE => {
                // not doing anything
            },
            Edge::UNDIRECTED_EDGE => {
                if let Some(dest_vertex) = self.vertices.iter_mut().find(|v| v.v_id == dest_id) {
                    dest_vertex.delete_edge(src_id);
                }
            },
        }
    }
    pub fn delete_vertex(&mut self, v_id: u32) {

        for v in &mut self.vertices {
            // self.delete_edge(v.get_vertex_id(), v_id);
            v.delete_edge(v_id);
        }

        // should execute at very end
        self.vertices.retain(|v| v.get_vertex_id() != v_id);
    }
    pub fn check_neighbors(&self, src_id: u32, dest_id: u32) {
        if let Some(v) = self.vertices.iter().find(|v| v.get_vertex_id() == src_id) {
            for ee in &v.edges {
                if ee.get_dest_id() == dest_id {
                    println!("src id: {} and dest id: {} are neighbors.\n", src_id, dest_id);
                    return;
                }
            }
            println!("Src id: {} and dest id: {} are NOT neighbors.\n", src_id, dest_id);
        }
    }
    pub fn print_neigbors(&self, v_id: u32) {
        
        if let Some(v) = self.vertices.iter().find(|v| v.get_vertex_id() == v_id) {
            print!("{} ({}) -> ", v_id, v.get_name());
            for ee in &v.edges {
                print!("{} - ", ee.get_dest_id());
            }
            println!();
        }
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in &self.vertices {
            print!("{} ({}) -> ", v.get_vertex_id(), v.get_name());
            for e in  v.get_edges() {        
                print!("{} - ", e.get_dest_id());
            }
            println!();
        }
        write!(f, "{}", "")
    }
}