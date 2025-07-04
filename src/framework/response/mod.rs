mod api_fail;

pub use api_fail::*;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub type ApiResponse<ResultType> = Result<ResultType, ApiFailure>;

pub trait ApiResult: DeserializeOwned + Debug {}
