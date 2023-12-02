//! # ModuleLoader
//!
use super::ModuleName;
// use crate::parser::code_section_parser::CodeSectionParser;
use crate::{CodeSection, Error, Result, SourceCode};
use std::collections;
use std::path;
use std::sync;
use std::thread;

type Attempted = sync::Arc<sync::RwLock<collections::HashSet<ModuleName>>>;
type Loaded = sync::Arc<sync::RwLock<collections::HashMap<ModuleName, CodeSection>>>;
type Failed = sync::Arc<sync::RwLock<collections::HashMap<ModuleName, Error>>>;

#[derive(Debug, Default)]
pub(super) struct ModuleLoader {
    pub(super) attempted: Attempted,
    pub(super) loaded: Loaded,
    pub(super) failed: Failed,
}

fn loader_thread(module_name: ModuleName, _loaded: Loaded, failed: Failed) {
    let p: path::PathBuf = module_name.clone().into();

    let _source_code = match SourceCode::open(&p) {
        Ok(s) => s,
        Err(e) => {
            let mut failed = failed.write().expect("attempted write");
            failed.insert(module_name, e);
            return;
        }
    };

    /*
    let code_section = if let Some(code_section_source) = &self.source_code.code_section {
        match CodeSectionParser::parse(code_section_source, self) {
        cs
    } else {
        // XXX insert an error into failed
    };
    */
    todo!()
}

impl ModuleLoader {
    /// Recursively and concurrently loads all of the `required_modules` for the given `code_section`.
    // TODO: get rid of expects
    pub(super) fn load(&self, code_section: &CodeSection) -> Result<&Self> {
        // only try the ones which we haven't.  it's possible another module could have already
        // loaded the ones we want
        let mut to_attempt = vec![];
        let attempted_borrow = sync::Arc::clone(&self.attempted);
        let mut attempted = attempted_borrow.write().expect("attempted write");
        for module_name in &code_section.required_modules {
            if attempted.contains(module_name) {
                continue;
            } else {
                // insert into `attempted` pre-emptively so none of the other threads attempt it
                attempted.insert(module_name.clone());
                to_attempt.push(module_name.clone());
            }
        }
        drop(attempted);

        let threads: Vec<thread::JoinHandle<_>> = to_attempt
            .into_iter()
            .map(|module_name| {
                let loaded = sync::Arc::clone(&self.loaded);
                let failed = sync::Arc::clone(&self.failed);
                thread::spawn(move || loader_thread(module_name, loaded, failed))
            })
            .collect();

        for t in threads {
            t.join().expect("thread join");
        }

        Ok(self)
    }

    // pub(super) fn into_loaded(&self) -> collections::HashMap<ModuleName, CodeSection> {

    // }
}
