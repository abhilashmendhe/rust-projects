

use std::{fs::File, io::Read};

use rusqlite::{params, Connection, Result};
use rusqlite::config::DbConfig;

#[derive(Debug)]
struct Person {
    id: Option<i32>,
    name: String,
}
#[derive(Debug)]
struct Pet {
    id: Option<i32>,
    name: String,
}

impl  Person {
    fn new(name: String) -> Person{
        Person{
            id: None,
            name,
        }
    }
    fn new_from_db(id: i32, name: String) -> Person {
        Person {
            id: Some(id),
            name,
        }
    }
    fn add_to_db(&mut self, conn: &Connection) -> Result<()>{
        conn.execute(
            "INSERT INTO people (name) VALUES (?1)", 
            params![self.name]
        )?;
        let id = conn.last_insert_rowid() as i32;
        self.id = Some(id);
        Ok(())
    }

    fn get_all_from_db(conn: &Connection) -> Result<Vec<Person>> {
        let mut stmt = conn.prepare("SELECT id, name FROM people")?;
        let person_iter = stmt.query_map([], |row| {
            let id: i32 = row.get(0)?;
            let name: String = row.get(1)?;

            Ok(Person::new_from_db(id, name))
        })?;
        
        let mut peoples = vec![];
        for person in person_iter {
            // println!("Found person {:?}", person.unwrap());
            peoples.push(person?);
        }
        Ok(peoples)
    }
}
impl  Pet {
    fn new(name: String) -> Pet{
        Pet{
            id: None,
            name,
        }
    }
    fn new_from_db(id: i32, name: String) -> Pet {
        Pet {
            id: Some(id),
            name,
        }
    }
    fn add_to_db(&mut self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO pets (name) VALUES (?1)", 
            params![self.name]
        )?;
        let id = conn.last_insert_rowid() as i32;
        self.id = Some(id);
        Ok(())
    }

    fn get_all_from_db(conn: &Connection) -> Result<Vec<Pet>> {
        let mut stmt = conn.prepare("SELECT id, name FROM pets")?;
        let pet_iter = stmt.query_map([], |row| {
            let id: i32 = row.get(0)?;
            let name: String = row.get(1)?;

            Ok(Pet::new_from_db(id, name))
        })?;
        
        let mut pets = vec![];
        for pet in pet_iter {
            // println!("Found person {:?}", person.unwrap());
            pets.push(pet?);
        }
        Ok(pets)
    }
}

struct PetOwner {
    id: i32,
    person_id: Option<i32>, 
    pet_id: Option<i32>,
    person: Person,
    pet: Pet
}

impl PetOwner {
    fn new_from_db(id: i32, person: Person, pet: Pet) -> PetOwner {
        PetOwner {
            id,
            person_id: person.id,
            pet_id: pet.id,
            person,
            pet
        }
    } 
    fn add_to_db(conn: &Connection, person:Person, pet: Pet) -> Result<PetOwner>{
        conn.execute("INSERT INTO pet_owners (person_id, pet_id) VALUES (?1,?2)", 
        params![person.id.unwrap(), pet.id.unwrap()])?;
        let id = conn.last_insert_rowid() as i32;

        Ok(PetOwner {
            id, 
            person_id: person.id,
            pet_id: pet.id,
            pet,
            person
        })
    }
}
fn read_sql_from_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
fn main() -> Result<()> {
    // let conn = Connection::open("test.db")?;
    let conn = Connection::open_in_memory()?;
    
    conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, true)?;
    
    let sql_file_contents = read_sql_from_file("all.sql");
    
    conn.execute_batch(
        &sql_file_contents,
    )?;

    let mut me = Person::new("Abhilash".to_string());
    me.add_to_db(&conn)?;
    println!("{:?}",me);
    let person_iter = Person::get_all_from_db(&conn)?;
    for person in person_iter {
        println!("Found person {:?}",person);
    }

    let mut pet1 = Pet::new("Piku".to_string());
    pet1.add_to_db(&conn)?;
    println!("{:?}",pet1);
    let pet_iter = Pet::get_all_from_db(&conn)?;
    for pet in pet_iter {
        println!("Found pet {:?}",pet);
    }
    Ok(())
}