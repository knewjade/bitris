extern crate cc;

fn main() {
    // 変更を検知するファイル
    if cfg!(feature="japanese") {
        use std::env;

        println!("cargo:rerun-if-changed=bitris_cpp/bitris/src/**/*.cpp");
        println!("cargo:rerun-if-changed=bitris_cpp/bitris/include/**/*.hpp");

        // 環境変数でコンパイラが指定されていない場合はgcc/g++を利用する
        env::set_var("CC", env::var("CC").unwrap_or_else(|_| "gcc".to_string()));
        env::set_var("CXX", env::var("CXX").unwrap_or_else(|_| "g++".to_string()));

        cc::Build::new()
            .cpp(true)
            .warnings(true)
            .flag("-std=c++2b")
            .flag("-Wall")
            .flag("-Wextra")
            .flag("-v")
            .flag("-O3")
            .flag("-flto")
            .flag("-march=native")
            .flag("-g")
            .flag("-pipe")
            .flag("-MMD")
            .flag("-MP")
            .files(
                std::fs::read_dir("bitris_cpp/bitris/src")
                    .unwrap()
                    .flat_map(|entry| {
                        let entry = entry.unwrap();
                        let path = entry.path();
                        if path.extension().map_or(false, |ext| ext == "cpp") {
                            Some(path)
                        } else {
                            None
                        }
                    })
            )
            .include("bitris_cpp/bitris/include")
            .cpp_link_stdlib("stdc++")
            .compile("libbitris_cpp.a");
    }
}
