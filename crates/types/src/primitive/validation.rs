use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderPoolNewOrderResult {
    Valid,
    Invalid,
    TransitionedToBlock,
    Error(String)
}

impl OrderPoolNewOrderResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, OrderPoolNewOrderResult::Valid)
    }
}

impl<T: Into<Self>, E: std::error::Error> From<Result<T, E>> for OrderPoolNewOrderResult {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(v) => v.into(),
            Err(e) => OrderPoolNewOrderResult::Error(e.to_string())
        }
    }
}
//
