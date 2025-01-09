cmake_minimum_required(VERSION 3.6)

set(LIBRARY_TYPE STATIC)
set(PROJECT_NAME bitris)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_FLAGS "-Wall")
set(CMAKE_CXX_FLAGS_RELEASE "-O3 -flto -march=native -DNDEBUG")
#set(CMAKE_CXX_FLAGS_RELEASE "-O3 -flto -march=native -DNDEBUG")

project(${PROJECT_NAME})

file(GLOB_RECURSE SRC src/*.cpp)
list(FILTER SRC EXCLUDE REGEX ".*hello\\.cpp$")

file(GLOB_RECURSE SRC2 src/*.cpp)

include_directories(${PROJECT_BINARY_DIR})
include_directories(include)
include_directories(cxxbridge)

add_library(${PROJECT_NAME} ${LIBRARY_TYPE} ${SRC})
add_library(${PROJECT_NAME}_hello ${LIBRARY_TYPE} ${SRC2})
