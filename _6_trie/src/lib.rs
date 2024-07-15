use std::ptr;

#[derive(Debug)]
pub struct TrieNode {
    pub child_node: Vec<*mut TrieNode>,
    pub word_end: bool
}

impl TrieNode {
    pub fn new() -> *mut TrieNode {
        Box::into_raw(Box::new(Self {
            child_node: vec![ptr::null_mut::<TrieNode>(); 26],
            word_end: false,
        }))
    }
}

#[derive(Debug)]
pub struct Trie {
    pub root: *mut TrieNode
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new()
        }
    }

    pub fn insert(&mut self, word: String) {
        unsafe {
            let mut t_trie = self.root;
            for ch in word.chars() {
                let ind = ch as usize - 'a' as usize;
                
                if (*t_trie).child_node[ind].is_null() {
                    (*t_trie).child_node[ind] = TrieNode::new();
                }
                t_trie = (*t_trie).child_node[ind];
            }
            (*t_trie).word_end = true;
        }  
    }

    pub fn print_all_words(root: *mut TrieNode, word: &mut String, allwords: &mut Vec<String>) {
        unsafe {    
            // let c_nodes = (*root).child_node.clone();
            if (*root).word_end {
                // println!("{}",word);
                allwords.push(word.to_string());
                // return;
            }
            for (ind, key) in (*root).child_node.clone().iter().enumerate() {
                // println!("{},{:?}",ind, key);
                if ! key.is_null() {
                    // print!("{}",(b'a' + ind as u8) as char);
                    word.push((b'a' + ind as u8) as char);
                    Self::print_all_words((*root).child_node[ind], word, allwords);
                    // println!("{} - {}",word,(*root).word_end);
                    word.pop();
                }
            }
            // println!();
        }
    }

    pub fn find(root: *mut TrieNode, word: String) -> bool {
        let mut t_node = root;
        let mut f = true;
        for ch in word.chars() {
            unsafe {
                let ind = ch as usize - 'a' as usize;
                if ! (*t_node).child_node[ind].is_null() {
                    t_node = (*t_node).child_node[ind];
                } else {
                    f = false;
                    break;
                }
            }
        }
        unsafe {
            if ! (*t_node).word_end {
                f = false;
            }
        }
        f
    }
    pub fn delete(root: *mut TrieNode, word: String) {
    
        let mut t_node = root;
        for ch in word.chars() {
            unsafe {
                let ind = ch as usize - 'a' as usize;
                if ! (*t_node).child_node[ind].is_null() {
                    t_node = (*t_node).child_node[ind];
                } 
            }
        }
        unsafe {
            let status = (*t_node).word_end;
            if ! status {
                println!("{} not found. Can't delete from the data structure",word);
            } else {
                (*t_node).word_end = false;
                println!("{} deleted!",word);
            }
        }
    }
}




