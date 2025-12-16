# Test discovery script for CMake
# This script runs each test executable with --discover to get the list of tests
# and generates a CTestTestfile.cmake with all discovered tests

file(GLOB test_executables "${TEST_DIR}/*")
set(test_file "${CMAKE_CURRENT_BINARY_DIR}/DiscoveredTests.cmake")

file(WRITE ${test_file} "# Auto-generated test file - do not edit\n\n")

foreach(test_exe ${test_executables})
    # Skip if not an executable
    if(NOT IS_DIRECTORY ${test_exe})
        get_filename_component(suite_name ${test_exe} NAME_WE)
        
        execute_process(
            COMMAND ${test_exe} --discover
            OUTPUT_VARIABLE test_names
            OUTPUT_STRIP_TRAILING_WHITESPACE
            RESULT_VARIABLE result
            ERROR_QUIET
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
    endif()
endforeach()

message(STATUS "Test discovery complete. Tests written to ${test_file}")
