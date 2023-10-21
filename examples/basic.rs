use elflib::ElfParser;

fn main() -> elflib::Result<()> {
    // let mut content = std::fs::read("/lib/modules/6.2.0-34-generic/kernel/net/nfc/nfc.ko").unwrap();
    // let mut content = std::fs::read("/usr/bin/ls").unwrap();
    let mut content = std::fs::read("/tmp/aa/main").unwrap();
    // let mut content = std::fs::read("/tmp/aa/mainppc").unwrap();
    let parser = ElfParser::new(&mut content)?;
    let hdr = parser.header()?;
    println!("{:?}", hdr);

    let records = parser.program_headers()?;
    for x in records.iter() {
        println!("{:?}", x);
    }
    Ok(())
}
