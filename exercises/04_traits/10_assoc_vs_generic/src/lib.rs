// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

// Exponent 是泛型， 但是有默认值为要为实现的类型， 要实现的类型可以通过：：访问关联类型
// 关联类型通常用来控制输出的类型
pub trait Power<Exponent = Self> {
    type Output;

    fn power(&self, n: Exponent) -> Self::Output;
}

impl Power<u16> for u32 {
    type Output = u32;
    fn power(&self, n: u16) -> Self::Output {
        self.pow(n.into())
    }
}

impl Power for u32 {
    type Output = u32;
    fn power(&self, n: Self) -> Self::Output {
        self.pow(n.into())
    }
}

impl Power<&u32> for u32 {
    type Output = u32;
    fn power(&self, n: &u32) -> Self::Output {
        self.pow(*n)
    }
}
#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
