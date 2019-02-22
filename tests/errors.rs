extern crate peg;

peg::parser!{ grammar parser() for str {
	pub rule one_letter -> () = ['a'..='z']

    pub rule parse -> usize
        = v:( "a" / "\n" )* { v.len() }

    pub rule error_pos = ("a" / "\n" / "\r")*
}}

#[test]
fn test_eof() {
	assert_eq!(parser::one_letter("t"), Ok(()));

    let err = parser::one_letter("tt").unwrap_err();
    assert_eq!(err.location.line, 1);
    assert_eq!(err.location.column, 2);
    assert_eq!(err.location.offset, 1);
    assert_eq!(format!("{}", err.expected), "EOF");
}

#[test]
fn test_errors() {
    let err = parser::parse(r#"
aaaa
aaaaaa
aaaabaaaa
"#).unwrap_err();

    assert_eq!(err.location.line, 4);
    assert_eq!(err.location.column, 5);
    assert_eq!(err.location.offset, 17);
    assert_eq!(format!("{}", err.expected), r#"one of "\n", "a", EOF"#);
}

#[test]
fn test_error_pos() {
    let err = parser::error_pos("aab\n").unwrap_err();
    assert_eq!(err.location.line, 1);
    assert_eq!(err.location.column, 3);
    assert_eq!(err.location.offset, 2);
    assert_eq!(err.expected.to_string(), r#"one of "\n", "\r", "a", EOF"#);

    let err = parser::error_pos("aa\naaaa\nbaaa\n").unwrap_err();
    assert_eq!(err.location.line, 3);
    assert_eq!(err.location.column, 1);

    let err = parser::error_pos("aa\naaaa\naaab\naa").unwrap_err();
    assert_eq!(err.location.line, 3);
    assert_eq!(err.location.column, 4);

    let err = parser::error_pos("aa\r\naaaa\r\naaab\r\naa").unwrap_err();
    assert_eq!(err.location.line, 3);
    assert_eq!(err.location.column, 4);
}