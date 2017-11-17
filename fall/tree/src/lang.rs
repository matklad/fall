use std::sync::Arc;
use {Text, TextBuf, TextRange, TextEdit, File, NodeType, NodeTypeInfo, IToken, INode, Metrics, tu};

pub trait LanguageImpl: 'static + Send + Sync {
    fn tokenize<'t>(&'t self, text: Text<'t>) -> Box<Iterator<Item=IToken> + 't>;
    fn parse(&self, text: Text, tokens: &[IToken], metrics: &Metrics) -> INode;
    fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo;
}

#[derive(Clone)]
pub struct Language {
    imp: Arc<LanguageImpl>
}

impl Language {
    pub fn new<I: LanguageImpl>(imp: I) -> Language {
        Language { imp: Arc::new(imp) }
    }

    pub fn parse<T: Into<TextBuf>>(&self, text: T) -> File {
        let text: TextBuf = text.into();
        let metrics = Metrics::new();
        let tokens: Vec<IToken> = metrics.measure_time("lexing", || {
            self.imp.tokenize(text.as_slice()).collect()
        });
        metrics.record("relexed region", text.len().utf8_len() as u64, "");
        let inode = self.imp.parse(text.as_slice(), &tokens, &metrics);
        File::new(self.clone(), text, metrics, tokens, inode)
    }

    pub fn reparse(&self, file: &File, edit: TextEdit) -> File {
        let new_text = edit.apply(file.text());

        if let Some(damage) = edit.damage() {
            let mut l = tu(0);
            let mut last_reusable = None;
            for (i, &t) in file.tokens().iter().enumerate() {
                l += t.len;
                if l + tu(1) >= damage {
                    break
                }
                if file.language().node_type_info(t.ty).whitespace_like {
                    last_reusable = Some((i, l))
                }
            }

            if let Some((i, l)) = last_reusable {
                let metrics = Metrics::new();

                let mut tokens: Vec<_> = file.tokens().iter().take(i + 1).cloned().collect();
                {
                    let text_tail = new_text.slice(TextRange::from_to(l, new_text.len()));
                    metrics.measure_time("lexing", || {
                        tokens.extend(self.imp.tokenize(text_tail))
                    });
                    metrics.record("relexed region", text_tail.len().utf8_len() as u64, "");
                }
                let inode = self.imp.parse(new_text.as_slice(), &tokens, &metrics);
                return File::new(self.clone(), new_text, metrics, tokens, inode)
            }
        }
        self.parse(new_text)
    }

    pub fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
        self.imp.node_type_info(ty)
    }
}
