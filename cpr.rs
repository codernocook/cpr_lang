use std::env;
use std::fs;

struct Interpreter {
    variables: Vec<Variable>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            variables: Vec::new(),
        }
    }
    
    fn parse(&mut self, filename: &str) {
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
            
        for line in contents.lines() {
            if line.starts_with("//") {
                // skip comments
                continue;
            }
            else if line.starts_with("p(") && line.ends_with(");") {
                // print statement
                let statement = line.trim()[2..line.len()-2].to_string();

                if statement.contains('"') {
                    // this is a string not a variable
                    if statement.starts_with("\"") && statement.ends_with("\"") {
                        let final_result = statement.replace("\"", "");

                        println!("{}", final_result);
                    } else {
                        println!("[ERROR]: Missing quote.")
                    }
                } else {
                    // this maybe a variable
                }
            }
            else if line.starts_with("if (") && line.ends_with("):") {
                // if statement
                let statement = line.trim()[4..line.len()-2].to_string();
                let value = if statement == "statement" { "true" } else { "false" };
                println!("{}", value);
            }
            else if line.starts_with("vr ") {
                // variable declaration
                let parts: Vec<&str> = line.split_whitespace().collect();
                let name = parts[1].split(":").next().unwrap().to_string();
                let datatype = parts[1].split(":").skip(1).next().unwrap().to_string();
                let value = parts[3].trim_matches('\"').to_string();
                let variable = Variable::new(name, datatype, value);
                self.variables.push(variable);
            }
            else if line.starts_with("fn ") && line.ends_with("():") {
                // function declaration
                let name = line.trim()[3..line.len()-2].to_string();
                println!("Function: {}", name);
            }
            else {
                // unrecognized syntax
                println!("Unrecognized syntax: {}", line);
            }
        }
    }
}

#[allow(dead_code)]
struct Variable {
    name: String,
    datatype: VariableType,
    value: String,
}

impl Variable {
    fn new(name: String, datatype: String, value: String) -> Self {
        let var_type = match datatype.as_str() {
            "string" => VariableType::String,
            "int" => VariableType::Integer,
            "bool" => VariableType::Boolean,
            _ => panic!("Unknown variable type: {}", datatype),
        };
        Variable {
            name: name,
            datatype: var_type,
            value: value,
        }
    }
}

enum VariableType {
    String,
    Integer,
    Boolean
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./cpr filename.cpr");
        return;
    }

    let filename = &args[1];
    let mut interpreter = Interpreter::new();
    interpreter.parse(filename);
}