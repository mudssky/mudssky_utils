pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
pub fn my_function(a: u64, b: &str, c: Vec<u64>, d: bool) {
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", d);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
