#[cfg(test)]
use super::*;

#[test]
fn can_be_created() {
    let id1 = LocationId::new("/", "+001/001").unwrap();
    assert_eq!(format!("{}", id1), "+001/001");

    let id2 = LocationId::new(".", "+1000.0002.0053").unwrap();
    assert_eq!(format!("{}", id2), "+1000.0002.0053");
}

#[test]
fn can_be_created_with_builder() {
    let mut builder = LocationId::builder();
    let _ = builder.id("+189.254.023");
    let id = builder.build();
    assert_eq!(format!("{}", id), "+189.254.023");
}

#[test]
#[should_panic(expected = "empty id string")]
fn empty_cannot_be_created() {
    LocationId::new("/", "").unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic(expected = "code group deviates in length")]
fn wrong_group_length_cannot_be_created() {
    LocationId::new("/", "+0001/0200/012").unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic(expected = "no code groups")]
fn lack_of_code_groups_cannot_be_created() {
    LocationId::new(".", "+").unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic(expected = "mismatching prefix")]
fn wrong_prefix_cannot_be_created() {
    LocationId::new("/", "=001.001").unwrap_or_else(|error| panic!("{}", error));
}

#[test]
fn equals_should_compare_equal() {
    let id1 = LocationId::new(ID_SEGMENT_DELIMITER_DEFAULT, "+120.010.001").unwrap();
    let id2 = LocationId::new(ID_SEGMENT_DELIMITER_DEFAULT, "+120.010.001").unwrap();
    assert!(id1 == id2);
}

#[test]
fn unequals_should_compare_not_equal() {
    let id1 = LocationId::new(ID_SEGMENT_DELIMITER_DEFAULT, "+120.010.001").unwrap();
    let id2 = LocationId::new(ID_SEGMENT_DELIMITER_DEFAULT, "+120.010.002").unwrap();
    assert!(id1 != id2);
}
