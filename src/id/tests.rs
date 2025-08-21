#[cfg(test)]
use super::*;

#[test]
fn can_be_created_with_new() {
    let seg_delimiter = ".";

    let candidate1 = "+0100.0001";
    let tool_id1 = Id::new("+", seg_delimiter, candidate1).unwrap();
    let id1 = format!("{}", tool_id1);
    assert_eq!(id1, candidate1);

    let candidate2 = "=01.01";
    let tool_id2 = Id::new("=", seg_delimiter, candidate2).unwrap();
    let id2 = format!("{}", tool_id2);
    assert_eq!(id2, candidate2);
}

#[test]
fn can_be_created_with_builder() {
    let candidate1 = "=0100.0001";
    let mut builder = IdBuilder::new("=", ".");
    let _ = builder.id(candidate1);
    let tool_id1 = builder.build();

    let id1 = format!("{}", tool_id1);
    assert_eq!(id1, candidate1);

    let candidate2 = "=01.01";
    let mut builder = IdBuilder::new("=", ".");
    let _ = builder.id(candidate2);
    let tool_id2 = builder.build();

    let id2 = format!("{}", tool_id2);
    assert_eq!(id2, candidate2);
}

#[test]
#[should_panic(expected = "empty id string")]
fn empty_cannot_be_created() {
    let seg_delimiter = ".";
    Id::new("+", seg_delimiter, "").unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic(expected = "code group deviates in length")]
fn wrong_group_length_cannot_be_created() {
    let seg_delimiter = ".";
    let candidate = "+0100.001";
    Id::new("+", seg_delimiter, candidate).unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic(expected = "code group deviates in length")]
fn wrong_delimiter_cannot_be_created() {
    let seg_delimiter = "*";
    let candidate = "+0100.0001*0010";
    Id::new("+", seg_delimiter, candidate).unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic(expected = "no code groups")]
fn lack_of_code_groups_cannot_be_created() {
    let seg_delimiter = ".";
    let candidate = "+";
    Id::new("+", seg_delimiter, candidate).unwrap_or_else(|error| panic!("{}", error));
}
