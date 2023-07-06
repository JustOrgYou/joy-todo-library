use crate::core::prelude::Notebook;
use crate::parsers::parser::{Decoder, ParseError};
pub struct OrgModeDecoder {}

impl Decoder for OrgModeDecoder {
    fn decode(&self, _notebook: Notebook) -> Result<String, ParseError> {
        Ok(String::from(
            "* [#A] Task 1\n\
             * Task 2",
        ))
    }
}
