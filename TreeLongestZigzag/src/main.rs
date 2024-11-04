fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod testn{
    #[test]
    fn test() {
        assert!(true);
    }
}