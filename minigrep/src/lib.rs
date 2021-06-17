use std::env;
use std::error::Error;
use std::fmt;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));        
    }
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args)-> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough parameters");
        }
        args.next(); 
        let query = match args.next() {
            Some(arg)=> arg,
            None=>return Err("query string, i.e. first parameter is missing")
        };
        let filename = match args.next() {
            Some(arg)=>arg,
            None=>return Err("file name, i.e. second parameter is missing")
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive } )
    }
}

pub fn search_case_sensitive<'a>(query: &str, contents:&'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents:&'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[derive(Debug)]
pub struct FileError {
    file_name : String,
    inner_error: std::io::Error
}

impl std::error::Error for FileError {}
impl FileError {
    pub fn new(file_name:&str, io_error: std::io::Error) -> FileError{
        FileError {
            file_name: String::from(file_name),
            inner_error: io_error
        }
    } 
}

impl fmt::Display for FileError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}-{}", self.file_name, self.inner_error)
  }
}

pub fn run(config:Config)->Result<(), Box<dyn Error>> {
    let Config {filename, query, case_sensitive}  = config;
    let name =String::from(&filename[..]);

    let read_result = fs::read_to_string(name);
    let contents = match read_result{
        Ok(file_content)=>file_content,
        Err(e)=>return Err(Box::new(FileError::new(&filename[..], e)))
    }; 

    let results = if case_sensitive {
        search_case_sensitive(&query, &contents)
    }
    else { 
        search_case_insensitive(&query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}