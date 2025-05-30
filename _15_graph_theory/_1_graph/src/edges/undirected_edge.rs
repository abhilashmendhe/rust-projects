use super::etrait::EdgeTrait;

#[derive(Debug, Clone)]
pub struct UnDirectedEdge {
    pub dest_id: u32,
    pub weight: Option<u32>,
    visited: bool
}

impl UnDirectedEdge {
    pub fn new() -> Self {
        UnDirectedEdge { dest_id: 0, weight: None, visited: false }
    }
}

impl EdgeTrait for UnDirectedEdge{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn EdgeTrait> {
        Box::new(self.clone())
    }
    
    fn set_dest_id(&mut self, dest_id: u32) {
        self.dest_id = dest_id;
    }
    fn get_dest_id(&self) -> u32 {
        self.dest_id
    }
    fn set_weight(&mut self, weight: Option<u32>) {
        self.weight = weight
    }
    fn get_weight(&self) -> Option<u32> {
        self.weight
    }
    fn is_visited(&self) -> bool {
        self.visited
    }
}