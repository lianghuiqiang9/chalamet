

cargo run --release -- --logm 16 -l 1774 -e 56 --plaintext-bits 10 


cargo run --release -- --logm 17 -l 1774 -e 56 --plaintext-bits 10 

cargo run --release -- --logm 18 -l 1774 -e 56 --plaintext-bits 10 

cargo run --release -- --logm 19 -l 1774 -e 56 --plaintext-bits 9 

cargo run --release -- --logm 20 -l 1774 -e 56 --plaintext-bits 9 

script -f output.txt


# 感悟
1. 还是很有设计的，关于query和respond， 将这些都设计成1 1 1，然后乘完之后直接就加，这不是非常好的嘛，代价就是一样的，唯一的难受的地方就是数据库必须编码成一列。
2. fingerprint是 hash(k)+v 不是额外的hash(k)||v
