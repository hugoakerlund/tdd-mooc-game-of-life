pub struct RLEParser {
    file_contents: String,
    name: String,
    creator: String,
    comments: String,
    header: String,
    pattern: String
}

impl RLEParser {
    pub fn new(file_contents: &str) -> Result<Self, &'static str>{
        Ok(Self { file_contents: file_contents.to_string(),
                  name: String::new(),
                  creator: String::new(),
                  comments: String::new(),
                  header: String::new(),
                  pattern: String::new()
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

    pub fn parse_file(&mut self) -> () {
        for line in self.file_contents.split("\n") {
            if line.starts_with("#N") {
                self.name = line[3..line.len()].to_string();
            }
            else if line.starts_with("#O") {
                self.creator = line[3..line.len()].to_string();
            }
            else if line.starts_with("#C") {
                self.comments += &line[3..line.len()].to_string();
                self.comments += "\n";
            }
            else if line.starts_with("x") {
                self.header = line.to_string();
            }
            else if self.header.len() > 0 {
                self.pattern += line;
            }
        }
    }
}
