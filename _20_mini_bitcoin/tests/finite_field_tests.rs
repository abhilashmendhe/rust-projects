use _20_mini_bitcoin::finite_field::FieldElement;

#[test]
fn compare_finite_field() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(6, 13);
    
    assert!(a != b); 
    // assert!(a == b); // Fails
}

#[test]
fn add_finite_field() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(12, 13);
    let c = FieldElement::new(6, 13);
    assert!(a+b == c);
}

#[test]
fn sub_finite_field() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(12, 13);
    let c = FieldElement::new(8, 13);
    assert!(a-b == c);
}

#[test]
fn mul_finite_field() {
    let a = FieldElement::new(3, 13);
    let b = FieldElement::new(12, 13);
    let c = FieldElement::new(10, 13);
    assert!(a*b == c);
}

#[test]
fn exp_finite_field() {
    let f1 = FieldElement::new(2, 5);
    assert!(f1.pow_modulo(10) == 4);

    let f2 = FieldElement::new(3, 23);
    assert!(f2.pow_modulo(29) == 2);
}