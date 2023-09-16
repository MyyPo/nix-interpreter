#[cfg(test)]
mod tests {
    use crate::lexer::chars::*;
    use crate::lexer::tokens::*;
    use crate::lexer::*;

    #[test]
    fn token_try_from_char() {
        let test_cases: Vec<(char, CharType)> = vec![
            ('=', CharType::Equals),
            ('+', CharType::Plus),
            ('-', CharType::Minus),
            ('(', CharType::OpenParen),
            (')', CharType::CloseParen),
            ('{', CharType::OpenBrace),
            ('}', CharType::CloseBrace),
            (',', CharType::Comma),
            (';', CharType::Semicolon),
            ('[', CharType::OpenSquare),
            (']', CharType::CloseSquare),
            (',', CharType::Comma),
            ('A', CharType::Char),
            ('h', CharType::Char),
            (' ', CharType::Whitespace),
            ('@', CharType::At),
        ];

        for (input, want) in test_cases {
            let got = CharType::try_from(input).unwrap();
            assert_eq!(got, want)
        }
    }

    #[test]
    fn invalid_token_try_from_char() {
        let test_cases: Vec<(char, InvalidTokenError)> = vec![('н', InvalidTokenError::new('н'))];

        for (input, want_error) in test_cases.iter() {
            let got = CharType::try_from(*input);
            match got {
                Ok(_) => {
                    panic!("Expected error but got Ok instead")
                }
                Err(got_error) => assert_eq!(got_error, *want_error),
            }
        }
    }

    #[test]
    fn tokenize_correct_str() {
        let test_cases: Vec<(&str, Vec<TokenType>)> = vec![
            (
                "# comment = 10;
                mine =-15
                 #; other comment",
                vec![
                    TokenType::Ident("mine"),
                    TokenType::Assign,
                    TokenType::ArithmNegation,
                    TokenType::Int(15),
                ],
            ),
            (
                "var-with-hyphens = 1",
                vec![
                    TokenType::Ident("var-with-hyphens"),
                    TokenType::Assign,
                    TokenType::Int(1),
                ],
            ),
            (
                "xyz = 10;",
                vec![
                    TokenType::Ident("xyz"),
                    TokenType::Assign,
                    TokenType::Int(10),
                    TokenType::Semicolon,
                ],
            ),
            (
                "hello=155+34.8;",
                vec![
                    TokenType::Ident("hello"),
                    TokenType::Assign,
                    TokenType::Int(155),
                    TokenType::AdditiveOperator(AdditiveOperator::Add),
                    TokenType::Flo(34.8),
                    TokenType::Semicolon,
                ],
            ),
            (
                "\"string that exists\"",
                vec![TokenType::StrLiteral("string that exists")],
            ),
            ("\"\"", vec![TokenType::StrLiteral("")]),
            (
                "value = \"something\";",
                vec![
                    TokenType::Ident("value"),
                    TokenType::Assign,
                    TokenType::StrLiteral("something"),
                    TokenType::Semicolon,
                ],
            ),
            (
                "''
                    I can't not use ', so sad
                '';",
                vec![
                    TokenType::StrLiteral(
                        "
                    I can't not use ', so sad
                ",
                    ),
                    TokenType::Semicolon,
                ],
            ),
            (
                "bashScript = ''
                    #!/usr/bin/env bash
                    # This is a simple Bash script that greets the user

                    # Prompt the user for their name
                    echo \"What's your name?\"
                    read name

                    # Print a greeting message
                    echo \"Hello, $name! Welcome to the Bash scripting world.\"

                    # End of the script
                '';",
                vec![
                    TokenType::Ident("bashScript"),
                    TokenType::Assign,
                    TokenType::StrLiteral(
                        "
                    #!/usr/bin/env bash
                    # This is a simple Bash script that greets the user

                    # Prompt the user for their name
                    echo \"What's your name?\"
                    read name

                    # Print a greeting message
                    echo \"Hello, $name! Welcome to the Bash scripting world.\"

                    # End of the script
                ",
                    ),
                    TokenType::Semicolon,
                ],
            ),
        ];

        for (input, want) in test_cases {
            let mut lexer = Lexer::new(input);
            let got = lexer.tokenize();
            assert_eq!(*got, want);
        }
    }

    #[test]
    fn tokenize_path() {
        let test_cases: Vec<(&str, Vec<TokenType>)> = vec![
            ("./Cargo.toml", vec![TokenType::Path("./Cargo.toml")]),
            (
                "myPath=./Cargo.toml",
                vec![
                    TokenType::Ident("myPath"),
                    TokenType::Assign,
                    TokenType::Path("./Cargo.toml"),
                ],
            ),
            (
                "homePath=~/Music",
                vec![
                    TokenType::Ident("homePath"),
                    TokenType::Assign,
                    TokenType::Path("~/Music"),
                ],
            ),
            (
                "absolutePath=/etc/nixos",
                vec![
                    TokenType::Ident("absolutePath"),
                    TokenType::Assign,
                    TokenType::Path("/etc/nixos"),
                ],
            ),
            (
                "nixPath=<nixpkgs>",
                vec![
                    TokenType::Ident("nixPath"),
                    TokenType::Assign,
                    TokenType::NixPath("<nixpkgs>"),
                ],
            ),
        ];

        for (input, want) in test_cases {
            let mut lexer = Lexer::new(input);
            let got = lexer.tokenize();
            assert_eq!(*got, want);
        }
    }

    #[test]
    fn tokenize_operators() {
        let test_cases: Vec<(&str, Vec<TokenType>)> = vec![
            (
                "5 <1.1",
                vec![
                    TokenType::Int(5),
                    TokenType::ArithmComparison(ArithmComparison::Less),
                    TokenType::Flo(1.1),
                ],
            ),
            (
                "5.1>1.1",
                vec![
                    TokenType::Flo(5.1),
                    TokenType::ArithmComparison(ArithmComparison::More),
                    TokenType::Flo(1.1),
                ],
            ),
            (
                "5.1<=1.1",
                vec![
                    TokenType::Flo(5.1),
                    TokenType::ArithmComparison(ArithmComparison::LessOrEquals),
                    TokenType::Flo(1.1),
                ],
            ),
            (
                "5.1>=1.1",
                vec![
                    TokenType::Flo(5.1),
                    TokenType::ArithmComparison(ArithmComparison::MoreOrEquals),
                    TokenType::Flo(1.1),
                ],
            ),
            (
                "war!=peace",
                vec![
                    TokenType::Ident("war"),
                    TokenType::LogicalComparison(LogicalComparison::CompareNotEquals),
                    TokenType::Ident("peace"),
                ],
            ),
            (
                "\"foo\"==\"foo\"",
                vec![
                    TokenType::StrLiteral("foo"),
                    TokenType::LogicalComparison(LogicalComparison::CompareEquals),
                    TokenType::StrLiteral("foo"),
                ],
            ),
            (
                "itWont++workInRealCode",
                vec![
                    TokenType::Ident("itWont"),
                    TokenType::Concat,
                    TokenType::Ident("workInRealCode"),
                ],
            ),
            (
                "15.14 / 18",
                vec![
                    TokenType::Flo(15.14),
                    TokenType::MultiplicativeOperator(MultiplicativeOperator::Div),
                    TokenType::Int(18),
                ],
            ),
            (
                "3 *5.15",
                vec![
                    TokenType::Int(3),
                    TokenType::MultiplicativeOperator(MultiplicativeOperator::Mult),
                    TokenType::Flo(5.15),
                ],
            ),
        ];

        for (input, want) in test_cases {
            let mut lexer = Lexer::new(input);
            let got = lexer.tokenize();
            assert_eq!(*got, want);
        }
    }

    #[test]
    #[should_panic = "Unexpected EOF, expecting a second, closing single quote"]
    fn try_tokenize_no_closing_squote() {
        let test_cases: Vec<&str> = vec![
            "bashScript = ''
                    #!/usr/bin/env bash
                    # This is a simple Bash script that greets the user

                    # Prompt the user for their name
                    echo \"What's your name?\"
                    read name

                    # Print a greeting message
                    echo \"Hello, $name! Welcome to the Bash scripting world.\"

                    # End of the script
                ;",
        ];

        for input in test_cases {
            let mut lexer = Lexer::new(input);
            let _ = lexer.tokenize();
        }
    }

    #[test]
    #[should_panic = "Unexpected EOF, expecting a second, closing double quote"]
    fn try_tokenize_no_closing_dquote() {
        let test_cases: Vec<&str> = vec!["\"Oops forgot to close this one!", "weirdness = \""];

        for input in test_cases {
            let mut lexer = Lexer::new(input);
            let _ = lexer.tokenize();
        }
    }
}
