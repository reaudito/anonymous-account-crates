# Modes

## Dev Mode
Quick Development: Leveraging dev-mode

RISC0_DEV_MODE=1 cargo run --release

RISC0_DEV_MODE=1 RUST_LOG=info cargo run --release

## Real Proof Generation

RISC0_DEV_MODE=0 cargo run --release
