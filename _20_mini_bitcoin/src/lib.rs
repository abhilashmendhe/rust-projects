pub mod finite_field;


#[cfg(test)]
mod tests {
    use crate::finite_field::FieldElement;


    #[test]
    fn compare_finite_field() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(6, 13);
        
        assert!(a != b); 
        // assert!(a == b); // Fails
    }
}