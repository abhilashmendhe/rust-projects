use std::{env::{args, current_dir}, fs::read_dir, process::exit};

use _1_cli::read_file;

fn main() {

    let pargs = args().collect::<Vec<String>>();
    if pargs[1] == "echo".to_string() {
        if pargs.len() < 3 {
            eprintln!("Please pass the string to echo!!");
            exit(1);
        }
        println!("{:?}",pargs[2..].join(" "));
    } else if pargs[1] == "cat".to_string() {
        if pargs.len() < 3 {
            eprintln!("Please pass the file path for cat!!");
            exit(1);
        }
        let mut cur_path = current_dir().unwrap();

        let f_path = pargs[2].clone();
        cur_path.push(f_path);
        match read_file(cur_path) {
            Ok(content) => {
                print!("{}",content);
            },
            Err(e) => {
                println!("{e}");
                exit(1);
            }
        }
    } else if pargs[1] == "ls".to_string() {
        if pargs.len() < 3 {
            eprintln!("Please pass the path / for listing files/dirictories!!");
            exit(1);
        }

        let fpath = pargs[2].clone();
        let files = read_dir(fpath).unwrap();
        let mut allfilestr:Vec<String> = Vec::new();
        // println!("\x1b[93mError\x1b[0m");
        for file in files {
            let f = file.unwrap();
            if f.file_name().into_string().unwrap().starts_with("."){
                continue;
            }
            if f.file_type().unwrap().is_dir() {
                // if f.file_name().eq("tmp") {
                //     println!("{:#?}",f.metadata().unwrap().permissions().mode());
                // }
                let dirname = format!("\x1b[1;94m{}\x1b[0m",f.file_name().to_str().unwrap());
                allfilestr.push(dirname);
            } else if f.file_type().unwrap().is_file() {
                allfilestr.push(f.file_name().into_string().unwrap());
                
            } else if f.file_type().unwrap().is_symlink() {
                // println!("{:#?}",f.metadata().unwrap());
                let dirname = format!("\x1b[0;92m{}\x1b[0m",f.file_name().to_str().unwrap());
                allfilestr.push(dirname);
            }
            // allfilestr.push(' ');
            // println!("{:?},{:?},{:?}",f.type_id(),f.file_name(),f.file_type().unwrap());
        }
        // allfilestr.sort();
        println!("{}",allfilestr.join("\n"));
    } else if pargs[1] == "find".to_string() {
        if pargs.len() < 3 {
            eprintln!("Please pass the path / for listing files/dirictories!!");
            exit(1);
        }
        let fpath = pargs[2].clone();
        if pargs.len() < 4 {
            eprintln!("Please pass the filename/dir for searching!!");
            exit(1);
        }
        let fse = pargs[3].clone();

        let allfiles = read_dir(fpath).unwrap();
        for fp in allfiles {
            let fname = fp.unwrap().file_name().into_string().unwrap();
            if fname.to_lowercase().contains(&fse) {
                println!("{}",fname);
            }
        }
    } else if pargs[1] == "grep".to_string() {
        if pargs.len() < 3 {
            eprintln!("Please pass the filename to search text!!");
            exit(1);
        }
        let file_name = pargs[2].clone();
        if pargs.len() < 4 {
            eprintln!("Please pass the search text!!");
            exit(1);
        }
        let se = pargs[3].clone();
        let mut cur_path = current_dir().unwrap();

        cur_path.push(file_name);
        // println!("{:?}",cur_path);
        match read_file(cur_path) {
            Ok(c) => {
                let sent = c.split(".").collect::<Vec<&str>>();
                for s in sent {
                    if s.contains(&se) {
                        println!("{}",s);
                    }
                }
            },
            Err(e)=>{
                println!("{e}");
            }
        }
    } else {
        panic!("Unknown command!!");
    }
}

