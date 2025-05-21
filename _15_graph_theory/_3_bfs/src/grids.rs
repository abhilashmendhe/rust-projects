
/*
    Many problems in grah theory can be represented using a grid. Grids are a form of implicit graph
    because we can determine a node's neigbors based on our location within the grid.

    In this we will try to run BFS on maze to find start and end path in the quickest way possible.
*/

use std::collections::{HashSet, VecDeque};

pub fn grid_bfs() {

    let my_grid = [
        ["S", ".", ".", "#", ".", ".", "."],
        ["E", "#", ".", ".", ".", "#", "."],
        [".", "#", ".", ".", ".", ".", "."],
        [".", ".", "#", "#", ".", ".", "."],
        ["#", ".", "#", "E", ".", "#", "."],
    ];

    let row_len = my_grid.len();
    let col_len = my_grid[0].len();

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let start = (0 as i32 ,0 as i32);
    visited.insert(start);
    queue.push_back(start);

    while !queue.is_empty() {

        let fpop_cell = queue.pop_front().unwrap();
        print!("{:?} ({}) - ", fpop_cell,my_grid[fpop_cell.0 as usize][fpop_cell.1 as usize]);
        println!("{:?}", queue);
        if my_grid[fpop_cell.0 as usize][fpop_cell.1 as usize] == "E" {
            println!("Found");
            break;
        }

        // Get neigbors
        // top
        let top = (fpop_cell.0 - 1, fpop_cell.1);
        if top.0 >= 0 && top.1 >= 0 {
            if my_grid[top.0 as usize][top.1 as usize] == "." {
                if !visited.contains(&top) {
                    visited.insert(top);
                    queue.push_back(top);
                }
            }
        }
        // left
        let left = (fpop_cell.0, fpop_cell.1 - 1);
        if left.0 >= 0 && left.1 >= 0 {
            if my_grid[left.0 as usize][left.1 as usize] == "." {
                if !visited.contains(&left) {
                    visited.insert(left);
                    queue.push_back(left);
                }
            }
        }
        // bottom
        let bottom = (fpop_cell.0 + 1, fpop_cell.1);
        if bottom.0 < row_len as i32 && bottom.1 < col_len as i32 {
            if my_grid[bottom.0 as usize][bottom.1 as usize] == "." {
                if !visited.contains(&bottom) {
                    visited.insert(bottom);
                    queue.push_back(bottom);
                }
            }
        }
        // right
        let right = (fpop_cell.0, fpop_cell.1 + 1);
        if right.0 < row_len as i32 && right.1 < col_len as i32 {
            if !visited.contains(&right) {
                if my_grid[right.0 as usize][right.1 as usize] == "." {
                    visited.insert(right);
                    queue.push_back(right);
                }
            }
        }

        // println!("{:?}, {:?}, {:?}, {:?}", top, left, bottom, right);
    }
    println!()
}