use std::env;

fn main() {
    if let Ok(_) = env::var("DEP_OPENSSL_LIBRESSL") {
        println!("cargo:rustc-cfg=libressl");
    }

    if let Ok(_) = env::var("DEP_OPENSSL_BABASSL") {
        println!("cargo:rustc-cfg=babassl");
    }

    if let Ok(v) = env::var("DEP_OPENSSL_LIBRESSL_VERSION") {
        println!("cargo:rustc-cfg=libressl{}", v);
    }

    if let Ok(v) = env::var("DEP_OPENSSL_BABASSL_VERSION") {
        println!("cargo:rustc-cfg=babassl{}", v);
    }

    if let Ok(vars) = env::var("DEP_OPENSSL_CONF") {
        for var in vars.split(",") {
            println!("cargo:rustc-cfg=osslconf=\"{}\"", var);
        }
    }

    if let Ok(version) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
        let version = u64::from_str_radix(&version, 16).unwrap();

        if version >= 0x1_00_01_00_0 {
            println!("cargo:rustc-cfg=ossl101");
        }
        if version >= 0x1_00_02_00_0 {
            println!("cargo:rustc-cfg=ossl102");
        }
        if version >= 0x1_01_00_00_0 {
            println!("cargo:rustc-cfg=ossl110");
        }
        if version >= 0x1_01_00_07_0 {
            println!("cargo:rustc-cfg=ossl110g");
        }
        if version >= 0x1_01_01_00_0 {
            println!("cargo:rustc-cfg=ossl111");
        }
    }

    if let Ok(version) = env::var("DEP_OPENSSL_LIBRESSL_VERSION_NUMBER") {
        let version = u64::from_str_radix(&version, 16).unwrap();

        if version >= 0x2_06_01_00_0 {
            println!("cargo:rustc-cfg=libressl261");
        }

        if version >= 0x2_07_00_00_0 {
            println!("cargo:rustc-cfg=libressl270");
        }

        if version >= 0x2_07_01_00_0 {
            println!("cargo:rustc-cfg=libressl271");
        }

        if version >= 0x2_07_03_00_0 {
            println!("cargo:rustc-cfg=libressl273");
        }

        if version >= 0x2_08_00_00_0 {
            println!("cargo:rustc-cfg=libressl280");
        }

        if version >= 0x2_09_01_00_0 {
            println!("cargo:rustc-cfg=libressl291");
        }
    }

    if let Ok(version) = env::var("DEP_OPENSSL_BABASSL_VERSION_NUMBER") {
        let version = u64::from_str_radix(&version, 16).unwrap();

        if version >= 0x8_00_00_00_0 {
            println!("cargo:rustc-cfg=babassl800");
        }

        if version >= 0x8_01_00_00_0 {
            println!("cargo:rustc-cfg=babassl810");
        }
    }
}
