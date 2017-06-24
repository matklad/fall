# Fall: Not Yet Another Parser Generator

This is a work in progress hobby project. If you are looking for a production ready parser generator for Rust,
consider [pest](https://github.com/pest-parser/pest), [lalrpop](https://github.com/nikomatsakis/lalrpop) or
[nom](https://github.com/Geal/nom). If you are looking for a production grade IDE-ready parser generator, take a look
at [Grammar Kit](https://github.com/JetBrains/Grammar-Kit) or [Papa Carlo](https://github.com/Eliah-Lakhin/papa-carlo).

## Scope

The ambitious goal is to create a parsing framework, suitable for tools interacting with the source code, such as
editors, IDEs, refactoring tools or code formatters.

## Design constraints

The syntax tree must not be abstract, it should include all the whitespace and comments and be a lossless representation
of the original text.

All the languages should share the same syntax tree data structure. That is, it should be possible to write non-generic
code for processing syntax of any language. It should also be possible to provide a single C API to interact with a
syntax tree of any language.

Parser should be able to deal with incomplete input gracefully. It should be able to recognize syntactic constructs
even if some required elements are missing and it should attempt to resynchronize input after an error.

## Non goals

Parser need not guarantee that the input grammar is unambiguous.

Parser need not guarantee sane worse case performance for any grammar. Nevertheless, it is expected that most sane
programming languages could be parsed efficiently.

## Nice to haves

Implementing parsers should be interactive: user should see the grammar, the example input and the parse tree
simultaneously.

Parsing should be incremental: changing something inside the code block should cause only the block to be reparsed.

Parsing should be fast: even with incrementally, there are certain bad cases (unclosed quote), where one has to reparse
the whole input.

## Code structure


### Tree Model

The entry point is `fall_tree/node/mod.rs`. It defines the structure of the syntax tree which roughly looks like this:

```rust
type NodeType = 32;

struct File { ... }

#[derive(Clone, Copy)]
struct Node<'f> {
    file: &'f File,
    ...
}

impl<'f> Node<'f'> {
    fn ty(&self) -> NodeType { ... }
    fn parent(&self) -> Node<'f> { ... }
    fn children(&self) -> impl Iterator<Item=Node<'f'>> { ... }
    fn text_range(&self) -> (usize, usize) { ... }
    fn text(&self) -> &str { ... }
}
```

The main element is a non-generic `Node` which is a `Copy` handle representing some range in the input text, together
with its type (which is just an integer constant) and subranges. It is the main API that the consumers of the syntax
tree would use.

While having an untyped API is needed for working with several different languages together, for each particular
language a typed API is easier to work with. You can layer a typed API on top of Nodes easily, using the following
pattern

```rust

struct RustFunction {
    node: Node
}

impl RustFunction {
    fn new(node: Node) -> RustFunction {
        assert_eq!(node.ty(), RUST_FUNCTION);
        RustFunction { node: node }
    }

    fn name(&self) -> &str {
        let ident_child = child_of_type_exn(self.node, IDENT);
        ident_child.text()
    }
}
```

Such typed wrappers are generated automatically. See `fall_tree/ast.rs` and `fall_tree/visitor.rs` for a generic
implementation of this pattern and how it can be used to travers trees in a type-safe manner (imo, this is the most
beautiful piece of code here so far:) ). It's also interesting that you can create a single typed wrapper around
*several* node types, which allows to express an arbitrary [non-]hierarchy of node types. See `AstClass` for details.


### Parsing

By itself, `fall_tree` does not impose any particular way of constructing trees. It should be possible to connect it to
a hand written, a generated or an external parser. Currently a specific parser generator is the main way to create
trees. `fall_parse` contains runtime for the parser (currently, parser is mostly interpreted), and `fall_gen`
contains the corresponding generator, which generates a lexer, a parser and the AST. The parser is roughly a
"hand-written recursive descent" plus (to be implemented) Pratt parser for expressions. Some call this style
of parsing PEG.

### Grammar

To learn the details of the grammar spec, it is best to look at the examples (sorry, no docs yet because
everything is in flux). The examples are in the `lang` subdirectory, look for the `syntax.fall` files.  The most
complete is `lang/fall`, which is the language of the parser generator itself: fall was bootstraped fully, starting from
`split_lines`, `split_whitespace` style parser.

Here are some interesting bits of the grammar.

The `<commit>` specifier allows parser to recognize incomplete syntactic constructs. For example, for the

```
rule function {
  'fn' ident <commit> '(' fn_args ')' '->' type expr
}
```

the parser would recognize `fn foo` as an incomplete function, and would give the following tree:

```
FUNCTION
  FN
  IDENT "foo"
  ERROR '(' expected
```

The `<with_skip to_skip rule>` function allows to skip some tokens to resynchronize input. For example,
`<with_skip 'fn' function>` would skip the tokens (creating an error node) until the `fn` keyword, and then launch
`function` parser.

The `<layer cover contents>` rule allows to "approximately" parse a fragment of input, which helps with error recovery
and incremental and lazy reparsing. Let's look at the concrete example:

```
pub rule block_expr {
  '{' <layer block_body {<opt seq_expr> <rep {'|' seq_expr}>}> '}'
}

rule block_body { <rep balanced> }
rule balanced {
  '{' <commit> block_body '}'
| <not '}'>
}
```

Here, `block_body` parses an arbitrary sequence of tokens with the sole restriction that `{` and `}` are balanced. When
parsing the innards of `block_expr`, the parser would first find the borders of the `bock_body`, and than it would parse
the contents of the `block_body` with the more detailed `{<opt seq_expr> <rep {'|' seq_expr}>}`. Crucially, if the
detailed rule fails, than all the remaining tokens inside the block body will be marked as an errors, but the parsing
outside of the blocks will continue as usual. Moreover, if the user types anything inside the block, the parser will
check if the block's borders do not change (this would be the case unless `{` or `}` is typed) and if it is the case,
it will only reparse the block itself.

The `example` blocks allow to quickly get feedback about the current grammar. You can write something like

```
pub rule struct_def {
  <opt 'pub'> 'struct' <commit> ident
  '{' <layer block_body <rep struct_field>>'}'
}

example r"
  struct Foo {
    a: A,
    pub b: B,
  }
"
```

and then run `cargo run --bin gen --example rust.fall` to render the syntax tree of the example block. `watch.sh`
wraps this into convenient "rerender example on save" script.

### VS Code plugin

There is a VS Code plugin in the `code` director, which demonstrates how `fall` can be used from an editor. The plugin
currently supports only the `fall` language itself. All features are implemented in Rust in an editor agnostic way in
`lang/fall/src/editor_api.rs`. It should be possible to hook up this code with any editor, by either dynamically or
statically linking in the Rust crate, or by wrapping it into an RPC.

## Current status

Something works :)

Here's a screenshoot showing fall parsing its own syntax with error recovery:

![Fall file with syntax tree](https://user-images.githubusercontent.com/1711539/27507725-a248b5f4-58dd-11e7-8e34-db6331a145c3.png)
