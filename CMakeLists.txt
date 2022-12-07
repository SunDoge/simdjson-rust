cmake_minimum_required(VERSION 3.14)

project(simdjson_cxx)

set(CMAKE_CXX_STANDARD 17)

set(CXXBRIDGE ${CMAKE_CURRENT_SOURCE_DIR}/target/cxxbridge)

include(FetchContent)

FetchContent_Declare(
  simdjson
  GIT_REPOSITORY https://github.com/simdjson/simdjson.git
  GIT_TAG  tags/v3.0.1
  GIT_SHALLOW TRUE)

FetchContent_MakeAvailable(simdjson)

include_directories(${CMAKE_CURRENT_SOURCE_DIR})
include_directories(${CXXBRIDGE})


add_library(simdjson_cxx SHARED src/simdjson_cxx.cc ${CXXBRIDGE}/simdjson-rust/src/bridge.rs.cc)
target_link_libraries(simdjson_cxx simdjson)

install(TARGETS simdjson_cxx)