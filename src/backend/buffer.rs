pub struct TextBuffer {
    lines: Vec<Line>,
}

impl TextBuffer {
    pub fn new() -> Self {
        TextBuffer {
            lines: vec![Line::new()],
        }
    }
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn lines(&self) -> impl Iterator<Item = &Line>{
        self.lines.iter()
    }

    pub fn get_line(&self, index: usize) -> Option<&Line> {
        self.lines.get(index)
    }
    pub fn get_line_mut(&mut self, index: usize) -> Option<&mut Line> {
        self.lines.get_mut(index)
    }

    /// allocate a new line at the specified index
    pub fn new_line(&mut self, index: usize){
        self.lines.insert(index, Line::new())
    }

    pub fn insert_line(&mut self, index: usize, content: String) {
        self.lines.insert(index, Line::from(content))
    }
}
pub struct Line(pub String, pub usize);

impl Line {
    pub fn new() -> Self {
        Line(String::new(), 0)
    }
}

impl From<String> for Line{
    fn from(content: String) -> Self {
        Self(content, 0)
    }
}