#[cfg(test)]
mod tests {
    use crate::polynomial_reborn::PolynomialReborn;

    #[test]
    fn test_add() {
        let poly1 = PolynomialReborn::new(vec![1, 2, 3]);
        let poly2 = PolynomialReborn::new(vec![4, 5, 6]);
        let result = poly1.add(poly2);
        let expected = PolynomialReborn::new(vec![5, 7, 9]);
        assert_eq!(result.coefficients, expected.coefficients);
    }

    #[test]
    fn test_add_different_lengths() {
        let poly1 = PolynomialReborn::new(vec![1, 2]);
        let poly2 = PolynomialReborn::new(vec![3, 4, 5]);
        let result = poly1.add(poly2);
        let expected = PolynomialReborn::new(vec![4, 6, 5]);
        assert_eq!(result.coefficients, expected.coefficients);
    }

    #[test]
    fn test_multiply() {
        let poly1 = PolynomialReborn::new(vec![1, 2, 3]);
        let poly2 = PolynomialReborn::new(vec![4, 5]);
        let result = poly1.multiply(poly2);
        let expected = PolynomialReborn::new(vec![4, 13, 22, 15]);
        assert_eq!(result.coefficients, expected.coefficients);
    }

    #[test]
    fn test_multiply_zero() {
        let poly1 = PolynomialReborn::new(vec![1, 2, 3]);
        let poly2 = PolynomialReborn::new(vec![0]);
        let result = poly1.multiply(poly2).remove_leading_zeroes();
        let expected = PolynomialReborn::new(vec![0]);
        assert_eq!(result.coefficients, expected.coefficients);
    }

    #[test]
    fn test_polynomial_multiplication_uneven_length() {
        let p1 = PolynomialReborn::new(vec![7, 36, 12]);
        let p2 = PolynomialReborn::new(vec![3, 8]);
        let result = p1.multiply(p2);
        let expected = PolynomialReborn::new(vec![21, 164, 324, 96]);
        assert_eq!(result.coefficients, expected.coefficients);

        let p1 = PolynomialReborn::new(vec![4, 9, 15]);
        let p2 = PolynomialReborn::new(vec![5]);
        let result = p1.multiply(p2);
        let expected = PolynomialReborn::new(vec![20, 45, 75]);
        assert_eq!(result.coefficients, expected.coefficients);

        let p1 = PolynomialReborn::new(vec![2, 7, 19, 23]);
        let p2 = PolynomialReborn::new(vec![11, 5, 9]);
        let result = p1.multiply(p2);
        let expected = PolynomialReborn::new(vec![22, 87, 262, 411, 286, 207]);
        assert_eq!(result.coefficients, expected.coefficients);
    }

    #[test]
    fn test_divide_simple() {
        let dividend = PolynomialReborn::new(vec![1,-3,0,2]); //vec![2,0,-3,1]
        // 2x^3 - 3x + 1
        let divisor = PolynomialReborn::new(vec![-1, 1]); // x - 1
        let (quotient, remainder) = dividend.divide(divisor);
        assert_eq!(quotient.coefficients, vec![-1, 2, 2]);
        assert_eq!(remainder.remove_leading_zeroes().coefficients, vec![0]);
    }
    #[test]
    fn test_divide_no_remainder() {
        let dividend = PolynomialReborn::new(vec![6, -5, -4, 3]); // 3x^3 - 4x^2 - 5x + 6
        let divisor = PolynomialReborn::new(vec![-2, 3]); // 3x - 2
        let (quotient, remainder) = dividend.divide(divisor);

        assert_eq!(quotient.coefficients, vec![-1, 0, 1]);
        assert_eq!(remainder.coefficients, vec![4, -2, -2]);
    }
    #[test]
    fn test_divide_with_zero_coefficients() {
        let dividend = PolynomialReborn::new(vec![0, 0, 1]); // x^2
        let divisor = PolynomialReborn::new(vec![1]); // 1
        let (quotient, remainder) = dividend.divide(divisor);
        assert_eq!(quotient.coefficients, vec![0, 0, 1]); // x^2
        assert_eq!(remainder.coefficients, vec![0]);
    }
}
