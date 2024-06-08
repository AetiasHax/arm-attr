use std::{fs, io::Read, path::PathBuf};

use anyhow::{anyhow, Result};
use arm_attr::{read::Endian, AttributeDisplayOptions, BuildAttrs};
use clap::Parser;
use object::{elf::SHT_ARM_ATTRIBUTES, Endianness, File, Object, ObjectSection, SectionKind};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input ELF file
    #[arg(short, long)]
    file: PathBuf,

    /// Show default values of attributes not specified in the input
    #[arg(short = 'd', long)]
    show_defaults: bool,

    /// Show target attributes. Enabled by default if -p and -m are not given
    #[arg(short = 't', long)]
    show_target: bool,

    /// Show procedure-call attributes. Enabled by default if -t and -m are not given
    #[arg(short = 'p', long)]
    show_pcs: bool,

    /// Show miscellaneous attributes. Enabled by default if -t and -p are not given
    #[arg(short = 'm', long)]
    show_misc: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let show_all = !args.show_target && !args.show_pcs && !args.show_misc;

    let data = {
        let mut file = fs::File::open(args.file)?;
        let mut data = vec![];
        file.read_to_end(&mut data)?;
        data
    };
    let file: File<&[u8]> = File::parse(data.as_ref())?;
    let arm_attrs = file
        .sections()
        .find(|s| s.kind() == SectionKind::Elf(SHT_ARM_ATTRIBUTES) && s.name() == Ok(".ARM.attributes"))
        .ok_or(anyhow!("No attributes section found"))?;
    let attrs_data = arm_attrs.data()?;

    let build_attrs = BuildAttrs::new(attrs_data, convert_endian(file.endianness()))?;
    for section in build_attrs.subsections().map(|s| s.unwrap()) {
        println!("Vendor: {}", section.vendor_name());
        if !section.is_aeabi() {
            continue;
        }
        let attributes = section.into_public_attributes()?;
        println!("    File scope:");
        println!(
            "{}",
            attributes.file_scope.display(AttributeDisplayOptions {
                indent: 8,
                show_defaults: args.show_defaults,
                show_target: show_all || args.show_target,
                show_pcs: show_all || args.show_pcs,
                show_misc: show_all || args.show_misc
            })
        );
    }

    Ok(())
}

fn convert_endian(endian: Endianness) -> Endian {
    match endian {
        Endianness::Little => Endian::Little,
        Endianness::Big => Endian::Big,
    }
}
