pub struct MultiBuf {
    chunks: Vec<u32>,
    pos: usize,
}

impl MultiBuf {
    pub fn new() -> Self {
        Self {
            chunks: Vec::with_capacity(64),
            pos: 0,
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn clear(&mut self) {
        self.chunks.clear();
        self.pos = 0;
    }
}

pub fn next_chunk(buf: &mut MultiBuf) {
    buf.chunks.push(buf.pos as u32);
    buf.pos += 1;
    // next.map_or(&[], Vec::as_slice);
}

#[cxx::bridge]
pub mod ffi {
    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf);
    }

    unsafe extern "C++" {
        include!("hello.hpp");

        // type BlobstoreClient;

        // fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;

        fn set_at(v: i32) -> i32;

        // fn search() -> BlobMetadata;

        fn search(
            boards: [u16; 10], spawn_piece: u8, spawn_orientation: u8, spawn_bx: u8, spawn_by: u8,
        ) -> [u16; 10];

        fn search3(multi_but: Pin<&mut MultiBuf>);
    }
}
