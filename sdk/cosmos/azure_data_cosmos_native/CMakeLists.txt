# cSpell:ignore cosmosctest CRATETYPES endforeach

project(cosmosctest C)
cmake_minimum_required(VERSION 4.1)

# CMake automatically uses this option, but we should define it.
option(BUILD_SHARED_LIBS "Build using shared libraries" ON)

include(FetchContent)
include(CTest)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.5.2
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(
    MANIFEST_PATH ./Cargo.toml
    CRATETYPES staticlib cdylib
)

set(TEST_FILES
    ./c_tests/version.c)

foreach(test_file ${TEST_FILES})
    get_filename_component(test_name ${test_file} NAME_WE)
    add_executable(${test_name} ${test_file})
    target_link_libraries(${test_name} PRIVATE cosmosclient)
    add_test(${test_name} ${test_name})
endforeach()

