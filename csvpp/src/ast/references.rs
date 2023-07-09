use super::{FunctionName, VariableName};

#[derive(Debug)]
pub struct AstReferences {
    functions: Vec<FunctionName>,
    variables: Vec<VariableName>,
}

fn extract_dfs(ast: Ast, fns: &mut Vec<FunctionName>, vars: &mut Vec<VariableName>) {
    // TODO
}

/// Does a depth first search on `ast` and parses out all identifiers that might be able to be
/// eval()ed
fn extract_references(ast: Ast) -> AstReferences {
    let mut fns = vec![];
    let mut vars = vec![];

    extract_dfs(ast, &mut fns, &mut vars);

    AstReferences {
        functions: fns,
        variables: vars,
    }
}
