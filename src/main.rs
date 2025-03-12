use clap::Parser;
use cat_rust::Cat;
#[derive(Parser, Debug)]
struct Arg {
    #[arg(short = 'n')]
    line_number_mode: bool,

    #[arg(short = 'w')]
    write_mode: bool,

    #[arg(short = 'r')]
    reverse_parse_mode: bool,

    #[arg(short = 'e')]
    mark_end_of_line_mode: bool,

    #[arg(short = 's')]
    ignore_blank_line_mode: bool,

    file_content: Vec<String>
}

fn main() {
    let args = Arg::parse();
    let mut cat = Cat::new();

    cat.line_number_mode = args.line_number_mode;
    cat.ignore_blank_line_mode = args.ignore_blank_line_mode;
    cat.write_mode = args.write_mode;
    cat.reversed_parse_mode = args.reverse_parse_mode;
    cat.mark_end_of_line_mode = args.mark_end_of_line_mode;

    cat.file_content = args.file_content;

    let file_paths = cat.file_content.split_off(0);

    for file_path in file_paths {
        cat.parse_file(file_path.as_str());
    }

    let result = cat.result();
    
    println!("{}", result);
}