#include <iostream>
#include <SFML/Network.hpp>
#include <string>

static bool port_is_open(const std::string& address, int port)
{
    return (sf::TcpSocket().connect(address, port) == sf::Socket::Done);
}


int main()
{
    std::string address;
    int port;
    // Get the address.
    std::cout << "Address: " << std::flush;
    std::getline(std::cin, address);
    // Get the port.
    std::cout << "Port: " << std::flush;
    std::cin >> port;
    // Scan!
    std::cout << "Scanning " << address << "...\n" << "Port " << port << " : ";
    if (port_is_open(address, port))
        std::cout << "OPEN" << std::endl;
    else
        std::cout << "CLOSED" << std::endl;
    return 0;
}

#include <iostream>
#include <string>

int main() {
  std::cout << std::string{"hi"} << std::endl;
}
