use clap::{ Parser };
use std::fs;
use std::io::Write;
use tempfile;

#[derive(Parser, Debug)]
pub struct Args {
    //Takes in multiple input strings
    pub files: Vec<String>,
}

pub struct Cat {
    pub content: Vec<String>,
    pub line_count: u8,
    pub write_mode: bool,
    pub line_number_mode: bool,
}

impl Cat {
 /****************************************************************************
 * function    : parseArg 
 * description : takes in a file and convert it into a string. Then we add it to our vector
 * argument(s) : 
 *    - path to the file
 * return void     : 
 ****************************************************************************/
    pub fn parse_file(&mut self, path: String) {
        let file = fs::read_to_string(path)
        .expect("error parsing file");
        self.content.push(file);
    }

    pub fn run(&mut self) {
        for text in self.content.iter() {
            println!("{}", text);
        }
    }
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
            line_count: 1,
            write_mode: false,
            line_number_mode: false,
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    // Test if file parsing is working correctly
    #[test]
    fn test_parse_file() {
        //Creating a temporary file
        let mut tmp_file = tempfile::NamedTempFile::new()
        .expect("Failed to create temp file");

        write!(tmp_file, "hello world").expect("Failed to write content to file");

        let mut cat = Cat::new();

        let file_path = tmp_file.path().to_str().unwrap();

        cat.parse_file(file_path.to_string());
        
        assert_eq!(cat.content, vec!["hello world"]);
    }
   
}
