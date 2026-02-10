use target_tuples::{match_targets, TargetRef};

const TARGET_STRINGS: &[&str] = &[
    "x86_64-pc-linux-gnu",
    "x86_64-linux-musl",
    "x86_64-unknown-linux-gnux32",
    "i386-lilium-kernel",
    "i486-windows-msvc",
    "i786-pc-elf",
    "wc65c816-elf",
    "w65-snes-elf",
];

pub fn wildcard() {
    for targ in TARGET_STRINGS.iter().copied().map(TargetRef::parse) {
        match_targets! {
            targ {
                * => {}
            }
        }
    }
}

pub fn match_funny_targets() {
    for (name, targ) in TARGET_STRINGS
        .iter()
        .copied()
        .map(|v| (v, TargetRef::parse(v)))
    {
        match_targets! {
            targ {
                x86_64-pc-linux-gnu => {
                    assert_eq!(name, "x86_64-pc-linux-gnu");
                }
                x86_64-*-linux-musl => {
                    assert_eq!(name, "x86_64-linux-musl");
                }
                x86_64-unknown-linux-gnux32 => {
                    assert_eq!(name, "x86_64-unknown-linux-gnux32");
                }
                i386-*-lilium-kernel => {
                    assert_eq!(name, "i386-lilium-kernel");
                }
                i486-*-windows-msvc => {
                    assert_eq!(name, "i486-windows-msvc");
                }
                i786-pc-elf => {
                    assert_eq!(name, "i786-pc-elf");
                }
                wc65c816-*-elf => {
                    assert_eq!(name, "wc65c816-elf");
                }
                w65-*-snes-elf => {
                    assert_eq!(name, "w65-snes-elf");
                }
                * => panic!("Wildcard captured unexpectedly")
            }
        }
    }
}
