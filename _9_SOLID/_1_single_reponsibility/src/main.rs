/*
SINGLE REPONSIBILITY PRINCIPLE()
Definition: A class should have only one reason to change, meaning it should have only one 
            job or responsibility.
Purpose: This helps in making the class more cohesive and reduces the risk of changes 
        affecting unrelated functionality, thus making the code easier to maintain.
*/

mod good_example;
use good_example::Invoice;

use crate::good_example::{Book, InvoiceDao, InvoicePrinter};

// mod bad_example;
// use crate::bad_example::Book;

fn main() {
    let book = Book::new(
        String::from("Chess"),
        String::from("Vidith"),
        1299.9,
        2020
    );

    let invoice = Invoice::new(book, 10);
    println!("Total price: {}", invoice.total_price());
    let db1 = InvoiceDao::new(invoice.clone());
    db1.save_to_db();

    let invoiceprint = InvoicePrinter::new(invoice.clone());
    invoiceprint.print();
}
