#[cfg(feature = "mimalloc")]
mod mimalloc {
    use mimalloc::MiMalloc;

    #[global_allocator]
    static GLOBAL: MiMalloc = MiMalloc;
}
