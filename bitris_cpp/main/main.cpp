#include <iostream>
#include <hello.hpp>
#include <board.hpp>

int main() {
    Board64 board = Board64::blank();
    board.set_at(2, 1);
    std::cout << board.to_string() << std::endl;

    const int at = set_at(1);
    std::cout << at << std::endl;
    return 0;
}
