Description:

- this program written in Rust-lang, demonstrates the working of proof of work (hashcash) concept used by many blockchains as consensus mechanism (for agreement purpose) and other applications (for mitigation of DOS attack!).
  Requirements:

1. Rust (stable version)
2. Time
3. Curiousity
4. Thankfulness

Usage:

1. Compile:
   - cargo build --release
2. Run
   - HELP:
   * Windows: .\target\release\hashcash_pow.exe
   * Linux: ./target/release/hashcash_pow
   - exec name (from target->release) <num of hex zeroes needed in the hash> <num of threads to be spawned>
   - eg., .\target\release\hashcash_pow.exe 5 4 (on windows)
   - eg., ./target/release/hashcash_pow 5 4 (on Linux)
