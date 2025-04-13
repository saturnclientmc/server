use std::{collections::HashMap, str::FromStr};

pub trait ParamMap {
    fn parse_param<T: FromStr>(&self, param: &str) -> Result<T, crate::response::Error>;
}

impl ParamMap for HashMap<String, String> {
    fn parse_param<T: FromStr>(&self, param: &str) -> Result<T, crate::response::Error> {
        match self.get(param) {
            Some(value) => T::from_str(value).map_err(|_| crate::response::Error::InvalidParameter {
                param: param.to_string(),
                reason: "Failed to parse parameter value".to_string(),
            }),
            None => Err(crate::response::Error::ParameterNotFound(param.to_string())),
        }
    }
}

pub fn parse(s: &str) -> Result<(String, HashMap<String, String>), crate::response::Error> {
    let mut parts = s.split("@");

    let method = parts.next()
        .ok_or_else(|| crate::response::Error::InvalidRequest("Empty request".to_string()))?
        .trim()
        .to_lowercase();

    if method.is_empty() {
        return Err(crate::response::Error::InvalidRequest("Method name cannot be empty".to_string()));
    }

    let mut params = HashMap::new();
    for part in parts {
        let key_value: Vec<&str> = part.split("=").collect();
        match key_value.as_slice() {
            [key, value] => {
                let key = key.trim();
                if key.is_empty() {
                    return Err(crate::response::Error::InvalidRequest("Parameter key cannot be empty".to_string()));
                }
                params.insert(key.to_string(), value.trim().to_string());
            },
            _ => return Err(crate::response::Error::InvalidRequest(
                format!("Invalid parameter format in part: {}", part)
            )),
        }
    }

    Ok((method, params))
}
