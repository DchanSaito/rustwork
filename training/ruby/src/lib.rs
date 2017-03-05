extern crate libc;

use std::cmp;
use std::ffi::{CStr};

// 引数はstrをとる
// 文字列型にはString、strがあり、"aaa"で定義したものはstr型となる
// 関数は基本はprivateなので他のファイルで使いたいならpubをつける必要がある
pub fn lcs(s: &str, t: &str) -> i32 {
  let n = s.len();
  let m = t.len();
  // シャドーイング
  let mut s = s.chars();
  let mut t = t.chars();
  // 2次元可変長配列を作成
  let mut dp = vec![vec![0; n + 1]; m + 1];

  for i in 0..n {
    for j in 0..m {
      // 感覚的には
      // s[i] == t[j]
      // をしたいのだがRustはサポートしてません
      if s.next() == t.next() {
        dp[i + 1][j + 1] = dp[i][j] + 1;
      } else {
        dp[i + 1][j + 1] = cmp::max(dp[i][j + 1], dp[i + 1][j]);
      }
    }
  }

  // Rubyと似ていて返り値はreturn不要
  // その代わりに ; これはいらない
  dp[n][m]
}

// Rubyで使う関数
#[no_mangle]
pub extern fn lcs_ruby(s: *const libc::c_char, t: *const libc::c_char) -> i32 {
  let s = unsafe { CStr::from_ptr(s) };
  let s = s.to_str().unwrap(); 
  let t = unsafe { CStr::from_ptr(t) };
  let t = t.to_str().unwrap(); 

  let n = s.len();
  let m = t.len();
  let mut s = s.chars();
  let mut t = t.chars();
  let mut dp = vec![vec![0; n + 1]; m + 1];

  for i in 0..n {
    for j in 0..m {
      if s.next() == t.next() {
        dp[i + 1][j + 1] = dp[i][j] + 1;
      } else {
        dp[i + 1][j + 1] = cmp::max(dp[i][j + 1], dp[i + 1][j]);
      }
    }
  }

  dp[n][m]
}
