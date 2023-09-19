use super::Template;
use crate::ast::{Functions, Variables};
use crate::{Result, SourceCode, Spreadsheet};
use serde::{Deserialize, Serialize};
use std::convert;
use std::path;

/// A template stripped down to just it's serializable fields.  This is internal to this module and
/// should be converted as we read from or write to the object files.
#[derive(Deserialize, Serialize)]
struct TemplateAtRest {
    pub functions: Functions,
    pub spreadsheet: Spreadsheet,
    pub variables: Variables,
    csv_line_number: usize,
}

impl convert::From<&Template<'_>> for TemplateAtRest {
    fn from(template: &Template) -> Self {
        TemplateAtRest {
            functions: template.functions.clone(),
            spreadsheet: template.spreadsheet.borrow().clone(),
            variables: template.variables.clone(),
            csv_line_number: template.csv_line_number,
        }
    }
}

impl Template<'_> {
    /* TODO: read and use object files for linking
    fn from_template_at_rest(&self) -> Self {
        todo!()
    }
    */

    pub fn write_object_file(&self, source_code: &SourceCode) -> Result<path::PathBuf> {
        let object_code_filename = source_code.object_code_filename();
        /* TODO spend some more time thinking about what would be a good representation
        // let mut s = flexbuffers::FlexbufferSerializer::new();

        let template_at_rest = TemplateAtRest::from(self);
        // let serializer = template_at_rest.serialize(&mut s).unwrap();
        let file = fs::File::create(&object_code_filename).unwrap();
        let writer = ciborium::into_writer(&template_at_rest, &file).unwrap();
        fs::write(&object_code_filename, writer).map_err(|e| {
            Error::ObjectWriteError {
                filename: object_code_filename.clone(),
                message: format!("Error writing object file: {}", e),
            }
        })?;
        */

        Ok(object_code_filename)
    }
}
