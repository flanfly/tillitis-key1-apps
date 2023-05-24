cargo build --release --target=riscv32i-unknown-none-elf
llvm-objcopy --input-target=elf32-littleriscv --output-target=binary target/riscv32i-unknown-none-elf/release/app app.bin

ls -lah app.bin
