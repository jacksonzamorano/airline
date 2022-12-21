use std::{env::args, fs};

use demo::create_demo;

pub mod assets;
pub mod demo;
pub mod server;

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() == 1 {
        run_demo();
    }
    if args.len() >= 2 {
        if &args[1] == "compile" {
            // Just compile
            println!("Compiling html assets...");
            let folder = fs::read_dir(&args[2]).expect("Could not open specified folder!");
            let mut output = String::new();
            output += "pub struct Assets {}\nimpl Assets {\n";
            for file in folder {
                let f_unwrap = file.unwrap();
                let contents = fs::read_to_string(f_unwrap.path())
                    .unwrap()
                    .replace("\"", "\\\"");
                let f_name = f_unwrap.file_name().to_str().unwrap().replace(".html", "").to_uppercase();
                output += "\tpub const ";
                output += &f_name;
                output += ":&str = \"";
                output += &contents;
                output += "\";\n\n"
            }
            output += "}";
            _ = fs::write("src/assets.rs", output);
        }
    }
}

fn run_demo() {
    create_demo();
}
