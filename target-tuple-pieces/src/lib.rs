#![cfg_attr(not(test), no_std)]
#![allow(
    clippy::upper_case_acronyms,
    clippy::manual_non_exhaustive,
    clippy::match_like_matches_macro
)] // Kill clippy for MSRV

use core::fmt::Formatter;
use core::{fmt::Display, str::FromStr};

///
/// The result of FromStr::from_str, when parsing a field (other than vendor),
///  with a value that is not known to the library
#[derive(Debug, Clone, Copy)]
pub struct UnknownError;

impl Display for UnknownError {
    fn fmt(&self, fmt: &mut Formatter) -> core::fmt::Result {
        fmt.write_str("Unknown or invalid target or component")
    }
}

///
/// The Architecture field of a target tuple
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[non_exhaustive]
pub enum Architecture {
    Unknown,
    X86_16(u8),
    X86_32(u8),
    X86_64 { microarch: u8 },
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
    M6502,
    M65C02,
    SPC700,
    Clever,
    HoleyBytes,
}

impl FromStr for Architecture {
    type Err = UnknownError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "i86" | "i8086" | "i086" => Self::X86_16(0),
            "i186" => Self::X86_16(1),
            "i286" => Self::X86_16(2),
            "i386" => Self::X86_32(3),
            "i486" => Self::X86_32(4),
            "i586" => Self::X86_32(5),
            "i686" => Self::X86_32(6),
            "i786" => Self::X86_32(7),
            "amd64" | "x86_64" | "x86_64h" | "x64" => Self::X86_64 { microarch: 1 },
            "x86_64v2" => Self::X86_64 { microarch: 2 },
            "x86_64v3" => Self::X86_64 { microarch: 3 },
            "x86_64v4" => Self::X86_64 { microarch: 4 },
            "armeb" => Self::ArmBe,
            "arm" => Self::Arm,
            "aarch64" | "arm64" | "arm64e" => Self::Aarch64,
            "aarch64_be" | "arm64_be" => Self::Aarch64Be,
            "aarch64_32" | "arm64_32" => Self::Aarch64_32,
            s if s.starts_with("clever") => Self::Clever,
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
            "wc65c816" | "65816" | "w65c816" | "65c816" | "w65" => Self::Wc65c816,
            "6502" | "6502x" | "6502X" => Self::M6502,
            "65c02" | "65C02" => Self::M65C02,
            "wasm32" => Self::Wasm32,
            "wasm64" => Self::Wasm64,

            "spc700" | "spc" => Self::SPC700,
            "holeybytes" | "hbvm" | "hb" => Self::HoleyBytes,

            _ => return Err(UnknownError),
        })
    }
}

impl Display for Architecture {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.canonical_name().fmt(f)
    }
}

impl Architecture {
    /// Parses the Architecture in a "lossy" manner
    /// This is equivalent to [`Self::from_str`], but returns [`Architecture::Unknown`], instead of an error,
    ///  on an unknown architecture.
    /// This is useful (in conjunction with an actual target name)
    pub fn parse(st: &str) -> Self {
        Self::from_str(st).unwrap_or(Architecture::Unknown)
    }

    ///
    /// Returns the canonical name of the target
    /// The canonical name, when passed into `[`Self::parse`] will yield an equivalent value,
    /// Formatting an Architecture yields this string
    pub fn canonical_name(&self) -> &'static str {
        match self {
            Architecture::Unknown => "unknown",
            Architecture::X86_16(0) => "i86",
            Architecture::X86_16(1) => "i186",
            Architecture::X86_16(2..) => "i286",
            Architecture::X86_32(..=3) => "i386",
            Architecture::X86_32(4) => "i486",
            Architecture::X86_32(5) => "i586",
            Architecture::X86_32(6) => "i686",
            Architecture::X86_32(7..) => "i786",
            Architecture::X86_64 {
                microarch: 0 | 1 | 5..,
            } => "x86_64",
            Architecture::X86_64 { microarch: 2 } => "x86_64v2",
            Architecture::X86_64 { microarch: 3 } => "x86_64v3",
            Architecture::X86_64 { microarch: 4 } => "x86_64v4",
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
            Architecture::Wc65c816 => "w65",
            Architecture::MipsLE => "mipsel",
            Architecture::Mips64LE => "mips64el",
            Architecture::M6502 => "6502",
            Architecture::M65C02 => "6502",
            Architecture::SPC700 => "spc700",
            Architecture::Clever => "clever",
            Architecture::HoleyBytes => "holeybytes",
        }
    }
}

///
/// The Vendor field of a target tuple
///
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[non_exhaustive]
pub enum Vendor {
    Unknown = 0,
    Apple = 1,
    PC = 2,
    SCEI = 3,
    Freescale = 4,
    IBM = 5,
    ImaginationTechnologies = 6,
    MipsTechnologies = 7,
    NVIDIA = 8,
    CSR = 9,
    Myriad = 10,
    AMD = 11,
    Mesa = 12,
    SUSE = 13,
    OpenEmbedded = 14,
    WDC = 15,
}

impl FromStr for Vendor {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "apple" => Self::Apple,
            "pc" => Self::PC,
            // "nes" => Self::NES,
            // "snes" | "snesdev" => Self::SNES,
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
            "wdc" => Self::WDC,
            _ => Self::Unknown,
        })
    }
}

impl Display for Vendor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.canonical_name().fmt(f)
    }
}

impl Vendor {
    /// Parses the Vendor in a "lossy" manner
    /// This is equivalent to [`Self::from_str`].
    /// Note that an unknown vendor is not considered an error.
    pub fn parse(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }

    ///
    /// Returns the canonical name of the vendor
    /// The canonical name, when passed into `[`Self::parse`] will yield an equivalent value,
    /// Formatting a Vendor yields this string
    pub fn canonical_name(&self) -> &'static str {
        match self {
            Vendor::Apple => "apple",
            Vendor::PC => "pc",
            Vendor::Unknown => "unknown",
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
            Vendor::WDC => "wdc",
        }
    }
}

///
/// The Operating System Field of a target tuple
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[non_exhaustive]
pub enum OS {
    Unknown = 0,

    Ananas = 1,
    CloudABI = 2,
    Darwin = 3,
    DragonFly = 4,
    FreeBSD = 5,
    Fuchsia = 6,
    IOS = 7,
    KFreeBSD = 8,
    Linux = 9,
    Lv2 = 10,
    MacOSX = 11,
    NetBSD = 12,
    OpenBSD = 13,
    Solaris = 14,
    Win32 = 15,
    ZOS = 16,
    Haiku = 17,
    Minix = 18,
    RTEMS = 19,
    NaCl = 20,
    AIX = 21,
    CUDA = 22,
    NVCL = 23,
    AMDHSA = 24,
    PS4 = 25,
    ELFIAMCU = 26,
    TvOS = 27,
    WatchOS = 28,
    Mesa3D = 29,
    Contiki = 30,
    AMDPAL = 31,
    HermitCore = 32,
    Hurd = 33,
    WASI = 34,
    Emscripten = 35,
    SNES = 37, // Not an OS, but the currently config.sub places it in the os field
    NES = 38,  // likewise
    None = 39, // No OS
    CleverOS = 40,
    AbleOS = 41,
    Lilium = 42,
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
            x if x.starts_with("snes") => Self::SNES,
            x if x.starts_with("nes") => Self::NES,
            x if x.starts_with("cleveros") => Self::CleverOS,
            x if x.starts_with("ableos") => Self::AbleOS,
            x if x.starts_with("lilium") => Self::Lilium,
            "none" => Self::None,

            _ => return Err(UnknownError),
        })
    }
}

impl Display for OS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.canonical_name().fmt(f)
    }
}

impl OS {
    /// Parses the OS in a "lossy" manner
    /// This is equivalent to [`Self::from_str`], except that [`OS::Unknown`] is returned, instead of an error, on an unknown OS Field
    pub fn parse(s: &str) -> Self {
        Self::from_str(s).unwrap_or(Self::Unknown)
    }

    ///
    /// Returns the canonical name of the operating system
    /// The canonical name, when passed into `[`Self::parse`] will yield an equivalent value,
    /// Formatting an OS yields this string
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
            OS::SNES => "snes",
            OS::NES => "nes",
            OS::None => "none",
            OS::CleverOS => "cleveros",
            OS::AbleOS => "ableos",
            OS::Lilium => "lilium",
        }
    }
}

///
/// The Environment field of target tuples
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Environment {
    Unknown = 0,
    GNU = 1,
    GNUABIN32 = 2,
    GNUABI64 = 3,
    GNUEABI = 4,
    GNUEABIHF = 5,
    GNUX32 = 6,
    CODE16 = 7,
    EABI = 8,
    EABIHF = 9,
    Android = 10,
    Musl = 11,
    MuslEABI = 12,
    MuslEABIHF = 13,

    MSVC = 15,
    Itanium = 16,
    Cygnus = 17,
    CoreCLR = 18,
    Simulator = 19,
    MacABI = 20,

    Standard = 23,
    Kernel = 24,
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
            x if x.starts_with("std") => Self::Standard,
            x if x.starts_with("kernel") => Self::Kernel,
            _ => return Err(UnknownError),
        })
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.canonical_name().fmt(f)
    }
}

impl Environment {
    /// Parses the Environment name in a "lossy" manner
    /// This is equivalent to [`Self::from_str`], except that [`Environment::Unknown`] is returned, instead of an error, on an unknown OS Field
    pub fn parse(s: &str) -> Self {
        Self::from_str(s).unwrap_or(Self::Unknown)
    }

    ///
    /// Returns the canonical name of the environment
    /// The canonical name, when passed into [`Self::parse`] will yield an equivalent value,
    /// Formatting an Environment yields this string
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
            Environment::Standard => "std",
            Environment::Kernel => "kernel",
        }
    }
}

///
/// The object format used by a target
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[non_exhaustive]
pub enum ObjectFormat {
    Unknown = 0,
    XCoff = 1,
    Coff = 2,
    Elf = 3,
    Goff = 4,
    MachO = 5,
    Wasm = 6,

    Xo65 = 7,
    O65 = 8,
    WlaObj = 9,
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
            x if x.ends_with("xo65") => Self::Xo65,
            x if x.ends_with("o65") => Self::O65,
            x if x.ends_with("wlaobj") => Self::WlaObj,
            x if x.ends_with("wla") => Self::WlaObj,
            _ => return Err(UnknownError),
        })
    }
}

impl Display for ObjectFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.canonical_name().fmt(f)
    }
}

impl ObjectFormat {
    /// Parses the ObjectFormat name in a "lossy" manner, from the end of the Environment field
    /// This is equivalent to [`Self::from_str`], except that [`ObjectFormat::Unknown`] is returned, instead of an error, on an unknown OS Field
    pub fn parse(s: &str) -> Self {
        Self::from_str(s).unwrap_or(Self::Unknown)
    }

    ///
    /// Returns the canonical name of the object format
    /// The canonical name, when passed into [`Self::parse`] will yield an equivalent value,
    /// Formatting an ObjectFormat yields this string
    pub fn canonical_name(&self) -> &'static str {
        match self {
            ObjectFormat::Unknown => "unknown",
            ObjectFormat::XCoff => "xcoff",
            ObjectFormat::Coff => "coff",
            ObjectFormat::Elf => "elf",
            ObjectFormat::Goff => "goff",
            ObjectFormat::MachO => "macho",
            ObjectFormat::Wasm => "wasm",
            ObjectFormat::Xo65 => "xo65",
            ObjectFormat::O65 => "o65",
            ObjectFormat::WlaObj => "wlaobj",
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct System {
    os: Option<OS>,
    env: Option<Environment>,
    objfmt: Option<ObjectFormat>,
}

impl System {
    /// Converts the specified pieces into an object file. At least one must be [`Some`]
    ///
    /// ## Panics
    /// Panics if all of the pieces are [`None`]
    pub const fn from_pieces(
        os: Option<OS>,
        env: Option<Environment>,
        objfmt: Option<ObjectFormat>,
    ) -> Self {
        assert!(os.is_some() || env.is_some() || objfmt.is_some());

        Self { os, env, objfmt }
    }

    pub const fn from_os(os: OS) -> Self {
        Self {
            os: Some(os),
            env: None,
            objfmt: None,
        }
    }

    pub const fn from_os_env(os: OS, env: Environment) -> Self {
        Self {
            os: Some(os),
            env: Some(env),
            objfmt: None,
        }
    }

    pub const fn from_env(env: Environment) -> Self {
        Self {
            os: None,
            env: Some(env),
            objfmt: None,
        }
    }

    pub const fn from_objfmt(objfmt: ObjectFormat) -> Self {
        Self {
            os: None,
            env: None,
            objfmt: Some(objfmt),
        }
    }

    pub const fn os(&self) -> Option<OS> {
        self.os
    }

    pub const fn env(&self) -> Option<Environment> {
        self.env
    }

    pub const fn object_format(&self) -> Option<ObjectFormat> {
        self.objfmt
    }
}

impl core::fmt::Display for System {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut sep = "";
        if let Some(os) = self.os {
            os.fmt(f)?;
            sep = "-";
        }

        if let Some(env) = self.env {
            f.write_str(sep)?;
            sep = "";
            env.fmt(f)?;
        }

        if let Some(objfmt) = self.objfmt {
            f.write_str(sep)?;
            objfmt.fmt(f)?;
        }

        Ok(())
    }
}

impl FromStr for System {
    type Err = UnknownError;

    fn from_str(sys: &str) -> Result<Self, Self::Err> {
        if let Some((os, senv)) = sys.split_once('-') {
            let os = os.parse::<OS>()?;

            let env = senv.parse::<Environment>();
            let objfmt = senv.parse::<ObjectFormat>();

            env.map(|_| ()).or_else(|_| objfmt.map(|_| ()))?;

            Ok(Self {
                os: Some(os),
                env: env.ok(),
                objfmt: objfmt.ok(),
            })
        } else if let Ok(os) = sys.parse::<OS>() {
            Ok(Self {
                os: Some(os),
                env: None,
                objfmt: None,
            })
        } else {
            let env = sys.parse::<Environment>();
            let objfmt = sys.parse::<ObjectFormat>();

            env.map(|_| ()).or_else(|_| objfmt.map(|_| ()))?;

            Ok(Self {
                os: None,
                env: env.ok(),
                objfmt: objfmt.ok(),
            })
        }
    }
}
