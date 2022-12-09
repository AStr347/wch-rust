# WCH-RUST

## Instruments

* Cargo 1.65.0 (4bc8f24d3 2022-10-20)
* MounRiver Studio Community toolchain
  * OpenOCD
* VSCode Extensions
  * rust-analyzer - v0.3.1309
  * Native Debug - Patched v0.26.0 -> 0.27.0-beta git fork(<https://github.com/AStr347/Native_Debug_With_Registers>)

## Flash/Debug

For flashing and debuging set self paths into '.vscode/settings.json' like this:

```json
{
    "projname":"wch-rust",
    "openocddir":".../MRS_Community/toolchain/OpenOCD/bin",
}
```

## Build

Add to your Rust riscv32 targets support

```console
rustup target add riscv32imac-unknown-none-elf
```

## Objdump, ObjCopy and other

Add to your cargo support

```console
cargo install cargo-binutils
rustup component add llvm-tools-preview
```
