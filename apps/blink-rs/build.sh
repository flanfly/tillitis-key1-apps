cargo clean

cargo build --release --target=riscv32i-unknown-none-elf

#/opt/homebrew/opt/llvm/bin/clang \
#  -target riscv32-unknown-none-elf \
#  -march=rv32iczmmul -mabi=ilp32 -mcmodel=medany \
#  -static \
#  -nostdlib -mno-relax -flto \
#  -T ../../../tkey-libs/app.lds target/riscv32i-unknown-none-elf/release/app -o app.elf

llvm-objcopy --input-target=elf32-littleriscv --output-target=binary target/riscv32i-unknown-none-elf/release/app app.bin
