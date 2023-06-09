use std::env;
use std::fs;
use std::fs::File;
use std::collections::HashMap;

extern crate dirs;

const HELP: &str = 
"USAGE:
    tcg <name> -t <template> -i <input> -o <output>
FLAGS:
    -h, --help          Print the help screen
OPTIONS:
    -i, --input         The input file of the problem solution (use % to format with problem name)
    -o, --output        The output file of your problem solution (use % to format with problem name)
    -t, --template      Choose a custom template to use for your problem solution
";

fn main() {
    let config_path_temp = dirs::home_dir().unwrap_or_default().to_string_lossy().to_string();
    let config_path = format!("{}/.config/tcg",config_path_temp);

    let problem_name = env::args().nth(1)
                        .unwrap_or_else(|| {
                            eprintln!("Problem name not provided");
                            std::process::exit(1);
                        });

    match problem_name {
        ref name if name.starts_with("-h") | name.starts_with("--help") => {
            println!("{}", HELP);
            std::process::exit(0);
        }
        ref name if name.starts_with("-") => {
            println!("Problem name not provided");
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
                std::process::exit(0)
            }
            "-t" | "--template" => {
                if let Some(arg_config) = args.next() {
                    let template_name: Vec<&str> = arg_config.split(".").collect();

                    if template_name.len() != 2 {
                        eprintln!("Invalid template format");
                        std::process::exit(1);
                    }

                    let language_extension = &template_name[1].to_string();

                    let template_file = fs::read_to_string(format!("{}/{}", config_path, arg_config))
                                            .unwrap_or_else(|err| {
                                                println!("{}", err);
                                                std::process::exit(1)
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
                println!("{}", err);
                std::process::exit(1)
            });

    File::create(options.get("input_file").unwrap()).expect("Couldn't create input file");
    File::create(options.get("output_file").unwrap()).expect("Couldn't create output file");

    let file_content = options.get("template_file").unwrap()
                            .replace("%input%", options.get("input").unwrap())
                            .replace("%output%", options.get("output").unwrap());

    fs::write(options.get("problem_file").unwrap(), file_content).expect("Unable to write problem file");
}