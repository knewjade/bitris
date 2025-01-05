#include "board.hpp"

//void Board64::unset_at(const Location& location) {
//    cols[location.x] &= ~(1ULL << location.y);
//}

//bool Board64::is_free_at(const Location& location) const {
//    return !is_occupied_at(location);
//}

//bool Board64::is_empty() const {
//    for (int x = 0; x < WIDTH; ++x) {
//        if (cols[x] != 0) {
//          return false;
//        }
//    }
//    return true;
//}

//void Board64::invert() {
//    for (int x = 0; x < WIDTH; ++x) {
//        cols[x] = ~cols[x];
//    }
//}

//bool Board64::overlaps(const Board64& other) const {
//    for (int x = 0; x < WIDTH; ++x) {
//        if (cols[x] & other.cols[x]) {
//            return true;
//        }
//    }
//    return false;
//}

//void Board64::merge(const Board64& other) {
//    for (int x = 0; x < WIDTH; ++x) {
//        cols[x] |= other.cols[x];
//    }
//}

//void Board64::remove_all(const Board64& other) {
//    for (int x = 0; x < WIDTH; ++x) {
//        cols[x] &= ~other.cols[x];
//    }
//}

std::string Board64::to_string() const {
    std::string str;
    for (int y = HEIGHT - 1; y >= 0; --y) {
        for (int x = 0; x < WIDTH; ++x) {
            str += (is_occupied_at(Location(x, y)) ? '#' : '.');
        }
        str += '\n';
    }
    return str;
}
