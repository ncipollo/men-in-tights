use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION,
    CONTENT_TYPE, USER_AGENT,
};

pub trait StrHeaderMap {
    fn insert_str(&mut self, name: &'static str, value: &'static str) -> Option<HeaderValue>;
    fn insert_str_value(&mut self, name: HeaderName, value: &'static str) -> Option<HeaderValue>;
}

impl StrHeaderMap for HeaderMap {
    fn insert_str(&mut self, name: &'static str, value: &'static str) -> Option<HeaderValue> {
        self.insert(header_name(name), header_value(value))
    }

    fn insert_str_value(&mut self, name: HeaderName, value: &'static str) -> Option<HeaderValue> {
        self.insert(name, header_value(value))
    }
}

fn header_name(name: &'static str) -> HeaderName {
    name.parse()
        .expect(&format!("failed to parse header name: {name}"))
}

fn header_value(value: &'static str) -> HeaderValue {
    value
        .parse()
        .expect(&format!("failed to parse header value: {value}"))
}

pub fn standard() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert_str_value(ACCEPT, "*/*");
    headers.insert_str_value(ACCEPT_ENCODING, "gzip, deflate");
    headers.insert_str_value(
        ACCEPT_LANGUAGE,
        "en;q=1, fr;q=0.9, de;q=0.8, ja;q=0.7, nl;q=0.6, it;q=0.5",
    );
    headers.insert_str_value(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded; charset=utf-8",
    );
    headers.insert_str_value(header_name("X-Robinhood-API-Version"), "1.0.0");
    headers.insert_str_value(CONNECTION, "keep-alive");
    headers.insert_str_value(USER_AGENT, "Robinhood/823 (iPhone; iOS 7.1.2; Scale/2.00)");
    return headers;
}

#[cfg(test)]
mod test {
    use reqwest::header::{ACCEPT, HeaderMap, HeaderValue};
    use crate::headers::{standard, StrHeaderMap};

    #[test]
    fn str_header_map_insert_str() {
        let mut headers = HeaderMap::new();
        let result = headers.insert_str("name", "value");
        let expected = "value".parse::<HeaderValue>().unwrap();
        assert_eq!(result, None);
        assert_eq!(headers.get("name").unwrap(), expected);
    }

    #[test]
    fn str_header_map_insert_value() {
        let mut headers = HeaderMap::new();
        let result = headers.insert_str_value(ACCEPT, "value");
        let expected = "value".parse::<HeaderValue>().unwrap();
        assert_eq!(result, None);
        assert_eq!(headers.get(ACCEPT).unwrap(), expected);
    }

    #[test]
    fn standard_verify_headers() {
        let headers = standard();
        assert_eq!(7, headers.len())
    }
}
