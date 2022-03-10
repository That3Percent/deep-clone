mod std_impls;

macro_rules! from_clone {
    ($($name:ty),*) => {
        $(
            impl crate::DeepClone for $name {
                #[must_use]
                #[inline]
                fn deep_clone(&self) -> Self {
                    ::std::clone::Clone::clone(self)
                }
            }
        )*
    };
}
pub(crate) use from_clone;

pub trait DeepClone: Sized {
    #[must_use]
    fn deep_clone(&self) -> Self;

    #[inline]
    fn deep_clone_from(&mut self, source: &Self) {
        *self = source.deep_clone()
    }
}
