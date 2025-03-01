#[derive(Debug, Clone)]
pub struct Edge {
    pub destination_id: u32,
    pub weight: i32
}

impl Edge {
    pub fn new(destination_id: u32, weight: i32) -> Self {
        Self {
            destination_id,
            weight
        }
    }
    pub fn set_edge_values(&mut self, destination_id: u32, weight: i32) {
        self.destination_id = destination_id;
        self.weight = weight;
    }
    pub fn set_weight(&mut self, weight: i32) {
        self.weight = weight;
    }

    pub fn get_destination_id(&self) -> u32 {
        self.destination_id
    }
    pub fn get_weight(&self) -> i32 {
        self.weight 
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.destination_id == other.destination_id
    }
}