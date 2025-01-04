cmake_minimum_required(VERSION 3.6)

# コンパイラー
set(CMAKE_C_COMPILER gcc)
set(CMAKE_CXX_COMPILER g++)

# プロジェクト名
project(bitris_cpp)

# 出力先
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# サブディレクトリの登録
#add_subdirectory(googletest)
add_subdirectory(bitris)
add_subdirectory(main)

# ターゲットの設定
set_target_properties(bitris PROPERTIES FOLDER library)
#set_target_properties(test PROPERTIES FOLDER executable)
set_target_properties(main PROPERTIES FOLDER executable)
