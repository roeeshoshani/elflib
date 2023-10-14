use elflib::ElfParser;

fn main() -> elflib::Result<()> {
    let mut content = std::fs::read("/lib/modules/6.2.0-34-generic/kernel/net/nfc/nfc.ko").unwrap();
    let mut parser = ElfParser::new(&mut content)?;
    let hdr = parser.header()?;
    println!("{:?}", hdr);
    let mut shdrs = parser.section_headers()?;

    let mut i = 0;
    for x in shdrs.iter() {
        println!("{:?}", x.unwrap().name_offset());
    }
    Ok(())
}
