use crate::ports::PortError;

pub fn map_pool_err<E>(err: E) -> PortError
where E: std::error::Error {
    error!("{}", err);
    PortError::Connection
}

pub fn map_query_result_err(err: diesel::result::Error) -> PortError {
    error!("QueryResultError: {:#?}, {}", err, err);
    match err {
        diesel::result::Error::DatabaseError(_, _) => PortError::Database, // TODO: Handle kind
        diesel::result::Error::NotFound => PortError::NotFound,
        diesel::result::Error::QueryBuilderError(_) => PortError::InvalidInput,
        diesel::result::Error::DeserializationError(_) => PortError::MalformedData,
        diesel::result::Error::SerializationError(_) => PortError::InvalidInput,
        err => PortError::Custom(Box::new(err)),
    }
}
