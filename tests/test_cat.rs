use cat_rust::Cat;

#[cfg(test)]
mod tests {
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

        println!("{}", res);

        assert_eq!(res, "Hello world");
    }
}