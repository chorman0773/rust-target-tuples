#![deny(warnings, unsafe_code)]
#![cfg_attr(not(any(doc, test)), no_std)]

extern crate alloc;

mod pieces;

pub use pieces::*;

#[doc(hidden)]
#[macro_export]
macro_rules! __to_target {
    ($first:ident-$($rest:ident)-+) => {
        ::core::concat!(::core::stringify!($first) $(,"-",::core::stringify!($rest))*)
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __match_target_pattern {
    ($arch:ident-$vendor:ident-$os:ident-$env:ident) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!($arch - $vendor - $os - $env));
            *targ == mtarg
        }

        __check
    }};

    ($arch:ident-$vendor:ident-$sys:ident) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!($arch - $vendor - $sys));
            targ == mtarg
        }

        __check
    }};

    (*-$vendor:ident-$os:ident-$env:ident) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!(x86_64 - $vendor - $os - $env));

            targ.vendor() == mtarg.vendor()
                && targ.operating_system() == mtarg.operating_system()
                && targ.environment() == mtarg.environment()
                && targ.object_format() == mtarg.object_format()
        }

        __check
    }};
    (*-$vendor:ident-$sys:ident) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!(x86_64 - $vendor - $sys));

            targ.vendor() == mtarg.vendor()
                && targ.operating_system() == mtarg.operating_system()
                && targ.environment() == mtarg.environment()
                && targ.object_format() == mtarg.object_format()
        }

        __check
    }};

    ($arch:ident-*-$os:ident-$env:ident) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!($arch - unknown - $os - $env));

            targ.arch() == mtarg.arch()
                && targ.operating_system() == mtarg.operating_system()
                && targ.environment() == mtarg.environment()
                && targ.object_format() == mtarg.object_format()
        }

        __check
    }};
    ($arch:ident-*-$sys:ident) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!($arch - unknown - $sys));

            targ.arch() == mtarg.arch()
                && targ.operating_system() == mtarg.operating_system()
                && targ.environment() == mtarg.environment()
                && targ.object_format() == mtarg.object_format()
        }

        __check
    }};

    ($arch:ident-$vendor:ident-*-$env:ident) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!($arch - $vendor - none - $env));

            targ.arch() == mtarg.arch()
                && targ.vendor() == mtarg.vendor()
                && targ.environment() == mtarg.environment()
                && targ.object_format() == mtarg.object_format()
        }

        __check
    }};

    ($arch:ident-$vendor:ident-*) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!($arch - $vendor - elf));

            targ.arch() == mtarg.arch() && targ.vendor() == mtarg.vendor()
        }

        __check
    }};

    ($arch:ident-$vendor:ident-$os:ident-*) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!($arch - $vendor - $os - elf));

            targ.vendor() == mtarg.vendor() && targ.operating_system() == mtarg.operating_system()
        }

        __check
    }};

    (*-*-$os:ident-$env:ident) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!(x86_64 - unknown - $os - $env));

            targ.operating_system() == mtarg.operating_system()
                && targ.environment() == mtarg.environment()
                && targ.object_format() == mtarg.object_format()
        }

        __check
    }};
    (*-*-$sys:ident) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!(x86_64 - unknown - $sys));

            targ.operating_system() == mtarg.operating_system()
                && targ.environment() == mtarg.environment()
                && targ.object_format() == mtarg.object_format()
        }

        __check
    }};

    (*-$vendor:ident-*-$env:ident) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!(x86_64 - $vendor - none - $env));

            targ.vendor() == mtarg.vendor()
                && targ.environment() == mtarg.environment()
                && targ.object_format() == mtarg.object_format()
        }

        __check
    }};

    (*-$vendor:ident-*) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!(x86_64 - $vendor - elf));

            targ.vendor() == mtarg.vendor()
        }

        __check
    }};

    (*-$vendor:ident-$os:ident-*) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!(x86_64 - $vendor - $os - elf));

            targ.vendor() == mtarg.vendor() && targ.operating_system() == mtarg.operating_system()
        }

        __check
    }};

    ($arch:ident-*-*-$env:ident) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!($arch - unknown - none - $env));

            targ.arch() == mtarg.arch()
                && targ.environment() == mtarg.environment()
                && targ.object_format() == mtarg.object_format()
        }

        __check
    }};

    ($arch:ident-*-*) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!($arch - unknown - elf));

            targ.arch() == mtarg.arch()
        }

        __check
    }};

    ($arch:ident-*-$os:ident-*) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!($arch - unknown - $os - elf));

            targ.arch() == mtarg.arch() && targ.operating_system() == mtarg.operating_system()
        }

        __check
    }};

    (*-*-*-$env:ident) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!(x86_64 - unknown - none - $env));

            targ.environment() == mtarg.environment()
                && targ.object_format() == mtarg.object_format()
        }

        __check
    }};

    (*-*-$os:ident-*) => {{
        fn __check(targ: &$crate::Target) -> bool {
            let mtarg = $crate::Target::parse($crate::__to_target!(x86_64 - unknown - $os - elf));

            targ.operating_system() == mtarg.operating_system()
        }

        __check
    }};

    (*-*-*) => {{
        fn __check(_: &$crate::Target) -> bool {
            true
        }
        __check
    }};

    (*) => {{
        fn __check(_: &$crate::Target) -> bool {
            true
        }
        __check
    }};
}

#[macro_export]
macro_rules! match_targets{
    {
        match ($targ:expr) {
            $($($comp:tt)-* => $exp:expr),* $(,)?
        }
    } => {
        {
            let __val: &$crate::Target = &$targ;
            #[allow(unreachable_code)]
            loop {
                $(if ($crate::__match_target_pattern!($($comp)-*))(&__val){
                    break $exp
                })*

                unreachable!("Incomplete Exhaustive Target Pattern (add a wildcard match as * => )")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Target;

    #[test]
    pub fn test_match_easy() {
        let target = Target::parse("x86_64-pc-linux-gnu");
        match_targets! {
            match (target) {
                x86_64-pc-linux-gnu => {},
                * => panic!("Invalid Target")
            }
        }
    }

    #[test]
    pub fn test_match_ref() {
        let target = Target::parse("x86_64-pc-linux-gnu");
        match_targets! {
            match (&target) {
                x86_64-pc-linux-gnu => {},
                * => panic!("Invalid Target")
            }
        }
    }

    #[test]
    pub fn target_match_arch_wildcard() {
        let target = Target::parse("x86_64-pc-linux-gnu");
        match_targets! {
            match (target) {
                *-pc-linux-gnu => {},
                * => panic!("Invalid Target")
            }
        }
    }

    #[test]
    pub fn target_match_vendor_wildcard() {
        let target = Target::parse("x86_64-pc-linux-gnu");
        match_targets! {
            match (target) {
                x86_64-*-linux-gnu => {},
                * => panic!("Invalid Target")
            }
        }
    }

    #[test]
    pub fn target_match_os_wildcard() {
        let target = Target::parse("x86_64-pc-linux-gnu");
        match_targets! {
            match (target) {
                x86_64-pc-*-gnu => {},
                * => panic!("Invalid Target")
            }
        }
    }

    #[test]
    pub fn target_match_env_wildcard() {
        let target = Target::parse("x86_64-pc-linux-gnu");
        match_targets! {
            match (target) {
                x86_64-pc-linux-* => {},
                * => panic!("Invalid Target")
            }
        }
    }

    #[test]
    pub fn target_match_sys_wildcard() {
        let target = Target::parse("x86_64-pc-linux-gnu");
        match_targets! {
            match (target) {
                x86_64-pc-* => {},
                * => panic!("Invalid Target")
            }
        }
    }

    #[test]
    pub fn target_match_first() {
        let target = Target::parse("x86_64-pc-linux-gnu");
        match_targets! {
            match (target) {
                x86_64-pc-linux-gnu => {},
                x86_64-*-linux-* => panic!("Incorrect Match"),
                * => panic!("Incorrect Match"),
            }
        }
    }

    #[test]
    pub fn target_match_ignore_wrong_matches() {
        let target = Target::parse("x86_64-pc-linux-gnu");
        match_targets! {
            match (target){
                i686-*-linux-gnu => panic!("Incorrect Match"),
                x86_64-*-linux-gnu => {},
                * => panic!("Incorrect Match")
            }
        }
    }
}
