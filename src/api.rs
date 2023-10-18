const BASE_URL: &str = "https://api.robinhood.com";

pub fn url(path: &str) -> String {
    format!("{BASE_URL}/{path}")
}

#[cfg(test)]
mod test {
    use crate::api;

    #[test]
    fn url() {
        let expected = "https://api.robinhood.com/moneys";
        assert_eq!(api::url("moneys"), expected)
    }
}