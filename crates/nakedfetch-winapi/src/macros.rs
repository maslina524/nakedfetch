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

macro_rules! define_struct {
    (
        typedef struct $name:ident {
            $($ftype:ident $fname:ident;)*
        };
    ) => {
        #[repr(C)]
        #[doc = concat!("### NTAPI: [", ::core::stringify!($name), "](https://ntdoc.m417z.com/", ::proc::lower!($name), ")")]
        #[::proc::snake_struct]
        pub struct $name {
            $(pub $fname: $ftype,)*
        }
    };
}

macro_rules! ENUM {
    ($(#[$attrs:meta])* enum $name:ident {
        $($variant:ident $(= $discr:expr)?),* $(,)?
    }) => (
        #[repr(i32)]
        #[derive(Debug, Clone)]
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
        #[derive(Clone, Copy)]
        $(#[$attrs])*
        pub union $name {
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
            #[allow(clashing_extern_declarations, reason = "Warn on tests")]
            pub safe fn $($function)*;
        }
    );
}

macro_rules! define_fn {
    // Entry: NTAPI
    ($( $return:ident NTAPI $name:ident ( VOID ) ; )+) => {
        $( define_fn!(@emit $name () -> $return); )+
    };
    ($( $return:ident NTAPI $name:ident ( $($args:tt)* ) ; )+) => {
        $( define_fn!(@munch [$name] [$return] [] $($args)*); )+
    };

    // Munch: VOID
    (@munch [$name:ident] [$return:ty] [$($acc:tt)*] VOID) => {
        define_fn!(@emit $name ($($acc)*) -> $return);
    };
    // Munch: No args
    (@munch [$name:ident] [$return:ty] [$($acc:tt)*]) => {
        define_fn!(@emit $name ($($acc)*) -> $return);
    };

    // Munch: With SAL, With *
    (@munch [$name:ident] [$return:ty] [$($acc:tt)*]
        $sal:ident $field_ty:ident * $field_name:ident, $($rest:tt)*
    ) => {
        define_fn!(@munch [$name] [$return] [$($acc)* ($field_name, *mut $field_ty)] $($rest)*);
    };
    (@munch [$name:ident] [$return:ty] [$($acc:tt)*]
        $sal:ident $field_ty:ident * $field_name:ident
    ) => {
        define_fn!(@emit $name ($($acc)* ($field_name, *mut $field_ty)) -> $return);
    };

    // Munch: With SAL, No *
    (@munch [$name:ident] [$return:ty] [$($acc:tt)*]
        $sal:ident $field_ty:ident $field_name:ident, $($rest:tt)*
    ) => {
        define_fn!(@munch [$name] [$return] [$($acc)* ($field_name, $field_ty)] $($rest)*);
    };
    (@munch [$name:ident] [$return:ty] [$($acc:tt)*]
        $sal:ident $field_ty:ident $field_name:ident
    ) => {
        define_fn!(@emit $name ($($acc)* ($field_name, $field_ty)) -> $return);
    };

    // Munch: No SAL, With *
    (@munch [$name:ident] [$return:ty] [$($acc:tt)*]
        $field_ty:ident * $field_name:ident, $($rest:tt)*
    ) => {
        define_fn!(@munch [$name] [$return] [$($acc)* ($field_name, *mut $field_ty)] $($rest)*);
    };
    (@munch [$name:ident] [$return:ty] [$($acc:tt)*]
        $field_ty:ident * $field_name:ident
    ) => {
        define_fn!(@emit $name ($($acc)* ($field_name, *mut $field_ty)) -> $return);
    };

    // Munch: No SAL, No *
    (@munch [$name:ident] [$return:ty] [$($acc:tt)*]
        $field_ty:ident $field_name:ident, $($rest:tt)*
    ) => {
        define_fn!(@munch [$name] [$return] [$($acc)* ($field_name, $field_ty)] $($rest)*);
    };
    (@munch [$name:ident] [$return:ty] [$($acc:tt)*]
        $field_ty:ident $field_name:ident
    ) => {
        define_fn!(@emit $name ($($acc)* ($field_name, $field_ty)) -> $return);
    };

    // Emit
    (@emit $name:ident ($(($fname:ident, $fty:ty))*) -> $return:ty) => {
        unsafe extern "system" {
            #[doc = concat!("### NTAPI: [", ::core::stringify!($name), "](https://ntdoc.m417z.com/", ::proc::lower!($name), ")")]
            #[link_name = ::core::stringify!($name)]
            pub fn $name($(::proc::snake_ident!($fname): $fty),*) -> $return;
        }
    };
}
