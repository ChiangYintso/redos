# Red OS

The Red OS requires Rust nightly version.

## Compile and Make bootable image
```shell script
cargo make x64
```

## Build and run OS in QEMU
run in RISCV 64
```shell
make run-riscv64
```

```shell script
cargo xrun --target x86_64-unknown-redos.json
```

## Run test
Test RISC-V 64
```shell
$ make run FEAT="test_riscv64"
```

```shell script
cargo xtest
```

## References
- [Blog OS](https://github.com/phil-opp/blog_os)
- [xy_os](https://github.com/LearningOS/rcore_step_by_step)
- [writing-an-os-in-rust](https://github.com/rustcc/writing-an-os-in-rust)
- [rcore-os](https://rcore-os.github.io/rCore-Tutorial-deploy/)