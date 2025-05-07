cmake_minimum_required(VERSION 3.5)
project(QtRustIntegration)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)

find_package(Qt5 COMPONENTS Widgets REQUIRED)

# Path ke Rust .so hasil build
set(RUST_LIB_PATH "${CMAKE_SOURCE_DIR}/../rust_backend/target/release")
include_directories(${CMAKE_CURRENT_SOURCE_DIR} ${RUST_LIB_PATH})
link_directories(${RUST_LIB_PATH})

add_executable(QtRustIntegration
    main.cpp
    mainwindow.cpp
    mainwindow.h
    mainwindow.ui
)

# Link ke Qt dan Rust library (tanpa 'lib' dan tanpa '.so')
target_link_libraries(QtRustIntegration
    PRIVATE
    Qt5::Widgets
    rust_backend
)
