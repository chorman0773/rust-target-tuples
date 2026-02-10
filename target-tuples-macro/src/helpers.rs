use proc_macro::{Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use target_tuple_pieces::{Architecture, Environment, ObjectFormat, Vendor, OS};

use crate::emit_path;

pub struct Wildcard;

impl AsConstructor for Wildcard {
    fn into_ctor(&self, span: Span, _dcrate: &TokenStream) -> TokenStream {
        [TokenTree::Ident(Ident::new("_", span))]
            .into_iter()
            .collect()
    }
}

pub trait AsConstructor {
    fn into_ctor(&self, span: Span, dcrate: &TokenStream) -> TokenStream;
}

impl<T: AsConstructor> AsConstructor for Option<T> {
    fn into_ctor(&self, span: Span, dcrate: &TokenStream) -> TokenStream {
        match self {
            Some(v) => {
                let mut base: TokenStream =
                    emit_path(dcrate, ["__core", "option", "Option", "Some"], span).collect();
                base.extend([TokenTree::Group(Group::new(
                    proc_macro::Delimiter::Parenthesis,
                    v.into_ctor(span, dcrate),
                ))]);
                base
            }
            None => emit_path(dcrate, ["__core", "option", "Option", "None"], span).collect(),
        }
    }
}

impl AsConstructor for Architecture {
    fn into_ctor(&self, span: Span, dcrate: &TokenStream) -> TokenStream {
        let (name, tail) = match self {
            Self::X86_16(g) => (
                "X86_16",
                TokenStream::from_iter([TokenTree::Group(Group::new(
                    proc_macro::Delimiter::Parenthesis,
                    [TokenTree::Literal(Literal::u8_suffixed(*g))]
                        .into_iter()
                        .collect(),
                ))]),
            ),
            Self::X86_32(g) => (
                "X86_32",
                TokenStream::from_iter([TokenTree::Group(Group::new(
                    proc_macro::Delimiter::Parenthesis,
                    [TokenTree::Literal(Literal::u8_suffixed(*g))]
                        .into_iter()
                        .collect(),
                ))]),
            ),
            Self::X86_64 { microarch } => (
                "X86_64",
                TokenStream::from_iter([TokenTree::Group(Group::new(
                    proc_macro::Delimiter::Brace,
                    [
                        TokenTree::Ident(Ident::new_raw("microarch", Span::call_site())),
                        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                        TokenTree::Literal(Literal::u8_suffixed(*microarch)),
                    ]
                    .into_iter()
                    .collect(),
                ))]),
            ),
            Architecture::Unknown => ("Unknown", TokenStream::new()),
            Architecture::Arm => ("Arm", TokenStream::new()),
            Architecture::ArmBe => ("ArmBe", TokenStream::new()),
            Architecture::Aarch64 => ("Aarch64", TokenStream::new()),
            Architecture::Aarch64Be => ("Aarch64Be", TokenStream::new()),
            Architecture::Aarch64_32 => ("Aarch64_32", TokenStream::new()),
            Architecture::Mips => ("Mips", TokenStream::new()),
            Architecture::MipsLE => ("MipsLE", TokenStream::new()),
            Architecture::Mips64 => ("Mips64", TokenStream::new()),
            Architecture::Mips64LE => ("Mips64LE", TokenStream::new()),
            Architecture::PowerPC32 => ("PowerPC32", TokenStream::new()),
            Architecture::PowerPC64 => ("PowerPC64", TokenStream::new()),
            Architecture::PowerPC64le => ("PowerPC64le", TokenStream::new()),
            Architecture::RiscV32 => ("RiscV32", TokenStream::new()),
            Architecture::RiscV64 => ("RiscV64", TokenStream::new()),
            Architecture::Sparc => ("Sparc", TokenStream::new()),
            Architecture::SparcV9 => ("SparcV9", TokenStream::new()),
            Architecture::SparcEL => ("SparcEL", TokenStream::new()),
            Architecture::Wasm32 => ("Wasm32", TokenStream::new()),
            Architecture::Wasm64 => ("Wasm64", TokenStream::new()),
            Architecture::Wc65c816 => ("Wc65c816", TokenStream::new()),
            Architecture::M6502 => ("M6502", TokenStream::new()),
            Architecture::M65C02 => ("M65C02", TokenStream::new()),
            Architecture::SPC700 => ("SPC700", TokenStream::new()),
            Architecture::Clever => ("Clever", TokenStream::new()),
            Architecture::HoleyBytes => ("HoleyBytes", TokenStream::new()),
            _ => unimplemented!("Version Mismatch between target-tuples-macro and target-tuples"),
        };

        let mut base =
            emit_path(dcrate, ["pieces", "Architecture", name], span).collect::<TokenStream>();
        base.extend(tail);

        base
    }
}

impl AsConstructor for Vendor {
    fn into_ctor(&self, span: Span, dcrate: &TokenStream) -> TokenStream {
        let name = format!("{self:?}");

        emit_path(dcrate, ["pieces", "Vendor", &name], span).collect()
    }
}

impl AsConstructor for OS {
    fn into_ctor(&self, span: Span, dcrate: &TokenStream) -> TokenStream {
        let name = format!("{self:?}");

        emit_path(dcrate, ["pieces", "OS", &name], span).collect()
    }
}

impl AsConstructor for Environment {
    fn into_ctor(&self, span: Span, dcrate: &TokenStream) -> TokenStream {
        let name = format!("{self:?}");

        emit_path(dcrate, ["pieces", "Environment", &name], span).collect()
    }
}

impl AsConstructor for ObjectFormat {
    fn into_ctor(&self, span: Span, dcrate: &TokenStream) -> TokenStream {
        let name = format!("{self:?}");

        emit_path(dcrate, ["pieces", "ObjectFormat", &name], span).collect()
    }
}
