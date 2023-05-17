use std::env;
use std::fs;
use std::fs::File;
use std::collections::HashMap;

const HELP: &str = 
"USAGE:
    tcg <name> -l <language_extension> --i %.in -o %.out
FLAGS:
    -h, --help          Print the help screen
OPTIONS:
    -l, --language      The language extension of your problem solution
    -i, --input         The input file of the problem solution (use % to format with problem name)
    -o, --output        The output file of your problem solution (use % to format with problem name)
    -t, --template      Choose a custom template to use for your problem solution
";

fn main() {
    let problem_name = env::args().nth(1)
        .expect("Failed to read problem name")
        .to_string();

    match problem_name{
        ref s if s.starts_with("-h") | s.starts_with("--help") => {
            println!("{}", HELP);
            std::process::exit(1);
        }
        ref s if s.starts_with("-") => {
            println!("Problem name cannot be empty");
            std::process::exit(1);
        }
        _ => {}
    }

    let mut options = HashMap::new();

    let mut args = env::args().skip(2);

    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" | "--help" => {
                println!("{}", HELP); 
                std::process::exit(1)
            }
            "-t" | "--template" => {
                if let Some(arg_config) = args.next() {
                    let template_name: Vec<&str> = arg_config.split(".").collect();
                    let language_extension = &template_name[1].to_string();

                    let template_file = fs::read_to_string(format!("templates/{}",arg_config))
                                            .unwrap_or_else(|err| {
                                                handle_error(err.to_string());
                                                String::new()
                                            });
                    
                    options.insert("template_file".to_string(), template_file);

                    let problem_file = format!("{}/{}.{}", problem_name, problem_name, language_extension);

                    options.insert("problem_file".to_string(), problem_file.clone());

                } else {
                    println!("No value specified for parameter --template.");
                    std::process::exit(1);
                }
            }
            "-i" | "--input" => {
                if let Some(arg_config) = args.next() {
                    let input = arg_config.replace("%", &problem_name);
                    let input_file = format!("{}/{}", problem_name, input);
                    
                    options.insert("input".to_string(), input.clone());
                    options.insert("input_file".to_string(), input_file);
                } else {
                    println!("No value specified for parameter --input.");
                    std::process::exit(1);
                }
            }
            "-o" | "--output" => {
                if let Some(arg_config) = args.next() {
                    let output = arg_config.replace("%", &problem_name);
                    let output_file = format!("{}/{}", problem_name, output);

                    options.insert("output".to_string(), output.clone());
                    options.insert("output_file".to_string(), output_file);
                } else {
                    println!("No value specified for parameter --output.");
                    std::process::exit(1);
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

    fs::create_dir(&problem_name)
            .unwrap_or_else(|err| {
                handle_error(err.to_string());
                String::new();
            });

    File::create(options.get("input_file").unwrap()).expect("Couldn't create input file");
    File::create(options.get("output_file").unwrap()).expect("Couldn't create output file");

    let file_content = options.get("template_file").unwrap()
                            .replace("%input%", options.get("input").unwrap())
                            .replace("%output%", options.get("output").unwrap());

    fs::write(options.get("problem_file").unwrap(), file_content).expect("Unable to write problem file");
}

fn handle_error(message: String) {
    println!("{}", message);
    std::process::exit(1);
}