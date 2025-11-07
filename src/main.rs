mod pcd;

fn main() {
    let q = pcd::Chunk::debug_empty();
    let _ = q.write("hello.bin");
    pcd::Chunk::read("hello.bin");
}
