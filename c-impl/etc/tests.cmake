enable_testing()

# these tests just check that the resulting point is on the curve

add_test(NAME t_simple_hash_to_g1 COMMAND "${PROJECT_SOURCE_DIR}/test/run_simple_test.sh" "${CMAKE_BINARY_DIR}/apps/hash_to_g1")
add_test(NAME t_simple_hash_to_g2 COMMAND "${PROJECT_SOURCE_DIR}/test/run_simple_test.sh" "${CMAKE_BINARY_DIR}/apps/hash_to_g2")

# these tests require sage, to test end-to-end correctness
if (DEFINED SAGEMATH)
    add_test(NAME t_hash_to_g1 COMMAND "${PROJECT_SOURCE_DIR}/test/run_test.sh" "${CMAKE_BINARY_DIR}/apps/hash_to_g1" "${PROJECT_SOURCE_DIR}/test/test.sage" u2)
    add_test(NAME t_hash_to_g2 COMMAND "${PROJECT_SOURCE_DIR}/test/run_test.sh" "${CMAKE_BINARY_DIR}/apps/hash_to_g2" "${PROJECT_SOURCE_DIR}/test/g2_test.sage" u2 32)
endif (DEFINED SAGEMATH)

# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
