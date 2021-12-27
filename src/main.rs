fn main() {
    let mut pattern: String = String::new();

    for (index, arg) in std::env::args().enumerate() {
        if index == 1 {
            pattern = arg;
        } else if index > 1 {
            match std::fs::read_to_string(&arg) {
                Ok(file) => {
                    println!("Matches in the file: {}", &arg);

                    for (index, line) in file.lines().enumerate() {
                        if line.contains(&pattern) {
                            let col = line.find(&pattern).unwrap();
                            let content_match =
                                &line[col..line[col..].find(" ").unwrap_or(line.len()) + col];

                            println!("row {} / col {} \n {:#?}", index, col, content_match);
                        }
                    }
                }
                Err(_) => println!("Can't read file at the path: {}", arg),
            };
        }
    }
}
