use std::collections::BTreeMap;
use std::sync::Arc;
use tonic::{async_trait, transport::Server, Request, Response, Status};
use crate::log_service::LogRecord;
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

pub mod log_service {
    tonic::include_proto!("log_service");
}

use log_service::log_service_server::*;

pub struct LogService {
    memtable: Arc<Mutex<BTreeMap<Vec<u8>, Vec<u8>>>>,
    wal: Arc<Mutex<File>>,
}
impl LogService {
    fn new(file: File) -> Self {
        LogService { memtable: Arc::new(Mutex::new(BTreeMap::new())), wal: Arc::new(Mutex::new(file)) }
    }
}
#[async_trait]
impl log_service::log_service_server::LogService for LogService {
    async fn add_record(&self, request: Request<LogRecord>) -> Result<Response<()>, Status> {
        let record = request.into_inner();
        let key: Vec<u8> = record.key;
        let value: Vec<u8> = record.value;

        {
            let mut wal = self.wal.lock().await;
            wal.write_all(&key).await?;
            wal.write_all(&value).await?;
        }

        {
            let mut memtable = self.memtable.lock().await;
            memtable.insert(key, value);
            if memtable.len() > 10 {
                eprintln!("over high water mark, flushing");
                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open("c1.bin")
                    .await?;
                for (k, v) in memtable.iter() {
                    file.write_all(k).await?;
                    file.write_all(v).await?;
                }
                memtable.clear();
            }
        }

        Ok(Response::new(()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let file = OpenOptions::new()
        .append(true)
        .open("wal.bin")
        .await?;

    let service = LogService::new(file);
    Server::builder()
        .add_service(LogServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}