// Minimal replacement for numeric_limits of integers. -*- C++ -*-

// Copyright (C) 2020 Free Software Foundation, Inc.
//
// This file is part of the GNU ISO C++ Library.  This library is free
// software; you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the
// Free Software Foundation; either version 3, or (at your option)
// any later version.

// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// Under Section 7 of GPL version 3, you are granted additional
// permissions described in the GCC Runtime Library Exception, version
// 3.1, as published by the Free Software Foundation.

// You should have received a copy of the GNU General Public License and
// a copy of the GCC Runtime Library Exception along with this program;
// see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
// <http://www.gnu.org/licenses/>.

/** @file bits/int_limits.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{limits}
 */

#ifndef _GLIBCXX_INT_LIMITS_H
#define _GLIBCXX_INT_LIMITS_H 1

#pragma GCC system_header

#if __cplusplus >= 201103L
#include <bits/c++config.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION
namespace __detail
{
  // This template is used for arbitrary signed and unsigned integer types
  // (by headers <bit> and <charconv>) and for specific integer types
  // (by <memory_resource> and <string_view>) but also for char (<charconv>).
  // For simplicity's sake, all integral types except bool are supported.

  // Lightweight alternative to numeric_limits<signed integer type>.
  template<typename _Tp, bool = is_signed<_Tp>::value>
    struct __int_limits
    {
      static_assert(is_integral<_Tp>::value, "unsupported specialization");
      using _Up = typename make_unsigned<_Tp>::type;
      static constexpr int digits = sizeof(_Tp) * __CHAR_BIT__ - 1;
      static constexpr _Tp min() noexcept { return _Tp(_Up(1) << digits); }
      static constexpr _Tp max() noexcept { return _Tp(_Up(~_Up(0)) >> 1); }
    };

  // Lightweight alternative to numeric_limits<unsigned integer type>.
  template<typename _Tp>
    struct __int_limits<_Tp, false>
    {
      static_assert(is_integral<_Tp>::value, "unsupported specialization");
      static constexpr int digits = sizeof(_Tp) * __CHAR_BIT__;
      static constexpr _Tp min() noexcept { return 0; }
      static constexpr _Tp max() noexcept { return _Tp(-1); }
    };

  template<> struct __int_limits<bool>; // not defined
}
_GLIBCXX_END_NAMESPACE_VERSION
} // namespace
#endif // C++11
#endif // _GLIBCXX_INT_LIMITS_H
