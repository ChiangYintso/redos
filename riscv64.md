## 反汇编
rust-objdump target/riscv64imac-unknown-none-elf/debug/redos -d --arch-name=riscv64

## 生成镜像
rust-objcopy target/riscv64imac-unknown-none-elf/debug/redos --strip-all -O binary target/riscv64imac-unknown-none-elf/debug/kernel.bin
