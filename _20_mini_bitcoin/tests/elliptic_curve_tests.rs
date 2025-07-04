use _20_mini_bitcoin::{elliptic_curves::Point, errors::BitcoinErrors};

#[test]
fn check_point_on_curve() {
    match Point::new(Some(-1), Some(-1), 5, 7) {
        Ok(point) => println!("✅ Success: {} is on elliptic curve!", point),
        Err(err) => println!("❌ Error: {}", err),
    };
    match Point::new(Some(-1), Some(-2), 5, 7) {
        Ok(point) => println!("✅ Success: {} is on elliptic curve!", point),
        Err(err) => println!("❌ Error: {}", err),
    };
    match Point::new(Some(2), Some(4), 5, 7) {
        Ok(point) => println!("✅ Success: {} is on elliptic curve!", point),
        Err(err) => println!("❌ Error: {}", err),
    };
    match Point::new(Some(18), Some(77), 5, 7) {
        Ok(point) => println!("✅ Success: {} is on elliptic curve!", point),
        Err(err) => println!("❌ Error: {}", err),
    };
    match Point::new(Some(5), Some(7), 5, 7) {
        Ok(point) => println!("✅ Success: {} is on elliptic curve!", point),
        Err(err) => println!("❌ Error: {}", err),
    };
}   

#[test]
fn compare_points() -> Result<(), BitcoinErrors> {
    let p1 = Point::new(Some(-1), Some(-1), 5, 7)?;
    let p2 = Point::new(Some(-1), Some(-1), 5, 7)?;
    assert!(p1 == p2);
    // assert!(p1 != p2); // fails 
    Ok(())
}