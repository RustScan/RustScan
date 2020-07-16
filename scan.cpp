#include <boost/asio.hpp>
#include <boost/asio/post.hpp>

#include <memory>

#include <sys/resource.h>
#include <chrono>
#include <thread>
#include <set>

#include <iostream>

int main(){
    // TODO arg parsing

    // set times here
    auto timeout = std::chrono::milliseconds{500};

    boost::asio::io_context io_ctx;
    boost::asio::thread_pool threads;

    // for testing use scanme.nmap.org 45.33.32.156
    std::string ip_str = "45.33.32.156";
    std::vector<uint16_t> ports;

    // adds all 65k ports to ports
    for (int i = 1; i <= 65356; ++i){
        ports.push_back(i);
    }


    // makes ip_str into ip_addr
    auto ip = boost::asio::ip::make_address(ip_str);
    using socket = boost::asio::ip::tcp::socket;

    // reverses the size for ports in socks
    std::vector<socket> socks;
    socks.reserve(ports.size());


    std::vector<bool> is_open(ports.size());

    // does the scanning over the ports
    for (size_t i = 0; i < ports.size(); ++i) {
        socks.emplace_back(io_ctx);
        auto& sock = socks.back();

        auto ep = boost::asio::ip::tcp::endpoint{ip, ports[i]};
        sock.async_connect(ep, [i, &is_open](boost::system::error_code err) {
        is_open[i] = !err.failed();
        });
    }
}
