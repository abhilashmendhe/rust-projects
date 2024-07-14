use std::{fs::{File, OpenOptions}, io::{BufRead, BufReader, BufWriter, Write}, sync::mpsc, thread::{self, sleep}, time::Duration};

fn main() {
    let input_filename = "input.txt";
    let output_filename = "output.txt";

    let (tx_input, rx_input) = mpsc::channel();
    let (tx_output, rx_output) = mpsc::channel::<i32>();

    // Separate thread to read from file
    let reader_thread = thread::spawn(move || {
        let input_file = File::open(input_filename).unwrap();
        let reader = BufReader::new(input_file);

        for line in reader.lines() {
            println!("Read line {:?}",line);
            tx_input.send(line.unwrap().parse::<i32>().unwrap()).unwrap();
        }
    });

    // Separate thread to write to file
    let writer_thread = thread::spawn(move || {
        let output_file = OpenOptions::new().write(true).create(true).open(output_filename).unwrap();
        let mut writer = BufWriter::new(output_file);

        for num in rx_output {
            let num_str = num.to_string();
            writer.write_all(num_str.as_bytes()).unwrap();
            writer.write_all(b"\n").unwrap();
            println!("Wrote num {}",num);
        }
        writer.flush().unwrap();
    });

    for num in rx_input {
        println!("Received message: {}",num);
        let num_squared = num * num;
        tx_output.send(num_squared).unwrap();
    }
    drop(tx_output);
    reader_thread.join();
    writer_thread.join();
}
