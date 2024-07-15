#[derive(Debug,Clone)]
pub struct Book {
    book_name: String,
    author: String, 
    price: f32,
    year: u32
}

impl Book {
    pub fn new(book_name: String, author: String, price: f32, year: u32) -> Self {
        Book {
            book_name,
            author, 
            price,
            year
        }
    }
}


#[derive(Debug, Clone)]
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

    // Only 1 function..
    pub fn total_price(&self) -> f32 {
        let price: f32 = self.book.price * (self.quantity as f32);
        price
    }
}
// impl Clone for Invoice {
//     fn clone(&self) -> Self {
//         self.to_owned()
//     }
// }
#[derive(Debug)]
pub struct InvoiceDao {
    invoice: Invoice
}

impl InvoiceDao {
    pub fn new(invoice: Invoice) -> Self {
        InvoiceDao {
            invoice
        }
    }

    pub fn save_to_db(&self) {
        // save to database
        // println!("{:#?}",self.invoice);
        println!("Saved to database....");
    }
}

#[derive(Debug)]
pub struct InvoicePrinter {
    invoice: Invoice
}
impl InvoicePrinter {
    pub fn new(invoice: Invoice) -> Self {
        InvoicePrinter {
            invoice
        }
    }

    pub fn print(&self) {
        // print invoice.
        println!("Printing Invoice");
        println!("{:#?}",self.invoice);
    }
}