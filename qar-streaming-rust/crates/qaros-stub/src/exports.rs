//! Turns the generated `qar_impl_*` list into a trait with not-implemented
//! defaults and one `#[no_mangle]` export per function, each recording the
//! call and honouring failures injected with `qar_stub_fail_next`.

#[allow(clippy::wildcard_imports)]
use qaros_sys::*;

use crate::state;

/// Return types of the C API, with the stub's value for an unimplemented
/// function and for an injected failure.
pub trait StubReturn: Sized {
    fn not_implemented(function: &'static str) -> Self;
    fn injected(code: i64) -> Option<Self>;
}

impl StubReturn for QarResult {
    fn not_implemented(function: &'static str) -> Self {
        state::error(
            QarStatusCode::QAR_STATUS_NOT_IMPLEMENTED,
            format!("{function} is not implemented in qaros-stub"),
        )
    }

    fn injected(code: i64) -> Option<Self> {
        let raw_code = code.try_into().ok()?;
        Some(state::error(QarStatusCode(raw_code), "injected failure"))
    }
}

impl StubReturn for bool {
    fn not_implemented(_function: &'static str) -> Self {
        false
    }

    fn injected(_code: i64) -> Option<Self> {
        None
    }
}

macro_rules! zero_id_return {
    ($($id_type:ty),*) => {
        $(impl StubReturn for $id_type {
            fn not_implemented(_function: &'static str) -> Self {
                Self { data: [0; 16] }
            }

            fn injected(_code: i64) -> Option<Self> {
                None
            }
        })*
    };
}

zero_id_return!(QarPeerId, QarSessionId, QarGuiPanelId);

macro_rules! define_stub_exports {
    ($(
        pub fn $name:ident($($argument:ident : $argument_type:ty),* $(,)?) $(-> $return_type:ty)?;
    )*) => {
        /// One method per `qar_impl_*` function; defaults return
        /// `QAR_STATUS_NOT_IMPLEMENTED` (or false / zero / nothing).
        #[allow(unused_variables, clippy::unused_unit)]
        pub trait StubApi {
            $(
                unsafe fn $name($($argument: $argument_type),*) $(-> $return_type)? {
                    $(return <$return_type as StubReturn>::not_implemented(stringify!($name));)?
                }
            )*
        }

        $(
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn $name($($argument: $argument_type),*) $(-> $return_type)? {
                state::record_call(stringify!($name));
                $(
                    if let Some(code) = state::take_injected_failure(stringify!($name))
                        && let Some(value) = <$return_type as StubReturn>::injected(code)
                    {
                        return value;
                    }
                )?
                // SAFETY: the caller upholds the C API contract.
                unsafe { <crate::Stub as StubApi>::$name($($argument),*) }
            }
        )*
    };
}

qar_impl_function_list!(define_stub_exports);
