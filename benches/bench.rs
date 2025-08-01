use criterion::{criterion_group, criterion_main, BenchmarkGroup, Criterion};
use keyword_pir_lwe::api::{
  generate_index_query_params, generate_kv_query_params, BaseParams,
  CommonParams, KVShard, Response, Shard,
};
use keyword_pir_lwe::db::{DatabaseMatrix, KeyValue};
use pi_rs_cli_utils::*;
use std::time::Duration;
use std::mem;
use keyword_pir_lwe::db::FilterParams;
//use std::time::{Instant};

fn total_data_size(data: &Vec<Vec<u32>>) -> usize {
  data.iter().map(|inner_vec| inner_vec.len() * mem::size_of::<u32>()).sum()
}

fn criterion_benchmark(c: &mut Criterion) {
  let CLIFlags {
    m,
    lwe_dim,
    elem_size,
    plaintext_bits,
    offline,
    keyword,
    ..
  } = parse_from_env();
  //println!("CLIFlags: {:?}",CLIFlags);
  let mut lwe_group = c.benchmark_group("lwe");

  println!("Chosen parameters are: m: {}, lwe_dim: {}, elem_size: {}, plaintext-bits: {}", m, lwe_dim, elem_size, plaintext_bits);
  println!("Benchmarking offline: {}", offline);
  println!("Benchmarking keyword: {}", keyword);
  println!("Setting up DB for benchmarking. This might take a while...");

  if keyword {
    let kv_db_eles = bench_utils::generate_kv_db_elems(m, (elem_size + 7) / 8);
    let keys: Vec<String> = kv_db_eles.iter().map(|e| e.0.clone()).collect();
    let values: Vec<String> = kv_db_eles.iter().map(|e| e.1.clone()).collect();
    //let _start = Instant::now();
    let shard = KVShard::from_base64_strings(
      &keys,
      &values,
      lwe_dim,
      m,
      elem_size,
      plaintext_bits,
    )
    .unwrap();
    //let _duration = _start.elapsed();
    //println!("Time elapsed in bff: {:?}", _duration);
    println!("A cdot db size : {:?} Kbytes", total_data_size(&shard.get_base_params().get_rhs())/1024);
    //println!("Size of _q: {:?} Kbytes", mem::size_of_val(_q.as_slice())/1024);

    println!("[KV] Setup complete, starting benchmarks...");

    println!("[KV] Benchmarking online steps...");
    _bench_client_kv_query(
      &mut lwe_group,
      &shard,
      (keys[0].clone(), values[0].clone()),
    );

    if offline {
      println!("[KV] Benchmarking offline steps...");
      lwe_group.sample_size(10);
      lwe_group.measurement_time(Duration::from_secs(100)); // To remove a warning, you can increase this to 500 or more.
      _bench_kv_db_generation(&mut lwe_group, &shard, &keys, &values);
    }
  } else {
    let db_eles = bench_utils::generate_db_eles(m, (elem_size + 7) / 8);
    let shard = Shard::from_base64_strings(
      &db_eles,
      lwe_dim,
      m,
      elem_size,
      plaintext_bits,
    )
    .unwrap();
    println!("[I] Setup complete, starting benchmarks");

    println!("[I] Benchmarking online steps...");
    _bench_client_query(&mut lwe_group, &shard);

    if offline {
      println!("[I] Benchmarking offline steps...");
      lwe_group.sample_size(10);
      lwe_group.measurement_time(Duration::from_secs(100)); // To remove a warning, you can increase this to 500 or more.
      _bench_db_generation(&mut lwe_group, &shard, &db_eles);
    }
  }
}

// 程序入口
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn _bench_db_generation(
  c: &mut BenchmarkGroup<criterion::measurement::WallTime>,
  shard: &Shard,
  db_eles: &[String],
) {
  let db = shard.get_db();
  let bp = shard.get_base_params();
  let w = db.get_row_width_self();

  c.bench_function(
    format!(
      "derive LHS from seed, lwe_dim: {}, m: {}, w: {}",
      bp.get_dim(),
      db.get_matrix_height(),
      w
    ),
    |b| {
      b.iter(|| CommonParams::from(bp));
    },
  );

  println!("Starting DB generation benchmarks");
  c.bench_function(
    format!(
      "generate db and params, m: {}, w: {}",
      db.get_matrix_height(),
      w
    ),
    |b| {
      b.iter(|| {
        Shard::from_base64_strings(
          db_eles,
          bp.get_dim(),
          db.get_matrix_height(),
          db.get_elem_size(),
          db.get_plaintext_bits(),
        )
        .unwrap();
      });
    },
  );
  println!("Finished DB generation benchmarks");
}

fn _bench_kv_db_generation(
  c: &mut BenchmarkGroup<criterion::measurement::WallTime>,
  shard: &KVShard,
  keys: &[String],
  values: &[String],
) {
  let db = shard.get_db();
  let bp = shard.get_base_params();
  let w = db.get_row_width_self();

  c.bench_function(
    format!(
      "[KV] derive LHS from seed, lwe_dim: {}, m: {}, w: {}",
      bp.get_dim(),
      db.get_matrix_height(),
      w
    ),
    |b| {
      b.iter(|| CommonParams::from(bp));
    },
  );

  println!("[KV] Starting DB generation benchmarks");
  c.bench_function(
    format!(
      "[KV] generate db and params, m: {}, w: {}",
      db.get_matrix_height(),
      w
    ),
    |b| {
      b.iter(|| {
        KVShard::from_base64_strings(
          keys,
          values,
          bp.get_dim(),
          db.get_matrix_height(),
          db.get_elem_size(),
          db.get_plaintext_bits(),
        )
        .unwrap();
      });
    },
  );
  println!("[KV] Finished DB generation benchmarks");
}

fn _bench_client_query(
  c: &mut BenchmarkGroup<criterion::measurement::WallTime>,
  shard: &Shard,
) {
  let db = shard.get_db();
  let bp = shard.get_base_params();
  let cp = CommonParams::from(bp);
  let w = db.get_row_width_self();
  let idx = 10;

  println!("Starting client query benchmarks");
  let mut _qp = generate_index_query_params(&cp, bp).unwrap();
  let _q = _qp.generate_query(idx).unwrap();
  let mut _resp = shard.respond(&_q).unwrap();
  c.bench_function(
    format!(
      "create client query params, lwe_dim: {}, m: {}, omega: {}",
      bp.get_dim(),
      db.get_matrix_height(),
      w
    ),
    |b| {
      b.iter(|| generate_index_query_params(&cp, bp));
    },
  );

  c.bench_function(
    format!(
      "client query prepare, lwe_dim: {}, m: {}, w: {}",
      bp.get_dim(),
      db.get_matrix_height(),
      w
    ),
    |b| {
      b.iter(|| {
        _qp.used = false;
        _qp.generate_query(idx).unwrap();
      });
    },
  );

  c.bench_function(
    format!(
      "server response compute, lwe_dim: {}, m: {}, w: {}",
      bp.get_dim(),
      db.get_matrix_height(),
      w
    ),
    |b| {
      b.iter(|| {
        shard.respond(&_q).unwrap();
      });
    },
  );

  c.bench_function(
    format!(
      "client parse server response, lwe_dim: {}, m: {}, w: {}",
      bp.get_dim(),
      db.get_matrix_height(),
      w
    ),
    |b| {
      b.iter(|| {
        let deser: Response = bincode::deserialize(&_resp).unwrap();
        _qp.parse_resp_as_base64(&deser);
      });
    },
  );
  println!("Finished client query benchmarks");
}

// 进入c和参数shard和keyword DB
fn _bench_client_kv_query(
  c: &mut BenchmarkGroup<criterion::measurement::WallTime>,
  shard: &KVShard,
  example_kv: (String, String),
) {
  let db = shard.get_db();
  let &FilterParams {
    seed: _,
    segment_length,
    segment_length_mask,
    segment_count_length,
  } = db.get_filter_params();

  println!(
    "[KV] Filter Params: segment-len: {}, segment-len-mask: {}, segment-count-len: {}",
    segment_length, segment_length_mask, segment_count_length
  );

  let bp = shard.get_base_params();

  let kv = KeyValue::from_base64_strings(
    &example_kv.0,
    &example_kv.1,
    bp.get_elem_size(),
    bp.get_plaintext_bits(),
  )
  .unwrap();
  let cp = CommonParams::from(bp);
  let w = db.get_row_width_self();

  println!("[KV] Starting client query benchmarks");
  let mut _qp = generate_kv_query_params(&cp, bp).unwrap();
  
  let _q = _qp.generate_query(&kv.key).unwrap();
  //println!("_q {:?} ", _q);
  //let _qq=_q.as_slice();
  //println!("_qq {:?} ", _qq);
  println!("Size of _q: {:?} Kbytes", mem::size_of_val(_q.as_slice())/1024); // 切片指向的数据大小（不是指针本身）
  let mut _resp = shard.respond(&_q).unwrap();
  //println!("_resp {:?} ", _resp);
  println!("Size of _resp: {:?} bytes", mem::size_of_val(&_resp)); // 切片指向的数据大小（不是指针本身）

  let resp: Response = bincode::deserialize(&_resp).unwrap();
  let output = _qp.parse_resp_as_row(&resp, &kv.key).unwrap();
  println!("expect_output {:?}, actural_output {:?} ", kv.value, output);
  //assert_eq!(output, kv.value);

  c.bench_function(
    format!(
      "[KV] create client query params, lwe_dim: {}, matrix_height: {}, omega: {}",
      bp.get_dim(),
      db.get_matrix_height(),
      w
    ),
    |b| {
      b.iter(|| generate_kv_query_params(&cp, bp));
    },
  );

  c.bench_function(
    format!(
      "[KV] create client query prepare, lwe_dim: {}, matrix_height: {}, omega: {}",
      bp.get_dim(),
      db.get_matrix_height(),
      w
    ),
    |b| {
      b.iter(|| {
        _qp.used = false;
        _qp.generate_query(&kv.key).unwrap();
      });
    },
  );

  c.bench_function(
    format!(
      "[KV] server response, lwe_dim: {}, matrix_height: {}, omega: {}",
      bp.get_dim(),
      db.get_matrix_height(),
      w
    ),
    |b| {
      b.iter(|| {
        shard.respond(&_q).unwrap();
      });
    },
  );

  c.bench_function(
    format!(
      "[KV] client parse server response, lwe_dim: {}, matrix_height: {}, omega: {}",
      bp.get_dim(),
      db.get_matrix_height(),
      w
    ),
    |b| {
      b.iter(|| {
        let deser: Response = bincode::deserialize(&_resp).unwrap();
        _qp.parse_resp_as_base64(&deser, &kv.key).unwrap();
      });
    },
  );
  println!("[KV] Finished client query benchmarks");
}

mod bench_utils {
  use rand_core::{OsRng, RngCore};

  pub fn generate_db_eles(num_eles: usize, ele_byte_len: usize) -> Vec<String> {
    let mut eles = Vec::with_capacity(num_eles);
    for _ in 0..num_eles {
      let mut ele = vec![0u8; ele_byte_len];
      OsRng.fill_bytes(&mut ele);
      let ele_str = base64::encode(ele);
      eles.push(ele_str);
    }
    eles
  }

  pub fn generate_kv_db_elems(
    num_eles: usize,
    ele_byte_len: usize,
  ) -> Vec<(String, String)> {
    let mut v = Vec::with_capacity(num_eles);
    for _ in 0..num_eles {
      let mut key = vec![0u8; 32];
      let mut ele = vec![0u8; ele_byte_len];
      OsRng.fill_bytes(&mut key);
      OsRng.fill_bytes(&mut ele);
      let key_str = base64::encode(key);
      let ele_str = base64::encode(ele);
      v.push((key_str, ele_str));
    }
    v
  }
}
