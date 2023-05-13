use std::env;

use cmake;

fn main() {
  let dst = cmake::build(".");
  let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
  let target_os = get_os_from_triple(target.as_str()).unwrap();

  println!("cargo:rustc-link-search=native={}/build", dst.display());
  println!("cargo:rustc-link-lib=static=audio_engine_juce");

  if target_os.contains("windows") {
    println!("cargo:rustc-link-lib=shell32");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=winmm");
    println!("cargo:rustc-link-lib=imm32");
    println!("cargo:rustc-link-lib=ole32");
    println!("cargo:rustc-link-lib=oleaut32");
    println!("cargo:rustc-link-lib=version");
    println!("cargo:rustc-link-lib=uuid");
    println!("cargo:rustc-link-lib=dinput8");
    println!("cargo:rustc-link-lib=dxguid");
    println!("cargo:rustc-link-lib=setupapi");
  } else if target_os.contains("darwin") {
    println!("cargo:rustc-link-lib=framework=Cocoa");
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=Accelerate");
    println!("cargo:rustc-link-lib=framework=Carbon");
    println!("cargo:rustc-link-lib=framework=ForceFeedback");
    println!("cargo:rustc-link-lib=framework=GameController");
    println!("cargo:rustc-link-lib=framework=CoreHaptics");
    println!("cargo:rustc-link-lib=framework=CoreVideo");
    println!("cargo:rustc-link-lib=framework=CoreAudio");
    println!("cargo:rustc-link-lib=framework=CoreMIDI");
    println!("cargo:rustc-link-lib=framework=AudioToolbox");
    println!("cargo:rustc-link-lib=framework=Metal");
    println!("cargo:rustc-link-lib=iconv");
  } else if target_os.contains("linux") {
    println!("cargo:rustc-link-lib=asound");
  } else {
    panic!("Unsupported target OS: {}", target_os);
  }
}

fn get_os_from_triple(triple: &str) -> Option<&str> {
  triple.splitn(3, "-").nth(2)
}
