use crate::{ElfParser, VariantStructBinaryDeserialize};
use binary_serde::{impl_binary_serde_for_bitflags_ty, BinarySerde, Endianness};
use bitflags::bitflags;
use elflib_macros::{define_raw_struct_by_variants, define_raw_struct_generic_bitlen};

pub const ELF_MAGIC: &[u8] = &[0x7f, b'E', b'L', b'F'];
pub const EI_NIDENT: usize = 16;
pub const ELF_IDENT_PADDING_SIZE: usize = EI_NIDENT - ElfIdentHeader::SERIALIZED_SIZE;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct DebugIgnore<T>(pub(crate) T);
impl<T> core::fmt::Debug for DebugIgnore<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "...")
    }
}
impl<T> From<T> for DebugIgnore<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
impl<T> core::ops::Deref for DebugIgnore<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> core::ops::DerefMut for DebugIgnore<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ElfFileInfo {
    pub endianness: Endianness,
    pub bit_length: ArchBitLength,
    pub os_abi: OsAbi,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, BinarySerde)]
pub struct ElfIdentHeader {
    pub magic: [u8; ELF_MAGIC.len()],
    pub bit_size: ArchBitLength,
    pub endianness: ElfEndianness,
    pub elf_version: ElfVersionInIdent,
    pub os_abi: OsAbi,
    pub abi_version: AbiVersion,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, BinarySerde)]
pub struct ElfIdent {
    pub header: ElfIdentHeader,
    pub padding: [u8; ELF_IDENT_PADDING_SIZE],
}

define_raw_struct_by_variants! {
    struct ProgramHeader32 {
        ty: ProgramHeaderType,
        offset: u32,
        virt_addr: u32,
        phys_addr: u32,
        size_in_file: u32,
        size_in_memory: u32,
        flags: ProgramHeaderFlags,
        alignment: u32,
    }
    struct ProgramHeader64 {
        ty: ProgramHeaderType,
        flags: ProgramHeaderFlags,
        offset: u64,
        virt_addr: u64,
        phys_addr: u64,
        size_in_file: u64,
        size_in_memory: u64,
        alignment: u64,
    }
}

define_raw_struct_generic_bitlen! {
    struct SectionHeader {
        name_offset: u32,
        ty: SectionHeaderType,
        flags: U,
        address: U,
        offset: U,
        size: U,
        link: u32,
        info: u32,
        address_alignemnt: U,
        entry_size: U,
    }
}

define_raw_struct_generic_bitlen! {
    struct ElfHeader {
        ident: ElfIdent,
        ty: ElfFileType,
        arch: Architechture,
        version: ElfVersion,
        entry: U,
        program_headers_off: U,
        section_headers_off: U,
        flags: ElfFlags,
        header_size: u16,
        program_header_entry_size: u16,
        program_headers_amount: u16,
        section_header_entry_size: u16,
        section_headers_amount: u16,
        section_names_section_index: u16,
    }
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AbiVersion {
    Valid = 0,
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ElfVersionInIdent {
    Current = 1,
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ElfVersion {
    Current = 1,
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ElfEndianness {
    Little = 1,
    Big = 2,
}
impl From<ElfEndianness> for binary_serde::Endianness {
    fn from(value: ElfEndianness) -> Self {
        match value {
            ElfEndianness::Little => Endianness::Little,
            ElfEndianness::Big => Endianness::Big,
        }
    }
}
impl From<binary_serde::Endianness> for ElfEndianness {
    fn from(value: binary_serde::Endianness) -> Self {
        match value {
            Endianness::Little => ElfEndianness::Little,
            Endianness::Big => ElfEndianness::Big,
        }
    }
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ArchBitLength {
    Arch32Bit = 1,
    Arch64Bit = 2,
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OsAbi {
    /// UNIX System V ABI
    Sysv = 0,
    /// HP-UX
    Hpux = 1,
    /// NetBSD.
    Netbsd = 2,
    /// Object uses GNU ELF extensions.
    Gnu = 3,
    /// Sun Solaris.
    Solaris = 6,
    /// IBM AIX.
    Aix = 7,
    /// SGI Irix.
    Irix = 8,
    /// FreeBSD.
    Freebsd = 9,
    /// Compaq TRU64 UNIX.
    Tru64 = 10,
    /// Novell Modesto.
    Modesto = 11,
    /// OpenBSD.
    Openbsd = 12,
    /// ARM EABI
    ArmAeabi = 64,
    /// ARM
    Arm = 97,
    /// Standalone (embedded) application
    Standalone = 255,
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ElfFileType {
    /// No file type
    None = 0,
    /// Relocatable file
    Rel = 1,
    /// Executable file
    Exec = 2,
    /// Shared object file
    Dyn = 3,
    /// Core file
    Core = 4,
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Architechture {
    /// No machine
    None = 0x0,
    /// AT&T WE 32100
    M32 = 0x1,
    /// SUN SPARC
    Sparc = 0x2,
    /// Intel 80386
    Intel386 = 0x3,
    /// Motorola m68k family
    Motorola68K = 0x4,
    /// Motorola m88k family
    Motorola88K = 0x5,
    /// Intel MCU
    Iamcu = 0x6,
    /// Intel 80860
    Intel860 = 0x7,
    /// MIPS R3000 big-endian
    Mips = 0x8,
    /// IBM System/370
    S370 = 0x9,
    /// MIPS R3000 little-endian
    MipsRs3Le = 0xa,
    /// HPPA
    Parisc = 0xf,
    /// Fujitsu VPP500
    Vpp500 = 0x11,
    /// Sun's "v8plus"
    Sparc32Plus = 0x12,
    /// Intel 80960
    Intel960 = 0x13,
    /// PowerPC
    Ppc = 0x14,
    /// PowerPC 64-bit
    Ppc64 = 0x15,
    /// IBM S390
    S390 = 0x16,
    /// IBM SPU/SPC
    Spu = 0x17,
    /// NEC V800 series
    V800 = 0x24,
    /// Fujitsu FR20
    Fr20 = 0x25,
    /// TRW RH-32
    Rh32 = 0x26,
    /// Motorola RCE
    Rce = 0x27,
    /// ARM
    Arm = 0x28,
    /// Digital Alpha
    FakeAlpha = 0x29,
    /// Hitachi SH
    Sh = 0x2a,
    /// SPARC v9 64-bit
    Sparcv9 = 0x2b,
    /// Siemens Tricore
    Tricore = 0x2c,
    /// Argonaut RISC Core
    Arc = 0x2d,
    /// Hitachi H8/300
    H8300 = 0x2e,
    /// Hitachi H8/300H
    H8300H = 0x2f,
    /// Hitachi H8S
    H8S = 0x30,
    /// Hitachi H8/500
    H8500 = 0x31,
    /// Intel Merced
    Ia64 = 0x32,
    /// Stanford MIPS-X
    MipsX = 0x33,
    /// Motorola Coldfire
    Coldfire = 0x34,
    /// Motorola M68HC12
    Motorola68Hc12 = 0x35,
    /// Fujitsu MMA Multimedia Accelerator
    Mma = 0x36,
    /// Siemens PCP
    Pcp = 0x37,
    /// Sony nCPU embeeded RISC
    Ncpu = 0x38,
    /// Denso NDR1 microprocessor
    Ndr1 = 0x39,
    /// Motorola Start*Core processor
    Starcore = 0x3a,
    /// Toyota ME16 processor
    Me16 = 0x3b,
    /// STMicroelectronic ST100 processor
    St100 = 0x3c,
    /// Advanced Logic Corp. Tinyj emb.fam
    Tinyj = 0x3d,
    /// AMD x86-64 architecture
    X8664 = 0x3e,
    /// Sony DSP Processor
    Pdsp = 0x3f,
    /// Digital PDP-10
    Pdp10 = 0x40,
    /// Digital PDP-11
    Pdp11 = 0x41,
    /// Siemens FX66 microcontroller
    Fx66 = 0x42,
    /// STMicroelectronics ST9+ 8/16 mc
    St9Plus = 0x43,
    /// STmicroelectronics ST7 8 bit mc
    St7 = 0x44,
    /// Motorola MC68HC16 microcontroller
    Motorola68Hc16 = 0x45,
    /// Motorola MC68HC11 microcontroller
    Motorola68Hc11 = 0x46,
    /// Motorola MC68HC08 microcontroller
    Motorola68Hc08 = 0x47,
    /// Motorola MC68HC05 microcontroller
    Motorola68Hc05 = 0x48,
    /// Silicon Graphics SVx
    Svx = 0x49,
    /// STMicroelectronics ST19 8 bit mc
    St19 = 0x4a,
    /// Digital VAX
    Vax = 0x4b,
    /// Axis Communications 32-bit emb.proc
    Cris = 0x4c,
    /// Infineon Technologies 32-bit emb.proc
    Javelin = 0x4d,
    /// Element 14 64-bit DSP Processor
    Firepath = 0x4e,
    /// LSI Logic 16-bit DSP Processor
    Zsp = 0x4f,
    /// Donald Knuth's educational 64-bit proc
    Mmix = 0x50,
    /// Harvard University machine-independent object files
    Huany = 0x51,
    /// SiTera Prism
    Prism = 0x52,
    /// Atmel AVR 8-bit microcontroller
    Avr = 0x53,
    /// Fujitsu FR30
    Fr30 = 0x54,
    /// Mitsubishi D10V
    D10V = 0x55,
    /// Mitsubishi D30V
    D30V = 0x56,
    /// NEC v850
    V850 = 0x57,
    /// Mitsubishi M32R
    M32R = 0x58,
    /// Matsushita MN10300
    Mn10300 = 0x59,
    /// Matsushita MN10200
    Mn10200 = 0x5a,
    /// picoJava
    Pj = 0x5b,
    /// OpenRISC 32-bit embedded processor
    Openrisc = 0x5c,
    /// ARC International ARCompact
    ArcCompact = 0x5d,
    /// Tensilica Xtensa Architecture
    Xtensa = 0x5e,
    /// Alphamosaic VideoCore
    Videocore = 0x5f,
    /// Thompson Multimedia General Purpose Proc
    TmmGpp = 0x60,
    /// National Semi. 32000
    Ns32K = 0x61,
    /// Tenor Network TPC
    Tpc = 0x62,
    /// Trebia SNP 1000
    Snp1K = 0x63,
    /// STMicroelectronics ST200
    St200 = 0x64,
    /// Ubicom IP2xxx
    Ip2K = 0x65,
    /// MAX processor
    Max = 0x66,
    /// National Semi. CompactRISC
    Cr = 0x67,
    /// Fujitsu F2MC16
    F2Mc16 = 0x68,
    /// Texas Instruments msp430
    Msp430 = 0x69,
    /// Analog Devices Blackfin DSP
    Blackfin = 0x6a,
    /// Seiko Epson S1C33 family
    SeC33 = 0x6b,
    /// Sharp embedded microprocessor
    Sep = 0x6c,
    /// Arca RISC
    Arca = 0x6d,
    /// PKU-Unity & MPRC Peking Uni. mc series
    Unicore = 0x6e,
    /// eXcess configurable cpu
    Excess = 0x6f,
    /// Icera Semi. Deep Execution Processor
    Dxp = 0x70,
    /// Altera Nios II
    AlteraNios2 = 0x71,
    /// National Semi. CompactRISC CRX
    Crx = 0x72,
    /// Motorola XGATE
    Xgate = 0x73,
    /// Infineon C16x/XC16x
    C166 = 0x74,
    /// Renesas M16C
    M16C = 0x75,
    /// Microchip Technology dsPIC30F
    Dspic30F = 0x76,
    /// Freescale Communication Engine RISC
    Ce = 0x77,
    /// Renesas M32C
    M32C = 0x78,
    /// Altium TSK3000
    Tsk3000 = 0x83,
    /// Freescale RS08
    Rs08 = 0x84,
    /// Analog Devices SHARC family
    Sharc = 0x85,
    /// Cyan Technology eCOG2
    Ecog2 = 0x86,
    /// Sunplus S+core7 RISC
    Score7 = 0x87,
    /// New Japan Radio (NJR) 24-bit DSP
    Dsp24 = 0x88,
    /// Broadcom VideoCore III
    Videocore3 = 0x89,
    /// RISC for Lattice FPGA
    Latticemico32 = 0x8a,
    /// Seiko Epson C17
    SeC17 = 0x8b,
    /// Texas Instruments TMS320C6000 DSP
    TiC6000 = 0x8c,
    /// Texas Instruments TMS320C2000 DSP
    TiC2000 = 0x8d,
    /// Texas Instruments TMS320C55x DSP
    TiC5500 = 0x8e,
    /// Texas Instruments App. Specific RISC
    TiArp32 = 0x8f,
    /// Texas Instruments Prog. Realtime Unit
    TiPru = 0x90,
    /// STMicroelectronics 64bit VLIW DSP
    MmdspPlus = 0xa0,
    /// Cypress M8C
    CypressM8C = 0xa1,
    /// Renesas R32C
    R32C = 0xa2,
    /// NXP Semi. TriMedia
    Trimedia = 0xa3,
    /// QUALCOMM DSP6
    Qdsp6 = 0xa4,
    /// Intel 8051 and variants
    Intel8051 = 0xa5,
    /// STMicroelectronics STxP7x
    Stxp7X = 0xa6,
    /// Andes Tech. compact code emb. RISC
    Nds32 = 0xa7,
    /// Cyan Technology eCOG1X
    Ecog1X = 0xa8,
    /// Dallas Semi. MAXQ30 mc
    Maxq30 = 0xa9,
    /// New Japan Radio (NJR) 16-bit DSP
    Ximo16 = 0xaa,
    /// M2000 Reconfigurable RISC
    Manik = 0xab,
    /// Cray NV2 vector architecture
    Craynv2 = 0xac,
    /// Renesas RX
    Rx = 0xad,
    /// Imagination Tech. META
    Metag = 0xae,
    /// MCST Elbrus
    McstElbrus = 0xaf,
    /// Cyan Technology eCOG16
    Ecog16 = 0xb0,
    /// National Semi. CompactRISC CR16
    Cr16 = 0xb1,
    /// Freescale Extended Time Processing Unit
    Etpu = 0xb2,
    /// Infineon Tech. SLE9X
    Sle9X = 0xb3,
    /// Intel L10M
    L10M = 0xb4,
    /// Intel K10M
    K10M = 0xb5,
    /// ARM AARCH64
    AArch64 = 0xb7,
    /// Amtel 32-bit microprocessor
    Avr32 = 0xb9,
    /// STMicroelectronics STM8
    Stm8 = 0xba,
    /// Tilera TILE64
    Tile64 = 0xbb,
    /// Tilera TILEPro
    Tilepro = 0xbc,
    /// Xilinx MicroBlaze
    Microblaze = 0xbd,
    /// NVIDIA CUDA
    Cuda = 0xbe,
    /// Tilera TILE-Gx
    Tilegx = 0xbf,
    /// CloudShield
    Cloudshield = 0xc0,
    /// KIPO-KAIST Core-A 1st gen.
    Corea1St = 0xc1,
    /// KIPO-KAIST Core-A 2nd gen.
    Corea2Nd = 0xc2,
    /// Synopsys ARCv2 ISA.
    Arcv2 = 0xc3,
    /// Open8 RISC
    Open8 = 0xc4,
    /// Renesas RL78
    Rl78 = 0xc5,
    /// Broadcom VideoCore V
    Videocore5 = 0xc6,
    /// Renesas 78KOR
    Renesas78Kor = 0xc7,
    /// Freescale 56800EX DSC
    Freescale56800Ex = 0xc8,
    /// Beyond BA1
    Ba1 = 0xc9,
    /// Beyond BA2
    Ba2 = 0xca,
    /// XMOS xCORE
    Xcore = 0xcb,
    /// Microchip 8-bit PIC(r)
    MchpPic = 0xcc,
    /// Intel Graphics Technology
    Intelgt = 0xcd,
    /// KM211 KM32
    Km32 = 0xd2,
    /// KM211 KMX32
    Kmx32 = 0xd3,
    /// KM211 KMX16
    Emx16 = 0xd4,
    /// KM211 KMX8
    Emx8 = 0xd5,
    /// KM211 KVARC
    Kvarc = 0xd6,
    /// Paneve CDP
    Cdp = 0xd7,
    /// Cognitive Smart Memory Processor
    Coge = 0xd8,
    /// Bluechip CoolEngine
    Cool = 0xd9,
    /// Nanoradio Optimized RISC
    Norc = 0xda,
    /// CSR Kalimba
    CsrKalimba = 0xdb,
    /// Zilog Z80
    Z80 = 0xdc,
    /// Controls and Data Services VISIUMcore
    Visium = 0xdd,
    /// FTDI Chip FT32
    Ft32 = 0xde,
    /// Moxie processor
    Moxie = 0xdf,
    /// AMD GPU
    Amdgpu = 0xe0,
    /// RISC-V
    Riscv = 0xf3,
    /// Linux BPF -- in-kernel virtual machine
    Bpf = 0xf7,
    /// C-SKY Or ArcA5
    CskyOrArcA5 = 0xfc,
    Alpha = 0x9026,
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ProgramHeaderType {
    /// Program header table entry unused
    Null = 0x0,
    /// Loadable program segment
    Load = 0x1,
    /// Dynamic linking information
    Dynamic = 0x2,
    /// Program interpreter
    Interp = 0x3,
    /// Auxiliary information
    Note = 0x4,
    /// Reserved
    Shlib = 0x5,
    /// Entry for header table itself
    Phdr = 0x6,
    /// Thread-local storage segment
    Tls = 0x7,
    /// Start of OS-specific
    Loos = 0x60000000,
    /// GCC .eh_frame_hdr segment
    GnuEhFrame = 0x6474e550,
    /// Indicates stack executability
    GnuStack = 0x6474e551,
    /// Read-only after relocation
    GnuRelro = 0x6474e552,
    /// GNU property
    GnuProperty = 0x6474e553,
    /// Losunw Or Sun Specific segment
    LosunwOrSunwbss = 0x6ffffffa,
    /// Stack segment
    Sunwstack = 0x6ffffffb,
    /// Hisunw Or End of OS-specific
    HisunwOrHios = 0x6fffffff,
    /// Register usage information Or PariscArchext
    MipsReginfoOrPariscArchext = 0x70000000,
    /// Runtime procedure table Or PariscUnwind
    MipsRtprocOrPariscUnwind = 0x70000001,
    MipsOptions = 0x70000002,
    /// FP mode requirement Or HpTls
    MipsAbiflagsOrHpTls = 0x70000003,
    HpCoreNone = 0x70000004,
    HpCoreVersion = 0x70000006,
    HpCoreKernel = 0x70000009,
    HpCoreComm = 0x7000000d,
    HpCoreProc = 0x70000012,
    HpCoreLoadable = 0x70000018,
    HpCoreStack = 0x7000001f,
    HpCoreShm = 0x70000027,
    HpCoreMmf = 0x70000030,
    HpParallel = 0x70000040,
    HpFastbind = 0x70000051,
    HpOptAnnot = 0x70000063,
    HpHslAnnot = 0x70000076,
    HpStack = 0x7000008a,
    /// ARM unwind segment Or arch extension bits
    ArmExidxOrIa64Archext = 0x7000008b,
    /// ia64 unwind bits
    Ia64Unwind = 0x7000008c,
    Ia64HpOptAnot = 0x7000009e,
    Ia64HpHslAnot = 0x700000b1,
    Ia64HpStack = 0x700000c5,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ProgramHeaderFlags: u32 {
        /// Segment is executable
        const X = 0x1;
        /// Segment is writable
        const W = 0x2;
        /// Segment is readable
        const R = 0x4;

        /// OS-specific
        const MASK_OS = 0xff00000;
        /// Processor-specific
        const MASK_PROC = 0xf0000000;

        const MIPS_LOCAL_OR_ARM_SB = 0x10000000;
        const PARISC_SBP_OR_HP_SBP = 0x8000000;
        const HP_PAGE_SIZE = 0x100000;
        const HP_FAR_SHARED = 0x200000;
        const HP_NEAR_SHARED = 0x400000;
        const HP_CODE = 0x1000000;
        const HP_MODIFY = 0x2000000;
        const HP_LAZYSWAP = 0x4000000;
        /// Position-independent segment.
        const ARM_PI = 0x20000000;
        /// Absolute segment.
        const ARM_ABS = 0x40000000;
        /// spec insns w/o recovery
        const IA_64_NORECOV = 0x80000000;
    }
}
impl_binary_serde_for_bitflags_ty! {ProgramHeaderFlags}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SectionHeaderType {
    /// Section header table entry unused
    Null = 0x0,
    /// Program data
    Progbits = 0x1,
    /// Symbol table
    Symtab = 0x2,
    /// String table
    Strtab = 0x3,
    /// Relocation entries with addends
    Rela = 0x4,
    /// Symbol hash table
    Hash = 0x5,
    /// Dynamic linking information
    Dynamic = 0x6,
    /// Notes
    Note = 0x7,
    /// Program space with no data (bss)
    Nobits = 0x8,
    /// Relocation entries, no addends
    Rel = 0x9,
    /// Reserved
    Shlib = 0xa,
    /// Dynamic linker symbol table
    Dynsym = 0xb,
    /// Array of constructors
    InitArray = 0xe,
    /// Array of destructors
    FiniArray = 0xf,
    /// Array of pre-constructors
    PreinitArray = 0x10,
    /// Section group
    Group = 0x11,
    /// Extended section indices
    SymtabShndx = 0x12,
    /// Start OS-specific.
    Loos = 0x60000000,
    /// Object attributes.
    GnuAttributes = 0x6ffffff5,
    /// GNU-style hash table.
    GnuHash = 0x6ffffff6,
    /// Prelink library list
    GnuLiblist = 0x6ffffff7,
    /// Checksum for DSO content.
    Checksum = 0x6ffffff8,
    /// Sun-specific low bound Or SunwMove
    LosunwOrSunwMove = 0x6ffffffa,
    SunwComdat = 0x6ffffffb,
    SunwSyminfo = 0x6ffffffc,
    /// Version definition section.
    GnuVerdef = 0x6ffffffd,
    /// Version needs section.
    GnuVerneed = 0x6ffffffe,
    /// Version symbol table Or Sun-specific high bound Or End OS-specific type
    GnuVersymOrHisunwOrHios = 0x6fffffff,
    /// Start of processor-specific Or Shared objects used in link Or Contains product specific ext
    LoprocOrMipsLiblistOrPariscExt = 0x70000000,
    /// End of processor-specific
    Hiproc = 0x7fffffff,
    /// Start of application-specific
    Louser = 0x80000000,
    /// End of application-specific
    Hiuser = 0x8fffffff,
    /// MipsMsym Or Unwind information Or AlphaDebug Or Unwind information
    MipsMsymOrPariscUnwindOrAlphaDebugOrX8664Unwind = 0x70000001,
    /// Conflicting symbols Or Debug info for optimized code Or AlphaReginfo
    MipsConflictOrPariscDocOrAlphaReginfo = 0x70000002,
    /// Global data area sizes.
    MipsGptab = 0x70000003,
    /// Reserved for SGI/MIPS compilers
    MipsUcode = 0x70000004,
    /// MIPS ECOFF debugging info.
    MipsDebug = 0x70000005,
    /// Register usage information.
    MipsReginfo = 0x70000006,
    MipsPackage = 0x70000007,
    MipsPacksym = 0x70000008,
    MipsReld = 0x70000009,
    MipsIface = 0x7000000b,
    MipsContent = 0x7000000c,
    /// Miscellaneous options.
    MipsOptions = 0x7000000d,
    MipsShdr = 0x70000010,
    MipsFdesc = 0x70000011,
    MipsExtsym = 0x70000012,
    MipsDense = 0x70000013,
    MipsPdesc = 0x70000014,
    MipsLocsym = 0x70000015,
    MipsAuxsym = 0x70000016,
    MipsOptsym = 0x70000017,
    MipsLocstr = 0x70000018,
    MipsLine = 0x70000019,
    MipsRfdesc = 0x7000001a,
    MipsDeltasym = 0x7000001b,
    MipsDeltainst = 0x7000001c,
    MipsDeltaclass = 0x7000001d,
    /// DWARF debugging information.
    MipsDwarf = 0x7000001e,
    MipsDeltadecl = 0x7000001f,
    MipsSymbolLib = 0x70000020,
    /// Event section.
    MipsEvents = 0x70000021,
    MipsTranslate = 0x70000022,
    MipsPixie = 0x70000023,
    MipsXlate = 0x70000024,
    MipsXlateDebug = 0x70000025,
    MipsWhirl = 0x70000026,
    MipsEhRegion = 0x70000027,
    MipsXlateOld = 0x70000028,
    MipsPdrException = 0x70000029,
    MipsXhash = 0x7000002b,
    /// ARM unwind section.
    ArmExidx = 0x7000002c,
    /// Preemption details.
    ArmPreemptmap = 0x7000002e,
    /// ARM attributes section.
    ArmAttributes = 0x70000031,
    /// CskyAttributes Or extension bits
    CskyAttributesOrIa64Ext = 0x70000032,
    /// unwind bits
    Ia64Unwind = 0x70000033,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SectionHeaderFlags: u32 {
        /// Writable
        const WRITE = 0x1;
        /// Occupies memory during execution
        const ALLOC = 0x2;
        /// Executable
        const EXECINSTR = 0x4;
        /// Might be merged
        const MERGE = 0x10;
        /// Contains nul-terminated strings
        const STRINGS = 0x20;
        /// `sh_info' contains SHT index
        const INFO_LINK = 0x40;
        /// Preserve order after combining
        const LINK_ORDER = 0x80;
        const OS_NONCONFORMING = 0x100;
        /// Section is member of a group.
        const GROUP = 0x200;
        /// Section hold thread-local data.
        const TLS = 0x400;
        /// Section with compressed data.
        const COMPRESSED = 0x800;
        /// OS-specific.
        const MASKOS = 0xff00000;
        /// Processor-specific
        const MASKPROC = 0xf0000000;
        /// Not to be GCed by linker.
        const GNU_RETAIN = 0x200000;
        /// ORDERED Or MIPS_ADDR Or Section far from gp
        const ORDERED_OR_MIPS_ADDR_OR_PARISC_HUGE = 0x40000000;
        /// EXCLUDE Or MIPS_STRINGS Or Static branch prediction code Or ARM_COMDEF
        const EXCLUDE_OR_MIPS_STRINGS_OR_PARISC_SBP_OR_ARM_COMDEF = 0x80000000;
        /// Must be in global data area Or ALPHA_GPREL Or Section contains an entry point Or section near gp
        const MIPS_GPREL_OR_ALPHA_GPREL_OR_ARM_ENTRYSECT_OR_IA_64_SHORT = 0x10000000;
        /// MIPS_MERGE Or Section with short addressing Or spec insns w/o recovery
        const MIPS_MERGE_OR_PARISC_SHORT_OR_IA_64_NORECOV = 0x20000000;
        const MIPS_NOSTRIP = 0x8000000;
        const MIPS_LOCAL = 0x4000000;
        const MIPS_NAMES = 0x2000000;
        const MIPS_NODUPE = 0x1000000;
    }
}
impl_binary_serde_for_bitflags_ty! {SectionHeaderFlags}

pub enum SymbolType {
    /// Symbol type is unspecified
    Notype = 0x0,
    /// Symbol is a data object
    Object = 0x1,
    /// Symbol is a code object
    Func = 0x2,
    /// Symbol associated with a section
    Section = 0x3,
    /// Symbol's name is file name
    File = 0x4,
    /// Symbol is a common data object
    Common = 0x5,
    /// Symbol is thread-local data object
    Tls = 0x6,
    /// Start of OS-specific Or Symbol is indirect code object
    LoosOrGnuIfunc = 0xa,
    /// End of OS-specific
    Hios = 0xc,
    /// Start of processor-specific Or Global register reserved to app Or Millicode function entry point
    LoprocOrSparcRegisterOrPariscMillicode = 0xd,
    /// End of processor-specific
    Hiproc = 0xf,
    HpOpaque = 0x10,
    /// HpStub Or A Thumb function Or A Thumb label
    HpStubOrArmTfuncOrArm16Bit = 0x12,
}

pub enum SymbolBinding {
    /// Local symbol
    Local = 0x0,
    /// Global symbol
    Global = 0x1,
    /// Weak symbol
    Weak = 0x2,
    /// Start of OS-specific Or Unique symbol
    LoosOrGnuUnique = 0xa,
    /// End of OS-specific
    Hios = 0xc,
    /// Start of processor-specific Or MipsSplitCommon
    LoprocOrMipsSplitCommon = 0xd,
    /// End of processor-specific
    Hiproc = 0xf,
}

pub enum SymbolVisibility {
    /// Default symbol visibility rules
    Default = 0x0,
    /// Processor specific hidden class
    Internal = 0x1,
    /// Sym unavailable in other modules
    Hidden = 0x2,
    /// Not preemptible, not exported
    Protected = 0x3,
}

pub enum DynamicTag {
    /// Marks end of dynamic section
    Null = 0x0,
    /// Name of needed library Or AlphaNum Or Ia64Num
    NeededOrAlphaNumOrIa64Num = 0x1,
    /// Size in bytes of PLT relocs Or SparcNum Or PpcNum
    PltrelszOrSparcNumOrPpcNum = 0x2,
    /// Processor defined value Or Extranum
    PltgotOrExtranum = 0x3,
    /// Address of symbol hash table Or Ppc64Num
    HashOrPpc64Num = 0x4,
    /// Address of string table
    Strtab = 0x5,
    /// Address of symbol table Or AArch64Num
    SymtabOrAArch64Num = 0x6,
    /// Address of Rela relocs
    Rela = 0x7,
    /// Total size of Rela relocs
    Relasz = 0x8,
    /// Size of one Rela reloc
    Relaent = 0x9,
    /// Size of string table
    Strsz = 0xa,
    /// Size of one symbol table entry Or Addrnum
    SymentOrAddrnum = 0xb,
    /// Address of init function Or Valnum
    InitOrValnum = 0xc,
    /// Address of termination function
    Fini = 0xd,
    /// Name of shared object
    Soname = 0xe,
    /// Library search path (deprecated)
    Rpath = 0xf,
    /// Start symbol search here Or Versiontagnum
    SymbolicOrVersiontagnum = 0x10,
    /// Address of Rel relocs
    Rel = 0x11,
    /// Total size of Rel relocs
    Relsz = 0x12,
    /// Size of one Rel reloc
    Relent = 0x13,
    /// Type of reloc in PLT
    Pltrel = 0x14,
    /// For debugging; unspecified
    Debug = 0x15,
    /// Reloc might modify .text
    Textrel = 0x16,
    /// Address of PLT relocs
    Jmprel = 0x17,
    /// Process relocations of object
    BindNow = 0x18,
    /// Array with addresses of init fct
    InitArray = 0x19,
    /// Array with addresses of fini fct
    FiniArray = 0x1a,
    /// Size in bytes of DT_INIT_ARRAY
    InitArraysz = 0x1b,
    /// Size in bytes of DT_FINI_ARRAY
    FiniArraysz = 0x1c,
    /// Library search path
    Runpath = 0x1d,
    /// Flags for the object being loaded
    Flags = 0x1e,
    /// Start of encoded range Or Array with addresses of preinit fct
    EncodingOrPreinitArray = 0x20,
    /// size in bytes of DT_PREINIT_ARRAY
    PreinitArraysz = 0x21,
    /// Address of SYMTAB_SHNDX section
    SymtabShndx = 0x22,
    /// Start of OS-specific
    Loos = 0x6000000d,
    /// End of OS-specific
    Hios = 0x6ffff000,
    /// Start of processor-specific
    Loproc = 0x70000000,
    /// End of processor-specific Or Most used by any processor Or Shared object to get values from
    HiprocOrProcnumOrFilter = 0x7fffffff,
    Valrnglo = 0x6ffffd00,
    /// Prelinking timestamp
    GnuPrelinked = 0x6ffffdf5,
    /// Size of conflict section
    GnuConflictsz = 0x6ffffdf6,
    /// Size of library list
    GnuLiblistsz = 0x6ffffdf7,
    Checksum = 0x6ffffdf8,
    Pltpadsz = 0x6ffffdf9,
    Moveent = 0x6ffffdfa,
    Movesz = 0x6ffffdfb,
    /// Feature selection (DTF_*).
    Feature1 = 0x6ffffdfc,
    /// Flags for DT_
    Posflag1 = 0x6ffffdfd,
    /// Size of syminfo table (in bytes)
    Syminsz = 0x6ffffdfe,
    /// Entry size of syminfo Or Valrnghi
    SyminentOrValrnghi = 0x6ffffdff,
    Addrrnglo = 0x6ffffe00,
    /// GNU-style hash table.
    GnuHash = 0x6ffffef5,
    TlsdescPlt = 0x6ffffef6,
    TlsdescGot = 0x6ffffef7,
    /// Start of conflict section
    GnuConflict = 0x6ffffef8,
    /// Library list
    GnuLiblist = 0x6ffffef9,
    /// Configuration information.
    Config = 0x6ffffefa,
    /// Dependency auditing.
    Depaudit = 0x6ffffefb,
    /// Object auditing.
    Audit = 0x6ffffefc,
    /// PLT padding.
    Pltpad = 0x6ffffefd,
    /// Move table.
    Movetab = 0x6ffffefe,
    /// Syminfo table Or Addrrnghi
    SyminfoOrAddrrnghi = 0x6ffffeff,
    Versym = 0x6ffffff0,
    Relacount = 0x6ffffff9,
    Relcount = 0x6ffffffa,
    /// State flags, see DF_1_* below.
    Flags1 = 0x6ffffffb,
    Verdef = 0x6ffffffc,
    /// Number of version definitions
    Verdefnum = 0x6ffffffd,
    Verneed = 0x6ffffffe,
    /// Number of needed versions
    Verneednum = 0x6fffffff,
    /// Shared object to load before self
    Auxiliary = 0x7ffffffd,
    /// SparcRegister Or Runtime linker interface version
    SparcRegisterOrMipsRldVersion = 0x70000001,
    /// Timestamp Or Address of _gp
    MipsTimeStampOrNios2Gp = 0x70000002,
    /// Checksum
    MipsIchecksum = 0x70000003,
    /// Version string (string tbl index)
    MipsIversion = 0x70000004,
    /// Flags
    MipsFlags = 0x70000005,
    /// Base address
    MipsBaseAddress = 0x70000006,
    MipsMsym = 0x70000007,
    /// Address of CONFLICT section
    MipsConflict = 0x70000008,
    /// Address of LIBLIST section
    MipsLiblist = 0x70000009,
    /// Number of local GOT entries
    MipsLocalGotno = 0x7000000a,
    /// Number of CONFLICT entries
    MipsConflictno = 0x7000000b,
    /// Number of LIBLIST entries
    MipsLiblistno = 0x70000010,
    /// Number of DYNSYM entries
    MipsSymtabno = 0x70000011,
    /// First external DYNSYM
    MipsUnrefextno = 0x70000012,
    /// First GOT entry in DYNSYM
    MipsGotsym = 0x70000013,
    /// Number of GOT page table entries
    MipsHipageno = 0x70000014,
    /// Address of run time loader map.
    MipsRldMap = 0x70000016,
    /// Delta C++ class definition.
    MipsDeltaClass = 0x70000017,
    MipsDeltaClassNo = 0x70000018,
    /// Delta C++ class instances.
    MipsDeltaInstance = 0x70000019,
    MipsDeltaInstanceNo = 0x7000001a,
    /// Delta relocations.
    MipsDeltaReloc = 0x7000001b,
    MipsDeltaRelocNo = 0x7000001c,
    MipsDeltaSym = 0x7000001d,
    MipsDeltaSymNo = 0x7000001e,
    MipsDeltaClasssym = 0x70000020,
    MipsDeltaClasssymNo = 0x70000021,
    /// Flags indicating for C++ flavor.
    MipsCxxFlags = 0x70000022,
    MipsPixieInit = 0x70000023,
    MipsSymbolLib = 0x70000024,
    MipsLocalpageGotidx = 0x70000025,
    MipsLocalGotidx = 0x70000026,
    MipsHiddenGotidx = 0x70000027,
    MipsProtectedGotidx = 0x70000028,
    /// Address of .options.
    MipsOptions = 0x70000029,
    /// Address of .interface.
    MipsInterface = 0x7000002a,
    MipsDynstrAlign = 0x7000002b,
    /// Size of the .interface section.
    MipsInterfaceSize = 0x7000002c,
    MipsRldTextResolveAddr = 0x7000002d,
    MipsPerfSuffix = 0x7000002e,
    /// (O32)Size of compact rel section.
    MipsCompactSize = 0x7000002f,
    /// GP value for aux GOTs.
    MipsGpValue = 0x70000030,
    /// Address of aux .dynamic.
    MipsAuxDynamic = 0x70000031,
    MipsPltgot = 0x70000032,
    MipsRwplt = 0x70000034,
    MipsRldMapRel = 0x70000035,
    MipsXhash = 0x70000036,
    MipsNumOrAlphaPltroOrPpcGot = 0x37,
    PpcOptOrPpc64Glink = 0x38,
    Ppc64Opd = 0x39,
    Ppc64Opdsz = 0x3b,
    Ppc64Opt = 0x3e,
    AArch64BtiPlt = 0x3f,
    AArch64PacPlt = 0x42,
    AArch64VariantPcsOrIa64PltReserve = 0x47,
}

pub enum NoteType {
    /// Contains copy of prstatus struct Or Contains a version string Or GnuAbiTag
    PrstatusOrVersionOrGnuAbiTag = 0x1,
    /// Prfpreg Or Contains copy of fpregset struct Or GnuHwcap
    PrfpregOrFpregsetOrGnuHwcap = 0x2,
    /// Contains copy of prpsinfo struct Or GnuBuildId
    PrpsinfoOrGnuBuildId = 0x3,
    /// Contains copy of prxregset struct Or Contains copy of task structure Or GnuGoldVersion
    PrxregOrTaskstructOrGnuGoldVersion = 0x4,
    /// String from sysinfo(SI_PLATFORM) Or GnuPropertyType0
    PlatformOrGnuPropertyType0 = 0x5,
    /// Contains copy of auxv array
    Auxv = 0x6,
    /// Contains copy of gwindows struct
    Gwindows = 0x7,
    /// Contains copy of asrset struct
    Asrs = 0x8,
    /// Contains copy of pstatus struct
    Pstatus = 0xa,
    /// Contains copy of psinfo struct
    Psinfo = 0xd,
    /// Contains copy of prcred struct
    Prcred = 0xe,
    /// Contains copy of utsname struct
    Utsname = 0xf,
    /// Contains copy of lwpstatus struct
    Lwpstatus = 0x10,
    /// Contains copy of lwpinfo struct
    Lwpsinfo = 0x11,
    /// Contains copy of fprxregset struct
    Prfpxreg = 0x14,
    Siginfo = 0x53494749,
    File = 0x46494c45,
    /// Contains copy of user_fxsr_struct
    Prxfpreg = 0x46e62b7f,
    /// PowerPC Altivec/VMX registers
    PpcVmx = 0x100,
    /// PowerPC SPE/EVR registers
    PpcSpe = 0x101,
    /// PowerPC VSX registers
    PpcVsx = 0x102,
    /// Target Address Register
    PpcTar = 0x103,
    /// Program Priority Register
    PpcPpr = 0x104,
    /// Data Stream Control Register
    PpcDscr = 0x105,
    /// Event Based Branch Registers
    PpcEbb = 0x106,
    /// Performance Monitor Registers
    PpcPmu = 0x107,
    /// TM checkpointed GPR Registers
    PpcTmCgpr = 0x108,
    /// TM checkpointed FPR Registers
    PpcTmCfpr = 0x109,
    /// TM checkpointed VMX Registers
    PpcTmCvmx = 0x10a,
    /// TM checkpointed VSX Registers
    PpcTmCvsx = 0x10b,
    /// TM Special Purpose Registers
    PpcTmSpr = 0x10c,
    PpcTmCtar = 0x10d,
    PpcTmCppr = 0x10e,
    PpcTmCdscr = 0x10f,
    PpcPkey = 0x110,
    /// i386 TLS slots (struct user_desc)
    I386Tls = 0x200,
    /// x86 io permission bitmap (1=deny)
    I386Ioperm = 0x201,
    /// x86 extended state using xsave
    X86Xstate = 0x202,
    /// s390 upper register halves
    S390HighGprs = 0x300,
    /// s390 timer register
    S390Timer = 0x301,
    /// s390 TOD clock comparator register
    S390Todcmp = 0x302,
    /// s390 TOD programmable register
    S390Todpreg = 0x303,
    /// s390 control registers
    S390Ctrs = 0x304,
    /// s390 prefix register
    S390Prefix = 0x305,
    /// s390 breaking event address
    S390LastBreak = 0x306,
    /// s390 system call restart data
    S390SystemCall = 0x307,
    /// s390 transaction diagnostic block
    S390Tdb = 0x308,
    S390VxrsLow = 0x309,
    /// s390 vector registers 16-31.
    S390VxrsHigh = 0x30a,
    /// s390 guarded storage registers.
    S390GsCb = 0x30b,
    S390GsBc = 0x30c,
    /// s390 runtime instrumentation.
    S390RiCb = 0x30d,
    /// ARM VFP/NEON registers
    ArmVfp = 0x400,
    /// ARM TLS register
    ArmTls = 0x401,
    /// ARM hardware breakpoint registers
    ArmHwBreak = 0x402,
    /// ARM hardware watchpoint registers
    ArmHwWatch = 0x403,
    /// ARM system call number
    ArmSystemCall = 0x404,
    ArmSve = 0x405,
    ArmPacMask = 0x406,
    ArmPacaKeys = 0x407,
    ArmPacgKeys = 0x408,
    ArmTaggedAddrCtrl = 0x409,
    ArmPacEnabledKeys = 0x40a,
    /// Vmcore Device Dump Note.
    Vmcoredd = 0x700,
    /// MIPS DSP ASE registers.
    MipsDsp = 0x800,
    /// MIPS floating-point mode.
    MipsFpMode = 0x801,
    /// MIPS SIMD registers.
    MipsMsa = 0x802,
    FdoPackagingMetadata = 0xcafe1a7e,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ElfFlags: u32 {
        const CPU32 = 0x810000;
        const SPARCV9_MM_OR_PPC64_ABI_OR_SH3 = 0x3;
        /// SPARCV9_TSO Or -mips1 code Or ARM_EABI_UNKNOWN Or SH_UNKNOWN Or RISCV_FLOAT_ABI_SOFT
        const SPARCV9_TSO_OR_MIPS_ARCH_1_OR_ARM_EABI_UNKNOWN_OR_SH_UNKNOWN_OR_RISCV_FLOAT_ABI_SOFT = 0x0;
        /// SPARCV9_PSO Or A .noreorder directive was used Or All addresses must be < 2GB Or ARM_RELEXEC Or SH1 Or High GPRs kernel facility needed Or RISCV_RVC
        const SPARCV9_PSO_OR_MIPS_NOREORDER_OR_ALPHA_32BIT_OR_ARM_RELEXEC_OR_SH1_OR_S390_HIGH_GPRS_OR_RISCV_RVC = 0x1;
        /// SPARCV9_RMO Or Contains PIC code Or Relocations for relaxing exist Or ARM_HASENTRY Or SH2 Or RISCV_FLOAT_ABI_SINGLE
        const SPARCV9_RMO_OR_MIPS_PIC_OR_ALPHA_CANRELAX_OR_ARM_HASENTRY_OR_SH2_OR_RISCV_FLOAT_ABI_SINGLE = 0x2;
        /// little endian data Or ARM_BE8
        const SPARC_LEDATA_OR_ARM_BE8 = 0x800000;
        const SPARC_EXT_MASK = 0xffff00;
        /// generic V8+ features Or ARM_OLD_ABI
        const SPARC_32PLUS_OR_ARM_OLD_ABI = 0x100;
        /// Sun UltraSPARC1 extensions Or Uses FP64 (12 callee-saved) Or ARM_SOFT_FLOAT Or NB conflicts with EF_ARM_SOFT_FLOAT
        const SPARC_SUN_US1_OR_MIPS_FP64_OR_ARM_SOFT_FLOAT_OR_ARM_ABI_FLOAT_SOFT = 0x200;
        /// HAL R1 extensions Or Uses IEEE 754-2008 NaN encoding Or ARM_VFP_FLOAT Or NB conflicts with EF_ARM_VFP_FLOAT
        const SPARC_HAL_R1_OR_MIPS_NAN2008_OR_ARM_VFP_FLOAT_OR_ARM_ABI_FLOAT_HARD = 0x400;
        /// Sun UltraSPARCIII extensions Or ARM_MAVERICK_FLOAT
        const SPARC_SUN_US3_OR_ARM_MAVERICK_FLOAT = 0x800;
        /// Uses PIC calling sequence Or ARM_INTERWORK Or ARM_SYMSARESORTED Or SH_DSP Or RISCV_FLOAT_ABI_DOUBLE
        const MIPS_CPIC_OR_ARM_INTERWORK_OR_ARM_SYMSARESORTED_OR_SH_DSP_OR_RISCV_FLOAT_ABI_DOUBLE = 0x4;
        const MIPS_XGOT_OR_ARM_APCS_26_OR_ARM_DYNSYMSUSESEGIDX_OR_SH3E = 0x8;
        /// MIPS_64BIT_WHIRL Or ARM_APCS_FLOAT Or ARM_MAPSYMSFIRST Or 64-bit ABI Or SH4_NOFPU
        const MIPS_64BIT_WHIRL_OR_ARM_APCS_FLOAT_OR_ARM_MAPSYMSFIRST_OR_IA_64_ABI64_OR_SH4_NOFPU = 0x10;
        const MIPS_ABI2_OR_ARM_PIC = 0x20;
        /// MIPS_ABI_ON32 Or 8-bit structure alignment is in use
        const MIPS_ABI_ON32_OR_ARM_ALIGN8 = 0x40;
        /// MIPS architecture level Or CSKY_ABIMASK
        const MIPS_ARCH_OR_CSKY_ABIMASK = 0xf0000000;
        /// -mips2 code Or CSKY_ABIV1
        const MIPS_ARCH_2_OR_CSKY_ABIV1 = 0x10000000;
        /// -mips3 code Or CSKY_ABIV2
        const MIPS_ARCH_3_OR_CSKY_ABIV2 = 0x20000000;
        /// -mips4 code.
        const MIPS_ARCH_4 = 0x30000000;
        /// -mips5 code.
        const MIPS_ARCH_5 = 0x40000000;
        /// MIPS32 code.
        const MIPS_ARCH_32 = 0x50000000;
        /// MIPS64 code.
        const MIPS_ARCH_64 = 0x60000000;
        /// MIPS32r2 code.
        const MIPS_ARCH_32R2 = 0x70000000;
        /// MIPS64r2 code Or PowerPC embedded flag
        const MIPS_ARCH_64R2_OR_PPC_EMB = 0x80000000;
        /// Trap nil pointer dereference Or PowerPC -mrelocatable flag
        const PARISC_TRAPNIL_OR_PPC_RELOCATABLE = 0x10000;
        /// Program uses arch. extensions.
        const PARISC_EXT = 0x20000;
        /// Program expects little endian.
        const PARISC_LSB = 0x40000;
        /// Program expects wide mode.
        const PARISC_WIDE = 0x80000;
        const PARISC_NO_KABP = 0x100000;
        /// Allow lazy swapping Or ARM_LE8
        const PARISC_LAZYSWAP_OR_ARM_LE8 = 0x400000;
        /// Architecture version Or CSKY_PROCESSOR
        const PARISC_ARCH_OR_CSKY_PROCESSOR = 0xffff;
        const PPC_RELOCATABLE_LIB = 0x8000;
        const ARM_NEW_ABI = 0x80;
        /// ARM_EABIMASK Or arch. version mask
        const ARM_EABIMASK_OR_IA_64_ARCH = 0xff000000;
        const ARM_EABI_VER1 = 0x1000000;
        const ARM_EABI_VER2 = 0x2000000;
        const ARM_EABI_VER3 = 0x3000000;
        const ARM_EABI_VER4 = 0x4000000;
        const ARM_EABI_VER5 = 0x5000000;
        const CSKY_OTHER = 0xfff0000;
        /// os-specific flags
        const IA_64_MASKOS = 0xf;
        const SH_MACH_MASK = 0x1f;
        const SH3_DSP = 0x5;
        const SH4AL_DSP_OR_RISCV_FLOAT_ABI_OR_RISCV_FLOAT_ABI_QUAD = 0x6;
        const SH4 = 0x9;
        const SH2E = 0xb;
        const SH4A = 0xc;
        const SH2A = 0xd;
        const SH4A_NOFPU = 0x11;
        const SH4_NOMMU_NOFPU = 0x12;
        const SH2A_NOFPU = 0x13;
        const SH3_NOMMU = 0x14;
        const SH2A_SH4_NOFPU = 0x15;
        const SH2A_SH3_NOFPU = 0x16;
        const SH2A_SH4 = 0x17;
        const SH2A_SH3E = 0x18;

    }
}
impl_binary_serde_for_bitflags_ty! {ElfFlags}
