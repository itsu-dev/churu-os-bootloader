# bootloader
A bootloader for UEFI. This project requires a nightly version of Rust.

# Setup
We recommend you to use DevContainer. Open this project with VSCode and follow the guide.
You will able to open this project in DevContainer.

## Requirements
- docker
- qemu

[My article](https://zenn.dev/itsu_dev/scraps/0949ff4cb8039b) may be useful for you to setup your environment.

# Development
## Build
```sh
# In DevContainer
cargo watch -- make
```

## Run
```sh
# In Host Machine
./run.sh
```

# References
- [ゼロからの OS 自作入門 | ゼロからのOS自作入門](https://zero.osdev.jp/)
- [Rustで自作OS - UEFIでブートまで](https://zenn.dev/yubrot/scraps/9735639c0c982d)
- [uchan-nos/mikanos: Educational Operating System](https://github.com/uchan-nos/mikanos)