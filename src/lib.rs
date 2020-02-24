pub struct VariantDef<Sym> {
    variants: Vec<Sym>,
}

impl<Sym> VariantDef<Sym> {
    pub fn new(variants: Vec<Sym>) -> Self {
        Self { variants }
    }
}

pub enum Pattern<'def, Sym> {
    Variable(Sym),
    Variant(Sym, &'def VariantDef<Sym>),
    Tuple(Vec<Pattern<'def, Sym>>),
}

pub struct Case<'def, Sym, Res> {
    pattern: Pattern<'def, Sym>,
    result: Res,
}

pub struct CaseAnalysis<'def, Sym, Res>(Vec<Case<'def, Sym, Res>>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let boolean = VariantDef {
            variants: vec!["True", "False"],
        };

        let fruit = VariantDef {
            variants: vec!["Citrus", "Berry", "Other"],
        };

        let _cases = CaseAnalysis(vec![
            Case {
                pattern: Pattern::Tuple(vec![
                    Pattern::Variant("True", &boolean),
                    Pattern::Variant("Berry", &fruit),
                ]),
                result: 1,
            },
            Case {
                pattern: Pattern::Tuple(vec![
                    Pattern::Variant("True", &boolean),
                    Pattern::Variable("x"),
                ]),
                result: 2,
            },
            Case {
                pattern: Pattern::Variable("_"),
                result: 3,
            },
        ]);
    }
}
