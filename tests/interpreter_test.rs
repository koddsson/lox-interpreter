use loxc::{execute, Error};

#[test]
fn empty_source() {
    let results = execute("").unwrap();
    assert!(results.is_empty());
}

#[test]
fn handles_number_literals() {
    let results = execute("2;").unwrap();
    assert_eq!("2", results[0]);
}

#[test]
fn handles_string_literals() {
    let results = execute("\"hello world!\";").unwrap();
    assert_eq!("\"hello world!\"", results[0]);
}

#[test]
fn handles_nil_literal() {
    let results = execute("nil;").unwrap();
    assert_eq!("nil", results[0]);
}

#[test]
fn handles_true_literal() {
    let results = execute("true;").unwrap();
    assert_eq!("true", results[0]);
}

#[test]
fn handles_false_literal() {
    let results = execute("false;").unwrap();
    assert_eq!("false", results[0]);
}

#[test]
fn errors_when_missing_semicolon() {
    assert!(execute("2").is_err());
}

#[test]
fn handles_unary_expressions() {
    let results = execute("-1;").unwrap();
    assert_eq!("-1", results[0]);
}

#[test]
fn handles_binary_expressions() {
    let results = execute("1+1;").unwrap();
    assert_eq!("2", results[0]);
}

#[test]
fn handles_concattenating_strings() {
    let results = execute("\"foo\"+\"bar\";").unwrap();
    assert_eq!("\"foobar\"", results[0]);
}

#[test]
fn handles_or_operations() {
    let results = execute("false or  \"bar\";").unwrap();
    assert_eq!("\"bar\"", results[0]);
}

#[test]
fn handles_and_operations() {
    let results = execute("true and  \"bar\";").unwrap();
    assert_eq!("\"bar\"", results[0]);
}

#[test]
fn handles_dividing_numbers() {
    let results = execute("10/4;").unwrap();
    assert_eq!("2.5", results[0]);
}

#[test]
fn errors_when_dividing_by_zero() {
    assert!(execute("10/0;").is_err());
}

#[test]
fn errors_when_trying_to_add_number_to_a_string() {
    assert_eq!(
        Some(Error::InterpreterError(
            "Runtime error in binary expression!".to_string()
        )),
        execute("\"hello\" + 10;").err()
    );
}
