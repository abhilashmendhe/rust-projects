use once_cell::sync::OnceCell;

static DB_OBJ:OnceCell<Database> = OnceCell::new();

#[derive(Debug,Clone)]
pub struct Database {
    url: String
}

impl Database {
    pub fn new(url: String) -> Self {
        let db_obj = DB_OBJ.get_or_init(|| Database {
            url
        });
        db_obj.clone()
    }
}