use core::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

use crate::rng;

pub fn num_digits(value: u32) -> u8 {
    value.checked_ilog10().unwrap_or(0) as u8 + 1
}

/// INVARIANT: All elements with an index less than self.len should be initialized
#[derive(Copy, Clone)]
pub struct List<T: Copy, const N: usize> {
    len: u8,
    elements: [MaybeUninit<T>; N],
}

impl<T: Copy, const N: usize> List<T, N> {
    pub const fn new() -> Self {
        Self {
            len: 0,
            elements: [MaybeUninit::uninit(); N],
        }
    }

    pub fn len(&self) -> u8 {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[must_use]
    pub fn insert(&mut self, value: T) -> Option<()> {
        const {
            assert!(N <= u8::MAX as usize);
        }

        if self.len >= N as u8 {
            return None;
        }

        self.elements[self.len as usize] = MaybeUninit::new(value);
        self.len += 1;

        Some(())
    }

    pub fn remove_random(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.swap_remove((rng::rng() % self.len as u32) as u8)
    }

    pub fn swap_remove(&mut self, index: u8) -> Option<T> {
        if index >= self.len {
            return None;
        }

        self.len -= 1;
        // SAFETY: removed was accessed from an index less than self.len
        let removed = unsafe { self.elements[index as usize].assume_init() };
        self.elements[index as usize] = self.elements[self.len as usize];

        Some(removed)
    }

    pub fn cascading_remove(&mut self, index: u8) -> Option<T> {
        if index >= self.len {
            return None;
        }

        // SAFETY: removed was accessed from an index less than self.len
        let removed = unsafe { self.elements[index as usize].assume_init() };
        self.len -= 1;

        for i in index + 1..self.len {
            self.elements[i as usize] = self.elements[(i + 1) as usize]
        }

        Some(removed)
    }
}

impl<T: Copy, const N: usize> Default for List<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy, const N: usize> Index<u8> for List<T, N> {
    type Output = T;

    fn index(&self, index: u8) -> &Self::Output {
        assert!(index < N as u8);
        unsafe { self.elements[index as usize].assume_init_ref() }
    }
}

impl<T: Copy, const N: usize> IndexMut<u8> for List<T, N> {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        assert!(index < N as u8);
        unsafe { self.elements[index as usize].assume_init_mut() }
    }
}
