use crate::Rouille;

pub struct NomDeLangue;

impl Rouille for NomDeLangue {
    fn tradurre(ident_str: &str) -> Option<&str> {
        Some(match ident_str {
            "" => "Err",
            "" => "Ok",
            "" => "String",
            "" => "HashMap",
            "" => "Default",
            "" => "Error",
            "" => "Option",
            "" => "Some",
            "" => "None",
            "" => "Result",
            "" => "Self",
            "" => "println",
            "" => "break",
            "" => "async",
            "" => "await",
            "" => "loop",
            "" => "move",
            "" => "crate",
            "" => "unreachable_code",
            "" => "as",
            "" => "const",
            "" => "trait",
            "" => "unsafe",
            "" => "in",
            "" => "from",
            "" => "dyn",
            "" => "unwrap",
            "" => "default",
            "" => "as_ref",
            "" => "io",
            "" => "extern",
            "" => "false",
            "" => "fn",
            "" => "super",
            "" => "insert",
            "" => "get",
            "" => "allow",
            "" | "" => "panic",
            "" => "mod",
            "" => "mut",
            "" => "new",
            "" => "where",
            "" => "for",
            "" => "get_or_insert_with",
            "" => "main",
            "" => "pub",
            "" => None?,
            "" => "return",
            "" => "impl",
            "" => "ref",
            "" => "match",
            "" => "if",
            "" => "else",
            "" => "self",
            "" => "let",
            "" => "static",
            "" => "struct",
            "" => "expect",
            "" => "while",
            "" => "use",
            "" => "into",
            "" => "true",
            "" => "enum",

            _ => ident_str,
        })
    }
}
