use std::io::{self, Write};
use _6_trie::{Trie, TrieNode};

fn main() {

    let mut trie = Trie::new();
    loop {
        println!("1) Insert word");
        println!("2) Get all words");
        println!("3) Check word present");
        println!("4) Delete word");
        println!("5) Exit");

        print!("Enter choice: ");
        io::stdout().flush().expect("unable to flush");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Unable to read line");
        let num = choice.trim().parse::<u8>().unwrap();
        if num == 1 {
            print!("Enter word to insert : ");
            io::stdout().flush().expect("unable to flush");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Unable to read line");
            let word = input.trim();
            // println!("{}",word.len());
            if ! word.chars().all(|ch| ch.is_alphabetic()) || word.len() == 0 {
               println!("Word should only contain alphabets with no length 0.\n");
               continue; 
            }
            trie.insert(word.to_string());
            println!("{} inserted!\n",word);

        } else if num == 2 {
            println!("\n{:?}\n",get_all_words(trie.root));
        } else if num == 3 {
            print!("Enter word to check : ");
            io::stdout().flush().expect("unable to flush");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Unable to read line");
            let word = input.trim();
            if ! word.chars().all(|ch| ch.is_alphabetic()) || word.len() == 0 {
                println!("Word should only contain alphabets with no length 0.\n");
                continue; 
            }
            let f = Trie::find(trie.root, word.to_string());
            if f {
                println!("\n{} present in `trie` data structure!\n",word);
            } else {
                println!("\n{} not found!\n",word);
            }
        } else if num == 4 {
            print!("Enter word to delete : ");
            io::stdout().flush().expect("unable to flush");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Unable to read line");
            let word = input.trim();
            if ! word.chars().all(|ch| ch.is_alphabetic()) || word.len() == 0 {
                println!("Word should only contain alphabets with no length 0.\n");
                continue; 
            }
            Trie::delete(trie.root, word.to_string());
            println!();
        } else {
            println!("Exiting.");
            break;
        }
    }   
    
}   

fn get_all_words(t_node: *mut TrieNode) -> Vec<String> {
    let mut word = String::new();
    let mut allwords:Vec<String> = Vec::new();
    Trie::print_all_words(t_node, &mut word, &mut allwords);
    allwords
}
