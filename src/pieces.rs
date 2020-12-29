use std::{fmt::Display, str::FromStr};

///
/// The result of FromStr::from_str, when parsing a field (other than vendor),
///  with a value that is not known to the library
#[derive(Debug, Clone, Copy)]
pub struct UnknownError;

///
/// The Architecture field of a target tuple
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Architecture {
    Unknown,
    X86,
    X86_64,
    Arm,
    ArmBe,
    Aarch64,
    Aarch64Be,
    Aarch64_32,
    Mips,
    MipsLE,
    Mips64,
    Mips64LE,
    PowerPC32,
    PowerPC64,
    PowerPC64le,
    RiscV32,
    RiscV64,
    Sparc,
    SparcV9,
    SparcEL,
    Wasm32,
    Wasm64,
    Wc65c816,
}

impl FromStr for Architecture {
    type Err = UnknownError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "i386" | "i486" | "i586" | "i686" | "i786" | "i886" | "i986" => Self::X86,
            "amd64" | "x86_64" | "x86_64h" => Self::X86_64,
            "armeb" => Self::ArmBe,
            "arm" => Self::Arm,
            "aarch64" | "arm64" | "arm64e" => Self::Aarch64,
            "aarch64_be" | "arm64_be" => Self::Aarch64Be,
            "aarch64_32" | "arm64_32" => Self::Aarch64_32,
            "powerpc" | "powerpcspe" | "ppc" | "ppc32" => Self::PowerPC32,
            "powerpc64" | "ppu" | "ppc64" => Self::PowerPC64,
            "powerpc64le" | "ppc64le" => Self::PowerPC64le,
            "mips" | "mipseb" | "mipsallegrex" | "mipsisa32r6" | "mipsr6" => Self::Mips,
            "mipsel" | "mipsallegrexel" | "mipsisa32r6el" | "mipsr6el" => Self::MipsLE,
            "mips64" | "mips64eb" | "mipsn32" | "mipsisa64r6" | "mips64r6" | "mipsn32r6" => {
                Self::Mips64
            }
            "mips64el" | "mipsn32el" | "mipsisa64r6el" | "mips64r6el" | "mipsn32r6el" => {
                Self::Mips64LE
            }
            "sparc" => Self::Sparc,
            "sparcel" => Self::SparcEL,
            "sparcv9" | "sparc64" => Self::SparcV9,
            "riscv32" => Self::RiscV32,
            "riscv64" => Self::RiscV64,
            "wc65c816" | "65816" | "w65c816" | "65c816" => Self::Wc65c816,
            "wasm32" => Self::Wasm32,
            "wasm64" => Self::Wasm64,

            _ => return Err(UnknownError),
        })
    }
}

impl Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.canonical_name().fmt(f)
    }
}

impl Architecture {
    /// Parses the Architecture in a "lossy" manner
    /// This is equivalent to [`Self::from_str`], but returns [`Architecture::Unknown`], instead of an error,
    ///  on an unknown architecture.
    /// This is useful (in conjunction with an actual target name)
    /// ## Example
    /// ```
    ///     use target_tuples::Architecture;
    ///     let arch = Architecture::parse("i386");
    ///     assert_eq!(arch,Architecture::X86);
    ///     let arch2: Architecture = "i486".parse().unwrap();
    ///     assert_eq!(arch,arch2);
    /// ```
    pub fn parse(st: &str) -> Self {
        Self::from_str(st).unwrap_or(Architecture::Unknown)
    }

    ///
    /// Returns the canonical name of the target
    /// The canonical name, when passed into `[`Self::parse`] will yield an equivalent value,
    /// Formatting an Architecture yields this string
    /// ## Examples
    /// ```
    ///    use target_tuples::Architecture;
    ///    let arch = Architecture::X86;
    ///    assert_eq!(Architecture::parse(arch.canonical_name()),arch);
    /// ```
    pub fn canonical_name(&self) -> &'static str {
        match self {
            Architecture::Unknown => "unknown",
            Architecture::X86 => "i386",
            Architecture::X86_64 => "x86_64",
            Architecture::Arm => "arm",
            Architecture::ArmBe => "armeb",
            Architecture::Aarch64 => "aarch64",
            Architecture::Aarch64Be => "aarch64_be",
            Architecture::Aarch64_32 => "aarch64_32",
            Architecture::Mips => "mips",
            Architecture::Mips64 => "mips64",
            Architecture::PowerPC32 => "powerpc",
            Architecture::PowerPC64 => "powerpc64",
            Architecture::PowerPC64le => "powerpc64le",
            Architecture::RiscV32 => "riscv32",
            Architecture::RiscV64 => "riscv64",
            Architecture::Sparc => "sparc",
            Architecture::SparcV9 => "sparcv9",
            Architecture::SparcEL => "sparcel",
            Architecture::Wasm32 => "wasm32",
            Architecture::Wasm64 => "wasm64",
            Architecture::Wc65c816 => "wc65c816",
            Architecture::MipsLE => "mipsel",
            Architecture::Mips64LE => "mips64el",
        }
    }
}

///
/// The Vendor field of a target tuple
///
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Vendor {
    Unknown,
    Apple,
    PC,
    SNES,
    SCEI,
    Freescale,
    IBM,
    ImaginationTechnologies,
    MipsTechnologies,
    NVIDIA,
    CSR,
    Myriad,
    AMD,
    Mesa,
    SUSE,
    OpenEmbedded,
}

impl FromStr for Vendor {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "apple" => Self::Apple,
            "pc" => Self::PC,
            "snes" | "snesdev" => Self::SNES,
            "scei" => Self::SCEI,
            "fsl" => Self::Freescale,
            "img" => Self::ImaginationTechnologies,
            "ibm" => Self::IBM,
            "mti" => Self::MipsTechnologies,
            "nvidia" => Self::NVIDIA,
            "csr" => Self::CSR,
            "myriad" => Self::Myriad,
            "amd" => Self::AMD,
            "mesa" => Self::Mesa,
            "suse" => Self::SUSE,
            "oe" => Self::OpenEmbedded,
            _ => Self::Unknown,
        })
    }
}

impl Display for Vendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.canonical_name().fmt(f)
    }
}

impl Vendor {
    /// Parses the Vendor in a "lossy" manner
    /// This is equivalent to [`Self::from_str`].
    /// Note that an unknown vendor is not considered an error.
    ///
    /// ## Example
    /// ```
    ///     use target_tuples::Vendor;
    ///     let vendor = Vendor::parse("pc");
    ///     assert_eq!(vendor,Vendor::PC);
    ///     let vendor2: Vendor = "pc".parse().unwrap();
    ///     assert_eq!(vendor,vendor2);
    /// ```
    pub fn parse(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }

    ///
    /// Returns the canonical name of the vendor
    /// The canonical name, when passed into `[`Self::parse`] will yield an equivalent value,
    /// Formatting a Vendor yields this string
    /// ## Examples
    /// ```
    ///    use target_tuples::Vendor;
    ///    let vendor = Vendor::Apple;
    ///    assert_eq!(Vendor::parse(vendor.canonical_name()),vendor);
    /// ```
    pub fn canonical_name(&self) -> &'static str {
        match self {
            Self::Apple => "apple",
            Self::PC => "pc",
            Self::SNES => "snes",
            Self::Unknown => "unknown",
            Vendor::SCEI => "scei",
            Vendor::Freescale => "fsl",
            Vendor::IBM => "ibm",
            Vendor::ImaginationTechnologies => "img",
            Vendor::MipsTechnologies => "mti",
            Vendor::NVIDIA => "nvidia",
            Vendor::CSR => "csr",
            Vendor::Myriad => "myriad",
            Vendor::AMD => "amd",
            Vendor::Mesa => "mesa",
            Vendor::SUSE => "suse",
            Vendor::OpenEmbedded => "oe",
        }
    }
}

///
/// The Operating System Field of a target tuple
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum OS {
    Unknown,

    Ananas,
    CloudABI,
    Darwin,
    DragonFly,
    FreeBSD,
    Fuchsia,
    IOS,
    KFreeBSD,
    Linux,
    Lv2,
    MacOSX,
    NetBSD,
    OpenBSD,
    Solaris,
    Win32,
    ZOS,
    Haiku,
    Minix,
    RTEMS,
    NaCl,
    AIX,
    CUDA,
    NVCL,
    AMDHSA,
    PS4,
    ELFIAMCU,
    TvOS,
    WatchOS,
    Mesa3D,
    Contiki,
    AMDPAL,
    HermitCore,
    Hurd,
    WASI,
    Emscripten,
    PhantomOS,
}

impl FromStr for OS {
    type Err = UnknownError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            x if x.starts_with("ananas") => Self::Ananas,
            x if x.starts_with("cloudabi") => Self::CloudABI,
            x if x.starts_with("darwin") => Self::Darwin,
            x if x.starts_with("dragonfly") => Self::DragonFly,
            x if x.starts_with("freebsd") => Self::FreeBSD,
            x if x.starts_with("fuchsia") => Self::Fuchsia,
            x if x.starts_with("ios") => Self::IOS,
            x if x.starts_with("kfreebsd") => Self::KFreeBSD,
            x if x.starts_with("linux") => Self::Linux,
            x if x.starts_with("lv2") => Self::Lv2,
            x if x.starts_with("macos") => Self::MacOSX,
            x if x.starts_with("netbsd") => Self::NetBSD,
            x if x.starts_with("openbsd") => Self::OpenBSD,
            x if x.starts_with("solaris") => Self::Solaris,
            x if x.starts_with("win32") | x.starts_with("windows") => Self::Win32,
            x if x.starts_with("zos") => Self::ZOS,
            x if x.starts_with("haiku") => Self::Haiku,
            x if x.starts_with("minix") => Self::Minix,
            x if x.starts_with("rtems") => Self::RTEMS,
            x if x.starts_with("nacl") => Self::NaCl,
            x if x.starts_with("aix") => Self::AIX,
            x if x.starts_with("cuda") => Self::CUDA,
            x if x.starts_with("nvcl") => Self::NVCL,
            x if x.starts_with("amdhsa") => Self::AMDHSA,
            x if x.starts_with("ps4") => Self::PS4,
            x if x.starts_with("elfiamcu") => Self::ELFIAMCU,
            x if x.starts_with("tvos") => Self::TvOS,
            x if x.starts_with("watchos") => Self::WatchOS,
            x if x.starts_with("mesa3d") => Self::Mesa3D,
            x if x.starts_with("contiki") => Self::Contiki,
            x if x.starts_with("amdpal") => Self::AMDPAL,
            x if x.starts_with("hermit") => Self::HermitCore,
            x if x.starts_with("hurd") => Self::Hurd,
            x if x.starts_with("wasi") => Self::WASI,
            x if x.starts_with("emscripten") => Self::Emscripten,
            x if x.starts_with("phantom") => Self::PhantomOS,

            _ => return Err(UnknownError),
        })
    }
}

impl Display for OS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.canonical_name().fmt(f)
    }
}

impl OS {
    /// Parses the OS in a "lossy" manner
    /// This is equivalent to [`Self::from_str`], except that [`OS::Unknown`] is returned, instead of an error, on an unknown OS Field
    ///
    /// ## Example
    /// ```
    ///     use target_tuples::OS;
    ///     let os = OS::parse("linux");
    ///     assert_eq!(os,OS::Linux);
    ///     let os2: OS = "linux".parse().unwrap();
    ///     assert_eq!(os,os2);
    /// ```
    pub fn parse(s: &str) -> Self {
        Self::from_str(s).unwrap_or(Self::Unknown)
    }

    ///
    /// Returns the canonical name of the operating system
    /// The canonical name, when passed into `[`Self::parse`] will yield an equivalent value,
    /// Formatting an OS yields this string
    /// ## Examples
    /// ```
    ///    use target_tuples::OS;
    ///    let os = OS::PS4;
    ///    assert_eq!(OS::parse(os.canonical_name()),os);
    /// ```
    pub fn canonical_name(&self) -> &'static str {
        match self {
            OS::Unknown => "unknown",
            OS::Ananas => "ananas",
            OS::CloudABI => "cloudabi",
            OS::Darwin => "darwin",
            OS::DragonFly => "dragonfly",
            OS::FreeBSD => "freebsd",
            OS::Fuchsia => "fuchsia",
            OS::IOS => "ios",
            OS::KFreeBSD => "kfreebsd",
            OS::Linux => "linux",
            OS::Lv2 => "lv2",
            OS::MacOSX => "macos",
            OS::NetBSD => "netbsd",
            OS::OpenBSD => "openbsd",
            OS::Solaris => "solaris",
            OS::Win32 => "win32",
            OS::ZOS => "zos",
            OS::Haiku => "haiku",
            OS::Minix => "minix",
            OS::RTEMS => "rtems",
            OS::NaCl => "nacl",
            OS::AIX => "aix",
            OS::CUDA => "cuda",
            OS::NVCL => "nvcl",
            OS::AMDHSA => "amdhsa",
            OS::PS4 => "ps4",
            OS::ELFIAMCU => "elfiamcu",
            OS::TvOS => "tvos",
            OS::WatchOS => "watchos",
            OS::Mesa3D => "mesa3d",
            OS::Contiki => "contiki",
            OS::AMDPAL => "amdpal",
            OS::HermitCore => "hermit",
            OS::Hurd => "hurd",
            OS::WASI => "wasi",
            OS::Emscripten => "emscripten",
            OS::PhantomOS => "phantom",
        }
    }
}

///
/// The Environment field of target tuples
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum Environment {
    Unknown,
    GNU,
    GNUABIN32,
    GNUABI64,
    GNUEABI,
    GNUEABIHF,
    GNUX32,
    CODE16,
    EABI,
    EABIHF,
    Android,
    Musl,
    MuslEABI,
    MuslEABIHF,

    MSVC,
    Itanium,
    Cygnus,
    CoreCLR,
    Simulator,
    MacABI,

    PhantomStandard,
    PhantomKernel,
}

impl FromStr for Environment {
    type Err = UnknownError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            x if x.starts_with("eabihf") => Self::EABIHF,
            x if x.starts_with("eabi") => Self::EABI,
            x if x.starts_with("gnuabin32") => Self::GNUABIN32,
            x if x.starts_with("gnuabi64") => Self::GNUABI64,
            x if x.starts_with("gnueabihf") => Self::GNUEABIHF,
            x if x.starts_with("gnueabi") => Self::GNUEABI,
            x if x.starts_with("gnux32") => Self::GNUX32,
            x if x.starts_with("gnu") => Self::GNU,
            x if x.starts_with("code16") => Self::CODE16,
            x if x.starts_with("android") => Self::Android,
            x if x.starts_with("musleabihf") => Self::MuslEABIHF,
            x if x.starts_with("musleabi") => Self::MuslEABI,
            x if x.starts_with("musl") => Self::Musl,
            x if x.starts_with("msvc") => Self::MSVC,
            x if x.starts_with("itanium") => Self::Itanium,
            x if x.starts_with("cygnus") => Self::Cygnus,
            x if x.starts_with("coreclr") => Self::CoreCLR,
            x if x.starts_with("simulator") => Self::Simulator,
            x if x.starts_with("macabi") => Self::MacABI,
            x if x.starts_with("pstd") || x.starts_with("standard") => Self::PhantomStandard,
            x if x.starts_with("pkrnl") || x.starts_with("kernel") => Self::PhantomKernel,
            _ => return Err(UnknownError),
        })
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.canonical_name().fmt(f)
    }
}

impl Environment {
    /// Parses the Environment name in a "lossy" manner
    /// This is equivalent to [`Self::from_str`], except that [`Environment::Unknown`] is returned, instead of an error, on an unknown OS Field
    ///
    /// ## Example
    /// ```
    ///     use target_tuples::Environment;
    ///     let env = Environment::parse("gnu");
    ///     assert_eq!(env,Environment::GNU);
    ///     let env2: Environment = "gnu".parse().unwrap();
    ///     assert_eq!(env,env2);
    /// ```
    pub fn parse(s: &str) -> Self {
        Self::from_str(s).unwrap_or(Self::Unknown)
    }

    ///
    /// Returns the canonical name of the environment
    /// The canonical name, when passed into [`Self::parse`] will yield an equivalent value,
    /// Formatting an Environment yields this string
    /// ## Examples
    /// ```
    ///    use target_tuples::Environment;
    ///    let os = Environment::Musl;
    ///    assert_eq!(Environment::parse(os.canonical_name()),os);
    /// ```
    pub fn canonical_name(&self) -> &'static str {
        match self {
            Environment::Unknown => "unknown",
            Environment::GNU => "gnu",
            Environment::GNUABIN32 => "gnuabin32",
            Environment::GNUABI64 => "gnuabi64",
            Environment::GNUEABI => "gnueabi",
            Environment::GNUEABIHF => "gnueabihf",
            Environment::GNUX32 => "gnux32",
            Environment::CODE16 => "code16",
            Environment::EABI => "eabi",
            Environment::EABIHF => "eabihf",
            Environment::Android => "android",
            Environment::Musl => "musl",
            Environment::MuslEABI => "musleabi",
            Environment::MuslEABIHF => "musleabihf",
            Environment::MSVC => "msvc",
            Environment::Itanium => "itanium",
            Environment::Cygnus => "cygnus",
            Environment::CoreCLR => "coreclr",
            Environment::Simulator => "simulator",
            Environment::MacABI => "macabi",
            Environment::PhantomStandard => "pstd",
            Environment::PhantomKernel => "pkrnl",
        }
    }
}

///
/// The object format used by a target
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum ObjectFormat {
    Unknown,
    XCoff,
    Coff,
    Elf,
    Goff,
    MachO,
    Wasm,
}

impl FromStr for ObjectFormat {
    type Err = UnknownError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            x if x.ends_with("xcoff") => Self::XCoff,
            x if x.ends_with("coff") => Self::Coff,
            x if x.ends_with("elf") => Self::Elf,
            x if x.ends_with("goff") => Self::Goff,
            x if x.ends_with("macho") => Self::MachO,
            x if x.ends_with("wasm") => Self::Wasm,
            _ => return Err(UnknownError),
        })
    }
}

impl Display for ObjectFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.canonical_name().fmt(f)
    }
}

impl ObjectFormat {
    /// Parses the ObjectFormat name in a "lossy" manner, from the end of the Environment field
    /// This is equivalent to [`Self::from_str`], except that [`ObjectFormat::Unknown`] is returned, instead of an error, on an unknown OS Field
    ///
    /// ## Example
    /// ```
    ///     use target_tuples::ObjectFormat;
    ///     let of = ObjectFormat::parse("gnuelf");
    ///     assert_eq!(of,ObjectFormat::Elf);
    ///     let of2: ObjectFormat = "pstdelf".parse().unwrap();
    ///     assert_eq!(of,of2);
    /// ```
    pub fn parse(s: &str) -> Self {
        Self::from_str(s).unwrap_or(Self::Unknown)
    }

    ///
    /// Returns the canonical name of the object format
    /// The canonical name, when passed into [`Self::parse`] will yield an equivalent value,
    /// Formatting an ObjectFormat yields this string
    /// ## Examples
    /// ```
    ///    use target_tuples::ObjectFormat;
    ///    let os = ObjectFormat::MachO;
    ///    assert_eq!(ObjectFormat::parse(os.canonical_name()),os);
    /// ```
    ///

    pub fn canonical_name(&self) -> &'static str {
        match self {
            ObjectFormat::Unknown => "unknown",
            ObjectFormat::XCoff => "xcoff",
            ObjectFormat::Coff => "coff",
            ObjectFormat::Elf => "elf",
            ObjectFormat::Goff => "goff",
            ObjectFormat::MachO => "macho",
            ObjectFormat::Wasm => "wasm",
        }
    }
}

///
/// The representation of a target tuple.
///
/// A Target Tuple is of the form arch-vendor-system, where system can be either os-env
///  or simply either os or env (the latter is used in the case of a freestanding target).
///
/// There are two types of target tuple: canonical and exact.
/// This type can be used to represent both.
///
/// The [`core::fmt::Display`] implementation will display the canonical tuple;
///  the function [`Self::get_name`] extracts the exact form that was parsed.
/// In any case, if any field, other than vendor, is unknown, or the form is not the one above,
///  the [`core::str::FromStr`] implementation will yield an UnknownError.
///
#[derive(Clone, Debug)]
pub struct Target {
    full: std::string::String,
    arch: Architecture,
    vendor: Vendor,
    // Invariant:
    // At least one of these fields is Some
    os: Option<OS>,
    env: Option<Environment>,
    objfmt: Option<ObjectFormat>,
}

impl FromStr for Target {
    type Err = UnknownError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        let arch = split.next().ok_or(UnknownError).and_then(|s| s.parse())?;
        let vendor = split
            .next()
            .ok_or(UnknownError)
            .and_then(|s| s.parse().map_err(|e| match e {}))?;
        let f3 = split.next().ok_or(UnknownError)?;
        let f4 = split.next();
        let os;
        let env;
        let objfmt;
        if let Some(s) = f4 {
            os = Some(f3.parse()?);
            env = s.parse().ok();
            objfmt = s.parse().ok();
            env.map(|_| ())
                .or_else(|| objfmt.map(|_| ()))
                .ok_or(UnknownError)?;
        } else if let Ok(o) = f3.parse() {
            os = Some(o);
            env = None;
            objfmt = None;
        } else if let Ok(e) = f3.parse() {
            os = None;
            env = Some(e);
            objfmt = f3.parse().ok();
        } else if let Ok(of) = f3.parse() {
            os = None;
            env = None;
            objfmt = Some(of);
        } else {
            return Err(UnknownError);
        }

        Ok(Self {
            full: s.to_owned(),
            arch,
            vendor,
            os,
            env,
            objfmt,
        })
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.arch.fmt(f)?;
        f.write_str("-")?;
        self.vendor.fmt(f)?;
        if let Some(os) = &self.os {
            f.write_str("-")?;
            os.fmt(f)?;
        }
        let mut last_field_sep = true;
        if let Some(env) = &self.env {
            last_field_sep = false;
            f.write_str("-")?;
            env.fmt(f)?;
        }
        if let Some(objfmt) = &self.objfmt {
            if last_field_sep {
                f.write_str("-")?;
            }
            objfmt.fmt(f)?;
        }
        Ok(())
    }
}

impl Target {
    ///
    /// Gets the exact name of the target tuple.
    pub fn get_name(&self) -> &str {
        &*self.full
    }

    /// Parses a target tuple of the form arch-vendor-system (where system is either os-env, os, or env).
    /// If a field is not known, it is left as unknown, and the original value will be available
    ///  through the exact name.
    ///
    /// Panics if s is not of the above form
    pub fn parse(s: &str) -> Self {
        let mut split = s.split('-');
        let arch = Architecture::parse(split.next().unwrap());
        let vendor = split
            .next()
            .ok_or(UnknownError)
            .and_then(|s| s.parse().map_err(|e| match e {}))
            .unwrap();
        let f3 = split.next().unwrap();
        let f4 = split.next();
        let os;
        let env;
        let objfmt;
        if let Some(s) = f4 {
            os = Some(f3.parse().unwrap_or(OS::Unknown));
            env = Some(s.parse().unwrap_or(Environment::Unknown));
            objfmt = s.parse().ok();
        } else if let Ok(o) = f3.parse() {
            os = Some(o);
            env = None;
            objfmt = None;
        } else if let Ok(e) = f3.parse() {
            os = None;
            env = Some(e);
            objfmt = f3.parse().ok();
        } else if let Ok(of) = f3.parse() {
            os = None;
            env = None;
            objfmt = Some(of);
        } else {
            os = Some(OS::Unknown);
            env = Some(Environment::Unknown);
            objfmt = None;
        }

        Self {
            full: s.to_owned(),
            arch,
            vendor,
            os,
            env,
            objfmt,
        }
    }

    ///
    /// Gets the value of the `os` field, or unknown if the os was omitted
    pub fn get_operating_system(&self) -> OS {
        self.os.unwrap_or(OS::Unknown)
    }

    ///
    /// Gets the value of the `env` field, or unknown if the environment was omitted
    pub fn get_environment(&self) -> Environment {
        self.env.unwrap_or(Environment::Unknown)
    }

    /// 
    /// Constructs a target tuple in canonical form from the specified components.
    pub fn from_components(
        arch: Architecture,
        vendor: Vendor,
        os: Option<OS>,
        env: Option<Environment>,
        objfmt: Option<ObjectFormat>,
    ) -> Self {
        let mut ret = Self {
            full: String::new(),
            arch,
            vendor,
            os,
            env,
            objfmt,
        };
        ret.full = format!("{}", &ret);
        ret
    }

    ///
    /// Gets the object format, either from the end of the `env` field, or the default for the target
    pub fn get_object_format(&self) -> ObjectFormat {
        if let Some(of) = self.objfmt {
            of
        } else {
            match (&self.arch, &self.os) {
                (Architecture::Unknown, Some(OS::MacOSX)) => ObjectFormat::MachO,
                (Architecture::Aarch64, Some(OS::MacOSX)) => ObjectFormat::MachO,
                (Architecture::Aarch64_32, Some(OS::MacOSX)) => ObjectFormat::MachO,
                (Architecture::Arm, Some(OS::MacOSX)) => ObjectFormat::MachO,
                (Architecture::X86, Some(OS::MacOSX)) => ObjectFormat::MachO,
                (Architecture::X86_64, Some(OS::MacOSX)) => ObjectFormat::MachO,
                (Architecture::Unknown, Some(OS::Win32)) => ObjectFormat::Coff,
                (Architecture::Aarch64, Some(OS::Win32)) => ObjectFormat::Coff,
                (Architecture::Aarch64_32, Some(OS::Win32)) => ObjectFormat::Coff,
                (Architecture::Arm, Some(OS::Win32)) => ObjectFormat::Coff,
                (Architecture::X86, Some(OS::Win32)) => ObjectFormat::Coff,
                (Architecture::X86_64, Some(OS::Win32)) => ObjectFormat::Coff,
                (Architecture::PowerPC32, Some(OS::AIX)) => ObjectFormat::XCoff,
                (Architecture::PowerPC64, Some(OS::AIX)) => ObjectFormat::XCoff,
                _ => ObjectFormat::Elf,
            }
        }
    }

    ///
    /// Gets the value of the Architecture field
    pub fn get_arch(&self) -> Architecture {
        self.arch
    }

    ///
    /// Gets the value of the vendor field.
    pub fn get_vendor(&self) -> Vendor {
        self.vendor
    }
}
