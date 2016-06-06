/// Basically the same as https://github.com/Techern/Varint-rs/blob/master/src/zigzag.rs
pub trait ZigZag<T> {
    fn zigzag(&self) -> T;
}

impl ZigZag<i32> for u32 {
    fn zigzag(&self) -> i32 {
        let n = *self;
        (n >> 1) as i32 ^ -((n & 1) as i32)
    }
}

impl ZigZag<u32> for i32 {
    fn zigzag(&self) -> u32 {
        let n = *self;
        ((n << 1) ^ (n >> 31)) as u32
    }
}

#[cfg(test)]
mod tests32 {
    use super::ZigZag;

    #[test]
    fn u32_to_i32() {
        assert_eq!(0i32, 0u32.zigzag());
        assert_eq!((-1 as i32), 1u32.zigzag());
        assert_eq!(1i32, 2u32.zigzag());
        assert_eq!((-2 as i32), 3u32.zigzag());
        assert_eq!(2147483647i32, 4294967294u32.zigzag());
        assert_eq!((-2147483648 as i32), 4294967295u32.zigzag());
    }

    #[test]
    fn i32_to_u32() {
        assert_eq!(0i32.zigzag(), 0u32);
        assert_eq!((-1 as i32).zigzag(), 1u32);
        assert_eq!(1i32.zigzag(), 2u32);
        assert_eq!((-2 as i32).zigzag(), 3u32);
        assert_eq!(2147483647i32.zigzag(), 4294967294u32);
        assert_eq!((-2147483648 as i32).zigzag(), 4294967295u32);
    }
}

impl ZigZag<i64> for u64 {
    fn zigzag(&self) -> i64 {
        let n = *self;
        ((n >> 1) as i64) ^ (-((n & 1) as i64))
    }
}

impl ZigZag<u64> for i64 {
    fn zigzag(&self) -> u64 {
        let n = *self;
        ((n << 1) ^ (n >> 63)) as u64
    }
}

#[cfg(test)]
mod tests64 {
    use super::ZigZag;

    #[test]
    fn u64_to_i64() {
        assert_eq!(0i64, 0u64.zigzag());
        assert_eq!((-1 as i64), 1u64.zigzag());
        assert_eq!(1i64, 2u64.zigzag());
        assert_eq!((-2 as i64), 3u64.zigzag());
        assert_eq!(2147483647i64, 4294967294u64.zigzag());
        assert_eq!((-2147483648 as i64), 4294967295u64.zigzag());
    }

    #[test]
    fn i64_to_u64() {
        assert_eq!(0i64.zigzag(), 0u64);
        assert_eq!((-1 as i64).zigzag(), 1u64);
        assert_eq!(1i64.zigzag(), 2u64);
        assert_eq!((-2 as i64).zigzag(), 3u64);
        assert_eq!(2147483647i64.zigzag(), 4294967294u64);
        assert_eq!((-2147483648 as i64).zigzag(), 4294967295u64);
    }
}
