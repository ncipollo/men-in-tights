const BASE_URL: &str = "https://api.robinhood.com";

pub fn api(path: &str) -> String {
    format!("{BASE_URL}/{path}")
}

#[cfg(test)]
mod test {
    use crate::urls;

    #[test]
    fn api() {
        let expected = "https://api.robinhood.com/moneys";
        assert_eq!(urls::api("moneys"), expected)
    }
}