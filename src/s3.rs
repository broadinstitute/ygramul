use crate::config::InputConfig;
use crate::error::Error;
use aws_config::BehaviorVersion;
use std::fmt::Display;
use tokio::io::AsyncBufReadExt;
use tokio::runtime::Runtime;

pub(crate) struct S3Uri {
    pub(crate) bucket: String,
    pub(crate) key: String,
}

impl S3Uri {
    pub(crate) fn new(bucket: String, key: String) -> S3Uri {
        S3Uri { bucket, key }
    }
    pub(crate) fn from_uri(uri: &str) -> Result<S3Uri, Error> {
        if let Some(stripped) = uri.strip_prefix("s3://") {
            let (bucket, key) = stripped.split_once('/').ok_or_else(|| {
                Error::from("S3 URI must contain a bucket and key separated by '/'")
            })?;
            Ok(S3Uri::new(bucket.to_string(), key.to_string()))
        } else {
            Err(Error::from("S3 URI must start with 's3://'"))
        }
    }
}

impl Display for S3Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "s3://{}/{}", self.bucket, self.key)
    }
}

pub(crate) fn cat(config: &InputConfig) -> Result<(), Error> {
    let file = &config.file;
    if let Ok(s3uri) = S3Uri::from_uri(file) {
        let runtime = Runtime::new()?;
        let s3_client = create_s3_client(&runtime)?;
        runtime.block_on(async {
            let resp = s3_client.get_object()
                .bucket(s3uri.bucket)
                .key(s3uri.key)
                .send()
                .await?;
            let body = resp.body;
            let stream = body.into_async_read();
            let reader = tokio::io::BufReader::new(stream);
            let mut lines = reader.lines();
            while let Some(line) = lines.next_line().await? {
                println!("{}", line);
            }
            Ok::<(), Error>(())
        })?;
        Ok(())
    } else {
        todo!();
    }
}

fn create_s3_client(runtime: &Runtime) -> Result<aws_sdk_s3::Client, Error> {
    runtime.block_on(async {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let s3_client = aws_sdk_s3::Client::new(&config);
        Ok(s3_client)
    })
}
