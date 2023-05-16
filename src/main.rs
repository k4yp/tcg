use std::env;
use std::fs;
use std::fs::File;
use std::collections::HashMap;

const HELP: &str = 
"USAGE:
    gen <name> -l <language_extension> --i %.in -o %.out
FLAGS:
    -h, --help          Print the help screen
OPTIONS:
    -l, --language      The language extension of your problem solution
    -i, --input         The input file of the problem solution (use % to format with problem name)
    -o, --output        The output file of your problem solution (use % to format with problem name)
";

const PY: &str =
r#"with open("%in%","r") as f:
    data = f.read()

with open("%out%","w") as f:
     f.write(data)"#;

fn main() {
    let problem_name = env::args().nth(1)
        .expect("Failed to read problem name")
        .to_string();

    if problem_name.starts_with("-") {
        panic!("Problem name cannot be empty");
    }

    let mut args = env::args().skip(2);

    let mut options = HashMap::new();

    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" | "--help" => println!("{}", HELP),
            "-l" | "--language" => {
                if let Some(arg_config) = args.next() {
                    let language_extension = &arg_config.to_string();
                    let problem_file = format!("{}/{}.{}", problem_name, problem_name, language_extension);

                    options.insert("problem_file".to_string(), problem_file.clone());
                } else {
                    panic!("No value specified for parameter --language.");
                }
            }
            "-i" | "--input" => {
                if let Some(arg_config) = args.next() {
                    let input = arg_config.replace("%", &problem_name);
                    let input_file = format!("{}/{}", problem_name, input);
                    
                    options.insert("input".to_string(), input.clone());
                    options.insert("input_file".to_string(), input_file);
                } else {
                    panic!("No value specified for parameter --input.");
                }
            }
            "-o" | "--output" => {
                if let Some(arg_config) = args.next() {
                    let output = arg_config.replace("%", &problem_name);
                    let output_file = format!("{}/{}", problem_name, output);

                    options.insert("output".to_string(), output.clone());
                    options.insert("output_file".to_string(), output_file);
                } else {
                    panic!("No value specified for parameter --output.");
                }
            }
            _ => {
                if arg.starts_with('-') {
                    println!("Unknown argument {}", arg);
                } else {
                    println!("Unknown positional argument {}", arg);
                }
            }
        }
    }

    fs::create_dir(&problem_name).expect("Couldn't create directory");

    File::create(options.get("input_file").unwrap()).expect("Couldn't create output file");
    File::create(options.get("output_file").unwrap()).expect("Couldn't create output file");

    let file_content = PY
                        .replace("%in%", options.get("input").unwrap())
                        .replace("%out%", options.get("output").unwrap());

    fs::write(options.get("problem_file").unwrap(), file_content).expect("Unable to write file");
}