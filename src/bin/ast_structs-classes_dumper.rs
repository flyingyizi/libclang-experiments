use std::env;
use std::path::{ PathBuf};
use clang::*;

fn main() {
    let mut args = env::args_os().skip(1);
    let resolved_path = args.next();

    let resolved_path = PathBuf::from(resolved_path.unwrap_or("examples/structs.hpp".into()));
    println!("Parsing  {:?} ...", resolved_path);

    // Acquire an instance of `Clang`
    let clang = Clang::new().unwrap();

    // Create a new `Index`
    let index = Index::new(&clang, false, false);

    // Parse a source file into a translation unit
    let tu = index.parser(resolved_path).parse().unwrap();
    //or outside got ast file:`clang++ -emit-ast -o structs.ast ./examples/structs.hpp`
    //then: let tu = TranslationUnit::from_ast(index, structs.ast);

    // Get the structs in this translation unit
    let structs = tu
        .get_entity()
        .get_children()
        .into_iter()
        .filter(|e| e.get_kind() == EntityKind::StructDecl)
        .collect::<Vec<_>>();
    let classes = tu
        .get_entity()
        .get_children()
        .into_iter()
        .filter(|e| e.get_kind() == EntityKind::ClassDecl)
        .collect::<Vec<_>>();

    println!("structs:--------------------");
    for s in structs {
        visit_entity(s, 0);
    }
    println!("--------------------");

    println!("classes:--------------------");
    for cls in classes {
        visit_entity(cls, 0);
    }
    println!("--------------------");
}

fn visit_entity(e: Entity, lev: usize) {
    let spaces = (0..lev).map(|_| "-").collect::<String>();

    match e.get_kind() {
        EntityKind::AccessSpecifier => {
            println!(
                "{spaces}{k:?}  {acc:?}",
                k = e.get_kind(),
                acc = e.get_accessibility().unwrap()
            );
        }

        _ => {
            println!(
                "{spaces}{k:?} {n:?}  {acc:?}",
                k = e.get_kind(),
                n = e.get_name().unwrap_or_default(),
                acc = e.get_accessibility()
            );

            for f in e.get_children() {
                visit_entity(f, lev + 1)
            }
        }
    }
}
