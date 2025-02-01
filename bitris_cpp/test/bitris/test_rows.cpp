 #include <gtest/gtest.h>

 #include "rows.hpp"

 namespace core {
     class RowsTest : public ::testing::Test {
     };

     TEST_F(RowsTest, top_y) {
         // empty
         {
             constexpr std::array<uint8_t, 10> board = {};
             EXPECT_EQ(rows::top_y(board, 6), -1);
         }
         // 1行目にブロック
         {
             constexpr std::array<uint8_t, 10> board = {
                 1,
             };
             EXPECT_EQ(rows::top_y(board, 6), 0);
         }
         // 7行目(spawn_yと同じ)にブロック
         {
             constexpr std::array<uint8_t, 10> board = {
                 0b01000000,
             };
             EXPECT_EQ(rows::top_y(board, 6), 6);
         }
         // 8行目(spawn_yの上)にブロック
         {
             constexpr std::array<uint8_t, 10> board = {
                 0b10000000,
             };
             EXPECT_EQ(rows::top_y(board, 6), -1);
         }
     }
 }
