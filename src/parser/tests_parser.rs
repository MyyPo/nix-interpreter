#[cfg(test)]
mod tests {
    use crate::lexer::tokens::*;
    use crate::lexer::*;
    use crate::parser::ast::*;
    use crate::parser::*;

    use std::collections::BTreeMap;

    #[test]
    fn parse_valid_binary_statements() {
        let test_cases: Vec<(&TokenStream, Expr)> = vec![
            (
                &[
                    TokenType::Int(4),
                    TokenType::AdditiveOperator(AdditiveOperator::Add),
                    TokenType::Flo(3.45),
                ],
                Expr::new_add(Expr::new_int(4), Expr::new_flo(3.45)),
            ),
            (
                &[
                    TokenType::Int(4),
                    TokenType::AdditiveOperator(AdditiveOperator::Sub),
                    TokenType::Flo(3.45),
                    TokenType::AdditiveOperator(AdditiveOperator::Sub),
                    TokenType::Int(1),
                ],
                Expr::new_sub(
                    Expr::new_sub(Expr::new_int(4), Expr::new_flo(3.45)),
                    Expr::new_int(1),
                ),
            ),
            (
                &[
                    TokenType::Int(4),
                    TokenType::AdditiveOperator(AdditiveOperator::Sub),
                    TokenType::Flo(3.45),
                    TokenType::AdditiveOperator(AdditiveOperator::Sub),
                    TokenType::Int(1),
                    TokenType::LogicalComparison(LogicalComparison::CompareEquals),
                    TokenType::Int(5),
                ],
                Expr::new_compare_equals(
                    Expr::new_sub(
                        Expr::new_sub(Expr::new_int(4), Expr::new_flo(3.45)),
                        Expr::new_int(1),
                    ),
                    Expr::new_int(5),
                ),
            ),
        ];

        for (input, want) in test_cases {
            let mut parser = AstParser::new(input);

            let got = parser.parse();
            assert_eq!(got, want);
        }
    }

    #[test]
    fn parse_valid_let_statements() {
        let mut test1 = BTreeMap::new();
        test1.insert(IdentExpr::new("x"), Expr::new_str("foo"));
        test1.insert(IdentExpr::new("y"), Expr::new_str("bar"));

        let mut test2 = BTreeMap::new();
        test2.insert(IdentExpr::new("newFlo"), Expr::new_flo(0.5));
        let mut test2_set = BTreeMap::new();
        test2_set.insert(
            "hey",
            Expr::new_add(Expr::new_ident("newFlo"), Expr::new_flo(1.5)),
        );
        test2_set.insert(
            "myList",
            Expr::new_list(vec![Expr::new_str("elem1"), Expr::new_str("elem2")]),
        );

        let test_cases: Vec<(&TokenStream, Expr)> = vec![
            (
                &[
                    TokenType::Let,
                    TokenType::Ident("x"),
                    TokenType::Assign,
                    TokenType::StrLiteral("foo"),
                    TokenType::Semicolon,
                    TokenType::Ident("y"),
                    TokenType::Assign,
                    TokenType::StrLiteral("bar"),
                    TokenType::Semicolon,
                    TokenType::In,
                    TokenType::Ident("x"),
                    TokenType::AdditiveOperator(AdditiveOperator::Add),
                    TokenType::Ident("y"),
                ],
                Expr::new_let(
                    test1,
                    Expr::new_add(Expr::new_ident("x"), Expr::new_ident("y")),
                ),
            ),
            (
                &[
                    TokenType::Let,
                    TokenType::Ident("newFlo"),
                    TokenType::Assign,
                    TokenType::Flo(0.5),
                    TokenType::Semicolon,
                    TokenType::In,
                    TokenType::OpenBrace,
                    TokenType::Ident("hey"),
                    TokenType::Assign,
                    TokenType::Ident("newFlo"),
                    TokenType::AdditiveOperator(AdditiveOperator::Add),
                    TokenType::Flo(1.5),
                    TokenType::Semicolon,
                    TokenType::Ident("myList"),
                    TokenType::Assign,
                    TokenType::OpenSquare,
                    TokenType::StrLiteral("elem1"),
                    TokenType::StrLiteral("elem2"),
                    TokenType::CloseSquare,
                    TokenType::Semicolon,
                    TokenType::CloseBrace,
                ],
                Expr::new_let(test2, Expr::new_set(test2_set)),
            ),
        ];

        for (input, want) in test_cases {
            let mut parser = AstParser::new(input);

            let got = parser.parse();
            assert_eq!(got, want);
        }
    }

    #[test]
    fn parse_valid_with_statements() {
        let test_cases: Vec<(&TokenStream, Expr)> = vec![(
            &[
                TokenType::With,
                TokenType::Ident("pkgs"),
                TokenType::Semicolon,
                TokenType::OpenSquare,
                TokenType::Ident("git"),
                TokenType::CloseSquare,
            ],
            Expr::new_with(
                Expr::new_ident("pkgs"),
                Expr::new_list(vec![Expr::new_ident("git")]),
            ),
        )];

        for (input, want) in test_cases {
            let mut parser = AstParser::new(input);

            let got = parser.parse();
            assert_eq!(got, want);
        }
    }

    #[test]
    fn parse_valid_if_statements() {
        let test_cases: Vec<(&TokenStream, Expr)> = vec![(
            &[
                TokenType::If,
                TokenType::Int(1),
                TokenType::AdditiveOperator(AdditiveOperator::Add),
                TokenType::Int(1),
                TokenType::LogicalComparison(LogicalComparison::CompareEquals),
                TokenType::Int(2),
                TokenType::Then,
                TokenType::StrLiteral("yes!"),
                TokenType::Else,
                TokenType::StrLiteral("no..."),
            ],
            Expr::new_if(
                Expr::new_compare_equals(
                    Expr::new_add(Expr::new_int(1), Expr::new_int(1)),
                    Expr::new_int(2),
                ),
                Expr::new_str("yes!"),
                Expr::new_str("no..."),
            ),
        )];

        for (input, want) in test_cases {
            let mut parser = AstParser::new(input);

            let got = parser.parse();
            assert_eq!(got, want);
        }
    }

    #[test]
    fn parse_valid_sets_statements() {
        let test_cases: Vec<(&TokenStream, Expr)> = vec![];

        for (input, want) in test_cases {
            let mut parser = AstParser::new(input);

            let got = parser.parse();
            assert_eq!(got, want);
        }
    }
}
