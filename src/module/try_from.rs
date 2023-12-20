use super::Module;
use crate::parser::code_section_parser::CodeSectionParser;
use crate::{ArcSourceCode, Error, ModulePath, Result, Scope, SourceCode, Spreadsheet};
use log::{debug, info};
use std::path;

impl TryFrom<path::PathBuf> for Module {
    type Error = Error;

    fn try_from(p: path::PathBuf) -> Result<Self> {
        info!("Loading module from {}", p.display());

        debug!("Loading SourceCode from {}", p.display());
        let source_code = ArcSourceCode::new(SourceCode::try_from(p)?);

        debug!("Loading spreadsheet section");
        let spreadsheet = Spreadsheet::parse(source_code.clone())?;

        debug!("Parsing code section");
        let (scope, required_modules) = if let Some(scope_source) = &source_code.code_section {
            let code_section = CodeSectionParser::parse(scope_source, source_code.clone())?;
            debug!("Parsed code section: {code_section:?}");
            code_section
        } else {
            (Scope::default(), vec![])
        };

        let module_path: ModulePath = source_code.filename.clone().try_into()?;
        debug!("Using ModulePath = {module_path}");

        let mut module = Self::new(source_code, module_path, scope, spreadsheet);
        module.required_modules = required_modules;
        Ok(module)
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
