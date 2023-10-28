use elflib::ElfParser;

fn main() -> elflib::Result<()> {
    // let mut content = std::fs::read("/lib/modules/6.2.0-34-generic/kernel/net/nfc/nfc.ko").unwrap();
    // let mut content = std::fs::read("/usr/bin/ls").unwrap();
    let mut content =
        std::fs::read("./test_binaries/build/mips64-linux-gnuabi64-gcc-main").unwrap();

    let parser = ElfParser::new(&mut content)?;
    let sections = parser.section_headers()?;
    for section_res in sections {
        let section = section_res?;
        match section.data()? {
            elflib::SectionData::RelocationSection(rel_section) => {
                for rel_entry_res in rel_section.entries {
                    let rel_entry = rel_entry_res?;
                    println!("{:?}", rel_entry.as_rel_or_rela());
                }
            }
            elflib::SectionData::SymbolTable(symbol_entries) => {
                for symbol_res in symbol_entries {
                    let symbol = symbol_res?;
                    println!("{:?}", symbol.name());
                }
            }
            _ => {}
        }
    }
    Ok(())
}
