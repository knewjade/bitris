#include <iostream>
#include <hello.hpp>

int main() {
    const auto hello1 = hello(1);
    std::cout << "Hello, C++!" << hello1 << std::endl;
    return 0;
}
