use std::fmt::Write;
use std::ascii::AsciiExt;

use ast::*;


impl Grammar {
    pub fn generate(&self) -> String {
        let mut result = String::new();
        let has_syn_rules = !self.syn_rules.is_empty();

        result.push_str("use std::sync::{Once, ONCE_INIT};\n");
        result.push_str("use fall_tree::{NodeType, NodeTypeInfo};\n");
        result.push_str("use fall_parse::Rule;\n");
        if has_syn_rules {
            result.push_str("use fall_parse::syn;\n");
        }

        result.push_str("pub use fall_tree::{ERROR, WHITESPACE};\n\n");
        for (i, t) in self.node_types.iter().enumerate() {
            writeln!(result, "pub const {:10}: NodeType = NodeType({});", scream(t), 100 + i)
                .unwrap();
        }

        result.push_str("\npub fn register_node_types() {\n");
        result.push_str("    static REGISTER: Once = ONCE_INIT;\n");
        result.push_str("    REGISTER.call_once(||{\n");
        for t in self.node_types.iter() {
            writeln!(result, "        {}.register(NodeTypeInfo {{ name: {:?} }});", scream(t), scream(t))
                .unwrap();
        }
        result.push_str("    });\n");
        result.push_str("}\n");

        result.push_str("\npub const TOKENIZER: &'static [Rule] = &[\n");
        for &LexRule { ref ty, ref re, ref f } in self.lex_rules.iter() {
            result.push_str("    ");
            let f = match *f {
                None => "None".to_owned(),
                Some(ref f) => format!("Some({})", f)
            };
            result.push_str(&format!("Rule {{ ty: {}, re: {}, f: {} }},", scream(ty), re, f));
            result.push_str("\n");
        }
        result.push_str("];\n");

        if has_syn_rules {
            self.generate_syn_rules(&mut result);
        }

        result
    }

    fn generate_syn_rules(&self, buff: &mut String) {
        buff.push_str("\npub const PARSER: &'static [syn::Rule] = &[\n");
        for rule in self.syn_rules.iter() {
            buff.push_str("    ");
            let ty = if self.node_types.contains(&rule.name) {
                format!("Some({})", scream(&rule.name))
            } else {
                "None".to_owned()
            };
            let alts = rule.alts.iter().map(|a| self.generate_alt(a)).collect::<Vec<_>>();
            buff.push_str(&format!(r#"syn::Rule {{ name: "{}", ty: {}, alts: &[{}] }},"#,
                                   rule.name, ty, alts.join(", ")));
            buff.push_str("\n");
        }
        buff.push_str("];\n");
    }

    fn generate_alt(&self, alt: &Alt) -> String {
        let parts = alt.parts.iter().map(|p| self.generate_part(p)).collect::<Vec<_>>();
        format!("syn::Alt {{ parts: &[{}], commit: {:?} }}", parts.join(", "), alt.commit)
    }

    fn generate_part(&self, part: &Part) -> String {
        match *part {
            Part::Rule(ref name) => {
                if self.lex_rules.iter().any(|l| &l.ty == name) {
                    format!("syn::Part::Token({})", scream(name))
                } else {
                    format!("syn::Part::Rule({:?})", name)
                }
            }
            Part::Rep(ref a) => format!("syn::Part::Rep({})", self.generate_alt(a)),
        }
    }
}


fn scream(word: &str) -> String {
    word.chars().map(|c| c.to_ascii_uppercase()).collect()
}


