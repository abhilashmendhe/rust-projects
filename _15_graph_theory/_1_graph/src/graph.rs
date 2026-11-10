use crate::edges::edge::Edge;
use crate::vertex::Vertex;
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Graph {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub edge_type: Edge,
}

impl Graph {
    pub fn new(etype: Edge) -> Graph {
        Graph {
            vertices: vec![],
            edges: vec![],
            edge_type: etype,
        }
    }

    pub fn add_vertex(&mut self, v_id: u32, v_name: String) {
        let v = Vertex::new(v_id, v_name);
        self.vertices.push(v);
    }

    pub fn get_vertex(&self, v_id: u32) -> Option<&Vertex> {
        // self.vertices.get(v_id as usize)
        self.vertices.iter().find(|v| v.v_id == v_id)
    }

    pub fn get_mut_vertex(&mut self, v_id: u32) -> Option<&mut Vertex> {
        // self.vertices.get_mut(v_id as usize)
        self.vertices.iter_mut().find(|v| v.v_id == v_id)
    }

    pub fn add_edge(&mut self, src_id: u32, dest_id: u32, weight: Option<i64>) {
        // if src_id == dest_id {
        //     println!("Don't want to create a self loop to a vertex!");
        //     return;
        // }

        // Check if the source vertex exists in the graph object
        if let Some(src_vertex) = self.vertices.iter_mut().find(|v| v.v_id == src_id) {
            src_vertex.add_edge(dest_id, weight, self.edge_type.clone());
        }

        match &self.edge_type {
            Edge::DIRECTED_EDGE => {
                // Not doing anything..
            }

            Edge::UNDIRECTED_EDGE => {
                // Now adding source to destination
                if let Some(dest_vertex) = self.vertices.iter_mut().find(|v| v.v_id == dest_id) {
                    dest_vertex.add_edge(src_id, weight, self.edge_type.clone());
                }
            }
        }
    }

    pub fn delete_edge(&mut self, src_id: u32, dest_id: u32) {
        if let Some(src_vertex) = self.vertices.iter_mut().find(|v| v.v_id == src_id) {
            src_vertex.delete_edge(dest_id);
        }

        match &self.edge_type {
            Edge::DIRECTED_EDGE => {
                // not doing anything
            }
            Edge::UNDIRECTED_EDGE => {
                if let Some(dest_vertex) = self.vertices.iter_mut().find(|v| v.v_id == dest_id) {
                    dest_vertex.delete_edge(src_id);
                }
            }
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
                    println!(
                        "src id: {} and dest id: {} are neighbors.\n",
                        src_id, dest_id
                    );
                    return;
                }
            }
            println!(
                "Src id: {} and dest id: {} are NOT neighbors.\n",
                src_id, dest_id
            );
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
    pub fn compute_unique_edges(&self) -> HashSet<(i64, u32, u32)> {
        let mut set_edges = HashSet::new();
        for v in &self.vertices {
            for ee in &v.edges {
                let from = ee.get_from_id();
                let to = ee.get_dest_id();
                if let Some(ew) = ee.get_weight() {
                    if !set_edges.contains(&(ew, from, to)) && !set_edges.contains(&(ew, to, from))
                    {
                        set_edges.insert((ew, from, to));
                    }
                } else {
                    if !set_edges.contains(&(i64::MIN, from, to))
                        && !set_edges.contains(&(i64::MIN, to, from))
                    {
                        set_edges.insert((i64::MIN, from, to));
                    }
                }
            }
        }
        set_edges
    }
    pub fn get_unique_edges(&self) -> Vec<(i64, u32, u32)> {
        self.compute_unique_edges().into_iter().collect()
    }
    pub fn get_unique_edges_sorted(&self) -> Vec<(i64, u32, u32)> {
        let mut edges = self.compute_unique_edges().into_iter().collect::<Vec<_>>();
        edges.sort_by_key(|(ew, _, _)| *ew);
        edges
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in &self.vertices {
            print!("{} ({}) -> ", v.get_vertex_id(), v.get_name());
            for e in v.get_edges() {
                if let Some(ew) = e.get_weight() {
                    print!("{}[{}] - ", e.get_dest_id(), ew);
                } else {
                    print!("{} - ", e.get_dest_id());
                }
            }
            println!();
        }
        write!(f, "{}", "")
    }
}
