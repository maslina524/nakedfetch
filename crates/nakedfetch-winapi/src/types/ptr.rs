use crate::{
    native::rtl::RtlUserProcessParameters,
    types::primitives::{LargeInteger, ULong},
};

pub type VoidPtr = *mut ();

pub type ULongPtr = *mut ULong;
pub type LargeIntegerPtr = *mut LargeInteger;

pub type IoApcRoutinePtr = *mut ();
pub type HeapPtr = *mut ();
pub type PebLdrDataRef = &'static ();
pub type RtlUserProcessParametersRef = &'static RtlUserProcessParameters;
