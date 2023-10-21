use crate::{ElfParser, VariantStructBinaryDeserialize};
use binary_serde::{
    binary_serde_bitfield, impl_binary_serde_for_bitflags_ty, BinarySerde, BitfieldBitOrder,
    Endianness,
};
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

#[binary_serde_bitfield(order = BitfieldBitOrder::LsbFirst)]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct RelocationEntryInfo32 {
    #[bits(8)]
    pub ty: RelocationTypeU8,

    #[bits(24)]
    pub symbol_index: u32,
}

#[derive(Debug, BinarySerde, PartialEq, Eq, Clone, Hash)]
pub struct RelocationEntry32 {
    pub offset: u32,
    pub info: RelocationEntryInfo32,
}

#[derive(Debug, BinarySerde, PartialEq, Eq, Clone, Hash)]
pub struct RelocationEntry64 {
    pub offset: u64,
    pub ty: RelocationType,
    pub symbol_index: u32,
}

#[derive(Debug, BinarySerde, PartialEq, Eq, Clone, Hash)]
pub struct RelocationEntryMips64 {
    pub offset: u64,
    pub symbol_index: u32,
    pub special_symbol_index: u8,
    pub ty3: RelocationTypeU8,
    pub ty2: RelocationTypeU8,
    pub ty: RelocationTypeU8,
}

pub enum RelocationEntry {
    RelocationEntry32(RelocationEntry32),
    RelocationEntry64(RelocationEntry64),
    RelocationEntryMips64(RelocationEntryMips64),
}
impl RelocationEntry {}

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
    /// Start OS-specific Or Ia64VmsTrace
    LoosOrIa64VmsTrace = 0x60000000,
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
    /// Version definition section Or Versions defined by file
    GnuVerdefOrSunwVerdef = 0x6ffffffd,
    /// Version needs section Or Versions needed by file
    GnuVerneedOrSunwVerneed = 0x6ffffffe,
    /// Version symbol table Or Sun-specific high bound Or End OS-specific type Or Symbol versions
    GnuVersymOrHisunwOrHiosOrSunwVersym = 0x6fffffff,
    /// Start of processor-specific Or Shared objects used in link Or Contains product specific ext Or V850Scommon
    LoprocOrMipsLiblistOrPariscExtOrV850Scommon = 0x70000000,
    /// End of processor-specific
    Hiproc = 0x7fffffff,
    /// Start of application-specific
    Louser = 0x80000000,
    /// End of application-specific
    Hiuser = 0x8fffffff,
    /// MipsMsym Or Unwind information Or AlphaDebug Or Unwind information Or V850Tcommon Or Section holds ARM unwind info Or C6000Unwind Or Section holds attributes Or Section holds attributes
    MipsMsymOrPariscUnwindOrAlphaDebugOrX8664UnwindOrV850TcommonOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributes =
        0x70000001,
    /// Conflicting symbols Or Debug info for optimized code Or AlphaReginfo Or V850Zcommon Or Section pre-emption details Or C6000Preemptmap
    MipsConflictOrPariscDocOrAlphaReginfoOrV850ZcommonOrArmPreemptmapOrC6000Preemptmap = 0x70000002,
    /// Global data area sizes Or Section holds attributes Or PariscAnnot Or Section holds ABI attributes Or Section holds attributes Or C6000Attributes Or Section holds attributes
    MipsGptabOrRiscvAttributesOrPariscAnnotOrMsp430AttributesOrArmAttributesOrC6000AttributesOrAArch64Attributes =
        0x70000003,
    /// Reserved for SGI/MIPS compilers Or PariscDlkm Or Section holds overlay debug info
    MipsUcodeOrPariscDlkmOrArmDebugoverlay = 0x70000004,
    /// MIPS ECOFF debugging info Or Section holds GDB and overlay integration info
    MipsDebugOrArmOverlaysection = 0x70000005,
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
    /// unwind bits Or Used by Renesas linker
    Ia64UnwindOrRenesasIop = 0x70000033,
    /// RenesasInfo Or Ordered Or Extension bits
    RenesasInfoOrOrderedOrIa64Ext = 0xa0000000,
    /// Unwind bits.
    Ia64Unwind = 0xa0000001,
    Ia64Lopsreg = 0xa8000001,
    Ia64Hipsreg = 0xb1000000,
    Ia64PriorityInit = 0xba000000,
    Ia64HpOptAnotOrIa64VmsLinkages = 0x60000004,
    Ia64VmsTieSignatures = 0x60000001,
    Ia64VmsDebug = 0x60000002,
    Ia64VmsDebugStr = 0x60000003,
    Ia64VmsSymbolVector = 0x60000005,
    Ia64VmsFixup = 0x60000006,
    Ia64VmsDisplayNameInfo = 0x60000007,
    PariscSymextn = 0x6000000f,
    PariscStubsOrHpOvlbits = 0x60000018,
    HpDlkm = 0x60000019,
    HpComdat = 0x6000001b,
    HpObjdict = 0x6000001e,
    HpAnnot = 0x60000022,
    MipsAbiflagsOrOrdered = 0x7000002a,
    /// Holds TI compiler's section flags.
    Msp430SecFlags = 0x7f000005,
    /// Holds TI compiler's symbol aliases.
    Msp430SymAliases = 0x7f000006,
    NfpMeconfig = 0x7f000007,
    NfpInitregOrNfpUdebug = 0x7f000009,
    TiIcode = 0x7f000000,
    TiXref = 0x7f000001,
    TiHandler = 0x7f000002,
    TiInitinfo = 0x7f000003,
    TiPhattrs = 0x7f000004,
    /// RELR relative relocations
    Relr = 0x13,
    /// incremental build data Or GnuVerdef Or GnuVerneed Or GnuVersym
    GnuIncrementalInputsOrGnuVerdefOrGnuVerneedOrGnuVersym = 0x6fff4700,
    /// New value, defined in Oct 4, 1999 Draft
    NewHiuser = 0xffffffff,
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

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum RelocationType {
    /// No reloc Or No reloc Or No reloc Or No reloc Or No reloc Or No reloc Or PpcNone Or No relocation Or No reloc Or no reloc Or none Or ShNone Or No reloc Or CrisNone Or No reloc Or No reloc Or No reloc Or No reloc Or No reloc Or No reloc Or No reloc Or RiscvNone Or No reloc Or MetagHiaddr16 Or Nds32None Or ArcNone Or Or1KNone
    M68KNoneOrI386NoneOrSparcNoneOrMipsNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrIa64NoneOrShNoneOrS390NoneOrCrisNoneOrX8664NoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrTilegxNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrArcNoneOrOr1KNone =
        0x0,
    /// Direct 32 bit Or Direct 32 bit Or Direct 8 bit Or Direct 16 bit Or Direct 32-bit reference Or Direct 32 bit Or 32bit absolute address Or Direct 32 bit Or ArmPc24 Or direct 32 bit (S + A) Or ShDir32 Or Direct 8 bit Or Cris8 Or Direct 64 bit Or Direct 32 bit Or Direct 16 bit Or Direct 32 bit Or Direct signed 16 bit Or Direct 32 bit Or Direct 64 bit Or Riscv32 Or Bpf6464 Or MetagLoaddr16 Or Arc8 Or Or1K32
    M68K32OrI38632OrSparc8OrMips16OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrArmPc24OrCkcoreAddr32OrShDir32OrS3908OrCris8OrX866464OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrTilegx64OrRiscv32OrBpf6464OrMetagLoaddr16OrArc8OrOr1K32 =
        0x1,
    /// Direct 16 bit Or PC relative 32 bit Or Direct 16 bit Or Direct 32 bit Or Left 21 bits of eff. address Or Direct 64 bit Or 26bit address, 2 bits ignored Or Direct 32 bit Or disp ((S + A - P) >> 2) & 0xff Or ShRel32 Or Direct 12 bit Or Cris16 Or PC relative 32 bit signed Or Direct 16 bit Or Direct 32 bit Or PC relative 32 bit Or Direct unsigned 16 bit Or Direct 16 bit Or Direct 32 bit Or Riscv64 Or 32bit absolute address Or Arc16 Or Or1K16
    M68K16OrI386Pc32OrSparc16OrMips32OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrArmAbs32OrCkcorePcrelimm8By4OrShRel32OrS39012OrCris16OrX8664Pc32OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrTilegx32OrRiscv64OrMetagAddr32OrArc16OrOr1K16 =
        0x2,
    /// Direct 8 bit Or 32 bit GOT entry Or Direct 32 bit Or PC relative 32 bit Or Right 17 bits of eff. address Or GP relative 32 bit Or 16bit absolute address Or PC relative 32 bit Or disp ((S + A - P) >> 1) & 0x7ff Or ShDir8Wpn Or Direct 16 bit Or Cris32 Or 32 bit GOT entry Or Direct 8 bit Or Direct 24 bit Or PC relative 64 bit Or PC relative 16 bit Or Direct 8 bit Or Direct 16 bit Or RiscvRelative Or No reloc Or Arc24 Or Or1K8
    M68K8OrI386Got32OrSparc32OrMipsRel32OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrArmRel32OrCkcorePcrelimm11By2OrShDir8WpnOrS39016OrCris32OrX8664Got32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrTilegx16OrRiscvRelativeOrMetagNoneOrArc24OrOr1K8 =
        0x3,
    /// PC relative 32 bit Or 32 bit PLT address Or PC relative 8 bit Or Direct 26 bit shifted Or 17 bits of eff. address Or GP relative 16 bit w/optimization Or lower 16bit of absolute address Or ArmPc13 Or ShInd12W Or Direct 32 bit Or Cris8Pcrel Or 32 bit PLT address Or PC-relative 32-bit Or PC relative 10 bit shifted Or Low 16 bits of PCREL32 Or Direct call Or PC relative 32 bit Or Direct 8 bit Or RiscvCopy Or MetagRelbranch Or Arc32 Or Or1KLo16InInsn
    M68KPc32OrI386Plt32OrSparcDisp8OrMips26OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrS39032OrCris8PcrelOrX8664Plt32OrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrTilegx8OrRiscvCopyOrMetagRelbranchOrArc32OrOr1KLo16InInsn =
        0x4,
    /// PC relative 16 bit Or Copy symbol at runtime Or PC relative 16 bit Or High 16 bit Or Optimization hint for LITERAL Or high 16bit of absolute address Or Direct 16 bit Or 32-bit rel (S + A - P) Or ShDir8Wpl Or PC relative 32 bit Or Cris16Pcrel Or Copy symbol at runtime Or PC-relative 16-bit signed Or PC relative 18 bit shifted Or Direct 64 bit Or 5 bit constant expression Or PC relative 16 bit Or PC relative 64 bit Or RiscvJumpSlot Or MetagGetsetoff Or ArcB26 Or Or1KHi16InInsn
    M68KPc16OrI386CopyOrSparcDisp16OrMipsHi16OrAlphaLituseOrPpcAddr16HiOrArmAbs16OrCkcorePcrel32OrShDir8WplOrS390Pc32OrCris16PcrelOrX8664CopyOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrTilegx64PcrelOrRiscvJumpSlotOrMetagGetsetoffOrArcB26OrOr1KHi16InInsn =
        0x5,
    /// PC relative 8 bit Or Create GOT entry Or PC relative 32 bit Or Low 16 bit Or Right 14 bits of eff. address Or Add displacement to GP Or adjusted high 16bit Or Direct 12 bit Or disp ((S + A - P) >>1) & 0x7ff Or ShDir8Wpz Or 12 bit GOT offset Or Cris32Pcrel Or Create GOT entry Or PC-relative 8-bit signed Or PC relative 26 bit shifted Or Low 16 bit Or 5 bit expression, shift 22 Or PC relative 8 bit Or PC relative 32 bit Or RiscvTlsDtpmod32 Or MetagReg32Op1 Or ArcB22Pcrel Or Or1KInsnRel26
    M68KPc8OrI386GlobDatOrSparcDisp32OrMipsLo16OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrArmAbs12OrCkcorePcreljsrImm11By2OrShDir8WpzOrS390Got12OrCris32PcrelOrX8664GlobDatOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrTilegx32PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrArcB22PcrelOrOr1KInsnRel26 =
        0x6,
    /// 32 bit PC relative GOT entry Or Create PLT entry Or PC relative 30 bit shifted Or GP relative 16 bit Or PC+4 relative 23 bit shifted Or 16bit address, 2 bits ignored Or Direct & 0x7C (LDR, STR) Or ShDir8Bp Or 32 bit GOT offset Or CrisGnuVtinherit Or Create PLT entry Or Ancient C++ vtable garbage.. Or High 16 bit with unsigned low Or Read-only small data area Or 6 bit constant expression Or Low 16 bit Or PC relative 16 bit Or RiscvTlsDtpmod64 Or MetagReg32Op2 Or ArcH30 Or Or1KGnuVtentry
    M68KGot32OrI386JmpSlotOrSparcWdisp30OrMipsGprel16OrAlphaBraddrOrPpcAddr14OrArmThmAbs5OrShDir8BpOrS390Got32OrCrisGnuVtinheritOrX8664JumpSlotOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrTilegx16PcrelOrRiscvTlsDtpmod64OrMetagReg32Op2OrArcH30OrOr1KGnuVtentry =
        0x7,
    /// 16 bit PC relative GOT entry Or Adjust by program base Or PC relative 22 bit shifted Or 16 bit literal entry Or PC+4 relative 16 bit shifted Or PpcAddr14Brtaken Or Direct 8 bit Or ShDir8W Or 32 bit PC relative PLT address Or CrisGnuVtentry Or Adjust by program base Or ... collection annotation Or High 16 bit with signed low Or Read-write small data area Or 8 bit constant expression Or High 16 bit Or PC relative 8 bit Or RiscvTlsDtprel32 Or MetagReg32Op3 Or ArcN8 Or Or1KGnuVtinherit
    M68KGot16OrI386RelativeOrSparcWdisp22OrMipsLiteralOrAlphaHintOrPpcAddr14BrtakenOrArmAbs8OrShDir8WOrS390Plt32OrCrisGnuVtentryOrX8664RelativeOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrTilegx8PcrelOrRiscvTlsDtprel32OrMetagReg32Op3OrArcN8OrOr1KGnuVtinherit =
        0x8,
    /// 8 bit PC relative GOT entry Or 32 bit offset to GOT Or High 22 bit Or 16 bit GOT entry Or 32-bit rel. address Or PC relative 16 bit Or PpcAddr14Brntaken Or ArmSbrel32 Or 32 bit adjust program base(B + A) Or ShDir8L Or Copy symbol at runtime Or CrisCopy Or X8664Gotpcrel Or Direct 24 bit Or Low 16 bit Or No reloc Or High 16 bit Or High 16 bit, adjusted Or hword 0 16-bit Or RiscvTlsDtprel64 Or MetagReg16Op1 Or ArcN16 Or Or1K32Pcrel
    M68KGot8OrI386GotoffOrSparcHi22OrMipsGot16OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrArmSbrel32OrCkcoreRelativeOrShDir8LOrS390CopyOrCrisCopyOrX8664GotpcrelOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrTilegxHw0OrRiscvTlsDtprel64OrMetagReg16Op1OrArcN16OrOr1K32Pcrel =
        0x9,
    /// 32 bit GOT offset Or 32 bit PC relative offset to GOT Or Direct 22 bit Or PC relative 16 bit Or Left 21 bits of rel. address Or PC relative 32 bit Or PC relative 26 bit Or PC relative 24 bit (Thumb32 BL) Or 32 bit adjust by program base Or Create GOT entry Or CrisGlobDat Or Direct 32 bit zero extended Or 32-bit PCrel offset to GOT Or 16 bit offset in SDA Or Symbol Op Symbol relocation Or Low 16 bit Or Copy relocation Or hword 1 16-bit Or RiscvTlsTprel32 Or Bpf6432 Or MetagReg16Op2 Or ArcN24 Or Or1K16Pcrel
    M68KGot32OOrI386GotpcOrSparc22OrMipsPc16OrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrS390GlobDatOrCrisGlobDatOrX866432OrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrTilegxHw1OrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrArcN24OrOr1K16Pcrel =
        0xa,
    /// 16 bit GOT offset Or I38632Plt Or Direct 13 bit Or 16 bit GOT entry for function Or Right 17 bits of rel. address Or PC relative 64 bit Or PC relative 16 bit Or ArmThmPc8 Or off between got and sym (S) Or Create PLT entry Or CrisJumpSlot Or Direct 32 bit sign extended Or 16-bit PCrel offset to GOT Or M32RGnuVtinherit Or GNU C++ vtable hierarchy Or High 16 bit, adjusted Or Create GOT entry Or hword 2 16-bit Or RiscvTlsTprel64 Or MetagReg16Op3 Or ArcN32 Or Or1K8Pcrel
    M68KGot16OOrI38632PltOrSparc13OrMipsCall16OrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrArmThmPc8OrCkcoreGlobDatOrS390JmpSlotOrCrisJumpSlotOrX866432SOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrTilegxHw2OrRiscvTlsTprel64OrMetagReg16Op3OrArcN32OrOr1K8Pcrel =
        0xb,
    /// 8 bit GOT offset Or Truncated 10 bit Or GP relative 32 bit Or 17 bits of rel. address Or PpcRel14Brtaken Or ArmAmpVcall9 Or PLT entry (S) Or Adjust by program base Or CrisRelative Or Direct 16 bit zero extended Or 32-bit offset from GOT Or M32RGnuVtentry Or GNU C++ vtable member usage Or 32 bit symbol value + addend Or Create PLT entry Or hword 3 16-bit Or MetagReg32Op4 Or ArcSda Or Or1KGotpcHi16
    M68KGot8OOrSparcLo10OrMipsGprel32OrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrS390RelativeOrCrisRelativeOrX866416OrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrTilegxHw3OrMetagReg32Op4OrArcSdaOrOr1KGotpcHi16 =
        0xc,
    /// 32 bit PC relative PLT address Or Truncated 10 bit GOT entry Or PpcRel14Brntaken Or Obsolete static relocation Or Dynamic relocation Or offset to GOT (S + A - GOT) Or 32 bit offset to GOT Or Cris16Got Or 16 bit sign extended pc relative Or 24-bit offset from GOT Or PC-relative GOT offset Or 16 bit symbol value + addend Or Adjust by program base Or last hword 0 16-bit Or MetagHiog Or ArcSectoff Or Or1KGotpcLo16
    M68KPlt32OrSparcGot10OrPpcRel14BrntakenOrArmSwi24OrArmTlsDescOrCkcoreGotoffOrS390Gotoff32OrCris16GotOrX8664Pc16OrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrTilegxHw0LastOrMetagHiogOrArcSectoffOrOr1KGotpcLo16 =
        0xd,
    /// 16 bit PC relative PLT address Or Offset in static TLS block Or 13 bit GOT entry Or Right 14 bits of rel. address Or PpcGot16 Or Reserved Or PC offset to GOT (GOT + A - P) Or 32 bit PC relative offset to GOT Or Cris32Got Or Direct 8 bit sign extended Or 16-bit offset from GOT Or GOT entry offset Or 8 bit symbol value + addend Or X1 pipe branch offset Or last hword 1 16-bit Or MetagLoog Or ArcS21HPcrel Or Or1KGot16
    M68KPlt16OrI386TlsTpoffOrSparcGot13OrPariscPcrel14ROrPpcGot16OrArmThmSwi8OrCkcoreGotpcOrS390GotpcOrCris32GotOrX86648OrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrTilegxHw1LastOrMetagLoogOrArcS21HPcrelOrOr1KGot16 =
        0xe,
    /// 8 bit PC relative PLT address Or I386TlsIe Or 22 bit GOT entry shifted Or PpcGot16Lo Or Reserved Or 32 bit GOT entry (G) Or 16 bit GOT offset Or Cris16Gotplt Or 8 bit sign extended pc relative Or 32-bit PCrel to PLT entry Or PLT offset (PC-relative) Or 16 bit GP pointer offset Or X1 pipe jump offset Or last hword 2 16-bit Or MetagRel8 Or ArcS21WPcrel Or Or1KPlt26
    M68KPlt8OrI386TlsIeOrSparcGot22OrPpcGot16LoOrArmXpc25OrCkcoreGot32OrS390Got16OrCris16GotpltOrX8664Pc8OrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrTilegxHw2LastOrMetagRel8OrArcS21WPcrelOrOr1KPlt26 =
        0xf,
    /// 32 bit PLT offset Or I386TlsGotie Or PC relative 10 bit truncated Or MipsShift5 Or PpcGot16Hi Or Reserved Or 32 bit PLT entry (G) Or PC relative 16 bit Or Cris32Gotplt Or ID of module containing symbol Or 16-bit PCrel to PLT entry Or Adjust by program base Or GNU C++ vtable hierarchy Or X1 pipe jump offset to PLT Or Copy relocation Or RiscvBranch Or MetagRel16 Or ArcS25HPcrel Or Or1KGotoffHi16
    M68KPlt32OOrI386TlsGotieOrSparcPc10OrMipsShift5OrPpcGot16HiOrArmThmXpc22OrCkcorePlt32OrS390Pc16OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrTilegxCopyOrRiscvBranchOrMetagRel16OrArcS25HPcrelOrOr1KGotoffHi16 =
        0x10,
    /// 16 bit PLT offset Or I386TlsLe Or PC relative 22 bit shifted Or MipsShift6 Or GP relative 32 bit, high 16 bits Or PpcGot16Ha Or ID of module containing symbol Or GOT entry in GLOB_DAT (GOT + G) Or PC relative 16 bit shifted by 1 Or Cris32Gotrel Or Offset in module's TLS block Or 32-bit offset to GOT entry Or Create PLT entry Or GNU C++ vtable member usage Or X0 pipe 8-bit Or Create GOT entry Or RiscvJal Or ArcS25WPcrel Or Or1KGotoffLo16
    M68KPlt16OOrI386TlsLeOrSparcPc22OrMipsShift6OrAlphaGprelhighOrPpcGot16HaOrArmTlsDtpmod32OrCkcoreAddrgotOrS390Pc16DblOrCris32GotrelOrX8664Dtpoff64OrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrTilegxGlobDatOrRiscvJalOrArcS25WPcrelOrOr1KGotoffLo16 =
        0x11,
    /// 8 bit PLT offset Or I386TlsGd Or 30 bit PC relative PLT address Or Mips64 Or Left 21 bits of rel. address Or GP relative 32 bit, low 16 bits Or PpcPltrel24 Or Offset in TLS block Or PLT entry in GLOB_DAT (GOT + G) Or 16 bit PC rel. PLT shifted by 1 Or Cris32PltGotrel Or Offset in initial TLS block Or 24-bit offset to GOT entry Or Create GOT entry Or Unconditional branch Or Y0 pipe 8-bit Or Create PLT entry Or RiscvCall Or ArcSda32 Or Or1KCopy
    M68KPlt8OOrI386TlsGdOrSparcWplt30OrMips64OrPariscDprel21LOrAlphaGprellowOrPpcPltrel24OrArmTlsDtpoff32OrCkcoreAddrpltOrS390Plt16DblOrCris32PltGotrelOrX8664Tpoff64OrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrTilegxJmpSlotOrRiscvCallOrArcSda32OrOr1KCopy =
        0x12,
    /// Copy symbol at runtime Or I386TlsLdm Or Copy symbol at runtime Or MipsGotDisp Or GP relative 16 bit Or PpcCopy Or Offset in static TLS block Or ((S + A - P) >> 1) & 0x3ffffff Or PC relative 32 bit shifted by 1 Or Cris32PltPcrel Or X8664Tlsgd Or 16-bit offset to GOT entry Or 64 bit offset to GOT Or Conditional branch Or X1 pipe 8-bit Or Adjust by program base Or RiscvCallPlt Or ArcSdaLdst Or Or1KGlobDat
    M68KCopyOrI386TlsLdmOrSparcCopyOrMipsGotDispOrAlphaGprel16OrPpcCopyOrArmTlsTpoff32OrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrX8664TlsgdOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrTilegxRelativeOrRiscvCallPltOrArcSdaLdstOrOr1KGlobDat =
        0x13,
    /// Create GOT entry Or I38616 Or Create GOT entry Or MipsGotPage Or PpcGlobDat Or Copy symbol at runtime Or disp ((S + A - P) >> 1) & 0xffff Or 32 bit PC rel. PLT shifted by 1 Or CrisNum Or X8664Tlsld Or Copy symbol at runtime Or 32 bit offset to GOT Or Indirect call through register Or Y1 pipe 8-bit Or X1 pipe branch offset Or RiscvGotHi20 Or Nds3232Rela Or ArcSdaLdst1 Or Or1KJmpSlot
    M68KGlobDatOrI38616OrSparcGlobDatOrMipsGotPageOrPpcGlobDatOrArmCopyOrCkcorePcrelImm16By2OrS390Plt32DblOrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrTilegxBroffX1OrRiscvGotHi20OrNds3232RelaOrArcSdaLdst1OrOr1KJmpSlot =
        0x14,
    /// Create PLT entry Or I386Pc16 Or Create PLT entry Or MipsGotOfst Or PpcJmpSlot Or Create GOT entry Or disp ((S + A - P) >> 2) & 0xffff Or 32 bit PC rel. GOT shifted by 1 Or Offset in TLS block Or Create GOT entry Or Runtime copy Or Nios2Align Or X1 pipe mtspr Or X1 pipe jump offset Or RiscvTlsGotHi20 Or ArcSdaLdst2 Or Or1KRelative
    M68KJmpSlotOrI386Pc16OrSparcJmpSlotOrMipsGotOfstOrPpcJmpSlotOrArmGlobDatOrCkcorePcrelImm16By4OrS390GotpcdblOrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrTilegxJumpoffX1OrRiscvTlsGotHi20OrArcSdaLdst2OrOr1KRelative =
        0x15,
    /// Adjust by program base Or I3868 Or Adjust by program base Or MipsGotHi16 Or Right 14 bits of rel. address Or PpcRelative Or Create PLT entry Or disp ((S + A - P) >> 1) & 0x3ff Or Direct 64 bit Or X8664Gottpoff Or Create PLT entry Or TLS Reloc Or 16 bit GOT entry Or X1 pipe mfspr Or X1 pipe jump offset to PLT Or RiscvTlsGdHi20 Or ArcSda16Ld Or Or1KTlsGdHi16
    M68KRelativeOrI3868OrSparcRelativeOrMipsGotHi16OrPariscDprel14ROrPpcRelativeOrArmJumpSlotOrCkcorePcrelImm10By2OrS39064OrX8664GottpoffOrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrTilegxJumpoffX1PltOrRiscvTlsGdHi20OrArcSda16LdOrOr1KTlsGdHi16 =
        0x16,
    /// 32 bit GOT offset for GD Or Tag for pushl in GD TLS code Or High 22 bit PLT entry Or MipsInsertA Or Create GOT entry Or PpcUaddr16 Or 32 bit PC relative offset to GOT Or (S + A) & 0xffff Or ShSwitch16 Or 64 bit PC relative PLT address Or 64 bit offset to GOT Or 32-bit offset for local dynamic Or TLS Module ID Or %hiadj of offset to GOT pointer Or X0 pipe low 16-bit Or X1 pipe 8-bit Or RiscvPcrelLo12S Or ArcS13Pcrel Or Or1KTlsLdmLo16
    M68KTlsGd32OrI386TlsGdPushOrSparcHiplt22OrMipsInsertAOrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrS390Plt64OrX8664Gotoff64OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrTilegxImm8X1OrRiscvPcrelLo12SOrArcS13PcrelOrOr1KTlsLdmLo16 =
        0x19,
    /// 16 bit GOT offset for GD Or I386TlsGdCall Or Truncated 10 bit PLT entry Or MipsInsertB Or GP-relative, left 21 bits Or Create PLT entry Or PpcRel32 Or 32 bit GOT entry Or high & low 16 bit GOTPC Or ShSwitch32 Or 32 bit PC rel. to GOT entry >> 1 Or X8664Gotpc32 Or Module-relative offset Or TLS Offset Within TLS Block Or %lo of PC relative offset Or X1 pipe low 16-bit Or Y1 pipe 8-bit Or RiscvHi20 Or ArcW Or Or1KTlsLdoHi16
    M68KTlsGd16OrI386TlsGdCallOrSparcLoplt10OrMipsInsertBOrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrS390GotentOrX8664Gotpc32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrTilegxImm8Y1OrRiscvHi20OrArcWOrOr1KTlsLdoHi16 =
        0x1a,
    /// 8 bit GOT offset for GD Or Tag for popl in GD TLS code Or PC rel 32 bit ref to PLT entry Or MipsDelete Or Adjust by program base Or PpcPlt32 Or Deprecated, 32 bit PLT address Or (GOT + A - P) & 0xffff Or ShUses Or 16 bit offset to GOT Or 64-bit GOT entry offset Or Mn10300TlsGotie Or TLS Offset Within TLS Block Or %hiadj of PC relative offset Or X0 pipe high 16-bit Or X1 pipe destination 8-bit Or RiscvLo12I Or Arc32Me Or Or1KTlsLdoLo16
    M68KTlsGd8OrI386TlsGdPopOrSparcPcplt32OrMipsDeleteOrAlphaRelativeOrPpcPlt32OrArmPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrX8664Got64OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrTilegxDestImm8X1OrRiscvLo12IOrArc32MeOrOr1KTlsLdoLo16 =
        0x1b,
    /// 32 bit GOT offset for LDM Or I386TlsLdm32 Or PC rel high 22 bit PLT entry Or MipsHigher Or AlphaTlsGdHi Or PpcPltrel32 Or PC relative 24 bit (BL, BLX) Or high & low 16 bit GOTOFF Or ShCount Or 64 bit offset to GOT Or X8664Gotpcrel64 Or Mn10300TlsIe Or TLS Offset From Thread Pointer Or 16 bit GOT offset for TLS GD Or X1 pipe high 16-bit Or X1 pipe mtspr Or RiscvLo12S Or ArcN32Me Or Or1KTlsIeHi16
    M68KTlsLdm32OrI386TlsLdm32OrSparcPcplt22OrMipsHigherOrAlphaTlsGdHiOrPpcPltrel32OrArmCallOrCkcoreGotoffHi16OrShCountOrS390Gotoff64OrX8664Gotpcrel64OrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrTilegxMtImm14X1OrRiscvLo12SOrArcN32MeOrOr1KTlsIeHi16 =
        0x1c,
    /// 16 bit GOT offset for LDM Or Tag for pushl in LDM TLS code Or PC rel trunc 10 bit PLT entry Or MipsHighest Or AlphaTlsgd Or PpcPlt16Lo Or ArmJump24 Or (S + A - GOT) & 0xffff Or ShAlign Or 12 bit offset to jump slot Or 64-bit PC relative offset to GOT Or Mn10300TlsLe Or TLS Offset From Thread Pointer Or 16 bit GOT offset for TLS LDM Or X0 pipe high 16-bit, adjusted Or X1 pipe mfspr Or RiscvTprelHi20 Or ArcSectoffMe Or Or1KTlsIeLo16
    M68KTlsLdm16OrI386TlsLdmPushOrSparcPcplt10OrMipsHighestOrAlphaTlsgdOrPpcPlt16LoOrArmJump24OrCkcoreGotoffLo16OrShAlignOrS390Gotplt12OrX8664Gotpc64OrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrTilegxMfImm14X1OrRiscvTprelHi20OrArcSectoffMeOrOr1KTlsIeLo16 =
        0x1d,
    /// 8 bit GOT offset for LDM Or I386TlsLdmCall Or Direct 10 bit Or MipsCallHi16 Or GP-relative, right 14 bits Or AlphaTlsLdm Or PpcPlt16Hi Or PC relative 24 bit (Thumb32 B.W) Or 12 bit disp GOT entry (G) Or ShCode Or 16 bit offset to jump slot Or like GOT64, says PLT entry needed Or ID of module containing symbol Or 16 bit module relative offset Or X1 pipe high 16-bit, adjusted Or X0 pipe mm "start" Or RiscvTprelLo12I Or MetagGnuVtinherit Or ArcSda32Me Or Or1KTlsLeHi16
    M68KTlsLdm8OrI386TlsLdmCallOrSparc10OrMipsCallHi16OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrArmThmJump24OrCkcoreGot12OrShCodeOrS390Gotplt16OrX8664Gotplt64OrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrTilegxMmstartX0OrRiscvTprelLo12IOrMetagGnuVtinheritOrArcSda32MeOrOr1KTlsLeHi16 =
        0x1e,
    /// 32 bit module-relative offset Or Tag for popl in LDM TLS code Or Direct 11 bit Or MipsCallLo16 Or AlphaDtpmod64 Or PpcPlt16Ha Or Adjust by program base Or high & low 16 bit GOT Or ShData Or 32 bit offset to jump slot Or X8664Pltoff64 Or Offset in module TLS block Or 16 bit GOT offset for TLS IE Or X0 pipe PC relative 16 bit Or X0 pipe mm "end" Or RiscvTprelLo12S Or MetagGnuVtentry Or ArcWMe Or Or1KTlsLeLo16
    M68KTlsLdo32OrI386TlsLdmPopOrSparc11OrMipsCallLo16OrAlphaDtpmod64OrPpcPlt16HaOrArmBaseAbsOrCkcoreGotHi16OrShDataOrS390Gotplt32OrX8664Pltoff64OrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrTilegxMmendX0OrRiscvTprelLo12SOrMetagGnuVtentryOrArcWMeOrOr1KTlsLeLo16 =
        0x1f,
    /// 16 bit module-relative offset Or Offset relative to TLS block Or Direct 64 bit Or MipsScnDisp Or AlphaGotdtprel Or PpcSdarel16 Or Obsolete Or (G & 0xffff) Or ShLabel Or 64 bit offset to jump slot Or Size of symbol plus 32-bit addend Or Offset in static TLS block Or 16 bit LE TP-relative offset Or X1 pipe PC relative 16 bit Or X0 pipe shift amount Or RiscvTprelAdd Or MetagHi16Gotoff Or ArcH30Me Or Or1KTlsTpoff
    M68KTlsLdo16OrI386TlsLdo32OrSparc64OrMipsScnDispOrAlphaGotdtprelOrPpcSdarel16OrArmAluPcrel70OrCkcoreGotLo16OrShLabelOrS390Gotplt64OrX8664Size32OrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrTilegxShamtX0OrRiscvTprelAddOrMetagHi16GotoffOrArcH30MeOrOr1KTlsTpoff =
        0x20,
    /// 8 bit module-relative offset Or I386TlsIe32 Or 10bit with secondary 13bit addend Or MipsRel16 Or AlphaDtprel64 Or PpcSectoff Or Obsolete Or 12 bit disp PLT entry (G) Or symbol + addend, add imm14 Or ShSwitch8 Or 32 bit rel. offset to jump slot Or Size of symbol plus 64-bit addend Or Mn10300SymDiff Or Direct 16 bit Or Module number Or X0 pipe PC relative low 16 bit Or X1 pipe shift amount Or RiscvAdd8 Or MetagLo16Gotoff Or ArcSectoffU8 Or Or1KTlsDtpoff
    M68KTlsLdo8OrI386TlsIe32OrSparcOlo10OrMipsRel16OrAlphaDtprel64OrPpcSectoffOrArmAluPcrel158OrCkcorePlt12OrIa64Imm14OrShSwitch8OrS390GotpltentOrX8664Size64OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrTilegxShamtX1OrRiscvAdd8OrMetagLo16GotoffOrArcSectoffU8OrOr1KTlsDtpoff =
        0x21,
    /// 32 bit GOT offset for IE Or I386TlsLe32 Or Top 22 bits of direct 64 bit Or MipsAddImmediate Or LT-relative, left 21 bits Or AlphaDtprelhi Or PpcSectoffLo Or Obsolete Or high & low 16 bit PLT Or symbol + addend, add imm22 Or ShGnuVtinherit Or 16 bit offset from GOT to PLT Or GOT offset for TLS descriptor Or Mn10300Align Or Direct 32 bit Or Module-relative offset Or X1 pipe PC relative low 16 bit Or Y0 pipe shift amount Or RiscvAdd16 Or MetagGetsetGotoff Or ArcSectoffS9 Or Or1KTlsDtpmod
    M68KTlsIe32OrI386TlsLe32OrSparcHh22OrMipsAddImmediateOrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrArmAluPcrel2315OrCkcorePltHi16OrIa64Imm22OrShGnuVtinheritOrS390Pltoff16OrX8664Gotpc32TlsdescOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrTilegxShamtY0OrRiscvAdd16OrMetagGetsetGotoffOrArcSectoffS9OrOr1KTlsDtpmod =
        0x22,
    /// 16 bit GOT offset for IE Or ID of module containing symbol Or High middle 10 bits of .. Or MipsPjump Or AlphaDtprello Or PpcSectoffHi Or Deprecated, prog. base relative Or G & 0xffff Or symbol + addend, mov imm64 Or ShGnuVtentry Or 32 bit offset from GOT to PLT Or X8664TlsdescCall Or Mn10300Num Or Direct 24 bit Or TP-relative offset Or X0 pipe PC relative high 16 bit Or Y1 pipe shift amount Or RiscvAdd32 Or MetagGetsetGot Or AcSectoffU8
    M68KTlsIe16OrI386TlsDtpmod32OrSparcHm10OrMipsPjumpOrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrIa64Imm64OrShGnuVtentryOrS390Pltoff32OrX8664TlsdescCallOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrTilegxShamtY1OrRiscvAdd32OrMetagGetsetGotOrAcSectoffU8 =
        0x23,
    /// 8 bit GOT offset for IE Or Offset in TLS block Or Low middle 22 bits of .. Or MipsRelgot Or AlphaDtprel16 Or PpcSectoffHa Or Deprecated, prog. base relative Or high & low 16 bit ADDRGOT Or symbol + addend, data4 MSB Or 16 bit offset from GOT to PLT Or TLS descriptor Or PC relative 10 bit shifted Or Copy symbol at runtime Or X1 pipe PC relative high 16 bit Or X0 pipe hword 0 Or RiscvAdd64 Or MetagHi16Gotpc Or AcSectoffU81
    M68KTlsIe8OrI386TlsDtpoff32OrSparcLm22OrMipsRelgotOrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrIa64Dir32MsbOrS390Pltoff64OrX8664TlsdescOrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrTilegxImm16X0Hw0OrRiscvAdd64OrMetagHi16GotpcOrAcSectoffU81 =
        0x24,
    /// M68KTlsLe32 Or Negated offset in static TLS block Or Top 22 bits of pc rel 64 bit Or MipsJalr Or AlphaGottprel Or word30 (S + A - P) >> 2 Or Deprecated, prog. base relative Or (GOT + G * 4) & 0xffff Or symbol + addend, data4 LSB Or Tag for load insn in TLS code Or Adjust indirectly by program base Or PC relative 18 bit shifted Or Create GOT entry Or X0 pipe PC relative ha() 16 bit Or X1 pipe hword 0 Or RiscvSub8 Or MetagLo16Gotpc Or AcSectoffU82
    M68KTlsLe32OrI386TlsTpoff32OrSparcPcHh22OrMipsJalrOrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrIa64Dir32LsbOrS390TlsLoadOrX8664IrelativeOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrTilegxImm16X1Hw0OrRiscvSub8OrMetagLo16GotpcOrAcSectoffU82 =
        0x25,
    /// M68KTlsLe16 Or 32-bit symbol size Or High middle 10 bit of .. Or Module number 32 bit Or LT-relative, right 14 bits Or AlphaTprel64 Or doubleword64 S + A Or ArmTarget1 Or high & low 16 bit ADDRPLT Or symbol + addend, data8 MSB Or S390TlsGdcall Or 64-bit adjust by program base Or PC relative 26 bit shifted Or Create PLT entry Or X1 pipe PC relative ha() 16 bit Or X0 pipe hword 1 Or RiscvSub16 Or MetagHi16Plt Or AcSectoffS9
    M68KTlsLe16OrI386Size32OrSparcPcHm10OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrPpc64Addr64OrArmTarget1OrCkcoreAddrpltHi16OrIa64Dir64MsbOrS390TlsGdcallOrX8664Relative64OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrTilegxImm16X0Hw1OrRiscvSub16OrMetagHi16PltOrAcSectoffS9 =
        0x26,
    /// M68KTlsLe8 Or GOT offset for TLS descriptor Or Low miggle 22 bits of .. Or Module-relative offset 32 bit Or AlphaTprelhi Or half16 #higher(S + A) Or Program base relative Or (GOT+G*4) & 0xffff Or symbol + addend, data8 LSB Or S390TlsLdcall Or High 16 bit with unsigned low Or Adjust by program base Or X0 pipe 16-bit GOT offset Or X1 pipe hword 1 Or RiscvSub32 Or MetagLo16Plt Or Nds32Copy Or AcSectoffS91
    M68KTlsLe8OrI386TlsGotdescOrSparcPcLm22OrMipsTlsDtprel32OrAlphaTprelhiOrPpc64Addr16HigherOrArmSbrel31OrCkcoreAddrpltLo16OrIa64Dir64LsbOrS390TlsLdcallOrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrTilegxImm16X1Hw1OrRiscvSub32OrMetagLo16PltOrNds32CopyOrAcSectoffS91 =
        0x27,
    /// 32 bit module number Or I386TlsDescCall Or PC relative 16 bit shifted Or Module number 64 bit Or AlphaTprello Or half16 #highera(S + A) Or ArmV4Bx Or disp ((S+A-P) >>1) & x3ffffff Or S390TlsGd32 Or High 16 bit with signed low Or 16 bit offset to GOT pointer Or X1 pipe 16-bit GOT offset Or X0 pipe hword 2 Or RiscvSub64 Or MetagRelbranchPlt Or Nds32GlobDat Or AcSectoffS92
    M68KTlsDtpmod32OrI386TlsDescCallOrSparcWdisp16OrMipsTlsDtpmod64OrAlphaTprelloOrPpc64Addr16HigheraOrArmV4BxOrCkcorePcrelJsrImm26By2OrS390TlsGd32OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrTilegxImm16X0Hw2OrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92 =
        0x28,
    /// 32 bit module-relative offset Or I386TlsDesc Or PC relative 19 bit shifted Or Module-relative offset 64 bit Or 32 bits section rel. address Or AlphaTprel16 Or half16 #highest(S + A) Or ArmTarget2 Or (S+A-BTEXT) & 0xffff Or S390TlsGd64 Or X8664Gotpcrelx Or Low 16 bit Or Direct call in .noat section Or X0 pipe low 16-bit GOT offset Or X1 pipe hword 2 Or RiscvGnuVtinherit Or MetagGotoff Or Nds32JmpSlot Or ArcSectoffMe1
    M68KTlsDtprel32OrI386TlsDescOrSparcWdisp19OrMipsTlsDtprel64OrPariscSecrel32OrAlphaTprel16OrPpc64Addr16HighestOrArmTarget2OrCkcoreToffsetLo16OrS390TlsGd64OrX8664GotpcrelxOrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrTilegxImm16X1Hw2OrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1 =
        0x29,
    /// 32 bit TP-relative offset Or Adjust indirectly by program base Or was part of v9 ABI but was removed Or 16 bit GOT offset for GD Or half16 #highesta(S + A) Or 32 bit PC relative Or (S+A-BTEXT) & 0xffff Or @gprel(sym + add), add imm22 Or S390TlsGotie12 Or X8664RexGotpcrelx Or 16 bit offset in SDA Or %lo() of GOT entry Or X1 pipe low 16-bit GOT offset Or X0 pipe hword 3 Or RiscvGnuVtentry Or MetagPlt Or Nds32Relative Or ArcSectoffMe2
    M68KTlsTprel32OrI386IrelativeOrSparcGlobJmpOrMipsTlsGdOrPpc64Addr16HighestaOrArmPrel31OrCkcoreDoffsetLo16OrIa64Gprel22OrS390TlsGotie12OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrTilegxImm16X0Hw3OrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrArcSectoffMe2 =
        0x2a,
    /// M68KNum Or I386Got32X Or Direct 7 bit Or 16 bit GOT offset for LDM Or doubleword64 S + A Or Direct 16-bit (MOVW) Or disp ((S+A-P) >>1) & 0x3ffff Or @gprel(sym + add), mov imm64 Or S390TlsGotie32 Or X8664Num Or M32RRelaGnuVtinherit Or %hiadj() of GOT entry Or X0 pipe high 16-bit GOT offset Or X1 pipe hword 3 Or RiscvAlign Or MetagCopy Or ArcSectoff1
    M68KNumOrI386Got32XOrSparc7OrMipsTlsLdmOrPpc64Uaddr64OrArmMovwAbsNcOrCkcorePcrelImm18By2OrIa64Gprel64IOrS390TlsGotie32OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrTilegxImm16X1Hw3OrRiscvAlignOrMetagCopyOrArcSectoff1 =
        0x2b,
    /// I386Pc8 Or Direct 32 bit unaligned Or MipsGotLo16 Or PpcLocal24Pc Or Adjust by program base Or disp ((S + A - P) >> 2) & 0x3ff Or PC relative 64 bit Or Offset in initial TLS block Or Adjust by program base Or TLS General Dynamic Or 16 bit GOT entry for function Or X0 pipe 16-bit Or X0 pipe 8-bit Or RiscvPcrelHi20 Or ArcSda16Ld1 Or Or1KTlsGdLo16
    I386Pc8OrSparcUa32OrMipsGotLo16OrPpcLocal24PcOrArmRelativeOrCkcorePcrelImm10By4OrS390Pc64OrX8664Tpoff32OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrTilegxImm8X0OrRiscvPcrelHi20OrArcSda16Ld1OrOr1KTlsGdLo16 =
        0x17,
    /// I386TlsGd32 Or Direct 32 bit ref to PLT entry Or MipsSub Or Copy symbol at runtime Or PpcUaddr32 Or 32 bit offset to GOT Or high & low 16 bit ADDR Or 64 bit GOT offset Or PC relative 64 bit Or 32-bit offset for global dynamic Or TLS Local Dynamic Or %lo of offset to GOT pointer Or X1 pipe 16-bit Or Y0 pipe 8-bit Or RiscvPcrelLo12I Or ArcSda16Ld2 Or Or1KTlsLdmHi16
    I386TlsGd32OrSparcPlt32OrMipsSubOrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrS390Got64OrX8664Pc64OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrTilegxImm8Y0OrRiscvPcrelLo12IOrArcSda16Ld2OrOr1KTlsLdmHi16 =
        0x18,
    /// I386Num Or Direct 5 bit Or Module-relative offset, high 16 bits Or doubleword64 S + A - P Or Direct high 16-bit (MOVT) Or disp (S+A-BDATA) & 0x3ffff Or @gprel(sym + add), data4 MSB Or S390TlsGotie64 Or M32RRelaGnuVtentry Or %lo() of function GOT entry Or X1 pipe high 16-bit GOT offset Or X0 pipe last hword 0 Or RiscvRvcBranch Or MetagJmpSlot Or ArcSectoff2
    I386NumOrSparc5OrMipsTlsDtprelHi16OrPpc64Rel64OrArmMovtAbsOrCkcoreDoffsetImm18OrIa64Gprel32MsbOrS390TlsGotie64OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrTilegxImm16X0Hw0LastOrRiscvRvcBranchOrMetagJmpSlotOrArcSectoff2 =
        0x2c,
    /// Direct 6 bit Or Module-relative offset, low 16 bits Or doubleword64 L + A Or PC relative 16-bit (MOVW) Or disp ((S+A-BDATA)>>1) & 0x3ffff Or @gprel(sym + add), data4 LSB Or S390TlsLdm32 Or PC relative 32 bit Or %hiadj() of function GOT entry Or X0 pipe ha() 16-bit GOT offset Or X1 pipe last hword 0 Or RiscvRvcJump Or MetagRelative
    Sparc6OrMipsTlsDtprelLo16OrPpc64Plt64OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrIa64Gprel32LsbOrS390TlsLdm32OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrTilegxImm16X1Hw0LastOrRiscvRvcJumpOrMetagRelative =
        0x2d,
    /// PC relative 64 bit Or 16 bit GOT offset for IE Or AlphaNum Or doubleword64 L + A - P Or PC relative (MOVT) Or disp ((S+A-BDATA)>>2) & 0x3ffff Or @gprel(sym + add), data8 MSB Or S390TlsLdm64 Or X1 pipe ha() 16-bit GOT offset Or X0 pipe last hword 1 Or RiscvRvcLui Or MetagGlobDat
    SparcDisp64OrMipsTlsGottprelOrAlphaNumOrPpc64Pltrel64OrArmMovtPrelOrCkcoreDoffsetImm18By4OrIa64Gprel64MsbOrS390TlsLdm64OrTileproImm16X1GotHaOrTilegxImm16X0Hw1LastOrRiscvRvcLuiOrMetagGlobDat =
        0x2e,
    /// Direct 64 bit ref to PLT entry Or TP-relative offset, 32 bit Or half16* S + A - .TOC Or Direct 16 bit (Thumb32 MOVW) Or @gprel(sym + add), data8 LSB Or S390TlsIe32 Or X0 pipe mm "start" Or X1 pipe last hword 1 Or RiscvGprelI Or MetagTlsGd
    SparcPlt64OrMipsTlsTprel32OrPpc64Toc16OrArmThmMovwAbsNcOrIa64Gprel64LsbOrS390TlsIe32OrTileproMmstartX0OrTilegxImm16X1Hw1LastOrRiscvGprelIOrMetagTlsGd =
        0x2f,
    /// High 22 bit complemented Or TP-relative offset, 64 bit Or No relocation, set segment base Or half16 #lo(S + A - .TOC.) Or ArmThmMovtAbs Or disp (G >> 2) Or S390TlsIe64 Or 24 bit GOT entry Or X0 pipe mm "end" Or X0 pipe last hword 2 Or RiscvGprelS Or MetagTlsLdm
    SparcHix22OrMipsTlsTprel64OrPariscSegbaseOrPpc64Toc16LoOrArmThmMovtAbsOrCkcoreGotImm18By4OrS390TlsIe64OrM32RGot24OrTileproMmendX0OrTilegxImm16X0Hw2LastOrRiscvGprelSOrMetagTlsLdm =
        0x30,
    /// Truncated 11 bit complemented Or TP-relative offset, high 16 bits Or 32 bits segment rel. address Or half16 #hi(S + A - .TOC.) Or ArmThmMovwPrelNc Or disp (G >> 2) Or S390TlsIeent Or 26 bit PC relative to PLT shifted Or X1 pipe mm "start" Or X1 pipe last hword 2 Or RiscvTprelI Or MetagTlsLdoHi16
    SparcLox10OrMipsTlsTprelHi16OrPariscSegrel32OrPpc64Toc16HiOrArmThmMovwPrelNcOrCkcorePltImm18By4OrS390TlsIeentOrM32R26PltrelOrTileproMmstartX1OrTilegxImm16X1Hw2LastOrRiscvTprelIOrMetagTlsLdoHi16 =
        0x31,
    /// Direct high 12 of 44 bit Or TP-relative offset, low 16 bits Or PLT rel. address, left 21 bits Or half16 #ha(S + A - .TOC.) Or ArmThmMovtPrel Or disp ((S+A-P) >>2) & 0x7f Or @ltoff(sym + add), add imm22 Or S390TlsLe32 Or Copy symbol at runtime Or X1 pipe mm "end" Or X0 pipe PC relative hword 0 Or RiscvTprelS Or MetagTlsLdoLo16 Or ArcPc32
    SparcH44OrMipsTlsTprelLo16OrPariscPltoff21LOrPpc64Toc16HaOrArmThmMovtPrelOrCkcorePcrelImm7By4OrIa64Ltoff22OrS390TlsLe32OrM32RCopyOrTileproMmendX1OrTilegxImm16X0Hw0PcrelOrRiscvTprelSOrMetagTlsLdoLo16OrArcPc32 =
        0x32,
    /// Direct mid 22 of 44 bit Or MipsGlobDat Or doubleword64 .TOC Or ArmThmJump19 Or 32 bit offset to TLS block Or @ltoff(sym + add), mov imm64 Or S390TlsLe64 Or Create GOT entry Or X0 pipe shift amount Or X1 pipe PC relative hword 0 Or RiscvRelax Or MetagTlsLdo Or ArcGotpc32
    SparcM44OrMipsGlobDatOrPpc64TocOrArmThmJump19OrCkcoreTlsLe32OrIa64Ltoff64IOrS390TlsLe64OrM32RGlobDatOrTileproShamtX0OrTilegxImm16X1Hw0PcrelOrRiscvRelaxOrMetagTlsLdoOrArcGotpc32 =
        0x33,
    /// Direct low 10 of 44 bit Or half16* M + A Or ArmThmJump6 Or CkcoreTlsIe32 Or S390TlsLdo32 Or Create PLT entry Or X1 pipe shift amount Or X0 pipe PC relative hword 1 Or RiscvSub6 Or MetagTlsIe Or ArcPlt32
    SparcL44OrPpc64Pltgot16OrArmThmJump6OrCkcoreTlsIe32OrS390TlsLdo32OrM32RJmpSlotOrTileproShamtX1OrTilegxImm16X0Hw1PcrelOrRiscvSub6OrMetagTlsIeOrArcPlt32 =
        0x34,
    /// Global register usage Or half16 #lo(M + A) Or ArmThmAluPrel110 Or CkcoreTlsGd32 Or S390TlsLdo64 Or Adjust by program base Or Y0 pipe shift amount Or X1 pipe PC relative hword 1 Or RiscvSet6 Or MetagTlsIenonpic Or ArcCopy
    SparcRegisterOrPpc64Pltgot16LoOrArmThmAluPrel110OrCkcoreTlsGd32OrS390TlsLdo64OrM32RRelativeOrTileproShamtY0OrTilegxImm16X1Hw1PcrelOrRiscvSet6OrMetagTlsIenonpicOrArcCopy =
        0x35,
    /// Direct 64 bit unaligned Or PLT rel. address, right 14 bits Or half16 #hi(M + A) Or ArmThmPc12 Or CkcoreTlsLdm32 Or ID of module containing symbol Or 24 bit offset to GOT Or Y1 pipe shift amount Or X0 pipe PC relative hword 2 Or RiscvSet8 Or MetagTlsIenonpicHi16 Or ArcGlobDat
    SparcUa64OrPariscPltoff14ROrPpc64Pltgot16HiOrArmThmPc12OrCkcoreTlsLdm32OrS390TlsDtpmodOrM32RGotoffOrTileproShamtY1OrTilegxImm16X0Hw2PcrelOrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat =
        0x36,
    /// Direct 16 bit unaligned Or half16 #ha(M + A) Or Direct 32-bit Or CkcoreTlsLdo32 Or Offset in TLS block Or 24 bit PC relative offset to GOT Or X1 pipe destination 8-bit Or X1 pipe PC relative hword 2 Or RiscvSet16 Or MetagTlsIenonpicLo16 Or ArcJumpSlot
    SparcUa16OrPpc64Pltgot16HaOrArmAbs32NoiOrCkcoreTlsLdo32OrS390TlsDtpoffOrM32RGotpc24OrTileproDestImm8X1OrTilegxImm16X1Hw2PcrelOrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot =
        0x37,
    /// SparcTlsGdHi22 Or half16ds* (S + A) >> 2 Or PC relative 32-bit Or CkcoreTlsDtpmod32 Or S390TlsTpoff Or M32RGot16HiUlo Or X0 pipe PC relative hword 3 Or RiscvSet32 Or MetagTlsTpoff Or ArcRelative
    SparcTlsGdHi22OrPpc64Addr16DsOrArmRel32NoiOrCkcoreTlsDtpmod32OrS390TlsTpoffOrM32RGot16HiUloOrTilegxImm16X0Hw3PcrelOrRiscvSet32OrMetagTlsTpoffOrArcRelative =
        0x38,
    /// SparcTlsGdLo10 Or 32 bits LT-rel. function pointer Or half16ds  #lo(S + A) >> 2 Or PC relative (ADD, SUB) Or CkcoreTlsDtpoff32 Or Direct 20 bit Or M32RGot16HiSlo Or X1 pipe PC relative hword 3 Or Riscv32Pcrel Or MetagTlsDtpmod Or ArcGotoff
    SparcTlsGdLo10OrPariscLtoffFptr32OrPpc64Addr16LoDsOrArmAluPcG0NcOrCkcoreTlsDtpoff32OrS39020OrM32RGot16HiSloOrTilegxImm16X1Hw3PcrelOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff =
        0x39,
    /// SparcTlsGdAdd Or LT-rel. fct ptr, left 21 bits Or half16ds* (G + A) >> 2 Or PC relative (ADD, SUB) Or CkcoreTlsTpoff32 Or @pltoff(sym + add), add imm22 Or 20 bit GOT offset Or Low 16 bit GOT entry Or X0 pipe PC-rel last hword 0 Or RiscvIrelative Or MetagTlsDtpoff Or ArcGotpc
    SparcTlsGdAddOrPariscLtoffFptr21LOrPpc64Got16DsOrArmAluPcG0OrCkcoreTlsTpoff32OrIa64Pltoff22OrS390Got20OrM32RGot16LoOrTilegxImm16X0Hw0LastPcrelOrRiscvIrelativeOrMetagTlsDtpoffOrArcGotpc =
        0x3a,
    /// SparcTlsGdCall Or half16ds  #lo(G + A) >> 2 Or PC relative (ADD, SUB) Or @pltoff(sym + add), mov imm64 Or 20 bit offset to jump slot Or M32RGotpcHiUlo Or X1 pipe PC-rel last hword 0 Or RiscvNum Or MetagTlsLe Or ArcGot32
    SparcTlsGdCallOrPpc64Got16LoDsOrArmAluPcG1NcOrIa64Pltoff64IOrS390Gotplt20OrM32RGotpcHiUloOrTilegxImm16X1Hw0LastPcrelOrRiscvNumOrMetagTlsLeOrArcGot32 =
        0x3b,
    /// SparcTlsLdmHi22 Or half16ds  #lo(L + A) >> 2 Or PC relative (ADD, SUB) Or S390TlsGotie20 Or M32RGotpcHiSlo Or "jal" for TLS GD Or X0 pipe PC-rel last hword 1 Or MetagTlsLeHi16
    SparcTlsLdmHi22OrPpc64Plt16LoDsOrArmAluPcG1OrS390TlsGotie20OrM32RGotpcHiSloOrTileproTlsGdCallOrTilegxImm16X0Hw1LastPcrelOrMetagTlsLeHi16 =
        0x3c,
    /// SparcTlsLdmLo10 Or half16ds* (R + A) >> 2 Or PC relative (ADD, SUB) Or STT_GNU_IFUNC relocation Or M32RGotpcLo Or X0 pipe "addi" for TLS GD Or X1 pipe PC-rel last hword 1 Or MetagTlsLeLo16
    SparcTlsLdmLo10OrPpc64SectoffDsOrArmAluPcG2OrS390IrelativeOrM32RGotpcLoOrTileproImm8X0TlsGdAddOrTilegxImm16X1Hw1LastPcrelOrMetagTlsLeLo16 =
        0x3d,
    /// SparcTlsLdmAdd Or LT-rel. fct ptr, right 14 bits Or half16ds  #lo(R + A) >> 2 Or PC relative (LDR,STR,LDRB,STRB) Or @pltoff(sym + add), data8 MSB Or S390Num Or M32RGotoffHiUlo Or X1 pipe "addi" for TLS GD Or X0 pipe PC-rel last hword 2
    SparcTlsLdmAddOrPariscLtoffFptr14ROrPpc64SectoffLoDsOrArmLdrPcG1OrIa64Pltoff64MsbOrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrTilegxImm16X0Hw2LastPcrel =
        0x3e,
    /// SparcTlsLdmCall Or half16ds* (S + A - .TOC.) >> 2 Or PC relative (LDR,STR,LDRB,STRB) Or @pltoff(sym + add), data8 LSB Or M32RGotoffHiSlo Or Y0 pipe "addi" for TLS GD Or X1 pipe PC-rel last hword 2
    SparcTlsLdmCallOrPpc64Toc16DsOrArmLdrPcG2OrIa64Pltoff64LsbOrM32RGotoffHiSloOrTileproImm8Y0TlsGdAddOrTilegxImm16X1Hw2LastPcrel =
        0x3f,
    /// SparcTlsLdoHix22 Or 64 bits function address Or half16ds  #lo(S + A - .TOC.) >> 2 Or ArmLdrsPcG0 Or Low 16 bit offset to GOT Or Y1 pipe "addi" for TLS GD Or X0 pipe hword 0 GOT offset
    SparcTlsLdoHix22OrPariscFptr64OrPpc64Toc16LoDsOrArmLdrsPcG0OrM32RGotoffLoOrTileproImm8Y1TlsGdAddOrTilegxImm16X0Hw0Got =
        0x40,
    /// SparcTlsLdoLox10 Or 32 bits function address Or half16ds* (M + A) >> 2 Or ArmLdrsPcG1 Or "lw_tls" for TLS IE Or X1 pipe hword 0 GOT offset
    SparcTlsLdoLox10OrPariscPlabel32OrPpc64Pltgot16DsOrArmLdrsPcG1OrTileproTlsIeLoadOrTilegxImm16X1Hw0Got =
        0x41,
    /// SparcTlsLdoAdd Or Left 21 bits of fdesc address Or half16ds  #lo(M + A) >> 2 Or ArmLdrsPcG2 Or X0 pipe 16-bit TLS GD offset Or X0 pipe PC-rel PLT hword 0 Or ArcTlsDtpmod
    SparcTlsLdoAddOrPariscPlabel21LOrPpc64Pltgot16LoDsOrArmLdrsPcG2OrTileproImm16X0TlsGdOrTilegxImm16X0Hw0PltPcrelOrArcTlsDtpmod =
        0x42,
    /// SparcTlsIeHi22 Or none	(sym+add)@tls Or none	(sym+add)@tls Or PC relative (LDC, STC) Or @fptr(sym + add), mov imm64 Or X1 pipe 16-bit TLS GD offset Or X1 pipe PC-rel PLT hword 0 Or ArcTlsDtpoff
    SparcTlsIeHi22OrPpcTlsOrPpc64TlsOrArmLdcPcG0OrIa64Fptr64IOrTileproImm16X1TlsGdOrTilegxImm16X1Hw0PltPcrelOrArcTlsDtpoff =
        0x43,
    /// SparcTlsIeLo10 Or word32	(sym+add)@dtpmod Or doubleword64 (sym+add)@dtpmod Or PC relative (LDC, STC) Or @fptr(sym + add), data4 MSB Or X0 pipe low 16-bit TLS GD offset Or X0 pipe PC-rel PLT hword 1 Or ArcTlsTpoff
    SparcTlsIeLo10OrPpcDtpmod32OrPpc64Dtpmod64OrArmLdcPcG1OrIa64Fptr32MsbOrTileproImm16X0TlsGdLoOrTilegxImm16X0Hw1PltPcrelOrArcTlsTpoff =
        0x44,
    /// SparcTlsIeLd Or half16*	(sym+add)@tprel Or half16*	(sym+add)@tprel Or PC relative (LDC, STC) Or @fptr(sym + add), data4 LSB Or X1 pipe low 16-bit TLS GD offset Or X1 pipe PC-rel PLT hword 1 Or ArcTlsGdGot
    SparcTlsIeLdOrPpcTprel16OrPpc64Tprel16OrArmLdcPcG2OrIa64Fptr32LsbOrTileproImm16X1TlsGdLoOrTilegxImm16X1Hw1PltPcrelOrArcTlsGdGot =
        0x45,
    /// SparcTlsIeLdx Or Right 14 bits of fdesc address Or half16	(sym+add)@tprel@l Or half16	(sym+add)@tprel@l Or Program base relative (ADD,SUB) Or @fptr(sym + add), data8 MSB Or X0 pipe high 16-bit TLS GD offset Or X0 pipe PC-rel PLT hword 2 Or ArcTlsGdLd
    SparcTlsIeLdxOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrArmAluSbG0NcOrIa64Fptr64MsbOrTileproImm16X0TlsGdHiOrTilegxImm16X0Hw2PltPcrelOrArcTlsGdLd =
        0x46,
    /// SparcTlsIeAdd Or half16	(sym+add)@tprel@h Or half16	(sym+add)@tprel@h Or Program base relative (ADD,SUB) Or @fptr(sym + add), data8 LSB Or X1 pipe high 16-bit TLS GD offset Or X1 pipe PC-rel PLT hword 2 Or ArcTlsGdCall
    SparcTlsIeAddOrPpcTprel16HiOrPpc64Tprel16HiOrArmAluSbG0OrIa64Fptr64LsbOrTileproImm16X1TlsGdHiOrTilegxImm16X1Hw2PltPcrelOrArcTlsGdCall =
        0x47,
    /// SparcTlsLeHix22 Or 64 bits PC-rel. address Or half16	(sym+add)@tprel@ha Or half16	(sym+add)@tprel@ha Or Program base relative (ADD,SUB) Or @pcrel(sym + add), brl Or X0 pipe ha() 16-bit TLS GD offset Or X0 pipe last hword 0 GOT offset Or ArcTlsIeGot
    SparcTlsLeHix22OrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrArmAluSbG1NcOrIa64Pcrel60BOrTileproImm16X0TlsGdHaOrTilegxImm16X0Hw0LastGotOrArcTlsIeGot =
        0x48,
    /// SparcTlsLeLox10 Or word32	(sym+add)@tprel Or doubleword64 (sym+add)@tprel Or Program base relative (ADD,SUB) Or @pcrel(sym + add), ptb, call Or X1 pipe ha() 16-bit TLS GD offset Or X1 pipe last hword 0 GOT offset
    SparcTlsLeLox10OrPpcTprel32OrPpc64Tprel64OrArmAluSbG1OrIa64Pcrel21BOrTileproImm16X1TlsGdHaOrTilegxImm16X1Hw0LastGot =
        0x49,
    /// SparcTlsDtpmod32 Or 22 bits PC-rel. address Or half16*	(sym+add)@dtprel Or half16*	(sym+add)@dtprel Or Program base relative (ADD,SUB) Or @pcrel(sym + add), chk.s Or X0 pipe 16-bit TLS IE offset Or X0 pipe last hword 1 GOT offset Or ArcTlsDtpoffS9 Or ArcTlsLeS9
    SparcTlsDtpmod32OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrArmAluSbG2OrIa64Pcrel21MOrTileproImm16X0TlsIeOrTilegxImm16X0Hw1LastGotOrArcTlsDtpoffS9OrArcTlsLeS9 =
        0x4a,
    /// SparcTlsDtpmod64 Or PC-rel. address, right 14 bits Or half16	(sym+add)@dtprel@l Or half16	(sym+add)@dtprel@l Or ArmLdrSbG0 Or @pcrel(sym + add), fchkf Or X1 pipe 16-bit TLS IE offset Or X1 pipe last hword 1 GOT offset Or ArcTlsLe32
    SparcTlsDtpmod64OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrArmLdrSbG0OrIa64Pcrel21FOrTileproImm16X1TlsIeOrTilegxImm16X1Hw1LastGotOrArcTlsLe32 =
        0x4b,
    /// SparcTlsDtpoff32 Or PC rel. address, right 14 bits Or half16	(sym+add)@dtprel@h Or half16	(sym+add)@dtprel@h Or ArmLdrSbG1 Or @pcrel(sym + add), data4 MSB Or X0 pipe low 16-bit TLS IE offset Or X0 pipe PC-rel PLT hword 3
    SparcTlsDtpoff32OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrArmLdrSbG1OrIa64Pcrel32MsbOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrel =
        0x4c,
    /// SparcTlsDtpoff64 Or 16 bits PC-rel. address Or half16	(sym+add)@dtprel@ha Or half16	(sym+add)@dtprel@ha Or ArmLdrSbG2 Or @pcrel(sym + add), data4 LSB Or X1 pipe low 16-bit TLS IE offset Or X1 pipe PC-rel PLT hword 3
    SparcTlsDtpoff64OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrArmLdrSbG2OrIa64Pcrel32LsbOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrel =
        0x4d,
    /// SparcTlsTpoff32 Or 16 bits PC-rel. address Or word32	(sym+add)@dtprel Or doubleword64 (sym+add)@dtprel Or ArmLdrsSbG0 Or @pcrel(sym + add), data8 MSB Or X0 pipe high 16-bit TLS IE offset Or X0 pipe hword 0 TLS GD offset
    SparcTlsTpoff32OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrArmLdrsSbG0OrIa64Pcrel64MsbOrTileproImm16X0TlsIeHiOrTilegxImm16X0Hw0TlsGd =
        0x4e,
    /// SparcTlsTpoff64 Or 16 bits PC-rel. address Or half16*	(sym+add)@got@tlsgd Or half16*	(sym+add)@got@tlsgd Or ArmLdrsSbG1 Or @pcrel(sym + add), data8 LSB Or X1 pipe high 16-bit TLS IE offset Or X1 pipe hword 0 TLS GD offset
    SparcTlsTpoff64OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrArmLdrsSbG1OrIa64Pcrel64LsbOrTileproImm16X1TlsIeHiOrTilegxImm16X1Hw0TlsGd =
        0x4f,
    /// SparcGotdataHix22 Or 64 bits of eff. address Or half16	(sym+add)@got@tlsgd@l Or half16	(sym+add)@got@tlsgd@l Or ArmLdrsSbG2 Or X0 pipe ha() 16-bit TLS IE offset Or X0 pipe hword 0 TLS LE offset
    SparcGotdataHix22OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrArmLdrsSbG2OrTileproImm16X0TlsIeHaOrTilegxImm16X0Hw0TlsLe =
        0x50,
    /// SparcGotdataLox10 Or half16	(sym+add)@got@tlsgd@h Or half16	(sym+add)@got@tlsgd@h Or Program base relative (LDC,STC) Or X1 pipe ha() 16-bit TLS IE offset Or X1 pipe hword 0 TLS LE offset
    SparcGotdataLox10OrPpcGotTlsgd16HiOrPpc64GotTlsgd16HiOrArmLdcSbG0OrTileproImm16X1TlsIeHaOrTilegxImm16X1Hw0TlsLe =
        0x51,
    /// SparcGotdataOpHix22 Or half16	(sym+add)@got@tlsgd@ha Or half16	(sym+add)@got@tlsgd@ha Or Program base relative (LDC,STC) Or @ltoff(@fptr(s+a)), imm22 Or ID of module containing symbol Or X0 pipe last hword 0 LE off
    SparcGotdataOpHix22OrPpcGotTlsgd16HaOrPpc64GotTlsgd16HaOrArmLdcSbG1OrIa64LtoffFptr22OrTileproTlsDtpmod32OrTilegxImm16X0Hw0LastTlsLe =
        0x52,
    /// SparcGotdataOpLox10 Or 14 bits of eff. address Or half16*	(sym+add)@got@tlsld Or half16*	(sym+add)@got@tlsld Or Program base relative (LDC,STC) Or @ltoff(@fptr(s+a)), imm64 Or Offset in TLS block Or X1 pipe last hword 0 LE off
    SparcGotdataOpLox10OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrArmLdcSbG2OrIa64LtoffFptr64IOrTileproTlsDtpoff32OrTilegxImm16X1Hw0LastTlsLe =
        0x53,
    /// SparcGotdataOp Or 14 bits of eff. address Or half16	(sym+add)@got@tlsld@l Or half16	(sym+add)@got@tlsld@l Or ArmMovwBrelNc Or @ltoff(@fptr(s+a)), data4 MSB Or Offset in static TLS block Or X0 pipe last hword 1 LE off
    SparcGotdataOpOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrArmMovwBrelNcOrIa64LtoffFptr32MsbOrTileproTlsTpoff32OrTilegxImm16X0Hw1LastTlsLe =
        0x54,
    /// SparcH34 Or 16 bits of eff. address Or half16	(sym+add)@got@tlsld@h Or half16	(sym+add)@got@tlsld@h Or ArmMovtBrel Or @ltoff(@fptr(s+a)), data4 LSB Or X0 pipe 16-bit TLS LE offset Or X1 pipe last hword 1 LE off
    SparcH34OrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrArmMovtBrelOrIa64LtoffFptr32LsbOrTileproImm16X0TlsLeOrTilegxImm16X1Hw1LastTlsLe =
        0x55,
    /// SparcSize32 Or 16 bits of eff. address Or half16	(sym+add)@got@tlsld@ha Or half16	(sym+add)@got@tlsld@ha Or ArmMovwBrel Or @ltoff(@fptr(s+a)), data8 MSB Or X1 pipe 16-bit TLS LE offset Or X0 pipe last hword 0 GD off
    SparcSize32OrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrArmMovwBrelOrIa64LtoffFptr64MsbOrTileproImm16X1TlsLeOrTilegxImm16X0Hw0LastTlsGd =
        0x56,
    /// SparcSize64 Or 16 bits of eff. address Or half16*	(sym+add)@got@tprel Or half16ds*	(sym+add)@got@tprel Or ArmThmMovwBrelNc Or @ltoff(@fptr(s+a)), data8 LSB Or X0 pipe low 16-bit TLS LE offset Or X1 pipe last hword 0 GD off
    SparcSize64OrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrArmThmMovwBrelNcOrIa64LtoffFptr64LsbOrTileproImm16X0TlsLeLoOrTilegxImm16X1Hw0LastTlsGd =
        0x57,
    /// SparcWdisp10 Or 64 bits of GP-rel. address Or half16	(sym+add)@got@tprel@l Or half16ds (sym+add)@got@tprel@l Or ArmThmMovtBrel Or X1 pipe low 16-bit TLS LE offset Or X0 pipe last hword 1 GD off
    SparcWdisp10OrPariscGprel64OrPpcGotTprel16LoOrPpc64GotTprel16LoDsOrArmThmMovtBrelOrTileproImm16X1TlsLeLoOrTilegxImm16X0Hw1LastTlsGd =
        0x58,
    SparcJmpIrelOrPpcIrelativeOrPpc64Irelative = 0xf8,
    /// SparcIrelative Or half16   (sym+add-.) Or half16   (sym+add-.) Or ArmRxpc25
    SparcIrelativeOrPpcRel16OrPpc64Rel16OrArmRxpc25 = 0xf9,
    /// SparcGnuVtinherit Or half16   (sym+add-.)@l Or half16   (sym+add-.)@l Or ArmRsbrel32
    SparcGnuVtinheritOrPpcRel16LoOrPpc64Rel16LoOrArmRsbrel32 = 0xfa,
    /// SparcGnuVtentry Or half16   (sym+add-.)@h Or half16   (sym+add-.)@h Or ArmThmRpc22
    SparcGnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22 = 0xfb,
    /// SparcRev32 Or half16   (sym+add-.)@ha Or half16   (sym+add-.)@ha Or ArmRrel32
    SparcRev32OrPpcRel16HaOrPpc64Rel16HaOrArmRrel32 = 0xfc,
    SparcNumOrArmRabs22 = 0xfd,
    /// MipsCopy Or 16 bits LT-rel. function ptr
    MipsCopyOrPariscLtoffFptr16Wf = 0x7e,
    /// MipsJumpSlot Or 16 bits LT-rel. function ptr
    MipsJumpSlotOrPariscLtoffFptr16Df = 0x7f,
    /// MipsNum Or PariscLoreserve Or Copy relocation Or Obsolete Or dynamic reloc, imported PLT, MSB Or GNU C++ vtable hierarchy Or GNU C++ vtable hierarchy
    MipsNumOrPariscLoreserveOrPariscCopyOrArmMeTooOrIa64IpltmsbOrTileproGnuVtinheritOrTilegxGnuVtinherit =
        0x80,
    /// GP-rel. address, right 14 bits Or half16*	(sym+add)@got@dtprel Or half16ds*	(sym+add)@got@dtprel Or ArmTlsCall Or X0 pipe ha() 16-bit TLS LE offset
    PariscGprel14WrOrPpcGotDtprel16OrPpc64GotDtprel16DsOrArmTlsCallOrTileproImm16X0TlsLeHa = 0x5b,
    /// GP-rel. address, right 14 bits Or half16*	(sym+add)@got@dtprel@l Or half16ds (sym+add)@got@dtprel@l Or TLS relaxation Or @segrel(sym + add), data4 MSB Or X1 pipe ha() 16-bit TLS LE offset Or X0 pipe hword 0 TLS IE offset
    PariscGprel14DrOrPpcGotDtprel16LoOrPpc64GotDtprel16LoDsOrArmTlsDescseqOrIa64Segrel32MsbOrTileproImm16X1TlsLeHaOrTilegxImm16X0Hw0TlsIe =
        0x5c,
    /// 16 bits GP-rel. address Or half16*	(sym+add)@got@dtprel@h Or half16	(sym+add)@got@dtprel@h Or ArmThmTlsCall Or @segrel(sym + add), data4 LSB Or X1 pipe hword 0 TLS IE offset
    PariscGprel16FOrPpcGotDtprel16HiOrPpc64GotDtprel16HiOrArmThmTlsCallOrIa64Segrel32LsbOrTilegxImm16X1Hw0TlsIe =
        0x5d,
    /// 16 bits GP-rel. address Or half16*	(sym+add)@got@dtprel@ha Or half16	(sym+add)@got@dtprel@ha Or ArmPlt32Abs Or @segrel(sym + add), data8 MSB Or X0 pipe PC-rel PLT last hword 0
    PariscGprel16WfOrPpcGotDtprel16HaOrPpc64GotDtprel16HaOrArmPlt32AbsOrIa64Segrel64MsbOrTilegxImm16X0Hw0LastPltPcrel =
        0x5e,
    /// 16 bits GP-rel. address Or none	(sym+add)@tlsgd Or half16ds*	(sym+add)@tprel Or GOT entry Or @segrel(sym + add), data8 LSB Or X1 pipe PC-rel PLT last hword 0
    PariscGprel16DfOrPpcTlsgdOrPpc64Tprel16DsOrArmGotAbsOrIa64Segrel64LsbOrTilegxImm16X1Hw0LastPltPcrel =
        0x5f,
    /// 64 bits LT-rel. address Or none	(sym+add)@tlsld Or half16ds	(sym+add)@tprel@l Or PC relative GOT entry Or X0 pipe PC-rel PLT last hword 1
    PariscLtoff64OrPpcTlsldOrPpc64Tprel16LoDsOrArmGotPrelOrTilegxImm16X0Hw1LastPltPcrel = 0x60,
    /// LT-rel. address, right 14 bits Or half16	(sym+add)@tprel@highest Or ArmGotrelax Or X1 pipe PC-rel PLT last hword 2
    PariscLtoff14WrOrPpc64Tprel16HighestOrArmGotrelaxOrTilegxImm16X1Hw2LastPltPcrel = 0x63,
    /// LT-rel. address, right 14 bits Or half16	(sym+add)@tprel@highesta Or ArmGnuVtentry Or @secrel(sym + add), data4 MSB Or X0 pipe last hword 0 IE off
    PariscLtoff14DrOrPpc64Tprel16HighestaOrArmGnuVtentryOrIa64Secrel32MsbOrTilegxImm16X0Hw0LastTlsIe =
        0x64,
    /// 16 bits LT-rel. address Or PpcEmbNaddr32 Or half16ds* (sym+add)@dtprel Or ArmGnuVtinherit Or @secrel(sym + add), data4 LSB Or X1 pipe last hword 0 IE off
    PariscLtoff16FOrPpcEmbNaddr32OrPpc64Dtprel16DsOrArmGnuVtinheritOrIa64Secrel32LsbOrTilegxImm16X1Hw0LastTlsIe =
        0x65,
    /// 16 bits LT-rel. address Or PpcEmbNaddr16 Or half16ds	(sym+add)@dtprel@l Or PC relative & 0xFFE (Thumb16 B) Or @secrel(sym + add), data8 MSB Or X0 pipe last hword 1 IE off Or Nds32TlsTpoff
    PariscLtoff16WfOrPpcEmbNaddr16OrPpc64Dtprel16LoDsOrArmThmPc11OrIa64Secrel64MsbOrTilegxImm16X0Hw1LastTlsIeOrNds32TlsTpoff =
        0x66,
    /// 16 bits LT-rel. address Or PpcEmbNaddr16Lo Or half16	(sym+add)@dtprel@higher Or ArmThmPc9 Or @secrel(sym + add), data8 LSB Or X1 pipe last hword 1 IE off
    PariscLtoff16DfOrPpcEmbNaddr16LoOrPpc64Dtprel16HigherOrArmThmPc9OrIa64Secrel64LsbOrTilegxImm16X1Hw1LastTlsIe =
        0x67,
    /// 64 bits section rel. address Or PpcEmbNaddr16Hi Or half16	(sym+add)@dtprel@highera Or ArmTlsGd32
    PariscSecrel64OrPpcEmbNaddr16HiOrPpc64Dtprel16HigheraOrArmTlsGd32 = 0x68,
    /// 64 bits segment rel. address Or PpcEmbRelstLo Or Ppc64Tprel16High Or "jal" for TLS GD
    PariscSegrel64OrPpcEmbRelstLoOrPpc64Tprel16HighOrTilegxTlsGdCall = 0x70,
    /// PLT-rel. address, right 14 bits Or PpcEmbBitFld Or Ppc64Dtprel16Higha Or Y0 pipe "addi" for TLS GD
    PariscPltoff14WrOrPpcEmbBitFldOrPpc64Dtprel16HighaOrTilegxImm8Y0TlsGdAdd = 0x73,
    /// PLT-rel. address, right 14 bits Or 16 bit relative offset in SDA Or symbol + addend, data4 MSB Or Y1 pipe "addi" for TLS GD
    PariscPltoff14DrOrPpcEmbRelsdaOrIa64Ltv32MsbOrTilegxImm8Y1TlsGdAdd = 0x74,
    /// 16 bits LT-rel. address Or symbol + addend, data4 LSB Or "ld_tls" for TLS IE
    PariscPltoff16FOrIa64Ltv32LsbOrTilegxTlsIeLoad = 0x75,
    /// 16 bits PLT-rel. address Or symbol + addend, data8 MSB Or X0 pipe "addi" for TLS GD/IE
    PariscPltoff16WfOrIa64Ltv64MsbOrTilegxImm8X0TlsAdd = 0x76,
    /// 16 bits PLT-rel. address Or symbol + addend, data8 LSB Or X1 pipe "addi" for TLS GD/IE Or Nds32TlsDesc
    PariscPltoff16DfOrIa64Ltv64LsbOrTilegxImm8X1TlsAddOrNds32TlsDesc = 0x77,
    /// 64 bits LT-rel. function ptr Or Y0 pipe "addi" for TLS GD/IE
    PariscLtoffFptr64OrTilegxImm8Y0TlsAdd = 0x78,
    /// LT-rel. fct. ptr., right 14 bits Or @pcrel(sym + add), 64bit inst
    PariscLtoffFptr14WrOrIa64Pcrel64I = 0x7b,
    /// LT-rel. fct. ptr., right 14 bits.
    PariscLtoffFptr14Dr = 0x7c,
    /// 16 bits LT-rel. function ptr.
    PariscLtoffFptr16F = 0x7d,
    /// Dynamic reloc, imported PLT Or ArmThmTlsDescseq Or ArmThmTlsDescseq16 Or dynamic reloc, imported PLT, LSB Or GNU C++ vtable member usage Or GNU C++ vtable member usage
    PariscIpltOrArmThmTlsDescseqOrArmThmTlsDescseq16OrIa64IpltlsbOrTileproGnuVtentryOrTilegxGnuVtentry =
        0x81,
    /// Dynamic reloc, exported PLT Or ArmThmTlsDescseq32 Or TileproNum Or TilegxNum
    PariscEpltOrArmThmTlsDescseq32OrTileproNumOrTilegxNum = 0x82,
    /// 32 bits TP-rel. address.
    PariscTprel32 = 0x99,
    /// TP-rel. address, left 21 bits Or @ltoff(@tprel(s+a)), imm2
    PariscTprel21LOrIa64LtoffTprel22 = 0x9a,
    /// TP-rel. address, right 14 bits.
    PariscTprel14R = 0x9e,
    /// LT-TP-rel. address, left 21 bits Or ShCopy
    PariscLtoffTp21LOrShCopy = 0xa2,
    /// LT-TP-rel. address, right 14 bits Or @dtpmod(sym + add), data8 MSB Or ShGotoff
    PariscLtoffTp14ROrIa64Dtpmod64MsbOrShGotoff = 0xa6,
    /// 14 bits LT-TP-rel. address Or @dtpmod(sym + add), data8 LSB Or ShGotpc
    PariscLtoffTp14FOrIa64Dtpmod64LsbOrShGotpc = 0xa7,
    /// 64 bits TP-rel. address.
    PariscTprel64 = 0xd8,
    /// TP-rel. address, right 14 bits.
    PariscTprel14Wr = 0xdb,
    /// TP-rel. address, right 14 bits.
    PariscTprel14Dr = 0xdc,
    /// 16 bits TP-rel. address.
    PariscTprel16F = 0xdd,
    /// 16 bits TP-rel. address.
    PariscTprel16Wf = 0xde,
    /// 16 bits TP-rel. address.
    PariscTprel16Df = 0xdf,
    /// 64 bits LT-TP-rel. address.
    PariscLtoffTp64 = 0xe0,
    /// LT-TP-rel. address, right 14 bits.
    PariscLtoffTp14Wr = 0xe3,
    /// LT-TP-rel. address, right 14 bits.
    PariscLtoffTp14Dr = 0xe4,
    /// 16 bits LT-TP-rel. address.
    PariscLtoffTp16F = 0xe5,
    /// 16 bits LT-TP-rel. address.
    PariscLtoffTp16Wf = 0xe6,
    /// 16 bits LT-TP-rel. address.
    PariscLtoffTp16Df = 0xe7,
    PariscGnuVtentry = 0xe8,
    PariscGnuVtinherit = 0xe9,
    /// GD 21-bit left.
    PariscTlsGd21L = 0xea,
    /// GD 14-bit right.
    PariscTlsGd14R = 0xeb,
    /// GD call to __t_g_a.
    PariscTlsGdcall = 0xec,
    /// LD module 21-bit left.
    PariscTlsLdm21L = 0xed,
    /// LD module 14-bit right.
    PariscTlsLdm14R = 0xee,
    /// LD module call to __t_g_a.
    PariscTlsLdmcall = 0xef,
    /// LD offset 21-bit left.
    PariscTlsLdo21L = 0xf0,
    /// LD offset 14-bit right.
    PariscTlsLdo14R = 0xf1,
    /// DTP module 32-bit.
    PariscTlsDtpmod32 = 0xf2,
    /// DTP module 64-bit.
    PariscTlsDtpmod64 = 0xf3,
    /// DTP offset 32-bit.
    PariscTlsDtpoff32 = 0xf4,
    /// DTP offset 32-bit Or PariscTlsLe21L Or PariscTlsLe14R Or PariscTlsIe21L Or PariscTlsIe14R Or PariscTlsTprel32 Or PariscTlsTprel64
    PariscTlsDtpoff64OrPariscTlsLe21LOrPariscTlsLe14ROrPariscTlsIe21LOrPariscTlsIe14ROrPariscTlsTprel32OrPariscTlsTprel64 =
        0xf5,
    PariscHireserveOrPpcToc16OrArmRbase = 0xff,
    /// half16	(sym+add)@got@tprel@h Or half16	(sym+add)@got@tprel@h Or ArmThmMovwBrel Or X0 pipe high 16-bit TLS LE offset Or X1 pipe last hword 1 GD off
    PpcGotTprel16HiOrPpc64GotTprel16HiOrArmThmMovwBrelOrTileproImm16X0TlsLeHiOrTilegxImm16X1Hw1LastTlsGd =
        0x59,
    /// half16	(sym+add)@got@tprel@ha Or half16	(sym+add)@got@tprel@ha Or ArmTlsGotdesc Or X1 pipe high 16-bit TLS LE offset
    PpcGotTprel16HaOrPpc64GotTprel16HaOrArmTlsGotdescOrTileproImm16X1TlsLeHi = 0x5a,
    /// PpcEmbNaddr16Ha Or half16	(sym+add)@dtprel@highest Or ArmTlsLdm32
    PpcEmbNaddr16HaOrPpc64Dtprel16HighestOrArmTlsLdm32 = 0x69,
    /// PpcEmbSdai16 Or half16	(sym+add)@dtprel@highesta Or ArmTlsLdo32 Or 64-bit ID of symbol's module
    PpcEmbSdai16OrPpc64Dtprel16HighestaOrArmTlsLdo32OrTilegxTlsDtpmod64 = 0x6a,
    /// PpcEmbSda2I16 Or none	(sym+add)@tlsgd Or ArmTlsIe32 Or 64-bit offset in TLS block
    PpcEmbSda2I16OrPpc64TlsgdOrArmTlsIe32OrTilegxTlsDtpoff64 = 0x6b,
    /// PpcEmbSda2Rel Or none	(sym+add)@tlsld Or ArmTlsLe32 Or data 4 + REL Or 64-bit offset in static TLS block
    PpcEmbSda2RelOrPpc64TlsldOrArmTlsLe32OrIa64Rel32MsbOrTilegxTlsTpoff64 = 0x6c,
    /// 16 bit offset in SDA Or none Or ArmTlsLdo12 Or data 4 + REL Or 32-bit ID of symbol's module
    PpcEmbSda21OrPpc64TocsaveOrArmTlsLdo12OrIa64Rel32LsbOrTilegxTlsDtpmod32 = 0x6d,
    /// PpcEmbMrkref Or Ppc64Addr16High Or ArmTlsLe12 Or data 8 + REL Or 32-bit offset in TLS block
    PpcEmbMrkrefOrPpc64Addr16HighOrArmTlsLe12OrIa64Rel64MsbOrTilegxTlsDtpoff32 = 0x6e,
    /// PpcEmbRelsec16 Or Ppc64Addr16Higha Or ArmTlsIe12Gp Or data 8 + REL Or 32-bit offset in static TLS block
    PpcEmbRelsec16OrPpc64Addr16HighaOrArmTlsIe12GpOrIa64Rel64LsbOrTilegxTlsTpoff32 = 0x6f,
    /// PpcEmbRelstHi Or Ppc64Tprel16Higha Or X0 pipe "addi" for TLS GD
    PpcEmbRelstHiOrPpc64Tprel16HighaOrTilegxImm8X0TlsGdAdd = 0x71,
    /// PpcEmbRelstHa Or Ppc64Dtprel16High Or X1 pipe "addi" for TLS GD
    PpcEmbRelstHaOrPpc64Dtprel16HighOrTilegxImm8X1TlsGdAdd = 0x72,
    /// like EMB_SDA21, but lower 16 bit Or Copy symbol at runtime Or @dtprel(sym + add), data4 MSB
    PpcDiabSda21LoOrAArch64P32CopyOrIa64Dtprel32Msb = 0xb4,
    /// like EMB_SDA21, but high 16 bit Or Create GOT entry Or @dtprel(sym + add), data4 LSB
    PpcDiabSda21HiOrAArch64P32GlobDatOrIa64Dtprel32Lsb = 0xb5,
    /// like EMB_SDA21, adjusted high 16 Or Create PLT entry Or @dtprel(sym + add), data8 MSB
    PpcDiabSda21HaOrAArch64P32JumpSlotOrIa64Dtprel64Msb = 0xb6,
    /// like EMB_RELSDA, but lower 16 bit Or Adjust by program base Or @dtprel(sym + add), data8 LSB
    PpcDiabRelsdaLoOrAArch64P32RelativeOrIa64Dtprel64Lsb = 0xb7,
    /// like EMB_RELSDA, but high 16 bit Or Module number, 32 bit
    PpcDiabRelsdaHiOrAArch64P32TlsDtpmod = 0xb8,
    /// like EMB_RELSDA, adjusted high 16 Or Ppc64None Or 32bit absolute address Or 26bit address, word aligned Or 16bit absolute address Or lower 16bits of address Or high 16bits of address Or adjusted high 16bits Or 16bit address, word aligned Or Ppc64Addr14Brtaken Or Ppc64Addr14Brntaken Or PC-rel. 26 bit, word aligned Or PC relative 16 bit Or Ppc64Rel14Brtaken Or Ppc64Rel14Brntaken Or Ppc64Got16 Or Ppc64Got16Lo Or Ppc64Got16Hi Or Ppc64Got16Ha Or Ppc64Copy Or Ppc64GlobDat Or Ppc64JmpSlot Or Ppc64Relative Or Ppc64Uaddr32 Or Ppc64Uaddr16 Or Ppc64Rel32 Or Ppc64Plt32 Or Ppc64Pltrel32 Or Ppc64Plt16Lo Or Ppc64Plt16Hi Or Ppc64Plt16Ha Or Ppc64Sectoff Or Ppc64SectoffLo Or Ppc64SectoffHi Or Ppc64SectoffHa Or Module-relative offset, 32 bit
    PpcDiabRelsdaHaOrPpc64NoneOrPpc64Addr32OrPpc64Addr24OrPpc64Addr16OrPpc64Addr16LoOrPpc64Addr16HiOrPpc64Addr16HaOrPpc64Addr14OrPpc64Addr14BrtakenOrPpc64Addr14BrntakenOrPpc64Rel24OrPpc64Rel14OrPpc64Rel14BrtakenOrPpc64Rel14BrntakenOrPpc64Got16OrPpc64Got16LoOrPpc64Got16HiOrPpc64Got16HaOrPpc64CopyOrPpc64GlobDatOrPpc64JmpSlotOrPpc64RelativeOrPpc64Uaddr32OrPpc64Uaddr16OrPpc64Rel32OrPpc64Plt32OrPpc64Pltrel32OrPpc64Plt16LoOrPpc64Plt16HiOrPpc64Plt16HaOrPpc64SectoffOrPpc64SectoffLoOrPpc64SectoffHiOrPpc64SectoffHaOrAArch64P32TlsDtprel =
        0xb9,
    /// half16	(sym+add)@tprel@higher Or ArmGotBrel12 Or X1 pipe PC-rel PLT last hword 1
    Ppc64Tprel16HigherOrArmGotBrel12OrTilegxImm16X1Hw1LastPltPcrel = 0x61,
    /// half16	(sym+add)@tprel@highera Or ArmGotoff12 Or X0 pipe PC-rel PLT last hword 2
    Ppc64Tprel16HigheraOrArmGotoff12OrTilegxImm16X0Hw2LastPltPcrel = 0x62,
    Ppc64JmpIrel = 0xf7,
    /// TP-relative offset, 32 bit Or @ltoff(@dtprel(s+a)), imm22
    AArch64P32TlsTprelOrIa64LtoffDtprel22 = 0xba,
    /// TLS Descriptor.
    AArch64P32Tlsdesc = 0xbb,
    /// STT_GNU_IFUNC relocation.
    AArch64P32Irelative = 0xbc,
    /// Direct 64 bit.
    AArch64Abs64 = 0x101,
    /// Direct 32 bit.
    AArch64Abs32 = 0x102,
    /// Direct 16-bit.
    AArch64Abs16 = 0x103,
    /// PC-relative 64-bit.
    AArch64Prel64 = 0x104,
    /// PC-relative 32-bit.
    AArch64Prel32 = 0x105,
    /// PC-relative 16-bit.
    AArch64Prel16 = 0x106,
    /// Dir. MOVZ imm. from bits 15:0.
    AArch64MovwUabsG0 = 0x107,
    /// Likewise for MOVK; no check.
    AArch64MovwUabsG0Nc = 0x108,
    /// Dir. MOVZ imm. from bits 31:16.
    AArch64MovwUabsG1 = 0x109,
    /// Likewise for MOVK; no check.
    AArch64MovwUabsG1Nc = 0x10a,
    /// Dir. MOVZ imm. from bits 47:32.
    AArch64MovwUabsG2 = 0x10b,
    /// Likewise for MOVK; no check.
    AArch64MovwUabsG2Nc = 0x10c,
    /// Dir. MOV{K,Z} imm. from 63:48.
    AArch64MovwUabsG3 = 0x10d,
    /// Dir. MOV{N,Z} imm. from 15:0.
    AArch64MovwSabsG0 = 0x10e,
    /// Dir. MOV{N,Z} imm. from 31:16.
    AArch64MovwSabsG1 = 0x10f,
    /// Dir. MOV{N,Z} imm. from 47:32.
    AArch64MovwSabsG2 = 0x110,
    /// PC-rel. LD imm. from bits 20:2.
    AArch64LdPrelLo19 = 0x111,
    /// PC-rel. ADR imm. from bits 20:0.
    AArch64AdrPrelLo21 = 0x112,
    /// Page-rel. ADRP imm. from 32:12.
    AArch64AdrPrelPgHi21 = 0x113,
    /// Likewise; no overflow check.
    AArch64AdrPrelPgHi21Nc = 0x114,
    /// Dir. ADD imm. from bits 11:0.
    AArch64AddAbsLo12Nc = 0x115,
    /// Likewise for LD/ST; no check.
    AArch64Ldst8AbsLo12Nc = 0x116,
    /// PC-rel. TBZ/TBNZ imm. from 15:2.
    AArch64Tstbr14 = 0x117,
    /// PC-rel. cond. br. imm. from 20:2.
    AArch64Condbr19 = 0x118,
    /// PC-rel. B imm. from bits 27:2.
    AArch64Jump26 = 0x11a,
    /// Likewise for CALL.
    AArch64Call26 = 0x11b,
    /// Dir. ADD imm. from bits 11:1.
    AArch64Ldst16AbsLo12Nc = 0x11c,
    /// Likewise for bits 11:2.
    AArch64Ldst32AbsLo12Nc = 0x11d,
    /// Likewise for bits 11:3.
    AArch64Ldst64AbsLo12Nc = 0x11e,
    /// PC-rel. MOV{N,Z} imm. from 15:0.
    AArch64MovwPrelG0 = 0x11f,
    /// Likewise for MOVK; no check.
    AArch64MovwPrelG0Nc = 0x120,
    /// PC-rel. MOV{N,Z} imm. from 31:16.
    AArch64MovwPrelG1 = 0x121,
    /// Likewise for MOVK; no check.
    AArch64MovwPrelG1Nc = 0x122,
    /// PC-rel. MOV{N,Z} imm. from 47:32.
    AArch64MovwPrelG2 = 0x123,
    /// Likewise for MOVK; no check.
    AArch64MovwPrelG2Nc = 0x124,
    /// PC-rel. MOV{N,Z} imm. from 63:48.
    AArch64MovwPrelG3 = 0x125,
    /// Dir. ADD imm. from bits 11:4.
    AArch64Ldst128AbsLo12Nc = 0x12b,
    /// GOT-rel. off. MOV{N,Z} imm. 15:0.
    AArch64MovwGotoffG0 = 0x12c,
    /// Likewise for MOVK; no check.
    AArch64MovwGotoffG0Nc = 0x12d,
    /// GOT-rel. o. MOV{N,Z} imm. 31:16.
    AArch64MovwGotoffG1 = 0x12e,
    /// Likewise for MOVK; no check.
    AArch64MovwGotoffG1Nc = 0x12f,
    /// GOT-rel. o. MOV{N,Z} imm. 47:32.
    AArch64MovwGotoffG2 = 0x130,
    /// Likewise for MOVK; no check.
    AArch64MovwGotoffG2Nc = 0x131,
    /// GOT-rel. o. MOV{N,Z} imm. 63:48.
    AArch64MovwGotoffG3 = 0x132,
    /// GOT-relative 64-bit.
    AArch64Gotrel64 = 0x133,
    /// GOT-relative 32-bit.
    AArch64Gotrel32 = 0x134,
    /// PC-rel. GOT off. load imm. 20:2.
    AArch64GotLdPrel19 = 0x135,
    /// GOT-rel. off. LD/ST imm. 14:3.
    AArch64Ld64GotoffLo15 = 0x136,
    /// P-page-rel. GOT off. ADRP 32:12.
    AArch64AdrGotPage = 0x137,
    /// Dir. GOT off. LD/ST imm. 11:3.
    AArch64Ld64GotLo12Nc = 0x138,
    /// GOT-page-rel. GOT off. LD/ST 14:3
    AArch64Ld64GotpageLo15 = 0x139,
    /// PC-relative ADR imm. 20:0.
    AArch64TlsgdAdrPrel21 = 0x200,
    /// page-rel. ADRP imm. 32:12.
    AArch64TlsgdAdrPage21 = 0x201,
    /// direct ADD imm. from 11:0.
    AArch64TlsgdAddLo12Nc = 0x202,
    /// GOT-rel. MOV{N,Z} 31:16.
    AArch64TlsgdMovwG1 = 0x203,
    /// GOT-rel. MOVK imm. 15:0.
    AArch64TlsgdMovwG0Nc = 0x204,
    /// Like 512; local dynamic model.
    AArch64TlsldAdrPrel21 = 0x205,
    /// Like 513; local dynamic model.
    AArch64TlsldAdrPage21 = 0x206,
    /// Like 514; local dynamic model.
    AArch64TlsldAddLo12Nc = 0x207,
    /// Like 515; local dynamic model.
    AArch64TlsldMovwG1 = 0x208,
    /// Like 516; local dynamic model.
    AArch64TlsldMovwG0Nc = 0x209,
    /// TLS PC-rel. load imm. 20:2.
    AArch64TlsldLdPrel19 = 0x20a,
    /// TLS DTP-rel. MOV{N,Z} 47:32.
    AArch64TlsldMovwDtprelG2 = 0x20b,
    /// TLS DTP-rel. MOV{N,Z} 31:16.
    AArch64TlsldMovwDtprelG1 = 0x20c,
    /// Likewise; MOVK; no check.
    AArch64TlsldMovwDtprelG1Nc = 0x20d,
    /// TLS DTP-rel. MOV{N,Z} 15:0.
    AArch64TlsldMovwDtprelG0 = 0x20e,
    /// Likewise; MOVK; no check.
    AArch64TlsldMovwDtprelG0Nc = 0x20f,
    /// DTP-rel. ADD imm. from 23:12.
    AArch64TlsldAddDtprelHi12 = 0x210,
    /// DTP-rel. ADD imm. from 11:0.
    AArch64TlsldAddDtprelLo12 = 0x211,
    /// Likewise; no ovfl. check.
    AArch64TlsldAddDtprelLo12Nc = 0x212,
    /// DTP-rel. LD/ST imm. 11:0.
    AArch64TlsldLdst8DtprelLo12 = 0x213,
    /// Likewise; no check.
    AArch64TlsldLdst8DtprelLo12Nc = 0x214,
    /// DTP-rel. LD/ST imm. 11:1.
    AArch64TlsldLdst16DtprelLo12 = 0x215,
    /// Likewise; no check.
    AArch64TlsldLdst16DtprelLo12Nc = 0x216,
    /// DTP-rel. LD/ST imm. 11:2.
    AArch64TlsldLdst32DtprelLo12 = 0x217,
    /// Likewise; no check.
    AArch64TlsldLdst32DtprelLo12Nc = 0x218,
    /// DTP-rel. LD/ST imm. 11:3.
    AArch64TlsldLdst64DtprelLo12 = 0x219,
    /// Likewise; no check.
    AArch64TlsldLdst64DtprelLo12Nc = 0x21a,
    /// GOT-rel. MOV{N,Z} 31:16.
    AArch64TlsieMovwGottprelG1 = 0x21b,
    /// GOT-rel. MOVK 15:0.
    AArch64TlsieMovwGottprelG0Nc = 0x21c,
    /// Page-rel. ADRP 32:12.
    AArch64TlsieAdrGottprelPage21 = 0x21d,
    /// Direct LD off. 11:3.
    AArch64TlsieLd64GottprelLo12Nc = 0x21e,
    /// PC-rel. load imm. 20:2.
    AArch64TlsieLdGottprelPrel19 = 0x21f,
    /// TLS TP-rel. MOV{N,Z} 47:32.
    AArch64TlsleMovwTprelG2 = 0x220,
    /// TLS TP-rel. MOV{N,Z} 31:16.
    AArch64TlsleMovwTprelG1 = 0x221,
    /// Likewise; MOVK; no check.
    AArch64TlsleMovwTprelG1Nc = 0x222,
    /// TLS TP-rel. MOV{N,Z} 15:0.
    AArch64TlsleMovwTprelG0 = 0x223,
    /// Likewise; MOVK; no check.
    AArch64TlsleMovwTprelG0Nc = 0x224,
    /// TP-rel. ADD imm. 23:12.
    AArch64TlsleAddTprelHi12 = 0x225,
    /// TP-rel. ADD imm. 11:0.
    AArch64TlsleAddTprelLo12 = 0x226,
    /// Likewise; no ovfl. check.
    AArch64TlsleAddTprelLo12Nc = 0x227,
    /// TP-rel. LD/ST off. 11:0.
    AArch64TlsleLdst8TprelLo12 = 0x228,
    /// Likewise; no ovfl. check.
    AArch64TlsleLdst8TprelLo12Nc = 0x229,
    /// TP-rel. LD/ST off. 11:1.
    AArch64TlsleLdst16TprelLo12 = 0x22a,
    /// Likewise; no check.
    AArch64TlsleLdst16TprelLo12Nc = 0x22b,
    /// TP-rel. LD/ST off. 11:2.
    AArch64TlsleLdst32TprelLo12 = 0x22c,
    /// Likewise; no check.
    AArch64TlsleLdst32TprelLo12Nc = 0x22d,
    /// TP-rel. LD/ST off. 11:3.
    AArch64TlsleLdst64TprelLo12 = 0x22e,
    /// Likewise; no check.
    AArch64TlsleLdst64TprelLo12Nc = 0x22f,
    /// PC-rel. load immediate 20:2.
    AArch64TlsdescLdPrel19 = 0x230,
    /// PC-rel. ADR immediate 20:0.
    AArch64TlsdescAdrPrel21 = 0x231,
    /// Page-rel. ADRP imm. 32:12.
    AArch64TlsdescAdrPage21 = 0x232,
    /// Direct LD off. from 11:3.
    AArch64TlsdescLd64Lo12 = 0x233,
    /// Direct ADD imm. from 11:0.
    AArch64TlsdescAddLo12 = 0x234,
    /// GOT-rel. MOV{N,Z} imm. 31:16.
    AArch64TlsdescOffG1 = 0x235,
    /// GOT-rel. MOVK imm. 15:0; no ck.
    AArch64TlsdescOffG0Nc = 0x236,
    /// Relax LDR.
    AArch64TlsdescLdr = 0x237,
    /// Relax ADD.
    AArch64TlsdescAdd = 0x238,
    /// Relax BLR.
    AArch64TlsdescCall = 0x239,
    /// TP-rel. LD/ST off. 11:4.
    AArch64TlsleLdst128TprelLo12 = 0x23a,
    /// Likewise; no check.
    AArch64TlsleLdst128TprelLo12Nc = 0x23b,
    /// DTP-rel. LD/ST imm. 11:4.
    AArch64TlsldLdst128DtprelLo12 = 0x23c,
    /// Likewise; no check.
    AArch64TlsldLdst128DtprelLo12Nc = 0x23d,
    /// Copy symbol at runtime.
    AArch64Copy = 0x400,
    /// Create GOT entry.
    AArch64GlobDat = 0x401,
    /// Create PLT entry.
    AArch64JumpSlot = 0x402,
    /// Adjust by program base.
    AArch64Relative = 0x403,
    /// Module number, 64 bit.
    AArch64TlsDtpmod = 0x404,
    /// Module-relative offset, 64 bit.
    AArch64TlsDtprel = 0x405,
    /// TP-relative offset, 64 bit.
    AArch64TlsTprel = 0x406,
    /// TLS Descriptor.
    AArch64Tlsdesc = 0x407,
    /// STT_GNU_IFUNC relocation.
    AArch64Irelative = 0x408,
    ArmThmGotBrel12 = 0x83,
    ArmIrelativeOrShGot32 = 0xa0,
    ArmRpc24 = 0xfe,
    /// ArmNum Or ShNum Or Keep this the last entry
    ArmNumOrShNumOrM32RNum = 0x100,
    /// @pcrel(sym + add), 21bit inst Or Y1 pipe "addi" for TLS GD/IE
    Ia64Pcrel21BiOrTilegxImm8Y1TlsAdd = 0x79,
    /// @pcrel(sym + add), 22bit inst
    Ia64Pcrel22 = 0x7a,
    /// copy relocation
    Ia64Copy = 0x84,
    /// Addend and symbol difference
    Ia64Sub = 0x85,
    /// LTOFF22, relaxable.
    Ia64Ltoff22X = 0x86,
    /// Use of LTOFF22X.
    Ia64Ldxmov = 0x87,
    /// @tprel(sym + add), imm14 Or ShTlsLd32
    Ia64Tprel14OrShTlsLd32 = 0x91,
    /// @tprel(sym + add), imm22 Or ShTlsLdo32
    Ia64Tprel22OrShTlsLdo32 = 0x92,
    /// @tprel(sym + add), imm64 Or ShTlsIe32
    Ia64Tprel64IOrShTlsIe32 = 0x93,
    /// @tprel(sym + add), data8 MSB Or ShTlsDtpoff32
    Ia64Tprel64MsbOrShTlsDtpoff32 = 0x96,
    /// @tprel(sym + add), data8 LSB Or ShTlsTpoff32
    Ia64Tprel64LsbOrShTlsTpoff32 = 0x97,
    /// @ltoff(@dtpmod(sym + add)), imm22
    Ia64LtoffDtpmod22 = 0xaa,
    /// @dtprel(sym + add), imm14
    Ia64Dtprel14 = 0xb1,
    /// @dtprel(sym + add), imm22
    Ia64Dtprel22 = 0xb2,
    /// @dtprel(sym + add), imm64
    Ia64Dtprel64I = 0xb3,
    ShTlsGd32 = 0x90,
    ShTlsLe32 = 0x94,
    ShTlsDtpmod32 = 0x95,
    ShPlt32 = 0xa1,
    ShGlobDat = 0xa3,
    ShJmpSlot = 0xa4,
    ShRelative = 0xa5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RelocationTypeU8(pub RelocationType);
impl BinarySerde for RelocationTypeU8 {
    const SERIALIZED_SIZE: usize = 1;

    type RecursiveArray = <u8 as BinarySerde>::RecursiveArray;

    fn binary_serialize(&self, buf: &mut [u8], endianness: Endianness) {
        let value = self.0 as u8;
        value.binary_serialize(buf, endianness)
    }

    fn binary_deserialize(
        buf: &[u8],
        endianness: Endianness,
    ) -> Result<Self, binary_serde::DeserializeError> {
        let bytes = (buf[0] as u32).binary_serialize_to_array(endianness);
        Ok(Self(RelocationType::binary_deserialize(
            bytes.as_ref(),
            endianness,
        )?))
    }
}
