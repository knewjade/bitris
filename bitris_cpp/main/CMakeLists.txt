cmake_minimum_required(VERSION 3.6)

set(PROJECT_NAME main)
set(SRC_PROJECT_NAME bitris)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_FLAGS "-Wall")
set(CMAKE_CXX_FLAGS_RELEASE "-O2 -flto -march=native -DNDEBUG")
#set(CMAKE_CXX_FLAGS_RELEASE "-O3 -flto -march=native -DNDEBUG")

project(${PROJECT_NAME})

file(GLOB SRC ${CMAKE_CURRENT_LIST_DIR}/main.cpp)

include_directories(${${SRC_PROJECT_NAME}_SOURCE_DIR}/include)

add_executable(${PROJECT_NAME} ${SRC})

target_link_libraries(${PROJECT_NAME} ${SRC_PROJECT_NAME})