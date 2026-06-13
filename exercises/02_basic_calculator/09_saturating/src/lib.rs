pub fn factorial(n: u32) -> u32 {
    let mut result:u32 = 1;
    for i in 1..=n {
        // Use saturating multiplication to stop at the maximum value of u32
        // rather than overflowing and wrapping around
        result = result.saturating_mul(i); 
        // I - the above one will give error --> "can't call method `saturating_mul` on ambiguous numeric type `{integer}`" when we have no type defined for the result here --> `let mut result = 1;` so use `let mut result:u32 = 1;`

        // result = i.saturating_mul(result); 
        // II - the above one works even if we have `let mut result = 1;` since result is passed onto the `saturating_mul` method.
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        assert_eq!(factorial(20), u32::MAX);
    }

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
