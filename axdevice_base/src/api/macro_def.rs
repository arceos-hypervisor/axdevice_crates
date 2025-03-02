macro_rules! def_device_api {
    (
        $name:ident {
            $($(#[$fn_meta:meta])* fn $fn_name:ident($($arg_name:ident: $arg_type:ty),*) $(-> $ret_type:ty)?;)*
        }

        $($rest:tt)*
    ) => {
        ::paste::paste! {
            #[crate_interface::def_interface]
            #[doc(hidden)]
            pub trait [< $name:camel ApiTrait >] {
                $(
                    $(#[$fn_meta])*
                    fn $fn_name($($arg_name: $arg_type),*) $(-> $ret_type)?;
                )*
            }

            $(
                $(#[$fn_meta])*
                #[inline]
                pub fn $fn_name($($arg_name: $arg_type),*) $(-> $ret_type)? {
                    crate_interface::call_interface!([< $name:camel ApiTrait >]::$fn_name($($arg_name),*))
                }
            )*
        }

        $crate::api::macro_def::def_device_api! { $($rest)* }
    };
    () => {}
}

#[macro_export]
macro_rules! impl_device_api {
    ($name:ident @ $p:path {
        $($(#[$fn_meta:meta])* fn $fn_name:ident($($arg_name:ident: $arg_type:ty),*) $(-> $ret_type:ty)? $body:block)*
    }) => {
        $crate::api::__paste! {
            const _:() = {
                use $p::*;
                struct __Impl;

                #[crate_interface::impl_interface]
                impl $p::[< $name:camel ApiTrait >] for __Impl {
                    $(
                        $(#[$fn_meta])*
                        fn $fn_name($($arg_name: $arg_type),*) $(-> $ret_type)? $body
                    )*
                }

                ()
            };
        }
    };
}

pub(super) use def_device_api;
