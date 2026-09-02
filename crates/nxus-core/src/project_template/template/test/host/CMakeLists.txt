FILE(GLOB app_sources ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)
set(includes ${CMAKE_CURRENT_SOURCE_DIR}/include)

if(CONFIG___ZAPPY_PROJECT_NAME_SCREAMING___TEST_HOST)
  nuttx_add_application(
    NAME
    __ZAPPY_PROJECT_NAME_SNAKE___test_host
    INCLUDE_DIRECTORIES
      ${CMAKE_CURRENT_SOURCE_DIR}/include
    SRCS
      ${app_sources}
    STACKSIZE
      4096
    PRIORITY
      100
    # DEPENDS
    # lib1
  )
endif()
