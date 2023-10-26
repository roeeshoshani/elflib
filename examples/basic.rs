use binary_serde::BinarySerde;
use elflib::{
    elf_types::{SectionHeader32, SectionHeader64},
    ElfParser,
};

fn main() -> elflib::Result<()> {
    // let mut content = std::fs::read("/lib/modules/6.2.0-34-generic/kernel/net/nfc/nfc.ko").unwrap();
    // let mut content = std::fs::read("/usr/bin/ls").unwrap();
    // let mut content = std::fs::read("/tmp/aa/main").unwrap();
    // let mut content = std::fs::read("/tmp/aa/mainppc").unwrap();
    let mut content = std::fs::read("/tmp/aa/main32").unwrap();
    // let mut content = std::fs::read("/tmp/aa/mainmips").unwrap();

    println!("{:?}", SectionHeader32::SERIALIZED_SIZE);

    let parser = ElfParser::new(&mut content)?;
    let sections = parser.section_headers()?;
    for section_res in sections {
        let section = section_res?;
        if let Some(rela_section) = section.as_rel()? {
            for rela_entry_res in rela_section {
                let rela_entry = rela_entry_res?;
                println!("{:?}", rela_entry);
            }
        }
    }
    Ok(())
}
