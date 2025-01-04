cmake_minimum_required(VERSION 3.6)

set(LIBRARY_TYPE SHARED)
set(PROJECT_NAME bitris)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_FLAGS "-Wall")
set(CMAKE_CXX_FLAGS_RELEASE "-O2 -flto -march=native -DNDEBUG")
#set(CMAKE_CXX_FLAGS_RELEASE "-O3 -flto -march=native -DNDEBUG")

project(${PROJECT_NAME})

file(GLOB SRC src/*.cpp)

add_definitions(-D${PROJECT_NAME}_EXPORTS)

include_directories(${PROJECT_BINARY_DIR})
include_directories(include)

add_library(${PROJECT_NAME} ${LIBRARY_TYPE} ${SRC})
