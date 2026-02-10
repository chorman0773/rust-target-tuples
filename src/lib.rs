#![deny(warnings, unsafe_code)]
#![cfg_attr(not(any(doc, test)), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod pieces;

use core::{ops::Deref, str::FromStr};

use pieces::*;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct CanonicalTarget {
    pub arch: Architecture,
    pub vendor: Option<Vendor>,
    pub sys: System,
}

impl CanonicalTarget {
    pub fn guess_vendor(&self) -> Vendor {
        if let Some(vendor) = self.vendor {
            return vendor;
        }
        match (self.arch, self.sys.os()) {
            (
                Architecture::X86_16(_) | Architecture::X86_32(_) | Architecture::X86_64 { .. },
                _,
            ) => Vendor::PC,
            (Architecture::Wc65c816, _) => Vendor::WDC,
            _ => Vendor::Unknown,
        }
    }
}

impl core::str::FromStr for CanonicalTarget {
    type Err = UnknownError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (arch, rest) = s.split_once('-').ok_or(UnknownError)?;

        let arch = arch.parse::<Architecture>()?;

        let (vendor, sys) = if let Some((a, b)) = rest.split_once('-') {
            if b.contains('-') {
                // 4 component, this is vendor-os-env
                (Some(Vendor::parse(a)), b.parse::<System>()?)
            } else if let Ok(sys) = rest.parse::<System>() {
                (None, sys)
            } else {
                (Some(Vendor::parse(a)), b.parse::<System>()?)
            }
        } else {
            (None, rest.parse::<System>()?)
        };

        Ok(CanonicalTarget { arch, vendor, sys })
    }
}

impl core::fmt::Display for CanonicalTarget {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.arch.fmt(f)?;
        f.write_str("-")?;

        self.guess_vendor().fmt(f)?;

        f.write_str("-")?;

        self.sys.fmt(f)
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct TargetRef<'a> {
    exact: &'a str,
    canon: CanonicalTarget,
}

impl<'a> Deref for TargetRef<'a> {
    type Target = CanonicalTarget;

    fn deref(&self) -> &CanonicalTarget {
        &self.canon
    }
}

impl<'a> TargetRef<'a> {
    pub fn try_parse(st: &'a str) -> Result<Self, UnknownError> {
        let canon = CanonicalTarget::from_str(st)?;

        Ok(TargetRef { exact: st, canon })
    }

    pub fn parse(st: &'a str) -> Self {
        match Self::try_parse(st) {
            Ok(targ) => targ,
            Err(_) => panic!("Unknown target: {}", st),
        }
    }

    pub fn exact(&self) -> &'a str {
        self.exact
    }

    pub fn canonical(&self) -> CanonicalTarget {
        self.canon
    }
}

#[cfg(feature = "alloc")]
mod feature_alloc {
    use core::{ops::Deref, str::FromStr};

    use crate::{pieces::UnknownError, CanonicalTarget, TargetRef};
    use alloc::string::{String, ToString};

    #[derive(Clone, Debug, Hash, PartialEq, Eq)]
    pub struct OwnedTarget {
        exact: String,
        canon: CanonicalTarget,
    }

    impl<'a> Deref for OwnedTarget {
        type Target = CanonicalTarget;

        fn deref(&self) -> &CanonicalTarget {
            &self.canon
        }
    }

    impl OwnedTarget {
        pub fn from_canonical(canon: CanonicalTarget) -> Self {
            let exact = canon.to_string();

            Self { exact, canon }
        }
        pub fn from_owned(st: String) -> Result<Self, UnknownError> {
            let canon = CanonicalTarget::from_str(&st)?;

            Ok(OwnedTarget { exact: st, canon })
        }

        pub fn into_exact(self) -> String {
            self.exact
        }

        pub fn borrow<'a>(&'a self) -> TargetRef<'a> {
            TargetRef {
                exact: &self.exact,
                canon: self.canon,
            }
        }
    }

    impl FromStr for OwnedTarget {
        type Err = UnknownError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::from_owned(s.to_string())
        }
    }
}

#[cfg(feature = "alloc")]
pub use feature_alloc::*;

#[doc(hidden)]
pub use core as __core;

#[doc(hidden)]
pub use target_tuples_macro::__match_targets;

#[macro_export]
macro_rules! match_targets {
    {
        $expr:tt {
            $($inner:tt)*
        }
    } => {
        $crate::__match_targets!([$crate] $expr {
            $($inner)*
        })
    };
}
