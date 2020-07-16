project(portscanner)

find_package(Boost REQUIRED COMPONENTS system)

add_executable(portscanner your_file_name_here.cpp)
target_link_libraries(portscanner Boost::boost Boost::system)
