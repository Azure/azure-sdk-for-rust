use std::path::PathBuf;

pub fn setup_dotenv() -> Result<PathBuf, dotenv::Error> {
    dotenv::from_filename("./sdk/messaging_eventhubs/.env")
}

pub fn and_nested_result<T, E1, E2>(
    left: Result<Result<T, E1>, E2>,
    right: Result<Result<T, E1>, E2>,
) -> Result<Result<T, E1>, E2> {
    match (left, right) {
        (Ok(Ok(_)), Ok(r)) => Ok(r),
        (Ok(_), Err(err)) => Err(err),
        (Ok(Err(err)), _) => Ok(Err(err)),
        (Err(err), _) => Err(err),
    }
}
