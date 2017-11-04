use fall_tree::{Node, TextUnit, TextRange};
use fall_tree::visitor::{Visitor, NodeVisitor};
use ::*;

mod refdec;

use self::refdec::{Reference, Declaration};

pub fn resolve_reference(analysis: &Analysis, offset: TextUnit) -> Option<TextRange> {
    return refdec::resolve_reference(
        analysis,
        offset,
        &|node| ref_provider(analysis, node)
    );
}

pub fn find_usages(analysis: &Analysis, offset: TextUnit) -> Vec<TextRange> {
    return refdec::find_usages(
        analysis,
        offset,
        &|node| ref_provider(analysis, node),
        def_provider
    );
}

fn ref_provider<'f>(analysis: &Analysis<'f>, node: Node<'f>) -> Option<Reference<'f>> {
    Visitor(None)
        .visit::<RefExpr, _>(|result, ref_expr| {
            *result = Some(Reference::new(ref_expr.node(), |node| {
                let ref_expr = RefExpr::new(node);
                let target = match ref_expr.resolve() {
                    None => return None,
                    Some(t) => t
                };

                Some(match target {
                    RefKind::RuleReference(rule) => rule.into(),
                    RefKind::Param(param) => param.into(),
                    RefKind::Token(token) => token.into(),
                })
            }))
        })
        .visit::<AstSelector, _>(|result, selector| {
            *result = Some(Reference::new(selector.node(), |node| {
                let selector = AstSelector::new(node);
                let target = match selector.child_kind() {
                    None => return None,
                    Some(t) => t
                };
                Some(match target {
                    ChildKind::AstNode(node) => node.into(),
                    ChildKind::AstClass(cls) => cls.into(),
                    ChildKind::Token(token) => token.into()
                })
            }))
        })
        .visit_nodes(&[IDENT], |result, ident| {
            match ident.parent() {
                Some(parent) if parent.ty() == CALL_EXPR => {
                    let call = CallExpr::new(parent);
                    if let Ok(CallKind::RuleCall(..)) = call.kind() {
                        *result = Some(Reference::new(ident, |node| {
                            let call = CallExpr::new(node.parent().unwrap());
                            match call.kind().unwrap() {
                                CallKind::RuleCall(rule, ..) => Some(rule.into()),
                                _ => unimplemented!()
                            }
                        }))
                    }
                }
                _ => {}
            }
        })
        .walk_single_node(node)
}

fn def_provider<'f>(node: Node<'f>) -> Option<Declaration<'f>> {
    Visitor(None)
        .visit::<SynRule, _>(|result, node| *result = Some(node.into()))
        .visit::<LexRule, _>(|result, node| *result = Some(node.into()))
        .visit::<Parameter, _>(|result, node| *result = Some(node.into()))
        .visit::<AstNodeDef, _>(|result, node| *result = Some(node.into()))
        .visit::<AstClassDef, _>(|result, node| *result = Some(node.into()))
        .walk_single_node(node)
}

impl<'f> From<SynRule<'f>> for Declaration<'f> {
    fn from(rule: SynRule<'f>) -> Self {
        Declaration::with_name_ident(rule.node(), rule.name_ident())
    }
}

impl<'f> From<LexRule<'f>> for Declaration<'f> {
    fn from(rule: LexRule<'f>) -> Self {
        Declaration::new(rule.node())
    }
}

impl<'f> From<Parameter<'f>> for Declaration<'f> {
    fn from(rule: Parameter<'f>) -> Self {
        Declaration::new(rule.node())
    }
}

impl<'f> From<AstNodeDef<'f>> for Declaration<'f> {
    fn from(rule: AstNodeDef<'f>) -> Self {
        Declaration::with_name_ident(rule.node(), Some(rule.name_ident()))
    }
}

impl<'f> From<AstClassDef<'f>> for Declaration<'f> {
    fn from(rule: AstClassDef<'f>) -> Self {
        Declaration::with_name_ident(rule.node(), Some(rule.name_ident()))
    }
}

#[test]
fn test_find_refs() {
    use fall_tree::tu;

    let file = ::editor_api::analyse(r#####"
tokenizer {
  #[skip] whitespace r"\s+"

  number r"\d+"
  plus '+'
  minus '-'
  star '*'
  slash '/'
  bang '!'
  lparen '('
  rparen ')'
}

pub rule file { expr }

#[pratt]
rule expr {
  sum_expr | product_expr
  | factorial_expr
  | negate_expr
  | constant_expr | paren_expr
}

#[bin(2)]
pub rule product_expr { expr {'*' | '/'} expr }

#[bin(1)]
pub rule sum_expr { expr {'+' | '-'} expr }

#[atom]
pub rule constant_expr { number }

#[atom]
pub rule paren_expr { '(' expr ')' }

#[postfix]
pub rule factorial_expr { expr '!' }

#[prefix]
pub rule negate_expr { '-' expr }

test r"
  1 + --1! - -2!
"
"#####.to_string());
    file.analyse(|analysis| {
        let usages = find_usages(analysis, tu(309));

        assert_eq!(usages, vec![TextRange::from_len(tu(202), tu(12))]);
    })
}
