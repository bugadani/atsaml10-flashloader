use std::{fmt::Write as _, io::BufRead as _};

use duct::cmd;
use jep106::JEP106Code;
use probe_rs_target::{ChipFamily, CoreType, MemoryRegion, NvmRegion, RamRegion};

fn generate(target: &Variant) -> ChipFamily {
    let target_lower = target.name.to_lowercase();
    cmd!(
        "cargo",
        "rustc",
        "-p",
        "atsaml10-flashloader",
        "--release",
        "--no-default-features",
        "--features",
        &target_lower,
        "--target",
        "thumbv8m.base-none-eabi",
        "--",
        "-C",
        "inline-threshold=5",
        "-C",
        "no-vectorize-loops",
        "-C",
        "force-frame-pointers=no",
        "-C",
        "link-arg=--nmagic",
        "-C",
        "link-arg=-Talgo/link.x",
        "-C",
        "link-arg=-Tmemory.x",
    )
    .stderr_to_stdout()
    .run()
    .unwrap();

    let elf = "target/thumbv8m.base-none-eabi/release/atsaml10-flashloader";

    let output = cmd!("target-gen", "elf", "-n", &target_lower, elf)
        .stdout_capture()
        .run()
        .unwrap()
        .stdout;

    let yaml = String::from_utf8(output).unwrap();

    // Cut off last 3 lines
    let lines = yaml.lines().collect::<Vec<_>>();
    let yaml = lines[..lines.len() - 3].join("\n");

    let mut chip = serde_yaml::from_str::<ChipFamily>(&yaml).unwrap();

    // Replace placeholder values
    target.apply(&mut chip.variants[0]);

    chip
}

struct Family {
    name: &'static str,
    targets: &'static [Variant],
}

struct Variant {
    name: &'static str,
    ram_size: u64,
    flash_size: u64,
}

impl Variant {
    fn apply(&self, chip: &mut probe_rs_target::Chip) {
        chip.name = self.name.to_string();
        chip.cores[0].core_type = CoreType::Armv8m;
        chip.memory_map = vec![
            MemoryRegion::Nvm(NvmRegion {
                is_boot_memory: true,
                range: 0..self.flash_size,
                cores: vec!["main".to_string()],
                name: Some("Flash".to_string()),
                is_alias: false,
            }),
            MemoryRegion::Nvm(NvmRegion {
                is_boot_memory: false,
                range: 0x00400000..(0x00400000 + 2 * 1024),
                cores: vec!["main".to_string()],
                name: Some("Data Flash".to_string()),
                is_alias: false,
            }),
            MemoryRegion::Ram(RamRegion {
                is_boot_memory: false,
                range: 0x2000_0000..(0x2000_0000 + self.ram_size),
                cores: vec!["main".to_string()],
                name: Some("SRAM".to_string()),
            }),
        ];
    }
}

/// Some optimizations to improve the readability of the `serde_yaml` output:
/// - If `Option<T>` is `None`, it is serialized as `null` ... we want to omit it.
/// - If `Vec<T>` is empty, it is serialized as `[]` ... we want to omit it.
/// - `serde_yaml` serializes hex formatted integers as single quoted strings, e.g. '0x1234' ... we need to remove the single quotes so that it round-trips properly.
pub fn serialize_to_yaml_string(family: &ChipFamily) -> String {
    let yaml_string = serde_yaml::to_string(&family).unwrap();
    let mut reader = std::io::BufReader::new(yaml_string.as_bytes());
    let mut reader_line = String::new();

    let mut yaml_string = String::new();
    while reader.read_line(&mut reader_line).unwrap() > 0 {
        if reader_line.ends_with(": null\n")
            || reader_line.ends_with(": []\n")
            || reader_line.ends_with(": {}\n")
            || reader_line.ends_with(": false\n")
        {
            // Skip the line
            reader_line.clear();
            continue;
        }

        if (reader_line.contains("'0x") || reader_line.contains("'0X"))
            && reader_line.ends_with("'\n")
        {
            // Remove the single quotes
            reader_line = reader_line.replace('\'', "");
        }

        yaml_string.write_str(&reader_line).unwrap();
        reader_line.clear();
    }

    yaml_string
}

pub fn main() {
    let families = [
        Family {
            name: "SAML10",
            targets: &[
                Variant {
                    name: "ATSAML10x14",
                    ram_size: 4 * 1024,
                    flash_size: 16 * 1024,
                },
                Variant {
                    name: "ATSAML10x15",
                    ram_size: 8 * 1024,
                    flash_size: 32 * 1024,
                },
                Variant {
                    name: "ATSAML10x16",
                    ram_size: 16 * 1024,
                    flash_size: 64 * 1024,
                },
            ],
        },
        Family {
            name: "SAML11",
            targets: &[
                Variant {
                    name: "ATSAML11x14",
                    ram_size: 8 * 1024,
                    flash_size: 16 * 1024,
                },
                Variant {
                    name: "ATSAML11x15",
                    ram_size: 8 * 1024,
                    flash_size: 32 * 1024,
                },
                Variant {
                    name: "ATSAML11x16",
                    ram_size: 16 * 1024,
                    flash_size: 64 * 1024,
                },
            ],
        },
    ];

    _ = std::fs::create_dir("out");

    for family in families {
        let mut chips = ChipFamily {
            name: family.name.to_string(),
            manufacturer: Some(JEP106Code { id: 0x29, cc: 0 }),
            generated_from_pack: false,
            pack_file_release: None,
            variants: vec![],
            flash_algorithms: vec![],
            source: probe_rs_target::TargetDescriptionSource::BuiltIn,
        };

        for target in family.targets {
            let chip = generate(target);
            chips.variants.extend(chip.variants);
            chips.flash_algorithms.extend(chip.flash_algorithms);
        }

        let yaml = serialize_to_yaml_string(&chips);
        std::fs::write(format!("out/{}.yaml", family.name), yaml).unwrap();
    }
}
