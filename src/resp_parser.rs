use std::borrow::Cow;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidFormat,
    InvalidArrayLength,
    InvalidElementLength,
    UnexpectedEnd,
}

pub fn parse_resp_array(data: Cow<'_, str>) -> Result<Vec<String>, ParseError> {
    if !data.starts_with("*") {
        return Err(ParseError::InvalidFormat);
    }

    let (length, remaining) = data[1..]
        .split_once("\r\n")
        .ok_or(ParseError::UnexpectedEnd)?;

    let expected_array_length: usize =
        length.parse().map_err(|_| ParseError::InvalidArrayLength)?;

    let mut result = Vec::with_capacity(expected_array_length);
    let mut data = remaining;

    for _ in 0..expected_array_length {
        if !data.starts_with("$") {
            return Err(ParseError::InvalidFormat);
        }

        let (length, rest) = data[1..]
            .split_once("\r\n")
            .ok_or(ParseError::UnexpectedEnd)?;

        let element_length: usize = length
            .parse()
            .map_err(|_| ParseError::InvalidElementLength)?;

        if rest.len() < element_length + 2 {
            return Err(ParseError::UnexpectedEnd);
        }

        let (value, rest) = rest.split_at(element_length);

        if !rest.starts_with("\r\n") {
            return Err(ParseError::InvalidFormat);
        }

        result.push(value.to_string());

        data = &rest[2..];
    }

    if !data.is_empty() {
        return Err(ParseError::InvalidFormat);
    }

    Ok(result)
}
