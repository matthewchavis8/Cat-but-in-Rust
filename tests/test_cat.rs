use cat_rust::Cat;

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_cat_construct() {
        let cat1 = Cat::new();

        let cat2: Cat = Cat{
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
        };

        assert_eq!(cat1, cat2);
    }

    #[test]
    fn test_parse_file() {
        let mut cat = Cat::new();
        let file_path = "test.txt";

        let res = cat.parse_file(file_path);

        assert_eq!(res, "Hello world\n");
    }

    #[test]
    fn test_results() {
        let mut cat = Cat::new();
        let file_path = "test.txt";
        cat.parse_file(file_path);

        assert_eq!(cat.result(), "Hello world\n");

    }

    #[test]
    fn test_redirect_to_file() {
        let dir = tempdir().unwrap();
        let file_path1 = dir.path().join("test1.txt");
        let file_path2 = dir.path().join("test2.txt");
        let file_path3 = dir.path().join("test3.txt");

        let mut file1 = File::create(&file_path1).unwrap();
        writeln!(file1, "Hello").unwrap();

        let mut file2 = File::create(&file_path2).unwrap();
        writeln!(file2, "World").unwrap();

        let _file3 = File::create(&file_path3).unwrap();

        let mut cat = Cat::new();
        cat.file_content = vec![
            file_path1.to_str().unwrap().to_string(),
            file_path2.to_str().unwrap().to_string(),
            ">".to_string(),
            file_path3.to_str().unwrap().to_string(), 
        ];
        cat.redirection();

        let res = fs::read_to_string(&file_path3).unwrap();

        assert_eq!(res, "Hello\nWorld\n");
    }

    #[test]
    fn test_parse_without_blank_lines() {
        let mut cat = Cat::new();
        cat.ignore_blank_line_mode = true; 

        let file_path = "test.txt";

        let res = cat.parse_file(file_path);

        assert_eq!(res, "Hello world");
    }
}