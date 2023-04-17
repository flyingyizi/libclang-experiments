# `libclang-experiments` &mdash; Some experiments with `libclang` 

This repository contains some of experiments with the `rust wrapper of libclang`:[clang-rs](https://github.com/KyleMayes/clang-rs).  

# Requirements

* refer the `rust clang-rs`'s requirements

# The experiments

Let me briefly explain how to use some of the experiments.

## `ast_structs-classes_dumper` 

These programs walk an abstract syntax tree, This should result in an output similar to this:

```text
Executing task: cargo run --package libclangdemo --bin ast_structs-classes_dumper 

Parsing  "examples/structs.hpp" ...
structs:--------------------
StructDecl "A"  None
-FieldDecl "a"  Some(Public)
-FieldDecl "b"  Some(Public)
-FieldDecl "c"  Some(Public)
-FieldDecl "d"  Some(Public)
StructDecl "B"  None
-FieldDecl "a"  Some(Public)
-FieldDecl "b"  Some(Public)
-FieldDecl "c"  Some(Public)
-FieldDecl "d"  Some(Public)
StructDecl "C"  None
-FieldDecl "a"  Some(Public)
-FieldDecl "b"  Some(Public)
-FieldDecl "c"  Some(Public)
-FieldDecl "d"  Some(Public)
StructDecl "D"  None
-FieldDecl "a"  Some(Public)
-FieldDecl "b"  Some(Public)
-FieldDecl "c"  Some(Public)
-FieldDecl "d"  Some(Public)
--------------------
classes:--------------------
ClassDecl "foo"  None
-AccessSpecifier  Public
-Method "method"  Some(Public)
--ParmDecl "a"  None
--ParmDecl "b"  None
--CompoundStmt ""  None
-Constructor "foo"  Some(Public)
--ParmDecl "n_"  None
--ParmDecl "c_"  None
--ParmDecl "d_"  None
--MemberRef "n"  None
--InitListExpr ""  None
---UnexposedExpr "n_"  None
----DeclRefExpr "n_"  None
--MemberRef "c"  None
--InitListExpr ""  None
---UnexposedExpr "c_"  None
----DeclRefExpr "c_"  None
--MemberRef "d"  None
--InitListExpr ""  None
---UnexposedExpr "d_"  None
----DeclRefExpr "d_"  None
--CompoundStmt ""  None
-FriendDecl ""  Some(Public)
--FunctionDecl "operator<"  Some(Public)
---ParmDecl "lh"  None
----TypeRef "class foo"  None
---ParmDecl "rh"  None
----TypeRef "class foo"  None
---CompoundStmt ""  None
----ReturnStmt ""  None
-----UnexposedExpr ""  None
------CallExpr "operator<"  None
-------UnexposedExpr ""  None
--------UnexposedExpr "tie"  None
---------CallExpr "tie"  None
----------UnexposedExpr "tie"  None
-----------DeclRefExpr "tie"  None
------------NamespaceRef "std"  None
----------MemberRefExpr "n"  None
-----------DeclRefExpr "lh"  None
----------MemberRefExpr "c"  None
-----------DeclRefExpr "lh"  None
----------MemberRefExpr "d"  None
-----------DeclRefExpr "lh"  None
-------UnexposedExpr "operator<"  None
--------DeclRefExpr "operator<"  None
-------UnexposedExpr ""  None
--------UnexposedExpr "tie"  None
---------CallExpr "tie"  None
----------UnexposedExpr "tie"  None
-----------DeclRefExpr "tie"  None
------------NamespaceRef "std"  None
----------MemberRefExpr "n"  None
-----------DeclRefExpr "rh"  None
----------MemberRefExpr "c"  None
-----------DeclRefExpr "rh"  None
----------MemberRefExpr "d"  None
-----------DeclRefExpr "rh"  None
-AccessSpecifier  Private
-FieldDecl "n"  Some(Private)
-FieldDecl "c"  Some(Private)
-FieldDecl "d"  Some(Private)
--------------------
```


## count-function-extents

This should result in the following output:

```text
$ target\Debug\count-function-extents.exe
Parsing  "examples/functions.cc" ...
theAnswerToLifeTheUniverseAndEverything:4
sum:3
square:3
cube:3
factorial:6
fibonacci:14
```

