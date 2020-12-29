use std::{fmt::Display, str::FromStr};

#[derive(Debug,Clone,Copy)]
pub struct UnknownError;

#[non_exhaustive]
#[derive(Copy,Clone,PartialEq,Eq,Debug)]
pub enum Architecture{
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

impl FromStr for Architecture{
    type Err = UnknownError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s{
            "i386" | "i486" | "i586" | "i686" | "i786" | "i886" | "i986" => Self::X86,
            "amd64" | "x86_64" | "x86_64h" => Self::X86_64,
            "armeb" => Self::ArmBe,
            "arm" => Self::Arm,
            "aarch64" | "arm64" | "arm64e" => Self::Aarch64,
            "aarch64_be" | "arm64_be" => Self::Aarch64Be,
            "aarch64_32" | "arm64_32" => Self::Aarch64_32,
            "powerpc" | "powerpcspe" | "ppc" | "ppc32" => Self::PowerPC32,
            "powerpc64" | "ppu" |"ppc64" => Self::PowerPC64,
            "powerpc64le" | "ppc64le" => Self::PowerPC64le,
            "mips" | "mipseb" | "mipsallegrex" | "mipsisa32r6" | "mipsr6" => Self::Mips,
            "mipsel" | "mipsallegrexel" | "mipsisa32r6el" | "mipsr6el" => Self::MipsLE,
            "mips64" | "mips64eb" | "mipsn32" | "mipsisa64r6" | "mips64r6" | "mipsn32r6" => Self::Mips64,
            "mips64el" | "mipsn32el" | "mipsisa64r6el" | "mips64r6el" | "mipsn32r6el" => Self::Mips64LE,
            "sparc" => Self::Sparc,
            "sparcel" => Self::SparcEL,
            "sparcv9" | "sparc64" => Self::SparcV9,
            "riscv32" => Self::RiscV32,
            "riscv64" => Self::RiscV64,
            "wc65c816" | "65816" | "w65c816" | "65c816" => Self::Wc65c816,
            "wasm32" => Self::Wasm32,
            "wasm64" => Self::Wasm64,


            _ => return Err(UnknownError)
        })
    }
}

impl Display for Architecture{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.canonical_name().fmt(f)
    }
}

impl Architecture{
    pub fn parse(st: &str) -> Self{
        Self::from_str(st).unwrap_or(Architecture::Unknown)
    }

    #[must_use]
    pub fn is_unknown(&self) -> bool{
        match self{
            Self::Unknown => true,
            _ => false
        }
    }

    pub fn canonical_name(&self) -> &'static str{
        match self{
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
            Architecture::Mips64LE => "mips64el"
        }
    }

    pub fn is_64bit(&self) -> bool{
        match self{
            Self::X86_64 | Self::Aarch64 | Self::Aarch64Be | Self::Mips64 
            | Self::PowerPC64 | Self::PowerPC64le | Self::RiscV64 | Self::Wasm64 
            | Self::Mips64LE => true,
            _ => false
        }
    }
}

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
pub enum Vendor{
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
    OpenEmbedded
}

impl FromStr for Vendor{
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s{
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
            _ => Self::Unknown
        })
    }
}

impl Display for Vendor{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.canonical_name().fmt(f)
    }
}

impl Vendor{
    pub fn parse(s: &str) -> Self{
        Self::from_str(s).unwrap()
    }

    pub fn canonical_name(&self) -> &'static str{
        match self{
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
            Vendor::OpenEmbedded => "oe"
        }
    }
}

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
#[non_exhaustive]
pub enum OS{
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
    PhantomOS
}

impl FromStr for OS{
    type Err = UnknownError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s{
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
            x if x.starts_with("win32") | x.starts_with("windows")
                => Self::Win32,
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

            _ => return Err(UnknownError)
        })
    }
}

impl Display for OS{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.canonical_name().fmt(f)
    }
}

impl OS{
    pub fn parse(s: &str) -> Self{
        Self::from_str(s).unwrap_or(Self::Unknown)
    }

    pub fn canonical_name(&self) -> &'static str{
        match self{
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
            OS::PhantomOS => "phantom"
        }
    }
}

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
#[non_exhaustive]
pub enum Environment{
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

impl FromStr for Environment{
    type Err = UnknownError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s{
            x if x.starts_with("eabihf") => Self::EABIHF,
            x if x.starts_with("eabi") => Self::EABI,
            x if x.starts_with("gnuabin32") => Self::GNUABIN32,
            x if x.starts_with("gnuabi64") => Self::GNUABI64,
            x if x.starts_with("gnueabihf") => Self::GNUEABIHF,
            x if x.starts_with("gnueabi") => Self::GNUEABI,
            x if x.starts_with("gnux32") =>Self::GNUX32,
            x if x.starts_with("gnu") => Self::GNU,
            x if x.starts_with("code16") =>Self::CODE16,
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
            _ => return Err(UnknownError)
        })
    }
}

impl Display for Environment{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.canonical_name().fmt(f)
    }
}

impl Environment{
    pub fn parse(s: &str) -> Self{
        Self::from_str(s).unwrap_or(Self::Unknown)
    }

    pub fn canonical_name(&self) -> &'static str{
        match self{
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
            Environment::PhantomKernel => "pkrnl"
        }
    }
}

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
#[non_exhaustive]
pub enum ObjectFormat{
    Unknown,
    XCoff,
    Coff,
    Elf,
    Goff,
    MachO,
    Wasm,
}

impl FromStr for ObjectFormat{
    type Err = UnknownError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s{
            x if x.ends_with("xcoff") => Self::XCoff,
            x if x.ends_with("coff") => Self::Coff,
            x if x.ends_with("elf") => Self::Elf,
            x if x.ends_with("goff") => Self::Goff,
            x if x.ends_with("macho") => Self::MachO,
            x if x.ends_with("wasm") => Self::Wasm,
            _ => return Err(UnknownError)
        })
    }
}

impl Display for ObjectFormat{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.canonical_name().fmt(f)
    }
}

impl ObjectFormat{
    pub fn parse(s: &str) -> Self{
        Self::from_str(s).unwrap_or(Self::Unknown)
    }

    pub fn canonical_name(&self) -> &'static str{
        match self{
            ObjectFormat::Unknown => "unknown",
            ObjectFormat::XCoff => "xcoff",
            ObjectFormat::Coff => "coff",
            ObjectFormat::Elf => "elf",
            ObjectFormat::Goff => "goff",
            ObjectFormat::MachO => "macho",
            ObjectFormat::Wasm => "wasm"
        }
    }
}
