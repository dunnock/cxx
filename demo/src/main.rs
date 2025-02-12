#[cxx::bridge(namespace = "org::blobstore")]
mod ffi {
    // Shared structs with fields visible to both languages.
    pub struct BlobMetadata {
        size: usize,
        tags: Vec<String>,
    }

    // Rust types and signatures exposed to C++.
    extern "Rust" {
        type MultiBuf;

        fn create_multibuf(chunk: Vec<u8>) -> Box<MultiBuf>;
        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("demo/include/blobstore.h");

        type BlobstoreClient;

        pub fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        pub fn put(&self, parts: &mut MultiBuf) -> u64;
        pub fn tag(&self, blobid: u64, tag: &str);
        pub fn metadata(&self, blobid: u64) -> BlobMetadata;
    }
}

// An iterator over contiguous chunks of a discontiguous file object.
//
// Toy implementation uses a Vec<Vec<u8>> but in reality this might be iterating
// over some more complex Rust data structure like a rope, or maybe loading
// chunks lazily from somewhere.
pub struct MultiBuf {
    pub chunks: Vec<Vec<u8>>,
    pub pos: usize,
}
pub fn create_multibuf(chunk: Vec<u8>) -> Box<MultiBuf> {
    Box::new(MultiBuf {
        chunks: vec![chunk],
        pos: 0,
    })
}
pub fn next_chunk(buf: &mut MultiBuf) -> &[u8] {
    let next = buf.chunks.get(buf.pos);
    buf.pos += 1;
    next.map_or(&[], Vec::as_slice)
}

fn main() {
    let client = ffi::new_blobstore_client();

    // Upload a blob.
    let chunks = vec![b"fearless".to_vec(), b"concurrency".to_vec()];
    let mut buf = MultiBuf { chunks, pos: 0 };
    let blobid = client.put(&mut buf);
    println!("blobid = {}", blobid);

    // Add a tag.
    client.tag(blobid, "rust");

    // Read back the tags.
    let metadata = client.metadata(blobid);
    println!("tags = {:?}", metadata.tags);
}
