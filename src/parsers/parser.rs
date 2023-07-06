use crate::core::notebook::Notebook;

pub trait Encoder {
    fn encode(&self, text: &str) -> Result<Notebook, ParseError>;
}

pub trait Decoder {
    fn decode(&self, notebook: Notebook) -> Result<String, ParseError>;
    // fn decode(%self, task: Task) -> Result<String, ParseError>;
}

#[derive(Debug)]
pub enum ParseError {}
