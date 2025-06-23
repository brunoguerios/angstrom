use crate::blocklog::BlockLog;

pub mod log;
pub mod s3;

pub trait TelemetryOutput {
    fn output(&self, blocklog: &BlockLog);
}
