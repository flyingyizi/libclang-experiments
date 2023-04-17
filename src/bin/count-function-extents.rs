use std::path::PathBuf;

use clang::*;

fn main() {
    let resolved_path = PathBuf::from("examples/functions.cc");
    println!("Parsing  {:?} ...", resolved_path);

    let clang = Clang::new().unwrap();

    let index = Index::new(&clang, false, true);

    let default_arguments = ["-std=c++14"];
    let tu: TranslationUnit = index
        .parser(resolved_path)
        .arguments(&default_arguments)
        .parse()
        .unwrap_or_else(|_| panic!("Cannot parse "));

    let vv = collect_m(tu.get_entity());
    for v in vv {
        visit_functions(v);
    }
}

fn collect_m<'tu>(e: Entity<'tu>) -> Vec<Entity<'tu>> {
    let vv = e
        .get_children()
        .into_iter()
        .filter(|e| {
            let k = e.get_kind();

            k == EntityKind::FunctionDecl
                || k == EntityKind::Method
                || k == EntityKind::FunctionTemplate
        })
        .collect::<Vec<_>>();
    vv
}

fn visit_functions(e: Entity) {
    let range = e.get_range().unwrap();
    let start_location = range.get_start().get_spelling_location().line;
    let end_location = range.get_end().get_spelling_location().line;

    let name = e.get_name().unwrap();
    println!("{}:{}", name, end_location - start_location);
}
