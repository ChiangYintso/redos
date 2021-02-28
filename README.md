# Red OS

The Red OS requires Rust nightly version.

## Compile and Make bootable image
```shell script
cargo make x64
```

## Build and run OS in QEMU
Run in RISCV 64
```shell
make run-riscv64
```

Run in x86_64
```shell script
cargo xrun --target x86_64-unknown-redos.json
```

## Run test
Test for RISC-V 64
```shell
$ make run FEAT="test_riscv64"
```

Test for x86_64 (TODO)

```shell script
cargo xtest --target x86_64-unknown-redos.json
```

## References
- [Blog OS](https://github.com/phil-opp/blog_os)
- [xy_os](https://github.com/LearningOS/rcore_step_by_step)
- [writing-an-os-in-rust](https://github.com/rustcc/writing-an-os-in-rust)
- [rcore-os](https://rcore-os.github.io/rCore-Tutorial-deploy/)