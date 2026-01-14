pub struct Grid {
    grid: Vec<Vec<char>>,
    pattern: String,
    width: u8,
    height: u8
}

impl Grid {
    pub fn new(pattern: &str, width: u8, height: u8) -> Self {
        Self {
            grid: Vec::new(),
            pattern: pattern.to_string(),
            width: width,
            height: height
        }
    }

    pub fn get_width(&self) -> u8 {
        self.width
    }

    pub fn get_height(&self) -> u8 {
        self.height
    }

    pub fn get_pattern(&self) -> String {
        self.pattern.clone()
    }

    pub fn pattern_to_grid(&mut self) {
        let rows: Vec<&str> = self.pattern.split("$").collect();
        for row in rows {
            let parsed_row = self.parse_row(row);
            self.grid.push(parsed_row);
        }
    }

    fn parse_row(&self, row: &str) -> Vec<char> {
        let mut parsed_row: Vec<char> = Vec::new();
        let mut run_count = String::new();
        for i in 0..row.len() {
            let current_char: char = row.chars().nth(i).unwrap();

            if current_char.is_digit(10) {
                run_count += &current_char.to_string();
                continue;
            }
            
            let converted_char = self.convert_char(current_char).unwrap();
            if converted_char == '!' {
                break;
            }

            if run_count.len() > 0 {
                let count: u8 = run_count.parse().unwrap();
                println!("{}", count);
                for _j in 0..count {
                    parsed_row.push(converted_char);
                }
                run_count = "".to_string();
            }
            else {
                parsed_row.push(converted_char);
            }
        }
        parsed_row
    }

    fn convert_char(&self, c: char) -> Result<char, &'static str> {
        println!("{}", c);
        match c {
            'b' => Ok('.'),
            'o' => Ok('*'),
            '!' => Ok('!'),
            _ => Err("Invalid character")
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = self.grid.clone()
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        result
    }

}
