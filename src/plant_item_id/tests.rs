#[cfg(test)]
use crate::plant_item_id::*;

#[test]
fn can_be_created() {
    let seg_delimiter = '.';

    let candidate1 = "+0100.0001";
    let tool_id1 = PlantItemId::new(seg_delimiter, candidate1).unwrap();
    let id1 = format!("{}", tool_id1);
    assert_eq!(id1, candidate1);

    let candidate2 = "=01.01";
    let tool_id2 = PlantItemId::new(seg_delimiter, candidate2).unwrap();
    let id2 = format!("{}", tool_id2);
    assert_eq!(id2, candidate2);
}

#[test]
#[should_panic(expected = "empty id string")]
fn empty_cannot_be_created() {
    let seg_delimiter = '.';
    PlantItemId::new(seg_delimiter, "").unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic(expected = "code group deviates in length")]
fn wrong_group_length_cannot_be_created() {
    let seg_delimiter = '.';
    let candidate = "+0100.001";
    PlantItemId::new(seg_delimiter, candidate).unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic(expected = "code group deviates in length")]
fn wrong_delimiter_cannot_be_created() {
    let seg_delimiter = '*';
    let candidate = "+0100.0001*0010";
    PlantItemId::new(seg_delimiter, candidate).unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic(expected = "no code groups")]
fn lack_of_code_groups_cannot_be_created() {
    let seg_delimiter = '.';
    let candidate = "+";
    PlantItemId::new(seg_delimiter, candidate).unwrap_or_else(|error| panic!("{}", error));
}
