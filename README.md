# Reconstructing Rust Types: A Practical Guide for Reverse Engineers

This repository contains the supplementary files for the [_Reconstructing Rust Types: A Practical Guide for Reverse Engineers_ at NorthSec 2025](https://nsec.io/session/2025-reconstructing-rust-types-a-practical-guide-for-reverse-engineers.html), presented on May 15, 2025.

# Workshop setup

You will need:

- A static disassembler / decompiler tool, such as IDA, Ghidra, or Binary Ninja. This workshop is tool-agnostic, and can be done with the static disassembler / decompiler tool of your choice.
- An environment where you can download malware samples without them being removed from disk. We will not be executing, debugging, or dynamically analyzing malware; however, if you are on Windows, your antivirus software may remove the sample we're analyzing today after it's downloaded and extracted. You may wish to set up a non-Windows virtual machine for analysis, or set up an antivirus exclusion rule.

# Sample program: `panic_print_example`

- Source code: [materials/panic_print_example/](materials/panic_print_example/)
- Prebuilt binary: [materials/prebuilt-files/panic_print_example.exe](materials/prebuilt-files/panic_print_example.exe)
    - This binary was built with Rust 1.85.0, targeting `x86_64-pc-windows-gnu`, with the [Cargo `dev` profile](https://doc.rust-lang.org/cargo/reference/profiles.html#dev) (a debug build, with all symbols intact).
    - This binary has SHA-256 hash `ce15d62ba8e6d80c01c2977ecd136c912461b61b901418f1ffd338932c3fec15`

# Malware sample: _RustyClaw_

- Sample binary: [materials/prebuilt-files/b1fe8fbbb0b6de0f1dcd4146d674a71c511488a9eb4538689294bd782df040df.zip](materials/prebuilt-files/b1fe8fbbb0b6de0f1dcd4146d674a71c511488a9eb4538689294bd782df040df.zip), or [🔗 Download from MalwareBazaar](https://bazaar.abuse.ch/sample/b1fe8fbbb0b6de0f1dcd4146d674a71c511488a9eb4538689294bd782df040df/)
    - The password for the downloaded ZIP is `infected`.
    - The sample is malware, so treat with caution to avoid accidental execution. We will only be examining this sample statically.
    - The binary has SHA-256 hash `b1fe8fbbb0b6de0f1dcd4146d674a71c511488a9eb4538689294bd782df040df`
