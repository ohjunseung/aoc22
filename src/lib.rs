pub mod one {
    pub fn one() -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert!(one::one())
    }
}
