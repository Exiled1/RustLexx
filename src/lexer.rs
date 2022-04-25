use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub(crate) enum SyntaxKind{ 
    // Dunno where the name SyntaxKind came from but it seems to be widely used to represent syntax tokens :shrug:
    #[regex(" +")]
    Whitespace,
    #[token("def")]
    DefKw,
    #[token("=")]
    Assign,
    #[regex("[0-9]*.[0-9]*")]
    Float,
    #[regex(r"\t+")]
    Tab,
    #[regex(r"\n")]
    Newline,
    #[regex(r"[a-zA-Z][a-zA-Z]")]
    Identifier,
    #[error]
    Error
}



