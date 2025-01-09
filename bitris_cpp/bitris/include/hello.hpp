#pragma once

#include <vector>
#include <memory>

#include "pieces.hpp"
#include "search.hpp"

#include "bitris/src/myffi.rs.h"

struct MultiBuf;

inline int32_t set_at(const int32_t y) {
    return s::set_at_constexpr(y);
}

inline constexpr std::array<uint16_t, 10> search(
    std::array<uint16_t, 10> boards,
    uint8_t spawn_piece,
    uint8_t spawn_orientation,
    uint8_t spawn_bx,
    uint8_t spawn_by
) {
    return s::search(boards, spawn_piece, spawn_orientation, spawn_bx, spawn_by);
}

inline void search3(MultiBuf &multi_buf) {
    next_chunk(multi_buf);
    next_chunk(multi_buf);
    next_chunk(multi_buf);
}

// class BlobstoreClient {
// public:
//     BlobstoreClient();
//
//     explicit BlobstoreClient(const int i): value(i) {}
//
//     int value;
// };
//
// std::unique_ptr<BlobstoreClient> new_blobstore_client();
