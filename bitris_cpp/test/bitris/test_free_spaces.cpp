 #include <array>
 #include <gtest/gtest.h>

 #include "free_spaces.hpp"

 namespace core {
     class FreeSpacesTest : public ::testing::Test {
     };

     using Data = uint8_t;

     template<typename Data>
     std::array<Data, 10> to(typename data<Data>::type board) {
         alignas(32) std::array<Data, 10> array{};
         board.copy_to(&array[0], stdx::vector_aligned);
         return array;
     }

     template<typename Data>
     typename data<Data>::type block_at(typename data<Data>::type board, const size_t x, const size_t y) {
         board[x] = board[x] & ~(1U << y);
         return board;
     }

     template<typename Data>
     typename data<Data>::type free_at(typename data<Data>::type board, const size_t x, const size_t y) {
         board[x] = board[x] | (1U << y);
         return board;
     }

     template<typename Data>
     bool is_free_at(typename data<Data>::type board, const size_t x, const size_t y) {
         return board[x] & (1U << y);
     }

     template<typename Data>
     size_t popcount(typename data<Data>::type board) {
         size_t count = 0;
         for (int x = 0; x < 10; ++x) {
             count += std::bitset<bits<Data>::bit_size>(board[x]).count();
         }
         return count;
     }

     TEST_F(FreeSpacesTest, o_empty) {
         constexpr auto shape = Shape::O;
         const auto board = data<Data>::make_square<bits<Data>::full>();
         const auto all_free_spaces = free_spaces<Data, shape>::get(board);

         for (auto free_space: all_free_spaces) {
             data<Data>::show(free_space);
         }

         EXPECT_EQ(all_free_spaces.size(), 1);
         EXPECT_EQ(to<Data>(all_free_spaces[0]), (std::array<Data, 10>{
                       0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00
                       }));
     }

     TEST_F(FreeSpacesTest, o_north) {
         constexpr auto shape = Shape::O;
         constexpr auto index = static_cast<size_t>(Orientation::North);
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 2, 3);
         board = free_at<Data>(board, 3, 2);
         board = free_at<Data>(board, 3, 3);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);

         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[index], 2, 2));
         EXPECT_EQ(popcount<Data>(all_free_spaces[index]), 1);
     }

     TEST_F(FreeSpacesTest, t_empty) {
         constexpr auto shape = Shape::T;
         const auto board = data<Data>::make_square<bits<Data>::full>();
         const auto all_free_spaces = free_spaces<Data, shape>::get(board);

         for (auto free_space: all_free_spaces) {
             data<Data>::show(free_space);
         }

         EXPECT_EQ(all_free_spaces.size(), 4);
         EXPECT_EQ(to<Data>(all_free_spaces[0]), (std::array<Data, 10>{
                       0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00
                       }));
         EXPECT_EQ(to<Data>(all_free_spaces[1]), (std::array<Data, 10>{
                       0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0x00
                       }));
         EXPECT_EQ(to<Data>(all_free_spaces[2]), (std::array<Data, 10>{
                       0x00, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0x00
                       }));
         EXPECT_EQ(to<Data>(all_free_spaces[3]), (std::array<Data, 10>{
                       0x00, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE
                       }));
     }

     TEST_F(FreeSpacesTest, t_north) {
         constexpr auto shape = Shape::T;
         constexpr auto index = static_cast<size_t>(Orientation::North);
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 1, 2);
         board = free_at<Data>(board, 3, 2);
         board = free_at<Data>(board, 2, 3);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);

         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[index], 2, 2));
         EXPECT_EQ(popcount<Data>(all_free_spaces[index]), 1);
         for (int i = 0; i < 4; ++i) {
             if (i != index) {
                 EXPECT_EQ(popcount<Data>(all_free_spaces[i]), 0);
             }
         }
     }

     TEST_F(FreeSpacesTest, t_east) {
         constexpr auto shape = Shape::T;
         constexpr auto index = static_cast<size_t>(Orientation::East);
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 2, 1);
         board = free_at<Data>(board, 2, 3);
         board = free_at<Data>(board, 3, 2);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);

         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[index], 2, 2));
         EXPECT_EQ(popcount<Data>(all_free_spaces[index]), 1);
         for (int i = 0; i < 4; ++i) {
             if (i != index) {
                 EXPECT_EQ(popcount<Data>(all_free_spaces[i]), 0);
             }
         }
     }

     TEST_F(FreeSpacesTest, t_south) {
         constexpr auto shape = Shape::T;
         constexpr auto index = static_cast<size_t>(Orientation::South);
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 1, 2);
         board = free_at<Data>(board, 3, 2);
         board = free_at<Data>(board, 2, 1);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);

         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[index], 2, 2));
         EXPECT_EQ(popcount<Data>(all_free_spaces[index]), 1);
         for (int i = 0; i < 4; ++i) {
             if (i != index) {
                 EXPECT_EQ(popcount<Data>(all_free_spaces[i]), 0);
             }
         }
     }

     TEST_F(FreeSpacesTest, t_wast) {
         constexpr auto shape = Shape::T;
         constexpr auto index = static_cast<size_t>(Orientation::West);
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 2, 1);
         board = free_at<Data>(board, 2, 3);
         board = free_at<Data>(board, 1, 2);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);

         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[index], 2, 2));
         EXPECT_EQ(popcount<Data>(all_free_spaces[index]), 1);
         for (int i = 0; i < 4; ++i) {
             if (i != index) {
                 EXPECT_EQ(popcount<Data>(all_free_spaces[i]), 0);
             }
         }
     }

     TEST_F(FreeSpacesTest, l_empty) {
         const auto board = data<Data>::make_square<bits<Data>::full>();
         const auto all_free_spaces = free_spaces<Data, Shape::L>::get(board);

         for (auto free_space: all_free_spaces) {
             data<Data>::show(free_space);
         }

         EXPECT_EQ(all_free_spaces.size(), 4);
         EXPECT_EQ(to<Data>(all_free_spaces[0]), (std::array<Data, 10>{
                       0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00
                       }));
         EXPECT_EQ(to<Data>(all_free_spaces[1]), (std::array<Data, 10>{
                       0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0x00
                       }));
         EXPECT_EQ(to<Data>(all_free_spaces[2]), (std::array<Data, 10>{
                       0x00, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0x00
                       }));
         EXPECT_EQ(to<Data>(all_free_spaces[3]), (std::array<Data, 10>{
                       0x00, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE
                       }));
     }

     TEST_F(FreeSpacesTest, l_north) {
         constexpr auto shape = Shape::L;
         constexpr auto index = static_cast<size_t>(Orientation::North);
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 1, 2);
         board = free_at<Data>(board, 3, 2);
         board = free_at<Data>(board, 3, 3);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[index], 2, 2));
         EXPECT_EQ(popcount<Data>(all_free_spaces[index]), 1);
         for (int i = 0; i < 4; ++i) {
             if (i != index) {
                 EXPECT_EQ(popcount<Data>(all_free_spaces[i]), 0);
             }
         }
     }

     TEST_F(FreeSpacesTest, l_east) {
         constexpr auto shape = Shape::L;
         constexpr auto index = static_cast<size_t>(Orientation::East);
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 2, 1);
         board = free_at<Data>(board, 2, 3);
         board = free_at<Data>(board, 3, 1);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[index], 2, 2));
         EXPECT_EQ(popcount<Data>(all_free_spaces[index]), 1);
         for (int i = 0; i < 4; ++i) {
             if (i != index) {
                 EXPECT_EQ(popcount<Data>(all_free_spaces[i]), 0);
             }
         }
     }

     TEST_F(FreeSpacesTest, l_south) {
         constexpr auto shape = Shape::L;
         constexpr auto index = static_cast<size_t>(Orientation::South);
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 1, 2);
         board = free_at<Data>(board, 3, 2);
         board = free_at<Data>(board, 1, 1);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[index], 2, 2));
         EXPECT_EQ(popcount<Data>(all_free_spaces[index]), 1);
         for (int i = 0; i < 4; ++i) {
             if (i != index) {
                 EXPECT_EQ(popcount<Data>(all_free_spaces[i]), 0);
             }
         }
     }

     TEST_F(FreeSpacesTest, l_west) {
         constexpr auto shape = Shape::L;
         constexpr auto index = static_cast<size_t>(Orientation::West);
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 2, 1);
         board = free_at<Data>(board, 2, 3);
         board = free_at<Data>(board, 1, 3);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[index], 2, 2));
         EXPECT_EQ(popcount<Data>(all_free_spaces[index]), 1);
         for (int i = 0; i < 4; ++i) {
             if (i != index) {
                 EXPECT_EQ(popcount<Data>(all_free_spaces[i]), 0);
             }
         }
     }

     TEST_F(FreeSpacesTest, j_empty) {
         const auto board = data<Data>::make_square<bits<Data>::full>();
         const auto all_free_spaces = free_spaces<Data, Shape::J>::get(board);

         for (auto free_space: all_free_spaces) {
             data<Data>::show(free_space);
         }

         EXPECT_EQ(all_free_spaces.size(), 4);
         EXPECT_EQ(to<Data>(all_free_spaces[0]), (std::array<Data, 10>{
                       0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00
                       }));
         EXPECT_EQ(to<Data>(all_free_spaces[1]), (std::array<Data, 10>{
                       0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0x00
                       }));
         EXPECT_EQ(to<Data>(all_free_spaces[2]), (std::array<Data, 10>{
                       0x00, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0x00
                       }));
         EXPECT_EQ(to<Data>(all_free_spaces[3]), (std::array<Data, 10>{
                       0x00, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE
                       }));
     }

     TEST_F(FreeSpacesTest, j_north) {
         constexpr auto shape = Shape::J;
         constexpr auto index = static_cast<size_t>(Orientation::North);
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 1, 2);
         board = free_at<Data>(board, 3, 2);
         board = free_at<Data>(board, 1, 3);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[index], 2, 2));
         EXPECT_EQ(popcount<Data>(all_free_spaces[index]), 1);
         for (int i = 0; i < 4; ++i) {
             if (i != index) {
                 EXPECT_EQ(popcount<Data>(all_free_spaces[i]), 0);
             }
         }
     }

     TEST_F(FreeSpacesTest, j_east) {
         constexpr auto shape = Shape::J;
         constexpr auto index = static_cast<size_t>(Orientation::East);
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 2, 1);
         board = free_at<Data>(board, 2, 3);
         board = free_at<Data>(board, 3, 3);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[index], 2, 2));
         EXPECT_EQ(popcount<Data>(all_free_spaces[index]), 1);
         for (int i = 0; i < 4; ++i) {
             if (i != index) {
                 EXPECT_EQ(popcount<Data>(all_free_spaces[i]), 0);
             }
         }
     }

     TEST_F(FreeSpacesTest, j_south) {
         constexpr auto shape = Shape::J;
         constexpr auto index = static_cast<size_t>(Orientation::South);
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 1, 2);
         board = free_at<Data>(board, 3, 2);
         board = free_at<Data>(board, 3, 1);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[index], 2, 2));
         EXPECT_EQ(popcount<Data>(all_free_spaces[index]), 1);
         for (int i = 0; i < 4; ++i) {
             if (i != index) {
                 EXPECT_EQ(popcount<Data>(all_free_spaces[i]), 0);
             }
         }
     }

     TEST_F(FreeSpacesTest, j_west) {
         constexpr auto shape = Shape::J;
         constexpr auto index = static_cast<size_t>(Orientation::West);
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 2, 3);
         board = free_at<Data>(board, 2, 1);
         board = free_at<Data>(board, 1, 1);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[index], 2, 2));
         EXPECT_EQ(popcount<Data>(all_free_spaces[index]), 1);
         for (int i = 0; i < 4; ++i) {
             if (i != index) {
                 EXPECT_EQ(popcount<Data>(all_free_spaces[i]), 0);
             }
         }
     }

     TEST_F(FreeSpacesTest, i_empty) {
         const auto board = data<Data>::make_square<bits<Data>::full>();
         const auto all_free_spaces = free_spaces<Data, Shape::I>::get(board);

         for (auto free_space: all_free_spaces) {
             data<Data>::show(free_space);
         }

         EXPECT_EQ(all_free_spaces.size(), 4);
         EXPECT_EQ(to<Data>(all_free_spaces[0]), (std::array<Data, 10>{
                       0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00
                       }));
         EXPECT_EQ(to<Data>(all_free_spaces[1]), (std::array<Data, 10>{
                       0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC
                       }));
         EXPECT_EQ(to<Data>(all_free_spaces[2]), (std::array<Data, 10>{
                       0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00
                       }));
         EXPECT_EQ(to<Data>(all_free_spaces[3]), (std::array<Data, 10>{
                       0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE
                       }));
     }

     TEST_F(FreeSpacesTest, i_north_south) {
         constexpr auto shape = Shape::I;
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 1, 2);
         board = free_at<Data>(board, 3, 2);
         board = free_at<Data>(board, 4, 2);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[static_cast<size_t>(Orientation::North)], 2, 2));
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[static_cast<size_t>(Orientation::South)], 3, 2));

         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::North)]), 1);
         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::South)]), 1);

         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::East)]), 0);
         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::West)]), 0);
     }

     TEST_F(FreeSpacesTest, i_east_west) {
         constexpr auto shape = Shape::I;
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 2, 3);
         board = free_at<Data>(board, 2, 1);
         board = free_at<Data>(board, 2, 0);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[static_cast<size_t>(Orientation::East)], 2, 2));
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[static_cast<size_t>(Orientation::West)], 2, 1));

         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::East)]), 1);
         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::West)]), 1);

         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::North)]), 0);
         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::South)]), 0);
     }

     TEST_F(FreeSpacesTest, s_empty) {
         const auto board = data<Data>::make_square<bits<Data>::full>();
         const auto all_free_spaces = free_spaces<Data, Shape::S>::get(board);

         for (auto free_space: all_free_spaces) {
             data<Data>::show(free_space);
         }

         EXPECT_EQ(all_free_spaces.size(), 4);
         EXPECT_EQ(to<Data>(all_free_spaces[0]), (std::array<Data, 10>{
                       0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00
                       }));
         EXPECT_EQ(to<Data>(all_free_spaces[1]), (std::array<Data, 10>{
                       0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0x00
                       }));
         EXPECT_EQ(to<Data>(all_free_spaces[2]), (std::array<Data, 10>{
                       0x00, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0x00
                       }));
         EXPECT_EQ(to<Data>(all_free_spaces[3]), (std::array<Data, 10>{
                       0x00, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE
                       }));
     }

     TEST_F(FreeSpacesTest, s_north_south) {
         constexpr auto shape = Shape::S;
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 2, 3);
         board = free_at<Data>(board, 3, 3);
         board = free_at<Data>(board, 1, 2);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[static_cast<size_t>(Orientation::North)], 2, 2));
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[static_cast<size_t>(Orientation::South)], 2, 3));

         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::North)]), 1);
         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::South)]), 1);

         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::East)]), 0);
         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::West)]), 0);
     }

     TEST_F(FreeSpacesTest, s_east_west) {
         constexpr auto shape = Shape::S;
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 2, 3);
         board = free_at<Data>(board, 3, 2);
         board = free_at<Data>(board, 3, 1);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[static_cast<size_t>(Orientation::East)], 2, 2));
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[static_cast<size_t>(Orientation::West)], 3, 2));

         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::East)]), 1);
         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::West)]), 1);

         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::North)]), 0);
         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::South)]), 0);
     }

     TEST_F(FreeSpacesTest, z_empty) {
         const auto board = data<Data>::make_square<bits<Data>::full>();
         const auto all_free_spaces = free_spaces<Data, Shape::Z>::get(board);

         for (auto free_space: all_free_spaces) {
             data<Data>::show(free_space);
         }

         EXPECT_EQ(all_free_spaces.size(), 4);
         EXPECT_EQ(to<Data>(all_free_spaces[0]), (std::array<Data, 10>{
                       0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00
                       }));
         EXPECT_EQ(to<Data>(all_free_spaces[1]), (std::array<Data, 10>{
                       0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0x00
                       }));
         EXPECT_EQ(to<Data>(all_free_spaces[2]), (std::array<Data, 10>{
                       0x00, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0x00
                       }));
         EXPECT_EQ(to<Data>(all_free_spaces[3]), (std::array<Data, 10>{
                       0x00, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE
                       }));
     }

     TEST_F(FreeSpacesTest, z_north_south) {
         constexpr auto shape = Shape::Z;
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 2, 3);
         board = free_at<Data>(board, 1, 3);
         board = free_at<Data>(board, 3, 2);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[static_cast<size_t>(Orientation::North)], 2, 2));
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[static_cast<size_t>(Orientation::South)], 2, 3));

         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::North)]), 1);
         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::South)]), 1);

         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::East)]), 0);
         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::West)]), 0);
     }

     TEST_F(FreeSpacesTest, z_east_west) {
         constexpr auto shape = Shape::Z;
         auto board = data<Data>::make_zero();
         board = free_at<Data>(board, 2, 2);
         board = free_at<Data>(board, 3, 2);
         board = free_at<Data>(board, 3, 3);
         board = free_at<Data>(board, 2, 1);
         data<Data>::show(board);

         const auto all_free_spaces = free_spaces<Data, shape>::get(board);
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[static_cast<size_t>(Orientation::East)], 2, 2));
         EXPECT_TRUE(is_free_at<Data>(all_free_spaces[static_cast<size_t>(Orientation::West)], 3, 2));

         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::East)]), 1);
         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::West)]), 1);

         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::North)]), 0);
         EXPECT_EQ(popcount<Data>(all_free_spaces[static_cast<size_t>(Orientation::South)]), 0);
     }
 }
