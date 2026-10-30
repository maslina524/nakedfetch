macro_rules! STRUCT {
    ($(#[$attrs:meta])* struct $name:ident {
        $($field:ident: $ftype:ty),+ $(,)?
    }) => (
        #[repr(C)]
        #[derive(Clone)]
        $(#[$attrs])*
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

macro_rules! ENUM {
    ($(#[$attrs:meta])* enum $name:ident {
        $($variant:ident $(= $discr:expr)?),* $(,)?
    }) => (
        #[repr(i32)]
        #[derive(Clone, Copy)]
        $(#[$attrs])*
        pub enum $name {
            $($variant $(= $discr)?),*
        }

        const _: () = ::core::assert!(
            ::core::mem::size_of::<$name>() == ::core::mem::size_of::<i32>()
        );
    );
}
