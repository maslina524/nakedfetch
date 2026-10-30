macro_rules! STRUCT {
    ($(#[$attrs:meta])* struct $name:ident {
        $($field:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] #[derive(Clone, Copy)] $(#[$attrs])*
        pub struct $name {
            $(pub $field: $ftype,)+
        }

        impl Default for $name {
            fn default() -> $name {
                // SAFETY: Guarantee that all fields can be zeroable
                unsafe { ::core::mem::zeroed() }
            }
        }
    );
}