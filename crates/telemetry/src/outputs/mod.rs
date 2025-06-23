use std::pin::Pin;

use crate::blocklog::BlockLog;

pub mod log;
pub mod s3;

pub trait TelemetryOutput {
    fn output<'a>(
        &'a self,
        blocklog: &'a BlockLog
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
}
