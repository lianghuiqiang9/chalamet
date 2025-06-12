use keyword_pir_lwe::api::{
    generate_kv_query_params, BaseParams,
    CommonParams, KVShard, Response,
  };
use keyword_pir_lwe::db::KeyValue;
use std::time::Instant;
use std::mem;
use keyword_pir_lwe::db::FilterParams;

use rand_core::{OsRng, RngCore};

extern crate clap;
use clap::{Arg, App, value_t};

fn generate_kv_db_elems(
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

fn total_data_size(data: &Vec<Vec<u32>>) -> usize {
  data.iter().map(|inner_vec| inner_vec.len() * mem::size_of::<u32>()).sum()
}


#[derive(Debug)]
pub struct Args {
    pub logm: u32,
    pub lwe_dim: usize,
    pub elem_size: usize,
    pub plaintext_bits: usize,
}

impl Args {
    pub fn parse_args() -> Self {
        let matches = App::new("LWE/PIR 参数配置工具")
            .version("1.0")
            .author("Your Name")
            .about("用于设置 LWE 或 PIR 系统的参数")
            .arg(Arg::with_name("logm")
                .short("m")
                .long("logm")
                .value_name("LOGM")
                .help("数据库大小指数 m = 2^LOGM，默认为16")
                .takes_value(true)
                .default_value("16"))
            .arg(Arg::with_name("lwe_dim")
                .short("l")
                .long("lwe-dim")
                .value_name("DIM")
                .help("LWE 维度，默认为1774")
                .takes_value(true)
                .default_value("1774"))
            .arg(Arg::with_name("elem_size")
                .short("e")
                .long("elem-size")
                .value_name("BITS")
                .help("元素大小（bit），默认为56")
                .takes_value(true)
                .default_value("56"))
            .arg(Arg::with_name("plaintext_bits")
                .long("plaintext-bits")
                .value_name("BITS")
                .help("明文位数，默认为10")
                .takes_value(true)
                .default_value("10"))
            .get_matches();

        Args {
            logm: value_t!(matches, "logm", u32).unwrap_or_else(|e| e.exit()),
            lwe_dim: value_t!(matches, "lwe_dim", usize).unwrap_or_else(|e| e.exit()),
            elem_size: value_t!(matches, "elem_size", usize).unwrap_or_else(|e| e.exit()),
            plaintext_bits: value_t!(matches, "plaintext_bits", usize).unwrap_or_else(|e| e.exit()),
        }
    }
}


fn main(){
  let args = Args::parse_args();
  let m = 1usize << args.logm;

  println!("logm: {}", args.logm);
  println!("m = 2^{} = {}", args.logm, m);
  println!("LWE 维度: {}", args.lwe_dim);
  println!("元素大小（bit）: {}", args.elem_size);
  println!("明文位数: {}", args.plaintext_bits);

  let kv_db_eles = generate_kv_db_elems(m, (args.elem_size + 7) / 8); // +7 完全是为了多取一个u8
  let keys: Vec<String> = kv_db_eles.iter().map(|e| e.0.clone()).collect();
  let values: Vec<String> = kv_db_eles.iter().map(|e| e.1.clone()).collect();
  //let _start = Instant::now();
  let shard = KVShard::from_base64_strings(
    &keys,
    &values,
    args.lwe_dim,
    m,
    args.elem_size,
    args.plaintext_bits,
  )
  .unwrap();
  //let _duration = _start.elapsed();
  //println!("Time elapsed in bff: {:?}", _duration);
  println!("A cdot db size                    : {:?} Kbytes", (total_data_size(&shard.get_base_params().get_rhs()) as f32)/(1024 as f32));
  //println!("Size of _q: {:?} Kbytes", mem::size_of_val(_q.as_slice())/1024);

  let example_kv:(String, String)=(keys[0].clone(), values[0].clone());
  //println!("example_kv: {:?}",example_kv);
  let db = shard.get_db();
  let &FilterParams {
    seed: _,
    segment_length: _,
    segment_length_mask: _,
    segment_count_length:_,
  } = db.get_filter_params();


  //println!(
  //  "[KV] Filter Params: segment-len: {}, segment-len-mask: {}, segment-count-len: {}",
  //  segment_length, segment_length_mask, segment_count_length
  //);

  let bp = shard.get_base_params();

  let kv = KeyValue::from_base64_strings(
    &example_kv.0,
    &example_kv.1,
    bp.get_elem_size(),
    bp.get_plaintext_bits(),
  )
  .unwrap();
  let cp = CommonParams::from(bp);
  println!("row: {:?}, col: {:?}",db.entries.len(), db.entries[0].len());
  //let w = db.get_row_width_self();
  //println!("w: {:?}",w);
  let _start = Instant::now();
  let mut _qp = generate_kv_query_params(&cp, bp).unwrap();
  let _duration = _start.elapsed();
  println!("Time elapsed in client pre-query  : {:?}", _duration);

  let _start = Instant::now();
  let _q = _qp.generate_query(&kv.key).unwrap();
  let _duration = _start.elapsed();
  println!("Time elapsed in client query      : {:?}", _duration);
  //println!("_q {:?} ", _q);
  //let _qq=_q.as_slice();
  //println!("_qq {:?} ", _qq);
  //println!("_q.as_slice(): {}",_q.as_slice().len());
  println!("Size of query                     : {:?} Kbytes", (mem::size_of_val(_q.as_slice())as f32)/(1024 as f32)); // 切片指向的数据大小（不是指针本身）
  
  let _start = Instant::now();
  let mut _resp = shard.respond(&_q).unwrap();
  let _duration = _start.elapsed();
  println!("Time elapsed in server respond    : {:?}", _duration);
  //println!("_resp {:?} ", _resp);
  //println!("_resp.len(): {:?}",_resp.len());
  println!("Size of response                  : {:?} bytes", mem::size_of_val(&_resp)); // 切片指向的数据大小（不是指针本身）

  let _start = Instant::now();
  let resp: Response = bincode::deserialize(&_resp).unwrap();
  //println!("kv.key : {:?}",kv.key);
  let output = _qp.parse_resp_as_row(&resp, &kv.key).unwrap();
  let _duration = _start.elapsed();
  println!("Time elapsed in client reconstruct: {:?}", _duration);
  println!("expect_output {:?}, actural_output {:?} ", kv.value, output);
  //assert_eq!(output, kv.value);


}