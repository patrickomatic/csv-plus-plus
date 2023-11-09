use crate::{Error, Result, Runtime, Template};
use std::fs;
use std::path;

impl Template {
    pub fn write_object_file(&self, runtime: &Runtime) -> Result<path::PathBuf> {
        runtime.progress("Writing object file");

        let object_code_filename = runtime.source_code.object_code_filename();

        let object_file = fs::File::create(&object_code_filename).map_err(|e| {
            runtime.error(format!("IO error: {e:?}"));
            Error::ObjectWriteError {
                filename: object_code_filename.clone(),
                message: format!("Error opening object code for writing: {e}"),
            }
        })?;

        serde_cbor::to_writer(object_file, self).map_err(|e| {
            runtime.error(format!("CBOR write error: {e:?}"));
            Error::ObjectWriteError {
                filename: object_code_filename.clone(),
                message: format!("Error serializing object code for writing: {e}"),
            }
        })?;

        Ok(object_code_filename)
    }

    // pub fn read_object_file(&self) ->
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn write_object_file() {}
}
