use crate::log_service::LogRecord;
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use tonic::{async_trait, transport::Server, Request, Response, Status};

pub mod log_service {
    tonic::include_proto!("log_service");
}

use log_service::log_service_server::*;

struct State {
    memtable: BTreeMap<Vec<u8>, Vec<u8>>,
    filenames: Vec<String>,
}

impl State {
    fn new(filenames: Vec<String>) -> State {
        State {
            filenames,
            memtable: BTreeMap::new(),
        }
    }
}

pub struct LogService {
    wal: Arc<Mutex<File>>,
    state: Arc<RwLock<State>>,
}
impl LogService {
    fn new(file: File, filenames: Vec<String>) -> Self {
        LogService {
            state: Arc::new(RwLock::new(State::new(filenames))),
            wal: Arc::new(Mutex::new(file)),
        }
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
            let mut state = self.state.write().await;
            state.memtable.insert(key, value);
            if state.memtable.len() > 10 {
                eprintln!("over high water mark, flushing");
                let filename = format!("c{}.bin", state.filenames.len());
                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(&filename)
                    .await?;
                for (k, v) in state.memtable.iter() {
                    file.write_all(k).await?;
                    file.write_all(v).await?;
                }
                state.filenames.push(filename);
                state.memtable.clear();
                if state.filenames.len() > 9 {
                    unimplemented!("merge files and update filenames!");
                }
            }
        }

        Ok(Response::new(()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let wal = OpenOptions::new().append(true).open("wal.bin").await?;

    let mut filenames = Vec::new();
    for i in 0..10 {
        let filename = format!("c{i}.bin");
        match File::open(&filename).await {
            Ok(_) => { filenames.push(filename) }
            Err(_) => { break }
        }
    }

    println!("Found files: {filenames:?}");

    let service = LogService::new(wal, filenames);
    Server::builder()
        .add_service(LogServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
