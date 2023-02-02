use std::{
    env::args,
    fs,
    process::Command
};
pub mod server;

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() == 1 {
        println!("Welcome to Airline.")
    }
    if args.len() >= 2 {
        if &args[1] == "scan" {
            compile_assets(&args[2], true);
        } else if &args[2] == "compile" {
            compile_assets(&args[2], false);
        }
    }
}

fn compile_assets(assets_dir: &String, dev_mode: bool) {
    let folder = fs::read_dir(assets_dir).expect("Could not open specified folder!");
    let mut output = String::new();
    if dev_mode {
        output += "use std::fs::read;\n";
    }
    output += "pub struct Assets;\nimpl Assets {\n";
    for file in folder {
        let f_unwrap = file.unwrap();
        let contents = fs::read(f_unwrap.path())
            .unwrap();
        let f_path = f_unwrap.file_name().to_str().unwrap().to_string();
        let mut f_name = f_unwrap.file_name().to_str().unwrap().to_uppercase();
        f_name = f_name.split(".").collect::<Vec<&str>>()[0].to_string();
        if !dev_mode {
            output += "\tconst K_";
            output += &f_name;
            output += ":&'static [u8] = &[";
            for c in contents {
                output += &c.to_string();
                output += ",\n\t\t";
            }
            output += "];\n\n";
        }
        output += "\tpub fn ";
        output += &f_name.to_lowercase();
        output += "() -> Vec<u8> {\n";
        if !dev_mode {
            output += "\t\treturn Assets::K_";
            output += &f_name;
            output += ".to_vec()";
        } else {
            output += "\t\treturn read(\"";
            output += &assets_dir;
            if !assets_dir.ends_with(delimiter()) {
                output += delimiter();
            }
            output += &f_path;
            output += "\").unwrap()"
        }
        output += ";\n\t}\n\n";
    }
    output += "}";
    let mut output_path = String::new();
    output_path += "src";
    output_path += delimiter();
    output_path += "assets.rs";
    _ = fs::write(output_path, output);
    cargo_build(dev_mode);
}

fn cargo_build(dev_mode: bool) {
    let result = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args([
                "/C",
                if dev_mode {
                    "cargo build"
                } else {
                    "cargo build --release"
                },
            ])
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(if dev_mode {
                "cargo build"
            } else {
                "cargo build --release"
            })
            .output()
    }
    .unwrap();
    let result_err = String::from_utf8(result.stderr).unwrap();

    let build_results = result_err
        .lines()
        .filter(|x| x.starts_with("warning:") || x.starts_with("error:"))
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    println!("{}", build_results.join("\n"));
}

fn delimiter() -> &'static str {
    if cfg!(target_os = "windows") {
        "\\"
    } else {
        "/"
    }
}