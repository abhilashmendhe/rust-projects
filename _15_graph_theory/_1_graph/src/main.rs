use std::io::{self, Write};

use _1_graph::{graph::Graph, vertex::Vertex};


fn main() {

    let mut g = Graph::new();

    loop {
        println!("1. Add vertex");
        println!("2. Update vertex");
        println!("3. Delete vertex");
        println!("4. Add edge");
        println!("5. Update edge");
        println!("6. Delete edge");
        println!("7. Check if 2 vertices are neighbors");
        println!("8. Print all neighbors of a vertex");
        println!("9. Print graph");
        println!("10. Clear screen");
        println!("0. Exit program");

        print!("\n> ");
        io::stdout().flush().unwrap();

        // user input 
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim().parse::<u8>().unwrap();

        println!("input: {}", input);

        if input == 0 {
            break;
        } else if input == 1 {
            print!("Enter state id: ");
            io::stdout().flush().unwrap();
            let stdin = io::stdin();
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let state_id = input.trim().parse::<u32>().unwrap();

            print!("Enter state name: ");
            io::stdout().flush().unwrap();
            let stdin = io::stdin();
            let mut state_name = String::new();
            stdin.read_line(&mut state_name).unwrap();

            let v = Vertex::new(state_id, state_name.trim().to_string());

            g.add_vertex(v);
            
        } else if input == 2 {

        } else if input == 3 {

        } else if input == 4 {

            print!("Enter ID of the source vertex: ");
            io::stdout().flush().unwrap();
            let stdin = io::stdin();
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let src_id = input.trim().parse::<u32>().unwrap();

            print!("Enter ID of the destination vertex: ");
            io::stdout().flush().unwrap();
            let stdin = io::stdin();
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let dest_id = input.trim().parse::<u32>().unwrap();

            print!("Enter weight: ");
            io::stdout().flush().unwrap();
            let stdin = io::stdin();
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let weight = input.trim().parse::<i32>().unwrap();

            g.add_edge_by_id(src_id, dest_id, weight);
            
        } else if input == 5 {

        } else if input == 6 {

        } else if input == 7 {

        } else if input == 8 {

        } else if input == 9 {
            println!("{:#?}",g);
            // println!("{}",g);
        } else if input == 10 {
            std::process::Command::new("clear").status().unwrap();
        }
    }
    
}
