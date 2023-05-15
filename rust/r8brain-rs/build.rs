use std::env;

fn main() {
  let dst = cmake::Config::new(".").build_target("r8brain").profile("Release").build();
  println!("cargo:rustc-link-search=native={}/build", dst.display());

  let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
  let target_os = get_os_from_triple(target.as_str()).unwrap();

  if target_os.contains("windows") {
    println!("cargo:rustc-link-search=native={}/build/release", dst.display());
  }

  println!("cargo:rustc-link-lib=static=r8brain");

  if target_os.contains("darwin") {
    println!("cargo:rustc-link-lib=dylib=c++");
  } else if target_os.contains("linux") {
    println!("cargo:rustc-link-lib=dylib=stdc++");
  }
}

fn get_os_from_triple(triple: &str) -> Option<&str> {
  triple.splitn(3, "-").nth(2)
}
