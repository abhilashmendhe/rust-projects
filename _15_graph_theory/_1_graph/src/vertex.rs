use std::{collections::LinkedList};

use crate::edges::{edge::Edge, etrait::EdgeTrait};


#[derive(Debug, Clone)]
pub struct Vertex {
    pub v_id: u32,
    pub v_name: String,  
    pub edges: LinkedList<Box<dyn EdgeTrait>>,
    pub visited: bool
}

impl Vertex {
    pub fn new(v_id: u32, v_name: String) -> Self {
        Vertex { 
            v_id,
            v_name, 
            edges: LinkedList::new(),
            visited: false
        }
    }
    
    pub fn get_vertex_id(&self) -> u32 {
        self.v_id
    }
    pub fn get_name(&self) -> &String {
        &self.v_name
    }
    pub fn set_name(&mut self, v_name: String) {
        self.v_name = v_name;
    }
    pub fn get_edges(&self) -> &LinkedList<Box<dyn EdgeTrait>> {
        &self.edges
    }
    pub fn set_edges(&mut self, edges: LinkedList<Box<dyn EdgeTrait>>) {
        self.edges = edges;
    }
    pub fn is_visited(&self) -> bool {
        self.visited
    }
    pub fn add_edge(&mut self, dest_id: u32, weight: Option<u32>, edge_type: Edge) {

        // Check if already a connection to the destination vertex
        for ee in self.get_edges() {
            if ee.get_dest_id() == dest_id {
                return;
            }
        }

        let mut new_edge = Edge::new(edge_type);
        new_edge.set_dest_id(dest_id);
        new_edge.set_weight(weight);
        self.edges.push_back(new_edge);
    } 

    pub fn delete_edge(&mut self, dest_id: u32) {
        let mut new_edges: LinkedList<Box<dyn EdgeTrait>> = LinkedList::new();
        for ee in &self.edges {
            if ee.get_dest_id() != dest_id {
                new_edges.push_back(ee.clone_box());
            }
        }
        self.set_edges(new_edges);
    }
}