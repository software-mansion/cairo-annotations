macro_rules! impl_namespace {
    ($namespace:expr, $($struct_name:ident),+) => {
        $(
            impl crate::annotations::traits::Namespace for $struct_name {
                const NAMESPACE: &'static str = $namespace;
            }
        )+
    };
}

pub(crate) use impl_namespace;
