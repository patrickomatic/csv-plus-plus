use super::Module;
use crate::parser::code_section_parser::CodeSectionParser;
use crate::{ArcSourceCode, Error, ModulePath, Result, Scope, SourceCode, Spreadsheet};
use std::path;

impl TryFrom<path::PathBuf> for Module {
    type Error = Error;

    fn try_from(p: path::PathBuf) -> Result<Self> {
        let source_code = ArcSourceCode::new(SourceCode::try_from(p)?);
        let spreadsheet = Spreadsheet::parse(source_code.clone())?;

        let scope = if let Some(scope_source) = &source_code.code_section {
            let cs = CodeSectionParser::parse(scope_source, source_code.clone())?;
            cs
        } else {
            Scope::default()
        };

        let module_path: ModulePath = source_code.filename.clone().try_into()?;

        Ok(Self::new(source_code, module_path, scope, spreadsheet))
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
