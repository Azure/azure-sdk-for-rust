# Test discovery script for CMake.
#
# Runs each test executable with `--discover` to enumerate the tests inside
# it, then generates a `DiscoveredTests.cmake` with one `add_test(...)` entry
# per discovered test. The parent CMakeLists.txt `include()`s the generated
# file (OPTIONAL on the first configure pass; populated after the first build).
#
# cSpell:ignore STREQUAL endforeach azurecosmosdriver

file(GLOB test_executables "${TEST_DIR}/*")
set(test_file "${CMAKE_CURRENT_BINARY_DIR}/DiscoveredTests.cmake")

file(WRITE ${test_file} "# Auto-generated test file - do not edit\n\n")

# Copy the cdylib next to the test executables on Windows so the loader can
# find it without LD_LIBRARY_PATH-equivalent gymnastics.
if(WIN32)
    file(COPY "${LIBRARY_DIR}/azurecosmosdriver.dll"
         DESTINATION "${TEST_DIR}")
endif()

foreach(test_exe ${test_executables})
    # Skip if not an executable.
    if(WIN32)
        get_filename_component(ext ${test_exe} EXT)
        if(NOT ext STREQUAL ".exe")
            continue()
        endif()
    endif()
    if(NOT IS_DIRECTORY ${test_exe})
        message(STATUS "Discovering tests in: ${test_exe}")
        get_filename_component(suite_name ${test_exe} NAME_WE)

        execute_process(
            COMMAND ${test_exe} --discover
            OUTPUT_VARIABLE test_names
            OUTPUT_STRIP_TRAILING_WHITESPACE
            RESULT_VARIABLE result
        )

        if(result EQUAL 0 AND test_names)
            string(REPLACE "\n" ";" test_list "${test_names}")
            foreach(test_name ${test_list})
                if(NOT test_name STREQUAL "")
                    file(APPEND ${test_file}
                        "add_test(NAME \"${suite_name}::${test_name}\" COMMAND \"${test_exe}\" \"${test_name}\")\n")
                endif()
            endforeach()
        endif()
    else()
        message(WARNING "Skipping non-executable file: ${test_exe}")
    endif()
endforeach()

message(STATUS "Test discovery complete. Tests written to ${test_file}")
