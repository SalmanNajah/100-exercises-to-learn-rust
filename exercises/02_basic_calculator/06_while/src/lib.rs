// Rewrite the factorial function using a `while` loop.
pub fn factorial(mut n: u32) -> u32 {
    let mut a = 1;
    if n==0 || n ==1 { return 1 }
    while n>1 {
        a = a * n; //accumulating the values which needs to be returned
        n = n - 1; // reducing the n val from 5 to 1 
        }
    return a
}
// im very happy that i solved this by myself!

#[cfg(test)]
mod tests {
    use crate::factorial;

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
