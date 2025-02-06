use std::collections::HashMap;

trait Shape: ShapeClone {
    fn draw(&self, x: i32, y: i32, height: i32, width: i32, color: String);
}

trait ShapeClone {
    fn clone_box(&self) -> Box<dyn Shape>;
}

impl<T> ShapeClone for T
where
    T: 'static + Shape + Clone,
{
    fn clone_box(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Box<dyn Shape> {
        self.clone_box()
    }
}

#[derive(Debug, Clone, Copy)]
struct Line;

impl Line {
    fn new() -> Self {
        println!("Creating a line object");
        std::thread::sleep(std::time::Duration::from_millis(1500));
        println!("Done creating line object");
        Self
    }
}
impl Shape for Line {
    fn draw(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: String) {
        println!("Created line of color: {}", color);
    }
}

#[derive(Debug,Clone, Copy)]
struct Oval;
impl Oval {
    fn new() -> Self {
        println!("Creating a oval object");
        std::thread::sleep(std::time::Duration::from_millis(2000));
        println!("Done creating oval object");
        Self
    }
}
impl Shape for Oval {
    fn draw(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: String) {
        println!("Created oval shape of color: {}", color);
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum ShapeType {
    LINE,
    OVAL,
}
// Flyweight Factory
struct ShapeFactory {
    shapes: HashMap<ShapeType, Box<dyn Shape>>,
}

impl ShapeFactory {
    fn new() -> Self {
        Self {
            shapes: HashMap::new(),
        }
    }

    fn get_shape(&mut self, shape_type: ShapeType) -> Box<dyn Shape> {
        if let Some(shape) = self.shapes.get(&shape_type) {
            return shape.clone(); // Use the implemented Clone trait
        }

        let shape: Box<dyn Shape> = match shape_type {
            ShapeType::LINE => Box::new(Line::new()),
            ShapeType::OVAL => Box::new(Oval::new()),
        };

        self.shapes.insert(shape_type, shape.clone()); // Store a cloned version
        shape
    }
}

fn main() {
    let mut factory = ShapeFactory::new();

    let shape1 = factory.get_shape(ShapeType::LINE);
    shape1.draw(10, 20, 30, 40, "Red".to_string());

    let shape2 = factory.get_shape(ShapeType::OVAL);
    shape2.draw(50, 60, 70, 80, "Blue".to_string());

    let shape3 = factory.get_shape(ShapeType::LINE);
    shape3.draw(15, 25, 35, 45, "Green".to_string());
}