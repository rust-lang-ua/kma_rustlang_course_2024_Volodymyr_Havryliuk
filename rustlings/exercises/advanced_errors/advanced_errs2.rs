use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

// This is the custom error type that we will be using for the parser for
// `Climate`.
#[derive(Debug, PartialEq)]
enum ParseClimateError {
    Empty,
    BadLen,
    NoCity,
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
}

// This `From` implementation allows the `?` operator to work on
// `ParseIntError` values.
impl From<ParseIntError> for ParseClimateError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}

// This `From` implementation allows the `?` operator to work on
// `ParseFloatError` values.
impl From<ParseFloatError> for ParseClimateError {
    fn from(e: ParseFloatError) -> Self {
        Self::ParseFloat(e)
    }
}

// The `Display` trait allows for other code to obtain the error formatted
// as a user-visible string.
impl Display for ParseClimateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use ParseClimateError::*;
        match self {
            Empty => write!(f, "empty input"),
            BadLen => write!(f, "incorrect number of fields"),
            NoCity => write!(f, "no city name"),
            ParseInt(e) => write!(f, "error parsing year: {}", e),
            ParseFloat(e) => write!(f, "error parsing temperature: {}", e),
        }
    }
}

// Implement the `Error` trait for `ParseClimateError`.
impl Error for ParseClimateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseClimateError::ParseInt(err) => Some(err),
            ParseClimateError::ParseFloat(err) => Some(err),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Climate {
    city: String,
    year: u32,
    temp: f32,
}

// Parser for `Climate`.
impl FromStr for Climate {
    type Err = ParseClimateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseClimateError::Empty);
        }

        let v: Vec<_> = s.split(',').collect();
        if v.len() != 3 {
            return Err(ParseClimateError::BadLen);
        }

        let (city, year, temp) = match &v[..] {
            [city, year, temp] if !city.is_empty() => (city.to_string(), year, temp),
            [_, _, _] => return Err(ParseClimateError::NoCity),
            _ => return Err(ParseClimateError::BadLen),
        };

        let year: u32 = year.parse().map_err(ParseClimateError::from)?;
        let temp: f32 = temp.parse().map_err(ParseClimateError::from)?;

        Ok(Climate { city, year, temp })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{:?}", "Hong Kong,1999,25.7".parse::<Climate>()?);
    println!("{:?}", "".parse::<Climate>()?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let res = "".parse::<Climate>();
        assert_eq!(res, Err(ParseClimateError::Empty));
        assert_eq!(res.unwrap_err().to_string(), "empty input");
    }

    #[test]
    fn test_short() {
        let res = "Boston,1991".parse::<Climate>();
        assert_eq!(res, Err(ParseClimateError::BadLen));
        assert_eq!(res.unwrap_err().to_string(), "incorrect number of fields");
    }

    #[test]
    fn test_long() {
        let res = "Paris,1920,17.2,extra".parse::<Climate>();
        assert_eq!(res, Err(ParseClimateError::BadLen));
        assert_eq!(res.unwrap_err().to_string(), "incorrect number of fields");
    }

    #[test]
    fn test_no_city() {
        let res = ",1997,20.5".parse::<Climate>();
        assert_eq!(res, Err(ParseClimateError::NoCity));
        assert_eq!(res.unwrap_err().to_string(), "no city name");
    }

    #[test]
    fn test_parse_int_neg() {
        let res = "Barcelona,-25,22.3".parse::<Climate>();
        assert!(matches!(res, Err(ParseClimateError::ParseInt(_))));

        let err = res.unwrap_err();
        if let ParseClimateError::ParseInt(ref inner) = err {
            assert_eq!(
                err.to_string(),
                format!("error parsing year: {}", inner.to_string())
            );
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_parse_int_bad() {
        let res = "Beijing,foo,15.0".parse::<Climate>();
        assert!(matches!(res, Err(ParseClimateError::ParseInt(_))));

        let err = res.unwrap_err();
        if let ParseClimateError::ParseInt(ref inner) = err {
            assert_eq!(
                err.to_string(),
                format!("error parsing year: {}", inner.to_string())
            );
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_parse_float() {
        let res = "Manila,2001,bar".parse::<Climate>();
        assert!(matches!(res, Err(ParseClimateError::ParseFloat(_))));

        let err = res.unwrap_err();
        if let ParseClimateError::ParseFloat(ref inner) = err {
            assert_eq!(
                err.to_string(),
                format!("error parsing temperature: {}", inner.to_string())
            );
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_parse_good() {
        let res = "Munich,2015,23.1".parse::<Climate>();
        assert_eq!(
            res,
            Ok(Climate {
                city: "Munich".to_string(),
                year: 2015,
                temp: 23.1,
            })
        );
    }

    #[test]
    #[ignore]
    fn test_downcast() {
        let res = "São Paulo,-21,28.5".parse::<Climate>();
        assert!(matches!(res, Err(ParseClimateError::ParseInt(_))));

        let err = res.unwrap_err();
        let inner: Option<&(dyn Error + 'static)> = err.source();
        assert!(inner.is_some());
        assert!(inner.unwrap().is::<ParseIntError>());
    }
}
