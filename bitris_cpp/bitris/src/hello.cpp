#include <experimental/simd>

#include "hello.hpp"

using std::experimental::native_simd;
using Vec3D = std::array<native_simd<float>, 3>;
native_simd<float> scalar_product(Vec3D a, Vec3D b) {
  return a[0] + b[0];
}

int hello(int v) {
//  constexpr std::size_t VECREG_SIZE = native_simd<float>::size();
//    std::cout << "Hello, C++!" << std::endl;
//  std::cout << VECREG_SIZE << std::endl;
//#include <utility>
//    std::unreachable();


    Vec3D vec1 = {native_simd<float>((float) v), native_simd<float>((float) v), native_simd<float>((float) v)};
    Vec3D vec2 = {native_simd<float>(1.0f), native_simd<float>(1.0f), native_simd<float>(1.0f)};
    
    native_simd<float> result = scalar_product(vec1, vec2);
    
    float answer = result[0]; // Extract the scalar value of the dot product from the result

    return (int) answer;
}

//int hello(int v) {
//    // Use NEON to add 1 to each element of a vector
//    int32x4_t vec = vdupq_n_s32(v); // Set all elements to the input value
//    int32x4_t one = vdupq_n_s32(1); // Vector with all elements set to 1
//    int32x4_t result = vaddq_s32(vec, one); // Add the vectors element-wise
//
//    // Extract the first result (you can adapt this to do something else with the NEON vector)
//    return vgetq_lane_s32(result, 0);
//}

//int hello(int v) {
//    // Extract the first result (you can adapt this to do something else with the NEON vector)
//    // Use std::simd to add 1 to each element of a vector
//    std::simd<int32_t, std::simd_abi::fixed_size<4>> vec(v); // Set all elements to the input value
//    std::simd<int32_t, std::simd_abi::fixed_size<4>> one(1); // Vector with all elements set to 1
//    auto result = vec + one; // Add the vectors element-wise
//
//    // Extract the first result (you can adapt this to do something else with the SIMD vector)
//    return result[0];
//}
