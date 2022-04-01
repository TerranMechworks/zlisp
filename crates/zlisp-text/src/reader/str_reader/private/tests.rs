use super::*;
use crate::error::ErrorCode;
use assert_matches::assert_matches;

#[test]
fn peek_does_not_advance_if_peeked_again() {
    let mut reader = StrReader::new("()");
    assert_matches!(reader.peek().unwrap().token, Token::ListStart);
    assert_matches!(reader.peek().unwrap().token, Token::ListStart);
    reader.read_list_start().unwrap();
    assert_matches!(reader.peek().unwrap().token, Token::ListEnd);
    assert_matches!(reader.peek().unwrap().token, Token::ListEnd);
    reader.read_list_end().unwrap();
    assert_matches!(reader.peek().unwrap().token, Token::Eof);
    assert_matches!(reader.peek().unwrap().token, Token::Eof);
    reader.finish().unwrap();
}

#[test]
fn peek_does_not_modify_location() {
    let mut reader = StrReader::new("()");

    {
        let before = reader.location();
        let span = reader.peek().unwrap();
        assert_matches!(span.token, Token::ListStart);
        assert_eq!(span.loc, before);
        assert_eq!(reader.location(), before);

        reader.read_list_start().unwrap();
        assert_ne!(reader.location(), before);
    }

    {
        let before = reader.location();
        let span = reader.peek().unwrap();
        assert_matches!(span.token, Token::ListEnd);
        assert_eq!(span.loc, before);
        assert_eq!(reader.location(), before);

        reader.read_list_end().unwrap();
        assert_ne!(reader.location(), before);
    }

    {
        let before = reader.location();
        let span = reader.peek().unwrap();
        assert_matches!(span.token, Token::Eof);
        assert_eq!(span.loc, before);
        assert_eq!(reader.location(), before);

        reader.finish().unwrap();
    }
}

#[test]
fn peek_does_not_modify_finish() {
    let mut reader = StrReader::new("()");

    {
        let before = reader.clone().finish().unwrap_err();
        let span = reader.peek().unwrap();
        assert_matches!(span.token, Token::ListStart);
        let after = reader.clone().finish().unwrap_err();

        assert_matches!(
            after.code(),
            ErrorCode::ExpectedToken {
                expected: TokenType::Eof,
                found: TokenType::ListStart
            }
        );
        assert_matches!(
            before.code(),
            ErrorCode::ExpectedToken {
                expected: TokenType::Eof,
                found: TokenType::ListStart
            }
        );
        assert_eq!(after.location(), before.location());

        reader.read_list_start().unwrap();
    }

    {
        let before = reader.clone().finish().unwrap_err();
        let span = reader.peek().unwrap();
        assert_matches!(span.token, Token::ListEnd);
        let after = reader.clone().finish().unwrap_err();

        assert_matches!(
            after.code(),
            ErrorCode::ExpectedToken {
                expected: TokenType::Eof,
                found: TokenType::ListEnd
            }
        );
        assert_matches!(
            before.code(),
            ErrorCode::ExpectedToken {
                expected: TokenType::Eof,
                found: TokenType::ListEnd
            }
        );
        assert_eq!(after.location(), before.location());

        reader.read_list_end().unwrap();
    }

    {
        reader.clone().finish().unwrap();
        let span = reader.peek().unwrap();
        assert_matches!(span.token, Token::Eof);
        reader.finish().unwrap();
    }
}
