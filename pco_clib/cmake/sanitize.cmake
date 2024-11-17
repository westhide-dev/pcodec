OPTION(SANITIZE "Enable sanitizer" "none")

IF(SANITIZE STREQUAL "address")
    SET(ASAN_FLAGS "-fsanitize=address -fsanitize-address-use-after-scope")
ENDIF()

IF(SANITIZE STREQUAL "memory")
    SET(MSAN_FLAGS "-fsanitize=memory -fsanitize-memory-use-after-dtor -fsanitize-memory-track-origins -fno-optimize-sibling-calls -fPIC -fpie")
ENDIF()

IF(SANITIZE STREQUAL "thread")
    SET(TSAN_FLAGS "-fsanitize=thread")
ENDIF()

IF(NOT SANITIZE STREQUAL "none")
    SET(SANITIZE_FLAGS "-g -fno-omit-frame-pointer -DSANITIZER ${ASAN_FLAGS} ${MSAN_FLAGS} ${TSAN_FLAGS}")
    SET(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} ${SANITIZE_FLAGS}")
    SET(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${SANITIZE_FLAGS}")
ENDIF()
