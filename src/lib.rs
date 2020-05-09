use std::error::Error;
use std::fs;

/// Configuration to contain basic information of file and match operation
pub struct Configs {
    pattern: String,
    filename: String,
    is_case_sensitive: bool,
}

impl Configs {
    /// create new configuration
    pub fn new(args: &mut std::env::Args) -> Result<Configs, &'static str> {
        if args.len() < 3 {
            return Err("\
minigrep [pattern] [filename] case
adding \"case\" is optional to enable case sensitive grep");
        }
        
        args.next();
        
        let pattern = match args.next() {
            Some(arg) => arg,
            None => return Err("Pattern Not got")
        };
        
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Filenane Not got")
        };
        
        let is_case_sensitive = match args.next() {
            Some(arg) => arg == "case",
            None => false
        };
        
        Ok(Configs {
            pattern,
            filename,
            is_case_sensitive,
        })
    }
}

/// processing point of program
pub fn run(conf: Configs) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(conf.filename)?;

    if conf.is_case_sensitive {
        print_output(search_case_sensitive(&conf.pattern, &contents));
    } else {
        print_output(search_case_insensitive(&conf.pattern, &contents));
    }
    
    Ok(())
}

fn search_case_sensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
    .filter(
        |lin| lin
        .contains(&pattern))
    .collect()
}

fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
    .filter(
        |lin| lin.to_lowercase()
        .contains(&pattern.to_lowercase()))
    .collect()
}

fn print_output(out: Vec<&str>) {
    println!("matched: {}", out.len());
    for x in out {
        println!("{}", x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let pattern = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(pattern, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let pattern = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitive(pattern, contents)
        );
    }
}
