cmake_minimum_required(VERSION 3.20)
project(bench-uwebsockets CXX)

find_package(unofficial-uwebsockets CONFIG REQUIRED)

add_executable(bench main.cpp)
target_link_libraries(bench PRIVATE unofficial::uwebsockets::uwebsockets)