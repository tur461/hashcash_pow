Description:

- this program written in Rust-lang, demonstrates the working of proof of work (hashcash) concept used by many blockchains as consensus mechanism (for agreement purpose) and other applications (for mitigation of DOS attack!).

Requirements:

> &nbsp;Rust (stable version)
> &nbsp;Time
> &nbsp;Curiousity
> &nbsp;Thankfulness

Usage:

1. Compile:

   > &nbsp;&nbsp;cargo build --release

2. HELP:

   > Windows:
   >
   > > &nbsp;&nbsp;.\target\release\hashcash_pow.exe
   > > Linux:
   > > &nbsp;&nbsp;./target/release/hashcash_pow

3. Run
   > &nbsp;&nbsp;exec name [num of hex zeroes needed in the hash] [num of threads to be spawned]
4. Examples:
   > On Windows:
   >
   > > &nbsp;&nbsp;&nbsp;.\target\release\hashcash_pow.exe 5 4
   > > On Linux:
   > > &nbsp;&nbsp;&nbsp;./target/release/hashcash_pow 5 4
