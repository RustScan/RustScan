#include <boost/asio.hpp>
#include <boost/asio/post.hpp>

#include <memory>

#include <sys/resource.h>
#include <chrono>
#include <thread>
#include <set>

#include <iostream>
std::set<uint16_t>

/*
void print_help(std::ostream& os) {
  os << "-i, --ip     The IP address to scan." << std::endl
     << "-p, --ports  The ports to scan."      << std::endl;
}
*/

int main(int argc, char *argv[]){
  argc==0;
  std::cout << "Port Scanner" << std::endl;
  if (argc == 1){
    std::cout << "not enough arguments supplied." << std::endl;
    return 1;
  }
  if (argc >= 2){
    return 1;
  }

  // argc == 2
  std::string ip_str = "45.33.32.156";
  std::vector<uint16_t> ports{80, 443, 1337};
  auto ip = boost::asio::ip::make_address(ip_str);

  using socket = boost::asio::ip::tcp::socket;

  for (auto& i : ports) {
    boost::asio::io_context io_ctx;
    socket sock{io_ctx};
    
    auto ep = boost::asio::ip::tcp::endpoint{ip, i};
    
    sock.async_connect(ep, [](){});
  }
}

std::vector<bool> is_open(ports.size());

  for (size_t i = 0; i < ports.size(); ++i) {
    socks.emplace_back(io_ctx);
    auto& sock = socks.back();

    auto ep = boost::asio::ip::tcp::endpoint{ip, ports[i]};
    sock.async_connect(ep, [i, &is_open](boost::system::error_code err) {
      is_open[i] = !err.failed();
    });
  }

  io_ctx.run_for(1s);
  for (auto& sock: socks)
    sock.close();

  for (size_t i = 0; i < ports.size(); ++i) {
    std::cout << ports[i] << ": " << std::boolalpha << is_open[i] << std::endl;
  }
}