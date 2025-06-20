use crate::blocklog::BlockLog;

pub mod ec2;
pub mod log;

pub trait TelemetryOutput {
    fn output(&self, blocklog: &BlockLog);
}
