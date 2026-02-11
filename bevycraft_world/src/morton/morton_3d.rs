use std::hash::{Hash, Hasher};
use bevy::math::*;

pub trait MortonEncodable {
    fn encode_u64(&self) -> u64;
}

pub trait MortonDecodable {
    fn decode_u64(morton: u64) -> Self;
}

const MORTON_INDEX_MASK: u64 = 0x7;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Morton3D(u64);

impl From<u64> for Morton3D {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Morton3D> for u64 {
    fn from(value: Morton3D) -> Self {
        value.0
    }
}

impl Hash for Morton3D {
    
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.0)
    }
}

impl Morton3D {
    
    #[inline]
    pub fn encode<T: MortonEncodable>(value: T) -> Self {
        Self(value.encode_u64())
    }

    #[inline]
    pub fn decode<T: MortonDecodable>(&self) -> T {
        T::decode_u64(self.0)
    }
    
    #[inline]
    pub const fn raw(&self) -> u64 {
        self.0
    }

    #[inline]
    pub const fn get_morton_index(&self, index: usize) -> usize {
        debug_assert!(index <= 21, "A Morton can only hold 21 indices");
        
        unsafe { ((self.0 >> index.unchecked_mul(3)) & MORTON_INDEX_MASK) as usize }
    }

    #[inline]
    fn split_bits(n: impl Into<u64>) -> u64 {
        let mut x = n.into() & 0x1fffff;
        x = (x | x << 32) & 0x1f00000000ffff;
        x = (x | x << 16) & 0x1f0000ff0000ff;
        x = (x | x << 8) & 0x100f00f00f00f00f;
        x = (x | x << 4) & 0x10c30c30c30c30c3;
        (x | x << 2) & 0x1249249249249249
    }

    #[inline]
    fn join_bits(n: impl Into<u64>) -> u64 {
        let mut x = n.into() & 0x1249249249249249;
        x = (x ^ x >> 2) & 0x10c30c30c30c30c3;
        x = (x ^ x >> 4) & 0x100f00f00f00f00f;
        x = (x ^ x >> 8) & 0x1f0000ff0000ff;
        x = (x ^ x >> 16) & 0x1f00000000ffff;
        (x ^ x >> 32) & 0x1fffff
    }
}

impl MortonEncodable for IVec3 {
    fn encode_u64(&self) -> u64 {
        Morton3D::split_bits(self.x.unsigned_abs())
            | (Morton3D::split_bits(self.y.unsigned_abs()) << 1)
            | (Morton3D::split_bits(self.z.unsigned_abs()) << 2)
    }
}

impl MortonDecodable for IVec3 {
    fn decode_u64(morton: u64) -> Self {
        IVec3::new(
            Morton3D::join_bits(morton) as i32,
            Morton3D::join_bits(morton >> 1) as i32,
            Morton3D::join_bits(morton >> 2) as i32
        )
    }
}

impl MortonEncodable for UVec3 {
    fn encode_u64(&self) -> u64 {
        Morton3D::split_bits(self.x)
            | (Morton3D::split_bits(self.y) << 1)
            | (Morton3D::split_bits(self.z) << 2)
    }
}

impl MortonDecodable for UVec3 {
    fn decode_u64(morton: u64) -> Self {
        UVec3::new(
            Morton3D::join_bits(morton) as u32,
            Morton3D::join_bits(morton >> 1) as u32,
            Morton3D::join_bits(morton >> 2) as u32
        )
    }
}

impl MortonEncodable for [i32; 3] {
    fn encode_u64(&self) -> u64 {
        Morton3D::split_bits(self[0].unsigned_abs())
            | (Morton3D::split_bits(self[1].unsigned_abs()) << 1)
            | (Morton3D::split_bits(self[2].unsigned_abs()) << 2)
    }
}

impl MortonDecodable for [i32; 3] {
    fn decode_u64(morton: u64) -> Self {
        [
            Morton3D::join_bits(morton) as i32,
            Morton3D::join_bits(morton >> 1) as i32,
            Morton3D::join_bits(morton >> 2) as i32,
        ]
    }
}

impl MortonEncodable for (i32, i32, i32) {
    fn encode_u64(&self) -> u64 {
        Morton3D::split_bits(self.0.unsigned_abs())
            | (Morton3D::split_bits(self.1.unsigned_abs()) << 1)
            | (Morton3D::split_bits(self.2.unsigned_abs()) << 2)
    }
}

impl MortonDecodable for (i32, i32, i32) {
    fn decode_u64(morton: u64) -> Self {
        (
            Morton3D::join_bits(morton) as i32,
            Morton3D::join_bits(morton >> 1) as i32,
            Morton3D::join_bits(morton >> 2) as i32,
        )
    }
}