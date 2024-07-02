use std::{env, io, process};
use _2_todo::{read_sql_from_file, Task};
use colored::Colorize;
use rusqlite::{Connection, Result};


fn main() -> Result<()> {
    let pargs:Vec<String> = env::args().collect();

    if pargs.len() < 2 {
        println!("Please pass arguments to run the todo app!");
        process::exit(1);
    }

    let conn = Connection::open("tasks.db")?;
    let sql_file_contents = read_sql_from_file("schema.sql");
    
    conn.execute_batch(
        &sql_file_contents,
    )?;

    if pargs[1] == "list".to_string() {
        let alltask_iter = Task::get_all_from_db(&conn)?;
        for task in alltask_iter {
            if task.done {
                println!("{}. {} ({})",task.id.unwrap(),task.name.strikethrough(), task.t_date);
            } else {
                println!("{}. {} ({})",task.id.unwrap(),task.name,task.t_date);
            }
        }
    }
    else if pargs[1]=="add".to_string() {
        
        if pargs.len() < 3 {
            println!("Please pass 3rd argument to add the task!");
            process::exit(1);
        }
        let do_task: String = pargs[2].clone();
        let mut task = Task::new(do_task);
        task.add_to_db(&conn)?;
        println!("{:#?}",task);
        
    } else if pargs[1] == "remove".to_string() {
        if pargs.len() < 3 {
            println!("Please pass 3rd argument as id to delete the task!");
            process::exit(1);
        }
        let id: i32 = pargs[2].clone().parse::<i32>().unwrap();

        match Task::remove_from_db(id, &conn) {
            Ok(task) => {
                println!("Task with id: {} removed.",id);
                println!("{:#?}",task);
            }, 
            Err(_) => {
                println!("No task found to delete from the table 'tasks'");
            }
        }
    } else if pargs[1] == "get".to_string() {
        if pargs.len() < 3 {
            println!("Please pass 3rd argument as id to retrieve the task!");
            process::exit(1);
        }
        let id: i32 = pargs[2].clone().parse::<i32>().unwrap();
        
        match Task::get_task_by_id(id, &conn) {
            Ok(task) => {if task.done {
                println!("{}. {} ({})",task.id.unwrap(),task.name.strikethrough(), task.t_date);
            } else {
                println!("{}. {} ({})",task.id.unwrap(),task.name,task.t_date);
            }},
            Err(e) => {
                println!("Error: {}",e);
            }
        }
    } else if pargs[1] == "edit".to_string() {
        if pargs.len() < 3 {
            println!("Please pass 3rd argument as id to retrieve the task!");
            process::exit(1);
        }
        let id: i32 = pargs[2].clone().parse::<i32>().unwrap();
        match Task::get_task_by_id(id, &conn) {
            Ok(task) => {
                if ! task.done {
                    println!("\n{}. {} ({})\n",task.id.unwrap(),task.name,task.t_date);
                    println!("Please enter the new task to rename the task id: {}",task.id.unwrap());
                    let mut name = String::new();
                    let _ = io::stdin().read_line(&mut name);
                    // println!("{}",buffer);
                    let newtask = Task::new(name.trim().to_string());
                    newtask.update_task_by_id(id, &conn)?;
                } else {
                    println!("Task id: {} is already done. Can't edit!",task.id.unwrap());
                    process::exit(1);
                }
            },
            Err(e) => {
                println!("Error: {}",e);
            }
        }
    } else if pargs[1] == "done".to_string() {
        if pargs.len() < 3 {
            println!("Please pass 3rd argument as id to check the task!");
            process::exit(1);
        }
        let id: i32 = pargs[2].parse::<i32>().unwrap();
        // println!("{}",id);
        Task::make_check(id, &conn)?;
    } else {
        eprintln!("Invalid argument!");
    }
    Ok(())
}
