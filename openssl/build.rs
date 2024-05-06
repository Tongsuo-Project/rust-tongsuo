#![allow(
    clippy::inconsistent_digit_grouping,
    clippy::uninlined_format_args,
    clippy::unusual_byte_groupings
)]

use std::env;

fn main() {
    if env::var("DEP_OPENSSL_LIBRESSL").is_ok() {
        println!("cargo:rustc-cfg=libressl");
    }

    if env::var("DEP_OPENSSL_BORINGSSL").is_ok() {
        println!("cargo:rustc-cfg=boringssl");
    }

    if let Ok(v) = env::var("DEP_OPENSSL_LIBRESSL_VERSION_NUMBER") {
        let version = u64::from_str_radix(&v, 16).unwrap();

        if version >= 0x2_05_00_00_0 {
            println!("cargo:rustc-cfg=libressl250");
        }
        if version >= 0x2_05_01_00_0 {
            println!("cargo:rustc-cfg=libressl251");
        }
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
        if version >= 0x3_01_00_00_0 {
            println!("cargo:rustc-cfg=libressl310");
        }
        if version >= 0x3_02_01_00_0 {
            println!("cargo:rustc-cfg=libressl321");
        }
        if version >= 0x3_03_02_00_0 {
            println!("cargo:rustc-cfg=libressl332");
        }
        if version >= 0x3_04_00_00_0 {
            println!("cargo:rustc-cfg=libressl340");
        }
        if version >= 0x3_05_00_00_0 {
            println!("cargo:rustc-cfg=libressl350");
        }
        if version >= 0x3_06_00_00_0 {
            println!("cargo:rustc-cfg=libressl360");
        }
        if version >= 0x3_06_01_00_0 {
            println!("cargo:rustc-cfg=libressl361");
        }
        if version >= 0x3_07_00_00_0 {
            println!("cargo:rustc-cfg=libressl370");
        }
        if version >= 0x3_08_00_00_0 {
            println!("cargo:rustc-cfg=libressl380");
        }
        if version >= 0x3_08_02_00_0 {
            println!("cargo:rustc-cfg=libressl382");
        }
        if version >= 0x3_09_00_00_0 {
            println!("cargo:rustc-cfg=libressl390");
        }
    }

    if let Ok(_) = env::var("DEP_OPENSSL_TONGSUO") {
        println!("cargo:rustc-cfg=tongsuo");
    } else if let Ok(_) = env::var("DEP_OPENSSL_BABASSL") {
        println!("cargo:rustc-cfg=babassl");
    }

    if let Ok(_) = env::var("DEP_OPENSSL_BABASSL") {
        println!("cargo:rustc-cfg=babassl");
    }

    if let Ok(v) = env::var("DEP_OPENSSL_TONGSUO_VERSION") {
        println!("cargo:rustc-cfg=tongsuo{}", v);
    } else if let Ok(v) = env::var("DEP_OPENSSL_BABASSL_VERSION") {
        println!("cargo:rustc-cfg=babassl{}", v);
    }

    if let Ok(v) = env::var("DEP_OPENSSL_BABASSL_VERSION") {
        println!("cargo:rustc-cfg=babassl{}", v);
    }

    if let Ok(vars) = env::var("DEP_OPENSSL_CONF") {
        for var in vars.split(',') {
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
        if version >= 0x1_01_00_08_0 {
            println!("cargo:rustc-cfg=ossl110h");
        }
        if version >= 0x1_01_01_00_0 {
            println!("cargo:rustc-cfg=ossl111");
        }
        if version >= 0x3_00_00_00_0 {
            println!("cargo:rustc-cfg=ossl300");
        }
        if version >= 0x3_01_00_00_0 {
            println!("cargo:rustc-cfg=ossl310");
        }
        if version >= 0x3_02_00_00_0 {
            println!("cargo:rustc-cfg=ossl320");
        }
    }

    if let Ok(version) = env::var("DEP_OPENSSL_TONGSUO_VERSION_NUMBER") {
        let version = u64::from_str_radix(&version, 16).unwrap();

        if version >= 0x8_00_00_00_0 {
            println!("cargo:rustc-cfg=tongsuo800");
        }

        if version >= 0x8_01_00_00_0 {
            println!("cargo:rustc-cfg=tongsuo810");
        }

        if version >= 0x8_02_00_00_0 {
            println!("cargo:rustc-cfg=tongsuo820");
        }

        if version >= 0x8_03_00_00_0 {
            println!("cargo:rustc-cfg=tongsuo830");
        }

        if version >= 0x8_04_00_00_0 {
            println!("cargo:rustc-cfg=tongsuo840");
        }
    } else if let Ok(version) = env::var("DEP_OPENSSL_BABASSL_VERSION_NUMBER") {
        let version = u64::from_str_radix(&version, 16).unwrap();

        if version >= 0x8_00_00_00_0 {
            println!("cargo:rustc-cfg=babassl800");
        }

        if version >= 0x8_01_00_00_0 {
            println!("cargo:rustc-cfg=babassl810");
        }

        if version >= 0x8_02_00_00_0 {
            println!("cargo:rustc-cfg=babassl820");
        }

        if version >= 0x8_03_00_00_0 {
            println!("cargo:rustc-cfg=babassl830");
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
