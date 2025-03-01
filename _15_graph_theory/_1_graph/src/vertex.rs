use std::collections::LinkedList;

use crate::edge::Edge;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub state_id: u32, 
    pub state_name: String, 
    pub edge_list: LinkedList<Edge>
}

impl Vertex {
    pub fn new(state_id: u32, state_name: String) -> Self {
        Self {
            state_id,
            // state_name: state_name.to_lowercase(),
            state_name,
            edge_list: LinkedList::new()
        }
    }

    pub fn set_id(&mut self, state_id: u32) {
        self.state_id = state_id;
    }

    pub fn set_state_name(&mut self, state_name: String) {
        self.state_name = state_name;
    }

    pub fn get_id(&self) -> u32 {
        self.state_id
    }

    pub fn get_state_name(&self) -> &String {
        &self.state_name
    }

    pub fn get_edge_list(&self) -> &LinkedList<Edge> {
        &self.edge_list
    }
    pub fn add_edge(&mut self, edge: Edge) {
        self.edge_list.push_back(edge);
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}