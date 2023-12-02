pub fn lines(s: &str) -> Vec<&str> {
    let mut result = Vec::new();

    let parts = s.split("\n");

    for p in parts {
        result.push(p)
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
