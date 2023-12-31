pub fn get(openssl_version: Option<u64>, libressl_version: Option<u64>, babassl_version: Option<u64>) -> Vec<&'static str> {
    let mut cfgs = vec![];

    if let Some(libressl_version) = libressl_version {
        cfgs.push("libressl");

        if libressl_version >= 0x2_05_01_00_0 {
            cfgs.push("libressl251");
        }
        if libressl_version >= 0x2_06_01_00_0 {
            cfgs.push("libressl261");
        }
        if libressl_version >= 0x2_07_00_00_0 {
            cfgs.push("libressl270");
        }
        if libressl_version >= 0x2_07_01_00_0 {
            cfgs.push("libressl271");
        }
        if libressl_version >= 0x2_07_03_00_0 {
            cfgs.push("libressl273");
        }
        if libressl_version >= 0x2_08_00_00_0 {
            cfgs.push("libressl280");
        }
        if libressl_version >= 0x2_08_01_00_0 {
            cfgs.push("libressl281");
        }
        if libressl_version >= 0x2_09_01_00_0 {
            cfgs.push("libressl291");
        }
    } else {
        let openssl_version = openssl_version.unwrap();

        if openssl_version >= 0x1_00_01_00_0 {
            cfgs.push("ossl101");
        }
        if openssl_version >= 0x1_00_02_00_0 {
            cfgs.push("ossl102");
        }
        if openssl_version >= 0x1_00_02_06_0 {
            cfgs.push("ossl102f");
        }
        if openssl_version >= 0x1_00_02_08_0 {
            cfgs.push("ossl102h");
        }
        if openssl_version >= 0x1_01_00_00_0 {
            cfgs.push("ossl110");
        }
        if openssl_version >= 0x1_01_00_06_0 {
            cfgs.push("ossl110f");
        }
        if openssl_version >= 0x1_01_00_07_0 {
            cfgs.push("ossl110g");
        }
        if openssl_version >= 0x1_01_00_08_0 {
            cfgs.push("ossl110h");
        }
        if openssl_version >= 0x1_01_01_00_0 {
            cfgs.push("ossl111");
        }
        if openssl_version >= 0x1_01_01_02_0 {
            cfgs.push("ossl111b");
        }
        if openssl_version >= 0x1_01_01_03_0 {
            cfgs.push("ossl111c");
        }

        // FIXME: this is very ad-hoc, address this if BabaSSL moves to OpenSSL 3.0.0
        if let Some(babassl_version) = babassl_version {
            cfgs.push("babassl");

            if babassl_version >= 0x8_00_00_00_0 {
                cfgs.push("babassl800");
            }
            if babassl_version >= 0x8_01_00_00_0 {
                cfgs.push("babassl810");
            }
        }
    }

    cfgs
}
