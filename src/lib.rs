use std::error::Error;
use std::fs;

pub struct Configs {
    pattern: String,
    filename: String,
    is_case_sensitive: bool,
}

impl Configs {
    pub fn new(args: &Vec<String>) -> Result<Configs, &'static str> {
        if args.len() < 3 {
            return Err("\
minigrep [pattern] [filename] case
adding \"case\" is optional to enable case sensitive grep");
        }

        let mut is_sensitive = false;

        if let Some(o) = args.get(3) {
            if o == "case" {
                is_sensitive = true;
            }
        }

        Ok(Configs {
            pattern: args[1].clone(),
            filename: args[2].clone(),
            is_case_sensitive: is_sensitive,
        })
    }
}

pub fn run(conf: Configs) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(conf.filename)?;

    if conf.is_case_sensitive {
        print_output(search_case_sensitive(&conf.pattern, &contents));
    } else {
        print_output(search_case_insensitive(&conf.pattern, &contents));
    }
    
    Ok(())
}

pub fn search_case_sensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vec: Vec<&'a str> = vec![];

    for line in contents.lines() {
        if line.contains(pattern) {
            vec.push(line);
        }
    }

    vec
}

pub fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vec: Vec<&'a str> = vec![];
    let pattern = pattern.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&pattern) {
            vec.push(line);
        }
    }
    vec
}

pub fn print_output(out: Vec<&str>) {
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
