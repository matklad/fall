use fall_tree::File;
use fall_parse::syn::Parser;
use fall_parse;

pub mod grammar;


pub fn parse(text: String) -> File {
    grammar::register_node_types();
    fall_parse::parse(text, grammar::FILE, grammar::TOKENIZER, &|b| {
        let p = Parser::new(grammar::PARSER);
        p.parse(b);
    })
}