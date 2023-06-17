cmake_minimum_required(VERSION 3.21)

option(HAVE_LOGGER "have logger" ON)

project(ngimg)

add_executable(${PROJECT_NAME} main.c)

#[[
target_sources(${PROJECT_NAME} PRIVATE
    deps/imgui/imgui.cpp
    deps/imgui/imgui_draw.cpp
    deps/imgui/imgui_demo.cpp
    deps/imgui/imgui_tables.cpp
    deps/imgui/imgui_widgets.cpp
)
# target_include_directories(${PROJECT_NAME} PRIVATE )
find_package(OpenGL REQUIRED)

target_sources(${PROJECT_NAME} PRIVATE deps/imgui/backends/imgui_impl_opengl3.cpp deps/imgui/backends/imgui_impl_win32.cpp)
target_include_directories(${PROJECT_NAME} PRIVATE deps/imgui deps/imgui/backends ${OPENGL_INCLUDE_DIRS})
target_link_libraries(${PROJECT_NAME} PRIVATE ${OPENGL_LIBRARIES})
]]

# add_subdirectory(deps/SDL)

# target_link_libraries(${PROJECT_NAME} PRIVATE SDL2::SDL2)

if (HAVE_LOGGER)
    target_compile_definitions(${PROJECT_NAME} PRIVATE HAVE_LOGGER)
    target_sources(${PROJECT_NAME} PRIVATE nglog.c)
endif()
