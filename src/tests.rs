use crate::DataPoint;

#[test]
fn test_update_and_reverse() {
    let dp = DataPoint::new('a', 3);
    dp.update('b');
    dp.update('c');
    assert_eq!(dp.val.get(), 'c');
    dp.reverse();
    assert_eq!(dp.val.get(), 'a'); // Assuming reverse sets to the oldest value
}