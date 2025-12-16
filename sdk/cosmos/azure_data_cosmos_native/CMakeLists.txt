# cSpell:ignore cosmosctest CRATETYPES endforeach pkgconfig libdir includedir

cmake_minimum_required(VERSION 4.1)

project(cosmosctest C)

# CMake automatically uses this option, but we should define it.
option(BUILD_SHARED_LIBS "Build using shared libraries" ON)

include(FetchContent)
include(CTest)

# Set output directories
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/test)
set(CMAKE_INSTALL_PREFIX ${CMAKE_BINARY_DIR}/dist)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.5.2
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(
    MANIFEST_PATH ./Cargo.toml
)

# Copy header file to build/include directory
file(MAKE_DIRECTORY ${CMAKE_BINARY_DIR}/include)
configure_file(
    ${CMAKE_SOURCE_DIR}/include/azurecosmos.h
    ${CMAKE_BINARY_DIR}/include/azurecosmos.h
    COPYONLY
)

# Generate pkg-config file
set(PKG_CONFIG_NAME "azurecosmos")
set(PKG_CONFIG_DESCRIPTION "Azure Cosmos DB C API")
set(PKG_CONFIG_VERSION "0.1.0")
set(PKG_CONFIG_LIBS "-L\${libdir} -lazurecosmos")
set(PKG_CONFIG_CFLAGS "-I\${includedir}")

configure_file(
    ${CMAKE_SOURCE_DIR}/azurecosmos.pc.in
    ${CMAKE_BINARY_DIR}/azurecosmos.pc
    @ONLY
)

# Auto-discover test files
file(GLOB TEST_FILES ./c_tests/*.c)

# Build test executables
foreach(test_file ${TEST_FILES})
    get_filename_component(test_name ${test_file} NAME_WE)
    add_executable(${test_name} ${test_file})
    target_link_libraries(${test_name} PRIVATE azurecosmos)
endforeach()

# Run test discovery after build and trigger reconfiguration
add_custom_target(discover_tests ALL
    COMMAND ${CMAKE_COMMAND} 
        -DTEST_DIR=${CMAKE_RUNTIME_OUTPUT_DIRECTORY}
        -DCMAKE_CURRENT_BINARY_DIR=${CMAKE_CURRENT_BINARY_DIR}
        -P ${CMAKE_CURRENT_SOURCE_DIR}/cmake/DiscoverTests.cmake
    COMMAND ${CMAKE_COMMAND} ${CMAKE_CURRENT_SOURCE_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    DEPENDS ${TEST_FILES}
    COMMENT "Discovering all tests and reconfiguring..."
)

# Include discovered tests (will be populated after first build)
include(${CMAKE_CURRENT_BINARY_DIR}/DiscoveredTests.cmake OPTIONAL)
