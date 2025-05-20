use super::{directed_edge::DirectedEdge, etrait::EdgeTrait, undirected_edge::UnDirectedEdge};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Edge {
    DIRECTED_EDGE,
    UNDIRECTED_EDGE,
}

impl Edge {
    pub fn new(etype: Edge) -> Box<dyn EdgeTrait> {
        match etype {
            Edge::DIRECTED_EDGE => Box::new(DirectedEdge::new()),
            Edge::UNDIRECTED_EDGE => Box::new(UnDirectedEdge::new()),
        }
    }
}

