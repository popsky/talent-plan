use crate::error::KvError;
use bson::Document;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{prelude::*, BufReader, BufWriter, Cursor, SeekFrom};
use std::path::{Path, PathBuf};
use std::result;
use structopt::StructOpt;

pub type Result<T> = result::Result<T, KvError>;
const COMPACTION_THRESHOLD: i32 = 500;
#[derive(Serialize, Deserialize, Debug, StructOpt)]
pub enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}
/// A key-value store that support get,set,rm operations.
///
/// # Examples
///
/// ```
/// use kvs::KvStore;
///
/// let mut kv = KvStore::new();
/// kv.set("foo".to_owned(),"bar".to_owned());
/// assert_eq!(kv.get("foo".to_owned()).unwrap(), "bar".to_owned());
/// kv.remove("foo".to_owned());
/// assert_eq!(kv.get("foo".to_owned()), None);
pub struct KvStore {
    reader: BufReader<std::fs::File>,
    writer: BufWriter<std::fs::File>,
    keys: HashMap<String, u64>,
    count: i32,
    path: PathBuf,
}
impl KvStore {
    fn maybe_compact(&mut self) -> Result<()> {
        self.count = self.count + 1;
        if self.count >= COMPACTION_THRESHOLD {
            self.writer.flush()?;
            let tmp = self.path.with_file_name("log.bson.tmp");
            let mut writer =
                BufWriter::new(OpenOptions::new().append(true).create(true).open(&tmp)?);
            let mut keys = HashMap::new();
            let mut pos = writer.seek(SeekFrom::Current(0))?;
            for (key, offset) in &self.keys {
                self.reader.seek(SeekFrom::Start(*offset))?;
                let doc = Document::from_reader(&mut self.reader)?;
                doc.to_writer(&mut writer)?;
                keys.insert(String::from(key), pos);
                pos = writer.seek(SeekFrom::Current(0))?;
            }
            fs::rename(tmp, &self.path)?;
            let mut writer = BufWriter::new(
                OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&self.path)?,
            );
            writer.seek(SeekFrom::End(0))?;
            let reader = BufReader::new(File::open(&self.path)?);
            self.writer = writer;
            self.reader = reader;
            self.keys = keys;
            self.count = 0;
        }
        Ok(())
    }
    fn build_from_file(path: impl AsRef<Path>) -> Result<HashMap<String, u64>> {
        let mut keys = HashMap::new();
        let mut file = File::open(&path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let mut reader = Cursor::new(&buf[..]);
        let mut offset = reader.position();
        while let Ok(doc) = Document::from_reader(&mut reader) {
            let cmd = bson::from_document::<Command>(doc)?;
            match cmd {
                Command::Set { key, value: _ } => {
                    keys.insert(key, offset);
                }
                Command::Rm { key } => {
                    keys.remove(&key);
                }
                _ => panic!("log contain unsurported command."),
            }
            offset = reader.position();
        }
        Ok(keys)
    }
    pub fn open<P: AsRef<Path>>(dir: P) -> Result<KvStore> {
        let path = Path::new(dir.as_ref()).join("log.bson");
        let mut keys = HashMap::new();
        if path.exists() {
            keys = KvStore::build_from_file(&path)?;
        }
        let mut writer = BufWriter::new(OpenOptions::new().append(true).create(true).open(&path)?);
        writer.seek(SeekFrom::End(0))?;
        let reader = BufReader::new(File::open(&path)?);
        Ok(KvStore {
            reader,
            writer,
            keys,
            count: 0,
            path,
        })
    }
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        self.writer.flush()?;
        let value = self.keys.get(&key);
        if let Some(offset) = value {
            self.reader.seek(SeekFrom::Start(*offset))?;
            let doc = Document::from_reader(&mut self.reader)?;
            let cmd: Command = bson::from_document(doc)?;
            if let Command::Set { key: _, value } = cmd {
                return Ok(Some(value));
            } else {
                panic!("expect Set command present")
            }
        }
        Ok(None)
    }
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::Set { key, value };
        let bson = bson::to_bson(&cmd)?;
        let doc = bson.as_document().unwrap();
        let offset = self.writer.seek(SeekFrom::Current(0))?;
        doc.to_writer(&mut self.writer)?;
        if let Command::Set { key, value: _ } = cmd {
            self.keys.insert(key, offset);
            self.maybe_compact()?;
        }
        Ok(())
    }
    pub fn remove(&mut self, key: String) -> Result<()> {
        if let None = self.keys.get(&key) {
            return Err(KvError::KeyNotFound);
        }
        let cmd = Command::Rm { key };
        let bson = bson::to_bson(&cmd)?;
        let doc = bson.as_document().unwrap();
        doc.to_writer(&mut self.writer)?;
        if let Command::Rm { key } = cmd {
            self.keys.remove(&key);
            self.maybe_compact()?;
        }
        Ok(())
    }
}
