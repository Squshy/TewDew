use async_graphql::InputObject;
use std::cmp::{max, min};

const DEFAULT_SKIP: i16 = 0;
const DEFAULT_LIMIT: i16 = 10;

#[derive(InputObject)]
pub struct ListParams {
    pub skip: Option<i16>,
    pub limit: Option<i16>,
}

pub struct StrictListParams {
    pub skip: i64,
    pub limit: i64,
}

fn convert_to_strict_params(params: ListParams) -> StrictListParams {
    // Defaults
    let skip = params.skip.unwrap_or(DEFAULT_SKIP);
    let limit = params.limit.unwrap_or(DEFAULT_LIMIT);
    // Max values allowed
    let limit = min(limit, 100);
    // Don't allow negative numbers for lookup
    let skip = max(skip, 0);
    let limit = max(limit, 0);
    // Convert to i64 (required for sqlx)
    let skip = i64::from(skip);
    let limit = i64::from(limit);

    StrictListParams { skip, limit }
}

impl From<ListParams> for StrictListParams {
    fn from(params: ListParams) -> Self {
        convert_to_strict_params(params)
    }
}

impl From<Option<ListParams>> for StrictListParams {
    fn from(params: Option<ListParams>) -> Self {
        match params {
            Some(val) => convert_to_strict_params(val),
            None => StrictListParams {
                skip: i64::from(DEFAULT_SKIP),
                limit: i64::from(DEFAULT_LIMIT),
            },
        }
    }
}
