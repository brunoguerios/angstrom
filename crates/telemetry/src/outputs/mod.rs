use std::pin::Pin;

use crate::blocklog::BlockLog;

pub mod log;
pub mod s3;

pub trait TelemetryOutput: Send {
    fn output<'a>(&'a self, blocklog: BlockLog) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
}
