#[cfg(test)]
use super::*;

#[test]
fn can_be_created() {
    let id1 = EquipmentId::new('/', 3, "-001/001").unwrap();

    assert_eq!(format!("{}", id1), "-001/001");
}
