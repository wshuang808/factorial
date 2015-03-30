pub fn factorial(val:i64)->i64{
    let mut result = 1;
    for i in 1..val+1 {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_factorial(){
        assert_eq!(super::factorial(1), 1);
        assert_eq!(super::factorial(2), 2);
        assert_eq!(super::factorial(3), 6);
    }
}
