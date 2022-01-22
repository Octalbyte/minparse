

#[cfg(test)]
mod tests {
    use std::env;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }


    #[test]
    fn get_proc_name() {

        let result = crate::minparse::process_name();
        let args: Vec<String> = env::args().collect();
        assert_eq!(result, args[0]);
    }
}

pub mod minparse {

    use std::{env, collections::HashMap};

    pub fn process_name() -> String {
        let args = env::args();
        let args: Vec<String> = args.collect();
        let first = &args[0];
        return first.to_string();
    }

    pub fn subcommands() -> Vec<String>{
        let mut subcommands: Vec<String> = vec![];
        for i in env::args() {
            if i.starts_with("--") {
                break;
            }
            subcommands.push(i);
        }
        return subcommands;
    }

    pub fn switches() -> Vec<String> {
        let mut switches: Vec<String> = vec![];
        let args: Vec<String> = env::args().collect();
        let mut c_index: usize = 0;
        for i in &args {
            if c_index == 0 {
                c_index = 1;
                continue;
            }
            if i.starts_with("--"){
                if !(args.len() <= c_index+1) {
                    if args[c_index+1].starts_with("--") || c_index + 1 == args.len() {
                        switches.push(i.to_owned());
                    }
                }
                if args.len() == c_index +1 {
                    switches.push(i.to_owned());
                }

            }
            c_index = c_index+1;
        }
        return switches;
    }

    pub fn fields() -> HashMap<String, String> {
        let mut fields: HashMap<String, String> = HashMap::new();
        let args: Vec<String> = env::args().collect();
        let mut c_index: usize = 0;
        for i in &args {
            if c_index == 0 {
                c_index = 1;
                continue;
            }
            if i.starts_with("--"){
                if !(args.len() <= c_index+1) {
                    if !args[c_index+1].starts_with("--"){
                        fields.insert((&i.to_owned()).to_owned(), (&args[c_index+1]).to_owned());
                    }
                }
            }
            c_index = c_index+1;
        }
        return fields;
    }
}