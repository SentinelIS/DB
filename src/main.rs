mod storage;
mod wal;

use storage::{StorageEngine, Page};
use wal::{WriteAheadLog, WalRecord};

fn main() {
    println!("DBMS starting...");

    let mut storage = StorageEngine::new("data.db");
    let mut wal = WriteAheadLog::new("data.wal");

    // Create and modify a page
    let mut page = storage.allocate_page();

    // Change some data on the page
    let value = 1234u32.to_le_bytes();
    page.data[0..4].copy_from_slice(&value);

    // Write WAL record
    let record = WalRecord {
        page_id: page.id,
        offset: 0,
        length: 4,
        data: value.to_vec(),
    };
    wal.append(&record);

    // Write page to storage
    storage.write_page(&page);

    // Test: Read back the page
    let read_page = storage.read_page(page.id);
    let num = u32::from_le_bytes(read_page.data[0..4].try_into().unwrap());

    println!("Read number from page: {}", num);
}

