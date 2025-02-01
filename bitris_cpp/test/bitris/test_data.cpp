 #include <gtest/gtest.h>

 #include "search.hpp"

 namespace core {
     class DataTest : public ::testing::Test {
     };

     TEST_F(DataTest, make_zero_u64) {
         using Data = uint64_t;
         using data_t = data<Data>;
         const auto board = data_t::make_zero();
         for (size_t index = 0; index < data_t::type::size(); ++index) {
             ASSERT_EQ(board[index], 0);
         }
     }

     TEST_F(DataTest, make_square_u64) {
         using Data = uint64_t;
         using data_t = data<Data>;
         constexpr Data Value = 123;
         const auto board = data_t::make_square<Value>();
         for (size_t index = 0; index < data_t::type::size(); ++index) {
             ASSERT_EQ(board[index], Value);
         }
     }

     TEST_F(DataTest, make_spawn_1bit_u8) {
         using Data = uint8_t;
         using data_t = data<Data>;
         const auto free_space = data_t::from_str(
                 ""
                 "X.XXXXXXX."
                 "XXXXXXXXX."
                 "XXXXXXXXX."
                 "XXXXXXXXX."
                 "XXXXXXXXX."
                 "XXXXXXXXX."
                 "XXXXXXXXX."
                 "XXXXXXXXX."
         ).value();
         const auto spawn = data_t::make_spawn(free_space, 4, 7);
         for (size_t index = 0; index < data_t::type::size(); ++index) {
             if (index == 4) {
                 ASSERT_EQ(spawn[index], bits<Data>::one << 7);
             } else {
                 ASSERT_EQ(spawn[index], 0);
             }
         }
     }

     TEST_F(DataTest, calculate_spawn_area_no_horizontal_hole_u8) {
         using Data = uint8_t;
         using data_t = data<Data>;
         const auto free_space = data_t::from_str(
             ""
             "XXXXXXXXX."
             "XXXXXXXXX."
             "XXXXXXXXX."
             "XXXXXX..X."
             "XXXXXX..X."
             "XXXXXX..X."
             "XXXXXX...."
             ".XXXXX...."
         ).value();
         // over board
         {
             const auto expected = data_t::from_str(
                 ""
                 "XXXXXXXXX."
                 "XXXXXXXXX."
                 "XXXXXXXXX."
                 "XXXXXX..X."
                 "XXXXXX..X."
                 "XXXXXX..X."
                 "XXXXXX...."
                 ".XXXXX...."
             ).value();
             const auto spawn = data_t::calculate_spawn_area(free_space, 20);
             ASSERT_EQ(all_of(spawn == expected), true);
         }
         // in board, over horizontal hole row
         {
             const auto expected = data_t::from_str(
                 ""
                 ".........."
                 ".........."
                 "XXXXXXXXX."
                 "XXXXXX..X."
                 "XXXXXX..X."
                 "XXXXXX..X."
                 "XXXXXX...."
                 ".XXXXX...."
             ).value();
             const auto spawn = data_t::calculate_spawn_area(free_space, 5);
             ASSERT_EQ(all_of(spawn == expected), true);
         }
     }

     TEST_F(DataTest, calculate_spawn_area_horizontal_hole_u8) {
         using Data = uint8_t;
         using data_t = data<Data>;
         const auto free_space = data_t::from_str(
             ""
             "XXXXXXXXX."
             "XXXXXXXXX."
             "XXXXXXXXX."
             "XXXXXXXXX."
             ".XXXXXXXX."
             "XXXXXXXXX."
             "XXXXXXXX.."
             "XXXXXXXXX."
         ).value();
         // over board
         {
             const auto expected = data_t::from_str(
                 ""
                 "#########."
                 "#########."
                 "#########."
                 "#########."
                 ".########."
                 ".........."
                 ".........."
                 ".........."
             ).value();
             const auto spawn = data_t::calculate_spawn_area(free_space, 20);
             ASSERT_EQ(all_of(spawn == expected), true);
         }
         // in board, over horizontal hole row
         {
             const auto expected = data_t::from_str(
                 ""
                 ".........."
                 ".........."
                 ".........."
                 "#########."
                 ".########."
                 ".........."
                 ".........."
                 ".........."
             ).value();
             const auto spawn = data_t::calculate_spawn_area(free_space, 4);
             data_t::show(spawn);
             ASSERT_EQ(all_of(spawn == expected), true);
         }
         // in board, on horizontal hole row
         {
             const auto expected = data_t::from_str(
                 ""
                 ".........."
                 ".........."
                 ".........."
                 "#########."
                 "########.."
                 ".........."
             ).value();
             const auto spawn = data_t::calculate_spawn_area(free_space, 2);
             data_t::show(spawn);
             ASSERT_EQ(all_of(spawn == expected), true);
         }
         // in board, under horizontal hole row
         {
             const auto expected = data_t::from_str(
                 ""
                 ".........."
                 ".........."
                 ".........."
                 "########.."
                 ".........."
             ).value();
             const auto spawn = data_t::calculate_spawn_area(free_space, 1);
             data_t::show(spawn);
             ASSERT_EQ(all_of(spawn == expected), true);
         }
     }

     TEST_F(DataTest, is_continuous_line_u8) {
         using Data = uint8_t;
         using data_t = data<Data>;
         const auto free_space = data_t::from_str(
             ""
             "X........."
             ".........X"
             "XX...XX..X"
             "XXXX.XXXXX"
             "..XXXXXX.."
             "XXXXXXXXXX"
         ).value();
         ASSERT_EQ( data_t::is_continuous_line(free_space, 0), true);
         ASSERT_EQ( data_t::is_continuous_line(free_space, 1), true);
         ASSERT_EQ( data_t::is_continuous_line(free_space, 2), false);
         ASSERT_EQ( data_t::is_continuous_line(free_space, 3), false);
         ASSERT_EQ( data_t::is_continuous_line(free_space, 4), true);
         ASSERT_EQ( data_t::is_continuous_line(free_space, 5), true);
     }
 }
