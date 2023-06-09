use crate::compiler::Template;

pub trait CompilerTarget {
    fn write(&self, template: &Template) -> Result<(), CsvppError>;
}
