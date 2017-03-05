mod lib;

fn main() {
  let s = "abcd";
  let t = "becd";
  println!("{}", lib::lcs(s, t));
}
