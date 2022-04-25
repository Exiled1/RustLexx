mod lexer;

#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::lexer::SyntaxKind;

    use super::*;

    #[test]
    fn lex_space() {
        check(" ", SyntaxKind::Whitespace);
    }

    #[test]
    fn lex_def() {
        check("def", SyntaxKind::DefKw);
    }

    #[test]
    fn lex_assign() {
        check("=", SyntaxKind::Assign);
    }

    fn check(input: &str, kind_token: SyntaxKind){
        let mut lexer = SyntaxKind::lexer(input);
        assert_eq!(lexer.next(), Some(kind_token));
        assert_eq!(lexer.slice(), input);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

}