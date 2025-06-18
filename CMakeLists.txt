# We need to include the prologue before doing anything else,
# This essentially sets up the toolchain, parses MCU/BOARD.
# The prologue needs to know the OSIRIS_SOURCE_DIR.
set(OSIRIS_SOURCE_DIR ${CMAKE_SOURCE_DIR}/osiris)
include(osiris/cmake/prologue.cmake)

cmake_minimum_required(VERSION 3.28)

project(osiris_app_template C)

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.5
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH Cargo.toml NO_STD)

set(APP_TARGET_NAME OsirisAppTemplate)

file(WRITE null.c "")
add_executable(${APP_TARGET_NAME}
    null.c
)

target_link_libraries(${APP_TARGET_NAME} PRIVATE
    osiris_app_template
)

# After we defined our app target, we need to include the osiris epilogue.
# This will set the linker script, add the necessary libraries, inject symbols etc.
include(osiris/cmake/epilogue.cmake)