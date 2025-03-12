use clap::Parser;
use std::fs;

#[derive(PartialEq, Debug)]
pub struct Cat {
    pub file_content: Vec<String>,
    pub parsed_content: Vec<String>,
    pub output: String,
    pub line_count: u16,

    pub line_number_mode: bool,
    pub write_mode: bool,
    pub redirection_mode: bool,
    pub reversed_parse_mode: bool,
    pub mark_end_of_line_mode: bool,
    pub ignore_blank_line_mode: bool,
}

impl Cat {
    pub fn new() -> Cat {
        Cat {
            file_content: Vec::new(),
            parsed_content: Vec::new(),
            output: String::new(),
            line_count: 0,

            line_number_mode: false,
            write_mode: false,
            redirection_mode: false,
            reversed_parse_mode: false,
            mark_end_of_line_mode: false,
            ignore_blank_line_mode: false,
        }
    }
    pub fn parse_file(&mut self, file_path: &str) -> String {
        let content = fs::read_to_string(file_path)
            .expect("Error parsing file");
        
        let parsed_content = if self.line_number_mode {
            content
                .lines()
                .map(|line| {
                    self.line_count += 1;
                    format!("{}: {}", self.line_count, line)
                })
                .collect::<Vec<String>>()
                .join("\n")
        } else if self.mark_end_of_line_mode {
            content
                .lines()
                .map(|line| {
                    format!("{} $.", line)
                })
                .collect::<Vec<String>>()
                .join("\n")
        } else {
            content
        };

        let res = parsed_content;
        self.parsed_content.push(res.clone());
        res
    }
}