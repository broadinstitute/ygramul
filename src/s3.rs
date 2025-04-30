use crate::error::Error;
use aws_config::BehaviorVersion;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tokio::io::AsyncBufReadExt;
use tokio::runtime::Runtime;

pub(crate) struct S3Uri {
    pub(crate) bucket: String,
    pub(crate) key: String,
}

pub(crate) trait LineConsumer {
    fn consume(&mut self, line: String) -> Result<(), Error>;
}

struct LinePrinter {}

impl LineConsumer for LinePrinter {
    fn consume(&mut self, line: String) -> Result<(), Error> {
        println!("{}", line);
        Ok(())
    }
}

struct FileCollector {
    dir: String,
    files: Vec<String>,
}

impl FileCollector {
    pub(crate) fn new(dir: String) -> FileCollector {
        FileCollector { dir, files: Vec::new() }
    }
}
impl LineConsumer for FileCollector {
    fn consume(&mut self, line: String) -> Result<(), Error> {
        self.files.push(format!("{}/{}", self.dir, line));
        Ok(())
    }
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

pub(crate) fn cat(file: &str) -> Result<(), Error> {
    let mut line_consumer = LinePrinter {};
    process_file(file, &mut line_consumer)
}

pub(crate) fn ls(dir: &str) -> Result<(), Error> {
    let mut line_consumer = LinePrinter {};
    process_entries(dir, &mut line_consumer)
}

pub(crate) fn collect(dir: &str) -> Result<Vec<String>, Error> {
    let mut file_collector = FileCollector::new(dir.to_string());
    process_entries(dir, &mut file_collector)?;
    Ok(file_collector.files)
}

fn process_file<C: LineConsumer>(file: &str, line_consumer: &mut C) -> Result<(), Error> {
    if let Ok(s3uri) = S3Uri::from_uri(file) {
        let runtime = Runtime::new()?;
        let s3_client = create_s3_client(&runtime)?;
        runtime.block_on(async {
            let resp = s3_client
                .get_object()
                .bucket(s3uri.bucket)
                .key(s3uri.key)
                .send()
                .await?;
            let body = resp.body;
            let stream = body.into_async_read();
            let reader = tokio::io::BufReader::new(stream);
            let mut lines = reader.lines();
            while let Some(line) = lines.next_line().await? {
                line_consumer.consume(line)?;
            }
            Ok::<(), Error>(())
        })?;
        Ok(())
    } else {
        let reader = BufReader::new(File::open(file)?);
        for line in reader.lines() {
            let line = line?;
            line_consumer.consume(line)?;
        }
        Ok(())
    }
}

enum Iteration {
    Start,
    Continuation(String),
    Complete
}

fn process_entries<C: LineConsumer>(dir: &str, line_consumer: &mut C) -> Result<(), Error> {
    if let Ok(s3uri) = S3Uri::from_uri(dir) {
        let runtime = Runtime::new()?;
        let s3_client = create_s3_client(&runtime)?;
        runtime.block_on(async {
            let mut iteration = Iteration::Start;
            loop {
                let request = s3_client
                    .list_objects_v2()
                    .bucket(s3uri.bucket.clone())
                    .prefix(s3uri.key.clone());
                let request =
                match &iteration {
                    Iteration::Start => request,
                    Iteration::Continuation(token) => 
                        request.continuation_token(token.clone()),
                    Iteration::Complete => break
                };
                let response = request.send().await?;
                if response.is_truncated() == Some(true) {
                    iteration = Iteration::Continuation(
                        response.next_continuation_token.ok_or_else(|| {
                            Error::from("No continuation token found in S3 response")
                        })?
                    );
                } else {
                    iteration = Iteration::Complete;
                }
                let contents = response
                    .contents
                    .ok_or_else(|| Error::from("No contents found in S3 response"))?;
                for obj in contents {
                    let key = obj.key.ok_or_else(|| {
                        Error::from("No key found in S3 object")
                    })?;
                    line_consumer.consume(key)?;
                }
            }
            Ok::<(), Error>(())
        })
    } else {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_os_string = path.file_name().ok_or_else(|| {
                Error::from(format!(
                    "Failed to get file name from path '{}'.",
                    path.display()
                ))
            })?;
            let file_name = file_os_string.to_str().ok_or_else(|| {
                Error::from(format!(
                    "Failed to convert file name to string: '{}'.",
                    file_os_string.to_string_lossy()
                ))
            })?;
            line_consumer.consume(file_name.to_string())?;
        }
        Ok(())
    }
}

fn create_s3_client(runtime: &Runtime) -> Result<aws_sdk_s3::Client, Error> {
    runtime.block_on(async {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let s3_client = aws_sdk_s3::Client::new(&config);
        Ok(s3_client)
    })
}
