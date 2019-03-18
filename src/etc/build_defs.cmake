include_directories ("${PROJECT_SOURCE_DIR}/bint"
                     "${PROJECT_SOURCE_DIR}/bint2"
                     "${PROJECT_SOURCE_DIR}/curve"
                     "${PROJECT_SOURCE_DIR}/curve2"
                     "${PROJECT_SOURCE_DIR}/util")

file (GLOB LIB_SOURCES "${PROJECT_SOURCE_DIR}/bint/*.c"
                       "${PROJECT_SOURCE_DIR}/bint2/*.c"
                       "${PROJECT_SOURCE_DIR}/curve/*.c"
                       "${PROJECT_SOURCE_DIR}/curve2/*.c"
                       "${PROJECT_SOURCE_DIR}/util/*.c")
add_library (bls_hash STATIC ${LIB_SOURCES})
find_library (LIBGMP gmp)
find_library (LIBCRYPTO crypto)

macro (add_bh_exec exec_name)
    add_executable ("${exec_name}" "${exec_name}.c")
    target_link_libraries ("${exec_name}" bls_hash "${LIBGMP}" "${LIBCRYPTO}" ${ARGN})
endmacro (add_bh_exec)

# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
