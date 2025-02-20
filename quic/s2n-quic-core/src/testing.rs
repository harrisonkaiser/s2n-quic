// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use bolero_generator::TypeGenerator;

#[derive(Clone, Copy, Debug, TypeGenerator)]
pub struct InlineVec<T, const LEN: usize> {
    values: [T; LEN],

    #[generator(_code = "0..LEN")]
    len: usize,
}

impl<T, const LEN: usize> core::ops::Deref for InlineVec<T, LEN> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.values[..self.len]
    }
}

impl<T, const LEN: usize> core::ops::DerefMut for InlineVec<T, LEN> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values[..self.len]
    }
}
