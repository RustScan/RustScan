project(portscanner)

find_package(Boost REQUIRED COMPONENTS system)
find_package(Threads REQUIRED)

add_executable(portscanner scan.cpp)
target_link_libraries(portscanner Boost::boost Boost::system Threads::Threads)
