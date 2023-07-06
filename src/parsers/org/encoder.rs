use crate::core::notebook::{Notebook, NotebookMetadata};
use crate::parsers::parser::{Encoder, ParseError};

pub struct OrgModeEncoder {}

impl Encoder for OrgModeEncoder {
    fn encode(&self, _text: &str) -> Result<Notebook, ParseError> {
        Ok(Notebook::new(
            NotebookMetadata::new(None, None, Vec::default()),
            Vec::default(),
        ))
    }
}
