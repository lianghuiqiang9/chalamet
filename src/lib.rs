pub mod api;
// TODO: make this private ideally, need it for benchmarking atm
pub mod db;
pub mod errors;
mod utils;

use clap::{App, Arg};
use std::env;

//#[derive(Debug)]
pub struct CLIFlags {
  pub m: usize,
  pub lwe_dim: usize,
  pub plaintext_bits: usize,
  pub elem_size: usize,
  pub offline: bool,
  pub keyword: bool,
}

pub fn parse_cli_flags() -> CLIFlags {
  let matches = App::new("PIR example")
    .version("0.0.1")
    .author("Alex Davidson <coela@alxdavids.xyz>")
    .about("Flags for setting PIR parameters")
    .arg(
      Arg::with_name("matrix_height")
        .short("m")
        .long("matrix_height")
        .takes_value(true)
        .default_value("16")
        .help("Log2 of height of DB matrix"),
    )
    .arg(
      Arg::with_name("ele_size")
        .short("e")
        .long("ele_size")
        .takes_value(true)
        .default_value("13")
        .help("Log2 of element bit length"),
    )
    .arg(
      Arg::with_name("plaintext_bits")
        .short("p")
        .long("plaintext_bits")
        .takes_value(true)
        .default_value("10")
        .help("Number of plaintext bits encoded in each entry of DB matrix"),
    )
    .arg(
      Arg::with_name("dim")
        .short("d")
        .long("dim")
        .takes_value(true)
        .default_value("2048")
        .help("LWE dimension"),
    )
    .get_matches();

  let elem_size =
    parse_exp_to_usize(String::from(matches.value_of("ele_size").unwrap()));
  let lwe_dim: usize = String::from(matches.value_of("dim").unwrap())
    .parse()
    .unwrap();
  let plaintext_bits: usize =
    String::from(matches.value_of("plaintext_bits").unwrap())
      .parse()
      .unwrap();
  let m = parse_exp_to_usize(String::from(
    matches.value_of("matrix_height").unwrap(),
  ));
  CLIFlags {
    m,
    lwe_dim,
    plaintext_bits,
    elem_size,
    offline: true,
    keyword: true,
  }
}

// 将输入的信息放到CLFlags中
pub fn parse_from_env() -> CLIFlags {
  let elem_size: usize =
    env::var("PIR_ELEM_SIZE_BITS").unwrap().parse().unwrap();
  let lwe_dim: usize = env::var("PIR_LWE_DIM").unwrap().parse().unwrap();
  let plaintext_bits: usize =
    env::var("PIR_PLAINTEXT_BITS").unwrap().parse().unwrap();
  let m = parse_exp_to_usize(env::var("PIR_NUMBER_OF_ELEMENTS_EXP").unwrap());
  let offline = parse_val_to_bool(env::var("BENCH_DB_GEN").unwrap());
  let keyword = parse_val_to_bool(env::var("BENCH_KV").unwrap());
  CLIFlags {
    m,
    lwe_dim,
    plaintext_bits,
    elem_size,
    offline,
    keyword,
  }
}

pub fn parse_exp_to_usize(v: String) -> usize {
  let exp: u32 = v.parse().unwrap();
  let res = 2_u32.pow(exp) as usize;
  res
}

pub fn parse_val_to_bool(v: String) -> bool {
  let res: bool = v.parse().unwrap();
  res
}
