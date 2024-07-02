use std::{fs::File, io::Read};

use chrono::Utc;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Task {
    pub id: Option<i32>, 
    pub name: String,
    pub done: bool,
    pub t_date: String
}

impl Task {
    pub fn new(name: String) -> Task {
        Task {
            id: None, 
            name,
            done: false,
            t_date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
        }
    }
    pub fn new_from_db(id: i32, name: String, done: bool, t_date: String) -> Task {
        Task {
            id: Some(id),
            name,
            done,
            t_date
        }
    }
    pub fn add_to_db(&mut self, conn: &Connection) -> Result<()> {

        conn.execute(
            "INSERT INTO tasks (name, done, taskdate) VALUES (?1, ?2, ?3)",
            params![&self.name, &self.done, &self.t_date]
        )?;

        Ok(())
    }
    pub fn get_task_by_id(id: i32, conn: &Connection) -> Result<Task> {
        let mut stmt = conn.prepare(
            "SELECT * FROM tasks WHERE id = :id"
        )?;
        
        let c_row = stmt.query_row(params![id], |row| {
            let id: i32 = row.get(0)?;
            let name: String = row.get(1)?;
            let done: bool = row.get(2)?;
            let t_date: String = row.get(3)?;
            Ok(Task::new_from_db(id, name, done, t_date))
        })?;
        Ok(c_row)
    }
    pub fn remove_from_db(id: i32, conn: &Connection) -> Result<Task> {
        let task = Task::get_task_by_id(id, conn)?;
        let mut stmt = conn.prepare(
            "DELETE FROM tasks WHERE id = :id"
        )?;
        let _ = stmt.execute(params![id])?;
        Ok(task)
    }
    pub fn update_task_by_id(&self, id: i32, conn: &Connection) -> Result<()>{
        conn.execute(
            "UPDATE tasks SET name=?1, taskdate=?2 WHERE id=?3",
            params![&self.name, &self.t_date, id]
        )?;
        Ok(())
    }
    pub fn make_check(id: i32, conn: &Connection) -> Result<()> {
        conn.execute(
            "UPDATE tasks SET done=?1 WHERE id=?2",
            params![true, id]
        )?;
        Ok(())
    }
    pub fn get_all_from_db(conn: &Connection) -> Result<Vec<Task>> {
        let mut stmt = conn.prepare("SELECT * FROM tasks")?;
        let task_iter = stmt.query_map([], |row| {
            let id: i32 = row.get(0)?;
            let name: String = row.get(1)?;
            let done: bool = row.get(2)?;
            let t_date: String = row.get(3)?;
            Ok(Task::new_from_db(id, name, done, t_date))
        })?;
        let mut tasks = vec![];
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }
}

pub fn read_sql_from_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}