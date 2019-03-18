# force use of LTO
include(CheckIPOSupported)
check_ipo_supported()
set(CMAKE_INTERPROCEDURAL_OPTIMIZATION TRUE)

# pedantic build flags
set (CMAKE_C_STANDARD 11)
set (CMAKE_EXPORT_COMPILE_COMMANDS ON)
set (CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -pedantic -pedantic-errors -Werror -Wall -Wextra -Wshadow -Wpointer-arith -Wcast-qual -Wformat=2 -Wstrict-prototypes -Wmissing-prototypes")

# check for supported compiler versions
set (IS_GNU_COMPILER ("${CMAKE_C_COMPILER_ID}" STREQUAL "GNU"))
set (IS_CLANG_COMPILER ("${CMAKE_C_COMPILER_ID}" MATCHES "[Cc][Ll][Aa][Nn][Gg]"))
set (C_VERSION_LT_7 ("${CMAKE_C_COMPILER_VERSION}" VERSION_LESS 7))
set (C_VERSION_LT_8 ("${CMAKE_C_COMPILER_VERSION}" VERSION_LESS 8))
if ((${IS_GNU_COMPILER} AND ${C_VERSION_LT_8}) OR (${IS_CLANG_COMPILER} AND ${C_VERSION_LT_7}))
    message (FATAL_ERROR "You must compile this project with g++ >= 8 or clang >= 7.")
endif ()
if (${IS_CLANG_COMPILER})
    set (CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wloop-analysis")
endif ()

# unroll loops in bint.c
if (${IS_CLANG_COMPILER})
    # clang only supports -funroll-loops, not -funroll-all-loops
    set_source_files_properties("${PROJECT_SOURCE_DIR}/bint/bint.c" PROPERTIES COMPILE_OPTIONS "-funroll-loops")
else ()
    set_source_files_properties("${PROJECT_SOURCE_DIR}/bint/bint.c" PROPERTIES COMPILE_OPTIONS "-funroll-all-loops")
endif (${IS_CLANG_COMPILER})

# add some flags for the debug and sanitizer modes
set (CMAKE_C_FLAGS_DEBUG "${CMAKE_C_FLAGS_DEBUG} -ggdb3 -Og")
set (CMAKE_C_FLAGS_DEBUGASAN "${CMAKE_C_FLAGS_DEBUG} -fsanitize=undefined -fsanitize=address")
set (CMAKE_C_FLAGS_RELASAN "${CMAKE_C_FLAGS_RELEASE} -fsanitize=undefined -fsanitize=address")

# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
