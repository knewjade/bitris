fn main() {
    // 変更を検知するファイル
    if cfg!(feature="japanese") {
        use std::env;

        println!("cargo:rerun-if-changed=/bitris_cpp/bitris/src/**/*.cpp");
        println!("cargo:rerun-if-changed=/bitris_cpp/bitris/include/**/*.hpp");
        println!("cargo:rerun-if-changed=/build.rs");
        println!("cargo:rerun-if-changed=/src/myffi.rs");

        // 環境変数でコンパイラが指定されていない場合はgcc/g++を利用する
        env::set_var("CC", env::var("CC").unwrap_or_else(|_| "gcc".to_string()));
        env::set_var("CXX", env::var("CXX").unwrap_or_else(|_| "g++".to_string()));

        cxx_build::bridge("src/myffi.rs")
            .cpp(true)
            // .warnings(true)
            .flag_if_supported("-std=c++2b")
            .flag_if_supported("-Wall")
            .flag_if_supported("-Wextra")
            .flag_if_supported("-v")
            .flag_if_supported("-O3")
            .flag_if_supported("-flto")
            .flag_if_supported("-march=native")
            .flag_if_supported("-g")
            .flag_if_supported("-pipe")
            .flag_if_supported("-MMD")
            .flag_if_supported("-MP")
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
            // .cpp_link_stdlib("stdc++")
            .compile("bitris_cpp");
    }
}
