cmake_minimum_required (VERSION 3.9)
cmake_policy (SET CMP0054 NEW)
cmake_policy (SET CMP0069 NEW)
project (bls_hash)

include (etc/build_type.cmake)
include (etc/cflags.cmake)

include (etc/clang_format.cmake)
include (etc/clang_tidy.cmake)
include (etc/cppcheck.cmake)
include (etc/sage.cmake)

include (etc/build_defs.cmake)

add_subdirectory ("${PROJECT_SOURCE_DIR}/apps")

include (etc/tests.cmake)

# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
