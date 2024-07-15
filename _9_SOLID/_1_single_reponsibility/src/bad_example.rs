#[derive(Debug)]
pub struct Book {
    book_name: String,
    author: String, 
    price: u32,
    year: u32
}

impl Book {
    pub fn new(book_name: String, author: String, price: u32, year: u32) -> Self {
        Book {
            book_name,
            author, 
            price,
            year
        }
    }
}

#[derive(Debug)]
pub struct Invoice {
    book: Book,
    quantity: u32,
}

impl Invoice {
    pub fn new(book: Book, quantity: u32) -> Self {
        Invoice {
            book,
            quantity
        }
    }

    pub fn total_price(self) -> u32 {
        let price: u32 = &self.book.price * self.quantity;
        price
    }

    // bad should be in different struct or class
    pub fn print_invoice() {
        // print invoice
    }

    // bad should be in different struct or class
    pub fn save_to_db () {
        // save data to database
    }
}
