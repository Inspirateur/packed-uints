const U4_IN_U8: usize = 8 / 4;
const PARITY_MASK: usize = U4_IN_U8 - 1;

#[derive(Debug, Clone)]
pub enum PackedEnum {
    U4(Vec<u8>),
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
}

impl PackedEnum {
    pub fn mask(&self) -> usize {
        match self {
            Self::U4(_) => 15 as usize,
            Self::U8(_) => u8::MAX as usize,
            Self::U16(_) => u16::MAX as usize,
            Self::U32(_) => u32::MAX as usize,
        }
    }

    #[inline(always)]
    fn get(&self, i: usize) -> usize {
        match self {
            Self::U4(data) => {
                let shift = 4 * (i & PARITY_MASK);
                ((data[i/U4_IN_U8] >> shift ) & 0b1111) as usize
            }
            Self::U8(data) => data[i] as usize,
            Self::U16(data) => data[i] as usize,
            Self::U32(data) => data[i] as usize,
        }
    }

    #[inline(always)]
    fn set(&mut self, i: usize, value: usize) {
        match self {
            Self::U4(data) => {
                let shift: usize = 4 * (i & PARITY_MASK);
                let mask = 0b1111 << shift;
                let i = i / U4_IN_U8;
                data[i] &= !mask;
                data[i] |= (value as u8) << shift;
            }
            Self::U8(data) => {
                data[i] = value as u8;
            }
            Self::U16(data) => {
                data[i] = value as u16;
            }
            Self::U32(data) => {
                data[i] = value as u32;
            }
        }
    }

    fn set_range(&mut self, start: usize, end: usize, value: usize) {
        match self {
            Self::U4(data) => {
                // NOTE: this part assumes we're storing u4 in u8 (unlike the rest of the code)
                let shift = 4;
                let end = end - 1;
                let value = value as u8;
                let start_parity = start & 0b1;
                let end_parity = end & 0b1;
                let start = start / 2;
                let end = end / 2;
                if start_parity == 1 {
                    data[start] &= 0b1111;
                    data[start] |= value << shift;
                }
                if end_parity == 0 {
                    data[end] &= 0b1111 << shift;
                    data[end] |= value;
                }
                let value = value | value << shift;
                for i in (start + start_parity)..(end + end_parity) {
                    data[i] = value;
                }
            }
            Self::U8(data) => {
                for i in start..end {
                    data[i] = value as u8;
                }
            }
            Self::U16(data) => {
                for i in start..end {
                    data[i] = value as u16;
                }
            }
            Self::U32(data) => {
                for i in start..end {
                    data[i] = value as u32;
                }
            }
        }
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = usize> + 'a> {
        match self {
            Self::U4(data) => Box::new(data.iter().flat_map(|a| {
                [(a & 0b1111) as usize, (a >> 4) as usize]
            })),
            Self::U8(data) => Box::new(data.iter().map(|a| *a as usize)),
            Self::U16(data) => Box::new(data.iter().map(|a| *a as usize)),
            Self::U32(data) => Box::new(data.iter().map(|a| *a as usize)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PackedUints {
    pub data: PackedEnum,
    pub mask: usize,
    pub length: usize,
}

impl PackedUints {
    pub fn new(length: usize) -> Self {
        PackedUints::filled(length, 0)
    }

    pub fn filled(length: usize, value: usize) -> Self {
        let bits = value.max(2).ilog2();
        let data = if bits < 4 {
            let value = value | (value << 4);
            PackedEnum::U4(vec![value as u8; (length+U4_IN_U8-1) / U4_IN_U8])
        } else if bits < 8 {
            PackedEnum::U8(vec![value as u8; length])
        } else if bits < 16 {
            PackedEnum::U16(vec![value as u16; length])
        } else {
            PackedEnum::U32(vec![value as u32; length])
        };
        PackedUints { 
            data: data, 
            mask: 0b1111, 
            length: length 
        }
    }

    pub fn from(values: &[usize]) -> Self {
        let bits = values.iter().max().unwrap_or(&2).ilog2();
        let data = if bits < 4 {
            let mut res = vec![0; (values.len()+U4_IN_U8-1) / U4_IN_U8];
            for i in (0..values.len()).step_by(2) {
                res[i/2] = (values[i+1] << 4 | values[i]) as u8;
            }
            PackedEnum::U4(res)
        } else if bits < 8 {
            PackedEnum::U8(values.iter().map(|a| *a as u8).collect())
        } else if bits < 16 {
            PackedEnum::U16(values.iter().map(|a| *a as u16).collect())
        } else {
            PackedEnum::U32(values.iter().map(|a| *a as u32).collect())
        };
        PackedUints {
            mask: data.mask(),
            data,
            length: values.len() 
        }
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = usize> + 'a> {
        self.data.iter()
    }

    #[inline]
    pub fn get(&self, i: usize) -> usize {
        self.data.get(i)
    }

    #[inline]
    fn upscale_if_needed(&mut self, value: usize) {
        if (value & self.mask) != value {
            let bits = value.ilog2();
            self.data = if bits < 8 {
                PackedEnum::U8(self.data.iter().take(self.length).map(|a| a as u8).collect())
            } else if bits < 16 {
                PackedEnum::U16(self.data.iter().take(self.length).map(|a| a as u16).collect())
            } else {
                PackedEnum::U32(self.data.iter().take(self.length).map(|a| a as u32).collect())
            };
            self.mask = self.data.mask();
        }
    }

    #[inline]
    pub fn set(&mut self, i: usize, value: usize) {
        self.upscale_if_needed(value);
        self.data.set(i, value)
    }

    #[inline]
    pub fn set_range(&mut self, start: usize, end: usize, value: usize) {
        // check that both start and length are even
        self.upscale_if_needed(value);
        self.data.set_range(start, end, value);
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;
    use rand::Rng;
    use super::PackedUints;

    fn test_equal(usizes: &PackedUints, values: &[usize]) {
        for (i, value) in values.iter().enumerate() {
            assert_eq!(*value, usizes.get(i));
        }
    }

    fn roundtrip(usizes: &mut PackedUints, values: &[usize]) {
        for (i, value) in values.iter().enumerate() {
            usizes.set(i, *value);
        }
        test_equal(usizes, values);
    }

    #[test]
    pub fn test_from_iter() {
        let mut rng = rand::thread_rng();
        let values: [usize; 100] = [(); 100].map(|_| rng.gen_range(0..16));
        let usizes = PackedUints::from(&values);
        // retrieve them and test for equality
        for (a, b) in zip(values, usizes.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    pub fn test_u4() {
        // EASY: Every values are in range
        // holds integers of 4 bits (max = 2^4-1 = 15)
        let mut rng = rand::thread_rng();
        let mut usizes = PackedUints::new(100);
        let values: [usize; 100] = [(); 100].map(|_| rng.gen_range(0..16));
        roundtrip(&mut usizes, &values);
    }

    fn test_set_range(data_len: usize, start: usize, end: usize, value: usize) {
        let mut usizes = PackedUints::new(data_len);
        let mut values = vec![0; data_len];
        for i in start..end {
            values[i] = value;
        }
        usizes.set_range(start, end, value);
        test_equal(&usizes, &values);
    }

    #[test]
    pub fn test_set_range_1() {
        test_set_range(100, 0, 32, 7);
    }

    #[test]
    pub fn test_set_range_2() {
        test_set_range(100, 1, 32, 7);
    }

    #[test]
    pub fn test_set_range_3() {
        test_set_range(100, 1, 31, 7);
    }

    #[test]
    pub fn test_set_range_4() {
        test_set_range(100, 0, 31, 7);
    }

    #[test]
    pub fn test_reallocation() {
        // HARD: some values exceed the capacity of 2^bitsize-1, need to reallocate
        let mut rng = rand::thread_rng();
        let mut usizes = PackedUints::new(100);
        let values: [usize; 100] = [(); 100].map(|_| rng.gen_range(0..u32::MAX) as usize);
        roundtrip(&mut usizes, &values);
    }
}
