pub struct Model {
    pub file_path: Option<String>,
    pub link: Option<String>,
}

impl Model {
    pub fn build(file_path: String) -> Self {
        let mut model = Self {
            file_path: Some(file_path),
            link: None,
        };
        model.generate_link();
        model
    }

    fn generate_link(&mut self) {
        self.link = Some(String::from("123"));
    }
}
