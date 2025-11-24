use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

pub const PAGE_SIZE: usize = 4096;

pub struct Page {
    pub id: u64,
    pub data: [u8; PAGE_SIZE],
}

impl Page {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            data: [0; PAGE_SIZE],
        }
    }
}

pub struct StorageEngine {
    file: File,
}

impl StorageEngine {
    /// Opens or creates the databse-file
    pub fn new(path: &str) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("Could not open database file");

        Self { file }
    }

    /// Reads one page with given ID
    pub fn read_page(&mut self, page_id: u64) -> Page {
        let mut page = Page::new(page_id);

        let offset = page_id * PAGE_SIZE as u64;
        self.file
            .seek(SeekFrom::Start(offset))
            .expect("Seek failed");

        self.file
            .read_exact(&mut page.data)
            .expect("Failed to read page");

        page
    }

    /// Writes a Page
    pub fn write_page(&mut self, page: &Page) {
        let offset = page.id * PAGE_SIZE as u64;

        self.file
            .seek(SeekFrom::Start(offset))
            .expect("Seek failed");

        self.file
            .write_all(&page.data)
            .expect("Failed to write page");

        self.file.flush().unwrap();
    }

    /// Creates a new Page a the end of file
    pub fn allocate_page(&mut self) -> Page {
        let file_len = self.file.metadata().unwrap().len();
        let next_page_id = file_len / PAGE_SIZE as u64;

        let page = Page::new(next_page_id);
        self.write_page(&page);

        page
    }
}
