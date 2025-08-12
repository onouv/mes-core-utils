#[cfg(test)]
use crate::tool_id::*;

#[test]
fn can_be_created() {
    let seg_delimiter = '.';

    // group length 4, prefix +
    let candidate1 = "+0100.0001";
    let tool_id1 =
        ToolId::new('+', seg_delimiter, 4, candidate1).unwrap_or_else(|error| panic!("{}", error));
    let id1 = format!("{}", tool_id1);
    assert_eq!(id1, candidate1);

    // group length 2, prefix =
    let candidate2 = "=01.01";
    let tool_id2 =
        ToolId::new('=', seg_delimiter, 2, candidate2).unwrap_or_else(|error| panic!("{}", error));
    let id2 = format!("{}", tool_id2);
    assert_eq!(id2, candidate2);
}

#[test]
#[should_panic(expected = "empty id string")]
fn empty_cannot_be_created() {
    let seg_delimiter = '.';
    let prefix = '+';
    let group_len = 4;
    let candidate = "";
    ToolId::new(prefix, seg_delimiter, group_len, candidate)
        .unwrap_or_else(|error| panic!("{}", error));
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
fn lack_of_code_groups_cannot_be_created() {
    let seg_delimiter = '.';
    let prefix = '+';
    let group_len = 4;
    let candidate = "+";
    ToolId::new(prefix, seg_delimiter, group_len, candidate)
        .unwrap_or_else(|error| panic!("{}", error));
}
