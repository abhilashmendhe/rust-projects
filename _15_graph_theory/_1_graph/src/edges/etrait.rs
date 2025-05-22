use std::{any::Any, fmt::Debug};

pub trait EdgeTrait: Debug + Any {

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn clone_box(&self) -> Box<dyn EdgeTrait>;

    fn set_dest_id(&mut self, dest_id: u32);
    fn get_dest_id(&self) -> u32;
    fn set_weight(&mut self, weight: Option<u32>);
    fn get_weight(&self) -> Option<u32>;
    fn is_visited(&self) -> bool;
}

impl Clone for Box<dyn EdgeTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}