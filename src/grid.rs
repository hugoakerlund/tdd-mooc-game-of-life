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
        match c {
            'b' => Ok('.'),
            'o' => Ok('*'),
            '.' => Ok('b'),
            '*' => Ok('o'),
            '!' => Ok('!'),
            _ => Err("Invalid character")
        }
    }

    pub fn to_string(&self) -> String {
        let result = self.grid.clone()
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        result
    }

    pub fn grid_to_pattern(&self) -> String {
        let rows: Vec<String> = self.grid.clone()
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>();

        let pattern_rows: Vec<String> = self.row_to_pattern(rows);
        let mut pattern: String = pattern_rows.join("$");
        pattern += "!";
        pattern
    }

    fn row_to_pattern(&self, rows: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for i in 0..rows.len(){
            let row: &String = &rows[i];
            let mut new_row: String = String::new();
            let mut current_char: char = row.chars().nth(0).unwrap();
            let mut run_count: u8 = 1;

            let mut j: usize = 0;
            while j < row.len(){
                current_char = row.chars().nth(j).unwrap();
                if j < row.len() - 1 {
                    let mut next_char = row.chars().nth(j + 1).unwrap();
                    while j < row.len() - 1 && next_char == current_char {
                        run_count += 1;
                        j += 1;
                        
                        if j < row.len() - 1 {
                            next_char = row.chars().nth(j + 1).unwrap();
                        }
                        else {
                            break;
                        }
                    }
                }
                current_char = self.convert_char(current_char).unwrap();

                if run_count == 1 {
                    new_row += &current_char.to_string();
                }

                else if run_count > 1 {
                    let count: String = run_count.to_string();
                    new_row += &count;
                    new_row += &current_char.to_string();
                }
                j += 1;
                run_count = 1;
            }
            result.push(new_row);
        }
    result
    }

    pub fn get_cell_at(&self, row: usize, col: usize) -> char {
        return self.grid[row][col];
    }

    pub fn set_cell_at(&mut self, row: usize, col: usize, value: char) -> () {
        self.grid[row][col] = value;
    }

    pub fn count_live_neighbours(&self, row: i8, col: i8) -> u8 {
        let moves: Vec<(i8, i8)> = vec![(1, 0), (1, -1), 
                                        (0, -1), (-1, -1), 
                                        (-1, 0), (-1, 1), 
                                        (0, 1), (1, 1)];
        let mut count: u8 = 0;
        for (first, second) in moves {
            let new_row = row + first;
            let new_col = col + second;
            if new_col < 0 || new_row < 0 {
                continue;
            }
            if self.is_inside_grid(new_row as u8, new_col as u8) && self.is_alive(new_row as usize, new_col as usize){
                count += 1;
            }
        }
        count
    }

    fn is_inside_grid(&self, row: u8, col: u8) -> bool {
        return row < self.height && row >= 0 && 
               col < self.width && col >= 0;
    }

    fn is_alive(&self, row: usize, col: usize) -> bool {
        return self.grid[row][col] == '*';
    }
}
