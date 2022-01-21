

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod parse {

    use std::env;

    pub fn process_name() -> String {
        return env::args().collect();
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
            if i.starts_with("--"){
                if args[c_index].starts_with("--"){
                    switches.push(i.to_owned());
                }
            }
            c_index = c_index+1;
        }
        return switches;
    }
}