use std::{collections::HashMap, str::FromStr};

pub trait ParamMap {
    fn parse_param<T: FromStr>(&self, param: &str) -> Result<T, crate::response::Error>;
}

impl ParamMap for HashMap<String, String> {
    fn parse_param<T: FromStr>(&self, param: &str) -> Result<T, crate::response::Error> {
        match self.get(param) {
            Some(value) => T::from_str(value).map_err(|_| crate::response::Error::InvalidParameter),
            None => Err(crate::response::Error::InvalidParameter),
        }
    }
}

pub fn parse(s: &str) -> std::io::Result<(String, HashMap<String, String>)> {
    let mut parts = s.split("@");

    let method = parts.next().unwrap().trim().to_lowercase();

    let params = parts
        .map(|p| {
            let parts = p.split("=").collect::<Vec<&str>>();
            (parts[0].trim().to_string(), parts[1].trim().to_string())
        })
        .collect::<HashMap<String, String>>();

    Ok((method, params))
}
