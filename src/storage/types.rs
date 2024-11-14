pub mod types {
    #[derive(PartialEq, Clone, Copy)]
    pub enum CompressionStrategy {
        Uncompressed,
        RunLength,
        Bitmap,
        Xor
    }
}