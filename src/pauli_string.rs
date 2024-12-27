use std::{error::Error, fmt::Display, ops::Mul};

pub(super) const CHUNK_SIZE: usize = u64::BITS as usize;

// https://eugene-babichenko.github.io/blog/2019/11/13/rust-popcount-intrinsics/

#[cfg_attr(target_arch = "x86_64", target_feature(enable = "popcnt"))]
unsafe fn count(a: u64) -> u32 {
    a.count_ones()
}

// -------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum PauliStringError {
    PositionOutOfBound { size: usize, position: usize },
    InvalidPauliCode(char),
}

use PauliStringError::{InvalidPauliCode, PositionOutOfBound};

impl Display for PauliStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PositionOutOfBound { size, position } => write!(
                f,
                "Position {position} is out of bound of a Pauli string of size {size}"
            )?,
            InvalidPauliCode(code) => write!(
                f,
                "Invalid character code {code} of a Pauli matrix, code must be 'I', 'X', 'Y' or 'Z' only"
            )?,
        }
        Ok(())
    }
}

impl Error for PauliStringError {}

pub(super) type PauliStringResult<T> = Result<T, PauliStringError>;

// -------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord, Hash)]
struct PauliChunk {
    bitchunk1: u64,
    bitchunk2: u64,
}

impl PauliChunk {
    #[inline(always)]
    fn mul_abs(&self, other: &Self) -> Self {
        Self {
            bitchunk1: self.bitchunk1 ^ other.bitchunk1,
            bitchunk2: self.bitchunk2 ^ other.bitchunk2,
        }
    }
    #[inline(always)]
    fn mul_phase(&self, other: &Self) -> i32 {
        let new_bitchunk1 = self.bitchunk1 & !self.bitchunk2 & !other.bitchunk1 & other.bitchunk2
            | !self.bitchunk1 & self.bitchunk2 & other.bitchunk1 & other.bitchunk2
            | self.bitchunk1 & self.bitchunk2 & other.bitchunk1 & !other.bitchunk2;
        let new_bitchunk2 = !self.bitchunk1 & self.bitchunk2 & other.bitchunk1 & !other.bitchunk2
            | self.bitchunk1 & self.bitchunk2 & !other.bitchunk1 & other.bitchunk2
            | self.bitchunk1 & !self.bitchunk2 & other.bitchunk1 & other.bitchunk2;
        unsafe { count(new_bitchunk1) as i32 - count(new_bitchunk2) as i32 }
    }
    #[inline(always)]
    fn average(&self) -> u8 {
        if self.bitchunk1 ^ self.bitchunk2 == 0 {
            1
        } else {
            0
        }
    }
    #[inline(always)]
    fn hamming(&self) -> u32 {
        unsafe { count(self.bitchunk1 ^ self.bitchunk2) }
    }
}

impl Display for PauliChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..CHUNK_SIZE {
            match ((self.bitchunk1 >> i) & 1, (self.bitchunk2 >> i) & 1) {
                (0, 0) => write!(f, "I")?,
                (1, 0) => write!(f, "X")?,
                (0, 1) => write!(f, "Y")?,
                (1, 1) => write!(f, "Z")?,
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct PauliString<const N: usize>([PauliChunk; N]);

impl<const N: usize> Display for PauliString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &pauli_chunk in &self.0 {
            pauli_chunk.fmt(f)?;
        }
        Ok(())
    }
}

impl<const N: usize> PauliString<N> {
    pub(super) fn new() -> Self {
        Self([Default::default(); N])
    }
    pub(super) fn set_pauli(mut self, code: char, pos: u64) -> PauliStringResult<Self> {
        let chunk_number = pos / CHUNK_SIZE as u64;
        let pos_in_chunk = pos % CHUNK_SIZE as u64;
        if chunk_number >= N as u64 {
            Err(PositionOutOfBound {
                size: N,
                position: pos as usize,
            })
        } else {
            match code {
                'X' => {
                    self.0[chunk_number as usize].bitchunk1 |= 1 << pos_in_chunk;
                }
                'Y' => {
                    self.0[chunk_number as usize].bitchunk2 |= 1 << pos_in_chunk;
                }
                'Z' => {
                    self.0[chunk_number as usize].bitchunk1 |= 1 << pos_in_chunk;
                    self.0[chunk_number as usize].bitchunk2 |= 1 << pos_in_chunk;
                }
                other => return Err(InvalidPauliCode(other)),
            }
            Ok(self)
        }
    }
    pub(super) fn average(&self) -> u8 {
        let mut val = 1;
        for pauli_chunk in &self.0 {
            val &= pauli_chunk.average();
        }
        val
    }
    pub(super) fn comute(&self, other: &Self) -> bool {
        let total_phase: i32 = self
            .0
            .iter()
            .zip(&other.0)
            .map(|(lhs, rhs)| lhs.mul_phase(rhs))
            .sum();
        (total_phase % 2) == 0
    }
    pub(super) fn hamming(&self) -> u32 {
        self.0.iter().map(|chunk| chunk.hamming()).sum()
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<const N: usize> Mul<&PauliString<N>> for &PauliString<N> {
    type Output = (i32, PauliString<N>);
    #[inline(always)]
    fn mul(self, rhs: &PauliString<N>) -> Self::Output {
        // TODO: initialization is unnecessary. Think how to optimize it.
        // A trick with MaybeUninit + transmute seems not to work.
        let mut pauli_chunks = [PauliChunk::default(); N];
        let mut phase = 0i32;
        for (dst, (lhs, rhs)) in pauli_chunks.iter_mut().zip(self.0.iter().zip(rhs.0.iter())) {
            *dst = lhs.mul_abs(rhs);
            phase += lhs.mul_phase(rhs);
        }
        (phase, PauliString(pauli_chunks))
    }
}

#[cfg(test)]
mod tests {
    use super::PauliString;
    use std::fmt::Write;
    #[test]
    fn test_display_pauli_string() {
        let mut displ = String::with_capacity(128);
        let pauli_string = PauliString::<2>::new()
            .set_pauli('X', 0)
            .unwrap()
            .set_pauli('Y', 15)
            .unwrap()
            .set_pauli('Z', 63)
            .unwrap()
            .set_pauli('X', 64)
            .unwrap()
            .set_pauli('Y', 100)
            .unwrap()
            .set_pauli('Z', 127)
            .unwrap();
        write!(displ, "{pauli_string}").unwrap();
        let correct_displ = "XIIIIIIIIIIIIIIYIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIZXIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIYIIIIIIIIIIIIIIIIIIIIIIIIIIZ";
        assert_eq!(displ, correct_displ);
    }
    #[test]
    fn test_mul_pauli_strings_1() {
        let lhs = PauliString::<2>::new()
            .set_pauli('X', 0)
            .unwrap()
            .set_pauli('X', 7)
            .unwrap()
            .set_pauli('X', 15)
            .unwrap()
            .set_pauli('X', 23)
            .unwrap()
            .set_pauli('Y', 31)
            .unwrap()
            .set_pauli('Y', 39)
            .unwrap()
            .set_pauli('Y', 47)
            .unwrap()
            .set_pauli('Y', 55)
            .unwrap()
            .set_pauli('Z', 63)
            .unwrap()
            .set_pauli('Z', 64)
            .unwrap()
            .set_pauli('Z', 72)
            .unwrap()
            .set_pauli('Z', 80)
            .unwrap()
            .set_pauli('Z', 127)
            .unwrap();
        let rhs = PauliString::<2>::new()
            .set_pauli('X', 7)
            .unwrap()
            .set_pauli('Y', 15)
            .unwrap()
            .set_pauli('Z', 23)
            .unwrap()
            .set_pauli('X', 39)
            .unwrap()
            .set_pauli('Y', 47)
            .unwrap()
            .set_pauli('Z', 55)
            .unwrap()
            .set_pauli('X', 64)
            .unwrap()
            .set_pauli('Y', 72)
            .unwrap()
            .set_pauli('Z', 80)
            .unwrap()
            .set_pauli('X', 88)
            .unwrap()
            .set_pauli('Y', 96)
            .unwrap()
            .set_pauli('Z', 104)
            .unwrap()
            .set_pauli('Y', 127)
            .unwrap();
        let correct_result = PauliString::<2>::new()
            .set_pauli('X', 0)
            .unwrap()
            .set_pauli('Z', 15)
            .unwrap()
            .set_pauli('Y', 23)
            .unwrap()
            .set_pauli('Y', 31)
            .unwrap()
            .set_pauli('Z', 39)
            .unwrap()
            .set_pauli('X', 55)
            .unwrap()
            .set_pauli('Z', 63)
            .unwrap()
            .set_pauli('Y', 64)
            .unwrap()
            .set_pauli('X', 72)
            .unwrap()
            .set_pauli('X', 88)
            .unwrap()
            .set_pauli('Y', 96)
            .unwrap()
            .set_pauli('Z', 104)
            .unwrap()
            .set_pauli('X', 127)
            .unwrap();
        let (phase, result) = &lhs * &rhs;
        assert_eq!(result, correct_result);
        assert_eq!(phase, -1);
    }
    #[test]
    fn test_mul_pauli_strings_2() {
        let lhs = PauliString::<3>::new()
            .set_pauli('X', 0)
            .unwrap()
            .set_pauli('Y', 64)
            .unwrap()
            .set_pauli('Z', 150)
            .unwrap();
        let rhs = PauliString::<3>::new()
            .set_pauli('Y', 0)
            .unwrap()
            .set_pauli('Z', 64)
            .unwrap()
            .set_pauli('Z', 150)
            .unwrap();
        let (phase, result) = &lhs * &rhs;
        let correct_result = PauliString::<3>::new()
            .set_pauli('Z', 0)
            .unwrap()
            .set_pauli('X', 64)
            .unwrap();
        assert_eq!(result, correct_result);
        assert_eq!(phase, 2);
    }
}
