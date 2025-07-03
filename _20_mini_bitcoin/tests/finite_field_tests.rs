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
    let f2 = FieldElement::new(4, 5);
    assert!(f1.pow_modulo(10) == f2);

    let f3 = FieldElement::new(3, 23);
    let f4 = FieldElement::new(2, 23);
    assert!(f3.pow_modulo(29) == f4);
}

#[test]
fn div_finite_field() {
    let f1 = FieldElement::new(2, 19);
    let f2 = FieldElement::new(7, 19);
    let f3 = FieldElement::new(3, 19);
    assert!(f1/f2 == f3);

    let f1 = FieldElement::new(7, 19);
    let f2 = FieldElement::new(5, 19);
    let f3 = FieldElement::new(9, 19);
    assert!(f1/f2 == f3);
}

#[test]
fn neg_exp_finite_field() {
    let f1 = FieldElement::new(7, 13);
    let f2 = FieldElement::new(8, 13);
    assert!(f1.pow_modulo(-3) == f2);
}