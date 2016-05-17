// Copyright 2016 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.


/// The field is unsigned.
const UNSIGNED_FLAG: u64 = 32;

/// `has_unsigned_flag` checks if `UNSIGNED_FLAG` is set.
#[inline]
pub fn has_unsigned_flag(flag: u64) -> bool {
    flag & UNSIGNED_FLAG > 0
}