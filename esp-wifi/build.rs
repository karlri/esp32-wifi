#[cfg(any(
    feature = "esp32",
    feature = "esp32c2",
    feature = "esp32c3",
    feature = "esp32c6",
    feature = "esp32s2",
    feature = "esp32s3",
))]
fn main() -> Result<(), String> {
    match std::env::var("OPT_LEVEL") {
        Ok(level) => {
            if level != "2" && level != "3" {
                let message = format!(
                    "esp-wifi should be built with optimization level 2 or 3 - yours is {}. 
                    See https://github.com/esp-rs/esp-wifi",
                    level
                )
                .to_string();
                println!("cargo:warning={}", message);
            }
        }
        Err(_err) => (),
    }

    #[cfg(feature = "esp32")]
    {
        println!("cargo:rustc-cfg=esp32");
        // copy and link xtensa va arg hack
        use std::env;
        use std::fs;
        use std::fs::File;
        use std::io::Write;
        use std::path::PathBuf;
        fn copy_file(out: &PathBuf, from: &str, to: &str) {
            let mut file = File::create(out.join(to)).unwrap();
            file.write_all(&fs::read(from).unwrap()).unwrap();
        }
        let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        // provides extern "C" { fn va_list_ptr_get_u32(list: *mut [u32; 3]) -> u32; }
        copy_file(&out, "./lib_va_list_ptr_get_u32.a", "libvahack.a");
        println!("cargo:rustc-link-lib={}", "vahack");
        println!("cargo:rustc-link-search={}", out.display());
    }

    #[cfg(feature = "esp32c2")]
    println!("cargo:rustc-cfg=esp32c2");

    #[cfg(feature = "esp32c3")]
    println!("cargo:rustc-cfg=esp32c3");

    #[cfg(feature = "esp32c6")]
    println!("cargo:rustc-cfg=esp32c6");

    #[cfg(feature = "esp32s2")]
    println!("cargo:rustc-cfg=esp32s2");

    #[cfg(feature = "esp32s3")]
    println!("cargo:rustc-cfg=esp32s3");

    #[cfg(feature = "coex")]
    println!("cargo:rustc-cfg=coex");

    Ok(())
}

#[cfg(not(any(
    feature = "esp32",
    feature = "esp32c2",
    feature = "esp32c3",
    feature = "esp32c6",
    feature = "esp32s2",
    feature = "esp32s3",
)))]
fn main() {
    panic!("Select a chip via it's cargo feature");
}
