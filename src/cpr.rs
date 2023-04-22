    extern crate ansi_term;
    use std::env;
    use std::fs;
    use ansi_term::Colour;

    impl Interpreter {
        fn new() -> Self {
            Interpreter {
                variables: Vec::new(),
            }
        }
        
        fn parse(&mut self, filename: &str) {
            let contents = fs::read_to_string(filename)
                .expect("Something went wrong reading the file");
            
            let mut line_count = 0;

            for line in contents.lines() {
                line_count += 1;
                if line.starts_with("//") {
                    // skip comments
                    continue;
                }
                else if line.is_empty() {
                    continue;
                }
                /* (semicolon forces)
                if !line.trim_end().ends_with(';') && !line.trim_end().ends_with(':') {
                    println!("{} Missing semicolon or colon.\n  => Line: {}\n  => Content: {}", Colour::Red.paint("[ERROR]"), line_count, line);
                    return;
                }
                */
                else if line.starts_with("p(") && line.ends_with(")") || line.ends_with(");") {
                    // print statement
                    let mut statement = line.trim()[2..line.len()-2].to_string();

                    if line.ends_with(")") {
                        statement = line.trim()[2..line.len()-1].to_string();
                    }

                    if statement.contains('"') {
                        // this is a string not a variable
                        if statement.starts_with("\"") && statement.ends_with("\"") {
                            let over_quote_exploit = count_quotes(&statement);
                            if over_quote_exploit % 2 != 0 {
                                println!("{} Invalid quote.\n  => Line: {}\n  => Content: {}", Colour::Red.paint("[ERROR]"), line_count, line);
                                std::process::exit(1);
                            }
                            let final_result = statement.replace("\"", "");
                
                            println!("{}", final_result);
                        } else {
                            println!("{} Missing quote in string.\n  => Line: {}\n  => Content: {}", Colour::Red.paint("[ERROR]"), line_count, line);
                            std::process::exit(1);
                        }
                    } else {
                        // this is a variable
                        let variable_name = statement;
                        let variable_value = self.get_variable_value(&variable_name);
                        if let Some(value) = variable_value {
                            let value_str = match value {
                                VariableValue::Integer(i) => i.to_string(),
                                VariableValue::Float(f) => f.to_string(),
                                VariableValue::Boolean(b) => b.to_string(),
                                VariableValue::String(s) => s,
                                VariableValue::Array(a) => format!("{:?}", a.clone()),
                                VariableValue::Table(t) => format!("{:?}", t.clone()),
                            };
                            println!("{}", value_str);
                        } else {
                            println!("{} Unknown variable.\n  => Line: {}\n  => Content: {}", Colour::Red.paint("[ERROR]"), line_count, line);
                            std::process::exit(1);
                        }
                    }
                }
                else if line.starts_with("warn(") && line.ends_with(")") || line.ends_with(");") {
                    // print statement
                    let statement = line.trim()[2..line.len()-2].to_string();
                    if statement.contains('"') {
                        // this is a string not a variable
                        if statement.starts_with("\"") && statement.ends_with("\"") {
                            let over_quote_exploit = count_quotes(&statement);
                            if over_quote_exploit % 2 != 0 {
                                println!("{} Invalid quote.\n  => Line: {}\n  => Content: {}", Colour::Red.paint("[ERROR]"), line_count, line);
                                std::process::exit(1);
                            }
                            let final_result = statement.replace("\"", "");
                
                            println!("{} {}.\n  => Line: {}\n  => Content: {}", Colour::Yellow.paint("[WARNING]"), final_result, line_count, line);
                        } else {
                            println!("{} Missing quote in string.\n  => Line: {}\n  => Content: {}", Colour::Red.paint("[ERROR]"), line_count, line);
                            std::process::exit(1);
                        }
                    } else {
                        // this is a variable
                        let variable_name = statement;
                        let variable_value = self.get_variable_value(&variable_name);
                        if let Some(value) = variable_value {
                            let value_str = match value {
                                VariableValue::Integer(i) => i.to_string(),
                                VariableValue::Float(f) => f.to_string(),
                                VariableValue::Boolean(b) => b.to_string(),
                                VariableValue::String(s) => s,
                                VariableValue::Array(a) => format!("{:?}", a.clone()),
                                VariableValue::Table(t) => format!("{:?}", t.clone()),
                            };
                            println!("{} {}.\n  => Line: {}\n  => Content: {}", Colour::Yellow.paint("[WARNING]"), value_str, line_count, line);
                        } else {
                            println!("{} Unknown variable.\n  => Line: {}\n  => Content: {}", Colour::Red.paint("[ERROR]"), line_count, line);
                            std::process::exit(1);
                        }
                    }
                }
                else if line.starts_with("err(") && line.ends_with(")") || line.ends_with(");") {
                    // print statement
                    let statement = line.trim()[2..line.len()-2].to_string();
                    if statement.contains('"') {
                        // this is a string not a variable
                        if statement.starts_with("\"") && statement.ends_with("\"") {
                            let over_quote_exploit = count_quotes(&statement);
                            if over_quote_exploit % 2 != 0 {
                                println!("{} Invalid quote.\n  => Line: {}\n  => Content: {}", Colour::Red.paint("[ERROR]"), line_count, line);
                                std::process::exit(1);
                            }
                            let final_result = statement.replace("\"", "");
                
                            println!("{} {}.\n  => Line: {}\n  => Content: {}", Colour::Red.paint("[ERROR]"), final_result, line_count, line);
                            std::process::exit(1);
                        } else {
                            println!("{} Missing quote in string.\n  => Line: {}\n  => Content: {}", Colour::Red.paint("[ERROR]"), line_count, line);
                            std::process::exit(1);
                        }
                    } else {
                        // this is a variable
                        let variable_name = statement;
                        let variable_value = self.get_variable_value(&variable_name);
                        if let Some(value) = variable_value {
                            let value_str = match value {
                                VariableValue::Integer(i) => i.to_string(),
                                VariableValue::Float(f) => f.to_string(),
                                VariableValue::Boolean(b) => b.to_string(),
                                VariableValue::String(s) => s,
                                VariableValue::Array(a) => format!("{:?}", a.clone()),
                                VariableValue::Table(t) => format!("{:?}", t.clone()),
                            };
                            println!("{} {}.\n  => Line: {}\n  => Content: {}", Colour::Red.paint("[ERROR]"), value_str, line_count, line);
                            std::process::exit(1);
                        } else {
                            println!("{} Unknown variable.\n  => Line: {}\n  => Content: {}", Colour::Red.paint("[ERROR]"), line_count, line);
                            std::process::exit(1);
                        }
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
                    let mut parts = line.splitn(3, ':').map(str::trim);
                    let name = parts.next().unwrap_or("");
                    if name.is_empty() {
                        println!(
                            "{} Missing name of variable.\n  => Line: {}\n  => Content: {}",
                            Colour::Red.paint("[ERROR]"),
                            line_count,
                            line
                        );
                        std::process::exit(1);
                    }
                    let datatype = parts.next().unwrap_or("");
                    let mut datatype_parts = datatype.splitn(2, '=').map(str::trim);
                    let datatype_str = datatype_parts.next().unwrap_or("");
                    if datatype_str.is_empty() {
                        println!(
                            "{} Missing datatype of variable.\n  => Line: {}\n  => Content: {}",
                            Colour::Red.paint("[ERROR]"),
                            line_count,
                            line
                        );
                        std::process::exit(1);
                    }
                    let datatype = match datatype_str {
                        "string" => VariableType::String,
                        "int" => VariableType::Integer,
                        "bool" => VariableType::Boolean,
                        "array" => {
                            let inner_datatype_str = parts.next().unwrap_or("");
                            if inner_datatype_str.is_empty() {
                                panic!("Missing inner datatype of array variable.");
                            }
                            let inner_datatype = match inner_datatype_str {
                                "string" => VariableType::String,
                                "int" => VariableType::Integer,
                                "bool" => VariableType::Boolean,
                                _ => panic!("Unknown variable type: {}", inner_datatype_str),
                            };
                            VariableType::Array(Box::new(inner_datatype))
                        }
                        "table" => {
                            let types = parts.collect::<Vec<_>>();
                            if types.len() != 2 {
                                panic!("Table should have two types: key and value")
                            }
                            let key_type = match types[0] {
                                "string" => VariableType::String,
                                "int" => VariableType::Integer,
                                "bool" => VariableType::Boolean,
                                _ => panic!("Unknown variable type: {}", types[0]),
                            };
                            let value_type = match types[1] {
                                "string" => VariableType::String,
                                "int" => VariableType::Integer,
                                "bool" => VariableType::Boolean,
                                _ => panic!("Unknown variable type: {}", types[1]),
                            };
                            VariableType::Table(Box::new((key_type, value_type)))
                        }   
                        _ => panic!("Unknown variable type: {}", datatype_str),
                    };
                    let value = line.splitn(2, " = ").nth(1).map(|s| s.trim_end_matches(';').trim_matches('"').to_string()).unwrap_or_default();
                    if value.is_empty() {
                        println!(
                            "{} There isn't any value.\n  => Line: {}\n  => Content: {}",
                            Colour::Red.paint("[ERROR]"),
                            line_count,
                            line
                        );
                        std::process::exit(1);
                    }
                    let variable = Variable::new(name.to_string(), datatype, VariableValue::String(value));
                    self.variables.push(variable);
                }            
                else if line.starts_with("fn ") && line.ends_with("():") {
                    // function declaration
                    let name = line.trim()[3..line.len()-2].to_string();
                    println!("Function: {}", name);
                }
                else {
                    // unrecognized syntax
                    println!("{} Unrecognized syntax.\n  => Line: {}\n  => Content: {}", Colour::Red.paint("[ERROR]"), line_count, line);
                    std::process::exit(1);
                }
            }
        }
    }

    // Variables
    #[allow(dead_code)]
    #[derive(PartialEq)]
    enum VariableType {
        Integer,
        Float,
        Boolean,
        String,
        Array(Box<VariableType>),
        Table(Box<(VariableType, VariableType)>),
    }

    struct Variable {
        name: String,
        value: VariableValue,
        datatype: VariableType,
    }

    impl Variable {
        fn new(name: String, datatype: VariableType, value: VariableValue) -> Self {
            Variable {
                name,
                datatype,
                value,
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    enum VariableValue {
        Integer(i32),
        Float(f32),
        Boolean(bool),
        String(String),
        Array(Vec<VariableValue>),
        Table(Vec<(VariableValue, VariableValue)>),
    }

    impl Interpreter {
        fn get_variable_value(&self, variable_name: &str) -> Option<VariableValue> {
            for variable in &self.variables {
                if variable.name == variable_name {
                    return Some(variable.value.clone());
                }
            }
            None
        }

        #[allow(dead_code)]
        fn get_variable_value_type(value: &VariableValue) -> VariableType {
            match value {
                VariableValue::Boolean(_) => VariableType::Boolean,
                VariableValue::Integer(_) => VariableType::Integer,
                VariableValue::Float(_) => VariableType::Float,
                VariableValue::String(_) => VariableType::String,
                VariableValue::Array(values) => VariableType::Array(Box::new(Self::get_variable_value_type(&values[0]))),
                VariableValue::Table(values) => VariableType::Table(Box::new((Self::get_variable_value_type(&values[0].0), Self::get_variable_value_type(&values[0].1)))),
            }
        }

        #[allow(dead_code)]
        fn set_variable_value(&mut self, name: &str, value: VariableValue) -> bool {
            for variable in self.variables.iter_mut().rev() {
                if variable.name == name {
                    if variable.datatype == Self::get_variable_value_type(&value) {
                        variable.value = value;
                        return true;
                    } else {
                        return false;
                    }
                }
            }
            false
        }
    }

    struct Interpreter {
        variables: Vec<Variable>,
    }

    // Debug
    fn count_quotes(s: &str) -> usize {
        s.chars().filter(|c| *c == '"').count()
    }

    impl PartialEq for Variable {
        fn eq(&self, other: &Self) -> bool {
            self.datatype == other.datatype
            
        }
    }

    impl Clone for VariableValue {
        fn clone(&self) -> Self {
            match self {
                VariableValue::Integer(i) => VariableValue::Integer(*i),
                VariableValue::Float(f) => VariableValue::Float(*f),
                VariableValue::Boolean(b) => VariableValue::Boolean(*b),
                VariableValue::String(s) => VariableValue::String(s.clone()),
                VariableValue::Array(a) => VariableValue::Array(a.clone()),
                VariableValue::Table(t) => VariableValue::Table(t.clone()),
            }
        }
    }

    fn main() {
        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            println!("Usage: cpr filename.cpr");
            return;
        }

        let filename = &args[1];
        let mut interpreter = Interpreter::new();
        interpreter.parse(filename);
    }