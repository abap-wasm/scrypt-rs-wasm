use wasm_bindgen::prelude::*;

use scrypt::{scrypt, ScryptParams};

struct Test {
  password: &'static str,
  salt: &'static str,
  log_n: u8,
  r: u32,
  p: u32,
}

#[wasm_bindgen]
pub fn run() -> String {

  let t = Test {
    log_n: 15,
    r: 8,
    p: 1,
    password: "password",
    salt: "salt",
  };

  let params = ScryptParams::new(t.log_n, t.r, t.p).unwrap();
  let mut result = vec![0u8; 64];

  scrypt(
      t.password.as_bytes(),
      t.salt.as_bytes(),
      &params,
      &mut result,
  ).unwrap();

  let foo = result.iter().map(|b| format!("{:02x}", b)).collect::<String>();

  format!("Hello {}", foo)
}