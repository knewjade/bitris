#include <gtest/gtest.h>

#include "rows.hpp"

namespace core {
    class BitsTest : public ::testing::Test {
    };

    TEST_F(BitsTest, most_significant_index_uint8_t) {
        using T = uint8_t;
        EXPECT_EQ(bits<T>::most_significant_index(0), -1);
        for (int index = 0; index < 8; ++index) {
            EXPECT_EQ(bits<T>::most_significant_index(1U << index), index);
        }
    }

    TEST_F(BitsTest, most_significant_index_uint16_t) {
        using T = uint16_t;
        EXPECT_EQ(bits<T>::most_significant_index(0), -1);
        for (int index = 0; index < 16; ++index) {
            EXPECT_EQ(bits<T>::most_significant_index(1U << index), index);
        }
    }

    TEST_F(BitsTest, most_significant_index_uint32_t) {
        using T = uint32_t;
        EXPECT_EQ(bits<T>::most_significant_index(0), -1);
        for (int index = 0; index < 32; ++index) {
            EXPECT_EQ(bits<T>::most_significant_index(1UL << index), index);
        }
    }

    TEST_F(BitsTest, most_significant_index_uint64_t) {
        using T = uint64_t;
        EXPECT_EQ(bits<T>::most_significant_index(0), -1);
        for (int index = 0; index < 64; ++index) {
            EXPECT_EQ(bits<T>::most_significant_index(1ULL << index), index);
        }
    }

    TEST_F(BitsTest, used_rows) {
        using T = uint8_t;
        // empty
        {
            constexpr std::array<uint8_t, 10> board = {};
            EXPECT_EQ(bits<T>::used_rows(board), 0);
        }
        // 1,4行目にブロック
        {
            constexpr std::array<uint8_t, 10> board = {
                0b0001, 0, 0, 0, 0,
                0, 0, 0b1000, 0, 0,
            };
            EXPECT_EQ(bits<T>::used_rows(board), 0b1001);
        }
    }

    TEST_F(BitsTest, full) {
        EXPECT_EQ(bits<uint8_t>::full, 0xff);
        EXPECT_EQ(bits<uint16_t>::full, 0xffff);
        EXPECT_EQ(bits<uint32_t>::full, 0xffffffff);
        EXPECT_EQ(bits<uint64_t>::full, 0xffffffffffffffff);
    }

    TEST_F(BitsTest, bit_size) {
        EXPECT_EQ(bits<uint8_t>::bit_size, 8);
        EXPECT_EQ(bits<uint16_t>::bit_size, 16);
        EXPECT_EQ(bits<uint32_t>::bit_size, 32);
        EXPECT_EQ(bits<uint64_t>::bit_size, 64);
    }
}
