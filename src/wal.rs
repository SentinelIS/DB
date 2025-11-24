use std::fs::{OpenOptions, File};
use std::io::{Write, Read};

pub struct WalRecord {
    pub page_id: u64,
    pub offset: u64,
    pub length: u64,
    pub data: Vec<u8>,
}

pub struct WriteAheadLog {
    file: File,
}

impl WriteAheadLog {
    pub fn new(path: &str) -> Self {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .read(true)
            .open(path)
            .expect("Could not open WAL file");

        Self { file }
    }

    /// Append a WAL record to the log
    pub fn append(&mut self, record: &WalRecord) {
        // Write page_id, offset, length and data
        self.file.write_all(&record.page_id.to_le_bytes()).unwrap();
        self.file.write_all(&record.offset.to_le_bytes()).unwrap();
        self.file.write_all(&record.length.to_le_bytes()).unwrap();
        self.file.write_all(&record.data).unwrap();

        self.file.flush().unwrap();
    }

    /// Read all WAL records from the log
    pub fn read_all(&mut self) -> Vec<WalRecord> {
        let mut buf = Vec::new();
        self.file.read_to_end(&mut buf).unwrap();

        let mut records = Vec::new();
        let mut pos = 0;

        while pos + 24 <= buf.len() {
            let page_id = u64::from_le_bytes(buf[pos..pos + 8].try_into().unwrap());
            let offset = u64::from_le_bytes(buf[pos + 8..pos + 16].try_into().unwrap());
            let length = u64::from_le_bytes(buf[pos + 16..pos + 24].try_into().unwrap());
            pos += 24;

            if pos + length as usize > buf.len() {
                break;
            }

            let data = buf[pos..pos + length as usize].to_vec();
            pos += length as usize;

            records.push(WalRecord {
                page_id,
                offset,
                length,
                data,
            });
        }

        records
    }
}
