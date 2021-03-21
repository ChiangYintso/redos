#![feature(unchecked_math)]

use core::mem::size_of;

#[inline]
fn first_son(node: usize) -> usize {
    node - (lowbit(node) >> 1)
}

#[inline]
fn first_sibling(node: usize) -> usize {
    first_son(father(node))
}

#[inline]
fn next_sibling(node: usize) -> usize {
    node + (lowbit(node) >> 1)
}

#[inline]
fn father(node: usize) -> usize {
    node + lowbit(node)
}

#[inline]
fn lowbit(x: usize) -> usize {
    unsafe { x & ((!x).unchecked_add(1)) }
}

/// Bitmap implemented on tree-like array.
/// Valid elements are in range `[1, length]`
///
/// ```text
///                     10000
///                    / \    \
///                   /   \     \
///                 /\      \     \
///               /   \      \      \
///              /     \       \      \
///            /        \        \      \
///           /          \         \      \
///        1000         1100       1110   1111
///       /\   \        /   \      /
///      /  \  0111   1010  1011  1101
///     /    \         /
///   0100   0110    1001
///   /   \    \
/// 0010 0011  0101
///  /
/// 0001
/// ```
pub struct TreeBitMap {
    bit_tree: *mut usize,
    length: usize,
    used: usize,
}

impl TreeBitMap {
    /// All the bit flags are set to 1.
    ///
    /// # Safety
    /// Memory space that starts from `bit_vector` with length `length` should be valid.
    pub unsafe fn ones(mut bit_vector: *mut usize, length: usize) -> TreeBitMap {
        core::ptr::write_bytes(bit_vector, 0xff, length);
        bit_vector = bit_vector.offset(-1);
        TreeBitMap {
            bit_tree: bit_vector,
            length,
            used: 0,
        }
    }

    pub fn alloc_n(&mut self, n: usize) -> Option<usize> {
        if self.used + n > self.length || n == 0 {
            return None;
        }

        let block = self.find_available_block(n);

        None
    }

    fn find_available_block(&mut self, n: usize) -> usize {
        let mut father: usize = 0;
        let mut cur_node = first_sibling(self.length);
        loop {
            if cur_node != 0 {
                father = cur_node;
                cur_node = first_son(cur_node);
            } else if (cur_node & 1) == 1 {
                // all the sons are full.
            }
        }
    }

    #[inline]
    fn get(&self, n: usize) -> usize {
        unsafe { *self.bit_tree.add(n) }
    }

    pub fn flip(&mut self, n: usize) {
        let mut block_id = n / size_of::<usize>();
        let offset = n % size_of::<usize>();
        while block_id <= self.length {
            unsafe {
                // flip nth bitflag
                let bt = self.bit_tree.add(block_id);
                *bt ^= 1 << offset;
            }
            block_id += lowbit(block_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{first_son, TreeBitMap};

    // 8M
    static mut ARENA: [u8; 0x80_0000] = [0; 0x80_0000];

    #[test]
    fn it_works() {
        unsafe {
            let mut bitmap = TreeBitMap::ones(ARENA.as_ptr() as *mut usize, 1000);
            let end = 1000 * core::mem::size_of::<usize>();
            for i in 0..end {
                assert_eq!(ARENA[i], u8::MAX, "{}", i);
            }
            assert_eq!(ARENA[end + 1], 0);
        }
    }

    #[test]
    fn test_first_son() {
        assert_eq!(first_son(0b1001), 0b1001);
        assert_eq!(first_son(0b1010), 0b1001);
        assert_eq!(first_son(0b1000), 0b100);
    }
}
