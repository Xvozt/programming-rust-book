pub fn gcd(mut m: u64, mut n: u64) -> Result<u64, String> {
    if m == 0 || n == 0 {
        return Err(String::from("Numbers must be positive"))
    }

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }

        m = m % n
    }

    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_five_and_twenty_five() {
        let result = gcd(5, 25);
        assert_eq!(result, Ok(5));
    }
}
