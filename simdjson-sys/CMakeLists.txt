cmake_minimum_required(VERSION 3.15)
project(simdjson_sys LANGUAGES CXX)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

include(FetchContent)

fetchcontent_declare(
  simdjson
  GIT_REPOSITORY
  https://github.com/simdjson/simdjson.git
  GIT_TAG
  v4.6.4
  GIT_SHALLOW
  TRUE)
fetchcontent_makeavailable(simdjson)

install(TARGETS simdjson ARCHIVE DESTINATION lib)
install(DIRECTORY ${simdjson_SOURCE_DIR}/include/ DESTINATION include)
