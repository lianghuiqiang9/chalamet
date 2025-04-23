# run

make bench
RUST_BACKTRACE=1 PIR_NUMBER_OF_ELEMENTS_EXP=16 PIR_LWE_DIM=1774  PIR_ELEM_SIZE_BITS=8192  PIR_PLAINTEXT_BITS=10 PIR_NUM_SHARDS=8 BENCH_DB_GEN=true BENCH_KV=true cargo bench
warning: unexpected `cfg` condition value: `nightly`
  --> bff-modp/src/lib.rs:65:13
   |
65 | #![cfg_attr(feature = "nightly", feature(allocator_internals), needs_allocator)]
   |             ^^^^^^^^^^^^^^^^^^^
   |
   = note: expected values for `feature` are: `binary-fuse`, `binary-fuse-modp`, `default`, `hashbrown`, `libm`, `num-traits`, `rand`, `serde`, and `uniform-random`
   = help: consider adding `nightly` as a feature in `Cargo.toml`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration
   = note: `#[warn(unexpected_cfgs)]` on by default

warning: `xorf` (lib) generated 1 warning
    Finished `bench` profile [optimized] target(s) in 1.27s
     Running benches/bench.rs (target/release/deps/bench-0895647c6047ec3d)
WARNING: HTML report generation will become a non-default optional feature in Criterion.rs 0.4.0.
This feature is being moved to cargo-criterion (https://github.com/bheisler/cargo-criterion) and will be optional in a future version of Criterion.rs. To silence this warning, either switch to cargo-criterion or enable the 'html_reports' feature in your Cargo.toml.

Chosen parameters are: m: 65536, lwe_dim: 1774, elem_size: 8192, plaintext-bits: 10
Benchmarking offline: true
Benchmarking keyword: true
Setting up DB for benchmarking. This might take a while...
[KV] Setup complete, starting benchmarks...
[KV] Benchmarking online steps...
[KV] Filter Params: segment-len: 2048, segment-len-mask: 2047, segment-count-len: 73728
[KV] Starting client query benchmarks
Benchmarking lwe/[KV] create client query params, lwe_dim: 1774, matrix_height: 77824, omega: 820: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.1s, or reduce sample count to 50.
lwe/[KV] create client query params, lwe_dim: 1774, matrix_height: 77824, omega: 820                                                                            
                        time:   [84.131 ms 85.836 ms 87.577 ms]
                        change: [+3.7225% +6.0358% +8.7112%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
lwe/[KV] create client query prepare, lwe_dim: 1774, matrix_height: 77824, omega: 820                                                                             
                        time:   [15.358 µs 15.708 µs 16.140 µs]
                        change: [-41.797% -38.297% -34.082%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  3 (3.00%) high mild
  11 (11.00%) high severe
lwe/[KV] server response, lwe_dim: 1774, matrix_height: 77824, omega: 820                                                                            
                        time:   [27.830 ms 28.940 ms 30.363 ms]
                        change: [+21.569% +27.023% +34.088%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
lwe/[KV] client parse server response, lwe_dim: 1774, matrix_height: 77824, omega: 820                                                                            
                        time:   [230.89 µs 236.36 µs 242.59 µs]
                        change: [-12.245% -8.3380% -3.8144%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
[KV] Finished client query benchmarks
[KV] Benchmarking offline steps...
lwe/[KV] derive LHS from seed, lwe_dim: 1774, m: 77824, w: 820                                                                           
                        time:   [324.77 ms 332.10 ms 339.65 ms]
                        change: [+2.7836% +6.0058% +8.9217%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) low mild
[KV] Starting DB generation benchmarks
Benchmarking lwe/[KV] generate db and params, m: 77824, w: 820: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 100.0s. You may wish to increase target time to 757.1s.
lwe/[KV] generate db and params, m: 77824, w: 820                                                                          
                        time:   [62.485 s 67.105 s 71.959 s]
                        change: [-8.7076% -0.2040% +8.5338%] (p = 0.96 > 0.05)
                        No change in performance detected.
[KV] Finished DB generation benchmarks
