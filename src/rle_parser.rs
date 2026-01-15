pub struct RLEParser {
    file_contents: String,
    name: String,
    creator: String,
    comments: String,
    header: String,
    pattern: String,
    width: i8,
    height: i8,
    rule: String
}

impl RLEParser {
    pub fn new(file_contents: &str) -> Result<Self, &'static str>{
        Ok(Self { file_contents: file_contents.to_string(),
                  name: String::new(),
                  creator: String::new(),
                  comments: String::new(),
                  header: String::new(),
                  pattern: String::new(),
                  width: 0,
                  height: 0,
                  rule: String::new()
                 })
    }

    pub fn get_file_contents(&self) -> String {
        self.file_contents.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_header(&self) -> String {
        self.header.clone()
    }

    pub fn get_comments(&self) -> String {
        self.comments.clone()
    }

    pub fn get_pattern(&self) -> String {
        self.pattern.clone()
    }

    pub fn get_creator(&self) -> String {
        self.creator.clone()
    }

    pub fn get_width(&self) -> i8 {
        self.width
    }

    pub fn get_height(&self) -> i8 {
        self.height
    }

    pub fn get_rule(&self) -> String {
        self.rule.clone()
    }

    pub fn parse_file(&mut self) -> () {
        for line in self.file_contents.split("\n") {
            if line.starts_with("#N") {
                self.name = line[3..line.len()].trim().to_string();
            }
            else if line.starts_with("#O") {
                self.creator = line[3..line.len()].trim().to_string();
            }
            else if line.starts_with("#C") {
                self.comments += &line[3..line.len()].to_string();
                self.comments += "\n";
            }
            else if line.starts_with("x") {
                self.header = line.trim().to_string();
            }
            else if self.header.len() > 0 {
                self.pattern += line.trim();
            }
        }
        self.parse_header();
    }

    pub fn parse_header(&mut self) {
        let header_parts: Vec<&str> =  self.header.split(", ").collect();
        let width_part = header_parts[0];
        let height_part= header_parts[1];
        let rule_part = header_parts[2];

        let width = width_part.replace("x = ", "");
        let height = height_part.replace("y = ", "");
        let rule = rule_part.replace("rule = ", "");

        self.width = width.parse().unwrap();
        self.height = height.parse().unwrap();
        self.rule = rule.to_string();
    }
}
