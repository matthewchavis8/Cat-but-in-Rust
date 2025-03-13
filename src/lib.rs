
use std::fs;
use std::io::Write;

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
        } else if self.ignore_blank_line_mode {
            content
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| format!("{}", line))
                .collect::<Vec<String>>()
                .join("\n")
        } else {
            content
        };

        let res = parsed_content;
        self.parsed_content.push(res.clone());
        self.output.push_str(&res);
        res
    }

    pub fn redirection(&mut self) {
        if let Some(pos) = self.file_content.iter().position(|it| it == ">") {
            let files_to_redirect = self.file_content[..pos].to_vec().clone();
            let target_path = self.file_content[pos + 1].clone();

            for file_path in files_to_redirect {
                self.parse_file(file_path.as_str());
            }
            
            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(target_path)
                .expect("Could not open file");

            write!(file, "{}", self.output)
                .expect("Could not write to the file")

        }
    }

    pub fn result(self) -> String {
        if !self.parsed_content.is_empty() {
            if self.reversed_parse_mode {
                let mut rev = self.parsed_content.clone();
                rev.reverse();
                rev.join("\n\n")
            } else {
                self.parsed_content.join("\n\n")
            }
        } else {
            panic!("parsed Content was empty")
        }
    }
} 