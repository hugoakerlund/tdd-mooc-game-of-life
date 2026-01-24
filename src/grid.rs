pub struct Grid {
    grid: Vec<Vec<char>>,
    pattern: String,
    width: i8,
    height: i8,
}

impl Grid {
    pub fn new(pattern: &str, width: i8, height: i8) -> Self {
        Self {
            grid: Vec::new(),
            pattern: pattern.to_string(),
            width: width,
            height: height,
        }
    }
    pub fn to_string(&self) -> String {
        let result = self
            .grid
            .clone()
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        result
    }

    pub fn get_pattern_width(&self) -> i8 {
        self.width
    }

    pub fn get_pattern_height(&self) -> i8 {
        self.height
    }

    pub fn get_pattern(&self) -> String {
        self.pattern.clone()
    }

    pub fn rle_pattern_to_grid(&mut self) {
        let new_pattern = self.expand_empty_lines();
        let rows: Vec<&str> = new_pattern.split("$").collect();
        for row in rows {
            let parsed_row = self.decode_row(row);
            self.grid.push(parsed_row);
        }
    }

    fn expand_empty_lines(&self) -> String {
        let mut new_pattern: String = String::new();
        let mut run_count = String::new();
        for i in 0..self.pattern.len() {
            let current_char: char = self.pattern.chars().nth(i).unwrap();

            if current_char.is_digit(10) {
                run_count += &current_char.to_string();
                continue;
            }

            if current_char != '$' {
                new_pattern += &run_count.to_string();
                new_pattern.push(current_char);
                run_count = "".to_string();
                continue;
            } else {
                if run_count.len() > 0 {
                    let count: i8 = run_count.parse().unwrap();
                    for _j in 0..count {
                        new_pattern.push('$');
                        run_count = "".to_string();
                    }
                    continue;
                }
            }
            new_pattern.push(current_char);
        }
        new_pattern
    }

    fn decode_row(&self, row: &str) -> Vec<char> {
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
                let count: i8 = run_count.parse().unwrap();
                for _j in 0..count {
                    parsed_row.push(converted_char);
                }
                run_count = "".to_string();
            } else {
                parsed_row.push(converted_char);
            }
        }
        while parsed_row.len() < self.width.try_into().unwrap() {
            parsed_row.push('.');
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
            _ => Err("Invalid character"),
        }
    }

    pub fn grid_to_rle_pattern(&self) -> String {
        let rows: Vec<String> = self
            .grid
            .clone()
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>();

        let pattern_rows: Vec<String> = self.encode_row(rows);
        let mut pattern: String = pattern_rows.join("$");
        pattern = self.compress_empty_lines(&mut pattern);
        pattern += "!";
        pattern
    }

    fn encode_row(&self, rows: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for i in 0..rows.len() {
            let row: &String = &rows[i];
            let mut new_row: String = String::new();
            let mut current_char;
            let mut run_count: i8 = 1;

            let mut j: usize = 0;
            while j < row.len() {
                current_char = row.chars().nth(j).unwrap();
                if j < row.len() - 1 {
                    let mut next_char = row.chars().nth(j + 1).unwrap();
                    while j < row.len() - 1 && next_char == current_char {
                        run_count += 1;
                        j += 1;

                        if j < row.len() - 1 {
                            next_char = row.chars().nth(j + 1).unwrap();
                        } else {
                            break;
                        }
                    }
                }
                let converted_char = self.convert_char(current_char).unwrap();

                if j == row.len() - 1 && converted_char == 'b' {
                    break;
                } else if run_count == 1 {
                    new_row += &converted_char.to_string();
                } else if run_count > 1 {
                    let count: String = run_count.to_string();
                    new_row += &count;
                    new_row += &converted_char.to_string();
                }
                j += 1;
                run_count = 1;
            }
            result.push(new_row);
        }
        result
    }

    fn compress_empty_lines(&self, pattern: &mut String) -> String {
        let mut new_pattern = String::new();
        let mut empty_line_count;
        let mut i: usize = 0;
        while i < pattern.len() {
            let current_char = pattern.chars().nth(i).unwrap();
            if current_char == '!' {
                break;
            }
            if current_char != '$' {
                new_pattern.push_str(&current_char.to_string());
            }
            if i < pattern.len() - 1 && current_char == '$' {
                let mut next_char = pattern.chars().nth(i + 1).unwrap();
                if next_char == '$' {
                    empty_line_count = 0;
                    while next_char == '$' {
                        empty_line_count += 1;
                        i += 1;
                        if i < pattern.len() {
                            next_char = pattern.chars().nth(i).unwrap();
                            if next_char != '$' {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    if empty_line_count > 1 {
                        new_pattern.push_str(&empty_line_count.to_string());
                    }
                }
                new_pattern.push('$');
            }
            i += 1;
        }
        new_pattern
    }

    pub fn get_cell_at(&self, row: usize, col: usize) -> char {
        return self.grid[row][col];
    }

    pub fn set_cell_at(&mut self, row: usize, col: usize, value: char) -> () {
        self.grid[row][col] = value;
    }

    pub fn count_live_neighbours(&self, row: i8, col: i8) -> i8 {
        let moves: Vec<(i8, i8)> = vec![
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let mut count: i8 = 0;
        for (first, second) in moves {
            let new_row = row + first;
            let new_col = col + second;
            if self.is_alive(new_row, new_col) {
                count += 1;
            }
        }
        count
    }

    fn is_inside_grid(&self, row: i8, col: i8) -> bool {
        return row < self.height && row >= 0 && col < self.width && col >= 0;
    }

    pub fn is_alive(&self, row: i8, col: i8) -> bool {
        if !self.is_inside_grid(row, col) {
            return false;
        }
        return self.grid[row as usize][col as usize] == '*';
    }

    pub fn will_cell_live(&self, row: i8, col: i8) -> bool {
        let live_neighbours = self.count_live_neighbours(row as i8, col as i8);
        let mut is_alive: bool = self.is_alive(row, col);
        if !is_alive && live_neighbours == 3 {
            is_alive = true;
        } else if is_alive && (live_neighbours < 2 || live_neighbours > 3) {
            is_alive = false;
        }
        is_alive
    }

    pub fn detect_if_needs_expansion(&self) -> (bool, i8, i8, i8, i8) {
        let mut needs_expansion: bool = false;
        let mut top: i8 = 0;
        let mut bottom: i8 = 0;
        let mut left: i8 = 0;
        let mut right: i8 = 0;

        for i in 0..self.width {
            if self.count_live_neighbours(-1, i) == 3 {
                top = 1;
                break;
            }
        }
        for i in 0..self.width {
            if self.count_live_neighbours(self.height, i) == 3 {
                bottom = 1;
                break;
            }
        }
        for i in 0..self.height {
            if self.count_live_neighbours(i, -1) == 3 {
                left = 1;
                break;
            }
        }
        for i in 0..self.height {
            if self.count_live_neighbours(i, self.width) == 3 {
                right = 1;
                break;
            }
        }
        if top == 1 || bottom == 1 || left == 1 || right == 1 {
            needs_expansion = true;
        }
        (needs_expansion, top, bottom, left, right)
    }

    pub fn expand_grid(&mut self, top: i8, bottom: i8, left: i8, right: i8) {
        let new_width = self.width + left + right;
        let new_height = self.height + top + bottom;
        let mut new_grid: Vec<Vec<char>> = vec![vec!['.'; new_width as usize]; new_height as usize];
        for i in 0..self.height {
            for j in 0..self.width {
                let row = (top + i) as usize;
                let col = (left + j) as usize;
                new_grid[row][col] = self.grid[i as usize][j as usize];
            }
        }
        self.width = new_width;
        self.height = new_height;
        self.grid = new_grid;
    }

    pub fn next_generation(&mut self) {
        let grid_expansion = self.detect_if_needs_expansion();
        if grid_expansion.0 {
            let top = grid_expansion.1;
            let bottom = grid_expansion.2;
            let left = grid_expansion.3;
            let right = grid_expansion.4;
            self.expand_grid(top, bottom, left, right);
        }
        let mut next_generation_grid: Vec<Vec<char>> =
            vec![vec!['.'; self.width as usize]; self.height as usize];
        for row in 0..self.height {
            for col in 0..self.width {
                if self.will_cell_live(row, col) {
                    next_generation_grid[row as usize][col as usize] = '*';
                }
            }
        }
        self.grid = next_generation_grid;
    }
}
