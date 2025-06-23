use std::{error::Error, path::Path};

use aws_config::Region;
use aws_sdk_s3::{Client, primitives::ByteStream};
use chrono::{Datelike, Utc};

use super::TelemetryOutput;

pub struct S3Storage {
    client: Client,
    bucket: String
}

impl S3Storage {
    pub async fn new(bucket: &str, region: &str) -> Result<Self, Box<dyn Error>> {
        let config = aws_config::from_env()
            .region(Region::new(region.to_string()))
            .load()
            .await;
        let client = Client::new(&config);
        Ok(Self { client, bucket: bucket.to_string() })
    }

    /// Uploads data organized by `year/month/day/filename`
    pub async fn upload_block(
        &self,
        data: Vec<u8>,
        filename: &str
    ) -> Result<String, Box<dyn Error>> {
        let now = Utc::now();
        let key = format!("{}/{}/{}/{}", now.year(), now.month(), now.day(), filename);

        let byte_stream = ByteStream::from(data);
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(byte_stream)
            .send()
            .await?;

        Ok(key)
    }

    /// Downloads an object using full key (you can build it based on the date)
    pub async fn download_object(&self, key: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let resp = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        let data = resp.body.collect().await?.into_bytes().to_vec();
        Ok(data)
    }
}
