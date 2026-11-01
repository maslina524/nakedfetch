macro_rules! STRUCT {
    (Naked $(#[$attrs:meta])* struct $name:ident {
        $($field:ident: $ftype:ty),+ $(,)?
    }) => (
        #[repr(C)]
        $(#[$attrs])*
        pub struct $name {
            $(pub $field: $ftype,)+
        }
    );
    ($(#[$attrs:meta])* struct $name:ident {
        $($field:ident: $ftype:ty),+ $(,)?
    }) => {
        STRUCT! {
            Naked
            #[derive(Clone)]
            $(#[$attrs])*
            struct $name {
                $($field: $ftype,)+
            }
        }

        impl Default for $name {
            fn default() -> Self {
                // SAFETY: Guarantee that all fields can be zeroable
                unsafe { ::core::mem::zeroed() }
            }
        }
    }
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

macro_rules! UNION {
    ($(#[$attrs:meta])* union $name:ident {
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

macro_rules! impl_tag {
    ($trait:ident : $($name:ident),+ $(,)?) => {
        $(
            impl $trait for $name {}
        )+
    };
}

macro_rules! impl_sealed {
    ($($name:ident),+ $(,)?) => {
        $(
            impl sealed::Sealed for $name {}
        )+
    };
}

macro_rules! mod_sealed {
    () => {
        mod sealed {
            pub trait Sealed {}
        }
    };
}

macro_rules! link {
    ($library:literal $($link_name:literal)? $(#[$doc:meta])? fn $($function:tt)*) => (
        #[link(name = $library)]
        unsafe extern "system" {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    );
    ($library:literal $($link_name:literal)? $(#[$doc:meta])? safe fn $($function:tt)*) => (
        #[link(name = $library)]
        unsafe extern "system" {
            $(#[link_name=$link_name])?
            pub safe fn $($function)*;
        }
    );
}
