use std::error::Error;

use angstrom_types::primitive::CHAIN_ID;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{Client, primitives::ByteStream};
use chrono::{Datelike, Utc};

use crate::{blocklog::BlockLog, outputs::TelemetryOutput};

pub const ERROR_BUCKET: &str = "NodeErrors";
pub const ARCHIVE_BUCKET: &str = "ArchivedBlocks";
pub const REGION: &str = "ap-northeast-1";

/// S3 Storage functionality goes as the following.
///
/// 1) If we have a block that doesn't error. We will store the snapshot in an
///    deep-store archive s3 bucket that has a TTL of 1 week. This can be used
///    in the case of some bug that doesn't get automatically detected.
/// 2) If we have a block with an error. This will be put into a separate
///    bucket, No TTL. This will be a bucket that allows quick reads. This
///    bucket will also be linked to PagerDuty so that the team gets notified of
///    anything critical.
#[derive(Clone)]
pub struct S3Storage {
    client:         Client,
    archive_bucket: String,
    error_bucket:   String
}

impl TelemetryOutput for S3Storage {
    fn output(
        &self,
        blocklog: BlockLog
    ) -> std::pin::Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        let this = self.clone();
        Box::pin(async move {
            this.store_snapshot(&blocklog).await.unwrap();
        })
    }
}

impl S3Storage {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        // Explicitly set the region instead of relying on environment detection
        let config = aws_config::from_env()
            .region(Region::new(REGION.to_string()))
            .behavior_version(BehaviorVersion::latest())
            .load()
            .await;

        let client = Client::new(&config);

        Ok(Self {
            client,
            archive_bucket: ARCHIVE_BUCKET.to_string(),
            error_bucket: ERROR_BUCKET.to_string()
        })
    }

    pub async fn store_snapshot(&self, data: &BlockLog) -> eyre::Result<String> {
        let now = Utc::now();

        let chain = CHAIN_ID.get().unwrap();
        let key = format!(
            "{chain}/{}-{}-{}-{}-{:x}.bin",
            now.year(),
            now.month(),
            now.day(),
            data.blocknum(),
            data.error_unique_id()
        );

        let bucket = if data.has_error() { &self.error_bucket } else { &self.archive_bucket };

        self.client
            .put_object()
            .bucket(bucket)
            .key(&key)
            .body(ByteStream::from(data.to_deflate_base64_str().into_bytes()))
            .send()
            .await?;

        Ok(format!("{}/{}", bucket, key))
    }

    pub async fn retrieve_snapshot(&self, key: &str, is_error: bool) -> eyre::Result<BlockLog> {
        let bucket = if is_error { &self.error_bucket } else { &self.archive_bucket };

        let resp = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;

        let bytes = resp.body.collect().await?.into_bytes().to_vec();

        Ok(BlockLog::from_deflate_base64(&bytes))
    }
}
