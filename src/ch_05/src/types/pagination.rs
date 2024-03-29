use handle_errors::Error;
use std::collections::HashMap;

/// Pagination struct that is getting extracted
/// from query params
#[derive(Debug)]
pub(crate) struct Pagination {
    /// The index of the first item that has to be returned
    pub(crate) start: usize,
    /// The index of the last item that has to be returned
    pub(crate) end: usize,
}

/// Extract query parameters from the `/questions` route
/// # Example query
/// GET requests to this route can have a pagination attached so we just
/// return the questions we need
/// `/questions?start=1&end=10`
/// # Example usage
/// ```rust
/// let mut query = HashMap::new();
/// query.insert("start".to_string(), "1".to_string());
/// query.insert("end".to_string(), "10".to_string());
/// let p = types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.start, 1);
/// assert_eq!(p.end, 10);
/// ```
pub(crate) fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // this should be implemented as a From trait
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
}
