#[macro_export]
macro_rules! trait_union {
    ($new_trait_name:ident, $($existing_traits:tt)+) => {
        pub trait $new_trait_name: $($existing_traits)+ {}
        impl<T> $new_trait_name for T where T: $($existing_traits)+ {}
    };
}
