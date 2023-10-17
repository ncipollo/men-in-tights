use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION,
    CONTENT_TYPE, USER_AGENT,
};

pub fn standard() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, header_value("*/*"));
    headers.insert(ACCEPT_ENCODING, header_value("gzip, deflate"));
    headers.insert(
        ACCEPT_LANGUAGE,
        header_value("en;q=1, fr;q=0.9, de;q=0.8, ja;q=0.7, nl;q=0.6, it;q=0.5"),
    );
    headers.insert(
        CONTENT_TYPE,
        header_value("application/x-www-form-urlencoded; charset=utf-8"),
    );
    headers.insert(
        header_name("X-Robinhood-API-Version"),
        header_value("1.0.0"),
    );
    headers.insert(CONNECTION, header_value("keep-alive"));
    headers.insert(
        USER_AGENT,
        header_value("Robinhood/823 (iPhone; iOS 7.1.2; Scale/2.00)"),
    );
    return headers;
}

fn header_name(name: &'static str) -> HeaderName {
    name.parse().expect("failed to parse header name")
}

fn header_value(value: &'static str) -> HeaderValue {
    HeaderValue::from_static(value)
}


#[cfg(test)]
mod test {
    use crate::headers::standard;

    #[test]
    fn standard_verify_headers() {
        let headers = standard();
        assert_eq!(7, headers.len())
    }
}