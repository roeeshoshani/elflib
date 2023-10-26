use binary_serde::{impl_binary_serde_for_bitflags_ty, BinarySerde};
use bitflags::bitflags;

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
    I386 = 0x3,
    /// Motorola m68k family
    M68K = 0x4,
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
    /// Fujitsu VPP500 Or Old version of PowerPC.  Deprecated Or Fujitsu VPP500
    _Vpp500OrPpcOldOrVpp550 = 0x11,
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
    /// Motorola RCE Or Motorola M*Core May also be taken by Fujitsu MMA Or CskyOld
    _RceOrMcoreOrCskyOld = 0x27,
    /// ARM
    Arm = 0x28,
    /// Digital Alpha Or Digital Alpha
    _FakeAlphaOrOldAlpha = 0x29,
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
    /// OpenRISC 32-bit embedded processor Or OpenRISC 1000 32-bit embedded processor
    _OpenriscOrOr1K = 0x5c,
    /// ARC International ARCompact Or ArcA5
    _ArcCompactOrArcA5 = 0x5d,
    /// Tensilica Xtensa Architecture
    Xtensa = 0x5e,
    /// Alphamosaic VideoCore Or Old Sunplus S+core7 backend magic number. Written in the absence of an ABI
    _VideocoreOrScoreOld = 0x5f,
    /// Thompson Multimedia General Purpose Proc
    TmmGpp = 0x60,
    /// National Semi. 32000
    Ns32K = 0x61,
    /// Tenor Network TPC
    Tpc = 0x62,
    /// Trebia SNP 1000 Or Old value for picoJava.  Deprecated
    _Snp1KOrPjOld = 0x63,
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
    /// Motorola XGATE Or Old, value for National Semiconductor CompactRISC.  Deprecated
    _XgateOrCr16Old = 0x73,
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
    /// Analog Devices SHARC family Or Reserved
    _SharcOrRes133 = 0x85,
    /// Cyan Technology eCOG2
    Ecog2 = 0x86,
    /// Sunplus S+core7 RISC Or Sunplus Score
    _Score7OrScore = 0x87,
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
    /// Texas Instruments App. Specific RISC Or Reserved
    _TiArp32OrRes143 = 0x8f,
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
    /// Cyan Technology eCOG1X Or Cyan Technology eCOG1X family
    _Ecog1XOrEcog1 = 0xa8,
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
    /// Intel L10M Or Intel L1OM
    _L10MOrL1Om = 0xb4,
    /// Intel K10M Or Intel K1OM
    _K10MOrK1Om = 0xb5,
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
    /// Synopsys ARCv2 ISA Or Synopsys ARCompact V2
    _Arcv2OrArcCompact2 = 0xc3,
    /// Open8 RISC
    Open8 = 0xc4,
    /// Renesas RL78
    Rl78 = 0xc5,
    /// Broadcom VideoCore V
    Videocore5 = 0xc6,
    /// Renesas 78KOR Or Renesas 78K0R
    _Renesas78KorOrRenesas78K0R = 0xc7,
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
    /// KM211 KMX16 Or KM211 KMX16 16-bit processor
    _Emx16OrKmx16 = 0xd4,
    /// KM211 KMX8 Or KM211 KMX8 8-bit processor
    _Emx8OrKmx8 = 0xd5,
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
    /// C-SKY
    Csky = 0xfc,
    Alpha = 0x9026,
    /// Old version of Sparc v9, from before the ABI.  Deprecated Or Reserved
    _OldSparcv9OrRes011 = 0xb,
    /// Reserved
    Res012 = 0xc,
    /// Reserved
    Res013 = 0xd,
    /// Reserved
    Res014 = 0xe,
    /// Reserved
    Res016 = 0x10,
    /// Reserved
    Res024 = 0x18,
    /// Reserved
    Res025 = 0x19,
    /// Reserved
    Res026 = 0x1a,
    /// Reserved
    Res027 = 0x1b,
    /// Reserved
    Res028 = 0x1c,
    /// Reserved
    Res029 = 0x1d,
    /// Reserved
    Res030 = 0x1e,
    /// Reserved
    Res031 = 0x1f,
    /// Reserved
    Res032 = 0x20,
    /// Reserved
    Res033 = 0x21,
    /// Reserved
    Res034 = 0x22,
    /// Reserved
    Res035 = 0x23,
    /// Reserved
    Res121 = 0x79,
    /// Reserved
    Res122 = 0x7a,
    /// Reserved
    Res123 = 0x7b,
    /// Reserved
    Res124 = 0x7c,
    /// Reserved
    Res125 = 0x7d,
    /// Reserved
    Res126 = 0x7e,
    /// Reserved
    Res127 = 0x7f,
    /// Reserved
    Res128 = 0x80,
    /// Reserved
    Res129 = 0x81,
    /// Reserved
    Res130 = 0x82,
    /// Reserved
    Res145 = 0x91,
    /// Reserved
    Res146 = 0x92,
    /// Reserved
    Res147 = 0x93,
    /// Reserved
    Res148 = 0x94,
    /// Reserved
    Res149 = 0x95,
    /// Reserved
    Res150 = 0x96,
    /// Reserved
    Res151 = 0x97,
    /// Reserved
    Res152 = 0x98,
    /// Reserved
    Res153 = 0x99,
    /// Reserved
    Res154 = 0x9a,
    /// Reserved
    Res155 = 0x9b,
    /// Reserved
    Res156 = 0x9c,
    /// Reserved
    Res157 = 0x9d,
    /// Reserved
    Res158 = 0x9e,
    /// Reserved
    Res159 = 0x9f,
    /// Reserved by Intel
    Intel182 = 0xb6,
    /// Reserved by ARM
    Arm184 = 0xb8,
    /// Reserved by Intel
    Intel206 = 0xce,
    /// Reserved by Intel
    Intel207 = 0xcf,
    /// Reserved by Intel
    Intel208 = 0xd0,
    /// Reserved by Intel
    Intel209 = 0xd1,
    /// Lanai 32-bit processor.
    Lanai = 0xf4,
    /// CEVA Processor Architecture Family
    Ceva = 0xf5,
    /// CEVA X2 Processor Family
    CevaX2 = 0xf6,
    /// Graphcore Intelligent Processing Unit
    GraphcoreIpu = 0xf8,
    /// Imagination Technologies
    Img1 = 0xf9,
    /// Netronome Flow Processor.
    Nfp = 0xfa,
    /// NEC Vector Engine
    Ve = 0xfb,
    /// Synopsys ARCv2.3 64-bit
    ArcCompact364 = 0xfd,
    /// MOS Technology MCS 6502 processor
    Mcs6502 = 0xfe,
    /// Synopsys ARCv2.3 32-bit
    ArcCompact3 = 0xff,
    /// Kalray VLIW core of the MPPA processor family
    Kvx = 0x100,
    /// WDC 65816/65C816
    Wdc65816 = 0x101,
    /// LoongArch
    Loongarch = 0x102,
    /// ChipON KungFu32
    Kf32 = 0x103,
    /// LAPIS nX-U16/U8
    U16U8Core = 0x104,
    /// Tachyum
    Tachyum = 0x105,
    /// NXP 56800EF Digital Signal Controller (DSC)
    Nxp56800Ef = 0x106,
    AvrOld = 0x1057,
    Msp430Old = 0x1059,
    Mt = 0x2530,
    CygnusFr30 = 0x3330,
    Webassembly = 0x4157,
    S12Z = 0x4def,
    Dlx = 0x5aa5,
    CygnusFrv = 0x5441,
    Xc16X = 0x4688,
    CygnusD10V = 0x7650,
    CygnusD30V = 0x7676,
    Ip2KOld = 0x8217,
    CygnusPowerpc = 0x9025,
    CygnusM32R = 0x9041,
    CygnusV850 = 0x9080,
    S390Old = 0xa390,
    XtensaOld = 0xabc7,
    Xstormy16 = 0xad45,
    CygnusMn10300 = 0xbeef,
    CygnusMn10200 = 0xdead,
    M32COld = 0xfeb0,
    Iq2000 = 0xfeba,
    Nios32 = 0xfebb,
    /// Toshiba MeP
    CygnusMep = 0xf00d,
    MoxieOld = 0xfeed,
    /// Old MicroBlaze
    MicroblazeOld = 0xbaab,
    /// Adapteva's Epiphany architecture.
    AdaptevaEpiphany = 0x1223,
}
impl Architechture {
    /// Fujitsu VPP500
    pub const VPP500: Self = Self::_Vpp500OrPpcOldOrVpp550;
    /// Old version of PowerPC.  Deprecated.
    pub const PPC_OLD: Self = Self::_Vpp500OrPpcOldOrVpp550;
    /// Fujitsu VPP500
    pub const VPP550: Self = Self::_Vpp500OrPpcOldOrVpp550;
    /// Motorola RCE
    pub const RCE: Self = Self::_RceOrMcoreOrCskyOld;
    /// Motorola M*Core May also be taken by Fujitsu MMA
    pub const MCORE: Self = Self::_RceOrMcoreOrCskyOld;
    pub const CSKY_OLD: Self = Self::_RceOrMcoreOrCskyOld;
    /// Digital Alpha
    pub const FAKE_ALPHA: Self = Self::_FakeAlphaOrOldAlpha;
    /// Digital Alpha
    pub const OLD_ALPHA: Self = Self::_FakeAlphaOrOldAlpha;
    /// OpenRISC 32-bit embedded processor
    pub const OPENRISC: Self = Self::_OpenriscOrOr1K;
    /// OpenRISC 1000 32-bit embedded processor
    pub const OR1K: Self = Self::_OpenriscOrOr1K;
    /// ARC International ARCompact
    pub const ARC_COMPACT: Self = Self::_ArcCompactOrArcA5;
    pub const ARC_A5: Self = Self::_ArcCompactOrArcA5;
    /// Alphamosaic VideoCore
    pub const VIDEOCORE: Self = Self::_VideocoreOrScoreOld;
    /// Old Sunplus S+core7 backend magic number. Written in the absence of an ABI.
    pub const SCORE_OLD: Self = Self::_VideocoreOrScoreOld;
    /// Trebia SNP 1000
    pub const SNP1K: Self = Self::_Snp1KOrPjOld;
    /// Old value for picoJava.  Deprecated.
    pub const PJ_OLD: Self = Self::_Snp1KOrPjOld;
    /// Motorola XGATE
    pub const XGATE: Self = Self::_XgateOrCr16Old;
    /// Old, value for National Semiconductor CompactRISC.  Deprecated.
    pub const CR16_OLD: Self = Self::_XgateOrCr16Old;
    /// Analog Devices SHARC family
    pub const SHARC: Self = Self::_SharcOrRes133;
    /// Reserved
    pub const RES133: Self = Self::_SharcOrRes133;
    /// Sunplus S+core7 RISC
    pub const SCORE7: Self = Self::_Score7OrScore;
    /// Sunplus Score
    pub const SCORE: Self = Self::_Score7OrScore;
    /// Texas Instruments App. Specific RISC
    pub const TI_ARP32: Self = Self::_TiArp32OrRes143;
    /// Reserved
    pub const RES143: Self = Self::_TiArp32OrRes143;
    /// Cyan Technology eCOG1X
    pub const ECOG1X: Self = Self::_Ecog1XOrEcog1;
    /// Cyan Technology eCOG1X family
    pub const ECOG1: Self = Self::_Ecog1XOrEcog1;
    /// Intel L10M
    pub const L10M: Self = Self::_L10MOrL1Om;
    /// Intel L1OM
    pub const L1OM: Self = Self::_L10MOrL1Om;
    /// Intel K10M
    pub const K10M: Self = Self::_K10MOrK1Om;
    /// Intel K1OM
    pub const K1OM: Self = Self::_K10MOrK1Om;
    /// Synopsys ARCv2 ISA.
    pub const ARCV2: Self = Self::_Arcv2OrArcCompact2;
    /// Synopsys ARCompact V2
    pub const ARC_COMPACT2: Self = Self::_Arcv2OrArcCompact2;
    /// Renesas 78KOR
    pub const RENESAS_78KOR: Self = Self::_Renesas78KorOrRenesas78K0R;
    /// Renesas 78K0R.
    pub const RENESAS_78K0R: Self = Self::_Renesas78KorOrRenesas78K0R;
    /// KM211 KMX16
    pub const EMX16: Self = Self::_Emx16OrKmx16;
    /// KM211 KMX16 16-bit processor
    pub const KMX16: Self = Self::_Emx16OrKmx16;
    /// KM211 KMX8
    pub const EMX8: Self = Self::_Emx8OrKmx8;
    /// KM211 KMX8 8-bit processor
    pub const KMX8: Self = Self::_Emx8OrKmx8;
    /// Old version of Sparc v9, from before the ABI.  Deprecated.
    pub const OLD_SPARCV9: Self = Self::_OldSparcv9OrRes011;
    /// Reserved
    pub const RES011: Self = Self::_OldSparcv9OrRes011;
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
    /// Start of OS-specific Or HpTls
    _LoosOrHpTls = 0x60000000,
    /// GCC .eh_frame_hdr segment Or Solaris uses the same value
    _GnuEhFrameOrSunwEhFrame = 0x6474e550,
    /// Indicates stack executability
    GnuStack = 0x6474e551,
    /// Read-only after relocation
    GnuRelro = 0x6474e552,
    /// GNU property
    GnuProperty = 0x6474e553,
    /// Losunw Or Sun Specific segment
    _LosunwOrSunwbss = 0x6ffffffa,
    /// Stack segment
    Sunwstack = 0x6ffffffb,
    /// Hisunw Or End of OS-specific
    _HisunwOrHios = 0x6fffffff,
    /// Start of processor-specific Or Register usage information Or PariscArchext Or arch extension bits Or SpuInfo Or S390Pgste Or C6000Phattr Or AArch64Archext
    _LoprocOrMipsReginfoOrPariscArchextOrIa64ArchextOrSpuInfoOrS390PgsteOrC6000PhattrOrAArch64Archext =
        0x70000000,
    /// End of processor-specific
    Hiproc = 0x7fffffff,
    /// Runtime procedure table Or PariscUnwind Or ARM unwind segment Or ia64 unwind bits
    _MipsRtprocOrPariscUnwindOrArmExidxOrIa64Unwind = 0x70000001,
    _MipsOptionsOrPariscWeakorderOrAArch64MemtagMte = 0x70000002,
    /// FP mode requirement Or RiscvAttributes
    _MipsAbiflagsOrRiscvAttributes = 0x70000003,
    HpCoreNone = 0x60000001,
    HpCoreVersion = 0x60000002,
    HpCoreKernel = 0x60000003,
    HpCoreComm = 0x60000004,
    HpCoreProc = 0x60000005,
    HpCoreLoadable = 0x60000006,
    HpCoreStack = 0x60000007,
    HpCoreShm = 0x60000008,
    HpCoreMmf = 0x60000009,
    HpParallel = 0x60000010,
    HpFastbind = 0x60000011,
    _HpOptAnnotOrIa64HpOptAnot = 0x60000012,
    _HpHslAnnotOrIa64HpHslAnot = 0x60000013,
    _HpStackOrIa64HpStack = 0x60000014,
    HpCoreUtsname = 0x60000015,
    /// SFrame stack trace information
    GnuSframe = 0x6474e554,
    /// Like bss, but not immutable.
    OpenbsdMutable = 0x65a3dbe5,
    /// Fill with random data.
    OpenbsdRandomize = 0x65a3dbe6,
    /// Program does W^X violations.
    OpenbsdWxneeded = 0x65a3dbe7,
    /// No branch target CFI.
    OpenbsdNobtcfi = 0x65a3dbe8,
    /// Section for boot arguments.
    OpenbsdBootdata = 0x65a41be6,
    GnuMbindNum = 0x1000,
    GnuMbindLo = 0x6474e555,
    GnuMbindHi = 0x6474f554,
}
impl ProgramHeaderType {
    /// Start of OS-specific
    pub const LOOS: Self = Self::_LoosOrHpTls;
    pub const HP_TLS: Self = Self::_LoosOrHpTls;
    /// GCC .eh_frame_hdr segment
    pub const GNU_EH_FRAME: Self = Self::_GnuEhFrameOrSunwEhFrame;
    /// Solaris uses the same value
    pub const SUNW_EH_FRAME: Self = Self::_GnuEhFrameOrSunwEhFrame;
    pub const LOSUNW: Self = Self::_LosunwOrSunwbss;
    /// Sun Specific segment
    pub const SUNWBSS: Self = Self::_LosunwOrSunwbss;
    pub const HISUNW: Self = Self::_HisunwOrHios;
    /// End of OS-specific
    pub const HIOS: Self = Self::_HisunwOrHios;
    /// Start of processor-specific
    pub const LOPROC : Self = Self::_LoprocOrMipsReginfoOrPariscArchextOrIa64ArchextOrSpuInfoOrS390PgsteOrC6000PhattrOrAArch64Archext;
    /// Register usage information.
    pub const MIPS_REGINFO : Self = Self::_LoprocOrMipsReginfoOrPariscArchextOrIa64ArchextOrSpuInfoOrS390PgsteOrC6000PhattrOrAArch64Archext;
    pub const PARISC_ARCHEXT : Self = Self::_LoprocOrMipsReginfoOrPariscArchextOrIa64ArchextOrSpuInfoOrS390PgsteOrC6000PhattrOrAArch64Archext;
    /// arch extension bits
    pub const IA_64_ARCHEXT : Self = Self::_LoprocOrMipsReginfoOrPariscArchextOrIa64ArchextOrSpuInfoOrS390PgsteOrC6000PhattrOrAArch64Archext;
    pub const SPU_INFO : Self = Self::_LoprocOrMipsReginfoOrPariscArchextOrIa64ArchextOrSpuInfoOrS390PgsteOrC6000PhattrOrAArch64Archext;
    pub const S390_PGSTE : Self = Self::_LoprocOrMipsReginfoOrPariscArchextOrIa64ArchextOrSpuInfoOrS390PgsteOrC6000PhattrOrAArch64Archext;
    pub const C6000_PHATTR : Self = Self::_LoprocOrMipsReginfoOrPariscArchextOrIa64ArchextOrSpuInfoOrS390PgsteOrC6000PhattrOrAArch64Archext;
    pub const AARCH64_ARCHEXT : Self = Self::_LoprocOrMipsReginfoOrPariscArchextOrIa64ArchextOrSpuInfoOrS390PgsteOrC6000PhattrOrAArch64Archext;
    /// Runtime procedure table.
    pub const MIPS_RTPROC: Self = Self::_MipsRtprocOrPariscUnwindOrArmExidxOrIa64Unwind;
    pub const PARISC_UNWIND: Self = Self::_MipsRtprocOrPariscUnwindOrArmExidxOrIa64Unwind;
    /// ARM unwind segment.
    pub const ARM_EXIDX: Self = Self::_MipsRtprocOrPariscUnwindOrArmExidxOrIa64Unwind;
    /// ia64 unwind bits
    pub const IA_64_UNWIND: Self = Self::_MipsRtprocOrPariscUnwindOrArmExidxOrIa64Unwind;
    pub const MIPS_OPTIONS: Self = Self::_MipsOptionsOrPariscWeakorderOrAArch64MemtagMte;
    pub const PARISC_WEAKORDER: Self = Self::_MipsOptionsOrPariscWeakorderOrAArch64MemtagMte;
    pub const AARCH64_MEMTAG_MTE: Self = Self::_MipsOptionsOrPariscWeakorderOrAArch64MemtagMte;
    /// FP mode requirement.
    pub const MIPS_ABIFLAGS: Self = Self::_MipsAbiflagsOrRiscvAttributes;
    pub const RISCV_ATTRIBUTES: Self = Self::_MipsAbiflagsOrRiscvAttributes;
    pub const HP_OPT_ANNOT: Self = Self::_HpOptAnnotOrIa64HpOptAnot;
    pub const IA_64_HP_OPT_ANOT: Self = Self::_HpOptAnnotOrIa64HpOptAnot;
    pub const HP_HSL_ANNOT: Self = Self::_HpHslAnnotOrIa64HpHslAnot;
    pub const IA_64_HP_HSL_ANOT: Self = Self::_HpHslAnnotOrIa64HpHslAnot;
    pub const HP_STACK: Self = Self::_HpStackOrIa64HpStack;
    pub const IA_64_HP_STACK: Self = Self::_HpStackOrIa64HpStack;
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ProgramHeaderFlags: u32 {
        const OVERLAY = 0x8000000;
        const PARISC_SBP = 0x8000000;
        const HP_SBP = 0x8000000;
        const IA_64_NORECOV = 0x80000000;
        const HP_CODE = 0x40000;
        const HP_MODIFY = 0x80000;
        const HP_PAGE_SIZE = 0x100000;
        const HP_FAR_SHARED = 0x200000;
        const HP_NEAR_SHARED = 0x400000;
        const HP_LAZYSWAP = 0x800000;
        const HP_CODE_DEPR = 0x1000000;
        const HP_MODIFY_DEPR = 0x2000000;
        const HP_LAZYSWAP_DEPR = 0x4000000;
        /// PowerPC VLE.
        const PPC_VLE = 0x10000000;
        /// Segment contains the location addressed by the static base.
        const ARM_SB = 0x10000000;
        const MIPS_LOCAL = 0x10000000;
        /// Segment is position-independent.
        const ARM_PI = 0x20000000;
        /// Segment must be loaded at its base address.
        const ARM_ABS = 0x40000000;
        /// Segment is executable
        const X = 0x1;
        /// Segment is writable
        const W = 0x2;
        /// Segment is readable
        const R = 0x4;
        /// New value, Oct 4, 1999 Draft
        const MASKOS = 0xff00000;
        /// Processor-specific reserved bits
        const MASKPROC = 0xf0000000;
    }
}
impl_binary_serde_for_bitflags_ty! {ProgramHeaderFlags}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SectionHeaderType {
    /// unwind information Or V850Tcommon Or PariscUnwind Or MipsMsym Or Section holds ARM unwind info Or C6000Unwind Or Section holds attributes Or Section holds attributes Or AlphaDebug Or Unwind bits Or NfpMeconfig
    _X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig =
        0x70000001,
    /// V850Scommon Or PariscExt Or MipsLiblist Or Processor-specific semantics, lo Or Extension bits
    _V850ScommonOrPariscExtOrMipsLiblistOrLoprocOrIa64Ext = 0x70000000,
    /// V850Zcommon Or PariscDoc Or MipsConflict Or Section pre-emption details Or C6000Preemptmap Or AlphaReginfo Or NfpInitreg
    _V850ZcommonOrPariscDocOrMipsConflictOrArmPreemptmapOrC6000PreemptmapOrAlphaReginfoOrNfpInitreg =
        0x70000002,
    /// Used by Renesas linker Or NfpUdebug Or Application-specific semantics
    _RenesasIopOrNfpUdebugOrLouser = 0x80000000,
    RenesasInfo = 0xa0000000,
    /// Section holds attributes Or PariscAnnot Or MipsGptab Or Section holds ABI attributes Or Section holds attributes Or C6000Attributes Or Section holds attributes
    _RiscvAttributesOrPariscAnnotOrMipsGptabOrMsp430AttributesOrArmAttributesOrC6000AttributesOrAArch64Attributes =
        0x70000003,
    /// Ordered Or Processor-specific semantics, hi
    _OrderedOrHiproc = 0x7fffffff,
    Ia64Lopsreg = 0x78000000,
    Ia64Hipsreg = 0x78ffffff,
    Ia64PriorityInit = 0x79000000,
    _Ia64HpOptAnotOrIa64VmsLinkagesOrHpAnnot = 0x60000004,
    /// Ia64VmsTrace Or First of OS specific semantics Or HpOvlbits
    _Ia64VmsTraceOrLoosOrHpOvlbits = 0x60000000,
    _Ia64VmsTieSignaturesOrHpDlkm = 0x60000001,
    _Ia64VmsDebugOrHpComdat = 0x60000002,
    _Ia64VmsDebugStrOrHpObjdict = 0x60000003,
    Ia64VmsSymbolVector = 0x60000005,
    Ia64VmsFixup = 0x60000006,
    Ia64VmsDisplayNameInfo = 0x60000007,
    /// PariscDlkm Or MipsUcode Or Section holds overlay debug info
    _PariscDlkmOrMipsUcodeOrArmDebugoverlay = 0x70000004,
    _PariscSymextnOrMipsPacksym = 0x70000008,
    _PariscStubsOrMipsReld = 0x70000009,
    /// MipsDebug Or Section holds GDB and overlay integration info
    _MipsDebugOrArmOverlaysection = 0x70000005,
    MipsReginfo = 0x70000006,
    MipsPackage = 0x70000007,
    MipsIface = 0x7000000b,
    MipsContent = 0x7000000c,
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
    MipsDwarf = 0x7000001e,
    MipsDeltadecl = 0x7000001f,
    MipsSymbolLib = 0x70000020,
    MipsEvents = 0x70000021,
    MipsTranslate = 0x70000022,
    MipsPixie = 0x70000023,
    MipsXlate = 0x70000024,
    MipsXlateDebug = 0x70000025,
    MipsWhirl = 0x70000026,
    MipsEhRegion = 0x70000027,
    MipsXlateOld = 0x70000028,
    MipsPdrException = 0x70000029,
    MipsAbiflags = 0x7000002a,
    MipsXhash = 0x7000002b,
    /// Holds TI compiler's section flags.
    Msp430SecFlags = 0x7f000005,
    /// Holds TI compiler's symbol aliases.
    Msp430SymAliases = 0x7f000006,
    TiIcode = 0x7f000000,
    TiXref = 0x7f000001,
    TiHandler = 0x7f000002,
    TiInitinfo = 0x7f000003,
    TiPhattrs = 0x7f000004,
    /// Section header table entry unused
    Null = 0x0,
    /// Program specific (private) data
    Progbits = 0x1,
    /// Link editing symbol table
    Symtab = 0x2,
    /// A string table
    Strtab = 0x3,
    /// Relocation entries with addends
    Rela = 0x4,
    /// A symbol hash table
    Hash = 0x5,
    /// Information for dynamic linking
    Dynamic = 0x6,
    /// Information that marks file
    Note = 0x7,
    /// Section occupies no space in file
    Nobits = 0x8,
    /// Relocation entries, no addends
    Rel = 0x9,
    /// Reserved, unspecified semantics
    Shlib = 0xa,
    /// Dynamic linking symbol table
    Dynsym = 0xb,
    /// Array of ptrs to init functions
    InitArray = 0xe,
    /// Array of ptrs to finish functions
    FiniArray = 0xf,
    /// Array of ptrs to pre-init funcs
    PreinitArray = 0x10,
    /// Section contains a section group
    Group = 0x11,
    /// Indices for SHN_XINDEX entries
    SymtabShndx = 0x12,
    /// RELR relative relocations
    Relr = 0x13,
    /// Last of OS specific semantics Or Symbol versions Or Sun-specific high bound Or GnuVersym
    _HiosOrSunwVersymOrHisunwOrGnuVersym = 0x6fffffff,
    /// incremental build data
    GnuIncrementalInputs = 0x6fff4700,
    /// Object attributes
    GnuAttributes = 0x6ffffff5,
    /// GNU style symbol hash table
    GnuHash = 0x6ffffff6,
    /// List of prelink dependencies
    GnuLiblist = 0x6ffffff7,
    /// Versions defined by file Or GnuVerdef
    _SunwVerdefOrGnuVerdef = 0x6ffffffd,
    /// Versions needed by file Or GnuVerneed
    _SunwVerneedOrGnuVerneed = 0x6ffffffe,
    /// New value, defined in Oct 4, 1999 Draft
    Hiuser = 0xffffffff,
    /// Checksum for DSO content.
    Checksum = 0x6ffffff8,
    /// Sun-specific low bound Or SunwMove
    _LosunwOrSunwMove = 0x6ffffffa,
    SunwComdat = 0x6ffffffb,
    SunwSyminfo = 0x6ffffffc,
}
impl SectionHeaderType {
    /// unwind information
    pub const X86_64_UNWIND : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    pub const V850_TCOMMON : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    pub const PARISC_UNWIND : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    pub const MIPS_MSYM : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    /// Section holds ARM unwind info.
    pub const ARM_EXIDX : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    pub const C6000_UNWIND : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    /// Section holds attributes.
    pub const ARC_ATTRIBUTES : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    /// Section holds attributes.
    pub const CSKY_ATTRIBUTES : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    pub const ALPHA_DEBUG : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    /// Unwind bits.
    pub const IA_64_UNWIND : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    pub const NFP_MECONFIG : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    pub const V850_SCOMMON: Self = Self::_V850ScommonOrPariscExtOrMipsLiblistOrLoprocOrIa64Ext;
    pub const PARISC_EXT: Self = Self::_V850ScommonOrPariscExtOrMipsLiblistOrLoprocOrIa64Ext;
    pub const MIPS_LIBLIST: Self = Self::_V850ScommonOrPariscExtOrMipsLiblistOrLoprocOrIa64Ext;
    /// Processor-specific semantics, lo
    pub const LOPROC: Self = Self::_V850ScommonOrPariscExtOrMipsLiblistOrLoprocOrIa64Ext;
    /// Extension bits.
    pub const IA_64_EXT: Self = Self::_V850ScommonOrPariscExtOrMipsLiblistOrLoprocOrIa64Ext;
    pub const V850_ZCOMMON : Self = Self::_V850ZcommonOrPariscDocOrMipsConflictOrArmPreemptmapOrC6000PreemptmapOrAlphaReginfoOrNfpInitreg;
    pub const PARISC_DOC : Self = Self::_V850ZcommonOrPariscDocOrMipsConflictOrArmPreemptmapOrC6000PreemptmapOrAlphaReginfoOrNfpInitreg;
    pub const MIPS_CONFLICT : Self = Self::_V850ZcommonOrPariscDocOrMipsConflictOrArmPreemptmapOrC6000PreemptmapOrAlphaReginfoOrNfpInitreg;
    /// Section pre-emption details.
    pub const ARM_PREEMPTMAP : Self = Self::_V850ZcommonOrPariscDocOrMipsConflictOrArmPreemptmapOrC6000PreemptmapOrAlphaReginfoOrNfpInitreg;
    pub const C6000_PREEMPTMAP : Self = Self::_V850ZcommonOrPariscDocOrMipsConflictOrArmPreemptmapOrC6000PreemptmapOrAlphaReginfoOrNfpInitreg;
    pub const ALPHA_REGINFO : Self = Self::_V850ZcommonOrPariscDocOrMipsConflictOrArmPreemptmapOrC6000PreemptmapOrAlphaReginfoOrNfpInitreg;
    pub const NFP_INITREG : Self = Self::_V850ZcommonOrPariscDocOrMipsConflictOrArmPreemptmapOrC6000PreemptmapOrAlphaReginfoOrNfpInitreg;
    /// Used by Renesas linker.
    pub const RENESAS_IOP: Self = Self::_RenesasIopOrNfpUdebugOrLouser;
    pub const NFP_UDEBUG: Self = Self::_RenesasIopOrNfpUdebugOrLouser;
    /// Application-specific semantics
    pub const LOUSER: Self = Self::_RenesasIopOrNfpUdebugOrLouser;
    /// Section holds attributes.
    pub const RISCV_ATTRIBUTES : Self = Self::_RiscvAttributesOrPariscAnnotOrMipsGptabOrMsp430AttributesOrArmAttributesOrC6000AttributesOrAArch64Attributes;
    pub const PARISC_ANNOT : Self = Self::_RiscvAttributesOrPariscAnnotOrMipsGptabOrMsp430AttributesOrArmAttributesOrC6000AttributesOrAArch64Attributes;
    pub const MIPS_GPTAB : Self = Self::_RiscvAttributesOrPariscAnnotOrMipsGptabOrMsp430AttributesOrArmAttributesOrC6000AttributesOrAArch64Attributes;
    /// Section holds ABI attributes.
    pub const MSP430_ATTRIBUTES : Self = Self::_RiscvAttributesOrPariscAnnotOrMipsGptabOrMsp430AttributesOrArmAttributesOrC6000AttributesOrAArch64Attributes;
    /// Section holds attributes.
    pub const ARM_ATTRIBUTES : Self = Self::_RiscvAttributesOrPariscAnnotOrMipsGptabOrMsp430AttributesOrArmAttributesOrC6000AttributesOrAArch64Attributes;
    pub const C6000_ATTRIBUTES : Self = Self::_RiscvAttributesOrPariscAnnotOrMipsGptabOrMsp430AttributesOrArmAttributesOrC6000AttributesOrAArch64Attributes;
    /// Section holds attributes.
    pub const AARCH64_ATTRIBUTES : Self = Self::_RiscvAttributesOrPariscAnnotOrMipsGptabOrMsp430AttributesOrArmAttributesOrC6000AttributesOrAArch64Attributes;
    pub const ORDERED: Self = Self::_OrderedOrHiproc;
    /// Processor-specific semantics, hi
    pub const HIPROC: Self = Self::_OrderedOrHiproc;
    pub const IA_64_HP_OPT_ANOT: Self = Self::_Ia64HpOptAnotOrIa64VmsLinkagesOrHpAnnot;
    pub const IA_64_VMS_LINKAGES: Self = Self::_Ia64HpOptAnotOrIa64VmsLinkagesOrHpAnnot;
    pub const HP_ANNOT: Self = Self::_Ia64HpOptAnotOrIa64VmsLinkagesOrHpAnnot;
    pub const IA_64_VMS_TRACE: Self = Self::_Ia64VmsTraceOrLoosOrHpOvlbits;
    /// First of OS specific semantics
    pub const LOOS: Self = Self::_Ia64VmsTraceOrLoosOrHpOvlbits;
    pub const HP_OVLBITS: Self = Self::_Ia64VmsTraceOrLoosOrHpOvlbits;
    pub const IA_64_VMS_TIE_SIGNATURES: Self = Self::_Ia64VmsTieSignaturesOrHpDlkm;
    pub const HP_DLKM: Self = Self::_Ia64VmsTieSignaturesOrHpDlkm;
    pub const IA_64_VMS_DEBUG: Self = Self::_Ia64VmsDebugOrHpComdat;
    pub const HP_COMDAT: Self = Self::_Ia64VmsDebugOrHpComdat;
    pub const IA_64_VMS_DEBUG_STR: Self = Self::_Ia64VmsDebugStrOrHpObjdict;
    pub const HP_OBJDICT: Self = Self::_Ia64VmsDebugStrOrHpObjdict;
    pub const PARISC_DLKM: Self = Self::_PariscDlkmOrMipsUcodeOrArmDebugoverlay;
    pub const MIPS_UCODE: Self = Self::_PariscDlkmOrMipsUcodeOrArmDebugoverlay;
    /// Section holds overlay debug info.
    pub const ARM_DEBUGOVERLAY: Self = Self::_PariscDlkmOrMipsUcodeOrArmDebugoverlay;
    pub const PARISC_SYMEXTN: Self = Self::_PariscSymextnOrMipsPacksym;
    pub const MIPS_PACKSYM: Self = Self::_PariscSymextnOrMipsPacksym;
    pub const PARISC_STUBS: Self = Self::_PariscStubsOrMipsReld;
    pub const MIPS_RELD: Self = Self::_PariscStubsOrMipsReld;
    pub const MIPS_DEBUG: Self = Self::_MipsDebugOrArmOverlaysection;
    /// Section holds GDB and overlay integration info.
    pub const ARM_OVERLAYSECTION: Self = Self::_MipsDebugOrArmOverlaysection;
    /// Last of OS specific semantics
    pub const HIOS: Self = Self::_HiosOrSunwVersymOrHisunwOrGnuVersym;
    /// Symbol versions
    pub const SUNW_VERSYM: Self = Self::_HiosOrSunwVersymOrHisunwOrGnuVersym;
    /// Sun-specific high bound.
    pub const HISUNW: Self = Self::_HiosOrSunwVersymOrHisunwOrGnuVersym;
    pub const GNU_VERSYM: Self = Self::_HiosOrSunwVersymOrHisunwOrGnuVersym;
    /// Versions defined by file
    pub const SUNW_VERDEF: Self = Self::_SunwVerdefOrGnuVerdef;
    pub const GNU_VERDEF: Self = Self::_SunwVerdefOrGnuVerdef;
    /// Versions needed by file
    pub const SUNW_VERNEED: Self = Self::_SunwVerneedOrGnuVerneed;
    pub const GNU_VERNEED: Self = Self::_SunwVerneedOrGnuVerneed;
    /// Sun-specific low bound.
    pub const LOSUNW: Self = Self::_LosunwOrSunwMove;
    pub const SUNW_MOVE: Self = Self::_LosunwOrSunwMove;
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SectionHeaderFlags: u64 {
        const X86_64_LARGE = 0x10000000;
        const V850_GPREL = 0x10000000;
        const SCORE_GPREL = 0x10000000;
        /// Section near gp.
        const IA_64_SHORT = 0x10000000;
        const PARISC_WEAKORDER = 0x10000000;
        const M32R_CAN_RELAX = 0x10000000;
        const MIPS_GPREL = 0x10000000;
        /// PowerPC VLE text section.
        const PPC_VLE = 0x10000000;
        /// contains vliw code
        const MEP_VLIW = 0x10000000;
        /// Section contains an entry point.
        const ENTRYSECT = 0x10000000;
        const NIOS2_GPREL = 0x10000000;
        const ALPHA_GPREL = 0x10000000;
        /// Section contains an entry point
        const ARM_ENTRYSECT = 0x10000000;
        const V850_EPREL = 0x20000000;
        const SCORE_MERGE = 0x20000000;
        /// Spec insns w/o recovery.
        const IA_64_NORECOV = 0x20000000;
        const PARISC_SHORT = 0x20000000;
        const MIPS_MERGE = 0x20000000;
        /// Section contains only code and no data.
        const ARM_PURECODE = 0x20000000;
        const V850_R0REL = 0x40000000;
        const SCORE_ADDR = 0x40000000;
        const PARISC_HUGE = 0x40000000;
        const MIPS_ADDR = 0x40000000;
        const NFP_INIT2 = 0x40000000;
        /// treat sh_link,sh_info specially
        const ORDERED = 0x40000000;
        /// Absolute section.
        const RENESAS_ABS = 0x80000000;
        const SCORE_STRING = 0x80000000;
        const PARISC_SBP = 0x80000000;
        const MICROBLAZE_NOREAD = 0x80000000;
        const MIPS_STRING = 0x80000000;
        const NFP_INIT = 0x80000000;
        /// Section may be multiply defined in the input to a link step.
        const COMDEF = 0x80000000;
        const MMIX_CANRELAX = 0x80000000;
        const MCORE_NOREAD = 0x80000000;
        const EXCLUDE = 0x80000000;
        const MIPS_STRINGS = 0x80000000;
        const ARM_COMDEF = 0x80000000;
        /// Use unknown.
        const GHS_ABS = 0x400;
        /// Thread local storage section
        const TLS = 0x400;
        const SCORE_NOSTRIP = 0x8000000;
        const HP_COMDAT = 0x8000000;
        const MIPS_NOSTRIP = 0x8000000;
        const SCORE_LOCAL = 0x4000000;
        const HP_FAR_SHARED = 0x4000000;
        const MIPS_LOCAL = 0x4000000;
        const SCORE_NAMES = 0x2000000;
        const HP_NEAR_SHARED = 0x2000000;
        const MIPS_NAMES = 0x2000000;
        const SCORE_NODUPES = 0x1000000;
        /// HP specific TLS flag.
        const IA_64_HP_TLS = 0x1000000;
        const HP_TLS = 0x1000000;
        const MIPS_NODUPES = 0x1000000;
        /// Mbind section.
        const GNU_MBIND = 0x1000000;
        const MIPS_NODUPE = 0x1000000;
        /// Global for clustering.
        const IA_64_VMS_GLOBAL = 0x100000000;
        /// To be overlaid.
        const IA_64_VMS_OVERLAID = 0x200000000;
        /// Shared btw processes.
        const IA_64_VMS_SHARED = 0x400000000;
        /// Priv change mode vect.
        const IA_64_VMS_VECTOR = 0x800000000;
        /// Allocate beyond 2GB.
        const IA_64_VMS_ALLOC_64BIT = 0x1000000000;
        /// Export from sharable.
        const IA_64_VMS_PROTECTED = 0x2000000000;
        const HP_CONST = 0x800000;
        /// Writable data during execution
        const WRITE = 0x1;
        /// Occupies memory during execution
        const ALLOC = 0x2;
        /// Executable machine instructions
        const EXECINSTR = 0x4;
        /// Data in this section can be merged
        const MERGE = 0x10;
        /// Contains null terminated character strings
        const STRINGS = 0x20;
        /// sh_info holds section header table index
        const INFO_LINK = 0x40;
        /// Preserve section ordering when linking
        const LINK_ORDER = 0x80;
        /// OS specific processing required
        const OS_NONCONFORMING = 0x100;
        /// Member of a section group
        const GROUP = 0x200;
        /// Section with compressed data
        const COMPRESSED = 0x800;
        /// New value, Oct 4, 1999 Draft
        const MASKOS = 0xff00000;
        /// Section should not be garbage collected by the linker.
        const GNU_RETAIN = 0x200000;
        /// Processor-specific semantics
        const MASKPROC = 0xf0000000;
    }
}
impl_binary_serde_for_bitflags_ty! {SectionHeaderFlags}

pub enum SymbolType {
    /// Set for functions called at reset time.
    RenesasEntry = 0xe,
    /// PariscMilli Or global reg reserved to app Or Processor-specific semantics Or Global register reserved to app Or Millicode function entry point Or A Thumb function
    _PariscMilliOrRegisterOrLoprocOrSparcRegisterOrPariscMillicodeOrArmTfunc = 0xd,
    HpOpaque = 0xb,
    /// HpStub Or OS-specific semantics
    _HpStubOrHios = 0xc,
    /// A Thumb label Or Processor-specific semantics
    _Arm16BitOrHiproc = 0xf,
    /// Symbol type is unspecified
    Notype = 0x0,
    /// Symbol is a data object
    Object = 0x1,
    /// Symbol is a code object
    Func = 0x2,
    /// Symbol associated with a section
    Section = 0x3,
    /// Symbol gives a file name
    File = 0x4,
    /// An uninitialised common block
    Common = 0x5,
    /// Thread local data object
    Tls = 0x6,
    /// Complex relocation expression
    Relc = 0x8,
    /// Signed Complex relocation expression
    Srelc = 0x9,
    /// OS-specific semantics Or Symbol is an indirect code object
    _LoosOrGnuIfunc = 0xa,
}
impl SymbolType {
    pub const PARISC_MILLI: Self =
        Self::_PariscMilliOrRegisterOrLoprocOrSparcRegisterOrPariscMillicodeOrArmTfunc;
    /// global reg reserved to app.
    pub const REGISTER: Self =
        Self::_PariscMilliOrRegisterOrLoprocOrSparcRegisterOrPariscMillicodeOrArmTfunc;
    /// Processor-specific semantics
    pub const LOPROC: Self =
        Self::_PariscMilliOrRegisterOrLoprocOrSparcRegisterOrPariscMillicodeOrArmTfunc;
    /// Global register reserved to app.
    pub const SPARC_REGISTER: Self =
        Self::_PariscMilliOrRegisterOrLoprocOrSparcRegisterOrPariscMillicodeOrArmTfunc;
    /// Millicode function entry point.
    pub const PARISC_MILLICODE: Self =
        Self::_PariscMilliOrRegisterOrLoprocOrSparcRegisterOrPariscMillicodeOrArmTfunc;
    /// A Thumb function.
    pub const ARM_TFUNC: Self =
        Self::_PariscMilliOrRegisterOrLoprocOrSparcRegisterOrPariscMillicodeOrArmTfunc;
    pub const HP_STUB: Self = Self::_HpStubOrHios;
    /// OS-specific semantics
    pub const HIOS: Self = Self::_HpStubOrHios;
    /// A Thumb label.
    pub const ARM_16BIT: Self = Self::_Arm16BitOrHiproc;
    /// Processor-specific semantics
    pub const HIPROC: Self = Self::_Arm16BitOrHiproc;
    /// OS-specific semantics
    pub const LOOS: Self = Self::_LoosOrGnuIfunc;
    /// Symbol is an indirect code object
    pub const GNU_IFUNC: Self = Self::_LoosOrGnuIfunc;
}

pub enum SymbolBinding {
    /// VMS weak symbol.
    VmsWeak = 0xb,
    /// System symbol Or OS-specific semantics
    _VmsSystemOrHios = 0xc,
    /// HpAlias Or OS-specific semantics Or Symbol is unique in namespace
    _HpAliasOrLoosOrGnuUnique = 0xa,
    /// Symbol not visible outside obj
    Local = 0x0,
    /// Symbol visible outside obj
    Global = 0x1,
    /// Like globals, lower precedence
    Weak = 0x2,
    /// Processor-specific semantics Or MipsSplitCommon
    _LoprocOrMipsSplitCommon = 0xd,
    /// Processor-specific semantics
    Hiproc = 0xf,
}
impl SymbolBinding {
    /// System symbol.
    pub const VMS_SYSTEM: Self = Self::_VmsSystemOrHios;
    /// OS-specific semantics
    pub const HIOS: Self = Self::_VmsSystemOrHios;
    pub const HP_ALIAS: Self = Self::_HpAliasOrLoosOrGnuUnique;
    /// OS-specific semantics
    pub const LOOS: Self = Self::_HpAliasOrLoosOrGnuUnique;
    /// Symbol is unique in namespace
    pub const GNU_UNIQUE: Self = Self::_HpAliasOrLoosOrGnuUnique;
    /// Processor-specific semantics
    pub const LOPROC: Self = Self::_LoprocOrMipsSplitCommon;
    pub const MIPS_SPLIT_COMMON: Self = Self::_LoprocOrMipsSplitCommon;
}

pub enum SymbolVisibility {
    /// Visibility is specified by binding type
    Default = 0x0,
    /// OS specific version of STV_HIDDEN
    Internal = 0x1,
    /// Can only be seen inside currect component
    Hidden = 0x2,
    /// Treat as STB_LOCAL inside current component
    Protected = 0x3,
}

pub enum DynamicTag {
    _X8664PltOrIa64PltReserveOrPpcGotOrC6000DsbtBaseOrXtensaGotLocOffOrLoprocOrPpc64GlinkOrAlphaPltro =
        0x70000000,
    _X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz =
        0x70000001,
    _X8664PltentOrPpc64OptOrAArch64PacPltOrScoreSymtabnoOrMipsIchecksumOrC6000DsbtIndex =
        0x70000003,
    _ScoreLocalGotnoOrMipsTimeStampOrC6000PreemptmapOrNios2GpOrPpc64Opdsz = 0x70000002,
    _ScoreGotsymOrMipsIversion = 0x70000004,
    _ScoreUnrefextnoOrMipsFlagsOrAArch64VariantPcs = 0x70000005,
    _ScoreHipagenoOrMipsBaseAddress = 0x70000006,
    _Ia64VmsSubtypeOrHpEpltrelOrC6000GsymOffsetOrLoos = 0x6000000d,
    _Ia64VmsImgiocntOrHpFilteredOrC6000GstrOffset = 0x6000000f,
    _Ia64VmsLnkflagsOrPltSizeOrVxWrsTlsDataAlign = 0x60000015,
    _Ia64VmsVirMemBlkSizOrDltSize = 0x60000017,
    Ia64VmsIdent = 0x60000019,
    Ia64VmsNeededIdent = 0x6000001d,
    Ia64VmsImgRelaCnt = 0x6000001f,
    Ia64VmsSegRelaCnt = 0x60000021,
    Ia64VmsFixupRelaCnt = 0x60000023,
    Ia64VmsFixupNeeded = 0x60000025,
    Ia64VmsSymvecCnt = 0x60000027,
    Ia64VmsXlated = 0x6000002b,
    Ia64VmsStacksize = 0x6000002d,
    Ia64VmsUnwindsz = 0x6000002f,
    Ia64VmsUnwindCodseg = 0x60000031,
    Ia64VmsUnwindInfoseg = 0x60000033,
    Ia64VmsLinktime = 0x60000035,
    Ia64VmsSegNo = 0x60000037,
    Ia64VmsSymvecOffset = 0x60000039,
    Ia64VmsSymvecSeg = 0x6000003b,
    Ia64VmsUnwindOffset = 0x6000003d,
    Ia64VmsUnwindSeg = 0x6000003f,
    Ia64VmsStrtabOffset = 0x60000041,
    Ia64VmsSysverOffset = 0x60000043,
    Ia64VmsImgRelaOff = 0x60000045,
    Ia64VmsSegRelaOff = 0x60000047,
    Ia64VmsFixupRelaOff = 0x60000049,
    Ia64VmsPltgotOffset = 0x6000004b,
    Ia64VmsPltgotSeg = 0x6000004d,
    Ia64VmsFpmode = 0x6000004f,
    _HpLoadMapOrOldLoos = 0x60000000,
    HpDldFlags = 0x60000001,
    HpDldHook = 0x60000002,
    HpUx10Init = 0x60000003,
    HpUx10Initsz = 0x60000004,
    HpPreinit = 0x60000005,
    HpPreinitsz = 0x60000006,
    HpNeeded = 0x60000007,
    HpTimeStamp = 0x60000008,
    HpChecksum = 0x60000009,
    HpGstSize = 0x6000000a,
    HpGstVersion = 0x6000000b,
    HpGstHashval = 0x6000000c,
    HpEpltrelsz = 0x6000000e,
    _HpFilterTlsOrVxWrsTlsDataStart = 0x60000010,
    _HpCompatFilteredOrVxWrsTlsDataSize = 0x60000011,
    _HpLazyloadOrVxWrsTlsVarsStart = 0x60000012,
    _HpBindNowCountOrVxWrsTlsVarsSize = 0x60000013,
    Plt = 0x60000014,
    Dlt = 0x60000016,
    /// Map text private Or Needed Or AlphaNum Or Ia64Num
    _HpDebugPrivateOrNeededOrAlphaNumOrIa64Num = 0x1,
    /// Callback Or Pltrelsz Or SparcNum Or PpcNum
    _HpDebugCallbackOrPltrelszOrSparcNumOrPpcNum = 0x2,
    /// BOR callback Or Hash Or Ppc64Num
    _HpDebugCallbackBorOrHashOrPpc64Num = 0x4,
    /// No env var Or Relasz
    _HpNoEnvvarOrRelasz = 0x8,
    /// Bind now Or Symbolic Or Versiontagnum
    _HpBindNowOrSymbolicOrVersiontagnum = 0x10,
    /// Bind non-fatal Or Encoding Or PreinitArray
    _HpBindNonfatalOrEncodingOrPreinitArray = 0x20,
    /// Bind verbose
    HpBindVerbose = 0x40,
    /// Bind restricted
    HpBindRestricted = 0x80,
    /// Bind symbolic
    HpBindSymbolic = 0x100,
    /// RPATH first
    HpRpathFirst = 0x200,
    /// Bind depth-first
    HpBindDepthFirst = 0x400,
    /// Dld global sym table
    HpGst = 0x800,
    /// shared vtable support
    HpShlibFixed = 0x1000,
    /// merge shlib data segs
    HpMergeShlibSeg = 0x2000,
    /// never unload
    HpNodelete = 0x4000,
    /// bind only within group
    HpGroup = 0x8000,
    /// protected linkage table
    HpProtectLinkageTable = 0x10000,
    MipsMsym = 0x70000007,
    MipsConflict = 0x70000008,
    MipsLiblist = 0x70000009,
    MipsLocalGotno = 0x7000000a,
    MipsConflictno = 0x7000000b,
    MipsLiblistno = 0x70000010,
    MipsSymtabno = 0x70000011,
    MipsUnrefextno = 0x70000012,
    MipsGotsym = 0x70000013,
    MipsHipageno = 0x70000014,
    MipsRldMap = 0x70000016,
    MipsDeltaClass = 0x70000017,
    MipsDeltaClassNo = 0x70000018,
    MipsDeltaInstance = 0x70000019,
    MipsDeltaInstanceNo = 0x7000001a,
    MipsDeltaReloc = 0x7000001b,
    MipsDeltaRelocNo = 0x7000001c,
    MipsDeltaSym = 0x7000001d,
    MipsDeltaSymNo = 0x7000001e,
    MipsDeltaClasssym = 0x70000020,
    MipsDeltaClasssymNo = 0x70000021,
    MipsCxxFlags = 0x70000022,
    MipsPixieInit = 0x70000023,
    MipsSymbolLib = 0x70000024,
    MipsLocalpageGotidx = 0x70000025,
    MipsLocalGotidx = 0x70000026,
    MipsHiddenGotidx = 0x70000027,
    MipsProtectedGotidx = 0x70000028,
    MipsOptions = 0x70000029,
    MipsInterface = 0x7000002a,
    MipsDynstrAlign = 0x7000002b,
    MipsInterfaceSize = 0x7000002c,
    MipsRldTextResolveAddr = 0x7000002d,
    MipsPerfSuffix = 0x7000002e,
    MipsCompactSize = 0x7000002f,
    MipsGpValue = 0x70000030,
    MipsAuxDynamic = 0x70000031,
    MipsPltgot = 0x70000032,
    MipsRwplt = 0x70000034,
    MipsRldMapRel = 0x70000035,
    MipsXhash = 0x70000036,
    Null = 0x0,
    _PltgotOrExtranum = 0x3,
    Strtab = 0x5,
    _SymtabOrAArch64Num = 0x6,
    Rela = 0x7,
    Relaent = 0x9,
    Strsz = 0xa,
    _SymentOrAddrnum = 0xb,
    _InitOrValnum = 0xc,
    Fini = 0xd,
    Soname = 0xe,
    Rpath = 0xf,
    Rel = 0x11,
    Relsz = 0x12,
    Relent = 0x13,
    Pltrel = 0x14,
    Debug = 0x15,
    Textrel = 0x16,
    Jmprel = 0x17,
    BindNow = 0x18,
    InitArray = 0x19,
    FiniArray = 0x1a,
    InitArraysz = 0x1b,
    FiniArraysz = 0x1c,
    Runpath = 0x1d,
    Flags = 0x1e,
    PreinitArraysz = 0x21,
    SymtabShndx = 0x22,
    Relrsz = 0x23,
    Relr = 0x24,
    Relrent = 0x25,
    Hios = 0x6ffff000,
    _OldHiosOrVerneednum = 0x6fffffff,
    _HiprocOrFilter = 0x7fffffff,
    Valrnglo = 0x6ffffd00,
    GnuFlags1 = 0x6ffffdf4,
    GnuPrelinked = 0x6ffffdf5,
    GnuConflictsz = 0x6ffffdf6,
    GnuLiblistsz = 0x6ffffdf7,
    Checksum = 0x6ffffdf8,
    Pltpadsz = 0x6ffffdf9,
    Moveent = 0x6ffffdfa,
    Movesz = 0x6ffffdfb,
    /// Feature Or Feature selection (DTF_*)
    _FeatureOrFeature1 = 0x6ffffdfc,
    Posflag1 = 0x6ffffdfd,
    Syminsz = 0x6ffffdfe,
    _SyminentOrValrnghi = 0x6ffffdff,
    Addrrnglo = 0x6ffffe00,
    GnuHash = 0x6ffffef5,
    TlsdescPlt = 0x6ffffef6,
    TlsdescGot = 0x6ffffef7,
    GnuConflict = 0x6ffffef8,
    GnuLiblist = 0x6ffffef9,
    Config = 0x6ffffefa,
    Depaudit = 0x6ffffefb,
    Audit = 0x6ffffefc,
    Pltpad = 0x6ffffefd,
    Movetab = 0x6ffffefe,
    _SyminfoOrAddrrnghi = 0x6ffffeff,
    Relacount = 0x6ffffff9,
    Relcount = 0x6ffffffa,
    Flags1 = 0x6ffffffb,
    Verdef = 0x6ffffffc,
    Verdefnum = 0x6ffffffd,
    Verneed = 0x6ffffffe,
    Versym = 0x6ffffff0,
    Auxiliary = 0x7ffffffd,
    Used = 0x7ffffffe,
    /// Most used by any processor Or MipsNum
    _ProcnumOrMipsNum = 0x37,
}
impl DynamicTag {
    pub const X86_64_PLT : Self = Self::_X8664PltOrIa64PltReserveOrPpcGotOrC6000DsbtBaseOrXtensaGotLocOffOrLoprocOrPpc64GlinkOrAlphaPltro;
    pub const IA_64_PLT_RESERVE : Self = Self::_X8664PltOrIa64PltReserveOrPpcGotOrC6000DsbtBaseOrXtensaGotLocOffOrLoprocOrPpc64GlinkOrAlphaPltro;
    pub const PPC_GOT : Self = Self::_X8664PltOrIa64PltReserveOrPpcGotOrC6000DsbtBaseOrXtensaGotLocOffOrLoprocOrPpc64GlinkOrAlphaPltro;
    pub const C6000_DSBT_BASE : Self = Self::_X8664PltOrIa64PltReserveOrPpcGotOrC6000DsbtBaseOrXtensaGotLocOffOrLoprocOrPpc64GlinkOrAlphaPltro;
    pub const XTENSA_GOT_LOC_OFF : Self = Self::_X8664PltOrIa64PltReserveOrPpcGotOrC6000DsbtBaseOrXtensaGotLocOffOrLoprocOrPpc64GlinkOrAlphaPltro;
    pub const LOPROC : Self = Self::_X8664PltOrIa64PltReserveOrPpcGotOrC6000DsbtBaseOrXtensaGotLocOffOrLoprocOrPpc64GlinkOrAlphaPltro;
    pub const PPC64_GLINK : Self = Self::_X8664PltOrIa64PltReserveOrPpcGotOrC6000DsbtBaseOrXtensaGotLocOffOrLoprocOrPpc64GlinkOrAlphaPltro;
    pub const ALPHA_PLTRO : Self = Self::_X8664PltOrIa64PltReserveOrPpcGotOrC6000DsbtBaseOrXtensaGotLocOffOrLoprocOrPpc64GlinkOrAlphaPltro;
    pub const X86_64_PLTSZ : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    pub const RISCV_VARIANT_CC : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    pub const PPC_OPT : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    pub const PPC64_OPD : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    pub const AARCH64_BTI_PLT : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    pub const SCORE_BASE_ADDRESS : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    pub const MIPS_RLD_VERSION : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    pub const SPARC_REGISTER : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    pub const C6000_DSBT_SIZE : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    pub const XTENSA_GOT_LOC_SZ : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    pub const X86_64_PLTENT: Self =
        Self::_X8664PltentOrPpc64OptOrAArch64PacPltOrScoreSymtabnoOrMipsIchecksumOrC6000DsbtIndex;
    pub const PPC64_OPT: Self =
        Self::_X8664PltentOrPpc64OptOrAArch64PacPltOrScoreSymtabnoOrMipsIchecksumOrC6000DsbtIndex;
    pub const AARCH64_PAC_PLT: Self =
        Self::_X8664PltentOrPpc64OptOrAArch64PacPltOrScoreSymtabnoOrMipsIchecksumOrC6000DsbtIndex;
    pub const SCORE_SYMTABNO: Self =
        Self::_X8664PltentOrPpc64OptOrAArch64PacPltOrScoreSymtabnoOrMipsIchecksumOrC6000DsbtIndex;
    pub const MIPS_ICHECKSUM: Self =
        Self::_X8664PltentOrPpc64OptOrAArch64PacPltOrScoreSymtabnoOrMipsIchecksumOrC6000DsbtIndex;
    pub const C6000_DSBT_INDEX: Self =
        Self::_X8664PltentOrPpc64OptOrAArch64PacPltOrScoreSymtabnoOrMipsIchecksumOrC6000DsbtIndex;
    pub const SCORE_LOCAL_GOTNO: Self =
        Self::_ScoreLocalGotnoOrMipsTimeStampOrC6000PreemptmapOrNios2GpOrPpc64Opdsz;
    pub const MIPS_TIME_STAMP: Self =
        Self::_ScoreLocalGotnoOrMipsTimeStampOrC6000PreemptmapOrNios2GpOrPpc64Opdsz;
    pub const C6000_PREEMPTMAP: Self =
        Self::_ScoreLocalGotnoOrMipsTimeStampOrC6000PreemptmapOrNios2GpOrPpc64Opdsz;
    pub const NIOS2_GP: Self =
        Self::_ScoreLocalGotnoOrMipsTimeStampOrC6000PreemptmapOrNios2GpOrPpc64Opdsz;
    pub const PPC64_OPDSZ: Self =
        Self::_ScoreLocalGotnoOrMipsTimeStampOrC6000PreemptmapOrNios2GpOrPpc64Opdsz;
    pub const SCORE_GOTSYM: Self = Self::_ScoreGotsymOrMipsIversion;
    pub const MIPS_IVERSION: Self = Self::_ScoreGotsymOrMipsIversion;
    pub const SCORE_UNREFEXTNO: Self = Self::_ScoreUnrefextnoOrMipsFlagsOrAArch64VariantPcs;
    pub const MIPS_FLAGS: Self = Self::_ScoreUnrefextnoOrMipsFlagsOrAArch64VariantPcs;
    pub const AARCH64_VARIANT_PCS: Self = Self::_ScoreUnrefextnoOrMipsFlagsOrAArch64VariantPcs;
    pub const SCORE_HIPAGENO: Self = Self::_ScoreHipagenoOrMipsBaseAddress;
    pub const MIPS_BASE_ADDRESS: Self = Self::_ScoreHipagenoOrMipsBaseAddress;
    pub const IA_64_VMS_SUBTYPE: Self = Self::_Ia64VmsSubtypeOrHpEpltrelOrC6000GsymOffsetOrLoos;
    pub const HP_EPLTREL: Self = Self::_Ia64VmsSubtypeOrHpEpltrelOrC6000GsymOffsetOrLoos;
    pub const C6000_GSYM_OFFSET: Self = Self::_Ia64VmsSubtypeOrHpEpltrelOrC6000GsymOffsetOrLoos;
    pub const LOOS: Self = Self::_Ia64VmsSubtypeOrHpEpltrelOrC6000GsymOffsetOrLoos;
    pub const IA_64_VMS_IMGIOCNT: Self = Self::_Ia64VmsImgiocntOrHpFilteredOrC6000GstrOffset;
    pub const HP_FILTERED: Self = Self::_Ia64VmsImgiocntOrHpFilteredOrC6000GstrOffset;
    pub const C6000_GSTR_OFFSET: Self = Self::_Ia64VmsImgiocntOrHpFilteredOrC6000GstrOffset;
    pub const IA_64_VMS_LNKFLAGS: Self = Self::_Ia64VmsLnkflagsOrPltSizeOrVxWrsTlsDataAlign;
    pub const PLT_SIZE: Self = Self::_Ia64VmsLnkflagsOrPltSizeOrVxWrsTlsDataAlign;
    pub const VX_WRS_TLS_DATA_ALIGN: Self = Self::_Ia64VmsLnkflagsOrPltSizeOrVxWrsTlsDataAlign;
    pub const IA_64_VMS_VIR_MEM_BLK_SIZ: Self = Self::_Ia64VmsVirMemBlkSizOrDltSize;
    pub const DLT_SIZE: Self = Self::_Ia64VmsVirMemBlkSizOrDltSize;
    pub const HP_LOAD_MAP: Self = Self::_HpLoadMapOrOldLoos;
    pub const OLD_LOOS: Self = Self::_HpLoadMapOrOldLoos;
    pub const HP_FILTER_TLS: Self = Self::_HpFilterTlsOrVxWrsTlsDataStart;
    pub const VX_WRS_TLS_DATA_START: Self = Self::_HpFilterTlsOrVxWrsTlsDataStart;
    pub const HP_COMPAT_FILTERED: Self = Self::_HpCompatFilteredOrVxWrsTlsDataSize;
    pub const VX_WRS_TLS_DATA_SIZE: Self = Self::_HpCompatFilteredOrVxWrsTlsDataSize;
    pub const HP_LAZYLOAD: Self = Self::_HpLazyloadOrVxWrsTlsVarsStart;
    pub const VX_WRS_TLS_VARS_START: Self = Self::_HpLazyloadOrVxWrsTlsVarsStart;
    pub const HP_BIND_NOW_COUNT: Self = Self::_HpBindNowCountOrVxWrsTlsVarsSize;
    pub const VX_WRS_TLS_VARS_SIZE: Self = Self::_HpBindNowCountOrVxWrsTlsVarsSize;
    /// Map text private
    pub const HP_DEBUG_PRIVATE: Self = Self::_HpDebugPrivateOrNeededOrAlphaNumOrIa64Num;
    pub const NEEDED: Self = Self::_HpDebugPrivateOrNeededOrAlphaNumOrIa64Num;
    pub const ALPHA_NUM: Self = Self::_HpDebugPrivateOrNeededOrAlphaNumOrIa64Num;
    pub const IA_64_NUM: Self = Self::_HpDebugPrivateOrNeededOrAlphaNumOrIa64Num;
    /// Callback
    pub const HP_DEBUG_CALLBACK: Self = Self::_HpDebugCallbackOrPltrelszOrSparcNumOrPpcNum;
    pub const PLTRELSZ: Self = Self::_HpDebugCallbackOrPltrelszOrSparcNumOrPpcNum;
    pub const SPARC_NUM: Self = Self::_HpDebugCallbackOrPltrelszOrSparcNumOrPpcNum;
    pub const PPC_NUM: Self = Self::_HpDebugCallbackOrPltrelszOrSparcNumOrPpcNum;
    /// BOR callback
    pub const HP_DEBUG_CALLBACK_BOR: Self = Self::_HpDebugCallbackBorOrHashOrPpc64Num;
    pub const HASH: Self = Self::_HpDebugCallbackBorOrHashOrPpc64Num;
    pub const PPC64_NUM: Self = Self::_HpDebugCallbackBorOrHashOrPpc64Num;
    /// No env var
    pub const HP_NO_ENVVAR: Self = Self::_HpNoEnvvarOrRelasz;
    pub const RELASZ: Self = Self::_HpNoEnvvarOrRelasz;
    /// Bind now
    pub const HP_BIND_NOW: Self = Self::_HpBindNowOrSymbolicOrVersiontagnum;
    pub const SYMBOLIC: Self = Self::_HpBindNowOrSymbolicOrVersiontagnum;
    pub const VERSIONTAGNUM: Self = Self::_HpBindNowOrSymbolicOrVersiontagnum;
    /// Bind non-fatal
    pub const HP_BIND_NONFATAL: Self = Self::_HpBindNonfatalOrEncodingOrPreinitArray;
    pub const ENCODING: Self = Self::_HpBindNonfatalOrEncodingOrPreinitArray;
    pub const PREINIT_ARRAY: Self = Self::_HpBindNonfatalOrEncodingOrPreinitArray;
    pub const PLTGOT: Self = Self::_PltgotOrExtranum;
    pub const EXTRANUM: Self = Self::_PltgotOrExtranum;
    pub const SYMTAB: Self = Self::_SymtabOrAArch64Num;
    pub const AARCH64_NUM: Self = Self::_SymtabOrAArch64Num;
    pub const SYMENT: Self = Self::_SymentOrAddrnum;
    pub const ADDRNUM: Self = Self::_SymentOrAddrnum;
    pub const INIT: Self = Self::_InitOrValnum;
    pub const VALNUM: Self = Self::_InitOrValnum;
    pub const OLD_HIOS: Self = Self::_OldHiosOrVerneednum;
    pub const VERNEEDNUM: Self = Self::_OldHiosOrVerneednum;
    pub const HIPROC: Self = Self::_HiprocOrFilter;
    pub const FILTER: Self = Self::_HiprocOrFilter;
    pub const FEATURE: Self = Self::_FeatureOrFeature1;
    /// Feature selection (DTF_*).
    pub const FEATURE_1: Self = Self::_FeatureOrFeature1;
    pub const SYMINENT: Self = Self::_SyminentOrValrnghi;
    pub const VALRNGHI: Self = Self::_SyminentOrValrnghi;
    pub const SYMINFO: Self = Self::_SyminfoOrAddrrnghi;
    pub const ADDRRNGHI: Self = Self::_SyminfoOrAddrrnghi;
    /// Most used by any processor
    pub const PROCNUM: Self = Self::_ProcnumOrMipsNum;
    pub const MIPS_NUM: Self = Self::_ProcnumOrMipsNum;
}

pub enum NoteType {
    /// Object module name, version, and date/time Or HpCompiler Or Contains copy of prstatus struct Or Has a struct procinfo Or Spu Or Contains a version string Or GnuAbiTag Or NetbsdIdent Or OpenbsdIdent Or FreebsdAbiTag Or Force enable Mprotect
    _VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect =
        0x1,
    /// Language processor name Or HpCopyright Or Contains copy of fpregset struct Or Has auxv data Or Contains an architecture string Or Used by ld.so and kernel vDSO Or Prfpreg Or Force disable Mprotect
    _VmsLnmOrHpCopyrightOrFpregsetOrNetbsdcoreAuxvOrArchOrGnuHwcapOrPrfpregOrNetbsdPaxNomprotect =
        0x2,
    /// Source files Or HpVersion Or Contains copy of prpsinfo struct Or Stapsdt Or Generated by ld --build-id Or NetbsdPax
    _VmsSrcOrHpVersionOrPrpsinfoOrStapsdtOrGnuBuildIdOrNetbsdPax = 0x3,
    /// Title text Or HpSrcfileInfo Or Contains copy of task struct Or Contains GO buildid data Or Generated by gold Or Contains copy of prxregset struct Or Force enable Segvguard
    _VmsTitleOrHpSrcfileInfoOrTaskstructOrGoBuildidOrGnuGoldVersionOrPrxregOrNetbsdPaxGuard = 0x4,
    /// Entity ident consistency check Or HpLinker Or Generated by gcc Or NetbsdMarch Or String from sysinfo(SI_PLATFORM)
    _VmsEidcOrHpLinkerOrGnuPropertyType0OrNetbsdMarchOrPlatform = 0x5,
    /// Whole program floating-point mode Or HpInstrumented Or Contains copy of Elfxx_auxv_t
    _VmsFpmodeOrHpInstrumentedOrAuxv = 0x6,
    /// Date/time image was linked.
    VmsLinktime = 0x65,
    /// Image name string.
    VmsImgnam = 0x66,
    /// Image ident string.
    VmsImgid = 0x67,
    /// Linker ident string.
    VmsLinkid = 0x68,
    /// Image build ident string.
    VmsImgbid = 0x69,
    /// Global Symbol Table Name.
    VmsGstnam = 0x6a,
    /// Original setting of dynamic data.
    VmsOrigDyn = 0x6b,
    /// Date/time of last patch.
    VmsPatchtime = 0x6c,
    /// HpUxOptions Or Thread miscellaneous info Or Contains copy of gwindows struct
    _HpUxOptionsOrFreebsdThrmiscOrGwindows = 0x7,
    /// Contains a user_xfpregs_struct;
    Prxfpreg = 0x46e62b7f,
    /// PowerPC Altivec/VMX registers Or GnuBuildAttributeOpen
    _PpcVmxOrGnuBuildAttributeOpen = 0x100,
    /// PowerPC VSX registers
    PpcVsx = 0x102,
    /// PowerPC Target Address Register
    PpcTar = 0x103,
    /// PowerPC Program Priority Register
    PpcPpr = 0x104,
    /// PowerPC Data Stream Control Register
    PpcDscr = 0x105,
    /// PowerPC Event Based Branch Registers
    PpcEbb = 0x106,
    /// PowerPC Performance Monitor Registers
    PpcPmu = 0x107,
    /// PowerPC TM checkpointed GPR Registers
    PpcTmCgpr = 0x108,
    /// PowerPC TM checkpointed FPR Registers
    PpcTmCfpr = 0x109,
    /// PowerPC TM checkpointed VMX Registers
    PpcTmCvmx = 0x10a,
    /// PowerPC TM checkpointed VSX Registers
    PpcTmCvsx = 0x10b,
    /// PowerPC TM Special Purpose Registers
    PpcTmSpr = 0x10c,
    /// PowerPC TM checkpointed TAR
    PpcTmCtar = 0x10d,
    /// PowerPC TM checkpointed PPR
    PpcTmCppr = 0x10e,
    /// PowerPC TM checkpointed Data SCR
    PpcTmCdscr = 0x10f,
    /// x86 TLS information Or x86 segment base registers
    _I386TlsOrFreebsdX86Segbases = 0x200,
    /// x86 io permissions
    I386Ioperm = 0x201,
    /// x86 XSAVE extended state
    X86Xstate = 0x202,
    /// x86 CET state.
    X86Cet = 0x203,
    /// S/390 upper halves of GPRs
    S390HighGprs = 0x300,
    /// S390 timer
    S390Timer = 0x301,
    /// S390 TOD clock comparator
    S390Todcmp = 0x302,
    /// S390 TOD programmable register
    S390Todpreg = 0x303,
    /// S390 control registers
    S390Ctrs = 0x304,
    /// S390 prefix register
    S390Prefix = 0x305,
    /// S390 breaking event address
    S390LastBreak = 0x306,
    /// S390 system call restart data
    S390SystemCall = 0x307,
    /// S390 transaction diagnostic block
    S390Tdb = 0x308,
    /// S390 vector registers 0-15 upper half
    S390VxrsLow = 0x309,
    /// S390 vector registers 16-31
    S390VxrsHigh = 0x30a,
    /// s390 guarded storage registers
    S390GsCb = 0x30b,
    /// s390 guarded storage broadcast control block
    S390GsBc = 0x30c,
    /// ARM VFP registers
    ArmVfp = 0x400,
    /// AArch TLS registers
    ArmTls = 0x401,
    /// AArch hardware breakpoint registers
    ArmHwBreak = 0x402,
    /// AArch hardware watchpoint registers
    ArmHwWatch = 0x403,
    /// AArch ARM system call number
    ArmSystemCall = 0x404,
    /// AArch SVE registers.
    ArmSve = 0x405,
    /// AArch pointer authentication code masks
    ArmPacMask = 0x406,
    ArmPacaKeys = 0x407,
    ArmPacgKeys = 0x408,
    ArmTaggedAddrCtrl = 0x409,
    ArmPacEnabledKeys = 0x40a,
    /// AArch64 SME streaming SVE registers.
    ArmSsve = 0x40b,
    /// AArch64 SME ZA register.
    ArmZa = 0x40c,
    /// AArch64 SME2 ZT registers.
    ArmZt = 0x40d,
    /// ARC HS accumulator/extra registers.
    ArcV2 = 0x600,
    /// LoongArch CPU config registers
    LarchCpucfg = 0xa00,
    /// LoongArch Control State Registers
    LarchCsr = 0xa01,
    /// LoongArch SIMD eXtension registers
    LarchLsx = 0xa02,
    /// LoongArch Advanced SIMD eXtension registers
    LarchLasx = 0xa03,
    /// LoongArch Binary Translation registers
    LarchLbt = 0xa04,
    /// RISC-V Control and Status Registers
    RiscvCsr = 0x900,
    /// Fields of siginfo_t.
    Siginfo = 0x53494749,
    /// Description of mapped files.
    File = 0x46494c45,
    /// Contains copy of GDB's target description XML.
    GdbTdesc = 0xff000000,
    /// Has a struct pstatus Or Procstat vmmap data Or OpenbsdProcinfo
    _PstatusOrFreebsdProcstatVmmapOrOpenbsdProcinfo = 0xa,
    /// Has a struct fpregset Or Procstat umask data
    _FpregsOrFreebsdProcstatUmask = 0xc,
    /// Has a struct psinfo Or Procstat rlimit data
    _PsinfoOrFreebsdProcstatRlimit = 0xd,
    /// Has a struct lwpstatus_t Or Procstat auxv data Or Force enable ASLR
    _LwpstatusOrFreebsdProcstatAuxvOrNetbsdPaxAslr = 0x10,
    /// Has a struct lwpsinfo_t Or Thread ptrace miscellaneous info
    _LwpsinfoOrFreebsdPtlwpinfo = 0x11,
    /// Has a struct win32_pstatus
    Win32Pstatus = 0x12,
    /// Procstat proc data Or Contains copy of asrset struct Or Force disable Segvguard
    _FreebsdProcstatProcOrAsrsOrNetbsdPaxNoguard = 0x8,
    /// Procstat files data.
    FreebsdProcstatFiles = 0x9,
    /// Procstat groups data Or OpenbsdAuxv
    _FreebsdProcstatGroupsOrOpenbsdAuxv = 0xb,
    /// Procstat osreldate data Or Contains copy of prcred struct
    _FreebsdProcstatOsrelOrPrcred = 0xe,
    /// Procstat ps_strings data Or Contains copy of utsname struct
    _FreebsdProcstatPsstringsOrUtsname = 0xf,
    /// Has LWPSTATUS data
    NetbsdcoreLwpstatus = 0x18,
    /// start of machdep note types Or AmdgpuMetadata Or Force disable ASLR
    _NetbsdcoreFirstmachOrAmdgpuMetadataOrNetbsdPaxNoaslr = 0x20,
    /// OpenbsdRegs Or Contains copy of fprxregset struct
    _OpenbsdRegsOrPrfpxreg = 0x14,
    OpenbsdFpregs = 0x15,
    OpenbsdXfpregs = 0x16,
    OpenbsdWcookie = 0x17,
    /// GnuBuildAttributeFunc Or PowerPC SPE/EVR registers
    _GnuBuildAttributeFuncOrPpcSpe = 0x101,
    PpcPkey = 0x110,
    /// s390 runtime instrumentation.
    S390RiCb = 0x30d,
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
impl NoteType {
    /// Object module name, version, and date/time.
    pub const VMS_MHD : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    pub const HP_COMPILER : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    /// Contains copy of prstatus struct
    pub const PRSTATUS : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    /// Has a struct procinfo
    pub const NETBSDCORE_PROCINFO : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    pub const SPU : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    /// Contains a version string.
    pub const VERSION : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    pub const GNU_ABI_TAG : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    pub const NETBSD_IDENT : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    pub const OPENBSD_IDENT : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    pub const FREEBSD_ABI_TAG : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    /// Force enable Mprotect.
    pub const NETBSD_PAX_MPROTECT : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    /// Language processor name.
    pub const VMS_LNM : Self = Self::_VmsLnmOrHpCopyrightOrFpregsetOrNetbsdcoreAuxvOrArchOrGnuHwcapOrPrfpregOrNetbsdPaxNomprotect;
    pub const HP_COPYRIGHT : Self = Self::_VmsLnmOrHpCopyrightOrFpregsetOrNetbsdcoreAuxvOrArchOrGnuHwcapOrPrfpregOrNetbsdPaxNomprotect;
    /// Contains copy of fpregset struct
    pub const FPREGSET : Self = Self::_VmsLnmOrHpCopyrightOrFpregsetOrNetbsdcoreAuxvOrArchOrGnuHwcapOrPrfpregOrNetbsdPaxNomprotect;
    /// Has auxv data
    pub const NETBSDCORE_AUXV : Self = Self::_VmsLnmOrHpCopyrightOrFpregsetOrNetbsdcoreAuxvOrArchOrGnuHwcapOrPrfpregOrNetbsdPaxNomprotect;
    /// Contains an architecture string.
    pub const ARCH : Self = Self::_VmsLnmOrHpCopyrightOrFpregsetOrNetbsdcoreAuxvOrArchOrGnuHwcapOrPrfpregOrNetbsdPaxNomprotect;
    /// Used by ld.so and kernel vDSO.
    pub const GNU_HWCAP : Self = Self::_VmsLnmOrHpCopyrightOrFpregsetOrNetbsdcoreAuxvOrArchOrGnuHwcapOrPrfpregOrNetbsdPaxNomprotect;
    pub const PRFPREG : Self = Self::_VmsLnmOrHpCopyrightOrFpregsetOrNetbsdcoreAuxvOrArchOrGnuHwcapOrPrfpregOrNetbsdPaxNomprotect;
    /// Force disable Mprotect.
    pub const NETBSD_PAX_NOMPROTECT : Self = Self::_VmsLnmOrHpCopyrightOrFpregsetOrNetbsdcoreAuxvOrArchOrGnuHwcapOrPrfpregOrNetbsdPaxNomprotect;
    /// Source files.
    pub const VMS_SRC: Self = Self::_VmsSrcOrHpVersionOrPrpsinfoOrStapsdtOrGnuBuildIdOrNetbsdPax;
    pub const HP_VERSION: Self = Self::_VmsSrcOrHpVersionOrPrpsinfoOrStapsdtOrGnuBuildIdOrNetbsdPax;
    /// Contains copy of prpsinfo struct
    pub const PRPSINFO: Self = Self::_VmsSrcOrHpVersionOrPrpsinfoOrStapsdtOrGnuBuildIdOrNetbsdPax;
    pub const STAPSDT: Self = Self::_VmsSrcOrHpVersionOrPrpsinfoOrStapsdtOrGnuBuildIdOrNetbsdPax;
    /// Generated by ld --build-id.
    pub const GNU_BUILD_ID: Self =
        Self::_VmsSrcOrHpVersionOrPrpsinfoOrStapsdtOrGnuBuildIdOrNetbsdPax;
    pub const NETBSD_PAX: Self = Self::_VmsSrcOrHpVersionOrPrpsinfoOrStapsdtOrGnuBuildIdOrNetbsdPax;
    /// Title text.
    pub const VMS_TITLE : Self = Self::_VmsTitleOrHpSrcfileInfoOrTaskstructOrGoBuildidOrGnuGoldVersionOrPrxregOrNetbsdPaxGuard;
    pub const HP_SRCFILE_INFO : Self = Self::_VmsTitleOrHpSrcfileInfoOrTaskstructOrGoBuildidOrGnuGoldVersionOrPrxregOrNetbsdPaxGuard;
    /// Contains copy of task struct
    pub const TASKSTRUCT : Self = Self::_VmsTitleOrHpSrcfileInfoOrTaskstructOrGoBuildidOrGnuGoldVersionOrPrxregOrNetbsdPaxGuard;
    /// Contains GO buildid data.
    pub const GO_BUILDID : Self = Self::_VmsTitleOrHpSrcfileInfoOrTaskstructOrGoBuildidOrGnuGoldVersionOrPrxregOrNetbsdPaxGuard;
    /// Generated by gold.
    pub const GNU_GOLD_VERSION : Self = Self::_VmsTitleOrHpSrcfileInfoOrTaskstructOrGoBuildidOrGnuGoldVersionOrPrxregOrNetbsdPaxGuard;
    /// Contains copy of prxregset struct
    pub const PRXREG : Self = Self::_VmsTitleOrHpSrcfileInfoOrTaskstructOrGoBuildidOrGnuGoldVersionOrPrxregOrNetbsdPaxGuard;
    /// Force enable Segvguard.
    pub const NETBSD_PAX_GUARD : Self = Self::_VmsTitleOrHpSrcfileInfoOrTaskstructOrGoBuildidOrGnuGoldVersionOrPrxregOrNetbsdPaxGuard;
    /// Entity ident consistency check.
    pub const VMS_EIDC: Self = Self::_VmsEidcOrHpLinkerOrGnuPropertyType0OrNetbsdMarchOrPlatform;
    pub const HP_LINKER: Self = Self::_VmsEidcOrHpLinkerOrGnuPropertyType0OrNetbsdMarchOrPlatform;
    /// Generated by gcc.
    pub const GNU_PROPERTY_TYPE_0: Self =
        Self::_VmsEidcOrHpLinkerOrGnuPropertyType0OrNetbsdMarchOrPlatform;
    pub const NETBSD_MARCH: Self =
        Self::_VmsEidcOrHpLinkerOrGnuPropertyType0OrNetbsdMarchOrPlatform;
    /// String from sysinfo(SI_PLATFORM)
    pub const PLATFORM: Self = Self::_VmsEidcOrHpLinkerOrGnuPropertyType0OrNetbsdMarchOrPlatform;
    /// Whole program floating-point mode.
    pub const VMS_FPMODE: Self = Self::_VmsFpmodeOrHpInstrumentedOrAuxv;
    pub const HP_INSTRUMENTED: Self = Self::_VmsFpmodeOrHpInstrumentedOrAuxv;
    /// Contains copy of Elfxx_auxv_t
    pub const AUXV: Self = Self::_VmsFpmodeOrHpInstrumentedOrAuxv;
    pub const HP_UX_OPTIONS: Self = Self::_HpUxOptionsOrFreebsdThrmiscOrGwindows;
    /// Thread miscellaneous info.
    pub const FREEBSD_THRMISC: Self = Self::_HpUxOptionsOrFreebsdThrmiscOrGwindows;
    /// Contains copy of gwindows struct
    pub const GWINDOWS: Self = Self::_HpUxOptionsOrFreebsdThrmiscOrGwindows;
    /// PowerPC Altivec/VMX registers
    pub const PPC_VMX: Self = Self::_PpcVmxOrGnuBuildAttributeOpen;
    pub const GNU_BUILD_ATTRIBUTE_OPEN: Self = Self::_PpcVmxOrGnuBuildAttributeOpen;
    /// x86 TLS information
    pub const I386_TLS: Self = Self::_I386TlsOrFreebsdX86Segbases;
    /// x86 segment base registers
    pub const FREEBSD_X86_SEGBASES: Self = Self::_I386TlsOrFreebsdX86Segbases;
    /// Has a struct pstatus
    pub const PSTATUS: Self = Self::_PstatusOrFreebsdProcstatVmmapOrOpenbsdProcinfo;
    /// Procstat vmmap data.
    pub const FREEBSD_PROCSTAT_VMMAP: Self = Self::_PstatusOrFreebsdProcstatVmmapOrOpenbsdProcinfo;
    pub const OPENBSD_PROCINFO: Self = Self::_PstatusOrFreebsdProcstatVmmapOrOpenbsdProcinfo;
    /// Has a struct fpregset
    pub const FPREGS: Self = Self::_FpregsOrFreebsdProcstatUmask;
    /// Procstat umask data.
    pub const FREEBSD_PROCSTAT_UMASK: Self = Self::_FpregsOrFreebsdProcstatUmask;
    /// Has a struct psinfo
    pub const PSINFO: Self = Self::_PsinfoOrFreebsdProcstatRlimit;
    /// Procstat rlimit data.
    pub const FREEBSD_PROCSTAT_RLIMIT: Self = Self::_PsinfoOrFreebsdProcstatRlimit;
    /// Has a struct lwpstatus_t
    pub const LWPSTATUS: Self = Self::_LwpstatusOrFreebsdProcstatAuxvOrNetbsdPaxAslr;
    /// Procstat auxv data.
    pub const FREEBSD_PROCSTAT_AUXV: Self = Self::_LwpstatusOrFreebsdProcstatAuxvOrNetbsdPaxAslr;
    /// Force enable ASLR.
    pub const NETBSD_PAX_ASLR: Self = Self::_LwpstatusOrFreebsdProcstatAuxvOrNetbsdPaxAslr;
    /// Has a struct lwpsinfo_t
    pub const LWPSINFO: Self = Self::_LwpsinfoOrFreebsdPtlwpinfo;
    /// Thread ptrace miscellaneous info.
    pub const FREEBSD_PTLWPINFO: Self = Self::_LwpsinfoOrFreebsdPtlwpinfo;
    /// Procstat proc data.
    pub const FREEBSD_PROCSTAT_PROC: Self = Self::_FreebsdProcstatProcOrAsrsOrNetbsdPaxNoguard;
    /// Contains copy of asrset struct
    pub const ASRS: Self = Self::_FreebsdProcstatProcOrAsrsOrNetbsdPaxNoguard;
    /// Force disable Segvguard.
    pub const NETBSD_PAX_NOGUARD: Self = Self::_FreebsdProcstatProcOrAsrsOrNetbsdPaxNoguard;
    /// Procstat groups data.
    pub const FREEBSD_PROCSTAT_GROUPS: Self = Self::_FreebsdProcstatGroupsOrOpenbsdAuxv;
    pub const OPENBSD_AUXV: Self = Self::_FreebsdProcstatGroupsOrOpenbsdAuxv;
    /// Procstat osreldate data.
    pub const FREEBSD_PROCSTAT_OSREL: Self = Self::_FreebsdProcstatOsrelOrPrcred;
    /// Contains copy of prcred struct
    pub const PRCRED: Self = Self::_FreebsdProcstatOsrelOrPrcred;
    /// Procstat ps_strings data.
    pub const FREEBSD_PROCSTAT_PSSTRINGS: Self = Self::_FreebsdProcstatPsstringsOrUtsname;
    /// Contains copy of utsname struct
    pub const UTSNAME: Self = Self::_FreebsdProcstatPsstringsOrUtsname;
    /// start of machdep note types
    pub const NETBSDCORE_FIRSTMACH: Self =
        Self::_NetbsdcoreFirstmachOrAmdgpuMetadataOrNetbsdPaxNoaslr;
    pub const AMDGPU_METADATA: Self = Self::_NetbsdcoreFirstmachOrAmdgpuMetadataOrNetbsdPaxNoaslr;
    /// Force disable ASLR.
    pub const NETBSD_PAX_NOASLR: Self = Self::_NetbsdcoreFirstmachOrAmdgpuMetadataOrNetbsdPaxNoaslr;
    pub const OPENBSD_REGS: Self = Self::_OpenbsdRegsOrPrfpxreg;
    /// Contains copy of fprxregset struct
    pub const PRFPXREG: Self = Self::_OpenbsdRegsOrPrfpxreg;
    pub const GNU_BUILD_ATTRIBUTE_FUNC: Self = Self::_GnuBuildAttributeFuncOrPpcSpe;
    /// PowerPC SPE/EVR registers
    pub const PPC_SPE: Self = Self::_GnuBuildAttributeFuncOrPpcSpe;
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ElfFlags: u32 {
        const XGATE_ABI = 0xf;
        const M68HC11_ABI = 0xf;
        /// Which ISA
        const M68K_CF_ISA_MASK = 0xf;
        /// OS-specific flags.
        const IA_64_MASKOS = 0xf;
        const PICOJAVA_ARCH = 0xf;
        const BPF_CPUVER = 0xf;
        const XTENSA_MACH = 0xf;
        const M32R_IGNORE = 0xf;
        const NDS32_ELF_VERSION = 0xf;
        const XGATE_MACH_MASK = 0xf0;
        const M68HC11_MACH_MASK = 0xf0;
        const NDS_ABI = 0xf0;
        /// XGATE microcontroller.
        const XGATE_MACH = 0x80;
        const ARM_NEW_ABI = 0x80;
        const AVR_LINKRELAX_PREPARED = 0x80;
        /// -mmedia
        const FRV_MEDIA = 0x80;
        const MIPS_OPTIONS_FIRST = 0x80;
        /// And no function descriptors.
        const IA_64_NOFUNCDESC_CONS_GP = 0x80;
        /// mask for # of gprs
        const FRV_GPR_MASK = 0x3;
        /// specific cpu bits
        const IQ2000_CPU_MASK = 0x3;
        /// MS2
        const MT_CPU_MS2 = 0x3;
        /// specific cpu bits
        const MT_CPU_MASK = 0x3;
        const M68K_CF_ISA_A_PLUS = 0x3;
        /// Completion code.
        const IA_64_VMS_COMCOD = 0x3;
        const Z80_MACH_R800 = 0x3;
        const IA_64_VMS_COMCOD_ABORT = 0x3;
        const PPC64_ABI = 0x3;
        const SH3 = 0x3;
        const IQ2000_ALL_FLAGS = 0x3;
        const BFIN_PIC_FLAGS = 0x3;
        /// memory model mask
        const SPARCV9_MM = 0x3;
        const LOONGARCH_ABI_DOUBLE_FLOAT = 0x3;
        const MT_ALL_FLAGS = 0x3;
        /// -mgpr-32
        const FRV_GPR_32 = 0x1;
        /// sizeof(double) == 8.
        const RH850_FPU_DOUBLE = 0x1;
        const S390_HIGH_GPRS = 0x1;
        const MIPS_NOREORDER = 0x1;
        /// default
        const IQ2000_CPU_IQ2000 = 0x1;
        /// -fpic
        const BFIN_PIC = 0x1;
        const NIOS2_ARCH_R2 = 0x1;
        const LM32_MACH = 0x1;
        /// default
        const MT_CPU_MRISC = 0x1;
        const ALPHA_32BIT = 0x1;
        const CRIS_UNDERSCORE = 0x1;
        /// Object contains non-PIC code
        const VAX_NONPIC = 0x1;
        /// Aligned to 4-byte bounadries.
        const RH850_DATA_ALIGN4 = 0x1;
        /// 32-bits in size.
        const RH850_DOUBLE32 = 0x1;
        /// Set if [N]]M{ADD|SUB}F.S are used.
        const RH850_FPU20 = 0x1;
        const RH850_SIMD = 0x1;
        const RH850_CACHE = 0x1;
        const RH850_MMU = 0x1;
        const RISCV_RVC = 0x1;
        const VISIUM_ARCH_MCM = 0x1;
        /// ISA A except for div
        const M68K_CF_ISA_A_NODIV = 0x1;
        const Z80_MACH_Z80 = 0x1;
        const ARM_RELEXEC = 0x1;
        /// Trap NIL pointer dereferences.
        const IA_64_TRAPNIL = 0x1;
        const IA_64_VMS_COMCOD_WARNING = 0x1;
        const SH1 = 0x1;
        const OR1K_NODELAY = 0x1;
        /// partial store ordering
        const SPARCV9_PSO = 0x1;
        const LOONGARCH_ABI_SOFT_FLOAT = 0x1;
        const C6000_REL = 0x1;
        /// -mgpr-64
        const FRV_GPR_64 = 0x2;
        /// sizeof(double) == 4.
        const RH850_FPU_SINGLE = 0x2;
        const MIPS_PIC = 0x2;
        /// IQ10
        const IQ2000_CPU_IQ10 = 0x2;
        /// -mfdpic
        const BFIN_FDPIC = 0x2;
        /// MRISC2
        const MT_CPU_MRISC2 = 0x2;
        const ALPHA_CANRELAX = 0x2;
        const CRIS_VARIANT_V32 = 0x2;
        /// Aligned to 8-byte bounadries.
        const RH850_DATA_ALIGN8 = 0x2;
        /// 64-bits in size.
        const RH850_DOUBLE64 = 0x2;
        /// Set if ADSF.D or ADDF.D is used.
        const RH850_FPU30 = 0x2;
        const RISCV_FLOAT_ABI_SINGLE = 0x2;
        const VISIUM_ARCH_MCM24 = 0x2;
        const M68K_CF_ISA_A = 0x2;
        const Z80_MACH_Z180 = 0x2;
        const ARM_HASENTRY = 0x2;
        const IA_64_VMS_COMCOD_ERROR = 0x2;
        const SH2 = 0x2;
        /// relaxed store ordering
        const SPARCV9_RMO = 0x2;
        const LOONGARCH_ABI_SINGLE_FLOAT = 0x2;
        /// mask for # of fprs
        const FRV_FPR_MASK = 0xc;
        /// -msoft-float
        const FRV_FPR_NONE = 0xc;
        const SH4A = 0xc;
        /// -mfpr-32
        const FRV_FPR_32 = 0x4;
        const MIPS_CPIC = 0x4;
        const CRIS_VARIANT_COMMON_V10_V32 = 0x4;
        const RISCV_FLOAT_ABI_DOUBLE = 0x4;
        const VISIUM_ARCH_GR6 = 0x4;
        /// ISA_B except for USP
        const M68K_CF_ISA_B_NOUSP = 0x4;
        /// Contains VMS linkages info.
        const IA_64_VMS_LINKAGES = 0x4;
        const Z80_MACH_EZ80_Z80 = 0x4;
        const ARM_INTERWORK = 0x4;
        /// NB conflicts with EF_INTERWORK.
        const ARM_SYMSARESORTED = 0x4;
        /// Program uses arch. extensions.
        const IA_64_EXT = 0x4;
        const NDS_ABI_SHIFT = 0x4;
        const SH_DSP = 0x4;
        /// -mfpr-64
        const FRV_FPR_64 = 0x8;
        const MIPS_XGOT = 0x8;
        const RISCV_RVE = 0x8;
        /// PSR BE bit set (big-endian).
        const IA_64_BE = 0x8;
        const ARM_APCS_26 = 0x8;
        /// NB conflicts with EF_APCS26.
        const ARM_DYNSYMSUSESEGIDX = 0x8;
        const SH3E = 0x8;
        /// mask for dword support
        const FRV_DWORD_MASK = 0x30;
        const M68K_CF_MAC_MASK = 0x30;
        /// EMAC_B
        const M68K_CF_EMAC_B = 0x30;
        const AMDGPU_MACH_AMDGCN_GFX908 = 0x30;
        /// use double word insns
        const FRV_DWORD_YES = 0x10;
        const MIPS_UCODE = 0x10;
        const PICOJAVA_NEWCALLS = 0x10;
        /// --code-in-l1
        const BFIN_CODE_IN_L1 = 0x10;
        const RISCV_TSO = 0x10;
        /// MAC
        const M68K_CF_MAC = 0x10;
        /// 68HC12 microcontroller.
        const M68HC12_MACH = 0x10;
        const ARM_APCS_FLOAT = 0x10;
        /// NB conflicts with EF_APCS_FLOAT.
        const ARM_MAPSYMSFIRST = 0x10;
        /// 64-bit ABI.
        const IA_64_ABI64 = 0x10;
        const SH4_NOFPU = 0x10;
        const MIPS_64BIT_WHIRL = 0x10;
        /// don't use double word insn
        const FRV_DWORD_NO = 0x20;
        /// Registers r15-r24 (inclusive) are not used.
        const RH850_REGMODE22 = 0x20;
        const MIPS_ABI2 = 0x20;
        /// The (currently) non standard GNU calling convention
        const PICOJAVA_GNUCALLS = 0x20;
        /// --data-in-l1
        const BFIN_DATA_IN_L1 = 0x20;
        /// EMAC
        const M68K_CF_EMAC = 0x20;
        /// 68HCS12 microcontroller.
        const M68HCS12_MACH = 0x20;
        const ARM_PIC = 0x20;
        /// Only FP6-FP11 used.
        const IA_64_REDUCEDFP = 0x20;
        const AMDGPU_MACH_AMDGCN_MIN = 0x20;
        const AMDGPU_MACH_AMDGCN_GFX600 = 0x20;
        /// -mdouble
        const FRV_DOUBLE = 0x40;
        const RH850_REGMODE32 = 0x40;
        /// Has float insns
        const M68K_CF_FLOAT = 0x40;
        const LOONGARCH_OBJABI_V1 = 0x40;
        /// 8-bit structure alignment is in use.
        const ARM_ALIGN8 = 0x40;
        const MIPS_DYNAMIC = 0x40;
        /// gp as program wide constant.
        const IA_64_CONS_GP = 0x40;
        const AMDGPU_MACH_AMDGCN_GFX940 = 0x40;
        const MIPS_ABI_ON32 = 0x40;
        /// -fpic
        const FRV_PIC = 0x100;
        /// r4 is fixed.
        const RH850_GP_FIX = 0x100;
        const MIPS_32BITMODE = 0x100;
        /// Built as a library
        const MEP_LIBRARY = 0x100;
        const ARC_PIC = 0x100;
        const XTENSA_XT_INSN = 0x100;
        /// Object contains D-Float insn.
        const VAX_DFLOAT = 0x100;
        /// Load at absolute addresses.
        const IA_64_ABSOLUTE = 0x100;
        /// generic V8+ features
        const SPARC_32PLUS = 0x100;
        const ARM_OLD_ABI = 0x100;
        const SH_PIC = 0x100;
        const AMDGPU_FEATURE_XNACK_V3 = 0x100;
        const AMDGPU_FEATURE_XNACK_ANY_V4 = 0x100;
        /// used non pic safe relocs
        const FRV_NON_PIC_RELOCS = 0x200;
        /// r4 is callee save.
        const RH850_GP_NOFIX = 0x200;
        const MIPS_FP64 = 0x200;
        const XTENSA_XT_LIT = 0x200;
        /// Object contains G-Float insn.
        const VAX_GFLOAT = 0x200;
        /// Sun UltraSPARC1 extensions
        const SPARC_SUN_US1 = 0x200;
        const ARM_SOFT_FLOAT = 0x200;
        /// NB conflicts with EF_ARM_SOFT_FLOAT.
        const ARM_ABI_FLOAT_SOFT = 0x200;
        const AMDGPU_FEATURE_SRAMECC_V3 = 0x200;
        const AMDGPU_FEATURE_XNACK_OFF_V4 = 0x200;
        /// -mmuladd
        const FRV_MULADD = 0x400;
        /// r30 is fixed.
        const RH850_EP_FIX = 0x400;
        const MIPS_NAN2008 = 0x400;
        /// HAL R1 extensions
        const SPARC_HAL_R1 = 0x400;
        const ARM_VFP_FLOAT = 0x400;
        /// NB conflicts with EF_ARM_VFP_FLOAT.
        const ARM_ABI_FLOAT_HARD = 0x400;
        const AMDGPU_FEATURE_SRAMECC_ANY_V4 = 0x400;
        /// -fPIC
        const FRV_BIGPIC = 0x800;
        /// r30 is callee save.
        const RH850_EP_NOFIX = 0x800;
        /// Sun UltraSPARCIII extensions
        const SPARC_SUN_US3 = 0x800;
        const ARM_MAVERICK_FLOAT = 0x800;
        const AMDGPU_FEATURE_SRAMECC_OFF_V4 = 0x800;
        /// -mlibrary-pic
        const FRV_LIBPIC = 0x1000;
        /// r5 is fixed.
        const RH850_TP_FIX = 0x1000;
        /// -G 0, no small data ptr
        const FRV_G0 = 0x2000;
        /// r5 is callee save.
        const RH850_TP_NOFIX = 0x2000;
        /// -mnopack
        const FRV_NOPACK = 0x4000;
        /// r2 is fixed.
        const RH850_REG2_RESERVE = 0x4000;
        /// -mfdpic
        const FRV_FDPIC = 0x8000;
        /// r2 is callee saved.
        const RH850_REG2_NORESERVE = 0x8000;
        const M68K_CFV4E = 0x8000;
        /// i370 -mrelocatable-lib flag
        const I370_RELOCATABLE_LIB = 0x8000;
        /// PowerPC -mrelocatable-lib flag.
        const PPC_RELOCATABLE_LIB = 0x8000;
        /// Uses the FDPIC ABI.
        const SH_FDPIC = 0x8000;
        /// specific cpu bits
        const FRV_CPU_MASK = 0xff000000;
        /// Arch. version mask.
        const IA_64_ARCH = 0xff000000;
        /// specific cpu bits
        const MEP_CPU_MASK = 0xff000000;
        const ARM_EABIMASK = 0xff000000;
        /// generic FRV
        const FRV_CPU_GENERIC = 0x0;
        /// generic MEP
        const MEP_CPU_MEP = 0x0;
        const MEP_COP_NONE = 0x0;
        const ARM_EABI_UNKNOWN = 0x0;
        const NIOS2_ARCH_R1 = 0x0;
        const CRIS_VARIANT_ANY_V0_V10 = 0x0;
        /// -mips1 code.
        const MIPS_ARCH_1 = 0x0;
        const RISCV_FLOAT_ABI_SOFT = 0x0;
        const IA_64_VMS_COMCOD_SUCCESS = 0x0;
        const NDS32_ELF_VERSION_SHIFT = 0x0;
        /// For backwards compatibility.
        const SH_UNKNOWN = 0x0;
        /// Generic 68HC12/backward compatibility.
        const M68HC11_GENERIC = 0x0;
        /// total store ordering
        const SPARCV9_TSO = 0x0;
        const AMDGPU_FEATURE_XNACK_UNSUPPORTED_V4 = 0x0;
        const AMDGPU_FEATURE_SRAMECC_UNSUPPORTED_V4 = 0x0;
        /// FRV500
        const FRV_CPU_FR500 = 0x1000000;
        const M68K_M68000 = 0x1000000;
        /// MEP c2
        const MEP_CPU_C2 = 0x1000000;
        const ARM_EABI_VER1 = 0x1000000;
        /// Arch. version level 1 compat.
        const IA_64_ARCHVER_1 = 0x1000000;
        /// FRV300
        const FRV_CPU_FR300 = 0x2000000;
        const M68K_FIDO = 0x2000000;
        const MIPS_ARCH_ASE_MICROMIPS = 0x2000000;
        /// MEP c3
        const MEP_CPU_C3 = 0x2000000;
        const ARM_EABI_VER2 = 0x2000000;
        /// SIMPLE
        const FRV_CPU_SIMPLE = 0x3000000;
        const ARM_EABI_VER3 = 0x3000000;
        /// Tomcat, FR500 prototype
        const FRV_CPU_TOMCAT = 0x4000000;
        const MIPS_ARCH_ASE_M16 = 0x4000000;
        /// MEP c4
        const MEP_CPU_C4 = 0x4000000;
        const ARM_EABI_VER4 = 0x4000000;
        /// FRV400
        const FRV_CPU_FR400 = 0x5000000;
        const ARM_EABI_VER5 = 0x5000000;
        /// FRV550
        const FRV_CPU_FR550 = 0x6000000;
        const FRV_CPU_FR405 = 0x7000000;
        const FRV_CPU_FR450 = 0x8000000;
        const MIPS_ARCH_ASE_MDMX = 0x8000000;
        /// MEP c5
        const MEP_CPU_C5 = 0x8000000;
        const FRV_PIC_FLAGS = 0x9900;
        const FRV_ALL_FLAGS = 0xff00ffff;
        const MN10300_MACH = 0xff0000;
        const MIPS_MACH = 0xff0000;
        const H8_MACH = 0xff0000;
        const MEP_COP_MASK = 0xff0000;
        const V850_ARCH = 0xf0000000;
        const MIPS_ARCH = 0xf0000000;
        const RH850_ABI = 0xf0000000;
        const NDS_ARCH = 0xf0000000;
        const CSKY_ABIMASK = 0xf0000000;
        const V800_850E3 = 0x100000;
        const PARISC_NO_KABP = 0x100000;
        const RISCV_FLOAT_ABI = 0x6;
        const RISCV_FLOAT_ABI_QUAD = 0x6;
        const M68K_CF_ISA_C = 0x6;
        const Z80_MACH_Z80N = 0x6;
        const SH4AL_DSP = 0x6;
        const ARC_CPU_ARCV2HS = 0x6;
        const M68K_CPU32 = 0x810000;
        const CPU32 = 0x810000;
        const M68K_ARCH_MASK = 0x3818000;
        const M68K_CF_ISA_B = 0x5;
        const Z80_MACH_GBZ80 = 0x5;
        const SH3_DSP = 0x5;
        const ARC_CPU_ARCV2EM = 0x5;
        /// ISA C except for div
        const M68K_CF_ISA_C_NODIV = 0x7;
        const LOONGARCH_ABI_MODIFIER_MASK = 0x7;
        const M68K_CF_MASK = 0xff;
        /// Configuration index
        const MEP_INDEX_MASK = 0xff;
        const ARC_MACH_MSK = 0xff;
        const MSP430_MACH = 0xff;
        const Z80_MACH_MSK = 0xff;
        const AMDGPU_MACH = 0xff;
        /// i370 -mrelocatable flag
        const I370_RELOCATABLE = 0x10000;
        const PARISC_TRAPNIL = 0x10000;
        /// PowerPC -mrelocatable flag.
        const PPC_RELOCATABLE = 0x10000;
        const MEP_COP_AVC = 0x10000;
        const SCORE_MACH = 0xffff0000;
        const OMIT_PIC_FIXDD = 0xfff0000;
        const M32R_INST = 0xfff0000;
        const CSKY_OTHER = 0xfff0000;
        const SCORE_PIC = 0x80000000;
        /// PowerPC embedded flag.
        const PPC_EMB = 0x80000000;
        /// MIPS64r2 code.
        const MIPS_ARCH_64R2 = 0x80000000;
        const SCORE_FIXDEP = 0x40000000;
        /// -mips5 code.
        const MIPS_ARCH_5 = 0x40000000;
        const PARISC_EXT = 0x20000;
        const MEP_COP_AVC2 = 0x20000;
        const PARISC_LSB = 0x40000;
        const PARISC_WIDE = 0x80000;
        const PARISC_LAZYSWAP = 0x400000;
        const ARM_LE8 = 0x400000;
        const PARISC_ARCH = 0xffff;
        const CSKY_PROCESSOR = 0xffff;
        const M32R_ARCH = 0x30000000;
        /// -mips4 code.
        const MIPS_ARCH_4 = 0x30000000;
        const MIPS_ARCH_ASE = 0xf000000;
        const MIPS_ABI = 0xf000;
        /// MEP h1
        const MEP_CPU_H1 = 0x10000000;
        /// -mips2 code.
        const MIPS_ARCH_2 = 0x10000000;
        const CSKY_ABIV1 = 0x10000000;
        const MEP_COP_FMAX = 0x30000;
        const MEP_COP_IVC2 = 0x60000;
        const MEP_ALL_FLAGS = 0xffff01ff;
        /// FIXME: correct value?
        const RL78_CPU_RL78 = 0x79;
        /// FIXME: this collides with the E_FLAG_RX_... values below.
        const RX_CPU_RX = 0x79;
        /// specific cpu bits.
        const RL78_CPU_MASK = 0x7f;
        /// specific cpu bits
        const M32C_CPU_MASK = 0x7f;
        const RL78_ALL_FLAGS = 0x7f;
        const AVR_MACH = 0x7f;
        const M32C_ALL_FLAGS = 0x7f;
        const Z80_MACH_EZ80_ADL = 0x84;
        /// bits indicating V8+ type
        const SPARC_32PLUS_MASK = 0xffff00;
        /// reserved for vendor extensions
        const SPARC_EXT_MASK = 0xffff00;
        /// little endian data
        const SPARC_LEDATA = 0x800000;
        const ARM_BE8 = 0x800000;
        const LOONGARCH_OBJABI_MASK = 0xc0;
        const LOONGARCH_ABI_MASK = 0xc7;
        /// default
        const M32C_CPU_M16C = 0x75;
        /// m32c
        const M32C_CPU_M32C = 0x78;
        const NDS_ARCH_SHIFT = 0x1c;
        const NDS_INST = 0xfffff00;
        const SH_MACH_MASK = 0x1f;
        const SH4 = 0x9;
        const SH2E = 0xb;
        const SH2A = 0xd;
        const SH4A_NOFPU = 0x11;
        const SH4_NOMMU_NOFPU = 0x12;
        const SH2A_NOFPU = 0x13;
        const SH3_NOMMU = 0x14;
        const SH2A_SH4_NOFPU = 0x15;
        const SH2A_SH3_NOFPU = 0x16;
        const SH2A_SH4 = 0x17;
        const SH2A_SH3E = 0x18;
        const SH5 = 0xa;
        const ARC_OSABI_MSK = 0xf00;
        const ARC_ALL_MSK = 0xfff;
        /// specific cpu bits.
        const RX_CPU_MASK = 0x3ff;
        const RX_ALL_FLAGS = 0x3ff;
        const AMDGPU_MACH_AMDGCN_GFX601 = 0x21;
        const AMDGPU_MACH_AMDGCN_GFX700 = 0x22;
        const AMDGPU_MACH_AMDGCN_GFX701 = 0x23;
        const AMDGPU_MACH_AMDGCN_GFX702 = 0x24;
        const AMDGPU_MACH_AMDGCN_GFX703 = 0x25;
        const AMDGPU_MACH_AMDGCN_GFX704 = 0x26;
        const AMDGPU_MACH_AMDGCN_GFX801 = 0x28;
        const AMDGPU_MACH_AMDGCN_GFX802 = 0x29;
        const AMDGPU_MACH_AMDGCN_GFX803 = 0x2a;
        const AMDGPU_MACH_AMDGCN_GFX810 = 0x2b;
        const AMDGPU_MACH_AMDGCN_GFX900 = 0x2c;
        const AMDGPU_MACH_AMDGCN_GFX902 = 0x2d;
        const AMDGPU_MACH_AMDGCN_GFX904 = 0x2e;
        const AMDGPU_MACH_AMDGCN_GFX906 = 0x2f;
        const AMDGPU_MACH_AMDGCN_GFX909 = 0x31;
        const AMDGPU_MACH_AMDGCN_GFX90C = 0x32;
        const AMDGPU_MACH_AMDGCN_GFX1010 = 0x33;
        const AMDGPU_MACH_AMDGCN_GFX1011 = 0x34;
        const AMDGPU_MACH_AMDGCN_GFX1012 = 0x35;
        const AMDGPU_MACH_AMDGCN_GFX1030 = 0x36;
        const AMDGPU_MACH_AMDGCN_GFX1031 = 0x37;
        const AMDGPU_MACH_AMDGCN_GFX1032 = 0x38;
        const AMDGPU_MACH_AMDGCN_GFX1033 = 0x39;
        const AMDGPU_MACH_AMDGCN_GFX602 = 0x3a;
        const AMDGPU_MACH_AMDGCN_GFX705 = 0x3b;
        const AMDGPU_MACH_AMDGCN_GFX805 = 0x3c;
        const AMDGPU_MACH_AMDGCN_GFX1035 = 0x3d;
        const AMDGPU_MACH_AMDGCN_GFX1034 = 0x3e;
        const AMDGPU_MACH_AMDGCN_GFX90A = 0x3f;
        const AMDGPU_MACH_AMDGCN_GFX1013 = 0x42;
        const AMDGPU_MACH_AMDGCN_GFX1036 = 0x45;
        const AMDGPU_FEATURE_XNACK_V4 = 0x300;
        const AMDGPU_FEATURE_XNACK_ON_V4 = 0x300;
        const AMDGPU_FEATURE_SRAMECC_V4 = 0xc00;
        const AMDGPU_FEATURE_SRAMECC_ON_V4 = 0xc00;
        const CRIS_VARIANT_MASK = 0xe;
        /// -mips3 code.
        const MIPS_ARCH_3 = 0x20000000;
        const CSKY_ABIV2 = 0x20000000;
        /// MIPS32 code.
        const MIPS_ARCH_32 = 0x50000000;
        /// MIPS64 code.
        const MIPS_ARCH_64 = 0x60000000;
        /// MIPS32r2 code.
        const MIPS_ARCH_32R2 = 0x70000000;
    }
}
impl_binary_serde_for_bitflags_ty! {ElfFlags}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum RelocationType {
    /// FooNone Or PowerpcNone Or MipsNone Or TilegxNone Or S390None Or X8664None Or SparcNone Or ArmNone Or I386None Or AArch64None Or No reloc Or No reloc Or No reloc Or PpcNone Or no reloc Or ShNone Or CrisNone Or No reloc Or No reloc Or No reloc Or No reloc Or No reloc Or RiscvNone Or No reloc Or MetagHiaddr16 Or Nds32None Or Or1KNone Or Ppc64None Or none Or ArcNone
    _FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone =
        0x0,
    /// Foo32 Or PowerpcAddr32 Or Mips16 Or Tilegx64 Or S3908 Or X866464 Or Sparc8 Or ArmPc24 Or I38632 Or Direct 32 bit Or Direct 32-bit reference Or Direct 32 bit Or 32bit absolute address Or Direct 32 bit Or direct 32 bit (S + A) Or ShDir32 Or Cris8 Or Direct 32 bit Or Direct 16 bit Or Direct 32 bit Or Direct signed 16 bit Or Direct 32 bit Or Riscv32 Or Bpf6464 Or MetagLoaddr16 Or Or1K32 Or 32bit absolute address Or Arc8
    _Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8 =
        0x1,
    /// FooIllegal Or PowerpcAddr14Brntaken Or TilegxHw0 Or S390Copy Or MipsGot16 Or X8664Gotpcrel Or SparcHi22 Or ArmSbrel32 Or I386Gotoff Or 8 bit PC relative GOT entry Or 32-bit rel. address Or PC relative 16 bit Or PpcAddr14Brntaken Or 32 bit adjust program base(B + A) Or ShDir8L Or CrisCopy Or Direct 24 bit Or Low 16 bit Or No reloc Or High 16 bit Or High 16 bit, adjusted Or RiscvTlsDtprel64 Or MetagReg16Op1 Or Or1K32Pcrel Or Ppc64Addr14Brntaken Or ArcN16
    _FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16 =
        0x9,
    /// PariscTlsLe21L Or MicromipsCallLo16 Or TP-rel. address, left 21 bits Or @ltoff(@tprel(s+a)), imm2
    _PariscTlsLe21LOrMicromipsCallLo16OrPariscTprel21LOrIa64LtoffTprel22 = 0x9a,
    /// PariscTlsLe14R Or TP-rel. address, right 14 bits
    _PariscTlsLe14ROrPariscTprel14R = 0x9e,
    /// PariscTlsIe21L Or MicromipsTlsGd Or LT-TP-rel. address, left 21 bits Or ShCopy
    _PariscTlsIe21LOrMicromipsTlsGdOrPariscLtoffTp21LOrShCopy = 0xa2,
    /// PariscTlsIe14R Or MicromipsTlsGottprel Or LT-TP-rel. address, right 14 bits Or ShGotoff Or @dtpmod(sym + add), data8 MSB
    _PariscTlsIe14ROrMicromipsTlsGottprelOrPariscLtoffTp14ROrShGotoffOrIa64Dtpmod64Msb = 0xa6,
    /// PariscTlsTprel32 Or MicromipsCallHi16 Or 32 bits TP-rel. address
    _PariscTlsTprel32OrMicromipsCallHi16OrPariscTprel32 = 0x99,
    /// PariscTlsTprel64 Or PpcVleRel8 Or 64 bits TP-rel. address
    _PariscTlsTprel64OrPpcVleRel8OrPariscTprel64 = 0xd8,
    /// S39012 Or PowerpcAddr24 Or Tilegx32 Or Mips32 Or X8664Pc32 Or Sparc16 Or ArmAbs32 Or I386Pc32 Or Direct 16 bit Or Left 21 bits of eff. address Or Direct 64 bit Or 26bit address, 2 bits ignored Or disp ((S + A - P) >> 2) & 0xff Or ShRel32 Or Cris16 Or Direct 16 bit Or Direct 32 bit Or PC relative 32 bit Or Direct unsigned 16 bit Or Direct 16 bit Or Riscv64 Or 32bit absolute address Or Or1K16 Or 26bit address, word aligned Or Arc16
    _S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16 =
        0x2,
    /// S39016 Or PowerpcAddr16 Or Tilegx16 Or MipsRel32 Or X8664Got32 Or Sparc32 Or ArmRel32 Or I386Got32 Or Direct 8 bit Or Right 17 bits of eff. address Or GP relative 32 bit Or 16bit absolute address Or disp ((S + A - P) >> 1) & 0x7ff Or ShDir8Wpn Or Cris32 Or Direct 8 bit Or Direct 24 bit Or PC relative 64 bit Or PC relative 16 bit Or Direct 8 bit Or RiscvRelative Or No reloc Or Or1K8 Or 16bit absolute address Or Arc24
    _S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24 =
        0x3,
    /// S39032 Or PowerpcAddr16Lo Or Mips26 Or Tilegx8 Or X8664Plt32 Or SparcDisp8 Or ArmLdrPcG0 Or I386Plt32 Or PC relative 32 bit Or 17 bits of eff. address Or GP relative 16 bit w/optimization Or lower 16bit of absolute address Or ArmPc13 Or ShInd12W Or Cris8Pcrel Or PC-relative 32-bit Or PC relative 10 bit shifted Or Low 16 bits of PCREL32 Or Direct call Or PC relative 32 bit Or RiscvCopy Or MetagRelbranch Or Or1KLo16InInsn Or lower 16bits of address Or Arc32
    _S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32 =
        0x4,
    /// S390Pc32 Or PowerpcAddr16Hi Or MipsHi16 Or Tilegx64Pcrel Or X8664Copy Or SparcDisp16 Or ArmAbs16 Or I386Copy Or PC relative 16 bit Or Optimization hint for LITERAL Or high 16bit of absolute address Or 32-bit rel (S + A - P) Or ShDir8Wpl Or Cris16Pcrel Or PC-relative 16-bit signed Or PC relative 18 bit shifted Or Direct 64 bit Or 5 bit constant expression Or PC relative 16 bit Or RiscvJumpSlot Or MetagGetsetoff Or Or1KHi16InInsn Or high 16bits of address Or ArcB26
    _S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26 =
        0x5,
    /// S390Got12 Or PowerpcAddr16Ha Or MipsLo16 Or Tilegx32Pcrel Or X8664GlobDat Or SparcDisp32 Or ArmAbs12 Or I386GlobDat Or PC relative 8 bit Or Right 14 bits of eff. address Or Add displacement to GP Or adjusted high 16bit Or disp ((S + A - P) >>1) & 0x7ff Or ShDir8Wpz Or Cris32Pcrel Or PC-relative 8-bit signed Or PC relative 26 bit shifted Or Low 16 bit Or 5 bit expression, shift 22 Or PC relative 8 bit Or RiscvTlsDtpmod32 Or MetagReg32Op1 Or Or1KInsnRel26 Or adjusted high 16bits Or ArcB22Pcrel
    _S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel =
        0x6,
    /// S390Got32 Or PowerpcAddr14 Or Tilegx16Pcrel Or MipsGprel16 Or X8664JumpSlot Or SparcWdisp30 Or ArmThmAbs5 Or I386JumpSlot Or 32 bit PC relative GOT entry Or Create PLT entry Or PC+4 relative 23 bit shifted Or 16bit address, 2 bits ignored Or ShDir8Bp Or CrisGnuVtinherit Or Ancient C++ vtable garbage.. Or High 16 bit with unsigned low Or Read-only small data area Or 6 bit constant expression Or Low 16 bit Or RiscvTlsDtpmod64 Or MetagReg32Op2 Or Or1KGnuVtentry Or 16bit address, word aligned Or ArcH30
    _S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30 =
        0x7,
    /// S390Plt32 Or PowerpcAddr14Brtaken Or MipsLiteral Or Tilegx8Pcrel Or X8664Relative Or SparcWdisp22 Or ArmAbs8 Or I386Relative Or 16 bit PC relative GOT entry Or PC+4 relative 16 bit shifted Or PpcAddr14Brtaken Or ShDir8W Or CrisGnuVtentry Or ... collection annotation Or High 16 bit with signed low Or Read-write small data area Or 8 bit constant expression Or High 16 bit Or RiscvTlsDtprel32 Or MetagReg32Op3 Or Or1KGnuVtinherit Or Ppc64Addr14Brtaken Or ArcN8
    _S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8 =
        0x8,
    /// S390GlobDat Or PowerpcRel24 Or MipsPc16 Or TilegxHw1 Or X866432 Or Sparc22 Or ArmThmCall Or I386Gotpc Or 32 bit GOT offset Or Left 21 bits of rel. address Or PC relative 32 bit Or PC relative 26 bit Or PC relative 24 bit (Thumb32 BL) Or 32 bit adjust by program base Or CrisGlobDat Or 32-bit PCrel offset to GOT Or 16 bit offset in SDA Or Symbol Op Symbol relocation Or Low 16 bit Or Copy relocation Or RiscvTlsTprel32 Or Bpf6432 Or MetagReg16Op2 Or Or1K16Pcrel Or PC-rel. 26 bit, word aligned Or ArcN24
    _S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24 =
        0xa,
    /// S390JmpSlot Or PowerpcRel14 Or TilegxHw2 Or I38632Plt Or MipsCall16 Or X866432S Or Sparc13 Or ArmThmPc8 Or 16 bit GOT offset Or Right 17 bits of rel. address Or PC relative 64 bit Or PC relative 16 bit Or off between got and sym (S) Or CrisJumpSlot Or 16-bit PCrel offset to GOT Or M32RGnuVtinherit Or GNU C++ vtable hierarchy Or High 16 bit, adjusted Or Create GOT entry Or RiscvTlsTprel64 Or MetagReg16Op3 Or Or1K8Pcrel Or PC relative 16 bit Or ArcN32
    _S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32 =
        0xb,
    /// S390Relative Or PowerpcRel14Brtaken Or MipsGprel32 Or TilegxHw3 Or X866416 Or SparcLo10 Or ArmBrelAdj Or 8 bit GOT offset Or 17 bits of rel. address Or PpcRel14Brtaken Or ArmAmpVcall9 Or PLT entry (S) Or CrisRelative Or 32-bit offset from GOT Or M32RGnuVtentry Or GNU C++ vtable member usage Or 32 bit symbol value + addend Or Create PLT entry Or MetagReg32Op4 Or Or1KGotpcHi16 Or Ppc64Rel14Brtaken Or ArcSda
    _S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda =
        0xc,
    /// S390Gotoff32 Or PowerpcRel14Brntaken Or MipsUnused1 Or TilegxHw0Last Or X8664Pc16 Or SparcGot10 Or ArmTlsDesc Or 32 bit PC relative PLT address Or PpcRel14Brntaken Or Obsolete static relocation Or offset to GOT (S + A - GOT) Or Cris16Got Or 24-bit offset from GOT Or PC-relative GOT offset Or 16 bit symbol value + addend Or Adjust by program base Or MetagHiog Or Or1KGotpcLo16 Or Ppc64Rel14Brntaken Or ArcSectoff
    _S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff =
        0xd,
    /// S390Gotpc Or PowerpcGot16 Or MipsUnused2 Or TilegxHw1Last Or X86648 Or SparcGot13 Or ArmThmSwi8 Or I386TlsTpoff Or 16 bit PC relative PLT address Or Right 14 bits of rel. address Or PpcGot16 Or PC offset to GOT (GOT + A - P) Or Cris32Got Or 16-bit offset from GOT Or GOT entry offset Or 8 bit symbol value + addend Or X1 pipe branch offset Or MetagLoog Or Or1KGot16 Or Ppc64Got16 Or ArcS21HPcrel
    _S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel =
        0xe,
    /// S390Got16 Or PowerpcGot16Lo Or MipsUnused3 Or TilegxHw2Last Or X8664Pc8 Or SparcGot22 Or ArmXpc25 Or I386TlsIe Or 8 bit PC relative PLT address Or PpcGot16Lo Or 32 bit GOT entry (G) Or Cris16Gotplt Or 32-bit PCrel to PLT entry Or PLT offset (PC-relative) Or 16 bit GP pointer offset Or X1 pipe jump offset Or MetagRel8 Or Or1KPlt26 Or Ppc64Got16Lo Or ArcS21WPcrel
    _S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel =
        0xf,
    /// S390Pc16 Or PowerpcGot16Hi Or MipsShift5 Or TilegxCopy Or X8664Dtpmod64 Or SparcPc10 Or ArmThmXpc22 Or I386TlsGotie Or 32 bit PLT offset Or PpcGot16Hi Or 32 bit PLT entry (G) Or Cris32Gotplt Or 16-bit PCrel to PLT entry Or Adjust by program base Or GNU C++ vtable hierarchy Or X1 pipe jump offset to PLT Or RiscvBranch Or MetagRel16 Or Or1KGotoffHi16 Or Ppc64Got16Hi Or ArcS25HPcrel
    _S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel =
        0x10,
    /// S390Pc16Dbl Or PowerpcGot16Ha Or MipsShift6 Or TilegxGlobDat Or X8664Dtpoff64 Or SparcPc22 Or ArmTlsDtpmod32 Or I386TlsLe Or 16 bit PLT offset Or GP relative 32 bit, high 16 bits Or PpcGot16Ha Or GOT entry in GLOB_DAT (GOT + G) Or Cris32Gotrel Or 32-bit offset to GOT entry Or Create PLT entry Or GNU C++ vtable member usage Or X0 pipe 8-bit Or RiscvJal Or Or1KGotoffLo16 Or Ppc64Got16Ha Or ArcS25WPcrel
    _S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel =
        0x11,
    /// S390Plt16Dbl Or PpcPltrel24 Or Mips64 Or TilegxJmpSlot Or X8664Tpoff64 Or SparcWplt30 Or ArmTlsDtpoff32 Or I386TlsGd Or 8 bit PLT offset Or Left 21 bits of rel. address Or GP relative 32 bit, low 16 bits Or PLT entry in GLOB_DAT (GOT + G) Or Cris32PltGotrel Or 24-bit offset to GOT entry Or Create GOT entry Or Unconditional branch Or Y0 pipe 8-bit Or RiscvCall Or Or1KCopy Or ArcSda32
    _S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32 =
        0x12,
    /// S390Pc32Dbl Or PowerpcCopy Or MipsGotDisp Or TilegxRelative Or X8664Tlsgd Or SparcCopy Or ArmTlsTpoff32 Or I386TlsLdm Or Copy symbol at runtime Or GP relative 16 bit Or PpcCopy Or ((S + A - P) >> 1) & 0x3ffffff Or Cris32PltPcrel Or 16-bit offset to GOT entry Or 64 bit offset to GOT Or Conditional branch Or X1 pipe 8-bit Or RiscvCallPlt Or Or1KGlobDat Or Ppc64Copy Or ArcSdaLdst
    _S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst =
        0x13,
    /// S390Plt32Dbl Or PowerpcGlobDat Or MipsGotPage Or TilegxBroffX1 Or X8664Tlsld Or SparcGlobDat Or ArmCopy Or I38616 Or Create GOT entry Or PpcGlobDat Or disp ((S + A - P) >> 1) & 0xffff Or CrisNum Or Copy symbol at runtime Or 32 bit offset to GOT Or Indirect call through register Or Y1 pipe 8-bit Or RiscvGotHi20 Or Nds3232Rela Or Or1KJmpSlot Or Ppc64GlobDat Or ArcSdaLdst1
    _S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1 =
        0x14,
    /// S390Gotpcdbl Or PowerpcJmpSlot Or MipsGotOfst Or TilegxJumpoffX1 Or X8664Dtpoff32 Or SparcJmpSlot Or ArmGlobDat Or I386Pc16 Or Create PLT entry Or PpcJmpSlot Or disp ((S + A - P) >> 2) & 0xffff Or Create GOT entry Or Runtime copy Or Nios2Align Or X1 pipe mtspr Or RiscvTlsGotHi20 Or Or1KRelative Or Ppc64JmpSlot Or ArcSdaLdst2
    _S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2 =
        0x15,
    /// S39064 Or PowerpcRelative Or MipsGotHi16 Or TilegxJumpoffX1Plt Or X8664Gottpoff Or SparcRelative Or ArmJumpSlot Or I3868 Or Adjust by program base Or Right 14 bits of rel. address Or PpcRelative Or disp ((S + A - P) >> 1) & 0x3ff Or Create PLT entry Or TLS Reloc Or 16 bit GOT entry Or X1 pipe mfspr Or RiscvTlsGdHi20 Or Or1KTlsGdHi16 Or Ppc64Relative Or ArcSda16Ld
    _S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld =
        0x16,
    /// S390Pc64 Or PpcLocal24Pc Or MipsGotLo16 Or TilegxImm8X0 Or X8664Tpoff32 Or SparcUa32 Or ArmRelative Or I386Pc8 Or disp ((S + A - P) >> 2) & 0x3ff Or Adjust by program base Or TLS General Dynamic Or 16 bit GOT entry for function Or X0 pipe 16-bit Or RiscvPcrelHi20 Or Or1KTlsGdLo16 Or ArcSda16Ld1
    _S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrX8664Tpoff32OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1 =
        0x17,
    /// S390Got64 Or PowerpcUaddr32 Or MipsSub Or TilegxImm8Y0 Or X8664Pc64 Or SparcPlt32 Or ArmGotoff32 Or I386TlsGd32 Or Copy symbol at runtime Or PpcUaddr32 Or 32 bit offset to GOT Or high & low 16 bit ADDR Or 32-bit offset for global dynamic Or TLS Local Dynamic Or %lo of offset to GOT pointer Or X1 pipe 16-bit Or RiscvPcrelLo12I Or Or1KTlsLdmHi16 Or Ppc64Uaddr32 Or ArcSda16Ld2
    _S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2 =
        0x18,
    /// S390Plt64 Or PowerpcUaddr16 Or MipsInsertA Or TilegxImm8X1 Or X8664Gotoff64 Or SparcHiplt22 Or ArmBasePrel Or I386TlsGdPush Or 32 bit GOT offset for GD Or Create GOT entry Or PpcUaddr16 Or 32 bit PC relative offset to GOT Or (S + A) & 0xffff Or ShSwitch16 Or 32-bit offset for local dynamic Or TLS Module ID Or %hiadj of offset to GOT pointer Or X0 pipe low 16-bit Or RiscvPcrelLo12S Or Or1KTlsLdmLo16 Or Ppc64Uaddr16 Or ArcS13Pcrel
    _S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel =
        0x19,
    /// S390Gotent Or PowerpcRel32 Or MipsInsertB Or TilegxImm8Y1 Or X8664Gotpc32 Or SparcLoplt10 Or ArmGotBrel Or I386TlsGdCall Or 16 bit GOT offset for GD Or GP-relative, left 21 bits Or Create PLT entry Or PpcRel32 Or 32 bit GOT entry Or high & low 16 bit GOTPC Or ShSwitch32 Or Module-relative offset Or TLS Offset Within TLS Block Or %lo of PC relative offset Or X1 pipe low 16-bit Or RiscvHi20 Or Or1KTlsLdoHi16 Or Ppc64Rel32 Or ArcW
    _S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW =
        0x1a,
    /// S390Gotoff16 Or PowerpcPlt32 Or MipsDelete Or TilegxDestImm8X1 Or X8664Got64 Or SparcPcplt32 Or ArmPlt32 Or I386TlsGdPop Or 8 bit GOT offset for GD Or Adjust by program base Or PpcPlt32 Or (GOT + A - P) & 0xffff Or ShUses Or Mn10300TlsGotie Or TLS Offset Within TLS Block Or %hiadj of PC relative offset Or X0 pipe high 16-bit Or RiscvLo12I Or Or1KTlsLdoLo16 Or Ppc64Plt32 Or Arc32Me
    _S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me =
        0x1b,
    /// S390Gotoff64 Or PowerpcPltrel32 Or MipsHigher Or TilegxMtImm14X1 Or X8664Gotpcrel64 Or SparcPcplt22 Or ArmCall Or I386TlsLdm32 Or 32 bit GOT offset for LDM Or AlphaTlsGdHi Or PpcPltrel32 Or high & low 16 bit GOTOFF Or ShCount Or Mn10300TlsIe Or TLS Offset From Thread Pointer Or 16 bit GOT offset for TLS GD Or X1 pipe high 16-bit Or RiscvLo12S Or Or1KTlsIeHi16 Or Ppc64Pltrel32 Or ArcN32Me
    _S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me =
        0x1c,
    /// S390Gotplt12 Or PowerpcPlt16Lo Or MipsHighest Or TilegxMfImm14X1 Or X8664Gotpc64 Or SparcPcplt10 Or ArmJump24 Or I386TlsLdmPush Or 16 bit GOT offset for LDM Or AlphaTlsgd Or PpcPlt16Lo Or (S + A - GOT) & 0xffff Or ShAlign Or Mn10300TlsLe Or TLS Offset From Thread Pointer Or 16 bit GOT offset for TLS LDM Or X0 pipe high 16-bit, adjusted Or RiscvTprelHi20 Or Or1KTlsIeLo16 Or Ppc64Plt16Lo Or ArcSectoffMe
    _S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe =
        0x1d,
    /// S390Gotplt16 Or PowerpcPlt16Hi Or MipsCallHi16 Or TilegxMmstartX0 Or X8664Gotplt64 Or Sparc10 Or ArmThmJump24 Or I386TlsLdmCall Or 8 bit GOT offset for LDM Or GP-relative, right 14 bits Or AlphaTlsLdm Or PpcPlt16Hi Or 12 bit disp GOT entry (G) Or ShCode Or ID of module containing symbol Or 16 bit module relative offset Or X1 pipe high 16-bit, adjusted Or RiscvTprelLo12I Or MetagGnuVtinherit Or Or1KTlsLeHi16 Or Ppc64Plt16Hi Or ArcSda32Me
    _S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me =
        0x1e,
    /// S390Gotplt32 Or PowerpcPlt16Ha Or MipsCallLo16 Or TilegxMmendX0 Or X8664Pltoff64 Or Sparc11 Or ArmBaseAbs Or I386TlsLdmPop Or 32 bit module-relative offset Or AlphaDtpmod64 Or PpcPlt16Ha Or high & low 16 bit GOT Or ShData Or Offset in module TLS block Or 16 bit GOT offset for TLS IE Or X0 pipe PC relative 16 bit Or RiscvTprelLo12S Or MetagGnuVtentry Or Or1KTlsLeLo16 Or Ppc64Plt16Ha Or ArcWMe
    _S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe =
        0x1f,
    /// S390Gotplt64 Or PpcSdarel16 Or MipsScnDisp Or X8664Size32 Or TilegxShamtX0 Or Sparc64 Or ArmAluPcrel70 Or I386TlsLdo32 Or 16 bit module-relative offset Or AlphaGotdtprel Or (G & 0xffff) Or ShLabel Or Offset in static TLS block Or 16 bit LE TP-relative offset Or X1 pipe PC relative 16 bit Or RiscvTprelAdd Or MetagHi16Gotoff Or Or1KTlsTpoff Or ArcH30Me
    _S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me =
        0x20,
    /// S390Gotpltent Or PowerpcSectoff Or MipsRel16 Or X8664Size64 Or TilegxShamtX1 Or SparcOlo10 Or ArmAluPcrel158 Or I386TlsIe32 Or 8 bit module-relative offset Or AlphaDtprel64 Or PpcSectoff Or 12 bit disp PLT entry (G) Or ShSwitch8 Or Mn10300SymDiff Or Direct 16 bit Or Module number Or X0 pipe PC relative low 16 bit Or RiscvAdd8 Or MetagLo16Gotoff Or Or1KTlsDtpoff Or Ppc64Sectoff Or symbol + addend, add imm14 Or ArcSectoffU8
    _S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8 =
        0x21,
    /// S390Pltoff16 Or PowerpcSectoffLo Or MipsAddImmediate Or TilegxShamtY0 Or X8664Gotpc32Tlsdesc Or SparcHh22 Or ArmAluPcrel2315 Or I386TlsLe32 Or 32 bit GOT offset for IE Or LT-relative, left 21 bits Or AlphaDtprelhi Or PpcSectoffLo Or high & low 16 bit PLT Or ShGnuVtinherit Or Mn10300Align Or Direct 32 bit Or Module-relative offset Or X1 pipe PC relative low 16 bit Or RiscvAdd16 Or MetagGetsetGotoff Or Or1KTlsDtpmod Or Ppc64SectoffLo Or symbol + addend, add imm22 Or ArcSectoffS9
    _S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9 =
        0x22,
    /// S390Pltoff32 Or PowerpcSectoffHi Or MipsPjump Or TilegxShamtY1 Or X8664TlsdescCall Or SparcHm10 Or ArmLdrSbrel110Nc Or I386TlsDtpmod32 Or 16 bit GOT offset for IE Or AlphaDtprello Or PpcSectoffHi Or Deprecated, prog. base relative Or G & 0xffff Or ShGnuVtentry Or Mn10300Num Or Direct 24 bit Or TP-relative offset Or X0 pipe PC relative high 16 bit Or RiscvAdd32 Or MetagGetsetGot Or Ppc64SectoffHi Or symbol + addend, mov imm64 Or AcSectoffU8
    _S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8 =
        0x23,
    /// S390Pltoff64 Or PowerpcSectoffHa Or MipsRelgot Or TilegxImm16X0Hw0 Or X8664Tlsdesc Or SparcLm22 Or ArmAluSbrel1912Nc Or I386TlsDtpoff32 Or 8 bit GOT offset for IE Or AlphaDtprel16 Or PpcSectoffHa Or Deprecated, prog. base relative Or high & low 16 bit ADDRGOT Or PC relative 10 bit shifted Or Copy symbol at runtime Or X1 pipe PC relative high 16 bit Or RiscvAdd64 Or MetagHi16Gotpc Or Ppc64SectoffHa Or symbol + addend, data4 MSB Or AcSectoffU81
    _S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81 =
        0x24,
    /// S390TlsLoad Or PowerpcAddr30 Or MipsJalr Or TilegxImm16X1Hw0 Or X8664Irelative Or SparcPcHh22 Or ArmAluSbrel2720Ck Or I386TlsTpoff32 Or M68KTlsLe32 Or AlphaGottprel Or word30 (S + A - P) >> 2 Or Deprecated, prog. base relative Or (GOT + G * 4) & 0xffff Or PC relative 18 bit shifted Or Create GOT entry Or X0 pipe PC relative ha() 16 bit Or RiscvSub8 Or MetagLo16Gotpc Or symbol + addend, data4 LSB Or AcSectoffU82
    _S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82 =
        0x25,
    /// S390TlsGdcall Or Ppc64Addr64 Or MipsTlsDtpmod32 Or TilegxImm16X0Hw1 Or X8664Relative64 Or SparcPcHm10 Or ArmTarget1 Or M68KTlsLe16 Or 32-bit symbol size Or LT-relative, right 14 bits Or AlphaTprel64 Or high & low 16 bit ADDRPLT Or PC relative 26 bit shifted Or Create PLT entry Or X1 pipe PC relative ha() 16 bit Or RiscvSub16 Or MetagHi16Plt Or symbol + addend, data8 MSB Or AcSectoffS9
    _S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9 =
        0x26,
    /// S390TlsLdcall Or Ppc64Addr16Higher Or MipsTlsDtprel32 Or TilegxImm16X1Hw1 Or X8664Pc32Bnd Or SparcPcLm22 Or ArmSbrel31 Or I386TlsGotdesc Or M68KTlsLe8 Or AlphaTprelhi Or (GOT+G*4) & 0xffff Or High 16 bit with unsigned low Or Adjust by program base Or X0 pipe 16-bit GOT offset Or RiscvSub32 Or MetagLo16Plt Or Nds32Copy Or symbol + addend, data8 LSB Or AcSectoffS91
    _S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91 =
        0x27,
    /// S390TlsGd32 Or Ppc64Addr16Highera Or MipsTlsDtpmod64 Or TilegxImm16X0Hw2 Or X8664Plt32Bnd Or SparcWdisp16 Or ArmV4Bx Or I386TlsDescCall Or 32 bit module number Or AlphaTprello Or disp ((S+A-P) >>1) & x3ffffff Or High 16 bit with signed low Or 16 bit offset to GOT pointer Or X1 pipe 16-bit GOT offset Or RiscvSub64 Or MetagRelbranchPlt Or Nds32GlobDat Or AcSectoffS92
    _S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92 =
        0x28,
    /// S390TlsGd64 Or Ppc64Addr16Highest Or MipsTlsDtprel64 Or TilegxImm16X1Hw2 Or X8664Gotpcrelx Or SparcWdisp19 Or ArmTarget2 Or I386TlsDesc Or 32 bit module-relative offset Or 32 bits section rel. address Or AlphaTprel16 Or (S+A-BTEXT) & 0xffff Or Low 16 bit Or Direct call in .noat section Or X0 pipe low 16-bit GOT offset Or RiscvGnuVtinherit Or MetagGotoff Or Nds32JmpSlot Or ArcSectoffMe1
    _S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1 =
        0x29,
    /// S390TlsGotie12 Or Ppc64Addr16Highesta Or MipsTlsGd Or TilegxImm16X0Hw3 Or X8664RexGotpcrelx Or SparcGlobJmp Or ArmPrel31 Or I386Irelative Or 32 bit TP-relative offset Or (S+A-BTEXT) & 0xffff Or 16 bit offset in SDA Or %lo() of GOT entry Or X1 pipe low 16-bit GOT offset Or RiscvGnuVtentry Or MetagPlt Or Nds32Relative Or @gprel(sym + add), add imm22 Or ArcSectoffMe2
    _S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2 =
        0x2a,
    /// S390TlsGotie32 Or Ppc64Uaddr64 Or MipsTlsLdm Or TilegxImm16X1Hw3 Or Sparc7 Or ArmMovwAbsNc Or I386Got32X Or M68KNum Or disp ((S+A-P) >>1) & 0x3ffff Or X8664Num Or M32RRelaGnuVtinherit Or %hiadj() of GOT entry Or X0 pipe high 16-bit GOT offset Or RiscvAlign Or MetagCopy Or @gprel(sym + add), mov imm64 Or ArcSectoff1
    _S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1 =
        0x2b,
    /// S390TlsGotie64 Or Ppc64Rel64 Or MipsTlsDtprelHi16 Or TilegxImm16X0Hw0Last Or Sparc5 Or ArmMovtAbs Or I386Num Or disp (S+A-BDATA) & 0x3ffff Or M32RRelaGnuVtentry Or %lo() of function GOT entry Or X1 pipe high 16-bit GOT offset Or RiscvRvcBranch Or MetagJmpSlot Or @gprel(sym + add), data4 MSB Or ArcSectoff2
    _S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2 =
        0x2c,
    /// S390TlsLdm32 Or Ppc64Plt64 Or MipsTlsDtprelLo16 Or TilegxImm16X1Hw0Last Or Sparc6 Or ArmMovwPrelNc Or disp ((S+A-BDATA)>>1) & 0x3ffff Or PC relative 32 bit Or %hiadj() of function GOT entry Or X0 pipe ha() 16-bit GOT offset Or RiscvRvcJump Or MetagRelative Or @gprel(sym + add), data4 LSB
    _S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb =
        0x2d,
    /// S390TlsLdm64 Or Ppc64Pltrel64 Or MipsTlsGottprel Or TilegxImm16X0Hw1Last Or SparcDisp64 Or ArmMovtPrel Or AlphaNum Or disp ((S+A-BDATA)>>2) & 0x3ffff Or X1 pipe ha() 16-bit GOT offset Or RiscvRvcLui Or MetagGlobDat Or @gprel(sym + add), data8 MSB
    _S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb =
        0x2e,
    /// S390TlsIe32 Or Ppc64Toc16 Or MipsTlsTprel32 Or TilegxImm16X1Hw1Last Or SparcPlt64 Or ArmThmMovwAbsNc Or X0 pipe mm "start" Or RiscvGprelI Or MetagTlsGd Or @gprel(sym + add), data8 LSB
    _S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGdOrIa64Gprel64Lsb =
        0x2f,
    /// S390TlsIe64 Or Ppc64Toc16Lo Or MipsTlsTprel64 Or TilegxImm16X0Hw2Last Or SparcHix22 Or ArmThmMovtAbs Or No relocation, set segment base Or disp (G >> 2) Or 24 bit GOT entry Or X0 pipe mm "end" Or RiscvGprelS Or MetagTlsLdm
    _S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm =
        0x30,
    /// S390TlsIeent Or Ppc64Toc16Hi Or MipsTlsTprelHi16 Or TilegxImm16X1Hw2Last Or SparcLox10 Or ArmThmMovwPrelNc Or 32 bits segment rel. address Or disp (G >> 2) Or 26 bit PC relative to PLT shifted Or X1 pipe mm "start" Or RiscvTprelI Or MetagTlsLdoHi16
    _S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16 =
        0x31,
    /// S390TlsLe32 Or Ppc64Toc16Ha Or MipsTlsTprelLo16 Or TilegxImm16X0Hw0Pcrel Or SparcH44 Or ArmThmMovtPrel Or PLT rel. address, left 21 bits Or disp ((S+A-P) >>2) & 0x7f Or Copy symbol at runtime Or X1 pipe mm "end" Or RiscvTprelS Or MetagTlsLdoLo16 Or @ltoff(sym + add), add imm22 Or ArcPc32
    _S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32 =
        0x32,
    /// S390TlsLe64 Or Ppc64Toc Or MipsGlobDat Or TilegxImm16X1Hw0Pcrel Or SparcM44 Or ArmThmJump19 Or 32 bit offset to TLS block Or Create GOT entry Or X0 pipe shift amount Or RiscvRelax Or MetagTlsLdo Or @ltoff(sym + add), mov imm64 Or ArcGotpc32
    _S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32 =
        0x33,
    /// S390TlsLdo32 Or Ppc64Pltgot16 Or TilegxImm16X0Hw1Pcrel Or SparcL44 Or ArmThmJump6 Or CkcoreTlsIe32 Or Create PLT entry Or X1 pipe shift amount Or RiscvSub6 Or MetagTlsIe Or ArcPlt32
    _S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32 =
        0x34,
    /// S390TlsLdo64 Or Ppc64Pltgot16Lo Or TilegxImm16X1Hw1Pcrel Or SparcRegister Or ArmThmAluPrel110 Or CkcoreTlsGd32 Or Adjust by program base Or Y0 pipe shift amount Or RiscvSet6 Or MetagTlsIenonpic Or ArcCopy
    _S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy =
        0x35,
    /// S390TlsDtpmod Or Ppc64Pltgot16Hi Or TilegxImm16X0Hw2Pcrel Or SparcUa64 Or ArmThmPc12 Or PLT rel. address, right 14 bits Or CkcoreTlsLdm32 Or 24 bit offset to GOT Or Y1 pipe shift amount Or RiscvSet8 Or MetagTlsIenonpicHi16 Or ArcGlobDat
    _S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat =
        0x36,
    /// S390TlsDtpoff Or Ppc64Pltgot16Ha Or TilegxImm16X1Hw2Pcrel Or SparcUa16 Or ArmAbs32Noi Or CkcoreTlsLdo32 Or 24 bit PC relative offset to GOT Or X1 pipe destination 8-bit Or RiscvSet16 Or MetagTlsIenonpicLo16 Or ArcJumpSlot
    _S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot =
        0x37,
    _S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative =
        0x38,
    /// S39020 Or Ppc64Addr16LoDs Or TilegxImm16X1Hw3Pcrel Or SparcTlsGdLo10 Or ArmAluPcG0Nc Or 32 bits LT-rel. function pointer Or CkcoreTlsDtpoff32 Or M32RGot16HiSlo Or Riscv32Pcrel Or MetagTlsDtpmod Or ArcGotoff
    _S39020OrPpc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff =
        0x39,
    /// S390Got20 Or Ppc64Got16Ds Or TilegxImm16X0Hw0LastPcrel Or SparcTlsGdAdd Or ArmAluPcG0 Or LT-rel. fct ptr, left 21 bits Or CkcoreTlsTpoff32 Or Low 16 bit GOT entry Or RiscvIrelative Or MetagTlsDtpoff Or @pltoff(sym + add), add imm22 Or ArcGotpc
    _S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc =
        0x3a,
    /// S390Gotplt20 Or Ppc64Got16LoDs Or TilegxImm16X1Hw0LastPcrel Or SparcTlsGdCall Or ArmAluPcG1Nc Or M32RGotpcHiUlo Or RiscvNum Or MetagTlsLe Or @pltoff(sym + add), mov imm64 Or ArcGot32
    _S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrIa64Pltoff64IOrArcGot32 =
        0x3b,
    /// S390TlsGotie20 Or Ppc64Plt16LoDs Or MipsPc21S2 Or TilegxImm16X0Hw1LastPcrel Or SparcTlsLdmHi22 Or ArmAluPcG1 Or M32RGotpcHiSlo Or "jal" for TLS GD Or MetagTlsLeHi16
    _S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16 =
        0x3c,
    /// S390Irelative Or Ppc64SectoffDs Or MipsPc26S2 Or TilegxImm16X1Hw1LastPcrel Or SparcTlsLdmLo10 Or ArmAluPcG2 Or M32RGotpcLo Or X0 pipe "addi" for TLS GD Or MetagTlsLeLo16
    _S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16 =
        0x3d,
    /// S390Pc12Dbl Or Ppc64SectoffLoDs Or MipsPc18S3 Or TilegxImm16X0Hw2LastPcrel Or SparcTlsLdmAdd Or ArmLdrPcG1 Or LT-rel. fct ptr, right 14 bits Or S390Num Or M32RGotoffHiUlo Or X1 pipe "addi" for TLS GD Or @pltoff(sym + add), data8 MSB
    _S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb =
        0x3e,
    /// S390Plt12Dbl Or Ppc64Toc16Ds Or MipsPc19S2 Or TilegxImm16X1Hw2LastPcrel Or SparcTlsLdmCall Or ArmLdrPcG2 Or M32RGotoffHiSlo Or Y0 pipe "addi" for TLS GD Or @pltoff(sym + add), data8 LSB
    _S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrM32RGotoffHiSloOrTileproImm8Y0TlsGdAddOrIa64Pltoff64Lsb =
        0x3f,
    /// S390Pc24Dbl Or Ppc64Toc16LoDs Or MipsPchi16 Or TilegxImm16X0Hw0Got Or SparcTlsLdoHix22 Or ArmLdrsPcG0 Or 64 bits function address Or Low 16 bit offset to GOT Or Y1 pipe "addi" for TLS GD
    _S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd =
        0x40,
    /// S390Plt24Dbl Or Ppc64Pltgot16Ds Or MipsPclo16 Or TilegxImm16X1Hw0Got Or SparcTlsLdoLox10 Or ArmLdrsPcG1 Or 32 bits function address Or "lw_tls" for TLS IE
    _S390Plt24DblOrPpc64Pltgot16DsOrMipsPclo16OrTilegxImm16X1Hw0GotOrSparcTlsLdoLox10OrArmLdrsPcG1OrPariscPlabel32OrTileproTlsIeLoad =
        0x41,
    /// S390GnuVtinherit Or PowerpcRel16Lo Or MipsGnuRel16S2 Or X8664GnuVtinherit Or SparcGnuVtinherit Or I386GnuVtinherit Or half16   (sym+add-.)@l Or half16   (sym+add-.)@l Or ArmRsbrel32
    _S390GnuVtinheritOrPowerpcRel16LoOrMipsGnuRel16S2OrX8664GnuVtinheritOrSparcGnuVtinheritOrI386GnuVtinheritOrPpcRel16LoOrPpc64Rel16LoOrArmRsbrel32 =
        0xfa,
    /// S390GnuVtentry Or PowerpcRel16Hi Or SparcGnuVtentry Or X8664GnuVtentry Or I386GnuVtentry Or half16   (sym+add-.)@h Or half16   (sym+add-.)@h Or ArmThmRpc22
    _S390GnuVtentryOrPowerpcRel16HiOrSparcGnuVtentryOrX8664GnuVtentryOrI386GnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22 =
        0xfb,
    /// Ppc64Pltgot16LoDs Or TilegxImm16X0Hw0PltPcrel Or SparcTlsLdoAdd Or ArmLdrsPcG2 Or Left 21 bits of fdesc address Or X0 pipe 16-bit TLS GD offset Or ArcTlsDtpmod
    _Ppc64Pltgot16LoDsOrTilegxImm16X0Hw0PltPcrelOrSparcTlsLdoAddOrArmLdrsPcG2OrPariscPlabel21LOrTileproImm16X0TlsGdOrArcTlsDtpmod =
        0x42,
    /// PowerpcTls Or TilegxImm16X1Hw0PltPcrel Or SparcTlsIeHi22 Or ArmLdcPcG0 Or none	(sym+add)@tls Or none	(sym+add)@tls Or X1 pipe 16-bit TLS GD offset Or @fptr(sym + add), mov imm64 Or ArcTlsDtpoff
    _PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff =
        0x43,
    /// PowerpcDtpmod Or TilegxImm16X0Hw1PltPcrel Or SparcTlsIeLo10 Or ArmLdcPcG1 Or word32	(sym+add)@dtpmod Or doubleword64 (sym+add)@dtpmod Or X0 pipe low 16-bit TLS GD offset Or @fptr(sym + add), data4 MSB Or ArcTlsTpoff
    _PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff =
        0x44,
    /// PowerpcTprel16 Or TilegxImm16X1Hw1PltPcrel Or SparcTlsIeLd Or ArmLdcPcG2 Or half16*	(sym+add)@tprel Or half16*	(sym+add)@tprel Or X1 pipe low 16-bit TLS GD offset Or @fptr(sym + add), data4 LSB Or ArcTlsGdGot
    _PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot =
        0x45,
    /// PowerpcTprel16Lo Or TilegxImm16X0Hw2PltPcrel Or SparcTlsIeLdx Or ArmAluSbG0Nc Or Right 14 bits of fdesc address Or half16	(sym+add)@tprel@l Or half16	(sym+add)@tprel@l Or X0 pipe high 16-bit TLS GD offset Or @fptr(sym + add), data8 MSB Or ArcTlsGdLd
    _PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd =
        0x46,
    /// PowerpcTprel16Hi Or TilegxImm16X1Hw2PltPcrel Or SparcTlsIeAdd Or ArmAluSbG0 Or half16	(sym+add)@tprel@h Or half16	(sym+add)@tprel@h Or X1 pipe high 16-bit TLS GD offset Or @fptr(sym + add), data8 LSB Or ArcTlsGdCall
    _PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall =
        0x47,
    /// PowerpcTprel16Ha Or TilegxImm16X0Hw0LastGot Or SparcTlsLeHix22 Or ArmAluSbG1Nc Or 64 bits PC-rel. address Or half16	(sym+add)@tprel@ha Or half16	(sym+add)@tprel@ha Or X0 pipe ha() 16-bit TLS GD offset Or @pcrel(sym + add), brl Or ArcTlsIeGot
    _PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot =
        0x48,
    /// PowerpcTprel Or TilegxImm16X1Hw0LastGot Or SparcTlsLeLox10 Or ArmAluSbG1 Or word32	(sym+add)@tprel Or doubleword64 (sym+add)@tprel Or X1 pipe ha() 16-bit TLS GD offset Or @pcrel(sym + add), ptb, call
    _PowerpcTprelOrTilegxImm16X1Hw0LastGotOrSparcTlsLeLox10OrArmAluSbG1OrPpcTprel32OrPpc64Tprel64OrTileproImm16X1TlsGdHaOrIa64Pcrel21B =
        0x49,
    /// PowerpcDtprel16 Or TilegxImm16X0Hw1LastGot Or SparcTlsDtpmod32 Or ArmAluSbG2 Or 22 bits PC-rel. address Or half16*	(sym+add)@dtprel Or half16*	(sym+add)@dtprel Or X0 pipe 16-bit TLS IE offset Or @pcrel(sym + add), chk.s Or ArcTlsDtpoffS9 Or ArcTlsLeS9
    _PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9 =
        0x4a,
    /// PowerpcDtprel16Lo Or TilegxImm16X1Hw1LastGot Or SparcTlsDtpmod64 Or ArmLdrSbG0 Or PC-rel. address, right 14 bits Or half16	(sym+add)@dtprel@l Or half16	(sym+add)@dtprel@l Or X1 pipe 16-bit TLS IE offset Or @pcrel(sym + add), fchkf Or ArcTlsLe32
    _PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32 =
        0x4b,
    /// PowerpcDtprel16Hi Or SparcTlsDtpoff32 Or ArmLdrSbG1 Or PC rel. address, right 14 bits Or half16	(sym+add)@dtprel@h Or half16	(sym+add)@dtprel@h Or X0 pipe low 16-bit TLS IE offset Or X0 pipe PC-rel PLT hword 3 Or @pcrel(sym + add), data4 MSB
    _PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb =
        0x4c,
    /// PowerpcDtprel16Ha Or SparcTlsDtpoff64 Or ArmLdrSbG2 Or 16 bits PC-rel. address Or half16	(sym+add)@dtprel@ha Or half16	(sym+add)@dtprel@ha Or X1 pipe low 16-bit TLS IE offset Or X1 pipe PC-rel PLT hword 3 Or @pcrel(sym + add), data4 LSB
    _PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb =
        0x4d,
    /// PowerpcDtprel Or TilegxImm16X0Hw0TlsGd Or SparcTlsTpoff32 Or ArmLdrsSbG0 Or 16 bits PC-rel. address Or word32	(sym+add)@dtprel Or doubleword64 (sym+add)@dtprel Or X0 pipe high 16-bit TLS IE offset Or @pcrel(sym + add), data8 MSB
    _PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb =
        0x4e,
    /// PowerpcGotTlsgd16 Or TilegxImm16X1Hw0TlsGd Or SparcTlsTpoff64 Or ArmLdrsSbG1 Or 16 bits PC-rel. address Or half16*	(sym+add)@got@tlsgd Or half16*	(sym+add)@got@tlsgd Or X1 pipe high 16-bit TLS IE offset Or @pcrel(sym + add), data8 LSB
    _PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb =
        0x4f,
    /// PowerpcGotTlsgd16Lo Or SparcGotdataHix22 Or TilegxImm16X0Hw0TlsLe Or ArmLdrsSbG2 Or 64 bits of eff. address Or half16	(sym+add)@got@tlsgd@l Or half16	(sym+add)@got@tlsgd@l Or X0 pipe ha() 16-bit TLS IE offset
    _PowerpcGotTlsgd16LoOrSparcGotdataHix22OrTilegxImm16X0Hw0TlsLeOrArmLdrsSbG2OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrTileproImm16X0TlsIeHa =
        0x50,
    /// PowerpcGotTlsgd16Hi Or SparcGotdataLox10 Or TilegxImm16X1Hw0TlsLe Or ArmLdcSbG0 Or half16	(sym+add)@got@tlsgd@h Or half16	(sym+add)@got@tlsgd@h Or X1 pipe ha() 16-bit TLS IE offset
    _PowerpcGotTlsgd16HiOrSparcGotdataLox10OrTilegxImm16X1Hw0TlsLeOrArmLdcSbG0OrPpcGotTlsgd16HiOrPpc64GotTlsgd16HiOrTileproImm16X1TlsIeHa =
        0x51,
    /// PowerpcGotTlsgd16Ha Or SparcGotdataOpHix22 Or TilegxImm16X0Hw0LastTlsLe Or ArmLdcSbG1 Or half16	(sym+add)@got@tlsgd@ha Or half16	(sym+add)@got@tlsgd@ha Or ID of module containing symbol Or @ltoff(@fptr(s+a)), imm22
    _PowerpcGotTlsgd16HaOrSparcGotdataOpHix22OrTilegxImm16X0Hw0LastTlsLeOrArmLdcSbG1OrPpcGotTlsgd16HaOrPpc64GotTlsgd16HaOrTileproTlsDtpmod32OrIa64LtoffFptr22 =
        0x52,
    /// PowerpcGotTlsld16 Or SparcGotdataOpLox10 Or TilegxImm16X1Hw0LastTlsLe Or ArmLdcSbG2 Or 14 bits of eff. address Or half16*	(sym+add)@got@tlsld Or half16*	(sym+add)@got@tlsld Or Offset in TLS block Or @ltoff(@fptr(s+a)), imm64
    _PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I =
        0x53,
    /// PowerpcGotTlsld16Lo Or SparcGotdataOp Or TilegxImm16X0Hw1LastTlsLe Or ArmMovwBrelNc Or 14 bits of eff. address Or half16	(sym+add)@got@tlsld@l Or half16	(sym+add)@got@tlsld@l Or Offset in static TLS block Or @ltoff(@fptr(s+a)), data4 MSB
    _PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb =
        0x54,
    /// PowerpcGotTlsld16Hi Or TilegxImm16X1Hw1LastTlsLe Or SparcH34 Or ArmMovtBrel Or 16 bits of eff. address Or half16	(sym+add)@got@tlsld@h Or half16	(sym+add)@got@tlsld@h Or X0 pipe 16-bit TLS LE offset Or @ltoff(@fptr(s+a)), data4 LSB
    _PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb =
        0x55,
    /// PowerpcGotTlsld16Ha Or TilegxImm16X0Hw0LastTlsGd Or SparcSize32 Or ArmMovwBrel Or 16 bits of eff. address Or half16	(sym+add)@got@tlsld@ha Or half16	(sym+add)@got@tlsld@ha Or X1 pipe 16-bit TLS LE offset Or @ltoff(@fptr(s+a)), data8 MSB
    _PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb =
        0x56,
    /// PowerpcGotTprel16 Or TilegxImm16X1Hw0LastTlsGd Or SparcSize64 Or ArmThmMovwBrelNc Or 16 bits of eff. address Or half16*	(sym+add)@got@tprel Or half16ds*	(sym+add)@got@tprel Or X0 pipe low 16-bit TLS LE offset Or @ltoff(@fptr(s+a)), data8 LSB
    _PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb =
        0x57,
    /// PowerpcGotTprel16Lo Or TilegxImm16X0Hw1LastTlsGd Or SparcWdisp10 Or ArmThmMovtBrel Or 64 bits of GP-rel. address Or half16	(sym+add)@got@tprel@l Or half16ds (sym+add)@got@tprel@l Or X1 pipe low 16-bit TLS LE offset
    _PowerpcGotTprel16LoOrTilegxImm16X0Hw1LastTlsGdOrSparcWdisp10OrArmThmMovtBrelOrPariscGprel64OrPpcGotTprel16LoOrPpc64GotTprel16LoDsOrTileproImm16X1TlsLeLo =
        0x58,
    /// PowerpcGotTprel16Hi Or TilegxImm16X1Hw1LastTlsGd Or ArmThmMovwBrel Or half16	(sym+add)@got@tprel@h Or half16	(sym+add)@got@tprel@h Or X0 pipe high 16-bit TLS LE offset
    _PowerpcGotTprel16HiOrTilegxImm16X1Hw1LastTlsGdOrArmThmMovwBrelOrPpcGotTprel16HiOrPpc64GotTprel16HiOrTileproImm16X0TlsLeHi =
        0x59,
    /// PowerpcGotTprel16Ha Or TilegxIrelative Or ArmTlsGotdesc Or half16	(sym+add)@got@tprel@ha Or half16	(sym+add)@got@tprel@ha Or X1 pipe high 16-bit TLS LE offset
    _PowerpcGotTprel16HaOrTilegxIrelativeOrArmTlsGotdescOrPpcGotTprel16HaOrPpc64GotTprel16HaOrTileproImm16X1TlsLeHi =
        0x5a,
    /// PowerpcGotDtprel16 Or ArmTlsCall Or GP-rel. address, right 14 bits Or half16*	(sym+add)@got@dtprel Or half16ds*	(sym+add)@got@dtprel Or X0 pipe ha() 16-bit TLS LE offset
    _PowerpcGotDtprel16OrArmTlsCallOrPariscGprel14WrOrPpcGotDtprel16OrPpc64GotDtprel16DsOrTileproImm16X0TlsLeHa =
        0x5b,
    /// PowerpcGotDtprel16Lo Or TilegxImm16X0Hw0TlsIe Or ArmTlsDescseq Or GP-rel. address, right 14 bits Or half16*	(sym+add)@got@dtprel@l Or half16ds (sym+add)@got@dtprel@l Or X1 pipe ha() 16-bit TLS LE offset Or @segrel(sym + add), data4 MSB
    _PowerpcGotDtprel16LoOrTilegxImm16X0Hw0TlsIeOrArmTlsDescseqOrPariscGprel14DrOrPpcGotDtprel16LoOrPpc64GotDtprel16LoDsOrTileproImm16X1TlsLeHaOrIa64Segrel32Msb =
        0x5c,
    /// PowerpcGotDtprel16Hi Or TilegxImm16X1Hw0TlsIe Or ArmThmTlsCall Or 16 bits GP-rel. address Or half16*	(sym+add)@got@dtprel@h Or half16	(sym+add)@got@dtprel@h Or @segrel(sym + add), data4 LSB
    _PowerpcGotDtprel16HiOrTilegxImm16X1Hw0TlsIeOrArmThmTlsCallOrPariscGprel16FOrPpcGotDtprel16HiOrPpc64GotDtprel16HiOrIa64Segrel32Lsb =
        0x5d,
    /// PowerpcGotDtprel16Ha Or TilegxImm16X0Hw0LastPltPcrel Or ArmPlt32Abs Or 16 bits GP-rel. address Or half16*	(sym+add)@got@dtprel@ha Or half16	(sym+add)@got@dtprel@ha Or @segrel(sym + add), data8 MSB
    _PowerpcGotDtprel16HaOrTilegxImm16X0Hw0LastPltPcrelOrArmPlt32AbsOrPariscGprel16WfOrPpcGotDtprel16HaOrPpc64GotDtprel16HaOrIa64Segrel64Msb =
        0x5e,
    /// PpcTlsgd Or Ppc64Tprel16Ds Or TilegxImm16X1Hw0LastPltPcrel Or ArmGotAbs Or 16 bits GP-rel. address Or @segrel(sym + add), data8 LSB
    _PpcTlsgdOrPpc64Tprel16DsOrTilegxImm16X1Hw0LastPltPcrelOrArmGotAbsOrPariscGprel16DfOrIa64Segrel64Lsb =
        0x5f,
    /// PpcTlsld Or Ppc64Tprel16LoDs Or TilegxImm16X0Hw1LastPltPcrel Or ArmGotPrel Or 64 bits LT-rel. address
    _PpcTlsldOrPpc64Tprel16LoDsOrTilegxImm16X0Hw1LastPltPcrelOrArmGotPrelOrPariscLtoff64 = 0x60,
    _Ppc64Tprel16HigherOrTilegxImm16X1Hw1LastPltPcrelOrArmGotBrel12 = 0x61,
    _Ppc64Tprel16HigheraOrTilegxImm16X0Hw2LastPltPcrelOrArmGotoff12 = 0x62,
    /// Ppc64Tprel16Highest Or TilegxImm16X1Hw2LastPltPcrel Or ArmGotrelax Or LT-rel. address, right 14 bits
    _Ppc64Tprel16HighestOrTilegxImm16X1Hw2LastPltPcrelOrArmGotrelaxOrPariscLtoff14Wr = 0x63,
    /// Ppc64Tprel16Highesta Or Mips1626 Or TilegxImm16X0Hw0LastTlsIe Or ArmGnuVtentry Or LT-rel. address, right 14 bits Or @secrel(sym + add), data4 MSB
    _Ppc64Tprel16HighestaOrMips1626OrTilegxImm16X0Hw0LastTlsIeOrArmGnuVtentryOrPariscLtoff14DrOrIa64Secrel32Msb =
        0x64,
    /// PpcEmbNaddr32 Or Ppc64Dtprel16Ds Or Mips16Gprel Or TilegxImm16X1Hw0LastTlsIe Or ArmGnuVtinherit Or 16 bits LT-rel. address Or @secrel(sym + add), data4 LSB
    _PpcEmbNaddr32OrPpc64Dtprel16DsOrMips16GprelOrTilegxImm16X1Hw0LastTlsIeOrArmGnuVtinheritOrPariscLtoff16FOrIa64Secrel32Lsb =
        0x65,
    /// PpcEmbNaddr16 Or Ppc64Dtprel16LoDs Or Mips16Got16 Or TilegxImm16X0Hw1LastTlsIe Or ArmThmJump11 Or 16 bits LT-rel. address Or PC relative & 0xFFE (Thumb16 B) Or Nds32TlsTpoff Or @secrel(sym + add), data8 MSB
    _PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb =
        0x66,
    /// PpcEmbNaddr16Lo Or Ppc64Dtprel16Higher Or Mips16Call16 Or TilegxImm16X1Hw1LastTlsIe Or ArmThmJump8 Or 16 bits LT-rel. address Or ArmThmPc9 Or @secrel(sym + add), data8 LSB
    _PpcEmbNaddr16LoOrPpc64Dtprel16HigherOrMips16Call16OrTilegxImm16X1Hw1LastTlsIeOrArmThmJump8OrPariscLtoff16DfOrArmThmPc9OrIa64Secrel64Lsb =
        0x67,
    /// PpcEmbNaddr16Hi Or Ppc64Dtprel16Highera Or Mips16Hi16 Or ArmTlsGd32 Or 64 bits section rel. address
    _PpcEmbNaddr16HiOrPpc64Dtprel16HigheraOrMips16Hi16OrArmTlsGd32OrPariscSecrel64 = 0x68,
    _PpcEmbNaddr16HaOrPpc64Dtprel16HighestOrMips16Lo16OrArmTlsLdm32 = 0x69,
    _PpcEmbSdai16OrPpc64Dtprel16HighestaOrMips16TlsGdOrTilegxTlsDtpmod64OrArmTlsLdo32 = 0x6a,
    _PpcEmbSda2I16OrPpc64TlsgdOrMips16TlsLdmOrTilegxTlsDtpoff64OrArmTlsIe32 = 0x6b,
    /// PpcEmbSda2Rel Or Ppc64Tlsld Or Mips16TlsDtprelHi16 Or TilegxTlsTpoff64 Or ArmTlsLe32 Or data 4 + REL
    _PpcEmbSda2RelOrPpc64TlsldOrMips16TlsDtprelHi16OrTilegxTlsTpoff64OrArmTlsLe32OrIa64Rel32Msb =
        0x6c,
    /// PpcEmbSda21 Or Ppc64Tocsave Or Mips16TlsDtprelLo16 Or TilegxTlsDtpmod32 Or ArmTlsLdo12 Or data 4 + REL
    _PpcEmbSda21OrPpc64TocsaveOrMips16TlsDtprelLo16OrTilegxTlsDtpmod32OrArmTlsLdo12OrIa64Rel32Lsb =
        0x6d,
    /// PpcEmbMrkref Or Ppc64Addr16High Or Mips16TlsGottprel Or TilegxTlsDtpoff32 Or ArmTlsLe12 Or data 8 + REL
    _PpcEmbMrkrefOrPpc64Addr16HighOrMips16TlsGottprelOrTilegxTlsDtpoff32OrArmTlsLe12OrIa64Rel64Msb =
        0x6e,
    /// PpcEmbRelsec16 Or Ppc64Addr16Higha Or Mips16TlsTprelHi16 Or TilegxTlsTpoff32 Or ArmTlsIe12Gp Or data 8 + REL
    _PpcEmbRelsec16OrPpc64Addr16HighaOrMips16TlsTprelHi16OrTilegxTlsTpoff32OrArmTlsIe12GpOrIa64Rel64Lsb =
        0x6f,
    /// PpcEmbRelstLo Or Ppc64Tprel16High Or Mips16TlsTprelLo16 Or TilegxTlsGdCall Or ArmPrivate0 Or 64 bits segment rel. address
    _PpcEmbRelstLoOrPpc64Tprel16HighOrMips16TlsTprelLo16OrTilegxTlsGdCallOrArmPrivate0OrPariscSegrel64 =
        0x70,
    _PpcEmbRelstHiOrPpc64Tprel16HighaOrArmPrivate1OrTilegxImm8X0TlsGdAdd = 0x71,
    _PpcEmbRelstHaOrPpc64Dtprel16HighOrArmPrivate2OrTilegxImm8X1TlsGdAdd = 0x72,
    /// PpcEmbBitFld Or Ppc64Dtprel16Higha Or ArmPrivate3 Or TilegxImm8Y0TlsGdAdd Or PLT-rel. address, right 14 bits
    _PpcEmbBitFldOrPpc64Dtprel16HighaOrArmPrivate3OrTilegxImm8Y0TlsGdAddOrPariscPltoff14Wr = 0x73,
    /// PpcEmbRelsda Or Ppc64Rel24Notoc Or ArmPrivate4 Or TilegxImm8Y1TlsGdAdd Or PLT-rel. address, right 14 bits Or symbol + addend, data4 MSB
    _PpcEmbRelsdaOrPpc64Rel24NotocOrArmPrivate4OrTilegxImm8Y1TlsGdAddOrPariscPltoff14DrOrIa64Ltv32Msb =
        0x74,
    /// Ppc64Addr64Local Or ArmPrivate5 Or TilegxTlsIeLoad Or 16 bits LT-rel. address Or symbol + addend, data4 LSB
    _Ppc64Addr64LocalOrArmPrivate5OrTilegxTlsIeLoadOrPariscPltoff16FOrIa64Ltv32Lsb = 0x75,
    /// Ppc64Entry Or ArmPrivate6 Or TilegxImm8X0TlsAdd Or 16 bits PLT-rel. address Or symbol + addend, data8 MSB
    _Ppc64EntryOrArmPrivate6OrTilegxImm8X0TlsAddOrPariscPltoff16WfOrIa64Ltv64Msb = 0x76,
    /// PowerpcPltseq Or ArmPrivate7 Or TilegxImm8X1TlsAdd Or 16 bits PLT-rel. address Or Nds32TlsDesc Or symbol + addend, data8 LSB
    _PowerpcPltseqOrArmPrivate7OrTilegxImm8X1TlsAddOrPariscPltoff16DfOrNds32TlsDescOrIa64Ltv64Lsb =
        0x77,
    /// PowerpcPltcall Or ArmPrivate8 Or TilegxImm8Y0TlsAdd Or 64 bits LT-rel. function ptr
    _PowerpcPltcallOrArmPrivate8OrTilegxImm8Y0TlsAddOrPariscLtoffFptr64 = 0x78,
    /// Ppc64PltseqNotoc Or ArmPrivate9 Or TilegxImm8Y1TlsAdd Or @pcrel(sym + add), 21bit inst
    _Ppc64PltseqNotocOrArmPrivate9OrTilegxImm8Y1TlsAddOrIa64Pcrel21Bi = 0x79,
    /// Ppc64PltcallNotoc Or ArmPrivate10 Or @pcrel(sym + add), 22bit inst
    _Ppc64PltcallNotocOrArmPrivate10OrIa64Pcrel22 = 0x7a,
    /// Ppc64PcrelOpt Or ArmPrivate11 Or LT-rel. fct. ptr., right 14 bits Or @pcrel(sym + add), 64bit inst
    _Ppc64PcrelOptOrArmPrivate11OrPariscLtoffFptr14WrOrIa64Pcrel64I = 0x7b,
    /// Ppc64Rel24P9Notoc Or ArmPrivate12 Or LT-rel. fct. ptr., right 14 bits
    _Ppc64Rel24P9NotocOrArmPrivate12OrPariscLtoffFptr14Dr = 0x7c,
    /// Ppc64D34 Or TilegxGnuVtinherit Or ArmMeToo Or MipsNum Or PariscLoreserve Or Copy relocation Or GNU C++ vtable hierarchy Or dynamic reloc, imported PLT, MSB
    _Ppc64D34OrTilegxGnuVtinheritOrArmMeTooOrMipsNumOrPariscLoreserveOrPariscCopyOrTileproGnuVtinheritOrIa64Ipltmsb =
        0x80,
    /// Ppc64D34Lo Or TilegxGnuVtentry Or ArmThmTlsDescseq16 Or Dynamic reloc, imported PLT Or ArmThmTlsDescseq Or GNU C++ vtable member usage Or dynamic reloc, imported PLT, LSB
    _Ppc64D34LoOrTilegxGnuVtentryOrArmThmTlsDescseq16OrPariscIpltOrArmThmTlsDescseqOrTileproGnuVtentryOrIa64Ipltlsb =
        0x81,
    /// Ppc64D34Hi30 Or ArmThmTlsDescseq32 Or TilegxNum Or Dynamic reloc, exported PLT Or TileproNum
    _Ppc64D34Hi30OrArmThmTlsDescseq32OrTilegxNumOrPariscEpltOrTileproNum = 0x82,
    _Ppc64D34Ha30OrArmThmGotBrel12 = 0x83,
    /// Ppc64Pcrel34 Or copy relocation
    _Ppc64Pcrel34OrIa64Copy = 0x84,
    /// Ppc64GotPcrel34 Or Micromips26S1 Or Addend and symbol difference
    _Ppc64GotPcrel34OrMicromips26S1OrIa64Sub = 0x85,
    /// Ppc64PltPcrel34 Or MicromipsHi16 Or LTOFF22, relaxable
    _Ppc64PltPcrel34OrMicromipsHi16OrIa64Ltoff22X = 0x86,
    /// Ppc64PltPcrel34Notoc Or MicromipsLo16 Or Use of LTOFF22X
    _Ppc64PltPcrel34NotocOrMicromipsLo16OrIa64Ldxmov = 0x87,
    _Ppc64Addr16Higher34OrMicromipsGprel16OrArmThmBf16 = 0x88,
    _Ppc64Addr16Highera34OrMicromipsLiteralOrArmThmBf12 = 0x89,
    _Ppc64Addr16Highest34OrMicromipsGot16OrArmThmBf18 = 0x8a,
    _Ppc64Addr16Highesta34OrMicromipsPc7S1 = 0x8b,
    _Ppc64Rel16Higher34OrMicromipsPc10S1 = 0x8c,
    _Ppc64Rel16Highera34OrMicromipsPc16S1 = 0x8d,
    _Ppc64Rel16Highest34OrMicromipsCall16 = 0x8e,
    Ppc64Rel16Highesta34 = 0x8f,
    _Ppc64D28OrShTlsGd32 = 0x90,
    /// Ppc64Pcrel28 Or MicromipsGotDisp Or @tprel(sym + add), imm14 Or ShTlsLd32
    _Ppc64Pcrel28OrMicromipsGotDispOrIa64Tprel14OrShTlsLd32 = 0x91,
    /// Ppc64Tprel34 Or MicromipsGotPage Or @tprel(sym + add), imm22 Or ShTlsLdo32
    _Ppc64Tprel34OrMicromipsGotPageOrIa64Tprel22OrShTlsLdo32 = 0x92,
    /// Ppc64Dtprel34 Or MicromipsGotOfst Or @tprel(sym + add), imm64 Or ShTlsIe32
    _Ppc64Dtprel34OrMicromipsGotOfstOrIa64Tprel64IOrShTlsIe32 = 0x93,
    _Ppc64GotTlsgdPcrel34OrMicromipsGotHi16OrShTlsLe32 = 0x94,
    _Ppc64GotTlsldPcrel34OrMicromipsGotLo16OrShTlsDtpmod32 = 0x95,
    /// Ppc64GotTprelPcrel34 Or MicromipsSub Or @tprel(sym + add), data8 MSB Or ShTlsDtpoff32
    _Ppc64GotTprelPcrel34OrMicromipsSubOrIa64Tprel64MsbOrShTlsDtpoff32 = 0x96,
    /// Ppc64GotDtprelPcrel34 Or MicromipsHigher Or @tprel(sym + add), data8 LSB Or ShTlsTpoff32
    _Ppc64GotDtprelPcrel34OrMicromipsHigherOrIa64Tprel64LsbOrShTlsTpoff32 = 0x97,
    PpcVleRel15 = 0xd9,
    PpcVleRel24 = 0xda,
    /// PpcVleLo16A Or TP-rel. address, right 14 bits
    _PpcVleLo16AOrPariscTprel14Wr = 0xdb,
    /// PpcVleLo16D Or TP-rel. address, right 14 bits
    _PpcVleLo16DOrPariscTprel14Dr = 0xdc,
    /// PpcVleHi16A Or 16 bits TP-rel. address
    _PpcVleHi16AOrPariscTprel16F = 0xdd,
    /// PpcVleHi16D Or 16 bits TP-rel. address
    _PpcVleHi16DOrPariscTprel16Wf = 0xde,
    /// PpcVleHa16A Or 16 bits TP-rel. address
    _PpcVleHa16AOrPariscTprel16Df = 0xdf,
    /// PpcVleHa16D Or 64 bits LT-TP-rel. address
    _PpcVleHa16DOrPariscLtoffTp64 = 0xe0,
    PpcVleSda21 = 0xe1,
    PpcVleSda21Lo = 0xe2,
    /// PpcVleSdarelLo16A Or LT-TP-rel. address, right 14 bits
    _PpcVleSdarelLo16AOrPariscLtoffTp14Wr = 0xe3,
    /// PpcVleSdarelLo16D Or LT-TP-rel. address, right 14 bits
    _PpcVleSdarelLo16DOrPariscLtoffTp14Dr = 0xe4,
    /// PpcVleSdarelHi16A Or 16 bits LT-TP-rel. address
    _PpcVleSdarelHi16AOrPariscLtoffTp16F = 0xe5,
    /// PpcVleSdarelHi16D Or 16 bits LT-TP-rel. address
    _PpcVleSdarelHi16DOrPariscLtoffTp16Wf = 0xe6,
    /// PpcVleSdarelHa16A Or 16 bits LT-TP-rel. address
    _PpcVleSdarelHa16AOrPariscLtoffTp16Df = 0xe7,
    _PpcVleSdarelHa16DOrPariscGnuVtentry = 0xe8,
    /// Ppc64Rel16High Or LD offset 21-bit left
    _Ppc64Rel16HighOrPariscTlsLdo21L = 0xf0,
    /// Ppc64Rel16Higha Or LD offset 14-bit right
    _Ppc64Rel16HighaOrPariscTlsLdo14R = 0xf1,
    /// Ppc64Rel16Higher Or DTP module 32-bit
    _Ppc64Rel16HigherOrPariscTlsDtpmod32 = 0xf2,
    /// Ppc64Rel16Highera Or DTP module 64-bit
    _Ppc64Rel16HigheraOrPariscTlsDtpmod64 = 0xf3,
    /// Ppc64Rel16Highest Or DTP offset 32-bit
    _Ppc64Rel16HighestOrPariscTlsDtpoff32 = 0xf4,
    /// Ppc64Rel16Highesta Or DTP offset 32-bit
    _Ppc64Rel16HighestaOrPariscTlsDtpoff64 = 0xf5,
    PowerpcRel16DxHa = 0xf6,
    Ppc64JmpIrel = 0xf7,
    _PowerpcIrelativeOrMipsPc32OrSparcJmpIrelOrPpcIrelativeOrPpc64Irelative = 0xf8,
    /// PowerpcRel16 Or MipsEh Or SparcIrelative Or half16   (sym+add-.) Or half16   (sym+add-.) Or ArmRxpc25
    _PowerpcRel16OrMipsEhOrSparcIrelativeOrPpcRel16OrPpc64Rel16OrArmRxpc25 = 0xf9,
    /// PowerpcRel16Ha Or SparcRev32 Or half16   (sym+add-.)@ha Or half16   (sym+add-.)@ha Or ArmRrel32
    _PowerpcRel16HaOrSparcRev32OrPpcRel16HaOrPpc64Rel16HaOrArmRrel32 = 0xfc,
    _PowerpcGnuVtinheritOrMipsGnuVtinheritOrSparcNumOrArmRabs22 = 0xfd,
    _PowerpcGnuVtentryOrMipsGnuVtentryOrArmRpc24 = 0xfe,
    _PpcToc16OrPariscHireserveOrArmRbase = 0xff,
    /// MipsCopy Or ArmPrivate14 Or 16 bits LT-rel. function ptr
    _MipsCopyOrArmPrivate14OrPariscLtoffFptr16Wf = 0x7e,
    /// MipsJumpSlot Or ArmPrivate15 Or 16 bits LT-rel. function ptr
    _MipsJumpSlotOrArmPrivate15OrPariscLtoffFptr16Df = 0x7f,
    MicromipsHighest = 0x98,
    MicromipsScnDisp = 0x9b,
    MicromipsJalr = 0x9c,
    MicromipsHi0Lo16 = 0x9d,
    _MicromipsTlsLdmOrShGlobDat = 0xa3,
    _MicromipsTlsDtprelHi16OrShJmpSlot = 0xa4,
    _MicromipsTlsDtprelLo16OrShRelative = 0xa5,
    MicromipsTlsTprelHi16 = 0xa9,
    /// MicromipsTlsTprelLo16 Or @ltoff(@dtpmod(sym + add)), imm22
    _MicromipsTlsTprelLo16OrIa64LtoffDtpmod22 = 0xaa,
    MicromipsGprel7S2 = 0xac,
    MicromipsPc23S2 = 0xad,
    /// ArmPrivate13 Or 16 bits LT-rel. function ptr
    _ArmPrivate13OrPariscLtoffFptr16F = 0x7d,
    _ArmIrelativeOrShGot32 = 0xa0,
    I386UsedByIntel200 = 0xc8,
    /// AArch64Withdrawn Or ArmNum Or ShNum Or Keep this the last entry
    _AArch64WithdrawnOrArmNumOrShNumOrM32RNum = 0x100,
    AArch64Abs64 = 0x101,
    AArch64Abs32 = 0x102,
    AArch64Abs16 = 0x103,
    AArch64Prel64 = 0x104,
    AArch64Prel32 = 0x105,
    AArch64Prel16 = 0x106,
    AArch64MovwUabsG0 = 0x107,
    AArch64MovwUabsG0Nc = 0x108,
    AArch64MovwUabsG1 = 0x109,
    AArch64MovwUabsG1Nc = 0x10a,
    AArch64MovwUabsG2 = 0x10b,
    AArch64MovwUabsG2Nc = 0x10c,
    AArch64MovwUabsG3 = 0x10d,
    AArch64MovwSabsG0 = 0x10e,
    AArch64MovwSabsG1 = 0x10f,
    AArch64MovwSabsG2 = 0x110,
    AArch64LdPrelLo19 = 0x111,
    AArch64AdrPrelLo21 = 0x112,
    AArch64AdrPrelPgHi21 = 0x113,
    AArch64AdrPrelPgHi21Nc = 0x114,
    AArch64AddAbsLo12Nc = 0x115,
    AArch64Ldst8AbsLo12Nc = 0x116,
    AArch64Tstbr14 = 0x117,
    AArch64Condbr19 = 0x118,
    AArch64Jump26 = 0x11a,
    AArch64Call26 = 0x11b,
    AArch64Ldst16AbsLo12Nc = 0x11c,
    AArch64Ldst32AbsLo12Nc = 0x11d,
    AArch64Ldst64AbsLo12Nc = 0x11e,
    AArch64MovwPrelG0 = 0x11f,
    AArch64MovwPrelG0Nc = 0x120,
    AArch64MovwPrelG1 = 0x121,
    AArch64MovwPrelG1Nc = 0x122,
    AArch64MovwPrelG2 = 0x123,
    AArch64MovwPrelG2Nc = 0x124,
    AArch64MovwPrelG3 = 0x125,
    AArch64Ldst128AbsLo12Nc = 0x12b,
    AArch64MovwGotoffG0 = 0x12c,
    AArch64MovwGotoffG0Nc = 0x12d,
    AArch64MovwGotoffG1 = 0x12e,
    AArch64MovwGotoffG1Nc = 0x12f,
    AArch64MovwGotoffG2 = 0x130,
    AArch64MovwGotoffG2Nc = 0x131,
    AArch64MovwGotoffG3 = 0x132,
    AArch64Gotrel64 = 0x133,
    AArch64Gotrel32 = 0x134,
    AArch64GotLdPrel19 = 0x135,
    AArch64Ld64GotoffLo15 = 0x136,
    AArch64AdrGotPage = 0x137,
    AArch64Ld64GotLo12Nc = 0x138,
    AArch64Ld64GotpageLo15 = 0x139,
    AArch64TlsgdAdrPrel21 = 0x200,
    AArch64TlsgdAdrPage21 = 0x201,
    AArch64TlsgdAddLo12Nc = 0x202,
    AArch64TlsgdMovwG1 = 0x203,
    AArch64TlsgdMovwG0Nc = 0x204,
    AArch64TlsldAdrPrel21 = 0x205,
    AArch64TlsldAdrPage21 = 0x206,
    AArch64TlsldAddLo12Nc = 0x207,
    AArch64TlsldMovwG1 = 0x208,
    AArch64TlsldMovwG0Nc = 0x209,
    AArch64TlsldLdPrel19 = 0x20a,
    AArch64TlsldMovwDtprelG2 = 0x20b,
    AArch64TlsldMovwDtprelG1 = 0x20c,
    AArch64TlsldMovwDtprelG1Nc = 0x20d,
    AArch64TlsldMovwDtprelG0 = 0x20e,
    AArch64TlsldMovwDtprelG0Nc = 0x20f,
    AArch64TlsldAddDtprelHi12 = 0x210,
    AArch64TlsldAddDtprelLo12 = 0x211,
    AArch64TlsldAddDtprelLo12Nc = 0x212,
    AArch64TlsldLdst8DtprelLo12 = 0x213,
    AArch64TlsldLdst8DtprelLo12Nc = 0x214,
    AArch64TlsldLdst16DtprelLo12 = 0x215,
    AArch64TlsldLdst16DtprelLo12Nc = 0x216,
    AArch64TlsldLdst32DtprelLo12 = 0x217,
    AArch64TlsldLdst32DtprelLo12Nc = 0x218,
    AArch64TlsldLdst64DtprelLo12 = 0x219,
    AArch64TlsldLdst64DtprelLo12Nc = 0x21a,
    AArch64TlsieMovwGottprelG1 = 0x21b,
    AArch64TlsieMovwGottprelG0Nc = 0x21c,
    AArch64TlsieAdrGottprelPage21 = 0x21d,
    AArch64TlsieLd64GottprelLo12Nc = 0x21e,
    AArch64TlsieLdGottprelPrel19 = 0x21f,
    AArch64TlsleMovwTprelG2 = 0x220,
    AArch64TlsleMovwTprelG1 = 0x221,
    AArch64TlsleMovwTprelG1Nc = 0x222,
    AArch64TlsleMovwTprelG0 = 0x223,
    AArch64TlsleMovwTprelG0Nc = 0x224,
    AArch64TlsleAddTprelHi12 = 0x225,
    AArch64TlsleAddTprelLo12 = 0x226,
    AArch64TlsleAddTprelLo12Nc = 0x227,
    AArch64TlsleLdst8TprelLo12 = 0x228,
    AArch64TlsleLdst8TprelLo12Nc = 0x229,
    AArch64TlsleLdst16TprelLo12 = 0x22a,
    AArch64TlsleLdst16TprelLo12Nc = 0x22b,
    AArch64TlsleLdst32TprelLo12 = 0x22c,
    AArch64TlsleLdst32TprelLo12Nc = 0x22d,
    AArch64TlsleLdst64TprelLo12 = 0x22e,
    AArch64TlsleLdst64TprelLo12Nc = 0x22f,
    AArch64TlsdescLdPrel19 = 0x230,
    AArch64TlsdescAdrPrel21 = 0x231,
    AArch64TlsdescAdrPage21 = 0x232,
    AArch64TlsdescLd64Lo12 = 0x233,
    AArch64TlsdescAddLo12 = 0x234,
    AArch64TlsdescOffG1 = 0x235,
    AArch64TlsdescOffG0Nc = 0x236,
    AArch64TlsdescLdr = 0x237,
    AArch64TlsdescAdd = 0x238,
    AArch64TlsdescCall = 0x239,
    AArch64TlsleLdst128TprelLo12 = 0x23a,
    AArch64TlsleLdst128TprelLo12Nc = 0x23b,
    AArch64TlsldLdst128DtprelLo12 = 0x23c,
    AArch64TlsldLdst128DtprelLo12Nc = 0x23d,
    AArch64Copy = 0x400,
    AArch64GlobDat = 0x401,
    AArch64JumpSlot = 0x402,
    AArch64Relative = 0x403,
    /// AArch64TlsDtpmod64 Or Module number, 64 bit
    _AArch64TlsDtpmod64OrAArch64TlsDtpmod = 0x404,
    /// AArch64TlsDtprel64 Or Module-relative offset, 64 bit
    _AArch64TlsDtprel64OrAArch64TlsDtprel = 0x405,
    /// AArch64TlsTprel64 Or TP-relative offset, 64 bit
    _AArch64TlsTprel64OrAArch64TlsTprel = 0x406,
    AArch64Tlsdesc = 0x407,
    AArch64Irelative = 0x408,
    /// 14 bits LT-TP-rel. address Or ShGotpc Or @dtpmod(sym + add), data8 LSB
    _PariscLtoffTp14FOrShGotpcOrIa64Dtpmod64Lsb = 0xa7,
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
    /// like EMB_SDA21, but lower 16 bit Or Copy symbol at runtime Or @dtprel(sym + add), data4 MSB
    _PpcDiabSda21LoOrAArch64P32CopyOrIa64Dtprel32Msb = 0xb4,
    /// like EMB_SDA21, but high 16 bit Or Create GOT entry Or @dtprel(sym + add), data4 LSB
    _PpcDiabSda21HiOrAArch64P32GlobDatOrIa64Dtprel32Lsb = 0xb5,
    /// like EMB_SDA21, adjusted high 16 Or Create PLT entry Or @dtprel(sym + add), data8 MSB
    _PpcDiabSda21HaOrAArch64P32JumpSlotOrIa64Dtprel64Msb = 0xb6,
    /// like EMB_RELSDA, but lower 16 bit Or Adjust by program base Or @dtprel(sym + add), data8 LSB
    _PpcDiabRelsdaLoOrAArch64P32RelativeOrIa64Dtprel64Lsb = 0xb7,
    /// like EMB_RELSDA, but high 16 bit Or Module number, 32 bit
    _PpcDiabRelsdaHiOrAArch64P32TlsDtpmod = 0xb8,
    /// like EMB_RELSDA, adjusted high 16 Or Module-relative offset, 32 bit
    _PpcDiabRelsdaHaOrAArch64P32TlsDtprel = 0xb9,
    /// TP-relative offset, 32 bit Or @ltoff(@dtprel(s+a)), imm22
    _AArch64P32TlsTprelOrIa64LtoffDtprel22 = 0xba,
    /// TLS Descriptor.
    AArch64P32Tlsdesc = 0xbb,
    /// STT_GNU_IFUNC relocation.
    AArch64P32Irelative = 0xbc,
    /// @dtprel(sym + add), imm14
    Ia64Dtprel14 = 0xb1,
    /// @dtprel(sym + add), imm22
    Ia64Dtprel22 = 0xb2,
    /// @dtprel(sym + add), imm64
    Ia64Dtprel64I = 0xb3,
    ShPlt32 = 0xa1,
}
impl RelocationType {
    pub const FOO_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const POWERPC_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const MIPS_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const TILEGX_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const S390_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const X86_64_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const SPARC_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const ARM_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const I386_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const AARCH64_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc
    pub const M68K_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc.
    pub const PARISC_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc
    pub const ALPHA_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const PPC_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// no reloc
    pub const CKCORE_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const SH_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const CRIS_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc.
    pub const MN10300_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc.
    pub const M32R_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc.
    pub const MICROBLAZE_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc.
    pub const NIOS2_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc
    pub const TILEPRO_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const RISCV_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc
    pub const BPF_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const METAG_HIADDR16 : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const NDS32_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const OR1K_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const PPC64_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// none
    pub const IA64_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const ARC_NONE : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrS390NoneOrX8664NoneOrSparcNoneOrArmNoneOrI386NoneOrAArch64NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrCkcoreNoneOrShNoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    pub const FOO_32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    pub const POWERPC_ADDR32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    pub const MIPS_16 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    pub const TILEGX_64 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    pub const S390_8 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    pub const X86_64_64 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    pub const SPARC_8 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    pub const ARM_PC24 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    pub const I386_32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// Direct 32 bit
    pub const M68K_32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// Direct 32-bit reference.
    pub const PARISC_DIR32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// Direct 32 bit
    pub const ALPHA_REFLONG : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// 32bit absolute address
    pub const PPC_ADDR32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// Direct 32 bit.
    pub const AARCH64_P32_ABS32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// direct 32 bit (S + A)
    pub const CKCORE_ADDR32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    pub const SH_DIR32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    pub const CRIS_8 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// Direct 32 bit.
    pub const MN10300_32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// Direct 16 bit.
    pub const M32R_16 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// Direct 32 bit.
    pub const MICROBLAZE_32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// Direct signed 16 bit.
    pub const NIOS2_S16 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// Direct 32 bit
    pub const TILEPRO_32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    pub const RISCV_32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    pub const BPF_64_64 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    pub const METAG_LOADDR16 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    pub const OR1K_32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// 32bit absolute address
    pub const PPC64_ADDR32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    pub const ARC_8 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    pub const FOO_ILLEGAL : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    pub const POWERPC_ADDR14_BRNTAKEN : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    pub const TILEGX_HW0 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    pub const S390_COPY : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    pub const MIPS_GOT16 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    pub const X86_64_GOTPCREL : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    pub const SPARC_HI22 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    pub const ARM_SBREL32 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    pub const I386_GOTOFF : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    /// 8 bit PC relative GOT entry
    pub const M68K_GOT8 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    /// 32-bit rel. address.
    pub const PARISC_PCREL32 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    /// PC relative 16 bit
    pub const ALPHA_SREL16 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    pub const PPC_ADDR14_BRNTAKEN : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    /// 32 bit adjust program base(B + A)
    pub const CKCORE_RELATIVE : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    pub const SH_DIR8L : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    pub const CRIS_COPY : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    /// Direct 24 bit.
    pub const MN10300_24 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    /// Low 16 bit.
    pub const M32R_LO16 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    /// No reloc.
    pub const MICROBLAZE_64_NONE : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    /// High 16 bit.
    pub const NIOS2_HI16 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    /// High 16 bit, adjusted
    pub const TILEPRO_HA16 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    pub const RISCV_TLS_DTPREL64 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    pub const METAG_REG16OP1 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    pub const OR1K_32_PCREL : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    pub const PPC64_ADDR14_BRNTAKEN : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    pub const ARC_N16 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    pub const PARISC_TLS_LE21L: Self =
        Self::_PariscTlsLe21LOrMicromipsCallLo16OrPariscTprel21LOrIa64LtoffTprel22;
    pub const MICROMIPS_CALL_LO16: Self =
        Self::_PariscTlsLe21LOrMicromipsCallLo16OrPariscTprel21LOrIa64LtoffTprel22;
    /// TP-rel. address, left 21 bits.
    pub const PARISC_TPREL21L: Self =
        Self::_PariscTlsLe21LOrMicromipsCallLo16OrPariscTprel21LOrIa64LtoffTprel22;
    /// @ltoff(@tprel(s+a)), imm2
    pub const IA64_LTOFF_TPREL22: Self =
        Self::_PariscTlsLe21LOrMicromipsCallLo16OrPariscTprel21LOrIa64LtoffTprel22;
    pub const PARISC_TLS_LE14R: Self = Self::_PariscTlsLe14ROrPariscTprel14R;
    /// TP-rel. address, right 14 bits.
    pub const PARISC_TPREL14R: Self = Self::_PariscTlsLe14ROrPariscTprel14R;
    pub const PARISC_TLS_IE21L: Self =
        Self::_PariscTlsIe21LOrMicromipsTlsGdOrPariscLtoffTp21LOrShCopy;
    pub const MICROMIPS_TLS_GD: Self =
        Self::_PariscTlsIe21LOrMicromipsTlsGdOrPariscLtoffTp21LOrShCopy;
    /// LT-TP-rel. address, left 21 bits.
    pub const PARISC_LTOFF_TP21L: Self =
        Self::_PariscTlsIe21LOrMicromipsTlsGdOrPariscLtoffTp21LOrShCopy;
    pub const SH_COPY: Self = Self::_PariscTlsIe21LOrMicromipsTlsGdOrPariscLtoffTp21LOrShCopy;
    pub const PARISC_TLS_IE14R: Self =
        Self::_PariscTlsIe14ROrMicromipsTlsGottprelOrPariscLtoffTp14ROrShGotoffOrIa64Dtpmod64Msb;
    pub const MICROMIPS_TLS_GOTTPREL: Self =
        Self::_PariscTlsIe14ROrMicromipsTlsGottprelOrPariscLtoffTp14ROrShGotoffOrIa64Dtpmod64Msb;
    /// LT-TP-rel. address, right 14 bits.
    pub const PARISC_LTOFF_TP14R: Self =
        Self::_PariscTlsIe14ROrMicromipsTlsGottprelOrPariscLtoffTp14ROrShGotoffOrIa64Dtpmod64Msb;
    pub const SH_GOTOFF: Self =
        Self::_PariscTlsIe14ROrMicromipsTlsGottprelOrPariscLtoffTp14ROrShGotoffOrIa64Dtpmod64Msb;
    /// @dtpmod(sym + add), data8 MSB
    pub const IA64_DTPMOD64MSB: Self =
        Self::_PariscTlsIe14ROrMicromipsTlsGottprelOrPariscLtoffTp14ROrShGotoffOrIa64Dtpmod64Msb;
    pub const PARISC_TLS_TPREL32: Self = Self::_PariscTlsTprel32OrMicromipsCallHi16OrPariscTprel32;
    pub const MICROMIPS_CALL_HI16: Self = Self::_PariscTlsTprel32OrMicromipsCallHi16OrPariscTprel32;
    /// 32 bits TP-rel. address.
    pub const PARISC_TPREL32: Self = Self::_PariscTlsTprel32OrMicromipsCallHi16OrPariscTprel32;
    pub const PARISC_TLS_TPREL64: Self = Self::_PariscTlsTprel64OrPpcVleRel8OrPariscTprel64;
    pub const PPC_VLE_REL8: Self = Self::_PariscTlsTprel64OrPpcVleRel8OrPariscTprel64;
    /// 64 bits TP-rel. address.
    pub const PARISC_TPREL64: Self = Self::_PariscTlsTprel64OrPpcVleRel8OrPariscTprel64;
    pub const S390_12 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    pub const POWERPC_ADDR24 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    pub const TILEGX_32 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    pub const MIPS_32 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    pub const X86_64_PC32 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    pub const SPARC_16 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    pub const ARM_ABS32 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    pub const I386_PC32 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// Direct 16 bit
    pub const M68K_16 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// Left 21 bits of eff. address.
    pub const PARISC_DIR21L : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// Direct 64 bit
    pub const ALPHA_REFQUAD : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// 26bit address, 2 bits ignored.
    pub const PPC_ADDR24 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// disp ((S + A - P) >> 2) & 0xff
    pub const CKCORE_PCRELIMM8BY4 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    pub const SH_REL32 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    pub const CRIS_16 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// Direct 16 bit.
    pub const MN10300_16 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// Direct 32 bit.
    pub const M32R_32 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// PC relative 32 bit.
    pub const MICROBLAZE_32_PCREL : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// Direct unsigned 16 bit.
    pub const NIOS2_U16 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// Direct 16 bit
    pub const TILEPRO_16 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    pub const RISCV_64 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// 32bit absolute address
    pub const METAG_ADDR32 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    pub const OR1K_16 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// 26bit address, word aligned
    pub const PPC64_ADDR24 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    pub const ARC_16 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    pub const S390_16 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    pub const POWERPC_ADDR16 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    pub const TILEGX_16 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    pub const MIPS_REL32 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    pub const X86_64_GOT32 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    pub const SPARC_32 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    pub const ARM_REL32 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    pub const I386_GOT32 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// Direct 8 bit
    pub const M68K_8 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// Right 17 bits of eff. address.
    pub const PARISC_DIR17R : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// GP relative 32 bit
    pub const ALPHA_GPREL32 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// 16bit absolute address
    pub const PPC_ADDR16 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// disp ((S + A - P) >> 1) & 0x7ff
    pub const CKCORE_PCRELIMM11BY2 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    pub const SH_DIR8WPN : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    pub const CRIS_32 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// Direct 8 bit.
    pub const MN10300_8 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// Direct 24 bit.
    pub const M32R_24 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// PC relative 64 bit.
    pub const MICROBLAZE_64_PCREL : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// PC relative 16 bit.
    pub const NIOS2_PCREL16 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// Direct 8 bit
    pub const TILEPRO_8 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    pub const RISCV_RELATIVE : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// No reloc
    pub const METAG_NONE : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    pub const OR1K_8 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// 16bit absolute address
    pub const PPC64_ADDR16 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    pub const ARC_24 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    pub const S390_32 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    pub const POWERPC_ADDR16_LO : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    pub const MIPS_26 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    pub const TILEGX_8 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    pub const X86_64_PLT32 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    pub const SPARC_DISP8 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    pub const ARM_LDR_PC_G0 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    pub const I386_PLT32 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// PC relative 32 bit
    pub const M68K_PC32 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// 17 bits of eff. address.
    pub const PARISC_DIR17F : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// GP relative 16 bit w/optimization
    pub const ALPHA_LITERAL : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// lower 16bit of absolute address
    pub const PPC_ADDR16_LO : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    pub const ARM_PC13 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    pub const SH_IND12W : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    pub const CRIS_8_PCREL : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// PC-relative 32-bit.
    pub const MN10300_PCREL32 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// PC relative 10 bit shifted.
    pub const M32R_10_PCREL : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// Low 16 bits of PCREL32.
    pub const MICROBLAZE_32_PCREL_LO : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// Direct call.
    pub const NIOS2_CALL26 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// PC relative 32 bit
    pub const TILEPRO_32_PCREL : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    pub const RISCV_COPY : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    pub const METAG_RELBRANCH : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    pub const OR1K_LO_16_IN_INSN : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// lower 16bits of address
    pub const PPC64_ADDR16_LO : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    pub const ARC_32 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    pub const S390_PC32 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    pub const POWERPC_ADDR16_HI : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    pub const MIPS_HI16 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    pub const TILEGX_64_PCREL : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    pub const X86_64_COPY : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    pub const SPARC_DISP16 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    pub const ARM_ABS16 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    pub const I386_COPY : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// PC relative 16 bit
    pub const M68K_PC16 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// Optimization hint for LITERAL
    pub const ALPHA_LITUSE : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// high 16bit of absolute address
    pub const PPC_ADDR16_HI : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// 32-bit rel (S + A - P)
    pub const CKCORE_PCREL32 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    pub const SH_DIR8WPL : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    pub const CRIS_16_PCREL : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// PC-relative 16-bit signed.
    pub const MN10300_PCREL16 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// PC relative 18 bit shifted.
    pub const M32R_18_PCREL : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// Direct 64 bit.
    pub const MICROBLAZE_64 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// 5 bit constant expression.
    pub const NIOS2_IMM5 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// PC relative 16 bit
    pub const TILEPRO_16_PCREL : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    pub const RISCV_JUMP_SLOT : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    pub const METAG_GETSETOFF : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    pub const OR1K_HI_16_IN_INSN : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// high 16bits of address.
    pub const PPC64_ADDR16_HI : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    pub const ARC_B26 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    pub const S390_GOT12 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    pub const POWERPC_ADDR16_HA : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    pub const MIPS_LO16 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    pub const TILEGX_32_PCREL : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    pub const X86_64_GLOB_DAT : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    pub const SPARC_DISP32 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    pub const ARM_ABS12 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    pub const I386_GLOB_DAT : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// PC relative 8 bit
    pub const M68K_PC8 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// Right 14 bits of eff. address.
    pub const PARISC_DIR14R : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// Add displacement to GP
    pub const ALPHA_GPDISP : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// adjusted high 16bit
    pub const PPC_ADDR16_HA : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// disp ((S + A - P) >>1) & 0x7ff
    pub const CKCORE_PCRELJSR_IMM11BY2 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    pub const SH_DIR8WPZ : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    pub const CRIS_32_PCREL : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// PC-relative 8-bit signed.
    pub const MN10300_PCREL8 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// PC relative 26 bit shifted.
    pub const M32R_26_PCREL : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// Low 16 bit.
    pub const MICROBLAZE_32_LO : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// 5 bit expression, shift 22.
    pub const NIOS2_CACHE_OPX : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// PC relative 8 bit
    pub const TILEPRO_8_PCREL : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    pub const RISCV_TLS_DTPMOD32 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    pub const METAG_REG32OP1 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    pub const OR1K_INSN_REL_26 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// adjusted high 16bits.
    pub const PPC64_ADDR16_HA : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    pub const ARC_B22_PCREL : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    pub const S390_GOT32 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    pub const POWERPC_ADDR14 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    pub const TILEGX_16_PCREL : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    pub const MIPS_GPREL16 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    pub const X86_64_JUMP_SLOT : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    pub const SPARC_WDISP30 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    pub const ARM_THM_ABS5 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    pub const I386_JUMP_SLOT : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// 32 bit PC relative GOT entry
    pub const M68K_GOT32 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// Create PLT entry
    pub const I386_JMP_SLOT : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// PC+4 relative 23 bit shifted
    pub const ALPHA_BRADDR : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// 16bit address, 2 bits ignored
    pub const PPC_ADDR14 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    pub const SH_DIR8BP : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    pub const CRIS_GNU_VTINHERIT : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// Ancient C++ vtable garbage...
    pub const MN10300_GNU_VTINHERIT : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// High 16 bit with unsigned low.
    pub const M32R_HI16_ULO : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// Read-only small data area.
    pub const MICROBLAZE_SRO32 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// 6 bit constant expression.
    pub const NIOS2_IMM6 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// Low 16 bit
    pub const TILEPRO_LO16 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    pub const RISCV_TLS_DTPMOD64 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    pub const METAG_REG32OP2 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    pub const OR1K_GNU_VTENTRY : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// 16bit address, word aligned
    pub const PPC64_ADDR14 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    pub const ARC_H30 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    pub const S390_PLT32 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    pub const POWERPC_ADDR14_BRTAKEN : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    pub const MIPS_LITERAL : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    pub const TILEGX_8_PCREL : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    pub const X86_64_RELATIVE : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    pub const SPARC_WDISP22 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    pub const ARM_ABS8 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    pub const I386_RELATIVE : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    /// 16 bit PC relative GOT entry
    pub const M68K_GOT16 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    /// PC+4 relative 16 bit shifted
    pub const ALPHA_HINT : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    pub const PPC_ADDR14_BRTAKEN : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    pub const SH_DIR8W : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    pub const CRIS_GNU_VTENTRY : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    /// ... collection annotation.
    pub const MN10300_GNU_VTENTRY : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    /// High 16 bit with signed low.
    pub const M32R_HI16_SLO : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    /// Read-write small data area.
    pub const MICROBLAZE_SRW32 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    /// 8 bit constant expression.
    pub const NIOS2_IMM8 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    /// High 16 bit
    pub const TILEPRO_HI16 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    pub const RISCV_TLS_DTPREL32 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    pub const METAG_REG32OP3 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    pub const OR1K_GNU_VTINHERIT : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    pub const PPC64_ADDR14_BRTAKEN : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    pub const ARC_N8 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    pub const S390_GLOB_DAT : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    pub const POWERPC_REL24 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    pub const MIPS_PC16 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    pub const TILEGX_HW1 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    pub const X86_64_32 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    pub const SPARC_22 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    pub const ARM_THM_CALL : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    pub const I386_GOTPC : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// 32 bit GOT offset
    pub const M68K_GOT32O : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// Left 21 bits of rel. address.
    pub const PARISC_PCREL21L : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// PC relative 32 bit
    pub const ALPHA_SREL32 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// PC relative 26 bit
    pub const PPC_REL24 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// PC relative 24 bit (Thumb32 BL).
    pub const ARM_THM_PC22 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// 32 bit adjust by program base
    pub const CKCORE_COPY : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    pub const CRIS_GLOB_DAT : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// 32-bit PCrel offset to GOT.
    pub const MN10300_GOTPC32 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// 16 bit offset in SDA.
    pub const M32R_SDA16 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// Symbol Op Symbol relocation.
    pub const MICROBLAZE_32_SYM_OP_SYM : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// Low 16 bit.
    pub const NIOS2_LO16 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// Copy relocation
    pub const TILEPRO_COPY : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    pub const RISCV_TLS_TPREL32 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    pub const BPF_64_32 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    pub const METAG_REG16OP2 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    pub const OR1K_16_PCREL : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// PC-rel. 26 bit, word aligned
    pub const PPC64_REL24 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    pub const ARC_N24 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    pub const S390_JMP_SLOT : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    pub const POWERPC_REL14 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    pub const TILEGX_HW2 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    pub const I386_32PLT : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    pub const MIPS_CALL16 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    pub const X86_64_32S : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    pub const SPARC_13 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    pub const ARM_THM_PC8 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// 16 bit GOT offset
    pub const M68K_GOT16O : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// Right 17 bits of rel. address.
    pub const PARISC_PCREL17R : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// PC relative 64 bit
    pub const ALPHA_SREL64 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// PC relative 16 bit
    pub const PPC_REL14 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// off between got and sym (S)
    pub const CKCORE_GLOB_DAT : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    pub const CRIS_JUMP_SLOT : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// 16-bit PCrel offset to GOT.
    pub const MN10300_GOTPC16 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    pub const M32R_GNU_VTINHERIT : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// GNU C++ vtable hierarchy.
    pub const MICROBLAZE_GNU_VTINHERIT : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// High 16 bit, adjusted.
    pub const NIOS2_HIADJ16 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// Create GOT entry
    pub const TILEPRO_GLOB_DAT : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    pub const RISCV_TLS_TPREL64 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    pub const METAG_REG16OP3 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    pub const OR1K_8_PCREL : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// PC relative 16 bit
    pub const PPC64_REL14 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    pub const ARC_N32 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrI38632PltOrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    pub const S390_RELATIVE : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    pub const POWERPC_REL14_BRTAKEN : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    pub const MIPS_GPREL32 : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    pub const TILEGX_HW3 : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    pub const X86_64_16 : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    pub const SPARC_LO10 : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    pub const ARM_BREL_ADJ : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    /// 8 bit GOT offset
    pub const M68K_GOT8O : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    /// 17 bits of rel. address.
    pub const PARISC_PCREL17F : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    pub const PPC_REL14_BRTAKEN : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    pub const ARM_AMP_VCALL9 : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    /// PLT entry (S)
    pub const CKCORE_JUMP_SLOT : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    pub const CRIS_RELATIVE : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    /// 32-bit offset from GOT.
    pub const MN10300_GOTOFF32 : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    pub const M32R_GNU_VTENTRY : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    /// GNU C++ vtable member usage.
    pub const MICROBLAZE_GNU_VTENTRY : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    /// 32 bit symbol value + addend.
    pub const NIOS2_BFD_RELOC_32 : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    /// Create PLT entry
    pub const TILEPRO_JMP_SLOT : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    pub const METAG_REG32OP4 : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    pub const OR1K_GOTPC_HI16 : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    pub const PPC64_REL14_BRTAKEN : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    pub const ARC_SDA : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    pub const S390_GOTOFF32 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    pub const POWERPC_REL14_BRNTAKEN : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    pub const MIPS_UNUSED1 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    pub const TILEGX_HW0_LAST : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    pub const X86_64_PC16 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    pub const SPARC_GOT10 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    pub const ARM_TLS_DESC : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    /// 32 bit PC relative PLT address
    pub const M68K_PLT32 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    pub const PPC_REL14_BRNTAKEN : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    /// Obsolete static relocation.
    pub const ARM_SWI24 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    /// offset to GOT (S + A - GOT)
    pub const CKCORE_GOTOFF : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    pub const CRIS_16_GOT : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    /// 24-bit offset from GOT.
    pub const MN10300_GOTOFF24 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    /// PC-relative GOT offset.
    pub const MICROBLAZE_GOTPC_64 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    /// 16 bit symbol value + addend.
    pub const NIOS2_BFD_RELOC_16 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    /// Adjust by program base
    pub const TILEPRO_RELATIVE : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    pub const METAG_HIOG : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    pub const OR1K_GOTPC_LO16 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    pub const PPC64_REL14_BRNTAKEN : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    pub const ARC_SECTOFF : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    pub const S390_GOTPC : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    pub const POWERPC_GOT16 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    pub const MIPS_UNUSED2 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    pub const TILEGX_HW1_LAST : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    pub const X86_64_8 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    pub const SPARC_GOT13 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    pub const ARM_THM_SWI8 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    pub const I386_TLS_TPOFF : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    /// 16 bit PC relative PLT address
    pub const M68K_PLT16 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    /// Right 14 bits of rel. address.
    pub const PARISC_PCREL14R : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    pub const PPC_GOT16 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    /// PC offset to GOT (GOT + A - P)
    pub const CKCORE_GOTPC : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    pub const CRIS_32_GOT : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    /// 16-bit offset from GOT.
    pub const MN10300_GOTOFF16 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    /// GOT entry offset.
    pub const MICROBLAZE_GOT_64 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    /// 8 bit symbol value + addend.
    pub const NIOS2_BFD_RELOC_8 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    /// X1 pipe branch offset
    pub const TILEPRO_BROFF_X1 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    pub const METAG_LOOG : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    pub const OR1K_GOT16 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    pub const PPC64_GOT16 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    pub const ARC_S21H_PCREL : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrI386TlsTpoffOrM68KPlt16OrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    pub const S390_GOT16 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    pub const POWERPC_GOT16_LO : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    pub const MIPS_UNUSED3 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    pub const TILEGX_HW2_LAST : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    pub const X86_64_PC8 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    pub const SPARC_GOT22 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    pub const ARM_XPC25 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    pub const I386_TLS_IE : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    /// 8 bit PC relative PLT address
    pub const M68K_PLT8 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    pub const PPC_GOT16_LO : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    /// 32 bit GOT entry (G)
    pub const CKCORE_GOT32 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    pub const CRIS_16_GOTPLT : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    /// 32-bit PCrel to PLT entry.
    pub const MN10300_PLT32 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    /// PLT offset (PC-relative).
    pub const MICROBLAZE_PLT_64 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    /// 16 bit GP pointer offset.
    pub const NIOS2_GPREL : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    /// X1 pipe jump offset
    pub const TILEPRO_JOFFLONG_X1 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    pub const METAG_REL8 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    pub const OR1K_PLT26 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    pub const PPC64_GOT16_LO : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    pub const ARC_S21W_PCREL : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    pub const S390_PC16 : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    pub const POWERPC_GOT16_HI : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    pub const MIPS_SHIFT5 : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    pub const TILEGX_COPY : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    pub const X86_64_DTPMOD64 : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    pub const SPARC_PC10 : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    pub const ARM_THM_XPC22 : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    pub const I386_TLS_GOTIE : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    /// 32 bit PLT offset
    pub const M68K_PLT32O : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    pub const PPC_GOT16_HI : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    /// 32 bit PLT entry (G)
    pub const CKCORE_PLT32 : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    pub const CRIS_32_GOTPLT : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    /// 16-bit PCrel to PLT entry.
    pub const MN10300_PLT16 : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    /// Adjust by program base.
    pub const MICROBLAZE_REL : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    /// GNU C++ vtable hierarchy.
    pub const NIOS2_GNU_VTINHERIT : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    /// X1 pipe jump offset to PLT
    pub const TILEPRO_JOFFLONG_X1_PLT : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    pub const RISCV_BRANCH : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    pub const METAG_REL16 : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    pub const OR1K_GOTOFF_HI16 : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    pub const PPC64_GOT16_HI : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    pub const ARC_S25H_PCREL : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrX8664Dtpmod64OrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    pub const S390_PC16DBL : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    pub const POWERPC_GOT16_HA : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    pub const MIPS_SHIFT6 : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    pub const TILEGX_GLOB_DAT : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    pub const X86_64_DTPOFF64 : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    pub const SPARC_PC22 : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    pub const ARM_TLS_DTPMOD32 : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    pub const I386_TLS_LE : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    /// 16 bit PLT offset
    pub const M68K_PLT16O : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    /// GP relative 32 bit, high 16 bits
    pub const ALPHA_GPRELHIGH : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    pub const PPC_GOT16_HA : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    /// GOT entry in GLOB_DAT (GOT + G)
    pub const CKCORE_ADDRGOT : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    pub const CRIS_32_GOTREL : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    /// 32-bit offset to GOT entry.
    pub const MN10300_GOT32 : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    /// Create PLT entry.
    pub const MICROBLAZE_JUMP_SLOT : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    /// GNU C++ vtable member usage.
    pub const NIOS2_GNU_VTENTRY : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    /// X0 pipe 8-bit
    pub const TILEPRO_IMM8_X0 : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    pub const RISCV_JAL : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    pub const OR1K_GOTOFF_LO16 : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    pub const PPC64_GOT16_HA : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    pub const ARC_S25W_PCREL : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    pub const S390_PLT16DBL : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    pub const PPC_PLTREL24 : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    pub const MIPS_64 : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    pub const TILEGX_JMP_SLOT : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    pub const X86_64_TPOFF64 : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    pub const SPARC_WPLT30 : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    pub const ARM_TLS_DTPOFF32 : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    pub const I386_TLS_GD : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    /// 8 bit PLT offset
    pub const M68K_PLT8O : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    /// Left 21 bits of rel. address.
    pub const PARISC_DPREL21L : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    /// GP relative 32 bit, low 16 bits
    pub const ALPHA_GPRELLOW : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    /// PLT entry in GLOB_DAT (GOT + G)
    pub const CKCORE_ADDRPLT : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    pub const CRIS_32_PLT_GOTREL : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    /// 24-bit offset to GOT entry.
    pub const MN10300_GOT24 : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    /// Create GOT entry.
    pub const MICROBLAZE_GLOB_DAT : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    /// Unconditional branch.
    pub const NIOS2_UJMP : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    /// Y0 pipe 8-bit
    pub const TILEPRO_IMM8_Y0 : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    pub const RISCV_CALL : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    pub const OR1K_COPY : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    pub const ARC_SDA32 : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    pub const S390_PC32DBL : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    pub const POWERPC_COPY : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    pub const MIPS_GOT_DISP : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    pub const TILEGX_RELATIVE : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    pub const X86_64_TLSGD : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    pub const SPARC_COPY : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    pub const ARM_TLS_TPOFF32 : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    pub const I386_TLS_LDM : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    /// Copy symbol at runtime
    pub const M68K_COPY : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    /// GP relative 16 bit
    pub const ALPHA_GPREL16 : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    pub const PPC_COPY : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    /// ((S + A - P) >> 1) & 0x3ffffff
    pub const CKCORE_PCREL_IMM26BY2 : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    pub const CRIS_32_PLT_PCREL : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    /// 16-bit offset to GOT entry.
    pub const MN10300_GOT16 : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    /// 64 bit offset to GOT.
    pub const MICROBLAZE_GOTOFF_64 : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    /// Conditional branch.
    pub const NIOS2_CJMP : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    /// X1 pipe 8-bit
    pub const TILEPRO_IMM8_X1 : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    pub const RISCV_CALL_PLT : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    pub const OR1K_GLOB_DAT : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    pub const PPC64_COPY : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    pub const ARC_SDA_LDST : Self = Self::_S390Pc32DblOrPowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    pub const S390_PLT32DBL : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    pub const POWERPC_GLOB_DAT : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    pub const MIPS_GOT_PAGE : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    pub const TILEGX_BROFF_X1 : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    pub const X86_64_TLSLD : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    pub const SPARC_GLOB_DAT : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    pub const ARM_COPY : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    pub const I386_16 : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    /// Create GOT entry
    pub const M68K_GLOB_DAT : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    pub const PPC_GLOB_DAT : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    /// disp ((S + A - P) >> 1) & 0xffff
    pub const CKCORE_PCREL_IMM16BY2 : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    pub const CRIS_NUM : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    /// Copy symbol at runtime.
    pub const MN10300_COPY : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    /// 32 bit offset to GOT.
    pub const MICROBLAZE_GOTOFF_32 : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    /// Indirect call through register.
    pub const NIOS2_CALLR : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    /// Y1 pipe 8-bit
    pub const TILEPRO_IMM8_Y1 : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    pub const RISCV_GOT_HI20 : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    pub const NDS32_32_RELA : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    pub const OR1K_JMP_SLOT : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    pub const PPC64_GLOB_DAT : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    pub const ARC_SDA_LDST1 : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrX8664TlsldOrSparcGlobDatOrArmCopyOrI38616OrM68KGlobDatOrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    pub const S390_GOTPCDBL : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    pub const POWERPC_JMP_SLOT : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    pub const MIPS_GOT_OFST : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    pub const TILEGX_JUMPOFF_X1 : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    pub const X86_64_DTPOFF32 : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    pub const SPARC_JMP_SLOT : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    pub const ARM_GLOB_DAT : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    pub const I386_PC16 : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    /// Create PLT entry
    pub const M68K_JMP_SLOT : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    pub const PPC_JMP_SLOT : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    /// disp ((S + A - P) >> 2) & 0xffff
    pub const CKCORE_PCREL_IMM16BY4 : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    /// Create GOT entry.
    pub const MN10300_GLOB_DAT : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    /// Runtime copy.
    pub const MICROBLAZE_COPY : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    pub const NIOS2_ALIGN : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    /// X1 pipe mtspr
    pub const TILEPRO_MT_IMM15_X1 : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    pub const RISCV_TLS_GOT_HI20 : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    pub const OR1K_RELATIVE : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    pub const PPC64_JMP_SLOT : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    pub const ARC_SDA_LDST2 : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrX8664Dtpoff32OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    pub const S390_64 : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    pub const POWERPC_RELATIVE : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    pub const MIPS_GOT_HI16 : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    pub const TILEGX_JUMPOFF_X1_PLT : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    pub const X86_64_GOTTPOFF : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    pub const SPARC_RELATIVE : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    pub const ARM_JUMP_SLOT : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    pub const I386_8 : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    /// Adjust by program base
    pub const M68K_RELATIVE : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    /// Right 14 bits of rel. address.
    pub const PARISC_DPREL14R : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    pub const PPC_RELATIVE : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    /// disp ((S + A - P) >> 1) & 0x3ff
    pub const CKCORE_PCREL_IMM10BY2 : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    /// Create PLT entry.
    pub const MN10300_JMP_SLOT : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    /// TLS Reloc.
    pub const MICROBLAZE_TLS : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    /// 16 bit GOT entry.
    pub const NIOS2_GOT16 : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    /// X1 pipe mfspr
    pub const TILEPRO_MF_IMM15_X1 : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    pub const RISCV_TLS_GD_HI20 : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    pub const OR1K_TLS_GD_HI16 : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    pub const PPC64_RELATIVE : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    pub const ARC_SDA16_LD : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    pub const S390_PC64 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrX8664Tpoff32OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    pub const PPC_LOCAL24PC : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrX8664Tpoff32OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    pub const MIPS_GOT_LO16 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrX8664Tpoff32OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    pub const TILEGX_IMM8_X0 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrX8664Tpoff32OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    pub const X86_64_TPOFF32 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrX8664Tpoff32OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    pub const SPARC_UA32 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrX8664Tpoff32OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    pub const ARM_RELATIVE : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrX8664Tpoff32OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    pub const I386_PC8 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrX8664Tpoff32OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    /// disp ((S + A - P) >> 2) & 0x3ff
    pub const CKCORE_PCREL_IMM10BY4 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrX8664Tpoff32OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    /// Adjust by program base.
    pub const MN10300_RELATIVE : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrX8664Tpoff32OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    /// TLS General Dynamic.
    pub const MICROBLAZE_TLSGD : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrX8664Tpoff32OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    /// 16 bit GOT entry for function.
    pub const NIOS2_CALL16 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrX8664Tpoff32OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    /// X0 pipe 16-bit
    pub const TILEPRO_IMM16_X0 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrX8664Tpoff32OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    pub const RISCV_PCREL_HI20 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrX8664Tpoff32OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    pub const OR1K_TLS_GD_LO16 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrX8664Tpoff32OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    pub const ARC_SDA16_LD1 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrX8664Tpoff32OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    pub const S390_GOT64 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    pub const POWERPC_UADDR32 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    pub const MIPS_SUB : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    pub const TILEGX_IMM8_Y0 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    pub const X86_64_PC64 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    pub const SPARC_PLT32 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    pub const ARM_GOTOFF32 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    pub const I386_TLS_GD_32 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    /// Copy symbol at runtime
    pub const ALPHA_COPY : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    pub const PPC_UADDR32 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    /// 32 bit offset to GOT
    pub const ARM_GOTOFF : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    /// high & low 16 bit ADDR
    pub const CKCORE_ADDR_HI16 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    /// 32-bit offset for global dynamic.
    pub const MN10300_TLS_GD : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    /// TLS Local Dynamic.
    pub const MICROBLAZE_TLSLD : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    /// %lo of offset to GOT pointer.
    pub const NIOS2_GOTOFF_LO : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    /// X1 pipe 16-bit
    pub const TILEPRO_IMM16_X1 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    pub const RISCV_PCREL_LO12_I : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    pub const OR1K_TLS_LDM_HI16 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    pub const PPC64_UADDR32 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    pub const ARC_SDA16_LD2 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    pub const S390_PLT64 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    pub const POWERPC_UADDR16 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    pub const MIPS_INSERT_A : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    pub const TILEGX_IMM8_X1 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    pub const X86_64_GOTOFF64 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    pub const SPARC_HIPLT22 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    pub const ARM_BASE_PREL : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    pub const I386_TLS_GD_PUSH : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    /// 32 bit GOT offset for GD
    pub const M68K_TLS_GD32 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    /// Create GOT entry
    pub const ALPHA_GLOB_DAT : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    pub const PPC_UADDR16 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    /// 32 bit PC relative offset to GOT
    pub const ARM_GOTPC : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    /// (S + A) & 0xffff
    pub const CKCORE_ADDR_LO16 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    pub const SH_SWITCH16 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    /// 32-bit offset for local dynamic.
    pub const MN10300_TLS_LD : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    /// TLS Module ID.
    pub const MICROBLAZE_TLSDTPMOD32 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    /// %hiadj of offset to GOT pointer.
    pub const NIOS2_GOTOFF_HA : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    /// X0 pipe low 16-bit
    pub const TILEPRO_IMM16_X0_LO : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    pub const RISCV_PCREL_LO12_S : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    pub const OR1K_TLS_LDM_LO16 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    pub const PPC64_UADDR16 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    pub const ARC_S13_PCREL : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    pub const S390_GOTENT : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    pub const POWERPC_REL32 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    pub const MIPS_INSERT_B : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    pub const TILEGX_IMM8_Y1 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    pub const X86_64_GOTPC32 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    pub const SPARC_LOPLT10 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    pub const ARM_GOT_BREL : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    pub const I386_TLS_GD_CALL : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    /// 16 bit GOT offset for GD
    pub const M68K_TLS_GD16 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    /// GP-relative, left 21 bits.
    pub const PARISC_GPREL21L : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    /// Create PLT entry
    pub const ALPHA_JMP_SLOT : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    pub const PPC_REL32 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    /// 32 bit GOT entry
    pub const ARM_GOT32 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    /// high & low 16 bit GOTPC
    pub const CKCORE_GOTPC_HI16 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    pub const SH_SWITCH32 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    /// Module-relative offset.
    pub const MN10300_TLS_LDO : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    /// TLS Offset Within TLS Block.
    pub const MICROBLAZE_TLSDTPREL32 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    /// %lo of PC relative offset.
    pub const NIOS2_PCREL_LO : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    /// X1 pipe low 16-bit
    pub const TILEPRO_IMM16_X1_LO : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    pub const RISCV_HI20 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    pub const OR1K_TLS_LDO_HI16 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    pub const PPC64_REL32 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    pub const ARC_W : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    pub const S390_GOTOFF16 : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    pub const POWERPC_PLT32 : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    pub const MIPS_DELETE : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    pub const TILEGX_DEST_IMM8_X1 : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    pub const X86_64_GOT64 : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    pub const SPARC_PCPLT32 : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    pub const ARM_PLT32 : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    pub const I386_TLS_GD_POP : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    /// 8 bit GOT offset for GD
    pub const M68K_TLS_GD8 : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    /// Adjust by program base
    pub const ALPHA_RELATIVE : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    pub const PPC_PLT32 : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    /// (GOT + A - P) & 0xffff
    pub const CKCORE_GOTPC_LO16 : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    pub const SH_USES : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    pub const MN10300_TLS_GOTIE : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    /// TLS Offset Within TLS Block.
    pub const MICROBLAZE_TLSDTPREL64 : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    /// %hiadj of PC relative offset.
    pub const NIOS2_PCREL_HA : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    /// X0 pipe high 16-bit
    pub const TILEPRO_IMM16_X0_HI : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    pub const RISCV_LO12_I : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    pub const OR1K_TLS_LDO_LO16 : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    pub const PPC64_PLT32 : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    pub const ARC_32_ME : Self = Self::_S390Gotoff16OrPowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    pub const S390_GOTOFF64 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    pub const POWERPC_PLTREL32 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    pub const MIPS_HIGHER : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    pub const TILEGX_MT_IMM14_X1 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    pub const X86_64_GOTPCREL64 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    pub const SPARC_PCPLT22 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    pub const ARM_CALL : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    pub const I386_TLS_LDM_32 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    /// 32 bit GOT offset for LDM
    pub const M68K_TLS_LDM32 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    pub const ALPHA_TLS_GD_HI : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    pub const PPC_PLTREL32 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    /// high & low 16 bit GOTOFF
    pub const CKCORE_GOTOFF_HI16 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    pub const SH_COUNT : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    pub const MN10300_TLS_IE : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    /// TLS Offset From Thread Pointer.
    pub const MICROBLAZE_TLSGOTTPREL32 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    /// 16 bit GOT offset for TLS GD.
    pub const NIOS2_TLS_GD16 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    /// X1 pipe high 16-bit
    pub const TILEPRO_IMM16_X1_HI : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    pub const RISCV_LO12_S : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    pub const OR1K_TLS_IE_HI16 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    pub const PPC64_PLTREL32 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    pub const ARC_N32_ME : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    pub const S390_GOTPLT12 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    pub const POWERPC_PLT16_LO : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    pub const MIPS_HIGHEST : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    pub const TILEGX_MF_IMM14_X1 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    pub const X86_64_GOTPC64 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    pub const SPARC_PCPLT10 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    pub const ARM_JUMP24 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    pub const I386_TLS_LDM_PUSH : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    /// 16 bit GOT offset for LDM
    pub const M68K_TLS_LDM16 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    pub const ALPHA_TLSGD : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    pub const PPC_PLT16_LO : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    /// (S + A - GOT) & 0xffff
    pub const CKCORE_GOTOFF_LO16 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    pub const SH_ALIGN : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    pub const MN10300_TLS_LE : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    /// TLS Offset From Thread Pointer.
    pub const MICROBLAZE_TLSTPREL32 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    /// 16 bit GOT offset for TLS LDM.
    pub const NIOS2_TLS_LDM16 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    /// X0 pipe high 16-bit, adjusted
    pub const TILEPRO_IMM16_X0_HA : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    pub const RISCV_TPREL_HI20 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    pub const OR1K_TLS_IE_LO16 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    pub const PPC64_PLT16_LO : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    pub const ARC_SECTOFF_ME : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    pub const S390_GOTPLT16 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    pub const POWERPC_PLT16_HI : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    pub const MIPS_CALL_HI16 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    pub const TILEGX_MMSTART_X0 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    pub const X86_64_GOTPLT64 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    pub const SPARC_10 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    pub const ARM_THM_JUMP24 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    pub const I386_TLS_LDM_CALL : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    /// 8 bit GOT offset for LDM
    pub const M68K_TLS_LDM8 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    /// GP-relative, right 14 bits.
    pub const PARISC_GPREL14R : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    pub const ALPHA_TLS_LDM : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    pub const PPC_PLT16_HI : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    /// 12 bit disp GOT entry (G)
    pub const CKCORE_GOT12 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    pub const SH_CODE : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    /// ID of module containing symbol.
    pub const MN10300_TLS_DTPMOD : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    /// 16 bit module relative offset.
    pub const NIOS2_TLS_LDO16 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    /// X1 pipe high 16-bit, adjusted
    pub const TILEPRO_IMM16_X1_HA : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    pub const RISCV_TPREL_LO12_I : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    pub const METAG_GNU_VTINHERIT : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    pub const OR1K_TLS_LE_HI16 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    pub const PPC64_PLT16_HI : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    pub const ARC_SDA32_ME : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    pub const S390_GOTPLT32 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    pub const POWERPC_PLT16_HA : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    pub const MIPS_CALL_LO16 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    pub const TILEGX_MMEND_X0 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    pub const X86_64_PLTOFF64 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    pub const SPARC_11 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    pub const ARM_BASE_ABS : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    pub const I386_TLS_LDM_POP : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    /// 32 bit module-relative offset
    pub const M68K_TLS_LDO32 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    pub const ALPHA_DTPMOD64 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    pub const PPC_PLT16_HA : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    /// high & low 16 bit GOT
    pub const CKCORE_GOT_HI16 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    pub const SH_DATA : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    /// Offset in module TLS block.
    pub const MN10300_TLS_DTPOFF : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    /// 16 bit GOT offset for TLS IE.
    pub const NIOS2_TLS_IE16 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    /// X0 pipe PC relative 16 bit
    pub const TILEPRO_IMM16_X0_PCREL : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    pub const RISCV_TPREL_LO12_S : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    pub const METAG_GNU_VTENTRY : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    pub const OR1K_TLS_LE_LO16 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    pub const PPC64_PLT16_HA : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    pub const ARC_W_ME : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    pub const S390_GOTPLT64 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    pub const PPC_SDAREL16 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    pub const MIPS_SCN_DISP : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    pub const X86_64_SIZE32 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    pub const TILEGX_SHAMT_X0 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    pub const SPARC_64 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    pub const ARM_ALU_PCREL_7_0 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    pub const I386_TLS_LDO_32 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    /// 16 bit module-relative offset
    pub const M68K_TLS_LDO16 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    pub const ALPHA_GOTDTPREL : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    /// (G & 0xffff)
    pub const CKCORE_GOT_LO16 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    pub const SH_LABEL : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    /// Offset in static TLS block.
    pub const MN10300_TLS_TPOFF : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    /// 16 bit LE TP-relative offset.
    pub const NIOS2_TLS_LE16 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    /// X1 pipe PC relative 16 bit
    pub const TILEPRO_IMM16_X1_PCREL : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    pub const RISCV_TPREL_ADD : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    pub const METAG_HI16_GOTOFF : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    pub const OR1K_TLS_TPOFF : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    pub const ARC_H30_ME : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    pub const S390_GOTPLTENT : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    pub const POWERPC_SECTOFF : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    pub const MIPS_REL16 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    pub const X86_64_SIZE64 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    pub const TILEGX_SHAMT_X1 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    pub const SPARC_OLO10 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    pub const ARM_ALU_PCREL_15_8 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    pub const I386_TLS_IE_32 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    /// 8 bit module-relative offset
    pub const M68K_TLS_LDO8 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    pub const ALPHA_DTPREL64 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    pub const PPC_SECTOFF : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    /// 12 bit disp PLT entry (G)
    pub const CKCORE_PLT12 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    pub const SH_SWITCH8 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    pub const MN10300_SYM_DIFF : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    /// Direct 16 bit.
    pub const M32R_16_RELA : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    /// Module number.
    pub const NIOS2_TLS_DTPMOD : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    /// X0 pipe PC relative low 16 bit
    pub const TILEPRO_IMM16_X0_LO_PCREL : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    pub const RISCV_ADD8 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    pub const METAG_LO16_GOTOFF : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    pub const OR1K_TLS_DTPOFF : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    pub const PPC64_SECTOFF : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    /// symbol + addend, add imm14
    pub const IA64_IMM14 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    pub const ARC_SECTOFF_U8 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    pub const S390_PLTOFF16 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    pub const POWERPC_SECTOFF_LO : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    pub const MIPS_ADD_IMMEDIATE : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    pub const TILEGX_SHAMT_Y0 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    pub const X86_64_GOTPC32_TLSDESC : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    pub const SPARC_HH22 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    pub const ARM_ALU_PCREL_23_15 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    pub const I386_TLS_LE_32 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    /// 32 bit GOT offset for IE
    pub const M68K_TLS_IE32 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    /// LT-relative, left 21 bits.
    pub const PARISC_LTOFF21L : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    pub const ALPHA_DTPRELHI : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    pub const PPC_SECTOFF_LO : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    /// high & low 16 bit PLT
    pub const CKCORE_PLT_HI16 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    pub const SH_GNU_VTINHERIT : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    pub const MN10300_ALIGN : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    /// Direct 32 bit.
    pub const M32R_32_RELA : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    /// Module-relative offset.
    pub const NIOS2_TLS_DTPREL : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    /// X1 pipe PC relative low 16 bit
    pub const TILEPRO_IMM16_X1_LO_PCREL : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    pub const RISCV_ADD16 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    pub const METAG_GETSET_GOTOFF : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    pub const OR1K_TLS_DTPMOD : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    pub const PPC64_SECTOFF_LO : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    /// symbol + addend, add imm22
    pub const IA64_IMM22 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    pub const ARC_SECTOFF_S9 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    pub const S390_PLTOFF32 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    pub const POWERPC_SECTOFF_HI : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    pub const MIPS_PJUMP : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    pub const TILEGX_SHAMT_Y1 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    pub const X86_64_TLSDESC_CALL : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    pub const SPARC_HM10 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    pub const ARM_LDR_SBREL_11_0_NC : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    pub const I386_TLS_DTPMOD32 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    /// 16 bit GOT offset for IE
    pub const M68K_TLS_IE16 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    pub const ALPHA_DTPRELLO : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    pub const PPC_SECTOFF_HI : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    /// Deprecated, prog. base relative.
    pub const ARM_LDR_SBREL_11_0 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    /// G & 0xffff
    pub const CKCORE_PLT_LO16 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    pub const SH_GNU_VTENTRY : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    pub const MN10300_NUM : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    /// Direct 24 bit.
    pub const M32R_24_RELA : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    /// TP-relative offset.
    pub const NIOS2_TLS_TPREL : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    /// X0 pipe PC relative high 16 bit
    pub const TILEPRO_IMM16_X0_HI_PCREL : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    pub const RISCV_ADD32 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    pub const METAG_GETSET_GOT : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    pub const PPC64_SECTOFF_HI : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    /// symbol + addend, mov imm64
    pub const IA64_IMM64 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    pub const AC_SECTOFF_U8 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    pub const S390_PLTOFF64 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    pub const POWERPC_SECTOFF_HA : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    pub const MIPS_RELGOT : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    pub const TILEGX_IMM16_X0_HW0 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    pub const X86_64_TLSDESC : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    pub const SPARC_LM22 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    pub const ARM_ALU_SBREL_19_12_NC : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    pub const I386_TLS_DTPOFF32 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    /// 8 bit GOT offset for IE
    pub const M68K_TLS_IE8 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    pub const ALPHA_DTPREL16 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    pub const PPC_SECTOFF_HA : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    /// Deprecated, prog. base relative.
    pub const ARM_ALU_SBREL_19_12 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    /// high & low 16 bit ADDRGOT
    pub const CKCORE_ADDRGOT_HI16 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    /// PC relative 10 bit shifted.
    pub const M32R_10_PCREL_RELA : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    /// Copy symbol at runtime.
    pub const NIOS2_COPY : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    /// X1 pipe PC relative high 16 bit
    pub const TILEPRO_IMM16_X1_HI_PCREL : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    pub const RISCV_ADD64 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    pub const METAG_HI16_GOTPC : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    pub const PPC64_SECTOFF_HA : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    /// symbol + addend, data4 MSB
    pub const IA64_DIR32MSB : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    pub const AC_SECTOFF_U8_1 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    pub const S390_TLS_LOAD : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    pub const POWERPC_ADDR30 : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    pub const MIPS_JALR : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    pub const TILEGX_IMM16_X1_HW0 : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    pub const X86_64_IRELATIVE : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    pub const SPARC_PC_HH22 : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    pub const ARM_ALU_SBREL_27_20_CK : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    pub const I386_TLS_TPOFF32 : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    pub const M68K_TLS_LE32 : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    pub const ALPHA_GOTTPREL : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    /// word30 (S + A - P) >> 2
    pub const PPC64_ADDR30 : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    /// Deprecated, prog. base relative.
    pub const ARM_ALU_SBREL_27_20 : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    /// (GOT + G * 4) & 0xffff
    pub const CKCORE_ADDRGOT_LO16 : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    /// PC relative 18 bit shifted.
    pub const M32R_18_PCREL_RELA : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    /// Create GOT entry.
    pub const NIOS2_GLOB_DAT : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    /// X0 pipe PC relative ha() 16 bit
    pub const TILEPRO_IMM16_X0_HA_PCREL : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    pub const RISCV_SUB8 : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    pub const METAG_LO16_GOTPC : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    /// symbol + addend, data4 LSB
    pub const IA64_DIR32LSB : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    pub const AC_SECTOFF_U8_2 : Self = Self::_S390TlsLoadOrPowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    pub const S390_TLS_GDCALL : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    pub const PPC64_ADDR64 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    pub const MIPS_TLS_DTPMOD32 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    pub const TILEGX_IMM16_X0_HW1 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    pub const X86_64_RELATIVE64 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    pub const SPARC_PC_HM10 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    pub const ARM_TARGET1 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    pub const M68K_TLS_LE16 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    /// 32-bit symbol size
    pub const I386_SIZE32 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    /// LT-relative, right 14 bits.
    pub const PARISC_LTOFF14R : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    pub const ALPHA_TPREL64 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    /// high & low 16 bit ADDRPLT
    pub const CKCORE_ADDRPLT_HI16 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    /// PC relative 26 bit shifted.
    pub const M32R_26_PCREL_RELA : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    /// Create PLT entry.
    pub const NIOS2_JUMP_SLOT : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    /// X1 pipe PC relative ha() 16 bit
    pub const TILEPRO_IMM16_X1_HA_PCREL : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    pub const RISCV_SUB16 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    pub const METAG_HI16_PLT : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    /// symbol + addend, data8 MSB
    pub const IA64_DIR64MSB : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    pub const AC_SECTOFF_S9 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrMipsTlsDtpmod32OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    pub const S390_TLS_LDCALL : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    pub const PPC64_ADDR16_HIGHER : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    pub const MIPS_TLS_DTPREL32 : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    pub const TILEGX_IMM16_X1_HW1 : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    pub const X86_64_PC32_BND : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    pub const SPARC_PC_LM22 : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    pub const ARM_SBREL31 : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    pub const I386_TLS_GOTDESC : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    pub const M68K_TLS_LE8 : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    pub const ALPHA_TPRELHI : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    /// (GOT+G*4) & 0xffff
    pub const CKCORE_ADDRPLT_LO16 : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    /// High 16 bit with unsigned low
    pub const M32R_HI16_ULO_RELA : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    /// Adjust by program base.
    pub const NIOS2_RELATIVE : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    /// X0 pipe 16-bit GOT offset
    pub const TILEPRO_IMM16_X0_GOT : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    pub const RISCV_SUB32 : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    pub const METAG_LO16_PLT : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    pub const NDS32_COPY : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    /// symbol + addend, data8 LSB
    pub const IA64_DIR64LSB : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    pub const AC_SECTOFF_S9_1 : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrArmSbrel31OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    pub const S390_TLS_GD32 : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    pub const PPC64_ADDR16_HIGHERA : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    pub const MIPS_TLS_DTPMOD64 : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    pub const TILEGX_IMM16_X0_HW2 : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    pub const X86_64_PLT32_BND : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    pub const SPARC_WDISP16 : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    pub const ARM_V4BX : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    pub const I386_TLS_DESC_CALL : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    /// 32 bit module number
    pub const M68K_TLS_DTPMOD32 : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    pub const ALPHA_TPRELLO : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    /// disp ((S+A-P) >>1) & x3ffffff
    pub const CKCORE_PCREL_JSR_IMM26BY2 : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    /// High 16 bit with signed low
    pub const M32R_HI16_SLO_RELA : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    /// 16 bit offset to GOT pointer.
    pub const NIOS2_GOTOFF : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    /// X1 pipe 16-bit GOT offset
    pub const TILEPRO_IMM16_X1_GOT : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    pub const RISCV_SUB64 : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    pub const METAG_RELBRANCH_PLT : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    pub const NDS32_GLOB_DAT : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    pub const AC_SECTOFF_S9_2 : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    pub const S390_TLS_GD64 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    pub const PPC64_ADDR16_HIGHEST : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    pub const MIPS_TLS_DTPREL64 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    pub const TILEGX_IMM16_X1_HW2 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    pub const X86_64_GOTPCRELX : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    pub const SPARC_WDISP19 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    pub const ARM_TARGET2 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    pub const I386_TLS_DESC : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    /// 32 bit module-relative offset
    pub const M68K_TLS_DTPREL32 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    /// 32 bits section rel. address.
    pub const PARISC_SECREL32 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    pub const ALPHA_TPREL16 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    /// (S+A-BTEXT) & 0xffff
    pub const CKCORE_TOFFSET_LO16 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    /// Low 16 bit
    pub const M32R_LO16_RELA : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    /// Direct call in .noat section.
    pub const NIOS2_CALL26_NOAT : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    /// X0 pipe low 16-bit GOT offset
    pub const TILEPRO_IMM16_X0_GOT_LO : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    pub const RISCV_GNU_VTINHERIT : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    pub const METAG_GOTOFF : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    pub const NDS32_JMP_SLOT : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    pub const ARC_SECTOFF_ME_1 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    pub const S390_TLS_GOTIE12 : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    pub const PPC64_ADDR16_HIGHESTA : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    pub const MIPS_TLS_GD : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    pub const TILEGX_IMM16_X0_HW3 : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    pub const X86_64_REX_GOTPCRELX : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    pub const SPARC_GLOB_JMP : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    pub const ARM_PREL31 : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    pub const I386_IRELATIVE : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    /// 32 bit TP-relative offset
    pub const M68K_TLS_TPREL32 : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    /// (S+A-BTEXT) & 0xffff
    pub const CKCORE_DOFFSET_LO16 : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    /// 16 bit offset in SDA
    pub const M32R_SDA16_RELA : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    /// %lo() of GOT entry.
    pub const NIOS2_GOT_LO : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    /// X1 pipe low 16-bit GOT offset
    pub const TILEPRO_IMM16_X1_GOT_LO : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    pub const RISCV_GNU_VTENTRY : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    pub const METAG_PLT : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    pub const NDS32_RELATIVE : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    /// @gprel(sym + add), add imm22
    pub const IA64_GPREL22 : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    pub const ARC_SECTOFF_ME_2 : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrX8664RexGotpcrelxOrSparcGlobJmpOrArmPrel31OrI386IrelativeOrM68KTlsTprel32OrCkcoreDoffsetLo16OrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    pub const S390_TLS_GOTIE32 : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    pub const PPC64_UADDR64 : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    pub const MIPS_TLS_LDM : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    pub const TILEGX_IMM16_X1_HW3 : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    pub const SPARC_7 : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    pub const ARM_MOVW_ABS_NC : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    pub const I386_GOT32X : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    pub const M68K_NUM : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    /// disp ((S+A-P) >>1) & 0x3ffff
    pub const CKCORE_PCREL_IMM18BY2 : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    pub const X86_64_NUM : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    pub const M32R_RELA_GNU_VTINHERIT : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    /// %hiadj() of GOT entry.
    pub const NIOS2_GOT_HA : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    /// X0 pipe high 16-bit GOT offset
    pub const TILEPRO_IMM16_X0_GOT_HI : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    pub const RISCV_ALIGN : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    pub const METAG_COPY : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    /// @gprel(sym + add), mov imm64
    pub const IA64_GPREL64I : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    pub const ARC_SECTOFF_1 : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    pub const S390_TLS_GOTIE64 : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    pub const PPC64_REL64 : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    pub const MIPS_TLS_DTPREL_HI16 : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    pub const TILEGX_IMM16_X0_HW0_LAST : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    pub const SPARC_5 : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    pub const ARM_MOVT_ABS : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    pub const I386_NUM : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    /// disp (S+A-BDATA) & 0x3ffff
    pub const CKCORE_DOFFSET_IMM18 : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    pub const M32R_RELA_GNU_VTENTRY : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    /// %lo() of function GOT entry.
    pub const NIOS2_CALL_LO : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    /// X1 pipe high 16-bit GOT offset
    pub const TILEPRO_IMM16_X1_GOT_HI : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    pub const RISCV_RVC_BRANCH : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    pub const METAG_JMP_SLOT : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    /// @gprel(sym + add), data4 MSB
    pub const IA64_GPREL32MSB : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    pub const ARC_SECTOFF_2 : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    pub const S390_TLS_LDM32 : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    pub const PPC64_PLT64 : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    pub const MIPS_TLS_DTPREL_LO16 : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    pub const TILEGX_IMM16_X1_HW0_LAST : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    pub const SPARC_6 : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    pub const ARM_MOVW_PREL_NC : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    /// disp ((S+A-BDATA)>>1) & 0x3ffff
    pub const CKCORE_DOFFSET_IMM18BY2 : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    /// PC relative 32 bit.
    pub const M32R_REL32 : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    /// %hiadj() of function GOT entry.
    pub const NIOS2_CALL_HA : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    /// X0 pipe ha() 16-bit GOT offset
    pub const TILEPRO_IMM16_X0_GOT_HA : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    pub const RISCV_RVC_JUMP : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    pub const METAG_RELATIVE : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    /// @gprel(sym + add), data4 LSB
    pub const IA64_GPREL32LSB : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    pub const S390_TLS_LDM64 : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    pub const PPC64_PLTREL64 : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    pub const MIPS_TLS_GOTTPREL : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    pub const TILEGX_IMM16_X0_HW1_LAST : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    pub const SPARC_DISP64 : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    pub const ARM_MOVT_PREL : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    pub const ALPHA_NUM : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    /// disp ((S+A-BDATA)>>2) & 0x3ffff
    pub const CKCORE_DOFFSET_IMM18BY4 : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    /// X1 pipe ha() 16-bit GOT offset
    pub const TILEPRO_IMM16_X1_GOT_HA : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    pub const RISCV_RVC_LUI : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    pub const METAG_GLOB_DAT : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    /// @gprel(sym + add), data8 MSB
    pub const IA64_GPREL64MSB : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    pub const S390_TLS_IE32 : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGdOrIa64Gprel64Lsb;
    pub const PPC64_TOC16 : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGdOrIa64Gprel64Lsb;
    pub const MIPS_TLS_TPREL32 : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGdOrIa64Gprel64Lsb;
    pub const TILEGX_IMM16_X1_HW1_LAST : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGdOrIa64Gprel64Lsb;
    pub const SPARC_PLT64 : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGdOrIa64Gprel64Lsb;
    pub const ARM_THM_MOVW_ABS_NC : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGdOrIa64Gprel64Lsb;
    /// X0 pipe mm "start"
    pub const TILEPRO_MMSTART_X0 : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGdOrIa64Gprel64Lsb;
    pub const RISCV_GPREL_I : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGdOrIa64Gprel64Lsb;
    pub const METAG_TLS_GD : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGdOrIa64Gprel64Lsb;
    /// @gprel(sym + add), data8 LSB
    pub const IA64_GPREL64LSB : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGdOrIa64Gprel64Lsb;
    pub const S390_TLS_IE64 : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    pub const PPC64_TOC16_LO : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    pub const MIPS_TLS_TPREL64 : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    pub const TILEGX_IMM16_X0_HW2_LAST : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    pub const SPARC_HIX22 : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    pub const ARM_THM_MOVT_ABS : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    /// No relocation, set segment base.
    pub const PARISC_SEGBASE : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    /// disp (G >> 2)
    pub const CKCORE_GOT_IMM18BY4 : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    /// 24 bit GOT entry
    pub const M32R_GOT24 : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    /// X0 pipe mm "end"
    pub const TILEPRO_MMEND_X0 : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    pub const RISCV_GPREL_S : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    pub const METAG_TLS_LDM : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    pub const S390_TLS_IEENT : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    pub const PPC64_TOC16_HI : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    pub const MIPS_TLS_TPREL_HI16 : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    pub const TILEGX_IMM16_X1_HW2_LAST : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    pub const SPARC_LOX10 : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    pub const ARM_THM_MOVW_PREL_NC : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    /// 32 bits segment rel. address.
    pub const PARISC_SEGREL32 : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    /// disp (G >> 2)
    pub const CKCORE_PLT_IMM18BY4 : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    /// 26 bit PC relative to PLT shifted
    pub const M32R_26_PLTREL : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    /// X1 pipe mm "start"
    pub const TILEPRO_MMSTART_X1 : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    pub const RISCV_TPREL_I : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    pub const METAG_TLS_LDO_HI16 : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    pub const S390_TLS_LE32 : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    pub const PPC64_TOC16_HA : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    pub const MIPS_TLS_TPREL_LO16 : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    pub const TILEGX_IMM16_X0_HW0_PCREL : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    pub const SPARC_H44 : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    pub const ARM_THM_MOVT_PREL : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    /// PLT rel. address, left 21 bits.
    pub const PARISC_PLTOFF21L : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    /// disp ((S+A-P) >>2) & 0x7f
    pub const CKCORE_PCREL_IMM7BY4 : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    /// Copy symbol at runtime
    pub const M32R_COPY : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    /// X1 pipe mm "end"
    pub const TILEPRO_MMEND_X1 : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    pub const RISCV_TPREL_S : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    pub const METAG_TLS_LDO_LO16 : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    /// @ltoff(sym + add), add imm22
    pub const IA64_LTOFF22 : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    pub const ARC_PC32 : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    pub const S390_TLS_LE64 : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    pub const PPC64_TOC : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    pub const MIPS_GLOB_DAT : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    pub const TILEGX_IMM16_X1_HW0_PCREL : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    pub const SPARC_M44 : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    pub const ARM_THM_JUMP19 : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    /// 32 bit offset to TLS block
    pub const CKCORE_TLS_LE32 : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    /// Create GOT entry
    pub const M32R_GLOB_DAT : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    /// X0 pipe shift amount
    pub const TILEPRO_SHAMT_X0 : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    pub const RISCV_RELAX : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    pub const METAG_TLS_LDO : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    /// @ltoff(sym + add), mov imm64
    pub const IA64_LTOFF64I : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    pub const ARC_GOTPC32 : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    pub const S390_TLS_LDO32 : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    pub const PPC64_PLTGOT16 : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    pub const TILEGX_IMM16_X0_HW1_PCREL : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    pub const SPARC_L44 : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    pub const ARM_THM_JUMP6 : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    pub const CKCORE_TLS_IE32 : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    /// Create PLT entry
    pub const M32R_JMP_SLOT : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    /// X1 pipe shift amount
    pub const TILEPRO_SHAMT_X1 : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    pub const RISCV_SUB6 : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    pub const METAG_TLS_IE : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    pub const ARC_PLT32 : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    pub const S390_TLS_LDO64 : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    pub const PPC64_PLTGOT16_LO : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    pub const TILEGX_IMM16_X1_HW1_PCREL : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    pub const SPARC_REGISTER : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    pub const ARM_THM_ALU_PREL_11_0 : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    pub const CKCORE_TLS_GD32 : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    /// Adjust by program base
    pub const M32R_RELATIVE : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    /// Y0 pipe shift amount
    pub const TILEPRO_SHAMT_Y0 : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    pub const RISCV_SET6 : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    pub const METAG_TLS_IENONPIC : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    pub const ARC_COPY : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    pub const S390_TLS_DTPMOD : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    pub const PPC64_PLTGOT16_HI : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    pub const TILEGX_IMM16_X0_HW2_PCREL : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    pub const SPARC_UA64 : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    pub const ARM_THM_PC12 : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    /// PLT rel. address, right 14 bits.
    pub const PARISC_PLTOFF14R : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    pub const CKCORE_TLS_LDM32 : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    /// 24 bit offset to GOT
    pub const M32R_GOTOFF : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    /// Y1 pipe shift amount
    pub const TILEPRO_SHAMT_Y1 : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    pub const RISCV_SET8 : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    pub const METAG_TLS_IENONPIC_HI16 : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    pub const ARC_GLOB_DAT : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    pub const S390_TLS_DTPOFF : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    pub const PPC64_PLTGOT16_HA : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    pub const TILEGX_IMM16_X1_HW2_PCREL : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    pub const SPARC_UA16 : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    pub const ARM_ABS32_NOI : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    pub const CKCORE_TLS_LDO32 : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    /// 24 bit PC relative offset to GOT
    pub const M32R_GOTPC24 : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    /// X1 pipe destination 8-bit
    pub const TILEPRO_DEST_IMM8_X1 : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    pub const RISCV_SET16 : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    pub const METAG_TLS_IENONPIC_LO16 : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    pub const ARC_JUMP_SLOT : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    pub const S390_TLS_TPOFF : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    pub const PPC64_ADDR16_DS : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    pub const TILEGX_IMM16_X0_HW3_PCREL : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    pub const SPARC_TLS_GD_HI22 : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    pub const ARM_REL32_NOI : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    pub const CKCORE_TLS_DTPMOD32 : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    pub const M32R_GOT16_HI_ULO : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    pub const RISCV_SET32 : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    pub const METAG_TLS_TPOFF : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    pub const ARC_RELATIVE : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    pub const S390_20 : Self = Self::_S39020OrPpc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    pub const PPC64_ADDR16_LO_DS : Self = Self::_S39020OrPpc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    pub const TILEGX_IMM16_X1_HW3_PCREL : Self = Self::_S39020OrPpc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    pub const SPARC_TLS_GD_LO10 : Self = Self::_S39020OrPpc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    pub const ARM_ALU_PC_G0_NC : Self = Self::_S39020OrPpc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    /// 32 bits LT-rel. function pointer.
    pub const PARISC_LTOFF_FPTR32 : Self = Self::_S39020OrPpc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    pub const CKCORE_TLS_DTPOFF32 : Self = Self::_S39020OrPpc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    pub const M32R_GOT16_HI_SLO : Self = Self::_S39020OrPpc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    pub const RISCV_32_PCREL : Self = Self::_S39020OrPpc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    pub const METAG_TLS_DTPMOD : Self = Self::_S39020OrPpc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    pub const ARC_GOTOFF : Self = Self::_S39020OrPpc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    pub const S390_GOT20 : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    pub const PPC64_GOT16_DS : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    pub const TILEGX_IMM16_X0_HW0_LAST_PCREL : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    pub const SPARC_TLS_GD_ADD : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    pub const ARM_ALU_PC_G0 : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    /// LT-rel. fct ptr, left 21 bits.
    pub const PARISC_LTOFF_FPTR21L : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    pub const CKCORE_TLS_TPOFF32 : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    /// Low 16 bit GOT entry
    pub const M32R_GOT16_LO : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    pub const RISCV_IRELATIVE : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    pub const METAG_TLS_DTPOFF : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    /// @pltoff(sym + add), add imm22
    pub const IA64_PLTOFF22 : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    pub const ARC_GOTPC : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    pub const S390_GOTPLT20 : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrIa64Pltoff64IOrArcGot32;
    pub const PPC64_GOT16_LO_DS : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrIa64Pltoff64IOrArcGot32;
    pub const TILEGX_IMM16_X1_HW0_LAST_PCREL : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrIa64Pltoff64IOrArcGot32;
    pub const SPARC_TLS_GD_CALL : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrIa64Pltoff64IOrArcGot32;
    pub const ARM_ALU_PC_G1_NC : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrIa64Pltoff64IOrArcGot32;
    pub const M32R_GOTPC_HI_ULO : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrIa64Pltoff64IOrArcGot32;
    pub const RISCV_NUM : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrIa64Pltoff64IOrArcGot32;
    pub const METAG_TLS_LE : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrIa64Pltoff64IOrArcGot32;
    /// @pltoff(sym + add), mov imm64
    pub const IA64_PLTOFF64I : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrIa64Pltoff64IOrArcGot32;
    pub const ARC_GOT32 : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrIa64Pltoff64IOrArcGot32;
    pub const S390_TLS_GOTIE20 : Self = Self::_S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16;
    pub const PPC64_PLT16_LO_DS : Self = Self::_S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16;
    pub const MIPS_PC21_S2 : Self = Self::_S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16;
    pub const TILEGX_IMM16_X0_HW1_LAST_PCREL : Self = Self::_S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16;
    pub const SPARC_TLS_LDM_HI22 : Self = Self::_S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16;
    pub const ARM_ALU_PC_G1 : Self = Self::_S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16;
    pub const M32R_GOTPC_HI_SLO : Self = Self::_S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16;
    /// "jal" for TLS GD
    pub const TILEPRO_TLS_GD_CALL : Self = Self::_S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16;
    pub const METAG_TLS_LE_HI16 : Self = Self::_S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16;
    pub const S390_IRELATIVE : Self = Self::_S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16;
    pub const PPC64_SECTOFF_DS : Self = Self::_S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16;
    pub const MIPS_PC26_S2 : Self = Self::_S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16;
    pub const TILEGX_IMM16_X1_HW1_LAST_PCREL : Self = Self::_S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16;
    pub const SPARC_TLS_LDM_LO10 : Self = Self::_S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16;
    pub const ARM_ALU_PC_G2 : Self = Self::_S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16;
    pub const M32R_GOTPC_LO : Self = Self::_S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16;
    /// X0 pipe "addi" for TLS GD
    pub const TILEPRO_IMM8_X0_TLS_GD_ADD : Self = Self::_S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16;
    pub const METAG_TLS_LE_LO16 : Self = Self::_S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16;
    pub const S390_PC12DBL : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    pub const PPC64_SECTOFF_LO_DS : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    pub const MIPS_PC18_S3 : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    pub const TILEGX_IMM16_X0_HW2_LAST_PCREL : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    pub const SPARC_TLS_LDM_ADD : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    pub const ARM_LDR_PC_G1 : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    /// LT-rel. fct ptr, right 14 bits.
    pub const PARISC_LTOFF_FPTR14R : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    pub const S390_NUM : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    pub const M32R_GOTOFF_HI_ULO : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    /// X1 pipe "addi" for TLS GD
    pub const TILEPRO_IMM8_X1_TLS_GD_ADD : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    /// @pltoff(sym + add), data8 MSB
    pub const IA64_PLTOFF64MSB : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    pub const S390_PLT12DBL : Self = Self::_S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrM32RGotoffHiSloOrTileproImm8Y0TlsGdAddOrIa64Pltoff64Lsb;
    pub const PPC64_TOC16_DS : Self = Self::_S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrM32RGotoffHiSloOrTileproImm8Y0TlsGdAddOrIa64Pltoff64Lsb;
    pub const MIPS_PC19_S2 : Self = Self::_S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrM32RGotoffHiSloOrTileproImm8Y0TlsGdAddOrIa64Pltoff64Lsb;
    pub const TILEGX_IMM16_X1_HW2_LAST_PCREL : Self = Self::_S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrM32RGotoffHiSloOrTileproImm8Y0TlsGdAddOrIa64Pltoff64Lsb;
    pub const SPARC_TLS_LDM_CALL : Self = Self::_S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrM32RGotoffHiSloOrTileproImm8Y0TlsGdAddOrIa64Pltoff64Lsb;
    pub const ARM_LDR_PC_G2 : Self = Self::_S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrM32RGotoffHiSloOrTileproImm8Y0TlsGdAddOrIa64Pltoff64Lsb;
    pub const M32R_GOTOFF_HI_SLO : Self = Self::_S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrM32RGotoffHiSloOrTileproImm8Y0TlsGdAddOrIa64Pltoff64Lsb;
    /// Y0 pipe "addi" for TLS GD
    pub const TILEPRO_IMM8_Y0_TLS_GD_ADD : Self = Self::_S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrM32RGotoffHiSloOrTileproImm8Y0TlsGdAddOrIa64Pltoff64Lsb;
    /// @pltoff(sym + add), data8 LSB
    pub const IA64_PLTOFF64LSB : Self = Self::_S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrM32RGotoffHiSloOrTileproImm8Y0TlsGdAddOrIa64Pltoff64Lsb;
    pub const S390_PC24DBL : Self = Self::_S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd;
    pub const PPC64_TOC16_LO_DS : Self = Self::_S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd;
    pub const MIPS_PCHI16 : Self = Self::_S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd;
    pub const TILEGX_IMM16_X0_HW0_GOT : Self = Self::_S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd;
    pub const SPARC_TLS_LDO_HIX22 : Self = Self::_S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd;
    pub const ARM_LDRS_PC_G0 : Self = Self::_S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd;
    /// 64 bits function address.
    pub const PARISC_FPTR64 : Self = Self::_S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd;
    /// Low 16 bit offset to GOT
    pub const M32R_GOTOFF_LO : Self = Self::_S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd;
    /// Y1 pipe "addi" for TLS GD
    pub const TILEPRO_IMM8_Y1_TLS_GD_ADD : Self = Self::_S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd;
    pub const S390_PLT24DBL : Self = Self::_S390Plt24DblOrPpc64Pltgot16DsOrMipsPclo16OrTilegxImm16X1Hw0GotOrSparcTlsLdoLox10OrArmLdrsPcG1OrPariscPlabel32OrTileproTlsIeLoad;
    pub const PPC64_PLTGOT16_DS : Self = Self::_S390Plt24DblOrPpc64Pltgot16DsOrMipsPclo16OrTilegxImm16X1Hw0GotOrSparcTlsLdoLox10OrArmLdrsPcG1OrPariscPlabel32OrTileproTlsIeLoad;
    pub const MIPS_PCLO16 : Self = Self::_S390Plt24DblOrPpc64Pltgot16DsOrMipsPclo16OrTilegxImm16X1Hw0GotOrSparcTlsLdoLox10OrArmLdrsPcG1OrPariscPlabel32OrTileproTlsIeLoad;
    pub const TILEGX_IMM16_X1_HW0_GOT : Self = Self::_S390Plt24DblOrPpc64Pltgot16DsOrMipsPclo16OrTilegxImm16X1Hw0GotOrSparcTlsLdoLox10OrArmLdrsPcG1OrPariscPlabel32OrTileproTlsIeLoad;
    pub const SPARC_TLS_LDO_LOX10 : Self = Self::_S390Plt24DblOrPpc64Pltgot16DsOrMipsPclo16OrTilegxImm16X1Hw0GotOrSparcTlsLdoLox10OrArmLdrsPcG1OrPariscPlabel32OrTileproTlsIeLoad;
    pub const ARM_LDRS_PC_G1 : Self = Self::_S390Plt24DblOrPpc64Pltgot16DsOrMipsPclo16OrTilegxImm16X1Hw0GotOrSparcTlsLdoLox10OrArmLdrsPcG1OrPariscPlabel32OrTileproTlsIeLoad;
    /// 32 bits function address.
    pub const PARISC_PLABEL32 : Self = Self::_S390Plt24DblOrPpc64Pltgot16DsOrMipsPclo16OrTilegxImm16X1Hw0GotOrSparcTlsLdoLox10OrArmLdrsPcG1OrPariscPlabel32OrTileproTlsIeLoad;
    /// "lw_tls" for TLS IE
    pub const TILEPRO_TLS_IE_LOAD : Self = Self::_S390Plt24DblOrPpc64Pltgot16DsOrMipsPclo16OrTilegxImm16X1Hw0GotOrSparcTlsLdoLox10OrArmLdrsPcG1OrPariscPlabel32OrTileproTlsIeLoad;
    pub const S390_GNU_VTINHERIT : Self = Self::_S390GnuVtinheritOrPowerpcRel16LoOrMipsGnuRel16S2OrX8664GnuVtinheritOrSparcGnuVtinheritOrI386GnuVtinheritOrPpcRel16LoOrPpc64Rel16LoOrArmRsbrel32;
    pub const POWERPC_REL16_LO : Self = Self::_S390GnuVtinheritOrPowerpcRel16LoOrMipsGnuRel16S2OrX8664GnuVtinheritOrSparcGnuVtinheritOrI386GnuVtinheritOrPpcRel16LoOrPpc64Rel16LoOrArmRsbrel32;
    pub const MIPS_GNU_REL16_S2 : Self = Self::_S390GnuVtinheritOrPowerpcRel16LoOrMipsGnuRel16S2OrX8664GnuVtinheritOrSparcGnuVtinheritOrI386GnuVtinheritOrPpcRel16LoOrPpc64Rel16LoOrArmRsbrel32;
    pub const X86_64_GNU_VTINHERIT : Self = Self::_S390GnuVtinheritOrPowerpcRel16LoOrMipsGnuRel16S2OrX8664GnuVtinheritOrSparcGnuVtinheritOrI386GnuVtinheritOrPpcRel16LoOrPpc64Rel16LoOrArmRsbrel32;
    pub const SPARC_GNU_VTINHERIT : Self = Self::_S390GnuVtinheritOrPowerpcRel16LoOrMipsGnuRel16S2OrX8664GnuVtinheritOrSparcGnuVtinheritOrI386GnuVtinheritOrPpcRel16LoOrPpc64Rel16LoOrArmRsbrel32;
    pub const I386_GNU_VTINHERIT : Self = Self::_S390GnuVtinheritOrPowerpcRel16LoOrMipsGnuRel16S2OrX8664GnuVtinheritOrSparcGnuVtinheritOrI386GnuVtinheritOrPpcRel16LoOrPpc64Rel16LoOrArmRsbrel32;
    /// half16   (sym+add-.)@l
    pub const PPC_REL16_LO : Self = Self::_S390GnuVtinheritOrPowerpcRel16LoOrMipsGnuRel16S2OrX8664GnuVtinheritOrSparcGnuVtinheritOrI386GnuVtinheritOrPpcRel16LoOrPpc64Rel16LoOrArmRsbrel32;
    /// half16   (sym+add-.)@l
    pub const PPC64_REL16_LO : Self = Self::_S390GnuVtinheritOrPowerpcRel16LoOrMipsGnuRel16S2OrX8664GnuVtinheritOrSparcGnuVtinheritOrI386GnuVtinheritOrPpcRel16LoOrPpc64Rel16LoOrArmRsbrel32;
    pub const ARM_RSBREL32 : Self = Self::_S390GnuVtinheritOrPowerpcRel16LoOrMipsGnuRel16S2OrX8664GnuVtinheritOrSparcGnuVtinheritOrI386GnuVtinheritOrPpcRel16LoOrPpc64Rel16LoOrArmRsbrel32;
    pub const S390_GNU_VTENTRY : Self = Self::_S390GnuVtentryOrPowerpcRel16HiOrSparcGnuVtentryOrX8664GnuVtentryOrI386GnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22;
    pub const POWERPC_REL16_HI : Self = Self::_S390GnuVtentryOrPowerpcRel16HiOrSparcGnuVtentryOrX8664GnuVtentryOrI386GnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22;
    pub const SPARC_GNU_VTENTRY : Self = Self::_S390GnuVtentryOrPowerpcRel16HiOrSparcGnuVtentryOrX8664GnuVtentryOrI386GnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22;
    pub const X86_64_GNU_VTENTRY : Self = Self::_S390GnuVtentryOrPowerpcRel16HiOrSparcGnuVtentryOrX8664GnuVtentryOrI386GnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22;
    pub const I386_GNU_VTENTRY : Self = Self::_S390GnuVtentryOrPowerpcRel16HiOrSparcGnuVtentryOrX8664GnuVtentryOrI386GnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22;
    /// half16   (sym+add-.)@h
    pub const PPC_REL16_HI : Self = Self::_S390GnuVtentryOrPowerpcRel16HiOrSparcGnuVtentryOrX8664GnuVtentryOrI386GnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22;
    /// half16   (sym+add-.)@h
    pub const PPC64_REL16_HI : Self = Self::_S390GnuVtentryOrPowerpcRel16HiOrSparcGnuVtentryOrX8664GnuVtentryOrI386GnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22;
    pub const ARM_THM_RPC22 : Self = Self::_S390GnuVtentryOrPowerpcRel16HiOrSparcGnuVtentryOrX8664GnuVtentryOrI386GnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22;
    pub const PPC64_PLTGOT16_LO_DS : Self = Self::_Ppc64Pltgot16LoDsOrTilegxImm16X0Hw0PltPcrelOrSparcTlsLdoAddOrArmLdrsPcG2OrPariscPlabel21LOrTileproImm16X0TlsGdOrArcTlsDtpmod;
    pub const TILEGX_IMM16_X0_HW0_PLT_PCREL : Self = Self::_Ppc64Pltgot16LoDsOrTilegxImm16X0Hw0PltPcrelOrSparcTlsLdoAddOrArmLdrsPcG2OrPariscPlabel21LOrTileproImm16X0TlsGdOrArcTlsDtpmod;
    pub const SPARC_TLS_LDO_ADD : Self = Self::_Ppc64Pltgot16LoDsOrTilegxImm16X0Hw0PltPcrelOrSparcTlsLdoAddOrArmLdrsPcG2OrPariscPlabel21LOrTileproImm16X0TlsGdOrArcTlsDtpmod;
    pub const ARM_LDRS_PC_G2 : Self = Self::_Ppc64Pltgot16LoDsOrTilegxImm16X0Hw0PltPcrelOrSparcTlsLdoAddOrArmLdrsPcG2OrPariscPlabel21LOrTileproImm16X0TlsGdOrArcTlsDtpmod;
    /// Left 21 bits of fdesc address.
    pub const PARISC_PLABEL21L : Self = Self::_Ppc64Pltgot16LoDsOrTilegxImm16X0Hw0PltPcrelOrSparcTlsLdoAddOrArmLdrsPcG2OrPariscPlabel21LOrTileproImm16X0TlsGdOrArcTlsDtpmod;
    /// X0 pipe 16-bit TLS GD offset
    pub const TILEPRO_IMM16_X0_TLS_GD : Self = Self::_Ppc64Pltgot16LoDsOrTilegxImm16X0Hw0PltPcrelOrSparcTlsLdoAddOrArmLdrsPcG2OrPariscPlabel21LOrTileproImm16X0TlsGdOrArcTlsDtpmod;
    pub const ARC_TLS_DTPMOD : Self = Self::_Ppc64Pltgot16LoDsOrTilegxImm16X0Hw0PltPcrelOrSparcTlsLdoAddOrArmLdrsPcG2OrPariscPlabel21LOrTileproImm16X0TlsGdOrArcTlsDtpmod;
    pub const POWERPC_TLS : Self = Self::_PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff;
    pub const TILEGX_IMM16_X1_HW0_PLT_PCREL : Self = Self::_PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff;
    pub const SPARC_TLS_IE_HI22 : Self = Self::_PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff;
    pub const ARM_LDC_PC_G0 : Self = Self::_PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff;
    /// none	(sym+add)@tls
    pub const PPC_TLS : Self = Self::_PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff;
    /// none	(sym+add)@tls
    pub const PPC64_TLS : Self = Self::_PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff;
    /// X1 pipe 16-bit TLS GD offset
    pub const TILEPRO_IMM16_X1_TLS_GD : Self = Self::_PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff;
    /// @fptr(sym + add), mov imm64
    pub const IA64_FPTR64I : Self = Self::_PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff;
    pub const ARC_TLS_DTPOFF : Self = Self::_PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff;
    pub const POWERPC_DTPMOD : Self = Self::_PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff;
    pub const TILEGX_IMM16_X0_HW1_PLT_PCREL : Self = Self::_PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff;
    pub const SPARC_TLS_IE_LO10 : Self = Self::_PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff;
    pub const ARM_LDC_PC_G1 : Self = Self::_PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff;
    /// word32	(sym+add)@dtpmod
    pub const PPC_DTPMOD32 : Self = Self::_PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff;
    /// doubleword64 (sym+add)@dtpmod
    pub const PPC64_DTPMOD64 : Self = Self::_PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff;
    /// X0 pipe low 16-bit TLS GD offset
    pub const TILEPRO_IMM16_X0_TLS_GD_LO : Self = Self::_PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff;
    /// @fptr(sym + add), data4 MSB
    pub const IA64_FPTR32MSB : Self = Self::_PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff;
    pub const ARC_TLS_TPOFF : Self = Self::_PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff;
    pub const POWERPC_TPREL16 : Self = Self::_PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot;
    pub const TILEGX_IMM16_X1_HW1_PLT_PCREL : Self = Self::_PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot;
    pub const SPARC_TLS_IE_LD : Self = Self::_PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot;
    pub const ARM_LDC_PC_G2 : Self = Self::_PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot;
    /// half16*	(sym+add)@tprel
    pub const PPC_TPREL16 : Self = Self::_PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot;
    /// half16*	(sym+add)@tprel
    pub const PPC64_TPREL16 : Self = Self::_PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot;
    /// X1 pipe low 16-bit TLS GD offset
    pub const TILEPRO_IMM16_X1_TLS_GD_LO : Self = Self::_PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot;
    /// @fptr(sym + add), data4 LSB
    pub const IA64_FPTR32LSB : Self = Self::_PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot;
    pub const ARC_TLS_GD_GOT : Self = Self::_PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot;
    pub const POWERPC_TPREL16_LO : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    pub const TILEGX_IMM16_X0_HW2_PLT_PCREL : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    pub const SPARC_TLS_IE_LDX : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    pub const ARM_ALU_SB_G0_NC : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    /// Right 14 bits of fdesc address.
    pub const PARISC_PLABEL14R : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    /// half16	(sym+add)@tprel@l
    pub const PPC_TPREL16_LO : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    /// half16	(sym+add)@tprel@l
    pub const PPC64_TPREL16_LO : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    /// X0 pipe high 16-bit TLS GD offset
    pub const TILEPRO_IMM16_X0_TLS_GD_HI : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    /// @fptr(sym + add), data8 MSB
    pub const IA64_FPTR64MSB : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    pub const ARC_TLS_GD_LD : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    pub const POWERPC_TPREL16_HI : Self = Self::_PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall;
    pub const TILEGX_IMM16_X1_HW2_PLT_PCREL : Self = Self::_PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall;
    pub const SPARC_TLS_IE_ADD : Self = Self::_PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall;
    pub const ARM_ALU_SB_G0 : Self = Self::_PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall;
    /// half16	(sym+add)@tprel@h
    pub const PPC_TPREL16_HI : Self = Self::_PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall;
    /// half16	(sym+add)@tprel@h
    pub const PPC64_TPREL16_HI : Self = Self::_PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall;
    /// X1 pipe high 16-bit TLS GD offset
    pub const TILEPRO_IMM16_X1_TLS_GD_HI : Self = Self::_PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall;
    /// @fptr(sym + add), data8 LSB
    pub const IA64_FPTR64LSB : Self = Self::_PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall;
    pub const ARC_TLS_GD_CALL : Self = Self::_PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall;
    pub const POWERPC_TPREL16_HA : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    pub const TILEGX_IMM16_X0_HW0_LAST_GOT : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    pub const SPARC_TLS_LE_HIX22 : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    pub const ARM_ALU_SB_G1_NC : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    /// 64 bits PC-rel. address.
    pub const PARISC_PCREL64 : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    /// half16	(sym+add)@tprel@ha
    pub const PPC_TPREL16_HA : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    /// half16	(sym+add)@tprel@ha
    pub const PPC64_TPREL16_HA : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    /// X0 pipe ha() 16-bit TLS GD offset
    pub const TILEPRO_IMM16_X0_TLS_GD_HA : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    /// @pcrel(sym + add), brl
    pub const IA64_PCREL60B : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    pub const ARC_TLS_IE_GOT : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    pub const POWERPC_TPREL : Self = Self::_PowerpcTprelOrTilegxImm16X1Hw0LastGotOrSparcTlsLeLox10OrArmAluSbG1OrPpcTprel32OrPpc64Tprel64OrTileproImm16X1TlsGdHaOrIa64Pcrel21B;
    pub const TILEGX_IMM16_X1_HW0_LAST_GOT : Self = Self::_PowerpcTprelOrTilegxImm16X1Hw0LastGotOrSparcTlsLeLox10OrArmAluSbG1OrPpcTprel32OrPpc64Tprel64OrTileproImm16X1TlsGdHaOrIa64Pcrel21B;
    pub const SPARC_TLS_LE_LOX10 : Self = Self::_PowerpcTprelOrTilegxImm16X1Hw0LastGotOrSparcTlsLeLox10OrArmAluSbG1OrPpcTprel32OrPpc64Tprel64OrTileproImm16X1TlsGdHaOrIa64Pcrel21B;
    pub const ARM_ALU_SB_G1 : Self = Self::_PowerpcTprelOrTilegxImm16X1Hw0LastGotOrSparcTlsLeLox10OrArmAluSbG1OrPpcTprel32OrPpc64Tprel64OrTileproImm16X1TlsGdHaOrIa64Pcrel21B;
    /// word32	(sym+add)@tprel
    pub const PPC_TPREL32 : Self = Self::_PowerpcTprelOrTilegxImm16X1Hw0LastGotOrSparcTlsLeLox10OrArmAluSbG1OrPpcTprel32OrPpc64Tprel64OrTileproImm16X1TlsGdHaOrIa64Pcrel21B;
    /// doubleword64 (sym+add)@tprel
    pub const PPC64_TPREL64 : Self = Self::_PowerpcTprelOrTilegxImm16X1Hw0LastGotOrSparcTlsLeLox10OrArmAluSbG1OrPpcTprel32OrPpc64Tprel64OrTileproImm16X1TlsGdHaOrIa64Pcrel21B;
    /// X1 pipe ha() 16-bit TLS GD offset
    pub const TILEPRO_IMM16_X1_TLS_GD_HA : Self = Self::_PowerpcTprelOrTilegxImm16X1Hw0LastGotOrSparcTlsLeLox10OrArmAluSbG1OrPpcTprel32OrPpc64Tprel64OrTileproImm16X1TlsGdHaOrIa64Pcrel21B;
    /// @pcrel(sym + add), ptb, call
    pub const IA64_PCREL21B : Self = Self::_PowerpcTprelOrTilegxImm16X1Hw0LastGotOrSparcTlsLeLox10OrArmAluSbG1OrPpcTprel32OrPpc64Tprel64OrTileproImm16X1TlsGdHaOrIa64Pcrel21B;
    pub const POWERPC_DTPREL16 : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    pub const TILEGX_IMM16_X0_HW1_LAST_GOT : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    pub const SPARC_TLS_DTPMOD32 : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    pub const ARM_ALU_SB_G2 : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    /// 22 bits PC-rel. address.
    pub const PARISC_PCREL22F : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    /// half16*	(sym+add)@dtprel
    pub const PPC_DTPREL16 : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    /// half16*	(sym+add)@dtprel
    pub const PPC64_DTPREL16 : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    /// X0 pipe 16-bit TLS IE offset
    pub const TILEPRO_IMM16_X0_TLS_IE : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    /// @pcrel(sym + add), chk.s
    pub const IA64_PCREL21M : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    pub const ARC_TLS_DTPOFF_S9 : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    pub const ARC_TLS_LE_S9 : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    pub const POWERPC_DTPREL16_LO : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    pub const TILEGX_IMM16_X1_HW1_LAST_GOT : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    pub const SPARC_TLS_DTPMOD64 : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    pub const ARM_LDR_SB_G0 : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    /// PC-rel. address, right 14 bits.
    pub const PARISC_PCREL14WR : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    /// half16	(sym+add)@dtprel@l
    pub const PPC_DTPREL16_LO : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    /// half16	(sym+add)@dtprel@l
    pub const PPC64_DTPREL16_LO : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    /// X1 pipe 16-bit TLS IE offset
    pub const TILEPRO_IMM16_X1_TLS_IE : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    /// @pcrel(sym + add), fchkf
    pub const IA64_PCREL21F : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    pub const ARC_TLS_LE_32 : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    pub const POWERPC_DTPREL16_HI : Self = Self::_PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb;
    pub const SPARC_TLS_DTPOFF32 : Self = Self::_PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb;
    pub const ARM_LDR_SB_G1 : Self = Self::_PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb;
    /// PC rel. address, right 14 bits.
    pub const PARISC_PCREL14DR : Self = Self::_PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb;
    /// half16	(sym+add)@dtprel@h
    pub const PPC_DTPREL16_HI : Self = Self::_PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb;
    /// half16	(sym+add)@dtprel@h
    pub const PPC64_DTPREL16_HI : Self = Self::_PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb;
    /// X0 pipe low 16-bit TLS IE offset
    pub const TILEPRO_IMM16_X0_TLS_IE_LO : Self = Self::_PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb;
    /// X0 pipe PC-rel PLT hword 3
    pub const TILEGX_IMM16_X0_HW3_PLT_PCREL : Self = Self::_PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb;
    /// @pcrel(sym + add), data4 MSB
    pub const IA64_PCREL32MSB : Self = Self::_PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb;
    pub const POWERPC_DTPREL16_HA : Self = Self::_PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb;
    pub const SPARC_TLS_DTPOFF64 : Self = Self::_PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb;
    pub const ARM_LDR_SB_G2 : Self = Self::_PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb;
    /// 16 bits PC-rel. address.
    pub const PARISC_PCREL16F : Self = Self::_PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb;
    /// half16	(sym+add)@dtprel@ha
    pub const PPC_DTPREL16_HA : Self = Self::_PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb;
    /// half16	(sym+add)@dtprel@ha
    pub const PPC64_DTPREL16_HA : Self = Self::_PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb;
    /// X1 pipe low 16-bit TLS IE offset
    pub const TILEPRO_IMM16_X1_TLS_IE_LO : Self = Self::_PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb;
    /// X1 pipe PC-rel PLT hword 3
    pub const TILEGX_IMM16_X1_HW3_PLT_PCREL : Self = Self::_PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb;
    /// @pcrel(sym + add), data4 LSB
    pub const IA64_PCREL32LSB : Self = Self::_PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb;
    pub const POWERPC_DTPREL : Self = Self::_PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb;
    pub const TILEGX_IMM16_X0_HW0_TLS_GD : Self = Self::_PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb;
    pub const SPARC_TLS_TPOFF32 : Self = Self::_PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb;
    pub const ARM_LDRS_SB_G0 : Self = Self::_PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb;
    /// 16 bits PC-rel. address.
    pub const PARISC_PCREL16WF : Self = Self::_PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb;
    /// word32	(sym+add)@dtprel
    pub const PPC_DTPREL32 : Self = Self::_PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb;
    /// doubleword64 (sym+add)@dtprel
    pub const PPC64_DTPREL64 : Self = Self::_PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb;
    /// X0 pipe high 16-bit TLS IE offset
    pub const TILEPRO_IMM16_X0_TLS_IE_HI : Self = Self::_PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb;
    /// @pcrel(sym + add), data8 MSB
    pub const IA64_PCREL64MSB : Self = Self::_PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb;
    pub const POWERPC_GOT_TLSGD16 : Self = Self::_PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb;
    pub const TILEGX_IMM16_X1_HW0_TLS_GD : Self = Self::_PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb;
    pub const SPARC_TLS_TPOFF64 : Self = Self::_PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb;
    pub const ARM_LDRS_SB_G1 : Self = Self::_PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb;
    /// 16 bits PC-rel. address.
    pub const PARISC_PCREL16DF : Self = Self::_PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb;
    /// half16*	(sym+add)@got@tlsgd
    pub const PPC_GOT_TLSGD16 : Self = Self::_PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb;
    /// half16*	(sym+add)@got@tlsgd
    pub const PPC64_GOT_TLSGD16 : Self = Self::_PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb;
    /// X1 pipe high 16-bit TLS IE offset
    pub const TILEPRO_IMM16_X1_TLS_IE_HI : Self = Self::_PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb;
    /// @pcrel(sym + add), data8 LSB
    pub const IA64_PCREL64LSB : Self = Self::_PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb;
    pub const POWERPC_GOT_TLSGD16_LO : Self = Self::_PowerpcGotTlsgd16LoOrSparcGotdataHix22OrTilegxImm16X0Hw0TlsLeOrArmLdrsSbG2OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrTileproImm16X0TlsIeHa;
    pub const SPARC_GOTDATA_HIX22 : Self = Self::_PowerpcGotTlsgd16LoOrSparcGotdataHix22OrTilegxImm16X0Hw0TlsLeOrArmLdrsSbG2OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrTileproImm16X0TlsIeHa;
    pub const TILEGX_IMM16_X0_HW0_TLS_LE : Self = Self::_PowerpcGotTlsgd16LoOrSparcGotdataHix22OrTilegxImm16X0Hw0TlsLeOrArmLdrsSbG2OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrTileproImm16X0TlsIeHa;
    pub const ARM_LDRS_SB_G2 : Self = Self::_PowerpcGotTlsgd16LoOrSparcGotdataHix22OrTilegxImm16X0Hw0TlsLeOrArmLdrsSbG2OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrTileproImm16X0TlsIeHa;
    /// 64 bits of eff. address.
    pub const PARISC_DIR64 : Self = Self::_PowerpcGotTlsgd16LoOrSparcGotdataHix22OrTilegxImm16X0Hw0TlsLeOrArmLdrsSbG2OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrTileproImm16X0TlsIeHa;
    /// half16	(sym+add)@got@tlsgd@l
    pub const PPC_GOT_TLSGD16_LO : Self = Self::_PowerpcGotTlsgd16LoOrSparcGotdataHix22OrTilegxImm16X0Hw0TlsLeOrArmLdrsSbG2OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrTileproImm16X0TlsIeHa;
    /// half16	(sym+add)@got@tlsgd@l
    pub const PPC64_GOT_TLSGD16_LO : Self = Self::_PowerpcGotTlsgd16LoOrSparcGotdataHix22OrTilegxImm16X0Hw0TlsLeOrArmLdrsSbG2OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrTileproImm16X0TlsIeHa;
    /// X0 pipe ha() 16-bit TLS IE offset
    pub const TILEPRO_IMM16_X0_TLS_IE_HA : Self = Self::_PowerpcGotTlsgd16LoOrSparcGotdataHix22OrTilegxImm16X0Hw0TlsLeOrArmLdrsSbG2OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrTileproImm16X0TlsIeHa;
    pub const POWERPC_GOT_TLSGD16_HI : Self = Self::_PowerpcGotTlsgd16HiOrSparcGotdataLox10OrTilegxImm16X1Hw0TlsLeOrArmLdcSbG0OrPpcGotTlsgd16HiOrPpc64GotTlsgd16HiOrTileproImm16X1TlsIeHa;
    pub const SPARC_GOTDATA_LOX10 : Self = Self::_PowerpcGotTlsgd16HiOrSparcGotdataLox10OrTilegxImm16X1Hw0TlsLeOrArmLdcSbG0OrPpcGotTlsgd16HiOrPpc64GotTlsgd16HiOrTileproImm16X1TlsIeHa;
    pub const TILEGX_IMM16_X1_HW0_TLS_LE : Self = Self::_PowerpcGotTlsgd16HiOrSparcGotdataLox10OrTilegxImm16X1Hw0TlsLeOrArmLdcSbG0OrPpcGotTlsgd16HiOrPpc64GotTlsgd16HiOrTileproImm16X1TlsIeHa;
    pub const ARM_LDC_SB_G0 : Self = Self::_PowerpcGotTlsgd16HiOrSparcGotdataLox10OrTilegxImm16X1Hw0TlsLeOrArmLdcSbG0OrPpcGotTlsgd16HiOrPpc64GotTlsgd16HiOrTileproImm16X1TlsIeHa;
    /// half16	(sym+add)@got@tlsgd@h
    pub const PPC_GOT_TLSGD16_HI : Self = Self::_PowerpcGotTlsgd16HiOrSparcGotdataLox10OrTilegxImm16X1Hw0TlsLeOrArmLdcSbG0OrPpcGotTlsgd16HiOrPpc64GotTlsgd16HiOrTileproImm16X1TlsIeHa;
    /// half16	(sym+add)@got@tlsgd@h
    pub const PPC64_GOT_TLSGD16_HI : Self = Self::_PowerpcGotTlsgd16HiOrSparcGotdataLox10OrTilegxImm16X1Hw0TlsLeOrArmLdcSbG0OrPpcGotTlsgd16HiOrPpc64GotTlsgd16HiOrTileproImm16X1TlsIeHa;
    /// X1 pipe ha() 16-bit TLS IE offset
    pub const TILEPRO_IMM16_X1_TLS_IE_HA : Self = Self::_PowerpcGotTlsgd16HiOrSparcGotdataLox10OrTilegxImm16X1Hw0TlsLeOrArmLdcSbG0OrPpcGotTlsgd16HiOrPpc64GotTlsgd16HiOrTileproImm16X1TlsIeHa;
    pub const POWERPC_GOT_TLSGD16_HA : Self = Self::_PowerpcGotTlsgd16HaOrSparcGotdataOpHix22OrTilegxImm16X0Hw0LastTlsLeOrArmLdcSbG1OrPpcGotTlsgd16HaOrPpc64GotTlsgd16HaOrTileproTlsDtpmod32OrIa64LtoffFptr22;
    pub const SPARC_GOTDATA_OP_HIX22 : Self = Self::_PowerpcGotTlsgd16HaOrSparcGotdataOpHix22OrTilegxImm16X0Hw0LastTlsLeOrArmLdcSbG1OrPpcGotTlsgd16HaOrPpc64GotTlsgd16HaOrTileproTlsDtpmod32OrIa64LtoffFptr22;
    pub const TILEGX_IMM16_X0_HW0_LAST_TLS_LE : Self = Self::_PowerpcGotTlsgd16HaOrSparcGotdataOpHix22OrTilegxImm16X0Hw0LastTlsLeOrArmLdcSbG1OrPpcGotTlsgd16HaOrPpc64GotTlsgd16HaOrTileproTlsDtpmod32OrIa64LtoffFptr22;
    pub const ARM_LDC_SB_G1 : Self = Self::_PowerpcGotTlsgd16HaOrSparcGotdataOpHix22OrTilegxImm16X0Hw0LastTlsLeOrArmLdcSbG1OrPpcGotTlsgd16HaOrPpc64GotTlsgd16HaOrTileproTlsDtpmod32OrIa64LtoffFptr22;
    /// half16	(sym+add)@got@tlsgd@ha
    pub const PPC_GOT_TLSGD16_HA : Self = Self::_PowerpcGotTlsgd16HaOrSparcGotdataOpHix22OrTilegxImm16X0Hw0LastTlsLeOrArmLdcSbG1OrPpcGotTlsgd16HaOrPpc64GotTlsgd16HaOrTileproTlsDtpmod32OrIa64LtoffFptr22;
    /// half16	(sym+add)@got@tlsgd@ha
    pub const PPC64_GOT_TLSGD16_HA : Self = Self::_PowerpcGotTlsgd16HaOrSparcGotdataOpHix22OrTilegxImm16X0Hw0LastTlsLeOrArmLdcSbG1OrPpcGotTlsgd16HaOrPpc64GotTlsgd16HaOrTileproTlsDtpmod32OrIa64LtoffFptr22;
    /// ID of module containing symbol
    pub const TILEPRO_TLS_DTPMOD32 : Self = Self::_PowerpcGotTlsgd16HaOrSparcGotdataOpHix22OrTilegxImm16X0Hw0LastTlsLeOrArmLdcSbG1OrPpcGotTlsgd16HaOrPpc64GotTlsgd16HaOrTileproTlsDtpmod32OrIa64LtoffFptr22;
    /// @ltoff(@fptr(s+a)), imm22
    pub const IA64_LTOFF_FPTR22 : Self = Self::_PowerpcGotTlsgd16HaOrSparcGotdataOpHix22OrTilegxImm16X0Hw0LastTlsLeOrArmLdcSbG1OrPpcGotTlsgd16HaOrPpc64GotTlsgd16HaOrTileproTlsDtpmod32OrIa64LtoffFptr22;
    pub const POWERPC_GOT_TLSLD16 : Self = Self::_PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I;
    pub const SPARC_GOTDATA_OP_LOX10 : Self = Self::_PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I;
    pub const TILEGX_IMM16_X1_HW0_LAST_TLS_LE : Self = Self::_PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I;
    pub const ARM_LDC_SB_G2 : Self = Self::_PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I;
    /// 14 bits of eff. address.
    pub const PARISC_DIR14WR : Self = Self::_PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I;
    /// half16*	(sym+add)@got@tlsld
    pub const PPC_GOT_TLSLD16 : Self = Self::_PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I;
    /// half16*	(sym+add)@got@tlsld
    pub const PPC64_GOT_TLSLD16 : Self = Self::_PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I;
    /// Offset in TLS block
    pub const TILEPRO_TLS_DTPOFF32 : Self = Self::_PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I;
    /// @ltoff(@fptr(s+a)), imm64
    pub const IA64_LTOFF_FPTR64I : Self = Self::_PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I;
    pub const POWERPC_GOT_TLSLD16_LO : Self = Self::_PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb;
    pub const SPARC_GOTDATA_OP : Self = Self::_PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb;
    pub const TILEGX_IMM16_X0_HW1_LAST_TLS_LE : Self = Self::_PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb;
    pub const ARM_MOVW_BREL_NC : Self = Self::_PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb;
    /// 14 bits of eff. address.
    pub const PARISC_DIR14DR : Self = Self::_PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb;
    /// half16	(sym+add)@got@tlsld@l
    pub const PPC_GOT_TLSLD16_LO : Self = Self::_PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb;
    /// half16	(sym+add)@got@tlsld@l
    pub const PPC64_GOT_TLSLD16_LO : Self = Self::_PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb;
    /// Offset in static TLS block
    pub const TILEPRO_TLS_TPOFF32 : Self = Self::_PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb;
    /// @ltoff(@fptr(s+a)), data4 MSB
    pub const IA64_LTOFF_FPTR32MSB : Self = Self::_PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb;
    pub const POWERPC_GOT_TLSLD16_HI : Self = Self::_PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb;
    pub const TILEGX_IMM16_X1_HW1_LAST_TLS_LE : Self = Self::_PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb;
    pub const SPARC_H34 : Self = Self::_PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb;
    pub const ARM_MOVT_BREL : Self = Self::_PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb;
    /// 16 bits of eff. address.
    pub const PARISC_DIR16F : Self = Self::_PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb;
    /// half16	(sym+add)@got@tlsld@h
    pub const PPC_GOT_TLSLD16_HI : Self = Self::_PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb;
    /// half16	(sym+add)@got@tlsld@h
    pub const PPC64_GOT_TLSLD16_HI : Self = Self::_PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb;
    /// X0 pipe 16-bit TLS LE offset
    pub const TILEPRO_IMM16_X0_TLS_LE : Self = Self::_PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb;
    /// @ltoff(@fptr(s+a)), data4 LSB
    pub const IA64_LTOFF_FPTR32LSB : Self = Self::_PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb;
    pub const POWERPC_GOT_TLSLD16_HA : Self = Self::_PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb;
    pub const TILEGX_IMM16_X0_HW0_LAST_TLS_GD : Self = Self::_PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb;
    pub const SPARC_SIZE32 : Self = Self::_PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb;
    pub const ARM_MOVW_BREL : Self = Self::_PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb;
    /// 16 bits of eff. address.
    pub const PARISC_DIR16WF : Self = Self::_PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb;
    /// half16	(sym+add)@got@tlsld@ha
    pub const PPC_GOT_TLSLD16_HA : Self = Self::_PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb;
    /// half16	(sym+add)@got@tlsld@ha
    pub const PPC64_GOT_TLSLD16_HA : Self = Self::_PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb;
    /// X1 pipe 16-bit TLS LE offset
    pub const TILEPRO_IMM16_X1_TLS_LE : Self = Self::_PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb;
    /// @ltoff(@fptr(s+a)), data8 MSB
    pub const IA64_LTOFF_FPTR64MSB : Self = Self::_PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb;
    pub const POWERPC_GOT_TPREL16 : Self = Self::_PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb;
    pub const TILEGX_IMM16_X1_HW0_LAST_TLS_GD : Self = Self::_PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb;
    pub const SPARC_SIZE64 : Self = Self::_PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb;
    pub const ARM_THM_MOVW_BREL_NC : Self = Self::_PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb;
    /// 16 bits of eff. address.
    pub const PARISC_DIR16DF : Self = Self::_PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb;
    /// half16*	(sym+add)@got@tprel
    pub const PPC_GOT_TPREL16 : Self = Self::_PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb;
    /// half16ds*	(sym+add)@got@tprel
    pub const PPC64_GOT_TPREL16_DS : Self = Self::_PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb;
    /// X0 pipe low 16-bit TLS LE offset
    pub const TILEPRO_IMM16_X0_TLS_LE_LO : Self = Self::_PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb;
    /// @ltoff(@fptr(s+a)), data8 LSB
    pub const IA64_LTOFF_FPTR64LSB : Self = Self::_PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb;
    pub const POWERPC_GOT_TPREL16_LO : Self = Self::_PowerpcGotTprel16LoOrTilegxImm16X0Hw1LastTlsGdOrSparcWdisp10OrArmThmMovtBrelOrPariscGprel64OrPpcGotTprel16LoOrPpc64GotTprel16LoDsOrTileproImm16X1TlsLeLo;
    pub const TILEGX_IMM16_X0_HW1_LAST_TLS_GD : Self = Self::_PowerpcGotTprel16LoOrTilegxImm16X0Hw1LastTlsGdOrSparcWdisp10OrArmThmMovtBrelOrPariscGprel64OrPpcGotTprel16LoOrPpc64GotTprel16LoDsOrTileproImm16X1TlsLeLo;
    pub const SPARC_WDISP10 : Self = Self::_PowerpcGotTprel16LoOrTilegxImm16X0Hw1LastTlsGdOrSparcWdisp10OrArmThmMovtBrelOrPariscGprel64OrPpcGotTprel16LoOrPpc64GotTprel16LoDsOrTileproImm16X1TlsLeLo;
    pub const ARM_THM_MOVT_BREL : Self = Self::_PowerpcGotTprel16LoOrTilegxImm16X0Hw1LastTlsGdOrSparcWdisp10OrArmThmMovtBrelOrPariscGprel64OrPpcGotTprel16LoOrPpc64GotTprel16LoDsOrTileproImm16X1TlsLeLo;
    /// 64 bits of GP-rel. address.
    pub const PARISC_GPREL64 : Self = Self::_PowerpcGotTprel16LoOrTilegxImm16X0Hw1LastTlsGdOrSparcWdisp10OrArmThmMovtBrelOrPariscGprel64OrPpcGotTprel16LoOrPpc64GotTprel16LoDsOrTileproImm16X1TlsLeLo;
    /// half16	(sym+add)@got@tprel@l
    pub const PPC_GOT_TPREL16_LO : Self = Self::_PowerpcGotTprel16LoOrTilegxImm16X0Hw1LastTlsGdOrSparcWdisp10OrArmThmMovtBrelOrPariscGprel64OrPpcGotTprel16LoOrPpc64GotTprel16LoDsOrTileproImm16X1TlsLeLo;
    /// half16ds (sym+add)@got@tprel@l
    pub const PPC64_GOT_TPREL16_LO_DS : Self = Self::_PowerpcGotTprel16LoOrTilegxImm16X0Hw1LastTlsGdOrSparcWdisp10OrArmThmMovtBrelOrPariscGprel64OrPpcGotTprel16LoOrPpc64GotTprel16LoDsOrTileproImm16X1TlsLeLo;
    /// X1 pipe low 16-bit TLS LE offset
    pub const TILEPRO_IMM16_X1_TLS_LE_LO : Self = Self::_PowerpcGotTprel16LoOrTilegxImm16X0Hw1LastTlsGdOrSparcWdisp10OrArmThmMovtBrelOrPariscGprel64OrPpcGotTprel16LoOrPpc64GotTprel16LoDsOrTileproImm16X1TlsLeLo;
    pub const POWERPC_GOT_TPREL16_HI : Self = Self::_PowerpcGotTprel16HiOrTilegxImm16X1Hw1LastTlsGdOrArmThmMovwBrelOrPpcGotTprel16HiOrPpc64GotTprel16HiOrTileproImm16X0TlsLeHi;
    pub const TILEGX_IMM16_X1_HW1_LAST_TLS_GD : Self = Self::_PowerpcGotTprel16HiOrTilegxImm16X1Hw1LastTlsGdOrArmThmMovwBrelOrPpcGotTprel16HiOrPpc64GotTprel16HiOrTileproImm16X0TlsLeHi;
    pub const ARM_THM_MOVW_BREL : Self = Self::_PowerpcGotTprel16HiOrTilegxImm16X1Hw1LastTlsGdOrArmThmMovwBrelOrPpcGotTprel16HiOrPpc64GotTprel16HiOrTileproImm16X0TlsLeHi;
    /// half16	(sym+add)@got@tprel@h
    pub const PPC_GOT_TPREL16_HI : Self = Self::_PowerpcGotTprel16HiOrTilegxImm16X1Hw1LastTlsGdOrArmThmMovwBrelOrPpcGotTprel16HiOrPpc64GotTprel16HiOrTileproImm16X0TlsLeHi;
    /// half16	(sym+add)@got@tprel@h
    pub const PPC64_GOT_TPREL16_HI : Self = Self::_PowerpcGotTprel16HiOrTilegxImm16X1Hw1LastTlsGdOrArmThmMovwBrelOrPpcGotTprel16HiOrPpc64GotTprel16HiOrTileproImm16X0TlsLeHi;
    /// X0 pipe high 16-bit TLS LE offset
    pub const TILEPRO_IMM16_X0_TLS_LE_HI : Self = Self::_PowerpcGotTprel16HiOrTilegxImm16X1Hw1LastTlsGdOrArmThmMovwBrelOrPpcGotTprel16HiOrPpc64GotTprel16HiOrTileproImm16X0TlsLeHi;
    pub const POWERPC_GOT_TPREL16_HA : Self = Self::_PowerpcGotTprel16HaOrTilegxIrelativeOrArmTlsGotdescOrPpcGotTprel16HaOrPpc64GotTprel16HaOrTileproImm16X1TlsLeHi;
    pub const TILEGX_IRELATIVE : Self = Self::_PowerpcGotTprel16HaOrTilegxIrelativeOrArmTlsGotdescOrPpcGotTprel16HaOrPpc64GotTprel16HaOrTileproImm16X1TlsLeHi;
    pub const ARM_TLS_GOTDESC : Self = Self::_PowerpcGotTprel16HaOrTilegxIrelativeOrArmTlsGotdescOrPpcGotTprel16HaOrPpc64GotTprel16HaOrTileproImm16X1TlsLeHi;
    /// half16	(sym+add)@got@tprel@ha
    pub const PPC_GOT_TPREL16_HA : Self = Self::_PowerpcGotTprel16HaOrTilegxIrelativeOrArmTlsGotdescOrPpcGotTprel16HaOrPpc64GotTprel16HaOrTileproImm16X1TlsLeHi;
    /// half16	(sym+add)@got@tprel@ha
    pub const PPC64_GOT_TPREL16_HA : Self = Self::_PowerpcGotTprel16HaOrTilegxIrelativeOrArmTlsGotdescOrPpcGotTprel16HaOrPpc64GotTprel16HaOrTileproImm16X1TlsLeHi;
    /// X1 pipe high 16-bit TLS LE offset
    pub const TILEPRO_IMM16_X1_TLS_LE_HI : Self = Self::_PowerpcGotTprel16HaOrTilegxIrelativeOrArmTlsGotdescOrPpcGotTprel16HaOrPpc64GotTprel16HaOrTileproImm16X1TlsLeHi;
    pub const POWERPC_GOT_DTPREL16 : Self = Self::_PowerpcGotDtprel16OrArmTlsCallOrPariscGprel14WrOrPpcGotDtprel16OrPpc64GotDtprel16DsOrTileproImm16X0TlsLeHa;
    pub const ARM_TLS_CALL : Self = Self::_PowerpcGotDtprel16OrArmTlsCallOrPariscGprel14WrOrPpcGotDtprel16OrPpc64GotDtprel16DsOrTileproImm16X0TlsLeHa;
    /// GP-rel. address, right 14 bits.
    pub const PARISC_GPREL14WR : Self = Self::_PowerpcGotDtprel16OrArmTlsCallOrPariscGprel14WrOrPpcGotDtprel16OrPpc64GotDtprel16DsOrTileproImm16X0TlsLeHa;
    /// half16*	(sym+add)@got@dtprel
    pub const PPC_GOT_DTPREL16 : Self = Self::_PowerpcGotDtprel16OrArmTlsCallOrPariscGprel14WrOrPpcGotDtprel16OrPpc64GotDtprel16DsOrTileproImm16X0TlsLeHa;
    /// half16ds*	(sym+add)@got@dtprel
    pub const PPC64_GOT_DTPREL16_DS : Self = Self::_PowerpcGotDtprel16OrArmTlsCallOrPariscGprel14WrOrPpcGotDtprel16OrPpc64GotDtprel16DsOrTileproImm16X0TlsLeHa;
    /// X0 pipe ha() 16-bit TLS LE offset
    pub const TILEPRO_IMM16_X0_TLS_LE_HA : Self = Self::_PowerpcGotDtprel16OrArmTlsCallOrPariscGprel14WrOrPpcGotDtprel16OrPpc64GotDtprel16DsOrTileproImm16X0TlsLeHa;
    pub const POWERPC_GOT_DTPREL16_LO : Self = Self::_PowerpcGotDtprel16LoOrTilegxImm16X0Hw0TlsIeOrArmTlsDescseqOrPariscGprel14DrOrPpcGotDtprel16LoOrPpc64GotDtprel16LoDsOrTileproImm16X1TlsLeHaOrIa64Segrel32Msb;
    pub const TILEGX_IMM16_X0_HW0_TLS_IE : Self = Self::_PowerpcGotDtprel16LoOrTilegxImm16X0Hw0TlsIeOrArmTlsDescseqOrPariscGprel14DrOrPpcGotDtprel16LoOrPpc64GotDtprel16LoDsOrTileproImm16X1TlsLeHaOrIa64Segrel32Msb;
    pub const ARM_TLS_DESCSEQ : Self = Self::_PowerpcGotDtprel16LoOrTilegxImm16X0Hw0TlsIeOrArmTlsDescseqOrPariscGprel14DrOrPpcGotDtprel16LoOrPpc64GotDtprel16LoDsOrTileproImm16X1TlsLeHaOrIa64Segrel32Msb;
    /// GP-rel. address, right 14 bits.
    pub const PARISC_GPREL14DR : Self = Self::_PowerpcGotDtprel16LoOrTilegxImm16X0Hw0TlsIeOrArmTlsDescseqOrPariscGprel14DrOrPpcGotDtprel16LoOrPpc64GotDtprel16LoDsOrTileproImm16X1TlsLeHaOrIa64Segrel32Msb;
    /// half16*	(sym+add)@got@dtprel@l
    pub const PPC_GOT_DTPREL16_LO : Self = Self::_PowerpcGotDtprel16LoOrTilegxImm16X0Hw0TlsIeOrArmTlsDescseqOrPariscGprel14DrOrPpcGotDtprel16LoOrPpc64GotDtprel16LoDsOrTileproImm16X1TlsLeHaOrIa64Segrel32Msb;
    /// half16ds (sym+add)@got@dtprel@l
    pub const PPC64_GOT_DTPREL16_LO_DS : Self = Self::_PowerpcGotDtprel16LoOrTilegxImm16X0Hw0TlsIeOrArmTlsDescseqOrPariscGprel14DrOrPpcGotDtprel16LoOrPpc64GotDtprel16LoDsOrTileproImm16X1TlsLeHaOrIa64Segrel32Msb;
    /// X1 pipe ha() 16-bit TLS LE offset
    pub const TILEPRO_IMM16_X1_TLS_LE_HA : Self = Self::_PowerpcGotDtprel16LoOrTilegxImm16X0Hw0TlsIeOrArmTlsDescseqOrPariscGprel14DrOrPpcGotDtprel16LoOrPpc64GotDtprel16LoDsOrTileproImm16X1TlsLeHaOrIa64Segrel32Msb;
    /// @segrel(sym + add), data4 MSB
    pub const IA64_SEGREL32MSB : Self = Self::_PowerpcGotDtprel16LoOrTilegxImm16X0Hw0TlsIeOrArmTlsDescseqOrPariscGprel14DrOrPpcGotDtprel16LoOrPpc64GotDtprel16LoDsOrTileproImm16X1TlsLeHaOrIa64Segrel32Msb;
    pub const POWERPC_GOT_DTPREL16_HI : Self = Self::_PowerpcGotDtprel16HiOrTilegxImm16X1Hw0TlsIeOrArmThmTlsCallOrPariscGprel16FOrPpcGotDtprel16HiOrPpc64GotDtprel16HiOrIa64Segrel32Lsb;
    pub const TILEGX_IMM16_X1_HW0_TLS_IE : Self = Self::_PowerpcGotDtprel16HiOrTilegxImm16X1Hw0TlsIeOrArmThmTlsCallOrPariscGprel16FOrPpcGotDtprel16HiOrPpc64GotDtprel16HiOrIa64Segrel32Lsb;
    pub const ARM_THM_TLS_CALL : Self = Self::_PowerpcGotDtprel16HiOrTilegxImm16X1Hw0TlsIeOrArmThmTlsCallOrPariscGprel16FOrPpcGotDtprel16HiOrPpc64GotDtprel16HiOrIa64Segrel32Lsb;
    /// 16 bits GP-rel. address.
    pub const PARISC_GPREL16F : Self = Self::_PowerpcGotDtprel16HiOrTilegxImm16X1Hw0TlsIeOrArmThmTlsCallOrPariscGprel16FOrPpcGotDtprel16HiOrPpc64GotDtprel16HiOrIa64Segrel32Lsb;
    /// half16*	(sym+add)@got@dtprel@h
    pub const PPC_GOT_DTPREL16_HI : Self = Self::_PowerpcGotDtprel16HiOrTilegxImm16X1Hw0TlsIeOrArmThmTlsCallOrPariscGprel16FOrPpcGotDtprel16HiOrPpc64GotDtprel16HiOrIa64Segrel32Lsb;
    /// half16	(sym+add)@got@dtprel@h
    pub const PPC64_GOT_DTPREL16_HI : Self = Self::_PowerpcGotDtprel16HiOrTilegxImm16X1Hw0TlsIeOrArmThmTlsCallOrPariscGprel16FOrPpcGotDtprel16HiOrPpc64GotDtprel16HiOrIa64Segrel32Lsb;
    /// @segrel(sym + add), data4 LSB
    pub const IA64_SEGREL32LSB : Self = Self::_PowerpcGotDtprel16HiOrTilegxImm16X1Hw0TlsIeOrArmThmTlsCallOrPariscGprel16FOrPpcGotDtprel16HiOrPpc64GotDtprel16HiOrIa64Segrel32Lsb;
    pub const POWERPC_GOT_DTPREL16_HA : Self = Self::_PowerpcGotDtprel16HaOrTilegxImm16X0Hw0LastPltPcrelOrArmPlt32AbsOrPariscGprel16WfOrPpcGotDtprel16HaOrPpc64GotDtprel16HaOrIa64Segrel64Msb;
    pub const TILEGX_IMM16_X0_HW0_LAST_PLT_PCREL : Self = Self::_PowerpcGotDtprel16HaOrTilegxImm16X0Hw0LastPltPcrelOrArmPlt32AbsOrPariscGprel16WfOrPpcGotDtprel16HaOrPpc64GotDtprel16HaOrIa64Segrel64Msb;
    pub const ARM_PLT32_ABS : Self = Self::_PowerpcGotDtprel16HaOrTilegxImm16X0Hw0LastPltPcrelOrArmPlt32AbsOrPariscGprel16WfOrPpcGotDtprel16HaOrPpc64GotDtprel16HaOrIa64Segrel64Msb;
    /// 16 bits GP-rel. address.
    pub const PARISC_GPREL16WF : Self = Self::_PowerpcGotDtprel16HaOrTilegxImm16X0Hw0LastPltPcrelOrArmPlt32AbsOrPariscGprel16WfOrPpcGotDtprel16HaOrPpc64GotDtprel16HaOrIa64Segrel64Msb;
    /// half16*	(sym+add)@got@dtprel@ha
    pub const PPC_GOT_DTPREL16_HA : Self = Self::_PowerpcGotDtprel16HaOrTilegxImm16X0Hw0LastPltPcrelOrArmPlt32AbsOrPariscGprel16WfOrPpcGotDtprel16HaOrPpc64GotDtprel16HaOrIa64Segrel64Msb;
    /// half16	(sym+add)@got@dtprel@ha
    pub const PPC64_GOT_DTPREL16_HA : Self = Self::_PowerpcGotDtprel16HaOrTilegxImm16X0Hw0LastPltPcrelOrArmPlt32AbsOrPariscGprel16WfOrPpcGotDtprel16HaOrPpc64GotDtprel16HaOrIa64Segrel64Msb;
    /// @segrel(sym + add), data8 MSB
    pub const IA64_SEGREL64MSB : Self = Self::_PowerpcGotDtprel16HaOrTilegxImm16X0Hw0LastPltPcrelOrArmPlt32AbsOrPariscGprel16WfOrPpcGotDtprel16HaOrPpc64GotDtprel16HaOrIa64Segrel64Msb;
    pub const PPC_TLSGD : Self = Self::_PpcTlsgdOrPpc64Tprel16DsOrTilegxImm16X1Hw0LastPltPcrelOrArmGotAbsOrPariscGprel16DfOrIa64Segrel64Lsb;
    pub const PPC64_TPREL16_DS : Self = Self::_PpcTlsgdOrPpc64Tprel16DsOrTilegxImm16X1Hw0LastPltPcrelOrArmGotAbsOrPariscGprel16DfOrIa64Segrel64Lsb;
    pub const TILEGX_IMM16_X1_HW0_LAST_PLT_PCREL : Self = Self::_PpcTlsgdOrPpc64Tprel16DsOrTilegxImm16X1Hw0LastPltPcrelOrArmGotAbsOrPariscGprel16DfOrIa64Segrel64Lsb;
    pub const ARM_GOT_ABS : Self = Self::_PpcTlsgdOrPpc64Tprel16DsOrTilegxImm16X1Hw0LastPltPcrelOrArmGotAbsOrPariscGprel16DfOrIa64Segrel64Lsb;
    /// 16 bits GP-rel. address.
    pub const PARISC_GPREL16DF : Self = Self::_PpcTlsgdOrPpc64Tprel16DsOrTilegxImm16X1Hw0LastPltPcrelOrArmGotAbsOrPariscGprel16DfOrIa64Segrel64Lsb;
    /// @segrel(sym + add), data8 LSB
    pub const IA64_SEGREL64LSB : Self = Self::_PpcTlsgdOrPpc64Tprel16DsOrTilegxImm16X1Hw0LastPltPcrelOrArmGotAbsOrPariscGprel16DfOrIa64Segrel64Lsb;
    pub const PPC_TLSLD: Self =
        Self::_PpcTlsldOrPpc64Tprel16LoDsOrTilegxImm16X0Hw1LastPltPcrelOrArmGotPrelOrPariscLtoff64;
    pub const PPC64_TPREL16_LO_DS: Self =
        Self::_PpcTlsldOrPpc64Tprel16LoDsOrTilegxImm16X0Hw1LastPltPcrelOrArmGotPrelOrPariscLtoff64;
    pub const TILEGX_IMM16_X0_HW1_LAST_PLT_PCREL: Self =
        Self::_PpcTlsldOrPpc64Tprel16LoDsOrTilegxImm16X0Hw1LastPltPcrelOrArmGotPrelOrPariscLtoff64;
    pub const ARM_GOT_PREL: Self =
        Self::_PpcTlsldOrPpc64Tprel16LoDsOrTilegxImm16X0Hw1LastPltPcrelOrArmGotPrelOrPariscLtoff64;
    /// 64 bits LT-rel. address.
    pub const PARISC_LTOFF64: Self =
        Self::_PpcTlsldOrPpc64Tprel16LoDsOrTilegxImm16X0Hw1LastPltPcrelOrArmGotPrelOrPariscLtoff64;
    pub const PPC64_TPREL16_HIGHER: Self =
        Self::_Ppc64Tprel16HigherOrTilegxImm16X1Hw1LastPltPcrelOrArmGotBrel12;
    pub const TILEGX_IMM16_X1_HW1_LAST_PLT_PCREL: Self =
        Self::_Ppc64Tprel16HigherOrTilegxImm16X1Hw1LastPltPcrelOrArmGotBrel12;
    pub const ARM_GOT_BREL12: Self =
        Self::_Ppc64Tprel16HigherOrTilegxImm16X1Hw1LastPltPcrelOrArmGotBrel12;
    pub const PPC64_TPREL16_HIGHERA: Self =
        Self::_Ppc64Tprel16HigheraOrTilegxImm16X0Hw2LastPltPcrelOrArmGotoff12;
    pub const TILEGX_IMM16_X0_HW2_LAST_PLT_PCREL: Self =
        Self::_Ppc64Tprel16HigheraOrTilegxImm16X0Hw2LastPltPcrelOrArmGotoff12;
    pub const ARM_GOTOFF12: Self =
        Self::_Ppc64Tprel16HigheraOrTilegxImm16X0Hw2LastPltPcrelOrArmGotoff12;
    pub const PPC64_TPREL16_HIGHEST: Self =
        Self::_Ppc64Tprel16HighestOrTilegxImm16X1Hw2LastPltPcrelOrArmGotrelaxOrPariscLtoff14Wr;
    pub const TILEGX_IMM16_X1_HW2_LAST_PLT_PCREL: Self =
        Self::_Ppc64Tprel16HighestOrTilegxImm16X1Hw2LastPltPcrelOrArmGotrelaxOrPariscLtoff14Wr;
    pub const ARM_GOTRELAX: Self =
        Self::_Ppc64Tprel16HighestOrTilegxImm16X1Hw2LastPltPcrelOrArmGotrelaxOrPariscLtoff14Wr;
    /// LT-rel. address, right 14 bits.
    pub const PARISC_LTOFF14WR: Self =
        Self::_Ppc64Tprel16HighestOrTilegxImm16X1Hw2LastPltPcrelOrArmGotrelaxOrPariscLtoff14Wr;
    pub const PPC64_TPREL16_HIGHESTA : Self = Self::_Ppc64Tprel16HighestaOrMips1626OrTilegxImm16X0Hw0LastTlsIeOrArmGnuVtentryOrPariscLtoff14DrOrIa64Secrel32Msb;
    pub const MIPS16_26 : Self = Self::_Ppc64Tprel16HighestaOrMips1626OrTilegxImm16X0Hw0LastTlsIeOrArmGnuVtentryOrPariscLtoff14DrOrIa64Secrel32Msb;
    pub const TILEGX_IMM16_X0_HW0_LAST_TLS_IE : Self = Self::_Ppc64Tprel16HighestaOrMips1626OrTilegxImm16X0Hw0LastTlsIeOrArmGnuVtentryOrPariscLtoff14DrOrIa64Secrel32Msb;
    pub const ARM_GNU_VTENTRY : Self = Self::_Ppc64Tprel16HighestaOrMips1626OrTilegxImm16X0Hw0LastTlsIeOrArmGnuVtentryOrPariscLtoff14DrOrIa64Secrel32Msb;
    /// LT-rel. address, right 14 bits.
    pub const PARISC_LTOFF14DR : Self = Self::_Ppc64Tprel16HighestaOrMips1626OrTilegxImm16X0Hw0LastTlsIeOrArmGnuVtentryOrPariscLtoff14DrOrIa64Secrel32Msb;
    /// @secrel(sym + add), data4 MSB
    pub const IA64_SECREL32MSB : Self = Self::_Ppc64Tprel16HighestaOrMips1626OrTilegxImm16X0Hw0LastTlsIeOrArmGnuVtentryOrPariscLtoff14DrOrIa64Secrel32Msb;
    pub const PPC_EMB_NADDR32 : Self = Self::_PpcEmbNaddr32OrPpc64Dtprel16DsOrMips16GprelOrTilegxImm16X1Hw0LastTlsIeOrArmGnuVtinheritOrPariscLtoff16FOrIa64Secrel32Lsb;
    pub const PPC64_DTPREL16_DS : Self = Self::_PpcEmbNaddr32OrPpc64Dtprel16DsOrMips16GprelOrTilegxImm16X1Hw0LastTlsIeOrArmGnuVtinheritOrPariscLtoff16FOrIa64Secrel32Lsb;
    pub const MIPS16_GPREL : Self = Self::_PpcEmbNaddr32OrPpc64Dtprel16DsOrMips16GprelOrTilegxImm16X1Hw0LastTlsIeOrArmGnuVtinheritOrPariscLtoff16FOrIa64Secrel32Lsb;
    pub const TILEGX_IMM16_X1_HW0_LAST_TLS_IE : Self = Self::_PpcEmbNaddr32OrPpc64Dtprel16DsOrMips16GprelOrTilegxImm16X1Hw0LastTlsIeOrArmGnuVtinheritOrPariscLtoff16FOrIa64Secrel32Lsb;
    pub const ARM_GNU_VTINHERIT : Self = Self::_PpcEmbNaddr32OrPpc64Dtprel16DsOrMips16GprelOrTilegxImm16X1Hw0LastTlsIeOrArmGnuVtinheritOrPariscLtoff16FOrIa64Secrel32Lsb;
    /// 16 bits LT-rel. address.
    pub const PARISC_LTOFF16F : Self = Self::_PpcEmbNaddr32OrPpc64Dtprel16DsOrMips16GprelOrTilegxImm16X1Hw0LastTlsIeOrArmGnuVtinheritOrPariscLtoff16FOrIa64Secrel32Lsb;
    /// @secrel(sym + add), data4 LSB
    pub const IA64_SECREL32LSB : Self = Self::_PpcEmbNaddr32OrPpc64Dtprel16DsOrMips16GprelOrTilegxImm16X1Hw0LastTlsIeOrArmGnuVtinheritOrPariscLtoff16FOrIa64Secrel32Lsb;
    pub const PPC_EMB_NADDR16 : Self = Self::_PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb;
    pub const PPC64_DTPREL16_LO_DS : Self = Self::_PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb;
    pub const MIPS16_GOT16 : Self = Self::_PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb;
    pub const TILEGX_IMM16_X0_HW1_LAST_TLS_IE : Self = Self::_PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb;
    pub const ARM_THM_JUMP11 : Self = Self::_PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb;
    /// 16 bits LT-rel. address.
    pub const PARISC_LTOFF16WF : Self = Self::_PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb;
    /// PC relative & 0xFFE (Thumb16 B).
    pub const ARM_THM_PC11 : Self = Self::_PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb;
    pub const NDS32_TLS_TPOFF : Self = Self::_PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb;
    /// @secrel(sym + add), data8 MSB
    pub const IA64_SECREL64MSB : Self = Self::_PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb;
    pub const PPC_EMB_NADDR16_LO : Self = Self::_PpcEmbNaddr16LoOrPpc64Dtprel16HigherOrMips16Call16OrTilegxImm16X1Hw1LastTlsIeOrArmThmJump8OrPariscLtoff16DfOrArmThmPc9OrIa64Secrel64Lsb;
    pub const PPC64_DTPREL16_HIGHER : Self = Self::_PpcEmbNaddr16LoOrPpc64Dtprel16HigherOrMips16Call16OrTilegxImm16X1Hw1LastTlsIeOrArmThmJump8OrPariscLtoff16DfOrArmThmPc9OrIa64Secrel64Lsb;
    pub const MIPS16_CALL16 : Self = Self::_PpcEmbNaddr16LoOrPpc64Dtprel16HigherOrMips16Call16OrTilegxImm16X1Hw1LastTlsIeOrArmThmJump8OrPariscLtoff16DfOrArmThmPc9OrIa64Secrel64Lsb;
    pub const TILEGX_IMM16_X1_HW1_LAST_TLS_IE : Self = Self::_PpcEmbNaddr16LoOrPpc64Dtprel16HigherOrMips16Call16OrTilegxImm16X1Hw1LastTlsIeOrArmThmJump8OrPariscLtoff16DfOrArmThmPc9OrIa64Secrel64Lsb;
    pub const ARM_THM_JUMP8 : Self = Self::_PpcEmbNaddr16LoOrPpc64Dtprel16HigherOrMips16Call16OrTilegxImm16X1Hw1LastTlsIeOrArmThmJump8OrPariscLtoff16DfOrArmThmPc9OrIa64Secrel64Lsb;
    /// 16 bits LT-rel. address.
    pub const PARISC_LTOFF16DF : Self = Self::_PpcEmbNaddr16LoOrPpc64Dtprel16HigherOrMips16Call16OrTilegxImm16X1Hw1LastTlsIeOrArmThmJump8OrPariscLtoff16DfOrArmThmPc9OrIa64Secrel64Lsb;
    pub const ARM_THM_PC9 : Self = Self::_PpcEmbNaddr16LoOrPpc64Dtprel16HigherOrMips16Call16OrTilegxImm16X1Hw1LastTlsIeOrArmThmJump8OrPariscLtoff16DfOrArmThmPc9OrIa64Secrel64Lsb;
    /// @secrel(sym + add), data8 LSB
    pub const IA64_SECREL64LSB : Self = Self::_PpcEmbNaddr16LoOrPpc64Dtprel16HigherOrMips16Call16OrTilegxImm16X1Hw1LastTlsIeOrArmThmJump8OrPariscLtoff16DfOrArmThmPc9OrIa64Secrel64Lsb;
    pub const PPC_EMB_NADDR16_HI: Self =
        Self::_PpcEmbNaddr16HiOrPpc64Dtprel16HigheraOrMips16Hi16OrArmTlsGd32OrPariscSecrel64;
    pub const PPC64_DTPREL16_HIGHERA: Self =
        Self::_PpcEmbNaddr16HiOrPpc64Dtprel16HigheraOrMips16Hi16OrArmTlsGd32OrPariscSecrel64;
    pub const MIPS16_HI16: Self =
        Self::_PpcEmbNaddr16HiOrPpc64Dtprel16HigheraOrMips16Hi16OrArmTlsGd32OrPariscSecrel64;
    pub const ARM_TLS_GD32: Self =
        Self::_PpcEmbNaddr16HiOrPpc64Dtprel16HigheraOrMips16Hi16OrArmTlsGd32OrPariscSecrel64;
    /// 64 bits section rel. address.
    pub const PARISC_SECREL64: Self =
        Self::_PpcEmbNaddr16HiOrPpc64Dtprel16HigheraOrMips16Hi16OrArmTlsGd32OrPariscSecrel64;
    pub const PPC_EMB_NADDR16_HA: Self =
        Self::_PpcEmbNaddr16HaOrPpc64Dtprel16HighestOrMips16Lo16OrArmTlsLdm32;
    pub const PPC64_DTPREL16_HIGHEST: Self =
        Self::_PpcEmbNaddr16HaOrPpc64Dtprel16HighestOrMips16Lo16OrArmTlsLdm32;
    pub const MIPS16_LO16: Self =
        Self::_PpcEmbNaddr16HaOrPpc64Dtprel16HighestOrMips16Lo16OrArmTlsLdm32;
    pub const ARM_TLS_LDM32: Self =
        Self::_PpcEmbNaddr16HaOrPpc64Dtprel16HighestOrMips16Lo16OrArmTlsLdm32;
    pub const PPC_EMB_SDAI16: Self =
        Self::_PpcEmbSdai16OrPpc64Dtprel16HighestaOrMips16TlsGdOrTilegxTlsDtpmod64OrArmTlsLdo32;
    pub const PPC64_DTPREL16_HIGHESTA: Self =
        Self::_PpcEmbSdai16OrPpc64Dtprel16HighestaOrMips16TlsGdOrTilegxTlsDtpmod64OrArmTlsLdo32;
    pub const MIPS16_TLS_GD: Self =
        Self::_PpcEmbSdai16OrPpc64Dtprel16HighestaOrMips16TlsGdOrTilegxTlsDtpmod64OrArmTlsLdo32;
    pub const TILEGX_TLS_DTPMOD64: Self =
        Self::_PpcEmbSdai16OrPpc64Dtprel16HighestaOrMips16TlsGdOrTilegxTlsDtpmod64OrArmTlsLdo32;
    pub const ARM_TLS_LDO32: Self =
        Self::_PpcEmbSdai16OrPpc64Dtprel16HighestaOrMips16TlsGdOrTilegxTlsDtpmod64OrArmTlsLdo32;
    pub const PPC_EMB_SDA2I16: Self =
        Self::_PpcEmbSda2I16OrPpc64TlsgdOrMips16TlsLdmOrTilegxTlsDtpoff64OrArmTlsIe32;
    pub const PPC64_TLSGD: Self =
        Self::_PpcEmbSda2I16OrPpc64TlsgdOrMips16TlsLdmOrTilegxTlsDtpoff64OrArmTlsIe32;
    pub const MIPS16_TLS_LDM: Self =
        Self::_PpcEmbSda2I16OrPpc64TlsgdOrMips16TlsLdmOrTilegxTlsDtpoff64OrArmTlsIe32;
    pub const TILEGX_TLS_DTPOFF64: Self =
        Self::_PpcEmbSda2I16OrPpc64TlsgdOrMips16TlsLdmOrTilegxTlsDtpoff64OrArmTlsIe32;
    pub const ARM_TLS_IE32: Self =
        Self::_PpcEmbSda2I16OrPpc64TlsgdOrMips16TlsLdmOrTilegxTlsDtpoff64OrArmTlsIe32;
    pub const PPC_EMB_SDA2REL : Self = Self::_PpcEmbSda2RelOrPpc64TlsldOrMips16TlsDtprelHi16OrTilegxTlsTpoff64OrArmTlsLe32OrIa64Rel32Msb;
    pub const PPC64_TLSLD : Self = Self::_PpcEmbSda2RelOrPpc64TlsldOrMips16TlsDtprelHi16OrTilegxTlsTpoff64OrArmTlsLe32OrIa64Rel32Msb;
    pub const MIPS16_TLS_DTPREL_HI16 : Self = Self::_PpcEmbSda2RelOrPpc64TlsldOrMips16TlsDtprelHi16OrTilegxTlsTpoff64OrArmTlsLe32OrIa64Rel32Msb;
    pub const TILEGX_TLS_TPOFF64 : Self = Self::_PpcEmbSda2RelOrPpc64TlsldOrMips16TlsDtprelHi16OrTilegxTlsTpoff64OrArmTlsLe32OrIa64Rel32Msb;
    pub const ARM_TLS_LE32 : Self = Self::_PpcEmbSda2RelOrPpc64TlsldOrMips16TlsDtprelHi16OrTilegxTlsTpoff64OrArmTlsLe32OrIa64Rel32Msb;
    /// data 4 + REL
    pub const IA64_REL32MSB : Self = Self::_PpcEmbSda2RelOrPpc64TlsldOrMips16TlsDtprelHi16OrTilegxTlsTpoff64OrArmTlsLe32OrIa64Rel32Msb;
    pub const PPC_EMB_SDA21 : Self = Self::_PpcEmbSda21OrPpc64TocsaveOrMips16TlsDtprelLo16OrTilegxTlsDtpmod32OrArmTlsLdo12OrIa64Rel32Lsb;
    pub const PPC64_TOCSAVE : Self = Self::_PpcEmbSda21OrPpc64TocsaveOrMips16TlsDtprelLo16OrTilegxTlsDtpmod32OrArmTlsLdo12OrIa64Rel32Lsb;
    pub const MIPS16_TLS_DTPREL_LO16 : Self = Self::_PpcEmbSda21OrPpc64TocsaveOrMips16TlsDtprelLo16OrTilegxTlsDtpmod32OrArmTlsLdo12OrIa64Rel32Lsb;
    pub const TILEGX_TLS_DTPMOD32 : Self = Self::_PpcEmbSda21OrPpc64TocsaveOrMips16TlsDtprelLo16OrTilegxTlsDtpmod32OrArmTlsLdo12OrIa64Rel32Lsb;
    pub const ARM_TLS_LDO12 : Self = Self::_PpcEmbSda21OrPpc64TocsaveOrMips16TlsDtprelLo16OrTilegxTlsDtpmod32OrArmTlsLdo12OrIa64Rel32Lsb;
    /// data 4 + REL
    pub const IA64_REL32LSB : Self = Self::_PpcEmbSda21OrPpc64TocsaveOrMips16TlsDtprelLo16OrTilegxTlsDtpmod32OrArmTlsLdo12OrIa64Rel32Lsb;
    pub const PPC_EMB_MRKREF : Self = Self::_PpcEmbMrkrefOrPpc64Addr16HighOrMips16TlsGottprelOrTilegxTlsDtpoff32OrArmTlsLe12OrIa64Rel64Msb;
    pub const PPC64_ADDR16_HIGH : Self = Self::_PpcEmbMrkrefOrPpc64Addr16HighOrMips16TlsGottprelOrTilegxTlsDtpoff32OrArmTlsLe12OrIa64Rel64Msb;
    pub const MIPS16_TLS_GOTTPREL : Self = Self::_PpcEmbMrkrefOrPpc64Addr16HighOrMips16TlsGottprelOrTilegxTlsDtpoff32OrArmTlsLe12OrIa64Rel64Msb;
    pub const TILEGX_TLS_DTPOFF32 : Self = Self::_PpcEmbMrkrefOrPpc64Addr16HighOrMips16TlsGottprelOrTilegxTlsDtpoff32OrArmTlsLe12OrIa64Rel64Msb;
    pub const ARM_TLS_LE12 : Self = Self::_PpcEmbMrkrefOrPpc64Addr16HighOrMips16TlsGottprelOrTilegxTlsDtpoff32OrArmTlsLe12OrIa64Rel64Msb;
    /// data 8 + REL
    pub const IA64_REL64MSB : Self = Self::_PpcEmbMrkrefOrPpc64Addr16HighOrMips16TlsGottprelOrTilegxTlsDtpoff32OrArmTlsLe12OrIa64Rel64Msb;
    pub const PPC_EMB_RELSEC16 : Self = Self::_PpcEmbRelsec16OrPpc64Addr16HighaOrMips16TlsTprelHi16OrTilegxTlsTpoff32OrArmTlsIe12GpOrIa64Rel64Lsb;
    pub const PPC64_ADDR16_HIGHA : Self = Self::_PpcEmbRelsec16OrPpc64Addr16HighaOrMips16TlsTprelHi16OrTilegxTlsTpoff32OrArmTlsIe12GpOrIa64Rel64Lsb;
    pub const MIPS16_TLS_TPREL_HI16 : Self = Self::_PpcEmbRelsec16OrPpc64Addr16HighaOrMips16TlsTprelHi16OrTilegxTlsTpoff32OrArmTlsIe12GpOrIa64Rel64Lsb;
    pub const TILEGX_TLS_TPOFF32 : Self = Self::_PpcEmbRelsec16OrPpc64Addr16HighaOrMips16TlsTprelHi16OrTilegxTlsTpoff32OrArmTlsIe12GpOrIa64Rel64Lsb;
    pub const ARM_TLS_IE12GP : Self = Self::_PpcEmbRelsec16OrPpc64Addr16HighaOrMips16TlsTprelHi16OrTilegxTlsTpoff32OrArmTlsIe12GpOrIa64Rel64Lsb;
    /// data 8 + REL
    pub const IA64_REL64LSB : Self = Self::_PpcEmbRelsec16OrPpc64Addr16HighaOrMips16TlsTprelHi16OrTilegxTlsTpoff32OrArmTlsIe12GpOrIa64Rel64Lsb;
    pub const PPC_EMB_RELST_LO : Self = Self::_PpcEmbRelstLoOrPpc64Tprel16HighOrMips16TlsTprelLo16OrTilegxTlsGdCallOrArmPrivate0OrPariscSegrel64;
    pub const PPC64_TPREL16_HIGH : Self = Self::_PpcEmbRelstLoOrPpc64Tprel16HighOrMips16TlsTprelLo16OrTilegxTlsGdCallOrArmPrivate0OrPariscSegrel64;
    pub const MIPS16_TLS_TPREL_LO16 : Self = Self::_PpcEmbRelstLoOrPpc64Tprel16HighOrMips16TlsTprelLo16OrTilegxTlsGdCallOrArmPrivate0OrPariscSegrel64;
    pub const TILEGX_TLS_GD_CALL : Self = Self::_PpcEmbRelstLoOrPpc64Tprel16HighOrMips16TlsTprelLo16OrTilegxTlsGdCallOrArmPrivate0OrPariscSegrel64;
    pub const ARM_PRIVATE_0 : Self = Self::_PpcEmbRelstLoOrPpc64Tprel16HighOrMips16TlsTprelLo16OrTilegxTlsGdCallOrArmPrivate0OrPariscSegrel64;
    /// 64 bits segment rel. address.
    pub const PARISC_SEGREL64 : Self = Self::_PpcEmbRelstLoOrPpc64Tprel16HighOrMips16TlsTprelLo16OrTilegxTlsGdCallOrArmPrivate0OrPariscSegrel64;
    pub const PPC_EMB_RELST_HI: Self =
        Self::_PpcEmbRelstHiOrPpc64Tprel16HighaOrArmPrivate1OrTilegxImm8X0TlsGdAdd;
    pub const PPC64_TPREL16_HIGHA: Self =
        Self::_PpcEmbRelstHiOrPpc64Tprel16HighaOrArmPrivate1OrTilegxImm8X0TlsGdAdd;
    pub const ARM_PRIVATE_1: Self =
        Self::_PpcEmbRelstHiOrPpc64Tprel16HighaOrArmPrivate1OrTilegxImm8X0TlsGdAdd;
    pub const TILEGX_IMM8_X0_TLS_GD_ADD: Self =
        Self::_PpcEmbRelstHiOrPpc64Tprel16HighaOrArmPrivate1OrTilegxImm8X0TlsGdAdd;
    pub const PPC_EMB_RELST_HA: Self =
        Self::_PpcEmbRelstHaOrPpc64Dtprel16HighOrArmPrivate2OrTilegxImm8X1TlsGdAdd;
    pub const PPC64_DTPREL16_HIGH: Self =
        Self::_PpcEmbRelstHaOrPpc64Dtprel16HighOrArmPrivate2OrTilegxImm8X1TlsGdAdd;
    pub const ARM_PRIVATE_2: Self =
        Self::_PpcEmbRelstHaOrPpc64Dtprel16HighOrArmPrivate2OrTilegxImm8X1TlsGdAdd;
    pub const TILEGX_IMM8_X1_TLS_GD_ADD: Self =
        Self::_PpcEmbRelstHaOrPpc64Dtprel16HighOrArmPrivate2OrTilegxImm8X1TlsGdAdd;
    pub const PPC_EMB_BIT_FLD : Self = Self::_PpcEmbBitFldOrPpc64Dtprel16HighaOrArmPrivate3OrTilegxImm8Y0TlsGdAddOrPariscPltoff14Wr;
    pub const PPC64_DTPREL16_HIGHA : Self = Self::_PpcEmbBitFldOrPpc64Dtprel16HighaOrArmPrivate3OrTilegxImm8Y0TlsGdAddOrPariscPltoff14Wr;
    pub const ARM_PRIVATE_3 : Self = Self::_PpcEmbBitFldOrPpc64Dtprel16HighaOrArmPrivate3OrTilegxImm8Y0TlsGdAddOrPariscPltoff14Wr;
    pub const TILEGX_IMM8_Y0_TLS_GD_ADD : Self = Self::_PpcEmbBitFldOrPpc64Dtprel16HighaOrArmPrivate3OrTilegxImm8Y0TlsGdAddOrPariscPltoff14Wr;
    /// PLT-rel. address, right 14 bits.
    pub const PARISC_PLTOFF14WR : Self = Self::_PpcEmbBitFldOrPpc64Dtprel16HighaOrArmPrivate3OrTilegxImm8Y0TlsGdAddOrPariscPltoff14Wr;
    pub const PPC_EMB_RELSDA : Self = Self::_PpcEmbRelsdaOrPpc64Rel24NotocOrArmPrivate4OrTilegxImm8Y1TlsGdAddOrPariscPltoff14DrOrIa64Ltv32Msb;
    pub const PPC64_REL24_NOTOC : Self = Self::_PpcEmbRelsdaOrPpc64Rel24NotocOrArmPrivate4OrTilegxImm8Y1TlsGdAddOrPariscPltoff14DrOrIa64Ltv32Msb;
    pub const ARM_PRIVATE_4 : Self = Self::_PpcEmbRelsdaOrPpc64Rel24NotocOrArmPrivate4OrTilegxImm8Y1TlsGdAddOrPariscPltoff14DrOrIa64Ltv32Msb;
    pub const TILEGX_IMM8_Y1_TLS_GD_ADD : Self = Self::_PpcEmbRelsdaOrPpc64Rel24NotocOrArmPrivate4OrTilegxImm8Y1TlsGdAddOrPariscPltoff14DrOrIa64Ltv32Msb;
    /// PLT-rel. address, right 14 bits.
    pub const PARISC_PLTOFF14DR : Self = Self::_PpcEmbRelsdaOrPpc64Rel24NotocOrArmPrivate4OrTilegxImm8Y1TlsGdAddOrPariscPltoff14DrOrIa64Ltv32Msb;
    /// symbol + addend, data4 MSB
    pub const IA64_LTV32MSB : Self = Self::_PpcEmbRelsdaOrPpc64Rel24NotocOrArmPrivate4OrTilegxImm8Y1TlsGdAddOrPariscPltoff14DrOrIa64Ltv32Msb;
    pub const PPC64_ADDR64_LOCAL: Self =
        Self::_Ppc64Addr64LocalOrArmPrivate5OrTilegxTlsIeLoadOrPariscPltoff16FOrIa64Ltv32Lsb;
    pub const ARM_PRIVATE_5: Self =
        Self::_Ppc64Addr64LocalOrArmPrivate5OrTilegxTlsIeLoadOrPariscPltoff16FOrIa64Ltv32Lsb;
    pub const TILEGX_TLS_IE_LOAD: Self =
        Self::_Ppc64Addr64LocalOrArmPrivate5OrTilegxTlsIeLoadOrPariscPltoff16FOrIa64Ltv32Lsb;
    /// 16 bits LT-rel. address.
    pub const PARISC_PLTOFF16F: Self =
        Self::_Ppc64Addr64LocalOrArmPrivate5OrTilegxTlsIeLoadOrPariscPltoff16FOrIa64Ltv32Lsb;
    /// symbol + addend, data4 LSB
    pub const IA64_LTV32LSB: Self =
        Self::_Ppc64Addr64LocalOrArmPrivate5OrTilegxTlsIeLoadOrPariscPltoff16FOrIa64Ltv32Lsb;
    pub const PPC64_ENTRY: Self =
        Self::_Ppc64EntryOrArmPrivate6OrTilegxImm8X0TlsAddOrPariscPltoff16WfOrIa64Ltv64Msb;
    pub const ARM_PRIVATE_6: Self =
        Self::_Ppc64EntryOrArmPrivate6OrTilegxImm8X0TlsAddOrPariscPltoff16WfOrIa64Ltv64Msb;
    pub const TILEGX_IMM8_X0_TLS_ADD: Self =
        Self::_Ppc64EntryOrArmPrivate6OrTilegxImm8X0TlsAddOrPariscPltoff16WfOrIa64Ltv64Msb;
    /// 16 bits PLT-rel. address.
    pub const PARISC_PLTOFF16WF: Self =
        Self::_Ppc64EntryOrArmPrivate6OrTilegxImm8X0TlsAddOrPariscPltoff16WfOrIa64Ltv64Msb;
    /// symbol + addend, data8 MSB
    pub const IA64_LTV64MSB: Self =
        Self::_Ppc64EntryOrArmPrivate6OrTilegxImm8X0TlsAddOrPariscPltoff16WfOrIa64Ltv64Msb;
    pub const POWERPC_PLTSEQ : Self = Self::_PowerpcPltseqOrArmPrivate7OrTilegxImm8X1TlsAddOrPariscPltoff16DfOrNds32TlsDescOrIa64Ltv64Lsb;
    pub const ARM_PRIVATE_7 : Self = Self::_PowerpcPltseqOrArmPrivate7OrTilegxImm8X1TlsAddOrPariscPltoff16DfOrNds32TlsDescOrIa64Ltv64Lsb;
    pub const TILEGX_IMM8_X1_TLS_ADD : Self = Self::_PowerpcPltseqOrArmPrivate7OrTilegxImm8X1TlsAddOrPariscPltoff16DfOrNds32TlsDescOrIa64Ltv64Lsb;
    /// 16 bits PLT-rel. address.
    pub const PARISC_PLTOFF16DF : Self = Self::_PowerpcPltseqOrArmPrivate7OrTilegxImm8X1TlsAddOrPariscPltoff16DfOrNds32TlsDescOrIa64Ltv64Lsb;
    pub const NDS32_TLS_DESC : Self = Self::_PowerpcPltseqOrArmPrivate7OrTilegxImm8X1TlsAddOrPariscPltoff16DfOrNds32TlsDescOrIa64Ltv64Lsb;
    /// symbol + addend, data8 LSB
    pub const IA64_LTV64LSB : Self = Self::_PowerpcPltseqOrArmPrivate7OrTilegxImm8X1TlsAddOrPariscPltoff16DfOrNds32TlsDescOrIa64Ltv64Lsb;
    pub const POWERPC_PLTCALL: Self =
        Self::_PowerpcPltcallOrArmPrivate8OrTilegxImm8Y0TlsAddOrPariscLtoffFptr64;
    pub const ARM_PRIVATE_8: Self =
        Self::_PowerpcPltcallOrArmPrivate8OrTilegxImm8Y0TlsAddOrPariscLtoffFptr64;
    pub const TILEGX_IMM8_Y0_TLS_ADD: Self =
        Self::_PowerpcPltcallOrArmPrivate8OrTilegxImm8Y0TlsAddOrPariscLtoffFptr64;
    /// 64 bits LT-rel. function ptr.
    pub const PARISC_LTOFF_FPTR64: Self =
        Self::_PowerpcPltcallOrArmPrivate8OrTilegxImm8Y0TlsAddOrPariscLtoffFptr64;
    pub const PPC64_PLTSEQ_NOTOC: Self =
        Self::_Ppc64PltseqNotocOrArmPrivate9OrTilegxImm8Y1TlsAddOrIa64Pcrel21Bi;
    pub const ARM_PRIVATE_9: Self =
        Self::_Ppc64PltseqNotocOrArmPrivate9OrTilegxImm8Y1TlsAddOrIa64Pcrel21Bi;
    pub const TILEGX_IMM8_Y1_TLS_ADD: Self =
        Self::_Ppc64PltseqNotocOrArmPrivate9OrTilegxImm8Y1TlsAddOrIa64Pcrel21Bi;
    /// @pcrel(sym + add), 21bit inst
    pub const IA64_PCREL21BI: Self =
        Self::_Ppc64PltseqNotocOrArmPrivate9OrTilegxImm8Y1TlsAddOrIa64Pcrel21Bi;
    pub const PPC64_PLTCALL_NOTOC: Self = Self::_Ppc64PltcallNotocOrArmPrivate10OrIa64Pcrel22;
    pub const ARM_PRIVATE_10: Self = Self::_Ppc64PltcallNotocOrArmPrivate10OrIa64Pcrel22;
    /// @pcrel(sym + add), 22bit inst
    pub const IA64_PCREL22: Self = Self::_Ppc64PltcallNotocOrArmPrivate10OrIa64Pcrel22;
    pub const PPC64_PCREL_OPT: Self =
        Self::_Ppc64PcrelOptOrArmPrivate11OrPariscLtoffFptr14WrOrIa64Pcrel64I;
    pub const ARM_PRIVATE_11: Self =
        Self::_Ppc64PcrelOptOrArmPrivate11OrPariscLtoffFptr14WrOrIa64Pcrel64I;
    /// LT-rel. fct. ptr., right 14 bits.
    pub const PARISC_LTOFF_FPTR14WR: Self =
        Self::_Ppc64PcrelOptOrArmPrivate11OrPariscLtoffFptr14WrOrIa64Pcrel64I;
    /// @pcrel(sym + add), 64bit inst
    pub const IA64_PCREL64I: Self =
        Self::_Ppc64PcrelOptOrArmPrivate11OrPariscLtoffFptr14WrOrIa64Pcrel64I;
    pub const PPC64_REL24_P9NOTOC: Self =
        Self::_Ppc64Rel24P9NotocOrArmPrivate12OrPariscLtoffFptr14Dr;
    pub const ARM_PRIVATE_12: Self = Self::_Ppc64Rel24P9NotocOrArmPrivate12OrPariscLtoffFptr14Dr;
    /// LT-rel. fct. ptr., right 14 bits.
    pub const PARISC_LTOFF_FPTR14DR: Self =
        Self::_Ppc64Rel24P9NotocOrArmPrivate12OrPariscLtoffFptr14Dr;
    pub const PPC64_D34 : Self = Self::_Ppc64D34OrTilegxGnuVtinheritOrArmMeTooOrMipsNumOrPariscLoreserveOrPariscCopyOrTileproGnuVtinheritOrIa64Ipltmsb;
    pub const TILEGX_GNU_VTINHERIT : Self = Self::_Ppc64D34OrTilegxGnuVtinheritOrArmMeTooOrMipsNumOrPariscLoreserveOrPariscCopyOrTileproGnuVtinheritOrIa64Ipltmsb;
    pub const ARM_ME_TOO : Self = Self::_Ppc64D34OrTilegxGnuVtinheritOrArmMeTooOrMipsNumOrPariscLoreserveOrPariscCopyOrTileproGnuVtinheritOrIa64Ipltmsb;
    pub const MIPS_NUM : Self = Self::_Ppc64D34OrTilegxGnuVtinheritOrArmMeTooOrMipsNumOrPariscLoreserveOrPariscCopyOrTileproGnuVtinheritOrIa64Ipltmsb;
    pub const PARISC_LORESERVE : Self = Self::_Ppc64D34OrTilegxGnuVtinheritOrArmMeTooOrMipsNumOrPariscLoreserveOrPariscCopyOrTileproGnuVtinheritOrIa64Ipltmsb;
    /// Copy relocation.
    pub const PARISC_COPY : Self = Self::_Ppc64D34OrTilegxGnuVtinheritOrArmMeTooOrMipsNumOrPariscLoreserveOrPariscCopyOrTileproGnuVtinheritOrIa64Ipltmsb;
    /// GNU C++ vtable hierarchy
    pub const TILEPRO_GNU_VTINHERIT : Self = Self::_Ppc64D34OrTilegxGnuVtinheritOrArmMeTooOrMipsNumOrPariscLoreserveOrPariscCopyOrTileproGnuVtinheritOrIa64Ipltmsb;
    /// dynamic reloc, imported PLT, MSB
    pub const IA64_IPLTMSB : Self = Self::_Ppc64D34OrTilegxGnuVtinheritOrArmMeTooOrMipsNumOrPariscLoreserveOrPariscCopyOrTileproGnuVtinheritOrIa64Ipltmsb;
    pub const PPC64_D34_LO : Self = Self::_Ppc64D34LoOrTilegxGnuVtentryOrArmThmTlsDescseq16OrPariscIpltOrArmThmTlsDescseqOrTileproGnuVtentryOrIa64Ipltlsb;
    pub const TILEGX_GNU_VTENTRY : Self = Self::_Ppc64D34LoOrTilegxGnuVtentryOrArmThmTlsDescseq16OrPariscIpltOrArmThmTlsDescseqOrTileproGnuVtentryOrIa64Ipltlsb;
    pub const ARM_THM_TLS_DESCSEQ16 : Self = Self::_Ppc64D34LoOrTilegxGnuVtentryOrArmThmTlsDescseq16OrPariscIpltOrArmThmTlsDescseqOrTileproGnuVtentryOrIa64Ipltlsb;
    /// Dynamic reloc, imported PLT
    pub const PARISC_IPLT : Self = Self::_Ppc64D34LoOrTilegxGnuVtentryOrArmThmTlsDescseq16OrPariscIpltOrArmThmTlsDescseqOrTileproGnuVtentryOrIa64Ipltlsb;
    pub const ARM_THM_TLS_DESCSEQ : Self = Self::_Ppc64D34LoOrTilegxGnuVtentryOrArmThmTlsDescseq16OrPariscIpltOrArmThmTlsDescseqOrTileproGnuVtentryOrIa64Ipltlsb;
    /// GNU C++ vtable member usage
    pub const TILEPRO_GNU_VTENTRY : Self = Self::_Ppc64D34LoOrTilegxGnuVtentryOrArmThmTlsDescseq16OrPariscIpltOrArmThmTlsDescseqOrTileproGnuVtentryOrIa64Ipltlsb;
    /// dynamic reloc, imported PLT, LSB
    pub const IA64_IPLTLSB : Self = Self::_Ppc64D34LoOrTilegxGnuVtentryOrArmThmTlsDescseq16OrPariscIpltOrArmThmTlsDescseqOrTileproGnuVtentryOrIa64Ipltlsb;
    pub const PPC64_D34_HI30: Self =
        Self::_Ppc64D34Hi30OrArmThmTlsDescseq32OrTilegxNumOrPariscEpltOrTileproNum;
    pub const ARM_THM_TLS_DESCSEQ32: Self =
        Self::_Ppc64D34Hi30OrArmThmTlsDescseq32OrTilegxNumOrPariscEpltOrTileproNum;
    pub const TILEGX_NUM: Self =
        Self::_Ppc64D34Hi30OrArmThmTlsDescseq32OrTilegxNumOrPariscEpltOrTileproNum;
    /// Dynamic reloc, exported PLT
    pub const PARISC_EPLT: Self =
        Self::_Ppc64D34Hi30OrArmThmTlsDescseq32OrTilegxNumOrPariscEpltOrTileproNum;
    pub const TILEPRO_NUM: Self =
        Self::_Ppc64D34Hi30OrArmThmTlsDescseq32OrTilegxNumOrPariscEpltOrTileproNum;
    pub const PPC64_D34_HA30: Self = Self::_Ppc64D34Ha30OrArmThmGotBrel12;
    pub const ARM_THM_GOT_BREL12: Self = Self::_Ppc64D34Ha30OrArmThmGotBrel12;
    pub const PPC64_PCREL34: Self = Self::_Ppc64Pcrel34OrIa64Copy;
    /// copy relocation
    pub const IA64_COPY: Self = Self::_Ppc64Pcrel34OrIa64Copy;
    pub const PPC64_GOT_PCREL34: Self = Self::_Ppc64GotPcrel34OrMicromips26S1OrIa64Sub;
    pub const MICROMIPS_26_S1: Self = Self::_Ppc64GotPcrel34OrMicromips26S1OrIa64Sub;
    /// Addend and symbol difference
    pub const IA64_SUB: Self = Self::_Ppc64GotPcrel34OrMicromips26S1OrIa64Sub;
    pub const PPC64_PLT_PCREL34: Self = Self::_Ppc64PltPcrel34OrMicromipsHi16OrIa64Ltoff22X;
    pub const MICROMIPS_HI16: Self = Self::_Ppc64PltPcrel34OrMicromipsHi16OrIa64Ltoff22X;
    /// LTOFF22, relaxable.
    pub const IA64_LTOFF22X: Self = Self::_Ppc64PltPcrel34OrMicromipsHi16OrIa64Ltoff22X;
    pub const PPC64_PLT_PCREL34_NOTOC: Self =
        Self::_Ppc64PltPcrel34NotocOrMicromipsLo16OrIa64Ldxmov;
    pub const MICROMIPS_LO16: Self = Self::_Ppc64PltPcrel34NotocOrMicromipsLo16OrIa64Ldxmov;
    /// Use of LTOFF22X.
    pub const IA64_LDXMOV: Self = Self::_Ppc64PltPcrel34NotocOrMicromipsLo16OrIa64Ldxmov;
    pub const PPC64_ADDR16_HIGHER34: Self =
        Self::_Ppc64Addr16Higher34OrMicromipsGprel16OrArmThmBf16;
    pub const MICROMIPS_GPREL16: Self = Self::_Ppc64Addr16Higher34OrMicromipsGprel16OrArmThmBf16;
    pub const ARM_THM_BF16: Self = Self::_Ppc64Addr16Higher34OrMicromipsGprel16OrArmThmBf16;
    pub const PPC64_ADDR16_HIGHERA34: Self =
        Self::_Ppc64Addr16Highera34OrMicromipsLiteralOrArmThmBf12;
    pub const MICROMIPS_LITERAL: Self = Self::_Ppc64Addr16Highera34OrMicromipsLiteralOrArmThmBf12;
    pub const ARM_THM_BF12: Self = Self::_Ppc64Addr16Highera34OrMicromipsLiteralOrArmThmBf12;
    pub const PPC64_ADDR16_HIGHEST34: Self =
        Self::_Ppc64Addr16Highest34OrMicromipsGot16OrArmThmBf18;
    pub const MICROMIPS_GOT16: Self = Self::_Ppc64Addr16Highest34OrMicromipsGot16OrArmThmBf18;
    pub const ARM_THM_BF18: Self = Self::_Ppc64Addr16Highest34OrMicromipsGot16OrArmThmBf18;
    pub const PPC64_ADDR16_HIGHESTA34: Self = Self::_Ppc64Addr16Highesta34OrMicromipsPc7S1;
    pub const MICROMIPS_PC7_S1: Self = Self::_Ppc64Addr16Highesta34OrMicromipsPc7S1;
    pub const PPC64_REL16_HIGHER34: Self = Self::_Ppc64Rel16Higher34OrMicromipsPc10S1;
    pub const MICROMIPS_PC10_S1: Self = Self::_Ppc64Rel16Higher34OrMicromipsPc10S1;
    pub const PPC64_REL16_HIGHERA34: Self = Self::_Ppc64Rel16Highera34OrMicromipsPc16S1;
    pub const MICROMIPS_PC16_S1: Self = Self::_Ppc64Rel16Highera34OrMicromipsPc16S1;
    pub const PPC64_REL16_HIGHEST34: Self = Self::_Ppc64Rel16Highest34OrMicromipsCall16;
    pub const MICROMIPS_CALL16: Self = Self::_Ppc64Rel16Highest34OrMicromipsCall16;
    pub const PPC64_D28: Self = Self::_Ppc64D28OrShTlsGd32;
    pub const SH_TLS_GD_32: Self = Self::_Ppc64D28OrShTlsGd32;
    pub const PPC64_PCREL28: Self = Self::_Ppc64Pcrel28OrMicromipsGotDispOrIa64Tprel14OrShTlsLd32;
    pub const MICROMIPS_GOT_DISP: Self =
        Self::_Ppc64Pcrel28OrMicromipsGotDispOrIa64Tprel14OrShTlsLd32;
    /// @tprel(sym + add), imm14
    pub const IA64_TPREL14: Self = Self::_Ppc64Pcrel28OrMicromipsGotDispOrIa64Tprel14OrShTlsLd32;
    pub const SH_TLS_LD_32: Self = Self::_Ppc64Pcrel28OrMicromipsGotDispOrIa64Tprel14OrShTlsLd32;
    pub const PPC64_TPREL34: Self = Self::_Ppc64Tprel34OrMicromipsGotPageOrIa64Tprel22OrShTlsLdo32;
    pub const MICROMIPS_GOT_PAGE: Self =
        Self::_Ppc64Tprel34OrMicromipsGotPageOrIa64Tprel22OrShTlsLdo32;
    /// @tprel(sym + add), imm22
    pub const IA64_TPREL22: Self = Self::_Ppc64Tprel34OrMicromipsGotPageOrIa64Tprel22OrShTlsLdo32;
    pub const SH_TLS_LDO_32: Self = Self::_Ppc64Tprel34OrMicromipsGotPageOrIa64Tprel22OrShTlsLdo32;
    pub const PPC64_DTPREL34: Self =
        Self::_Ppc64Dtprel34OrMicromipsGotOfstOrIa64Tprel64IOrShTlsIe32;
    pub const MICROMIPS_GOT_OFST: Self =
        Self::_Ppc64Dtprel34OrMicromipsGotOfstOrIa64Tprel64IOrShTlsIe32;
    /// @tprel(sym + add), imm64
    pub const IA64_TPREL64I: Self = Self::_Ppc64Dtprel34OrMicromipsGotOfstOrIa64Tprel64IOrShTlsIe32;
    pub const SH_TLS_IE_32: Self = Self::_Ppc64Dtprel34OrMicromipsGotOfstOrIa64Tprel64IOrShTlsIe32;
    pub const PPC64_GOT_TLSGD_PCREL34: Self =
        Self::_Ppc64GotTlsgdPcrel34OrMicromipsGotHi16OrShTlsLe32;
    pub const MICROMIPS_GOT_HI16: Self = Self::_Ppc64GotTlsgdPcrel34OrMicromipsGotHi16OrShTlsLe32;
    pub const SH_TLS_LE_32: Self = Self::_Ppc64GotTlsgdPcrel34OrMicromipsGotHi16OrShTlsLe32;
    pub const PPC64_GOT_TLSLD_PCREL34: Self =
        Self::_Ppc64GotTlsldPcrel34OrMicromipsGotLo16OrShTlsDtpmod32;
    pub const MICROMIPS_GOT_LO16: Self =
        Self::_Ppc64GotTlsldPcrel34OrMicromipsGotLo16OrShTlsDtpmod32;
    pub const SH_TLS_DTPMOD32: Self = Self::_Ppc64GotTlsldPcrel34OrMicromipsGotLo16OrShTlsDtpmod32;
    pub const PPC64_GOT_TPREL_PCREL34: Self =
        Self::_Ppc64GotTprelPcrel34OrMicromipsSubOrIa64Tprel64MsbOrShTlsDtpoff32;
    pub const MICROMIPS_SUB: Self =
        Self::_Ppc64GotTprelPcrel34OrMicromipsSubOrIa64Tprel64MsbOrShTlsDtpoff32;
    /// @tprel(sym + add), data8 MSB
    pub const IA64_TPREL64MSB: Self =
        Self::_Ppc64GotTprelPcrel34OrMicromipsSubOrIa64Tprel64MsbOrShTlsDtpoff32;
    pub const SH_TLS_DTPOFF32: Self =
        Self::_Ppc64GotTprelPcrel34OrMicromipsSubOrIa64Tprel64MsbOrShTlsDtpoff32;
    pub const PPC64_GOT_DTPREL_PCREL34: Self =
        Self::_Ppc64GotDtprelPcrel34OrMicromipsHigherOrIa64Tprel64LsbOrShTlsTpoff32;
    pub const MICROMIPS_HIGHER: Self =
        Self::_Ppc64GotDtprelPcrel34OrMicromipsHigherOrIa64Tprel64LsbOrShTlsTpoff32;
    /// @tprel(sym + add), data8 LSB
    pub const IA64_TPREL64LSB: Self =
        Self::_Ppc64GotDtprelPcrel34OrMicromipsHigherOrIa64Tprel64LsbOrShTlsTpoff32;
    pub const SH_TLS_TPOFF32: Self =
        Self::_Ppc64GotDtprelPcrel34OrMicromipsHigherOrIa64Tprel64LsbOrShTlsTpoff32;
    pub const PPC_VLE_LO16A: Self = Self::_PpcVleLo16AOrPariscTprel14Wr;
    /// TP-rel. address, right 14 bits.
    pub const PARISC_TPREL14WR: Self = Self::_PpcVleLo16AOrPariscTprel14Wr;
    pub const PPC_VLE_LO16D: Self = Self::_PpcVleLo16DOrPariscTprel14Dr;
    /// TP-rel. address, right 14 bits.
    pub const PARISC_TPREL14DR: Self = Self::_PpcVleLo16DOrPariscTprel14Dr;
    pub const PPC_VLE_HI16A: Self = Self::_PpcVleHi16AOrPariscTprel16F;
    /// 16 bits TP-rel. address.
    pub const PARISC_TPREL16F: Self = Self::_PpcVleHi16AOrPariscTprel16F;
    pub const PPC_VLE_HI16D: Self = Self::_PpcVleHi16DOrPariscTprel16Wf;
    /// 16 bits TP-rel. address.
    pub const PARISC_TPREL16WF: Self = Self::_PpcVleHi16DOrPariscTprel16Wf;
    pub const PPC_VLE_HA16A: Self = Self::_PpcVleHa16AOrPariscTprel16Df;
    /// 16 bits TP-rel. address.
    pub const PARISC_TPREL16DF: Self = Self::_PpcVleHa16AOrPariscTprel16Df;
    pub const PPC_VLE_HA16D: Self = Self::_PpcVleHa16DOrPariscLtoffTp64;
    /// 64 bits LT-TP-rel. address.
    pub const PARISC_LTOFF_TP64: Self = Self::_PpcVleHa16DOrPariscLtoffTp64;
    pub const PPC_VLE_SDAREL_LO16A: Self = Self::_PpcVleSdarelLo16AOrPariscLtoffTp14Wr;
    /// LT-TP-rel. address, right 14 bits.
    pub const PARISC_LTOFF_TP14WR: Self = Self::_PpcVleSdarelLo16AOrPariscLtoffTp14Wr;
    pub const PPC_VLE_SDAREL_LO16D: Self = Self::_PpcVleSdarelLo16DOrPariscLtoffTp14Dr;
    /// LT-TP-rel. address, right 14 bits.
    pub const PARISC_LTOFF_TP14DR: Self = Self::_PpcVleSdarelLo16DOrPariscLtoffTp14Dr;
    pub const PPC_VLE_SDAREL_HI16A: Self = Self::_PpcVleSdarelHi16AOrPariscLtoffTp16F;
    /// 16 bits LT-TP-rel. address.
    pub const PARISC_LTOFF_TP16F: Self = Self::_PpcVleSdarelHi16AOrPariscLtoffTp16F;
    pub const PPC_VLE_SDAREL_HI16D: Self = Self::_PpcVleSdarelHi16DOrPariscLtoffTp16Wf;
    /// 16 bits LT-TP-rel. address.
    pub const PARISC_LTOFF_TP16WF: Self = Self::_PpcVleSdarelHi16DOrPariscLtoffTp16Wf;
    pub const PPC_VLE_SDAREL_HA16A: Self = Self::_PpcVleSdarelHa16AOrPariscLtoffTp16Df;
    /// 16 bits LT-TP-rel. address.
    pub const PARISC_LTOFF_TP16DF: Self = Self::_PpcVleSdarelHa16AOrPariscLtoffTp16Df;
    pub const PPC_VLE_SDAREL_HA16D: Self = Self::_PpcVleSdarelHa16DOrPariscGnuVtentry;
    pub const PARISC_GNU_VTENTRY: Self = Self::_PpcVleSdarelHa16DOrPariscGnuVtentry;
    pub const PPC64_REL16_HIGH: Self = Self::_Ppc64Rel16HighOrPariscTlsLdo21L;
    /// LD offset 21-bit left.
    pub const PARISC_TLS_LDO21L: Self = Self::_Ppc64Rel16HighOrPariscTlsLdo21L;
    pub const PPC64_REL16_HIGHA: Self = Self::_Ppc64Rel16HighaOrPariscTlsLdo14R;
    /// LD offset 14-bit right.
    pub const PARISC_TLS_LDO14R: Self = Self::_Ppc64Rel16HighaOrPariscTlsLdo14R;
    pub const PPC64_REL16_HIGHER: Self = Self::_Ppc64Rel16HigherOrPariscTlsDtpmod32;
    /// DTP module 32-bit.
    pub const PARISC_TLS_DTPMOD32: Self = Self::_Ppc64Rel16HigherOrPariscTlsDtpmod32;
    pub const PPC64_REL16_HIGHERA: Self = Self::_Ppc64Rel16HigheraOrPariscTlsDtpmod64;
    /// DTP module 64-bit.
    pub const PARISC_TLS_DTPMOD64: Self = Self::_Ppc64Rel16HigheraOrPariscTlsDtpmod64;
    pub const PPC64_REL16_HIGHEST: Self = Self::_Ppc64Rel16HighestOrPariscTlsDtpoff32;
    /// DTP offset 32-bit.
    pub const PARISC_TLS_DTPOFF32: Self = Self::_Ppc64Rel16HighestOrPariscTlsDtpoff32;
    pub const PPC64_REL16_HIGHESTA: Self = Self::_Ppc64Rel16HighestaOrPariscTlsDtpoff64;
    /// DTP offset 32-bit.
    pub const PARISC_TLS_DTPOFF64: Self = Self::_Ppc64Rel16HighestaOrPariscTlsDtpoff64;
    pub const POWERPC_IRELATIVE: Self =
        Self::_PowerpcIrelativeOrMipsPc32OrSparcJmpIrelOrPpcIrelativeOrPpc64Irelative;
    pub const MIPS_PC32: Self =
        Self::_PowerpcIrelativeOrMipsPc32OrSparcJmpIrelOrPpcIrelativeOrPpc64Irelative;
    pub const SPARC_JMP_IREL: Self =
        Self::_PowerpcIrelativeOrMipsPc32OrSparcJmpIrelOrPpcIrelativeOrPpc64Irelative;
    pub const PPC_IRELATIVE: Self =
        Self::_PowerpcIrelativeOrMipsPc32OrSparcJmpIrelOrPpcIrelativeOrPpc64Irelative;
    pub const PPC64_IRELATIVE: Self =
        Self::_PowerpcIrelativeOrMipsPc32OrSparcJmpIrelOrPpcIrelativeOrPpc64Irelative;
    pub const POWERPC_REL16: Self =
        Self::_PowerpcRel16OrMipsEhOrSparcIrelativeOrPpcRel16OrPpc64Rel16OrArmRxpc25;
    pub const MIPS_EH: Self =
        Self::_PowerpcRel16OrMipsEhOrSparcIrelativeOrPpcRel16OrPpc64Rel16OrArmRxpc25;
    pub const SPARC_IRELATIVE: Self =
        Self::_PowerpcRel16OrMipsEhOrSparcIrelativeOrPpcRel16OrPpc64Rel16OrArmRxpc25;
    /// half16   (sym+add-.)
    pub const PPC_REL16: Self =
        Self::_PowerpcRel16OrMipsEhOrSparcIrelativeOrPpcRel16OrPpc64Rel16OrArmRxpc25;
    /// half16   (sym+add-.)
    pub const PPC64_REL16: Self =
        Self::_PowerpcRel16OrMipsEhOrSparcIrelativeOrPpcRel16OrPpc64Rel16OrArmRxpc25;
    pub const ARM_RXPC25: Self =
        Self::_PowerpcRel16OrMipsEhOrSparcIrelativeOrPpcRel16OrPpc64Rel16OrArmRxpc25;
    pub const POWERPC_REL16_HA: Self =
        Self::_PowerpcRel16HaOrSparcRev32OrPpcRel16HaOrPpc64Rel16HaOrArmRrel32;
    pub const SPARC_REV32: Self =
        Self::_PowerpcRel16HaOrSparcRev32OrPpcRel16HaOrPpc64Rel16HaOrArmRrel32;
    /// half16   (sym+add-.)@ha
    pub const PPC_REL16_HA: Self =
        Self::_PowerpcRel16HaOrSparcRev32OrPpcRel16HaOrPpc64Rel16HaOrArmRrel32;
    /// half16   (sym+add-.)@ha
    pub const PPC64_REL16_HA: Self =
        Self::_PowerpcRel16HaOrSparcRev32OrPpcRel16HaOrPpc64Rel16HaOrArmRrel32;
    pub const ARM_RREL32: Self =
        Self::_PowerpcRel16HaOrSparcRev32OrPpcRel16HaOrPpc64Rel16HaOrArmRrel32;
    pub const POWERPC_GNU_VTINHERIT: Self =
        Self::_PowerpcGnuVtinheritOrMipsGnuVtinheritOrSparcNumOrArmRabs22;
    pub const MIPS_GNU_VTINHERIT: Self =
        Self::_PowerpcGnuVtinheritOrMipsGnuVtinheritOrSparcNumOrArmRabs22;
    pub const SPARC_NUM: Self = Self::_PowerpcGnuVtinheritOrMipsGnuVtinheritOrSparcNumOrArmRabs22;
    pub const ARM_RABS22: Self = Self::_PowerpcGnuVtinheritOrMipsGnuVtinheritOrSparcNumOrArmRabs22;
    pub const POWERPC_GNU_VTENTRY: Self = Self::_PowerpcGnuVtentryOrMipsGnuVtentryOrArmRpc24;
    pub const MIPS_GNU_VTENTRY: Self = Self::_PowerpcGnuVtentryOrMipsGnuVtentryOrArmRpc24;
    pub const ARM_RPC24: Self = Self::_PowerpcGnuVtentryOrMipsGnuVtentryOrArmRpc24;
    pub const PPC_TOC16: Self = Self::_PpcToc16OrPariscHireserveOrArmRbase;
    pub const PARISC_HIRESERVE: Self = Self::_PpcToc16OrPariscHireserveOrArmRbase;
    pub const ARM_RBASE: Self = Self::_PpcToc16OrPariscHireserveOrArmRbase;
    pub const MIPS_COPY: Self = Self::_MipsCopyOrArmPrivate14OrPariscLtoffFptr16Wf;
    pub const ARM_PRIVATE_14: Self = Self::_MipsCopyOrArmPrivate14OrPariscLtoffFptr16Wf;
    /// 16 bits LT-rel. function ptr.
    pub const PARISC_LTOFF_FPTR16WF: Self = Self::_MipsCopyOrArmPrivate14OrPariscLtoffFptr16Wf;
    pub const MIPS_JUMP_SLOT: Self = Self::_MipsJumpSlotOrArmPrivate15OrPariscLtoffFptr16Df;
    pub const ARM_PRIVATE_15: Self = Self::_MipsJumpSlotOrArmPrivate15OrPariscLtoffFptr16Df;
    /// 16 bits LT-rel. function ptr.
    pub const PARISC_LTOFF_FPTR16DF: Self = Self::_MipsJumpSlotOrArmPrivate15OrPariscLtoffFptr16Df;
    pub const MICROMIPS_TLS_LDM: Self = Self::_MicromipsTlsLdmOrShGlobDat;
    pub const SH_GLOB_DAT: Self = Self::_MicromipsTlsLdmOrShGlobDat;
    pub const MICROMIPS_TLS_DTPREL_HI16: Self = Self::_MicromipsTlsDtprelHi16OrShJmpSlot;
    pub const SH_JMP_SLOT: Self = Self::_MicromipsTlsDtprelHi16OrShJmpSlot;
    pub const MICROMIPS_TLS_DTPREL_LO16: Self = Self::_MicromipsTlsDtprelLo16OrShRelative;
    pub const SH_RELATIVE: Self = Self::_MicromipsTlsDtprelLo16OrShRelative;
    pub const MICROMIPS_TLS_TPREL_LO16: Self = Self::_MicromipsTlsTprelLo16OrIa64LtoffDtpmod22;
    /// @ltoff(@dtpmod(sym + add)), imm22
    pub const IA64_LTOFF_DTPMOD22: Self = Self::_MicromipsTlsTprelLo16OrIa64LtoffDtpmod22;
    pub const ARM_PRIVATE_13: Self = Self::_ArmPrivate13OrPariscLtoffFptr16F;
    /// 16 bits LT-rel. function ptr.
    pub const PARISC_LTOFF_FPTR16F: Self = Self::_ArmPrivate13OrPariscLtoffFptr16F;
    pub const ARM_IRELATIVE: Self = Self::_ArmIrelativeOrShGot32;
    pub const SH_GOT32: Self = Self::_ArmIrelativeOrShGot32;
    pub const AARCH64_WITHDRAWN: Self = Self::_AArch64WithdrawnOrArmNumOrShNumOrM32RNum;
    pub const ARM_NUM: Self = Self::_AArch64WithdrawnOrArmNumOrShNumOrM32RNum;
    pub const SH_NUM: Self = Self::_AArch64WithdrawnOrArmNumOrShNumOrM32RNum;
    /// Keep this the last entry.
    pub const M32R_NUM: Self = Self::_AArch64WithdrawnOrArmNumOrShNumOrM32RNum;
    pub const AARCH64_TLS_DTPMOD64: Self = Self::_AArch64TlsDtpmod64OrAArch64TlsDtpmod;
    /// Module number, 64 bit.
    pub const AARCH64_TLS_DTPMOD: Self = Self::_AArch64TlsDtpmod64OrAArch64TlsDtpmod;
    pub const AARCH64_TLS_DTPREL64: Self = Self::_AArch64TlsDtprel64OrAArch64TlsDtprel;
    /// Module-relative offset, 64 bit.
    pub const AARCH64_TLS_DTPREL: Self = Self::_AArch64TlsDtprel64OrAArch64TlsDtprel;
    pub const AARCH64_TLS_TPREL64: Self = Self::_AArch64TlsTprel64OrAArch64TlsTprel;
    /// TP-relative offset, 64 bit.
    pub const AARCH64_TLS_TPREL: Self = Self::_AArch64TlsTprel64OrAArch64TlsTprel;
    /// 14 bits LT-TP-rel. address.
    pub const PARISC_LTOFF_TP14F: Self = Self::_PariscLtoffTp14FOrShGotpcOrIa64Dtpmod64Lsb;
    pub const SH_GOTPC: Self = Self::_PariscLtoffTp14FOrShGotpcOrIa64Dtpmod64Lsb;
    /// @dtpmod(sym + add), data8 LSB
    pub const IA64_DTPMOD64LSB: Self = Self::_PariscLtoffTp14FOrShGotpcOrIa64Dtpmod64Lsb;
    /// like EMB_SDA21, but lower 16 bit
    pub const PPC_DIAB_SDA21_LO: Self = Self::_PpcDiabSda21LoOrAArch64P32CopyOrIa64Dtprel32Msb;
    /// Copy symbol at runtime.
    pub const AARCH64_P32_COPY: Self = Self::_PpcDiabSda21LoOrAArch64P32CopyOrIa64Dtprel32Msb;
    /// @dtprel(sym + add), data4 MSB
    pub const IA64_DTPREL32MSB: Self = Self::_PpcDiabSda21LoOrAArch64P32CopyOrIa64Dtprel32Msb;
    /// like EMB_SDA21, but high 16 bit
    pub const PPC_DIAB_SDA21_HI: Self = Self::_PpcDiabSda21HiOrAArch64P32GlobDatOrIa64Dtprel32Lsb;
    /// Create GOT entry.
    pub const AARCH64_P32_GLOB_DAT: Self =
        Self::_PpcDiabSda21HiOrAArch64P32GlobDatOrIa64Dtprel32Lsb;
    /// @dtprel(sym + add), data4 LSB
    pub const IA64_DTPREL32LSB: Self = Self::_PpcDiabSda21HiOrAArch64P32GlobDatOrIa64Dtprel32Lsb;
    /// like EMB_SDA21, adjusted high 16
    pub const PPC_DIAB_SDA21_HA: Self = Self::_PpcDiabSda21HaOrAArch64P32JumpSlotOrIa64Dtprel64Msb;
    /// Create PLT entry.
    pub const AARCH64_P32_JUMP_SLOT: Self =
        Self::_PpcDiabSda21HaOrAArch64P32JumpSlotOrIa64Dtprel64Msb;
    /// @dtprel(sym + add), data8 MSB
    pub const IA64_DTPREL64MSB: Self = Self::_PpcDiabSda21HaOrAArch64P32JumpSlotOrIa64Dtprel64Msb;
    /// like EMB_RELSDA, but lower 16 bit
    pub const PPC_DIAB_RELSDA_LO: Self =
        Self::_PpcDiabRelsdaLoOrAArch64P32RelativeOrIa64Dtprel64Lsb;
    /// Adjust by program base.
    pub const AARCH64_P32_RELATIVE: Self =
        Self::_PpcDiabRelsdaLoOrAArch64P32RelativeOrIa64Dtprel64Lsb;
    /// @dtprel(sym + add), data8 LSB
    pub const IA64_DTPREL64LSB: Self = Self::_PpcDiabRelsdaLoOrAArch64P32RelativeOrIa64Dtprel64Lsb;
    /// like EMB_RELSDA, but high 16 bit
    pub const PPC_DIAB_RELSDA_HI: Self = Self::_PpcDiabRelsdaHiOrAArch64P32TlsDtpmod;
    /// Module number, 32 bit.
    pub const AARCH64_P32_TLS_DTPMOD: Self = Self::_PpcDiabRelsdaHiOrAArch64P32TlsDtpmod;
    /// like EMB_RELSDA, adjusted high 16
    pub const PPC_DIAB_RELSDA_HA: Self = Self::_PpcDiabRelsdaHaOrAArch64P32TlsDtprel;
    /// Module-relative offset, 32 bit.
    pub const AARCH64_P32_TLS_DTPREL: Self = Self::_PpcDiabRelsdaHaOrAArch64P32TlsDtprel;
    /// TP-relative offset, 32 bit.
    pub const AARCH64_P32_TLS_TPREL: Self = Self::_AArch64P32TlsTprelOrIa64LtoffDtprel22;
    /// @ltoff(@dtprel(s+a)), imm22
    pub const IA64_LTOFF_DTPREL22: Self = Self::_AArch64P32TlsTprelOrIa64LtoffDtprel22;
}
