
# Benchmark

make bench-keyword-20-256
make bench-keyword-20-512
make bench-keyword-20-1024
make bench-keyword-20-2048

make bench-keyword-22-256
make bench-keyword-22-512
make bench-keyword-22-1024
make bench-keyword-22-2048

# main

cargo run --release -- --logm 16 -l 1774 -e 56 --plaintext-bits 10 

cargo run --release -- --logm 17 -l 1774 -e 56 --plaintext-bits 10 

cargo run --release -- --logm 18 -l 1774 -e 56 --plaintext-bits 10 

cargo run --release -- --logm 19 -l 1774 -e 56 --plaintext-bits 9 

cargo run --release -- --logm 20 -l 1774 -e 56 --plaintext-bits 9 

script -f output.txt


# Note
1. 还是很有设计的，关于query和respond， 将这些都设计成1 1 1，然后乘完之后直接就加，这不是非常好的嘛，代价就是一样的，唯一不好的地方就是数据库必须编码成一列。
2. fingerprint是 hash(k)+v 不是额外的hash(k)||v, , 所以我们做实验的时候要+32

32B: 256+32=288
64B: 512+32=544
128B: 1024+32=1056
256B: 2048+32=2080


cargo run --release -- --logm 20 -l 1774 -e 288 --plaintext-bits 9 
cargo run --release -- --logm 20 -l 1774 -e 544 --plaintext-bits 9 
cargo run --release -- --logm 20 -l 1774 -e 1056 --plaintext-bits 9
cargo run --release -- --logm 20 -l 1774 -e 2080 --plaintext-bits 9 

cargo run --release -- --logm 22 -l 1774 -e 288 --plaintext-bits 9
cargo run --release -- --logm 22 -l 1774 -e 544 --plaintext-bits 9 
cargo run --release -- --logm 22 -l 1774 -e 1056 --plaintext-bits 9
cargo run --release -- --logm 22 -l 1774 -e 2080 --plaintext-bits 9