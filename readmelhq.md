
# Benchmark
make bench-keyword-20-22-256-2048

make bench-keyword-20-256
make bench-keyword-20-512
make bench-keyword-20-1024
make bench-keyword-20-2048
make bench-keyword-22-256
make bench-keyword-22-512
make bench-keyword-22-1024
make bench-keyword-22-2048

# Note
1. fingerprint is hash(k)+v, not the H(k)||v, so we add 32 bits in our experiments.

32B: 256+32=288
64B: 512+32=544
128B: 1024+32=1056
256B: 2048+32=2080

# Main

cargo run --release -- --logm 20 -l 1774 -e 288 --plaintext-bits 9 
cargo run --release -- --logm 20 -l 1774 -e 544 --plaintext-bits 9 
cargo run --release -- --logm 20 -l 1774 -e 1056 --plaintext-bits 9
cargo run --release -- --logm 20 -l 1774 -e 2080 --plaintext-bits 9 

cargo run --release -- --logm 22 -l 1774 -e 288 --plaintext-bits 9
cargo run --release -- --logm 22 -l 1774 -e 544 --plaintext-bits 9 
cargo run --release -- --logm 22 -l 1774 -e 1056 --plaintext-bits 9
cargo run --release -- --logm 22 -l 1774 -e 2080 --plaintext-bits 9