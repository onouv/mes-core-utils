#[cfg(test)]
use crate::tool_id::*;

#[test]
fn can_be_created() {
    let seg_delimiter = '.';
    let prefix = '+';
    let group_len = 4;
    let candidate = "+0100.0001";
    let tool_id = ToolId::new(prefix, seg_delimiter, group_len, candidate)
        .unwrap_or_else(|error| panic!("{}", error));

    println!("{}", tool_id);

    let id = format!("{}", tool_id);

    assert_eq!(id, candidate);
}

#[test]
#[should_panic(expected = "code group deviates in length")]
fn wrong_group_length_cannot_be_created() {
    let seg_delimiter = '.';
    let prefix = '+';
    let group_len = 4;
    let candidate = "+0100.001";
    ToolId::new(prefix, seg_delimiter, group_len, candidate)
        .unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic(expected = "code group deviates in length")]
fn wrong_delimiter_cannot_be_created() {
    let seg_delimiter = '*';
    let prefix = '+';
    let group_len = 4;
    let candidate = "+0100.0001";
    ToolId::new(prefix, seg_delimiter, group_len, candidate)
        .unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic(expected = "no code groups")]
fn lack_of_code_groups_canot_be_created() {
    let seg_delimiter = '.';
    let prefix = '+';
    let group_len = 4;
    let candidate = "+";
    ToolId::new(prefix, seg_delimiter, group_len, candidate)
        .unwrap_or_else(|error| panic!("{}", error));
}
