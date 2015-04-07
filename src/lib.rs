pub fn exec(val:i64)->i64{
    let mut result = 1;
    if val > 0 {
        for i in 1..val+1 {
            result *= i;
        }
    }
    result
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_exec(){
        assert_eq!(super::exec(1), 1);
        assert_eq!(super::exec(2), 2);
        assert_eq!(super::exec(3), 6);
    }
}
