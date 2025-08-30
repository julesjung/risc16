default: run

# assemble program
assemble program="memcopy":
    @mkdir -p build
    @customasm assembler/architecture.asm programs/{{program}}.asm -o build/{{program}}.bin

# run program in emulator
run program="memcopy": (assemble program)
    @cargo run --manifest-path=emulator/Cargo.toml -q -- build/{{program}}.bin

test:
    @cargo test --manifest-path=emulator/Cargo.toml

lint:
    @cargo clippy --manifest-path=emulator/Cargo.toml --all

clean:
    rm -rf build/*