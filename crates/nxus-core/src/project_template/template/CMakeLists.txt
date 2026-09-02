add_subdirectory(lib)
add_subdirectory(app)
add_subdirectory(test)

set(nuttx_dir "$ENV{nuttx_dir}")

if(NOT TARGET docs)
add_custom_target(
    docs
    COMMAND doxide build && mkdocs build
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)
endif()

if(NOT TARGET docs-serve)
add_custom_target(
    docs-serve
    COMMAND mkdocs serve
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    USES_TERMINAL
)
endif()
