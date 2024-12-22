#[cfg(test)]
mod tests {
    use crate::polynomial::Polynomial;

    #[test]
    fn test_add() {
        let poly1 = Polynomial::new(vec![1, 2, 3]);
        let poly2 = Polynomial::new(vec![4, 5, 6]);
        let result = poly1.add(poly2);
        let expected = Polynomial::new(vec![5, 7, 9]);
        assert_eq!(result.coefficients, expected.coefficients);
    }

    #[test]
    fn test_add_different_lengths() {
        let poly1 = Polynomial::new(vec![1, 2]);
        let poly2 = Polynomial::new(vec![3, 4, 5]);
        let result = poly1.add(poly2);
        let expected = Polynomial::new(vec![4, 6, 5]);
        assert_eq!(result.coefficients, expected.coefficients);
    }

    #[test]
    fn test_multiply() {
        let poly1 = Polynomial::new(vec![1, 2, 3]);
        let poly2 = Polynomial::new(vec![4, 5]);
        let result = poly1.multiply(poly2);
        let expected = Polynomial::new(vec![4, 13, 22, 15]);
        assert_eq!(result.coefficients, expected.coefficients);
    }

    #[test]
    fn test_multiply_zero() {
        let poly1 = Polynomial::new(vec![1, 2, 3]);
        let poly2 = Polynomial::new(vec![0]);
        let result = poly1.multiply(poly2);
        let expected = Polynomial::new(vec![0]);
        assert_eq!(result.coefficients, expected.coefficients);
    }

    #[test]
    fn test_polynomial_multiplication_uneven_length() {
        let p1 = Polynomial::new(vec![7, 36, 12]);
        let p2 = Polynomial::new(vec![3, 8]);
        let result = p1.multiply(p2);
        let expected = Polynomial::new(vec![21, 164, 324, 96]);
        assert_eq!(result.coefficients, expected.coefficients);

        let p1 = Polynomial::new(vec![4, 9, 15]);
        let p2 = Polynomial::new(vec![5]);
        let result = p1.multiply(p2);
        let expected = Polynomial::new(vec![20, 45, 75]);
        assert_eq!(result.coefficients, expected.coefficients);

        let p1 = Polynomial::new(vec![2, 7, 19, 23]);
        let p2 = Polynomial::new(vec![11, 5, 9]);
        let result = p1.multiply(p2);
        let expected = Polynomial::new(vec![22, 87, 262, 411, 286, 207,]);
        assert_eq!(result.coefficients, expected.coefficients);
    }
}
