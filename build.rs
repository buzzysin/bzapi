fn main() {
  println!("cargo:rerun-if-changed=./migrations/sqlite");
  println!("cargo:rerun-if-changed=./migrations/postgres");
  println!("cargo:rerun-if-changed=../bzauth-rs/bzauth-rs");
  println!("cargo:rerun-if-changed=./.trigger");
  println!("cargo:rerun-if-changed=./.env");
}