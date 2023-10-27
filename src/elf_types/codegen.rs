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
    /// MIPS R3000 (officially, big-endian only)
    Mips = 0x8,
    /// IBM System/370
    S370 = 0x9,
    /// MIPS R3000 little-endian (Oct 4 1999 Draft).  Deprecated.
    MipsRs3Le = 0xa,
    /// Old version of Sparc v9, from before the ABI.  Deprecated Or Reserved
    _OldSparcv9OrRes011 = 0xb,
    /// Reserved
    Res012 = 0xc,
    /// Reserved
    Res013 = 0xd,
    /// Reserved
    Res014 = 0xe,
    /// HPPA
    Parisc = 0xf,
    /// Reserved
    Res016 = 0x10,
    /// Old version of PowerPC.  Deprecated Or Fujitsu VPP500 Or Fujitsu VPP500
    _PpcOldOrVpp550OrVpp500 = 0x11,
    /// Sun's "v8plus"
    Sparc32Plus = 0x12,
    /// Intel 80960
    Intel960 = 0x13,
    /// PowerPC
    Ppc = 0x14,
    /// 64-bit PowerPC
    Ppc64 = 0x15,
    /// IBM S/390
    S390 = 0x16,
    /// Sony/Toshiba/IBM SPU
    Spu = 0x17,
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
    /// NEC V800 series
    V800 = 0x24,
    /// Fujitsu FR20
    Fr20 = 0x25,
    /// TRW RH32
    Rh32 = 0x26,
    /// Motorola M*Core */ /* May also be taken by Fujitsu MMA Or Old name for MCore Or CskyOld
    _McoreOrRceOrCskyOld = 0x27,
    /// ARM
    Arm = 0x28,
    /// Digital Alpha Or Digital Alpha
    _OldAlphaOrFakeAlpha = 0x29,
    /// Renesas (formerly Hitachi) / SuperH SH
    Sh = 0x2a,
    /// SPARC v9 64-bit
    Sparcv9 = 0x2b,
    /// Siemens Tricore embedded processor
    Tricore = 0x2c,
    /// ARC Cores
    Arc = 0x2d,
    /// Renesas (formerly Hitachi) H8/300
    H8300 = 0x2e,
    /// Renesas (formerly Hitachi) H8/300H
    H8300H = 0x2f,
    /// Renesas (formerly Hitachi) H8S
    H8S = 0x30,
    /// Renesas (formerly Hitachi) H8/500
    H8500 = 0x31,
    /// Intel IA-64 Processor
    Ia64 = 0x32,
    /// Stanford MIPS-X
    MipsX = 0x33,
    /// Motorola Coldfire
    Coldfire = 0x34,
    /// Motorola M68HC12
    Motorola68Hc12 = 0x35,
    /// Fujitsu Multimedia Accelerator
    Mma = 0x36,
    /// Siemens PCP
    Pcp = 0x37,
    /// Sony nCPU embedded RISC processor
    Ncpu = 0x38,
    /// Denso NDR1 microprocessor
    Ndr1 = 0x39,
    /// Motorola Star*Core processor
    Starcore = 0x3a,
    /// Toyota ME16 processor
    Me16 = 0x3b,
    /// STMicroelectronics ST100 processor
    St100 = 0x3c,
    /// Advanced Logic Corp. TinyJ embedded processor
    Tinyj = 0x3d,
    /// Advanced Micro Devices X86-64 processor
    X8664 = 0x3e,
    /// Sony DSP Processor
    Pdsp = 0x3f,
    /// Digital Equipment Corp. PDP-10
    Pdp10 = 0x40,
    /// Digital Equipment Corp. PDP-11
    Pdp11 = 0x41,
    /// Siemens FX66 microcontroller
    Fx66 = 0x42,
    /// STMicroelectronics ST9+ 8/16 bit microcontroller
    St9Plus = 0x43,
    /// STMicroelectronics ST7 8-bit microcontroller
    St7 = 0x44,
    /// Motorola MC68HC16 Microcontroller
    Motorola68Hc16 = 0x45,
    /// Motorola MC68HC11 Microcontroller
    Motorola68Hc11 = 0x46,
    /// Motorola MC68HC08 Microcontroller
    Motorola68Hc08 = 0x47,
    /// Motorola MC68HC05 Microcontroller
    Motorola68Hc05 = 0x48,
    /// Silicon Graphics SVx
    Svx = 0x49,
    /// STMicroelectronics ST19 8-bit cpu
    St19 = 0x4a,
    /// Digital VAX
    Vax = 0x4b,
    /// Axis Communications 32-bit embedded processor
    Cris = 0x4c,
    /// Infineon Technologies 32-bit embedded cpu
    Javelin = 0x4d,
    /// Element 14 64-bit DSP processor
    Firepath = 0x4e,
    /// LSI Logic's 16-bit DSP processor
    Zsp = 0x4f,
    /// Donald Knuth's educational 64-bit processor
    Mmix = 0x50,
    /// Harvard's machine-independent format
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
    /// Renesas V850 (formerly NEC V850)
    V850 = 0x57,
    /// Renesas M32R (formerly Mitsubishi M32R)
    M32R = 0x58,
    /// Matsushita MN10300
    Mn10300 = 0x59,
    /// Matsushita MN10200
    Mn10200 = 0x5a,
    /// picoJava
    Pj = 0x5b,
    /// OpenRISC 1000 32-bit embedded processor Or Openrisc
    _Or1KOrOpenrisc = 0x5c,
    /// ARC International ARCompact processor Or ArcA5
    _ArcCompactOrArcA5 = 0x5d,
    /// Tensilica Xtensa Architecture
    Xtensa = 0x5e,
    /// Old Sunplus S+core7 backend magic number. Written in the absence of an ABI Or Alphamosaic VideoCore processor
    _ScoreOldOrVideocore = 0x5f,
    /// Thompson Multimedia General Purpose Processor
    TmmGpp = 0x60,
    /// National Semiconductor 32000 series
    Ns32K = 0x61,
    /// Tenor Network TPC processor
    Tpc = 0x62,
    /// Old value for picoJava.  Deprecated Or Trebia SNP 1000 processor
    _PjOldOrSnp1K = 0x63,
    /// STMicroelectronics ST200 microcontroller
    St200 = 0x64,
    /// Ubicom IP2022 micro controller
    Ip2K = 0x65,
    /// MAX Processor
    Max = 0x66,
    /// National Semiconductor CompactRISC
    Cr = 0x67,
    /// Fujitsu F2MC16
    F2Mc16 = 0x68,
    /// TI msp430 micro controller
    Msp430 = 0x69,
    /// ADI Blackfin
    Blackfin = 0x6a,
    /// S1C33 Family of Seiko Epson processors
    SeC33 = 0x6b,
    /// Sharp embedded microprocessor
    Sep = 0x6c,
    /// Arca RISC Microprocessor
    Arca = 0x6d,
    /// Microprocessor series from PKU-Unity Ltd. and MPRC of Peking University
    Unicore = 0x6e,
    /// eXcess: 16/32/64-bit configurable embedded CPU
    Excess = 0x6f,
    /// Icera Semiconductor Inc. Deep Execution Processor
    Dxp = 0x70,
    /// Altera Nios II soft-core processor
    AlteraNios2 = 0x71,
    /// National Semiconductor CRX
    Crx = 0x72,
    /// Old, value for National Semiconductor CompactRISC.  Deprecated Or Motorola XGATE embedded processor
    _Cr16OldOrXgate = 0x73,
    /// Infineon C16x/XC16x processor
    C166 = 0x74,
    /// Renesas M16C series microprocessors
    M16C = 0x75,
    /// Microchip Technology dsPIC30F Digital Signal Controller
    Dspic30F = 0x76,
    /// Freescale Communication Engine RISC core
    Ce = 0x77,
    /// Renesas M32C series microprocessors
    M32C = 0x78,
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
    /// Altium TSK3000 core
    Tsk3000 = 0x83,
    /// Freescale RS08 embedded processor
    Rs08 = 0x84,
    /// Reserved Or Analog Devices SHARC family
    _Res133OrSharc = 0x85,
    /// Cyan Technology eCOG2 microprocessor
    Ecog2 = 0x86,
    /// Sunplus Score Or Sunplus S+core7 RISC processor
    _ScoreOrScore7 = 0x87,
    /// New Japan Radio (NJR) 24-bit DSP Processor
    Dsp24 = 0x88,
    /// Broadcom VideoCore III processor
    Videocore3 = 0x89,
    /// RISC processor for Lattice FPGA architecture
    Latticemico32 = 0x8a,
    /// Seiko Epson C17 family
    SeC17 = 0x8b,
    /// Texas Instruments TMS320C6000 DSP family
    TiC6000 = 0x8c,
    /// Texas Instruments TMS320C2000 DSP family
    TiC2000 = 0x8d,
    /// Texas Instruments TMS320C55x DSP family
    TiC5500 = 0x8e,
    /// Reserved Or Texas Instruments App. Specific RISC
    _Res143OrTiArp32 = 0x8f,
    /// Texas Instruments Programmable Realtime Unit
    TiPru = 0x90,
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
    /// STMicroelectronics 64bit VLIW Data Signal Processor
    MmdspPlus = 0xa0,
    /// Cypress M8C microprocessor
    CypressM8C = 0xa1,
    /// Renesas R32C series microprocessors
    R32C = 0xa2,
    /// NXP Semiconductors TriMedia architecture family
    Trimedia = 0xa3,
    /// QUALCOMM DSP6 Processor
    Qdsp6 = 0xa4,
    /// Intel 8051 and variants
    Intel8051 = 0xa5,
    /// STMicroelectronics STxP7x family
    Stxp7X = 0xa6,
    /// Andes Technology compact code size embedded RISC processor family
    Nds32 = 0xa7,
    /// Cyan Technology eCOG1X family Or Cyan Technology eCOG1X family
    _Ecog1OrEcog1X = 0xa8,
    /// Dallas Semiconductor MAXQ30 Core Micro-controllers
    Maxq30 = 0xa9,
    /// New Japan Radio (NJR) 16-bit DSP Processor
    Ximo16 = 0xaa,
    /// M2000 Reconfigurable RISC Microprocessor
    Manik = 0xab,
    /// Cray Inc. NV2 vector architecture
    Craynv2 = 0xac,
    /// Renesas RX family
    Rx = 0xad,
    /// Imagination Technologies Meta processor architecture
    Metag = 0xae,
    /// MCST Elbrus general purpose hardware architecture
    McstElbrus = 0xaf,
    /// Cyan Technology eCOG16 family
    Ecog16 = 0xb0,
    /// National Semiconductor CompactRISC 16-bit processor
    Cr16 = 0xb1,
    /// Freescale Extended Time Processing Unit
    Etpu = 0xb2,
    /// Infineon Technologies SLE9X core
    Sle9X = 0xb3,
    /// Intel L1OM Or Intel L10M
    _L1OmOrL10M = 0xb4,
    /// Intel K1OM Or Intel K10M
    _K1OmOrK10M = 0xb5,
    /// Reserved by Intel
    Intel182 = 0xb6,
    /// ARM 64-bit architecture
    AArch64 = 0xb7,
    /// Reserved by ARM
    Arm184 = 0xb8,
    /// Atmel Corporation 32-bit microprocessor family
    Avr32 = 0xb9,
    /// STMicroeletronics STM8 8-bit microcontroller
    Stm8 = 0xba,
    /// Tilera TILE64 multicore architecture family
    Tile64 = 0xbb,
    /// Tilera TILEPro multicore architecture family
    Tilepro = 0xbc,
    /// Xilinx MicroBlaze 32-bit RISC soft processor core
    Microblaze = 0xbd,
    /// NVIDIA CUDA architecture
    Cuda = 0xbe,
    /// Tilera TILE-Gx multicore architecture family
    Tilegx = 0xbf,
    /// CloudShield architecture family
    Cloudshield = 0xc0,
    /// KIPO-KAIST Core-A 1st generation processor family
    Corea1St = 0xc1,
    /// KIPO-KAIST Core-A 2nd generation processor family
    Corea2Nd = 0xc2,
    /// Synopsys ARCompact V2 Or Synopsys ARCv2 ISA
    _ArcCompact2OrArcv2 = 0xc3,
    /// Open8 8-bit RISC soft processor core
    Open8 = 0xc4,
    /// Renesas RL78 family.
    Rl78 = 0xc5,
    /// Broadcom VideoCore V processor
    Videocore5 = 0xc6,
    /// Renesas 78K0R Or Renesas 78KOR
    _Renesas78K0ROrRenesas78Kor = 0xc7,
    /// Freescale 56800EX Digital Signal Controller (DSC)
    Freescale56800Ex = 0xc8,
    /// Beyond BA1 CPU architecture
    Ba1 = 0xc9,
    /// Beyond BA2 CPU architecture
    Ba2 = 0xca,
    /// XMOS xCORE processor family
    Xcore = 0xcb,
    /// Microchip 8-bit PIC(r) family
    MchpPic = 0xcc,
    /// Intel Graphics Technology
    Intelgt = 0xcd,
    /// Reserved by Intel
    Intel206 = 0xce,
    /// Reserved by Intel
    Intel207 = 0xcf,
    /// Reserved by Intel
    Intel208 = 0xd0,
    /// Reserved by Intel
    Intel209 = 0xd1,
    /// KM211 KM32 32-bit processor
    Km32 = 0xd2,
    /// KM211 KMX32 32-bit processor
    Kmx32 = 0xd3,
    /// KM211 KMX16 16-bit processor Or KM211 KMX16
    _Kmx16OrEmx16 = 0xd4,
    /// KM211 KMX8 8-bit processor Or KM211 KMX8
    _Kmx8OrEmx8 = 0xd5,
    /// KM211 KVARC processor
    Kvarc = 0xd6,
    /// Paneve CDP architecture family
    Cdp = 0xd7,
    /// Cognitive Smart Memory Processor
    Coge = 0xd8,
    /// Bluechip Systems CoolEngine
    Cool = 0xd9,
    /// Nanoradio Optimized RISC
    Norc = 0xda,
    /// CSR Kalimba architecture family
    CsrKalimba = 0xdb,
    /// Zilog Z80
    Z80 = 0xdc,
    /// Controls and Data Services VISIUMcore processor
    Visium = 0xdd,
    /// FTDI Chip FT32 high performance 32-bit RISC architecture
    Ft32 = 0xde,
    /// Moxie processor family
    Moxie = 0xdf,
    /// AMD GPU architecture
    Amdgpu = 0xe0,
    /// RISC-V
    Riscv = 0xf3,
    /// Lanai 32-bit processor.
    Lanai = 0xf4,
    /// CEVA Processor Architecture Family
    Ceva = 0xf5,
    /// CEVA X2 Processor Family
    CevaX2 = 0xf6,
    /// Linux BPF – in-kernel virtual machine.
    Bpf = 0xf7,
    /// Graphcore Intelligent Processing Unit
    GraphcoreIpu = 0xf8,
    /// Imagination Technologies
    Img1 = 0xf9,
    /// Netronome Flow Processor.
    Nfp = 0xfa,
    /// NEC Vector Engine
    Ve = 0xfb,
    /// C-SKY processor family.
    Csky = 0xfc,
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
    Alpha = 0x9026,
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
    /// Old version of Sparc v9, from before the ABI.  Deprecated.
    #[allow(non_upper_case_globals)]
    pub const OldSparcv9: Self = Self::_OldSparcv9OrRes011;
    /// Reserved
    #[allow(non_upper_case_globals)]
    pub const Res011: Self = Self::_OldSparcv9OrRes011;
    /// Old version of PowerPC.  Deprecated.
    #[allow(non_upper_case_globals)]
    pub const PpcOld: Self = Self::_PpcOldOrVpp550OrVpp500;
    /// Fujitsu VPP500
    #[allow(non_upper_case_globals)]
    pub const Vpp550: Self = Self::_PpcOldOrVpp550OrVpp500;
    /// Fujitsu VPP500
    #[allow(non_upper_case_globals)]
    pub const Vpp500: Self = Self::_PpcOldOrVpp550OrVpp500;
    /// Motorola M*Core */ /* May also be taken by Fujitsu MMA
    #[allow(non_upper_case_globals)]
    pub const Mcore: Self = Self::_McoreOrRceOrCskyOld;
    /// Old name for MCore
    #[allow(non_upper_case_globals)]
    pub const Rce: Self = Self::_McoreOrRceOrCskyOld;
    #[allow(non_upper_case_globals)]
    pub const CskyOld: Self = Self::_McoreOrRceOrCskyOld;
    /// Digital Alpha
    #[allow(non_upper_case_globals)]
    pub const OldAlpha: Self = Self::_OldAlphaOrFakeAlpha;
    /// Digital Alpha
    #[allow(non_upper_case_globals)]
    pub const FakeAlpha: Self = Self::_OldAlphaOrFakeAlpha;
    /// OpenRISC 1000 32-bit embedded processor
    #[allow(non_upper_case_globals)]
    pub const Or1K: Self = Self::_Or1KOrOpenrisc;
    #[allow(non_upper_case_globals)]
    pub const Openrisc: Self = Self::_Or1KOrOpenrisc;
    /// ARC International ARCompact processor
    #[allow(non_upper_case_globals)]
    pub const ArcCompact: Self = Self::_ArcCompactOrArcA5;
    #[allow(non_upper_case_globals)]
    pub const ArcA5: Self = Self::_ArcCompactOrArcA5;
    /// Old Sunplus S+core7 backend magic number. Written in the absence of an ABI.
    #[allow(non_upper_case_globals)]
    pub const ScoreOld: Self = Self::_ScoreOldOrVideocore;
    /// Alphamosaic VideoCore processor
    #[allow(non_upper_case_globals)]
    pub const Videocore: Self = Self::_ScoreOldOrVideocore;
    /// Old value for picoJava.  Deprecated.
    #[allow(non_upper_case_globals)]
    pub const PjOld: Self = Self::_PjOldOrSnp1K;
    /// Trebia SNP 1000 processor
    #[allow(non_upper_case_globals)]
    pub const Snp1K: Self = Self::_PjOldOrSnp1K;
    /// Old, value for National Semiconductor CompactRISC.  Deprecated.
    #[allow(non_upper_case_globals)]
    pub const Cr16Old: Self = Self::_Cr16OldOrXgate;
    /// Motorola XGATE embedded processor
    #[allow(non_upper_case_globals)]
    pub const Xgate: Self = Self::_Cr16OldOrXgate;
    /// Reserved
    #[allow(non_upper_case_globals)]
    pub const Res133: Self = Self::_Res133OrSharc;
    /// Analog Devices SHARC family
    #[allow(non_upper_case_globals)]
    pub const Sharc: Self = Self::_Res133OrSharc;
    /// Sunplus Score
    #[allow(non_upper_case_globals)]
    pub const Score: Self = Self::_ScoreOrScore7;
    /// Sunplus S+core7 RISC processor
    #[allow(non_upper_case_globals)]
    pub const Score7: Self = Self::_ScoreOrScore7;
    /// Reserved
    #[allow(non_upper_case_globals)]
    pub const Res143: Self = Self::_Res143OrTiArp32;
    /// Texas Instruments App. Specific RISC
    #[allow(non_upper_case_globals)]
    pub const TiArp32: Self = Self::_Res143OrTiArp32;
    /// Cyan Technology eCOG1X family
    #[allow(non_upper_case_globals)]
    pub const Ecog1: Self = Self::_Ecog1OrEcog1X;
    /// Cyan Technology eCOG1X family
    #[allow(non_upper_case_globals)]
    pub const Ecog1X: Self = Self::_Ecog1OrEcog1X;
    /// Intel L1OM
    #[allow(non_upper_case_globals)]
    pub const L1Om: Self = Self::_L1OmOrL10M;
    /// Intel L10M
    #[allow(non_upper_case_globals)]
    pub const L10M: Self = Self::_L1OmOrL10M;
    /// Intel K1OM
    #[allow(non_upper_case_globals)]
    pub const K1Om: Self = Self::_K1OmOrK10M;
    /// Intel K10M
    #[allow(non_upper_case_globals)]
    pub const K10M: Self = Self::_K1OmOrK10M;
    /// Synopsys ARCompact V2
    #[allow(non_upper_case_globals)]
    pub const ArcCompact2: Self = Self::_ArcCompact2OrArcv2;
    /// Synopsys ARCv2 ISA.
    #[allow(non_upper_case_globals)]
    pub const Arcv2: Self = Self::_ArcCompact2OrArcv2;
    /// Renesas 78K0R.
    #[allow(non_upper_case_globals)]
    pub const Renesas78K0R: Self = Self::_Renesas78K0ROrRenesas78Kor;
    /// Renesas 78KOR
    #[allow(non_upper_case_globals)]
    pub const Renesas78Kor: Self = Self::_Renesas78K0ROrRenesas78Kor;
    /// KM211 KMX16 16-bit processor
    #[allow(non_upper_case_globals)]
    pub const Kmx16: Self = Self::_Kmx16OrEmx16;
    /// KM211 KMX16
    #[allow(non_upper_case_globals)]
    pub const Emx16: Self = Self::_Kmx16OrEmx16;
    /// KM211 KMX8 8-bit processor
    #[allow(non_upper_case_globals)]
    pub const Kmx8: Self = Self::_Kmx8OrEmx8;
    /// KM211 KMX8
    #[allow(non_upper_case_globals)]
    pub const Emx8: Self = Self::_Kmx8OrEmx8;
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ProgramHeaderType {
    /// SpuInfo Or PariscArchext Or MipsReginfo Or C6000Phattr Or Processor-specific Or S390Pgste Or Arch extension bits, Or AArch64Archext
    _SpuInfoOrPariscArchextOrMipsReginfoOrC6000PhattrOrLoprocOrS390PgsteOrIa64ArchextOrAArch64Archext =
        0x70000000,
    _RiscvAttributesOrMipsAbiflags = 0x70000003,
    /// IA64 unwind bits Or ArmExidx Or PariscUnwind Or MipsRtproc
    _Ia64UnwindOrArmExidxOrPariscUnwindOrMipsRtproc = 0x70000001,
    _Ia64HpOptAnotOrHpOptAnnot = 0x60000012,
    _Ia64HpHslAnotOrHpHslAnnot = 0x60000013,
    _Ia64HpStackOrHpStack = 0x60000014,
    _PariscWeakorderOrMipsOptionsOrAArch64MemtagMte = 0x70000002,
    /// HpTls Or OS-specific
    _HpTlsOrLoos = 0x60000000,
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
    HpCoreUtsname = 0x60000015,
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
    /// Reserved, unspecified semantics
    Shlib = 0x5,
    /// Entry for header table itself
    Phdr = 0x6,
    /// Thread local storage segment
    Tls = 0x7,
    /// OS-specific Or Hisunw
    _HiosOrHisunw = 0x6fffffff,
    /// Processor-specific
    Hiproc = 0x7fffffff,
    /// Frame unwind information Or Solaris uses the same value
    _GnuEhFrameOrSunwEhFrame = 0x6474e550,
    /// Stack flags
    GnuStack = 0x6474e551,
    /// Read-only after relocation
    GnuRelro = 0x6474e552,
    /// GNU property
    GnuProperty = 0x6474e553,
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
    /// Losunw Or Sun Specific segment
    _LosunwOrSunwbss = 0x6ffffffa,
    /// Stack segment
    Sunwstack = 0x6ffffffb,
}
impl ProgramHeaderType {
    #[allow(non_upper_case_globals)]
    pub const SpuInfo : Self = Self::_SpuInfoOrPariscArchextOrMipsReginfoOrC6000PhattrOrLoprocOrS390PgsteOrIa64ArchextOrAArch64Archext;
    #[allow(non_upper_case_globals)]
    pub const PariscArchext : Self = Self::_SpuInfoOrPariscArchextOrMipsReginfoOrC6000PhattrOrLoprocOrS390PgsteOrIa64ArchextOrAArch64Archext;
    #[allow(non_upper_case_globals)]
    pub const MipsReginfo : Self = Self::_SpuInfoOrPariscArchextOrMipsReginfoOrC6000PhattrOrLoprocOrS390PgsteOrIa64ArchextOrAArch64Archext;
    #[allow(non_upper_case_globals)]
    pub const C6000Phattr : Self = Self::_SpuInfoOrPariscArchextOrMipsReginfoOrC6000PhattrOrLoprocOrS390PgsteOrIa64ArchextOrAArch64Archext;
    /// Processor-specific
    #[allow(non_upper_case_globals)]
    pub const Loproc : Self = Self::_SpuInfoOrPariscArchextOrMipsReginfoOrC6000PhattrOrLoprocOrS390PgsteOrIa64ArchextOrAArch64Archext;
    #[allow(non_upper_case_globals)]
    pub const S390Pgste : Self = Self::_SpuInfoOrPariscArchextOrMipsReginfoOrC6000PhattrOrLoprocOrS390PgsteOrIa64ArchextOrAArch64Archext;
    /// Arch extension bits,
    #[allow(non_upper_case_globals)]
    pub const Ia64Archext : Self = Self::_SpuInfoOrPariscArchextOrMipsReginfoOrC6000PhattrOrLoprocOrS390PgsteOrIa64ArchextOrAArch64Archext;
    #[allow(non_upper_case_globals)]
    pub const AArch64Archext : Self = Self::_SpuInfoOrPariscArchextOrMipsReginfoOrC6000PhattrOrLoprocOrS390PgsteOrIa64ArchextOrAArch64Archext;
    #[allow(non_upper_case_globals)]
    pub const RiscvAttributes: Self = Self::_RiscvAttributesOrMipsAbiflags;
    #[allow(non_upper_case_globals)]
    pub const MipsAbiflags: Self = Self::_RiscvAttributesOrMipsAbiflags;
    /// IA64 unwind bits.
    #[allow(non_upper_case_globals)]
    pub const Ia64Unwind: Self = Self::_Ia64UnwindOrArmExidxOrPariscUnwindOrMipsRtproc;
    #[allow(non_upper_case_globals)]
    pub const ArmExidx: Self = Self::_Ia64UnwindOrArmExidxOrPariscUnwindOrMipsRtproc;
    #[allow(non_upper_case_globals)]
    pub const PariscUnwind: Self = Self::_Ia64UnwindOrArmExidxOrPariscUnwindOrMipsRtproc;
    #[allow(non_upper_case_globals)]
    pub const MipsRtproc: Self = Self::_Ia64UnwindOrArmExidxOrPariscUnwindOrMipsRtproc;
    #[allow(non_upper_case_globals)]
    pub const Ia64HpOptAnot: Self = Self::_Ia64HpOptAnotOrHpOptAnnot;
    #[allow(non_upper_case_globals)]
    pub const HpOptAnnot: Self = Self::_Ia64HpOptAnotOrHpOptAnnot;
    #[allow(non_upper_case_globals)]
    pub const Ia64HpHslAnot: Self = Self::_Ia64HpHslAnotOrHpHslAnnot;
    #[allow(non_upper_case_globals)]
    pub const HpHslAnnot: Self = Self::_Ia64HpHslAnotOrHpHslAnnot;
    #[allow(non_upper_case_globals)]
    pub const Ia64HpStack: Self = Self::_Ia64HpStackOrHpStack;
    #[allow(non_upper_case_globals)]
    pub const HpStack: Self = Self::_Ia64HpStackOrHpStack;
    #[allow(non_upper_case_globals)]
    pub const PariscWeakorder: Self = Self::_PariscWeakorderOrMipsOptionsOrAArch64MemtagMte;
    #[allow(non_upper_case_globals)]
    pub const MipsOptions: Self = Self::_PariscWeakorderOrMipsOptionsOrAArch64MemtagMte;
    #[allow(non_upper_case_globals)]
    pub const AArch64MemtagMte: Self = Self::_PariscWeakorderOrMipsOptionsOrAArch64MemtagMte;
    #[allow(non_upper_case_globals)]
    pub const HpTls: Self = Self::_HpTlsOrLoos;
    /// OS-specific
    #[allow(non_upper_case_globals)]
    pub const Loos: Self = Self::_HpTlsOrLoos;
    /// OS-specific
    #[allow(non_upper_case_globals)]
    pub const Hios: Self = Self::_HiosOrHisunw;
    #[allow(non_upper_case_globals)]
    pub const Hisunw: Self = Self::_HiosOrHisunw;
    /// Frame unwind information
    #[allow(non_upper_case_globals)]
    pub const GnuEhFrame: Self = Self::_GnuEhFrameOrSunwEhFrame;
    /// Solaris uses the same value
    #[allow(non_upper_case_globals)]
    pub const SunwEhFrame: Self = Self::_GnuEhFrameOrSunwEhFrame;
    #[allow(non_upper_case_globals)]
    pub const Losunw: Self = Self::_LosunwOrSunwbss;
    /// Sun Specific segment
    #[allow(non_upper_case_globals)]
    pub const Sunwbss: Self = Self::_LosunwOrSunwbss;
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
    /// V850Scommon Or PariscExt Or MipsLiblist Or Processor-specific semantics, lo Or Extension bits Or SparcGotdata
    _V850ScommonOrPariscExtOrMipsLiblistOrLoprocOrIa64ExtOrSparcGotdata = 0x70000000,
    /// V850Zcommon Or PariscDoc Or MipsConflict Or Section pre-emption details Or C6000Preemptmap Or AlphaReginfo Or NfpInitreg
    _V850ZcommonOrPariscDocOrMipsConflictOrArmPreemptmapOrC6000PreemptmapOrAlphaReginfoOrNfpInitreg =
        0x70000002,
    /// Used by Renesas linker Or NfpUdebug Or Application-specific semantics
    _RenesasIopOrNfpUdebugOrLouser = 0x80000000,
    RenesasInfo = 0xa0000000,
    /// Section holds attributes Or PariscAnnot Or MipsGptab Or Section holds ABI attributes Or Section holds attributes Or C6000Attributes Or Section holds attributes
    _RiscvAttributesOrPariscAnnotOrMipsGptabOrMsp430AttributesOrArmAttributesOrC6000AttributesOrAArch64Attributes =
        0x70000003,
    /// Link editor is to sort the 						   entries in this section 						   based on the address 						   specified in the associated 						   symbol table entry Or Processor-specific semantics, hi
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
    GnuIncrementalSymtab = 0x6fff4701,
    GnuIncrementalRelocs = 0x6fff4702,
    GnuIncrementalGotPlt = 0x6fff4703,
    /// Checksum for DSO content.
    Checksum = 0x6ffffff8,
    /// Sun-specific low bound Or SunwMove
    _LosunwOrSunwMove = 0x6ffffffa,
    SunwComdat = 0x6ffffffb,
    SunwSyminfo = 0x6ffffffc,
}
impl SectionHeaderType {
    /// unwind information
    #[allow(non_upper_case_globals)]
    pub const X8664Unwind : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    #[allow(non_upper_case_globals)]
    pub const V850Tcommon : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    #[allow(non_upper_case_globals)]
    pub const PariscUnwind : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    #[allow(non_upper_case_globals)]
    pub const MipsMsym : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    /// Section holds ARM unwind info.
    #[allow(non_upper_case_globals)]
    pub const ArmExidx : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    #[allow(non_upper_case_globals)]
    pub const C6000Unwind : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    /// Section holds attributes.
    #[allow(non_upper_case_globals)]
    pub const ArcAttributes : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    /// Section holds attributes.
    #[allow(non_upper_case_globals)]
    pub const CskyAttributes : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    #[allow(non_upper_case_globals)]
    pub const AlphaDebug : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    /// Unwind bits.
    #[allow(non_upper_case_globals)]
    pub const Ia64Unwind : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    #[allow(non_upper_case_globals)]
    pub const NfpMeconfig : Self = Self::_X8664UnwindOrV850TcommonOrPariscUnwindOrMipsMsymOrArmExidxOrC6000UnwindOrArcAttributesOrCskyAttributesOrAlphaDebugOrIa64UnwindOrNfpMeconfig;
    #[allow(non_upper_case_globals)]
    pub const V850Scommon: Self =
        Self::_V850ScommonOrPariscExtOrMipsLiblistOrLoprocOrIa64ExtOrSparcGotdata;
    #[allow(non_upper_case_globals)]
    pub const PariscExt: Self =
        Self::_V850ScommonOrPariscExtOrMipsLiblistOrLoprocOrIa64ExtOrSparcGotdata;
    #[allow(non_upper_case_globals)]
    pub const MipsLiblist: Self =
        Self::_V850ScommonOrPariscExtOrMipsLiblistOrLoprocOrIa64ExtOrSparcGotdata;
    /// Processor-specific semantics, lo
    #[allow(non_upper_case_globals)]
    pub const Loproc: Self =
        Self::_V850ScommonOrPariscExtOrMipsLiblistOrLoprocOrIa64ExtOrSparcGotdata;
    /// Extension bits.
    #[allow(non_upper_case_globals)]
    pub const Ia64Ext: Self =
        Self::_V850ScommonOrPariscExtOrMipsLiblistOrLoprocOrIa64ExtOrSparcGotdata;
    #[allow(non_upper_case_globals)]
    pub const SparcGotdata: Self =
        Self::_V850ScommonOrPariscExtOrMipsLiblistOrLoprocOrIa64ExtOrSparcGotdata;
    #[allow(non_upper_case_globals)]
    pub const V850Zcommon : Self = Self::_V850ZcommonOrPariscDocOrMipsConflictOrArmPreemptmapOrC6000PreemptmapOrAlphaReginfoOrNfpInitreg;
    #[allow(non_upper_case_globals)]
    pub const PariscDoc : Self = Self::_V850ZcommonOrPariscDocOrMipsConflictOrArmPreemptmapOrC6000PreemptmapOrAlphaReginfoOrNfpInitreg;
    #[allow(non_upper_case_globals)]
    pub const MipsConflict : Self = Self::_V850ZcommonOrPariscDocOrMipsConflictOrArmPreemptmapOrC6000PreemptmapOrAlphaReginfoOrNfpInitreg;
    /// Section pre-emption details.
    #[allow(non_upper_case_globals)]
    pub const ArmPreemptmap : Self = Self::_V850ZcommonOrPariscDocOrMipsConflictOrArmPreemptmapOrC6000PreemptmapOrAlphaReginfoOrNfpInitreg;
    #[allow(non_upper_case_globals)]
    pub const C6000Preemptmap : Self = Self::_V850ZcommonOrPariscDocOrMipsConflictOrArmPreemptmapOrC6000PreemptmapOrAlphaReginfoOrNfpInitreg;
    #[allow(non_upper_case_globals)]
    pub const AlphaReginfo : Self = Self::_V850ZcommonOrPariscDocOrMipsConflictOrArmPreemptmapOrC6000PreemptmapOrAlphaReginfoOrNfpInitreg;
    #[allow(non_upper_case_globals)]
    pub const NfpInitreg : Self = Self::_V850ZcommonOrPariscDocOrMipsConflictOrArmPreemptmapOrC6000PreemptmapOrAlphaReginfoOrNfpInitreg;
    /// Used by Renesas linker.
    #[allow(non_upper_case_globals)]
    pub const RenesasIop: Self = Self::_RenesasIopOrNfpUdebugOrLouser;
    #[allow(non_upper_case_globals)]
    pub const NfpUdebug: Self = Self::_RenesasIopOrNfpUdebugOrLouser;
    /// Application-specific semantics
    #[allow(non_upper_case_globals)]
    pub const Louser: Self = Self::_RenesasIopOrNfpUdebugOrLouser;
    /// Section holds attributes.
    #[allow(non_upper_case_globals)]
    pub const RiscvAttributes : Self = Self::_RiscvAttributesOrPariscAnnotOrMipsGptabOrMsp430AttributesOrArmAttributesOrC6000AttributesOrAArch64Attributes;
    #[allow(non_upper_case_globals)]
    pub const PariscAnnot : Self = Self::_RiscvAttributesOrPariscAnnotOrMipsGptabOrMsp430AttributesOrArmAttributesOrC6000AttributesOrAArch64Attributes;
    #[allow(non_upper_case_globals)]
    pub const MipsGptab : Self = Self::_RiscvAttributesOrPariscAnnotOrMipsGptabOrMsp430AttributesOrArmAttributesOrC6000AttributesOrAArch64Attributes;
    /// Section holds ABI attributes.
    #[allow(non_upper_case_globals)]
    pub const Msp430Attributes : Self = Self::_RiscvAttributesOrPariscAnnotOrMipsGptabOrMsp430AttributesOrArmAttributesOrC6000AttributesOrAArch64Attributes;
    /// Section holds attributes.
    #[allow(non_upper_case_globals)]
    pub const ArmAttributes : Self = Self::_RiscvAttributesOrPariscAnnotOrMipsGptabOrMsp430AttributesOrArmAttributesOrC6000AttributesOrAArch64Attributes;
    #[allow(non_upper_case_globals)]
    pub const C6000Attributes : Self = Self::_RiscvAttributesOrPariscAnnotOrMipsGptabOrMsp430AttributesOrArmAttributesOrC6000AttributesOrAArch64Attributes;
    /// Section holds attributes.
    #[allow(non_upper_case_globals)]
    pub const AArch64Attributes : Self = Self::_RiscvAttributesOrPariscAnnotOrMipsGptabOrMsp430AttributesOrArmAttributesOrC6000AttributesOrAArch64Attributes;
    /// Link editor is to sort the 						   entries in this section 						   based on the address 						   specified in the associated 						   symbol table entry.
    #[allow(non_upper_case_globals)]
    pub const Ordered: Self = Self::_OrderedOrHiproc;
    /// Processor-specific semantics, hi
    #[allow(non_upper_case_globals)]
    pub const Hiproc: Self = Self::_OrderedOrHiproc;
    #[allow(non_upper_case_globals)]
    pub const Ia64HpOptAnot: Self = Self::_Ia64HpOptAnotOrIa64VmsLinkagesOrHpAnnot;
    #[allow(non_upper_case_globals)]
    pub const Ia64VmsLinkages: Self = Self::_Ia64HpOptAnotOrIa64VmsLinkagesOrHpAnnot;
    #[allow(non_upper_case_globals)]
    pub const HpAnnot: Self = Self::_Ia64HpOptAnotOrIa64VmsLinkagesOrHpAnnot;
    #[allow(non_upper_case_globals)]
    pub const Ia64VmsTrace: Self = Self::_Ia64VmsTraceOrLoosOrHpOvlbits;
    /// First of OS specific semantics
    #[allow(non_upper_case_globals)]
    pub const Loos: Self = Self::_Ia64VmsTraceOrLoosOrHpOvlbits;
    #[allow(non_upper_case_globals)]
    pub const HpOvlbits: Self = Self::_Ia64VmsTraceOrLoosOrHpOvlbits;
    #[allow(non_upper_case_globals)]
    pub const Ia64VmsTieSignatures: Self = Self::_Ia64VmsTieSignaturesOrHpDlkm;
    #[allow(non_upper_case_globals)]
    pub const HpDlkm: Self = Self::_Ia64VmsTieSignaturesOrHpDlkm;
    #[allow(non_upper_case_globals)]
    pub const Ia64VmsDebug: Self = Self::_Ia64VmsDebugOrHpComdat;
    #[allow(non_upper_case_globals)]
    pub const HpComdat: Self = Self::_Ia64VmsDebugOrHpComdat;
    #[allow(non_upper_case_globals)]
    pub const Ia64VmsDebugStr: Self = Self::_Ia64VmsDebugStrOrHpObjdict;
    #[allow(non_upper_case_globals)]
    pub const HpObjdict: Self = Self::_Ia64VmsDebugStrOrHpObjdict;
    #[allow(non_upper_case_globals)]
    pub const PariscDlkm: Self = Self::_PariscDlkmOrMipsUcodeOrArmDebugoverlay;
    #[allow(non_upper_case_globals)]
    pub const MipsUcode: Self = Self::_PariscDlkmOrMipsUcodeOrArmDebugoverlay;
    /// Section holds overlay debug info.
    #[allow(non_upper_case_globals)]
    pub const ArmDebugoverlay: Self = Self::_PariscDlkmOrMipsUcodeOrArmDebugoverlay;
    #[allow(non_upper_case_globals)]
    pub const PariscSymextn: Self = Self::_PariscSymextnOrMipsPacksym;
    #[allow(non_upper_case_globals)]
    pub const MipsPacksym: Self = Self::_PariscSymextnOrMipsPacksym;
    #[allow(non_upper_case_globals)]
    pub const PariscStubs: Self = Self::_PariscStubsOrMipsReld;
    #[allow(non_upper_case_globals)]
    pub const MipsReld: Self = Self::_PariscStubsOrMipsReld;
    #[allow(non_upper_case_globals)]
    pub const MipsDebug: Self = Self::_MipsDebugOrArmOverlaysection;
    /// Section holds GDB and overlay integration info.
    #[allow(non_upper_case_globals)]
    pub const ArmOverlaysection: Self = Self::_MipsDebugOrArmOverlaysection;
    /// Last of OS specific semantics
    #[allow(non_upper_case_globals)]
    pub const Hios: Self = Self::_HiosOrSunwVersymOrHisunwOrGnuVersym;
    /// Symbol versions
    #[allow(non_upper_case_globals)]
    pub const SunwVersym: Self = Self::_HiosOrSunwVersymOrHisunwOrGnuVersym;
    /// Sun-specific high bound.
    #[allow(non_upper_case_globals)]
    pub const Hisunw: Self = Self::_HiosOrSunwVersymOrHisunwOrGnuVersym;
    #[allow(non_upper_case_globals)]
    pub const GnuVersym: Self = Self::_HiosOrSunwVersymOrHisunwOrGnuVersym;
    /// Versions defined by file
    #[allow(non_upper_case_globals)]
    pub const SunwVerdef: Self = Self::_SunwVerdefOrGnuVerdef;
    #[allow(non_upper_case_globals)]
    pub const GnuVerdef: Self = Self::_SunwVerdefOrGnuVerdef;
    /// Versions needed by file
    #[allow(non_upper_case_globals)]
    pub const SunwVerneed: Self = Self::_SunwVerneedOrGnuVerneed;
    #[allow(non_upper_case_globals)]
    pub const GnuVerneed: Self = Self::_SunwVerneedOrGnuVerneed;
    /// Sun-specific low bound.
    #[allow(non_upper_case_globals)]
    pub const Losunw: Self = Self::_LosunwOrSunwMove;
    #[allow(non_upper_case_globals)]
    pub const SunwMove: Self = Self::_LosunwOrSunwMove;
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

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
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
    #[allow(non_upper_case_globals)]
    pub const PariscMilli: Self =
        Self::_PariscMilliOrRegisterOrLoprocOrSparcRegisterOrPariscMillicodeOrArmTfunc;
    /// global reg reserved to app.
    #[allow(non_upper_case_globals)]
    pub const Register: Self =
        Self::_PariscMilliOrRegisterOrLoprocOrSparcRegisterOrPariscMillicodeOrArmTfunc;
    /// Processor-specific semantics
    #[allow(non_upper_case_globals)]
    pub const Loproc: Self =
        Self::_PariscMilliOrRegisterOrLoprocOrSparcRegisterOrPariscMillicodeOrArmTfunc;
    /// Global register reserved to app.
    #[allow(non_upper_case_globals)]
    pub const SparcRegister: Self =
        Self::_PariscMilliOrRegisterOrLoprocOrSparcRegisterOrPariscMillicodeOrArmTfunc;
    /// Millicode function entry point.
    #[allow(non_upper_case_globals)]
    pub const PariscMillicode: Self =
        Self::_PariscMilliOrRegisterOrLoprocOrSparcRegisterOrPariscMillicodeOrArmTfunc;
    /// A Thumb function.
    #[allow(non_upper_case_globals)]
    pub const ArmTfunc: Self =
        Self::_PariscMilliOrRegisterOrLoprocOrSparcRegisterOrPariscMillicodeOrArmTfunc;
    #[allow(non_upper_case_globals)]
    pub const HpStub: Self = Self::_HpStubOrHios;
    /// OS-specific semantics
    #[allow(non_upper_case_globals)]
    pub const Hios: Self = Self::_HpStubOrHios;
    /// A Thumb label.
    #[allow(non_upper_case_globals)]
    pub const Arm16Bit: Self = Self::_Arm16BitOrHiproc;
    /// Processor-specific semantics
    #[allow(non_upper_case_globals)]
    pub const Hiproc: Self = Self::_Arm16BitOrHiproc;
    /// OS-specific semantics
    #[allow(non_upper_case_globals)]
    pub const Loos: Self = Self::_LoosOrGnuIfunc;
    /// Symbol is an indirect code object
    #[allow(non_upper_case_globals)]
    pub const GnuIfunc: Self = Self::_LoosOrGnuIfunc;
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
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
    #[allow(non_upper_case_globals)]
    pub const VmsSystem: Self = Self::_VmsSystemOrHios;
    /// OS-specific semantics
    #[allow(non_upper_case_globals)]
    pub const Hios: Self = Self::_VmsSystemOrHios;
    #[allow(non_upper_case_globals)]
    pub const HpAlias: Self = Self::_HpAliasOrLoosOrGnuUnique;
    /// OS-specific semantics
    #[allow(non_upper_case_globals)]
    pub const Loos: Self = Self::_HpAliasOrLoosOrGnuUnique;
    /// Symbol is unique in namespace
    #[allow(non_upper_case_globals)]
    pub const GnuUnique: Self = Self::_HpAliasOrLoosOrGnuUnique;
    /// Processor-specific semantics
    #[allow(non_upper_case_globals)]
    pub const Loproc: Self = Self::_LoprocOrMipsSplitCommon;
    #[allow(non_upper_case_globals)]
    pub const MipsSplitCommon: Self = Self::_LoprocOrMipsSplitCommon;
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
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
    #[allow(non_upper_case_globals)]
    pub const X8664Plt : Self = Self::_X8664PltOrIa64PltReserveOrPpcGotOrC6000DsbtBaseOrXtensaGotLocOffOrLoprocOrPpc64GlinkOrAlphaPltro;
    #[allow(non_upper_case_globals)]
    pub const Ia64PltReserve : Self = Self::_X8664PltOrIa64PltReserveOrPpcGotOrC6000DsbtBaseOrXtensaGotLocOffOrLoprocOrPpc64GlinkOrAlphaPltro;
    #[allow(non_upper_case_globals)]
    pub const PpcGot : Self = Self::_X8664PltOrIa64PltReserveOrPpcGotOrC6000DsbtBaseOrXtensaGotLocOffOrLoprocOrPpc64GlinkOrAlphaPltro;
    #[allow(non_upper_case_globals)]
    pub const C6000DsbtBase : Self = Self::_X8664PltOrIa64PltReserveOrPpcGotOrC6000DsbtBaseOrXtensaGotLocOffOrLoprocOrPpc64GlinkOrAlphaPltro;
    #[allow(non_upper_case_globals)]
    pub const XtensaGotLocOff : Self = Self::_X8664PltOrIa64PltReserveOrPpcGotOrC6000DsbtBaseOrXtensaGotLocOffOrLoprocOrPpc64GlinkOrAlphaPltro;
    #[allow(non_upper_case_globals)]
    pub const Loproc : Self = Self::_X8664PltOrIa64PltReserveOrPpcGotOrC6000DsbtBaseOrXtensaGotLocOffOrLoprocOrPpc64GlinkOrAlphaPltro;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Glink : Self = Self::_X8664PltOrIa64PltReserveOrPpcGotOrC6000DsbtBaseOrXtensaGotLocOffOrLoprocOrPpc64GlinkOrAlphaPltro;
    #[allow(non_upper_case_globals)]
    pub const AlphaPltro : Self = Self::_X8664PltOrIa64PltReserveOrPpcGotOrC6000DsbtBaseOrXtensaGotLocOffOrLoprocOrPpc64GlinkOrAlphaPltro;
    #[allow(non_upper_case_globals)]
    pub const X8664Pltsz : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    #[allow(non_upper_case_globals)]
    pub const RiscvVariantCc : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    #[allow(non_upper_case_globals)]
    pub const PpcOpt : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Opd : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    #[allow(non_upper_case_globals)]
    pub const AArch64BtiPlt : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    #[allow(non_upper_case_globals)]
    pub const ScoreBaseAddress : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    #[allow(non_upper_case_globals)]
    pub const MipsRldVersion : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    #[allow(non_upper_case_globals)]
    pub const SparcRegister : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    #[allow(non_upper_case_globals)]
    pub const C6000DsbtSize : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    #[allow(non_upper_case_globals)]
    pub const XtensaGotLocSz : Self = Self::_X8664PltszOrRiscvVariantCcOrPpcOptOrPpc64OpdOrAArch64BtiPltOrScoreBaseAddressOrMipsRldVersionOrSparcRegisterOrC6000DsbtSizeOrXtensaGotLocSz;
    #[allow(non_upper_case_globals)]
    pub const X8664Pltent: Self =
        Self::_X8664PltentOrPpc64OptOrAArch64PacPltOrScoreSymtabnoOrMipsIchecksumOrC6000DsbtIndex;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Opt: Self =
        Self::_X8664PltentOrPpc64OptOrAArch64PacPltOrScoreSymtabnoOrMipsIchecksumOrC6000DsbtIndex;
    #[allow(non_upper_case_globals)]
    pub const AArch64PacPlt: Self =
        Self::_X8664PltentOrPpc64OptOrAArch64PacPltOrScoreSymtabnoOrMipsIchecksumOrC6000DsbtIndex;
    #[allow(non_upper_case_globals)]
    pub const ScoreSymtabno: Self =
        Self::_X8664PltentOrPpc64OptOrAArch64PacPltOrScoreSymtabnoOrMipsIchecksumOrC6000DsbtIndex;
    #[allow(non_upper_case_globals)]
    pub const MipsIchecksum: Self =
        Self::_X8664PltentOrPpc64OptOrAArch64PacPltOrScoreSymtabnoOrMipsIchecksumOrC6000DsbtIndex;
    #[allow(non_upper_case_globals)]
    pub const C6000DsbtIndex: Self =
        Self::_X8664PltentOrPpc64OptOrAArch64PacPltOrScoreSymtabnoOrMipsIchecksumOrC6000DsbtIndex;
    #[allow(non_upper_case_globals)]
    pub const ScoreLocalGotno: Self =
        Self::_ScoreLocalGotnoOrMipsTimeStampOrC6000PreemptmapOrNios2GpOrPpc64Opdsz;
    #[allow(non_upper_case_globals)]
    pub const MipsTimeStamp: Self =
        Self::_ScoreLocalGotnoOrMipsTimeStampOrC6000PreemptmapOrNios2GpOrPpc64Opdsz;
    #[allow(non_upper_case_globals)]
    pub const C6000Preemptmap: Self =
        Self::_ScoreLocalGotnoOrMipsTimeStampOrC6000PreemptmapOrNios2GpOrPpc64Opdsz;
    #[allow(non_upper_case_globals)]
    pub const Nios2Gp: Self =
        Self::_ScoreLocalGotnoOrMipsTimeStampOrC6000PreemptmapOrNios2GpOrPpc64Opdsz;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Opdsz: Self =
        Self::_ScoreLocalGotnoOrMipsTimeStampOrC6000PreemptmapOrNios2GpOrPpc64Opdsz;
    #[allow(non_upper_case_globals)]
    pub const ScoreGotsym: Self = Self::_ScoreGotsymOrMipsIversion;
    #[allow(non_upper_case_globals)]
    pub const MipsIversion: Self = Self::_ScoreGotsymOrMipsIversion;
    #[allow(non_upper_case_globals)]
    pub const ScoreUnrefextno: Self = Self::_ScoreUnrefextnoOrMipsFlagsOrAArch64VariantPcs;
    #[allow(non_upper_case_globals)]
    pub const MipsFlags: Self = Self::_ScoreUnrefextnoOrMipsFlagsOrAArch64VariantPcs;
    #[allow(non_upper_case_globals)]
    pub const AArch64VariantPcs: Self = Self::_ScoreUnrefextnoOrMipsFlagsOrAArch64VariantPcs;
    #[allow(non_upper_case_globals)]
    pub const ScoreHipageno: Self = Self::_ScoreHipagenoOrMipsBaseAddress;
    #[allow(non_upper_case_globals)]
    pub const MipsBaseAddress: Self = Self::_ScoreHipagenoOrMipsBaseAddress;
    #[allow(non_upper_case_globals)]
    pub const Ia64VmsSubtype: Self = Self::_Ia64VmsSubtypeOrHpEpltrelOrC6000GsymOffsetOrLoos;
    #[allow(non_upper_case_globals)]
    pub const HpEpltrel: Self = Self::_Ia64VmsSubtypeOrHpEpltrelOrC6000GsymOffsetOrLoos;
    #[allow(non_upper_case_globals)]
    pub const C6000GsymOffset: Self = Self::_Ia64VmsSubtypeOrHpEpltrelOrC6000GsymOffsetOrLoos;
    #[allow(non_upper_case_globals)]
    pub const Loos: Self = Self::_Ia64VmsSubtypeOrHpEpltrelOrC6000GsymOffsetOrLoos;
    #[allow(non_upper_case_globals)]
    pub const Ia64VmsImgiocnt: Self = Self::_Ia64VmsImgiocntOrHpFilteredOrC6000GstrOffset;
    #[allow(non_upper_case_globals)]
    pub const HpFiltered: Self = Self::_Ia64VmsImgiocntOrHpFilteredOrC6000GstrOffset;
    #[allow(non_upper_case_globals)]
    pub const C6000GstrOffset: Self = Self::_Ia64VmsImgiocntOrHpFilteredOrC6000GstrOffset;
    #[allow(non_upper_case_globals)]
    pub const Ia64VmsLnkflags: Self = Self::_Ia64VmsLnkflagsOrPltSizeOrVxWrsTlsDataAlign;
    #[allow(non_upper_case_globals)]
    pub const PltSize: Self = Self::_Ia64VmsLnkflagsOrPltSizeOrVxWrsTlsDataAlign;
    #[allow(non_upper_case_globals)]
    pub const VxWrsTlsDataAlign: Self = Self::_Ia64VmsLnkflagsOrPltSizeOrVxWrsTlsDataAlign;
    #[allow(non_upper_case_globals)]
    pub const Ia64VmsVirMemBlkSiz: Self = Self::_Ia64VmsVirMemBlkSizOrDltSize;
    #[allow(non_upper_case_globals)]
    pub const DltSize: Self = Self::_Ia64VmsVirMemBlkSizOrDltSize;
    #[allow(non_upper_case_globals)]
    pub const HpLoadMap: Self = Self::_HpLoadMapOrOldLoos;
    #[allow(non_upper_case_globals)]
    pub const OldLoos: Self = Self::_HpLoadMapOrOldLoos;
    #[allow(non_upper_case_globals)]
    pub const HpFilterTls: Self = Self::_HpFilterTlsOrVxWrsTlsDataStart;
    #[allow(non_upper_case_globals)]
    pub const VxWrsTlsDataStart: Self = Self::_HpFilterTlsOrVxWrsTlsDataStart;
    #[allow(non_upper_case_globals)]
    pub const HpCompatFiltered: Self = Self::_HpCompatFilteredOrVxWrsTlsDataSize;
    #[allow(non_upper_case_globals)]
    pub const VxWrsTlsDataSize: Self = Self::_HpCompatFilteredOrVxWrsTlsDataSize;
    #[allow(non_upper_case_globals)]
    pub const HpLazyload: Self = Self::_HpLazyloadOrVxWrsTlsVarsStart;
    #[allow(non_upper_case_globals)]
    pub const VxWrsTlsVarsStart: Self = Self::_HpLazyloadOrVxWrsTlsVarsStart;
    #[allow(non_upper_case_globals)]
    pub const HpBindNowCount: Self = Self::_HpBindNowCountOrVxWrsTlsVarsSize;
    #[allow(non_upper_case_globals)]
    pub const VxWrsTlsVarsSize: Self = Self::_HpBindNowCountOrVxWrsTlsVarsSize;
    /// Map text private
    #[allow(non_upper_case_globals)]
    pub const HpDebugPrivate: Self = Self::_HpDebugPrivateOrNeededOrAlphaNumOrIa64Num;
    #[allow(non_upper_case_globals)]
    pub const Needed: Self = Self::_HpDebugPrivateOrNeededOrAlphaNumOrIa64Num;
    #[allow(non_upper_case_globals)]
    pub const AlphaNum: Self = Self::_HpDebugPrivateOrNeededOrAlphaNumOrIa64Num;
    #[allow(non_upper_case_globals)]
    pub const Ia64Num: Self = Self::_HpDebugPrivateOrNeededOrAlphaNumOrIa64Num;
    /// Callback
    #[allow(non_upper_case_globals)]
    pub const HpDebugCallback: Self = Self::_HpDebugCallbackOrPltrelszOrSparcNumOrPpcNum;
    #[allow(non_upper_case_globals)]
    pub const Pltrelsz: Self = Self::_HpDebugCallbackOrPltrelszOrSparcNumOrPpcNum;
    #[allow(non_upper_case_globals)]
    pub const SparcNum: Self = Self::_HpDebugCallbackOrPltrelszOrSparcNumOrPpcNum;
    #[allow(non_upper_case_globals)]
    pub const PpcNum: Self = Self::_HpDebugCallbackOrPltrelszOrSparcNumOrPpcNum;
    /// BOR callback
    #[allow(non_upper_case_globals)]
    pub const HpDebugCallbackBor: Self = Self::_HpDebugCallbackBorOrHashOrPpc64Num;
    #[allow(non_upper_case_globals)]
    pub const Hash: Self = Self::_HpDebugCallbackBorOrHashOrPpc64Num;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Num: Self = Self::_HpDebugCallbackBorOrHashOrPpc64Num;
    /// No env var
    #[allow(non_upper_case_globals)]
    pub const HpNoEnvvar: Self = Self::_HpNoEnvvarOrRelasz;
    #[allow(non_upper_case_globals)]
    pub const Relasz: Self = Self::_HpNoEnvvarOrRelasz;
    /// Bind now
    #[allow(non_upper_case_globals)]
    pub const HpBindNow: Self = Self::_HpBindNowOrSymbolicOrVersiontagnum;
    #[allow(non_upper_case_globals)]
    pub const Symbolic: Self = Self::_HpBindNowOrSymbolicOrVersiontagnum;
    #[allow(non_upper_case_globals)]
    pub const Versiontagnum: Self = Self::_HpBindNowOrSymbolicOrVersiontagnum;
    /// Bind non-fatal
    #[allow(non_upper_case_globals)]
    pub const HpBindNonfatal: Self = Self::_HpBindNonfatalOrEncodingOrPreinitArray;
    #[allow(non_upper_case_globals)]
    pub const Encoding: Self = Self::_HpBindNonfatalOrEncodingOrPreinitArray;
    #[allow(non_upper_case_globals)]
    pub const PreinitArray: Self = Self::_HpBindNonfatalOrEncodingOrPreinitArray;
    #[allow(non_upper_case_globals)]
    pub const Pltgot: Self = Self::_PltgotOrExtranum;
    #[allow(non_upper_case_globals)]
    pub const Extranum: Self = Self::_PltgotOrExtranum;
    #[allow(non_upper_case_globals)]
    pub const Symtab: Self = Self::_SymtabOrAArch64Num;
    #[allow(non_upper_case_globals)]
    pub const AArch64Num: Self = Self::_SymtabOrAArch64Num;
    #[allow(non_upper_case_globals)]
    pub const Syment: Self = Self::_SymentOrAddrnum;
    #[allow(non_upper_case_globals)]
    pub const Addrnum: Self = Self::_SymentOrAddrnum;
    #[allow(non_upper_case_globals)]
    pub const Init: Self = Self::_InitOrValnum;
    #[allow(non_upper_case_globals)]
    pub const Valnum: Self = Self::_InitOrValnum;
    #[allow(non_upper_case_globals)]
    pub const OldHios: Self = Self::_OldHiosOrVerneednum;
    #[allow(non_upper_case_globals)]
    pub const Verneednum: Self = Self::_OldHiosOrVerneednum;
    #[allow(non_upper_case_globals)]
    pub const Hiproc: Self = Self::_HiprocOrFilter;
    #[allow(non_upper_case_globals)]
    pub const Filter: Self = Self::_HiprocOrFilter;
    #[allow(non_upper_case_globals)]
    pub const Feature: Self = Self::_FeatureOrFeature1;
    /// Feature selection (DTF_*).
    #[allow(non_upper_case_globals)]
    pub const Feature1: Self = Self::_FeatureOrFeature1;
    #[allow(non_upper_case_globals)]
    pub const Syminent: Self = Self::_SyminentOrValrnghi;
    #[allow(non_upper_case_globals)]
    pub const Valrnghi: Self = Self::_SyminentOrValrnghi;
    #[allow(non_upper_case_globals)]
    pub const Syminfo: Self = Self::_SyminfoOrAddrrnghi;
    #[allow(non_upper_case_globals)]
    pub const Addrrnghi: Self = Self::_SyminfoOrAddrrnghi;
    /// Most used by any processor
    #[allow(non_upper_case_globals)]
    pub const Procnum: Self = Self::_ProcnumOrMipsNum;
    #[allow(non_upper_case_globals)]
    pub const MipsNum: Self = Self::_ProcnumOrMipsNum;
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
    #[allow(non_upper_case_globals)]
    pub const VmsMhd : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    #[allow(non_upper_case_globals)]
    pub const HpCompiler : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    /// Contains copy of prstatus struct
    #[allow(non_upper_case_globals)]
    pub const Prstatus : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    /// Has a struct procinfo
    #[allow(non_upper_case_globals)]
    pub const NetbsdcoreProcinfo : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    #[allow(non_upper_case_globals)]
    pub const Spu : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    /// Contains a version string.
    #[allow(non_upper_case_globals)]
    pub const Version : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    #[allow(non_upper_case_globals)]
    pub const GnuAbiTag : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    #[allow(non_upper_case_globals)]
    pub const NetbsdIdent : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    #[allow(non_upper_case_globals)]
    pub const OpenbsdIdent : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    #[allow(non_upper_case_globals)]
    pub const FreebsdAbiTag : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    /// Force enable Mprotect.
    #[allow(non_upper_case_globals)]
    pub const NetbsdPaxMprotect : Self = Self::_VmsMhdOrHpCompilerOrPrstatusOrNetbsdcoreProcinfoOrSpuOrVersionOrGnuAbiTagOrNetbsdIdentOrOpenbsdIdentOrFreebsdAbiTagOrNetbsdPaxMprotect;
    /// Language processor name.
    #[allow(non_upper_case_globals)]
    pub const VmsLnm : Self = Self::_VmsLnmOrHpCopyrightOrFpregsetOrNetbsdcoreAuxvOrArchOrGnuHwcapOrPrfpregOrNetbsdPaxNomprotect;
    #[allow(non_upper_case_globals)]
    pub const HpCopyright : Self = Self::_VmsLnmOrHpCopyrightOrFpregsetOrNetbsdcoreAuxvOrArchOrGnuHwcapOrPrfpregOrNetbsdPaxNomprotect;
    /// Contains copy of fpregset struct
    #[allow(non_upper_case_globals)]
    pub const Fpregset : Self = Self::_VmsLnmOrHpCopyrightOrFpregsetOrNetbsdcoreAuxvOrArchOrGnuHwcapOrPrfpregOrNetbsdPaxNomprotect;
    /// Has auxv data
    #[allow(non_upper_case_globals)]
    pub const NetbsdcoreAuxv : Self = Self::_VmsLnmOrHpCopyrightOrFpregsetOrNetbsdcoreAuxvOrArchOrGnuHwcapOrPrfpregOrNetbsdPaxNomprotect;
    /// Contains an architecture string.
    #[allow(non_upper_case_globals)]
    pub const Arch : Self = Self::_VmsLnmOrHpCopyrightOrFpregsetOrNetbsdcoreAuxvOrArchOrGnuHwcapOrPrfpregOrNetbsdPaxNomprotect;
    /// Used by ld.so and kernel vDSO.
    #[allow(non_upper_case_globals)]
    pub const GnuHwcap : Self = Self::_VmsLnmOrHpCopyrightOrFpregsetOrNetbsdcoreAuxvOrArchOrGnuHwcapOrPrfpregOrNetbsdPaxNomprotect;
    #[allow(non_upper_case_globals)]
    pub const Prfpreg : Self = Self::_VmsLnmOrHpCopyrightOrFpregsetOrNetbsdcoreAuxvOrArchOrGnuHwcapOrPrfpregOrNetbsdPaxNomprotect;
    /// Force disable Mprotect.
    #[allow(non_upper_case_globals)]
    pub const NetbsdPaxNomprotect : Self = Self::_VmsLnmOrHpCopyrightOrFpregsetOrNetbsdcoreAuxvOrArchOrGnuHwcapOrPrfpregOrNetbsdPaxNomprotect;
    /// Source files.
    #[allow(non_upper_case_globals)]
    pub const VmsSrc: Self = Self::_VmsSrcOrHpVersionOrPrpsinfoOrStapsdtOrGnuBuildIdOrNetbsdPax;
    #[allow(non_upper_case_globals)]
    pub const HpVersion: Self = Self::_VmsSrcOrHpVersionOrPrpsinfoOrStapsdtOrGnuBuildIdOrNetbsdPax;
    /// Contains copy of prpsinfo struct
    #[allow(non_upper_case_globals)]
    pub const Prpsinfo: Self = Self::_VmsSrcOrHpVersionOrPrpsinfoOrStapsdtOrGnuBuildIdOrNetbsdPax;
    #[allow(non_upper_case_globals)]
    pub const Stapsdt: Self = Self::_VmsSrcOrHpVersionOrPrpsinfoOrStapsdtOrGnuBuildIdOrNetbsdPax;
    /// Generated by ld --build-id.
    #[allow(non_upper_case_globals)]
    pub const GnuBuildId: Self = Self::_VmsSrcOrHpVersionOrPrpsinfoOrStapsdtOrGnuBuildIdOrNetbsdPax;
    #[allow(non_upper_case_globals)]
    pub const NetbsdPax: Self = Self::_VmsSrcOrHpVersionOrPrpsinfoOrStapsdtOrGnuBuildIdOrNetbsdPax;
    /// Title text.
    #[allow(non_upper_case_globals)]
    pub const VmsTitle : Self = Self::_VmsTitleOrHpSrcfileInfoOrTaskstructOrGoBuildidOrGnuGoldVersionOrPrxregOrNetbsdPaxGuard;
    #[allow(non_upper_case_globals)]
    pub const HpSrcfileInfo : Self = Self::_VmsTitleOrHpSrcfileInfoOrTaskstructOrGoBuildidOrGnuGoldVersionOrPrxregOrNetbsdPaxGuard;
    /// Contains copy of task struct
    #[allow(non_upper_case_globals)]
    pub const Taskstruct : Self = Self::_VmsTitleOrHpSrcfileInfoOrTaskstructOrGoBuildidOrGnuGoldVersionOrPrxregOrNetbsdPaxGuard;
    /// Contains GO buildid data.
    #[allow(non_upper_case_globals)]
    pub const GoBuildid : Self = Self::_VmsTitleOrHpSrcfileInfoOrTaskstructOrGoBuildidOrGnuGoldVersionOrPrxregOrNetbsdPaxGuard;
    /// Generated by gold.
    #[allow(non_upper_case_globals)]
    pub const GnuGoldVersion : Self = Self::_VmsTitleOrHpSrcfileInfoOrTaskstructOrGoBuildidOrGnuGoldVersionOrPrxregOrNetbsdPaxGuard;
    /// Contains copy of prxregset struct
    #[allow(non_upper_case_globals)]
    pub const Prxreg : Self = Self::_VmsTitleOrHpSrcfileInfoOrTaskstructOrGoBuildidOrGnuGoldVersionOrPrxregOrNetbsdPaxGuard;
    /// Force enable Segvguard.
    #[allow(non_upper_case_globals)]
    pub const NetbsdPaxGuard : Self = Self::_VmsTitleOrHpSrcfileInfoOrTaskstructOrGoBuildidOrGnuGoldVersionOrPrxregOrNetbsdPaxGuard;
    /// Entity ident consistency check.
    #[allow(non_upper_case_globals)]
    pub const VmsEidc: Self = Self::_VmsEidcOrHpLinkerOrGnuPropertyType0OrNetbsdMarchOrPlatform;
    #[allow(non_upper_case_globals)]
    pub const HpLinker: Self = Self::_VmsEidcOrHpLinkerOrGnuPropertyType0OrNetbsdMarchOrPlatform;
    /// Generated by gcc.
    #[allow(non_upper_case_globals)]
    pub const GnuPropertyType0: Self =
        Self::_VmsEidcOrHpLinkerOrGnuPropertyType0OrNetbsdMarchOrPlatform;
    #[allow(non_upper_case_globals)]
    pub const NetbsdMarch: Self = Self::_VmsEidcOrHpLinkerOrGnuPropertyType0OrNetbsdMarchOrPlatform;
    /// String from sysinfo(SI_PLATFORM)
    #[allow(non_upper_case_globals)]
    pub const Platform: Self = Self::_VmsEidcOrHpLinkerOrGnuPropertyType0OrNetbsdMarchOrPlatform;
    /// Whole program floating-point mode.
    #[allow(non_upper_case_globals)]
    pub const VmsFpmode: Self = Self::_VmsFpmodeOrHpInstrumentedOrAuxv;
    #[allow(non_upper_case_globals)]
    pub const HpInstrumented: Self = Self::_VmsFpmodeOrHpInstrumentedOrAuxv;
    /// Contains copy of Elfxx_auxv_t
    #[allow(non_upper_case_globals)]
    pub const Auxv: Self = Self::_VmsFpmodeOrHpInstrumentedOrAuxv;
    #[allow(non_upper_case_globals)]
    pub const HpUxOptions: Self = Self::_HpUxOptionsOrFreebsdThrmiscOrGwindows;
    /// Thread miscellaneous info.
    #[allow(non_upper_case_globals)]
    pub const FreebsdThrmisc: Self = Self::_HpUxOptionsOrFreebsdThrmiscOrGwindows;
    /// Contains copy of gwindows struct
    #[allow(non_upper_case_globals)]
    pub const Gwindows: Self = Self::_HpUxOptionsOrFreebsdThrmiscOrGwindows;
    /// PowerPC Altivec/VMX registers
    #[allow(non_upper_case_globals)]
    pub const PpcVmx: Self = Self::_PpcVmxOrGnuBuildAttributeOpen;
    #[allow(non_upper_case_globals)]
    pub const GnuBuildAttributeOpen: Self = Self::_PpcVmxOrGnuBuildAttributeOpen;
    /// x86 TLS information
    #[allow(non_upper_case_globals)]
    pub const I386Tls: Self = Self::_I386TlsOrFreebsdX86Segbases;
    /// x86 segment base registers
    #[allow(non_upper_case_globals)]
    pub const FreebsdX86Segbases: Self = Self::_I386TlsOrFreebsdX86Segbases;
    /// Has a struct pstatus
    #[allow(non_upper_case_globals)]
    pub const Pstatus: Self = Self::_PstatusOrFreebsdProcstatVmmapOrOpenbsdProcinfo;
    /// Procstat vmmap data.
    #[allow(non_upper_case_globals)]
    pub const FreebsdProcstatVmmap: Self = Self::_PstatusOrFreebsdProcstatVmmapOrOpenbsdProcinfo;
    #[allow(non_upper_case_globals)]
    pub const OpenbsdProcinfo: Self = Self::_PstatusOrFreebsdProcstatVmmapOrOpenbsdProcinfo;
    /// Has a struct fpregset
    #[allow(non_upper_case_globals)]
    pub const Fpregs: Self = Self::_FpregsOrFreebsdProcstatUmask;
    /// Procstat umask data.
    #[allow(non_upper_case_globals)]
    pub const FreebsdProcstatUmask: Self = Self::_FpregsOrFreebsdProcstatUmask;
    /// Has a struct psinfo
    #[allow(non_upper_case_globals)]
    pub const Psinfo: Self = Self::_PsinfoOrFreebsdProcstatRlimit;
    /// Procstat rlimit data.
    #[allow(non_upper_case_globals)]
    pub const FreebsdProcstatRlimit: Self = Self::_PsinfoOrFreebsdProcstatRlimit;
    /// Has a struct lwpstatus_t
    #[allow(non_upper_case_globals)]
    pub const Lwpstatus: Self = Self::_LwpstatusOrFreebsdProcstatAuxvOrNetbsdPaxAslr;
    /// Procstat auxv data.
    #[allow(non_upper_case_globals)]
    pub const FreebsdProcstatAuxv: Self = Self::_LwpstatusOrFreebsdProcstatAuxvOrNetbsdPaxAslr;
    /// Force enable ASLR.
    #[allow(non_upper_case_globals)]
    pub const NetbsdPaxAslr: Self = Self::_LwpstatusOrFreebsdProcstatAuxvOrNetbsdPaxAslr;
    /// Has a struct lwpsinfo_t
    #[allow(non_upper_case_globals)]
    pub const Lwpsinfo: Self = Self::_LwpsinfoOrFreebsdPtlwpinfo;
    /// Thread ptrace miscellaneous info.
    #[allow(non_upper_case_globals)]
    pub const FreebsdPtlwpinfo: Self = Self::_LwpsinfoOrFreebsdPtlwpinfo;
    /// Procstat proc data.
    #[allow(non_upper_case_globals)]
    pub const FreebsdProcstatProc: Self = Self::_FreebsdProcstatProcOrAsrsOrNetbsdPaxNoguard;
    /// Contains copy of asrset struct
    #[allow(non_upper_case_globals)]
    pub const Asrs: Self = Self::_FreebsdProcstatProcOrAsrsOrNetbsdPaxNoguard;
    /// Force disable Segvguard.
    #[allow(non_upper_case_globals)]
    pub const NetbsdPaxNoguard: Self = Self::_FreebsdProcstatProcOrAsrsOrNetbsdPaxNoguard;
    /// Procstat groups data.
    #[allow(non_upper_case_globals)]
    pub const FreebsdProcstatGroups: Self = Self::_FreebsdProcstatGroupsOrOpenbsdAuxv;
    #[allow(non_upper_case_globals)]
    pub const OpenbsdAuxv: Self = Self::_FreebsdProcstatGroupsOrOpenbsdAuxv;
    /// Procstat osreldate data.
    #[allow(non_upper_case_globals)]
    pub const FreebsdProcstatOsrel: Self = Self::_FreebsdProcstatOsrelOrPrcred;
    /// Contains copy of prcred struct
    #[allow(non_upper_case_globals)]
    pub const Prcred: Self = Self::_FreebsdProcstatOsrelOrPrcred;
    /// Procstat ps_strings data.
    #[allow(non_upper_case_globals)]
    pub const FreebsdProcstatPsstrings: Self = Self::_FreebsdProcstatPsstringsOrUtsname;
    /// Contains copy of utsname struct
    #[allow(non_upper_case_globals)]
    pub const Utsname: Self = Self::_FreebsdProcstatPsstringsOrUtsname;
    /// start of machdep note types
    #[allow(non_upper_case_globals)]
    pub const NetbsdcoreFirstmach: Self =
        Self::_NetbsdcoreFirstmachOrAmdgpuMetadataOrNetbsdPaxNoaslr;
    #[allow(non_upper_case_globals)]
    pub const AmdgpuMetadata: Self = Self::_NetbsdcoreFirstmachOrAmdgpuMetadataOrNetbsdPaxNoaslr;
    /// Force disable ASLR.
    #[allow(non_upper_case_globals)]
    pub const NetbsdPaxNoaslr: Self = Self::_NetbsdcoreFirstmachOrAmdgpuMetadataOrNetbsdPaxNoaslr;
    #[allow(non_upper_case_globals)]
    pub const OpenbsdRegs: Self = Self::_OpenbsdRegsOrPrfpxreg;
    /// Contains copy of fprxregset struct
    #[allow(non_upper_case_globals)]
    pub const Prfpxreg: Self = Self::_OpenbsdRegsOrPrfpxreg;
    #[allow(non_upper_case_globals)]
    pub const GnuBuildAttributeFunc: Self = Self::_GnuBuildAttributeFuncOrPpcSpe;
    /// PowerPC SPE/EVR registers
    #[allow(non_upper_case_globals)]
    pub const PpcSpe: Self = Self::_GnuBuildAttributeFuncOrPpcSpe;
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
    /// FooNone Or PowerpcNone Or MipsNone Or TilegxNone Or X8664None Or SparcNone Or I386None Or No reloc Or No reloc Or No reloc Or PpcNone Or No relocation Or No reloc Or no reloc Or ShNone Or No reloc Or CrisNone Or No reloc Or No reloc Or No reloc Or No reloc Or No reloc Or RiscvNone Or No reloc Or MetagHiaddr16 Or Nds32None Or Or1KNone Or Ppc64None Or none Or ArcNone
    _FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone =
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
    /// PariscTlsIe21L Or LT-TP-rel. address, left 21 bits Or ShCopy
    _PariscTlsIe21LOrPariscLtoffTp21LOrShCopy = 0xa2,
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
    /// S390JmpSlot Or PowerpcRel14 Or TilegxHw2 Or MipsCall16 Or X866432S Or Sparc13 Or ArmThmPc8 Or 16 bit GOT offset Or I38632Plt Or Right 17 bits of rel. address Or PC relative 64 bit Or PC relative 16 bit Or off between got and sym (S) Or CrisJumpSlot Or 16-bit PCrel offset to GOT Or M32RGnuVtinherit Or GNU C++ vtable hierarchy Or High 16 bit, adjusted Or Create GOT entry Or RiscvTlsTprel64 Or MetagReg16Op3 Or Or1K8Pcrel Or PC relative 16 bit Or ArcN32
    _S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32 =
        0xb,
    /// S390Relative Or PowerpcRel14Brtaken Or MipsGprel32 Or TilegxHw3 Or X866416 Or SparcLo10 Or ArmBrelAdj Or 8 bit GOT offset Or 17 bits of rel. address Or PpcRel14Brtaken Or ArmAmpVcall9 Or PLT entry (S) Or CrisRelative Or 32-bit offset from GOT Or M32RGnuVtentry Or GNU C++ vtable member usage Or 32 bit symbol value + addend Or Create PLT entry Or MetagReg32Op4 Or Or1KGotpcHi16 Or Ppc64Rel14Brtaken Or ArcSda
    _S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda =
        0xc,
    /// S390Gotoff32 Or PowerpcRel14Brntaken Or MipsUnused1 Or TilegxHw0Last Or X8664Pc16 Or SparcGot10 Or ArmTlsDesc Or 32 bit PC relative PLT address Or PpcRel14Brntaken Or Obsolete static relocation Or offset to GOT (S + A - GOT) Or Cris16Got Or 24-bit offset from GOT Or PC-relative GOT offset Or 16 bit symbol value + addend Or Adjust by program base Or MetagHiog Or Or1KGotpcLo16 Or Ppc64Rel14Brntaken Or ArcSectoff
    _S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff =
        0xd,
    /// S390Gotpc Or PowerpcGot16 Or MipsUnused2 Or TilegxHw1Last Or X86648 Or SparcGot13 Or ArmThmSwi8 Or 16 bit PC relative PLT address Or Offset in static TLS block Or Right 14 bits of rel. address Or PpcGot16 Or PC offset to GOT (GOT + A - P) Or Cris32Got Or 16-bit offset from GOT Or GOT entry offset Or 8 bit symbol value + addend Or X1 pipe branch offset Or MetagLoog Or Or1KGot16 Or Ppc64Got16 Or ArcS21HPcrel
    _S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel =
        0xe,
    /// S390Got16 Or PowerpcGot16Lo Or MipsUnused3 Or TilegxHw2Last Or X8664Pc8 Or SparcGot22 Or ArmXpc25 Or I386TlsIe Or 8 bit PC relative PLT address Or PpcGot16Lo Or 32 bit GOT entry (G) Or Cris16Gotplt Or 32-bit PCrel to PLT entry Or PLT offset (PC-relative) Or 16 bit GP pointer offset Or X1 pipe jump offset Or MetagRel8 Or Or1KPlt26 Or Ppc64Got16Lo Or ArcS21WPcrel
    _S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel =
        0xf,
    /// S390Pc16 Or PowerpcGot16Hi Or MipsShift5 Or TilegxCopy Or SparcPc10 Or ArmThmXpc22 Or I386TlsGotie Or 32 bit PLT offset Or PpcGot16Hi Or 32 bit PLT entry (G) Or Cris32Gotplt Or ID of module containing symbol Or 16-bit PCrel to PLT entry Or Adjust by program base Or GNU C++ vtable hierarchy Or X1 pipe jump offset to PLT Or RiscvBranch Or MetagRel16 Or Or1KGotoffHi16 Or Ppc64Got16Hi Or ArcS25HPcrel
    _S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel =
        0x10,
    /// S390Pc16Dbl Or PowerpcGot16Ha Or MipsShift6 Or TilegxGlobDat Or X8664Dtpoff64 Or SparcPc22 Or ArmTlsDtpmod32 Or I386TlsLe Or 16 bit PLT offset Or GP relative 32 bit, high 16 bits Or PpcGot16Ha Or GOT entry in GLOB_DAT (GOT + G) Or Cris32Gotrel Or 32-bit offset to GOT entry Or Create PLT entry Or GNU C++ vtable member usage Or X0 pipe 8-bit Or RiscvJal Or Or1KGotoffLo16 Or Ppc64Got16Ha Or ArcS25WPcrel
    _S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel =
        0x11,
    /// S390Plt16Dbl Or PpcPltrel24 Or Mips64 Or TilegxJmpSlot Or X8664Tpoff64 Or SparcWplt30 Or ArmTlsDtpoff32 Or I386TlsGd Or 8 bit PLT offset Or Left 21 bits of rel. address Or GP relative 32 bit, low 16 bits Or PLT entry in GLOB_DAT (GOT + G) Or Cris32PltGotrel Or 24-bit offset to GOT entry Or Create GOT entry Or Unconditional branch Or Y0 pipe 8-bit Or RiscvCall Or Or1KCopy Or ArcSda32
    _S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32 =
        0x12,
    /// S390Plt32Dbl Or PowerpcGlobDat Or MipsGotPage Or TilegxBroffX1 Or SparcGlobDat Or ArmCopy Or Create GOT entry Or I38616 Or PpcGlobDat Or disp ((S + A - P) >> 1) & 0xffff Or CrisNum Or X8664Tlsld Or Copy symbol at runtime Or 32 bit offset to GOT Or Indirect call through register Or Y1 pipe 8-bit Or RiscvGotHi20 Or Nds3232Rela Or Or1KJmpSlot Or Ppc64GlobDat Or ArcSdaLdst1
    _S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1 =
        0x14,
    /// S390Gotpcdbl Or PowerpcJmpSlot Or MipsGotOfst Or TilegxJumpoffX1 Or SparcJmpSlot Or ArmGlobDat Or I386Pc16 Or Create PLT entry Or PpcJmpSlot Or disp ((S + A - P) >> 2) & 0xffff Or Offset in TLS block Or Create GOT entry Or Runtime copy Or Nios2Align Or X1 pipe mtspr Or RiscvTlsGotHi20 Or Or1KRelative Or Ppc64JmpSlot Or ArcSdaLdst2
    _S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2 =
        0x15,
    /// S39064 Or PowerpcRelative Or MipsGotHi16 Or TilegxJumpoffX1Plt Or X8664Gottpoff Or SparcRelative Or ArmJumpSlot Or I3868 Or Adjust by program base Or Right 14 bits of rel. address Or PpcRelative Or disp ((S + A - P) >> 1) & 0x3ff Or Create PLT entry Or TLS Reloc Or 16 bit GOT entry Or X1 pipe mfspr Or RiscvTlsGdHi20 Or Or1KTlsGdHi16 Or Ppc64Relative Or ArcSda16Ld
    _S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld =
        0x16,
    /// S390Pc64 Or PpcLocal24Pc Or MipsGotLo16 Or TilegxImm8X0 Or SparcUa32 Or ArmRelative Or I386Pc8 Or disp ((S + A - P) >> 2) & 0x3ff Or Offset in initial TLS block Or Adjust by program base Or TLS General Dynamic Or 16 bit GOT entry for function Or X0 pipe 16-bit Or RiscvPcrelHi20 Or Or1KTlsGdLo16 Or ArcSda16Ld1
    _S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrX8664Tpoff32OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1 =
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
    /// S390TlsGdcall Or Ppc64Addr64 Or TilegxImm16X0Hw1 Or X8664Relative64 Or SparcPcHm10 Or ArmTarget1 Or M68KTlsLe16 Or 32-bit symbol size Or Module number 32 bit Or LT-relative, right 14 bits Or AlphaTprel64 Or high & low 16 bit ADDRPLT Or PC relative 26 bit shifted Or Create PLT entry Or X1 pipe PC relative ha() 16 bit Or RiscvSub16 Or MetagHi16Plt Or symbol + addend, data8 MSB Or AcSectoffS9
    _S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9 =
        0x26,
    /// S390TlsLdcall Or Ppc64Addr16Higher Or MipsTlsDtprel32 Or TilegxImm16X1Hw1 Or X8664Pc32Bnd Or SparcPcLm22 Or I386TlsGotdesc Or M68KTlsLe8 Or AlphaTprelhi Or Program base relative Or (GOT+G*4) & 0xffff Or High 16 bit with unsigned low Or Adjust by program base Or X0 pipe 16-bit GOT offset Or RiscvSub32 Or MetagLo16Plt Or Nds32Copy Or symbol + addend, data8 LSB Or AcSectoffS91
    _S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91 =
        0x27,
    /// S390TlsGd32 Or Ppc64Addr16Highera Or MipsTlsDtpmod64 Or TilegxImm16X0Hw2 Or X8664Plt32Bnd Or SparcWdisp16 Or ArmV4Bx Or I386TlsDescCall Or 32 bit module number Or AlphaTprello Or disp ((S+A-P) >>1) & x3ffffff Or High 16 bit with signed low Or 16 bit offset to GOT pointer Or X1 pipe 16-bit GOT offset Or RiscvSub64 Or MetagRelbranchPlt Or Nds32GlobDat Or AcSectoffS92
    _S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92 =
        0x28,
    /// S390TlsGd64 Or Ppc64Addr16Highest Or MipsTlsDtprel64 Or TilegxImm16X1Hw2 Or X8664Gotpcrelx Or SparcWdisp19 Or ArmTarget2 Or I386TlsDesc Or 32 bit module-relative offset Or 32 bits section rel. address Or AlphaTprel16 Or (S+A-BTEXT) & 0xffff Or Low 16 bit Or Direct call in .noat section Or X0 pipe low 16-bit GOT offset Or RiscvGnuVtinherit Or MetagGotoff Or Nds32JmpSlot Or ArcSectoffMe1
    _S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1 =
        0x29,
    /// S390TlsGotie12 Or Ppc64Addr16Highesta Or MipsTlsGd Or TilegxImm16X0Hw3 Or SparcGlobJmp Or ArmPrel31 Or 32 bit TP-relative offset Or Adjust indirectly by program base Or (S+A-BTEXT) & 0xffff Or X8664RexGotpcrelx Or 16 bit offset in SDA Or %lo() of GOT entry Or X1 pipe low 16-bit GOT offset Or RiscvGnuVtentry Or MetagPlt Or Nds32Relative Or @gprel(sym + add), add imm22 Or ArcSectoffMe2
    _S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2 =
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
    /// S390TlsIe32 Or Ppc64Toc16 Or MipsTlsTprel32 Or TilegxImm16X1Hw1Last Or SparcPlt64 Or ArmThmMovwAbsNc Or @gprel(sym + add), data8 LSB Or X0 pipe mm "start" Or RiscvGprelI Or MetagTlsGd
    _S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrIa64Gprel64LsbOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGd =
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
    /// S390Got20 Or Ppc64Got16Ds Or TilegxImm16X0Hw0LastPcrel Or SparcTlsGdAdd Or ArmAluPcG0 Or LT-rel. fct ptr, left 21 bits Or CkcoreTlsTpoff32 Or Low 16 bit GOT entry Or RiscvIrelative Or MetagTlsDtpoff Or @pltoff(sym + add), add imm22 Or ArcGotpc
    _S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc =
        0x3a,
    /// S390Gotplt20 Or Ppc64Got16LoDs Or TilegxImm16X1Hw0LastPcrel Or SparcTlsGdCall Or ArmAluPcG1Nc Or @pltoff(sym + add), mov imm64 Or M32RGotpcHiUlo Or RiscvNum Or MetagTlsLe Or ArcGot32
    _S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrIa64Pltoff64IOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrArcGot32 =
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
    /// S390Plt12Dbl Or Ppc64Toc16Ds Or MipsPc19S2 Or TilegxImm16X1Hw2LastPcrel Or SparcTlsLdmCall Or ArmLdrPcG2 Or @pltoff(sym + add), data8 LSB Or M32RGotoffHiSlo Or Y0 pipe "addi" for TLS GD
    _S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrIa64Pltoff64LsbOrM32RGotoffHiSloOrTileproImm8Y0TlsGdAdd =
        0x3f,
    /// S390Pc24Dbl Or Ppc64Toc16LoDs Or MipsPchi16 Or TilegxImm16X0Hw0Got Or SparcTlsLdoHix22 Or ArmLdrsPcG0 Or 64 bits function address Or Low 16 bit offset to GOT Or Y1 pipe "addi" for TLS GD
    _S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd =
        0x40,
    /// S390Plt24Dbl Or Ppc64Pltgot16Ds Or MipsPclo16 Or TilegxImm16X1Hw0Got Or SparcTlsLdoLox10 Or ArmLdrsPcG1 Or 32 bits function address Or "lw_tls" for TLS IE
    _S390Plt24DblOrPpc64Pltgot16DsOrMipsPclo16OrTilegxImm16X1Hw0GotOrSparcTlsLdoLox10OrArmLdrsPcG1OrPariscPlabel32OrTileproTlsIeLoad =
        0x41,
    /// S390GnuVtentry Or PowerpcRel16Hi Or SparcGnuVtentry Or X8664GnuVtentry Or I386GnuVtentry Or half16   (sym+add-.)@h Or half16   (sym+add-.)@h Or ArmThmRpc22
    _S390GnuVtentryOrPowerpcRel16HiOrSparcGnuVtentryOrX8664GnuVtentryOrI386GnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22 =
        0xfb,
    /// PowerpcCopy Or MipsGotDisp Or TilegxRelative Or X8664Tlsgd Or SparcCopy Or ArmTlsTpoff32 Or I386TlsLdm Or Copy symbol at runtime Or GP relative 16 bit Or PpcCopy Or ((S + A - P) >> 1) & 0x3ffffff Or PC relative 32 bit shifted by 1 Or Cris32PltPcrel Or 16-bit offset to GOT entry Or 64 bit offset to GOT Or Conditional branch Or X1 pipe 8-bit Or RiscvCallPlt Or Or1KGlobDat Or Ppc64Copy Or ArcSdaLdst
    _PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst =
        0x13,
    /// PowerpcPlt32 Or MipsDelete Or TilegxDestImm8X1 Or X8664Got64 Or SparcPcplt32 Or ArmPlt32 Or I386TlsGdPop Or 8 bit GOT offset for GD Or Adjust by program base Or PpcPlt32 Or (GOT + A - P) & 0xffff Or ShUses Or 16 bit offset to GOT Or Mn10300TlsGotie Or TLS Offset Within TLS Block Or %hiadj of PC relative offset Or X0 pipe high 16-bit Or RiscvLo12I Or Or1KTlsLdoLo16 Or Ppc64Plt32 Or Arc32Me
    _PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me =
        0x1b,
    /// PowerpcAddr30 Or MipsJalr Or TilegxImm16X1Hw0 Or X8664Irelative Or SparcPcHh22 Or ArmAluSbrel2720Ck Or I386TlsTpoff32 Or M68KTlsLe32 Or AlphaGottprel Or word30 (S + A - P) >> 2 Or Deprecated, prog. base relative Or (GOT + G * 4) & 0xffff Or Tag for load insn in TLS code Or PC relative 18 bit shifted Or Create GOT entry Or X0 pipe PC relative ha() 16 bit Or RiscvSub8 Or MetagLo16Gotpc Or symbol + addend, data4 LSB Or AcSectoffU82
    _PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82 =
        0x25,
    /// Ppc64Addr16LoDs Or TilegxImm16X1Hw3Pcrel Or SparcTlsGdLo10 Or ArmAluPcG0Nc Or 32 bits LT-rel. function pointer Or CkcoreTlsDtpoff32 Or Direct 20 bit Or M32RGot16HiSlo Or Riscv32Pcrel Or MetagTlsDtpmod Or ArcGotoff
    _Ppc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrS39020OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff =
        0x39,
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
    /// PowerpcGotTlsgd16Lo Or TilegxImm16X0Hw0TlsLe Or ArmLdrsSbG2 Or SparcGotdataHix22 Or 64 bits of eff. address Or half16	(sym+add)@got@tlsgd@l Or half16	(sym+add)@got@tlsgd@l Or X0 pipe ha() 16-bit TLS IE offset
    _PowerpcGotTlsgd16LoOrTilegxImm16X0Hw0TlsLeOrArmLdrsSbG2OrSparcGotdataHix22OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrTileproImm16X0TlsIeHa =
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
    /// Ppc64Tprel16Highesta Or TilegxImm16X0Hw0LastTlsIe Or ArmGnuVtentry Or LT-rel. address, right 14 bits Or @secrel(sym + add), data4 MSB
    _Ppc64Tprel16HighestaOrTilegxImm16X0Hw0LastTlsIeOrArmGnuVtentryOrPariscLtoff14DrOrIa64Secrel32Msb =
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
    /// Ppc64GotPcrel34 Or Addend and symbol difference
    _Ppc64GotPcrel34OrIa64Sub = 0x85,
    /// Ppc64PltPcrel34 Or MicromipsHi16 Or LTOFF22, relaxable
    _Ppc64PltPcrel34OrMicromipsHi16OrIa64Ltoff22X = 0x86,
    /// Ppc64PltPcrel34Notoc Or MicromipsLo16 Or Use of LTOFF22X
    _Ppc64PltPcrel34NotocOrMicromipsLo16OrIa64Ldxmov = 0x87,
    _Ppc64Addr16Higher34OrMicromipsGprel16 = 0x88,
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
    _PowerpcIrelativeOrSparcJmpIrelOrPpcIrelativeOrPpc64Irelative = 0xf8,
    /// PowerpcRel16 Or MipsEh Or SparcIrelative Or half16   (sym+add-.) Or half16   (sym+add-.) Or ArmRxpc25
    _PowerpcRel16OrMipsEhOrSparcIrelativeOrPpcRel16OrPpc64Rel16OrArmRxpc25 = 0xf9,
    /// PowerpcRel16Lo Or SparcGnuVtinherit Or half16   (sym+add-.)@l Or half16   (sym+add-.)@l Or ArmRsbrel32
    _PowerpcRel16LoOrSparcGnuVtinheritOrPpcRel16LoOrPpc64Rel16LoOrArmRsbrel32 = 0xfa,
    /// PowerpcRel16Ha Or SparcRev32 Or half16   (sym+add-.)@ha Or half16   (sym+add-.)@ha Or ArmRrel32
    _PowerpcRel16HaOrSparcRev32OrPpcRel16HaOrPpc64Rel16HaOrArmRrel32 = 0xfc,
    _PowerpcGnuVtinheritOrSparcNumOrArmRabs22 = 0xfd,
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
    MicromipsPc23S2 = 0xad,
    /// ArmPrivate13 Or 16 bits LT-rel. function ptr
    _ArmPrivate13OrPariscLtoffFptr16F = 0x7d,
    /// AArch64Withdrawn Or ArmNum Or ShNum Or Keep this the last entry
    _AArch64WithdrawnOrArmNumOrShNumOrM32RNum = 0x100,
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
    AArch64GlobDat = 0x401,
    AArch64JumpSlot = 0x402,
    AArch64Relative = 0x403,
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
    /// Direct 64 bit.
    AArch64Abs64 = 0x101,
    /// PC-relative ADR imm. 20:0.
    AArch64TlsgdAdrPrel21 = 0x200,
    /// Copy symbol at runtime.
    AArch64Copy = 0x400,
    /// Module number, 64 bit.
    AArch64TlsDtpmod = 0x404,
    _ArmIrelativeOrShGot32 = 0xa0,
    /// @dtprel(sym + add), imm14
    Ia64Dtprel14 = 0xb1,
    /// @dtprel(sym + add), imm22
    Ia64Dtprel22 = 0xb2,
    /// @dtprel(sym + add), imm64
    Ia64Dtprel64I = 0xb3,
    ShPlt32 = 0xa1,
}
impl RelocationType {
    #[allow(non_upper_case_globals)]
    pub const FooNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    #[allow(non_upper_case_globals)]
    pub const PowerpcNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    #[allow(non_upper_case_globals)]
    pub const MipsNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    #[allow(non_upper_case_globals)]
    pub const TilegxNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    #[allow(non_upper_case_globals)]
    pub const X8664None : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    #[allow(non_upper_case_globals)]
    pub const SparcNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    #[allow(non_upper_case_globals)]
    pub const I386None : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc
    #[allow(non_upper_case_globals)]
    pub const M68KNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc.
    #[allow(non_upper_case_globals)]
    pub const PariscNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc
    #[allow(non_upper_case_globals)]
    pub const AlphaNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    #[allow(non_upper_case_globals)]
    pub const PpcNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No relocation.
    #[allow(non_upper_case_globals)]
    pub const AArch64None : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc
    #[allow(non_upper_case_globals)]
    pub const ArmNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// no reloc
    #[allow(non_upper_case_globals)]
    pub const CkcoreNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    #[allow(non_upper_case_globals)]
    pub const ShNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc.
    #[allow(non_upper_case_globals)]
    pub const S390None : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    #[allow(non_upper_case_globals)]
    pub const CrisNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc.
    #[allow(non_upper_case_globals)]
    pub const Mn10300None : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc.
    #[allow(non_upper_case_globals)]
    pub const M32RNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc.
    #[allow(non_upper_case_globals)]
    pub const Nios2None : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc
    #[allow(non_upper_case_globals)]
    pub const TileproNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    #[allow(non_upper_case_globals)]
    pub const RiscvNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// No reloc
    #[allow(non_upper_case_globals)]
    pub const BpfNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    #[allow(non_upper_case_globals)]
    pub const MetagHiaddr16 : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    #[allow(non_upper_case_globals)]
    pub const Nds32None : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    #[allow(non_upper_case_globals)]
    pub const Or1KNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    #[allow(non_upper_case_globals)]
    pub const Ppc64None : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    /// none
    #[allow(non_upper_case_globals)]
    pub const Ia64None : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    #[allow(non_upper_case_globals)]
    pub const ArcNone : Self = Self::_FooNoneOrPowerpcNoneOrMipsNoneOrTilegxNoneOrX8664NoneOrSparcNoneOrI386NoneOrM68KNoneOrPariscNoneOrAlphaNoneOrPpcNoneOrAArch64NoneOrArmNoneOrCkcoreNoneOrShNoneOrS390NoneOrCrisNoneOrMn10300NoneOrM32RNoneOrMicroblazeNoneOrNios2NoneOrTileproNoneOrRiscvNoneOrBpfNoneOrMetagHiaddr16OrNds32NoneOrOr1KNoneOrPpc64NoneOrIa64NoneOrArcNone;
    #[allow(non_upper_case_globals)]
    pub const Foo32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    #[allow(non_upper_case_globals)]
    pub const PowerpcAddr32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    #[allow(non_upper_case_globals)]
    pub const Mips16 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    #[allow(non_upper_case_globals)]
    pub const Tilegx64 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    #[allow(non_upper_case_globals)]
    pub const S3908 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    #[allow(non_upper_case_globals)]
    pub const X866464 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    #[allow(non_upper_case_globals)]
    pub const Sparc8 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    #[allow(non_upper_case_globals)]
    pub const ArmPc24 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    #[allow(non_upper_case_globals)]
    pub const I38632 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// Direct 32 bit
    #[allow(non_upper_case_globals)]
    pub const M68K32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// Direct 32-bit reference.
    #[allow(non_upper_case_globals)]
    pub const PariscDir32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// Direct 32 bit
    #[allow(non_upper_case_globals)]
    pub const AlphaReflong : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// 32bit absolute address
    #[allow(non_upper_case_globals)]
    pub const PpcAddr32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// Direct 32 bit.
    #[allow(non_upper_case_globals)]
    pub const AArch64P32Abs32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// direct 32 bit (S + A)
    #[allow(non_upper_case_globals)]
    pub const CkcoreAddr32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    #[allow(non_upper_case_globals)]
    pub const ShDir32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    #[allow(non_upper_case_globals)]
    pub const Cris8 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// Direct 32 bit.
    #[allow(non_upper_case_globals)]
    pub const Mn1030032 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// Direct 16 bit.
    #[allow(non_upper_case_globals)]
    pub const M32R16 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// Direct 32 bit.
    #[allow(non_upper_case_globals)]
    pub const Microblaze32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// Direct signed 16 bit.
    #[allow(non_upper_case_globals)]
    pub const Nios2S16 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// Direct 32 bit
    #[allow(non_upper_case_globals)]
    pub const Tilepro32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    #[allow(non_upper_case_globals)]
    pub const Riscv32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    #[allow(non_upper_case_globals)]
    pub const Bpf6464 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    #[allow(non_upper_case_globals)]
    pub const MetagLoaddr16 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    #[allow(non_upper_case_globals)]
    pub const Or1K32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    /// 32bit absolute address
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr32 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    #[allow(non_upper_case_globals)]
    pub const Arc8 : Self = Self::_Foo32OrPowerpcAddr32OrMips16OrTilegx64OrS3908OrX866464OrSparc8OrArmPc24OrI38632OrM68K32OrPariscDir32OrAlphaReflongOrPpcAddr32OrAArch64P32Abs32OrCkcoreAddr32OrShDir32OrCris8OrMn1030032OrM32R16OrMicroblaze32OrNios2S16OrTilepro32OrRiscv32OrBpf6464OrMetagLoaddr16OrOr1K32OrPpc64Addr32OrArc8;
    #[allow(non_upper_case_globals)]
    pub const FooIllegal : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    #[allow(non_upper_case_globals)]
    pub const PowerpcAddr14Brntaken : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    #[allow(non_upper_case_globals)]
    pub const TilegxHw0 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    #[allow(non_upper_case_globals)]
    pub const S390Copy : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    #[allow(non_upper_case_globals)]
    pub const MipsGot16 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    #[allow(non_upper_case_globals)]
    pub const X8664Gotpcrel : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    #[allow(non_upper_case_globals)]
    pub const SparcHi22 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    #[allow(non_upper_case_globals)]
    pub const ArmSbrel32 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    #[allow(non_upper_case_globals)]
    pub const I386Gotoff : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    /// 8 bit PC relative GOT entry
    #[allow(non_upper_case_globals)]
    pub const M68KGot8 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    /// 32-bit rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscPcrel32 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    /// PC relative 16 bit
    #[allow(non_upper_case_globals)]
    pub const AlphaSrel16 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    #[allow(non_upper_case_globals)]
    pub const PpcAddr14Brntaken : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    /// 32 bit adjust program base(B + A)
    #[allow(non_upper_case_globals)]
    pub const CkcoreRelative : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    #[allow(non_upper_case_globals)]
    pub const ShDir8L : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    #[allow(non_upper_case_globals)]
    pub const CrisCopy : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    /// Direct 24 bit.
    #[allow(non_upper_case_globals)]
    pub const Mn1030024 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    /// Low 16 bit.
    #[allow(non_upper_case_globals)]
    pub const M32RLo16 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    /// No reloc.
    #[allow(non_upper_case_globals)]
    pub const Microblaze64None : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    /// High 16 bit.
    #[allow(non_upper_case_globals)]
    pub const Nios2Hi16 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    /// High 16 bit, adjusted
    #[allow(non_upper_case_globals)]
    pub const TileproHa16 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    #[allow(non_upper_case_globals)]
    pub const RiscvTlsDtprel64 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    #[allow(non_upper_case_globals)]
    pub const MetagReg16Op1 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    #[allow(non_upper_case_globals)]
    pub const Or1K32Pcrel : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr14Brntaken : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    #[allow(non_upper_case_globals)]
    pub const ArcN16 : Self = Self::_FooIllegalOrPowerpcAddr14BrntakenOrTilegxHw0OrS390CopyOrMipsGot16OrX8664GotpcrelOrSparcHi22OrArmSbrel32OrI386GotoffOrM68KGot8OrPariscPcrel32OrAlphaSrel16OrPpcAddr14BrntakenOrCkcoreRelativeOrShDir8LOrCrisCopyOrMn1030024OrM32RLo16OrMicroblaze64NoneOrNios2Hi16OrTileproHa16OrRiscvTlsDtprel64OrMetagReg16Op1OrOr1K32PcrelOrPpc64Addr14BrntakenOrArcN16;
    #[allow(non_upper_case_globals)]
    pub const PariscTlsLe21L: Self =
        Self::_PariscTlsLe21LOrMicromipsCallLo16OrPariscTprel21LOrIa64LtoffTprel22;
    #[allow(non_upper_case_globals)]
    pub const MicromipsCallLo16: Self =
        Self::_PariscTlsLe21LOrMicromipsCallLo16OrPariscTprel21LOrIa64LtoffTprel22;
    /// TP-rel. address, left 21 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscTprel21L: Self =
        Self::_PariscTlsLe21LOrMicromipsCallLo16OrPariscTprel21LOrIa64LtoffTprel22;
    /// @ltoff(@tprel(s+a)), imm2
    #[allow(non_upper_case_globals)]
    pub const Ia64LtoffTprel22: Self =
        Self::_PariscTlsLe21LOrMicromipsCallLo16OrPariscTprel21LOrIa64LtoffTprel22;
    #[allow(non_upper_case_globals)]
    pub const PariscTlsLe14R: Self = Self::_PariscTlsLe14ROrPariscTprel14R;
    /// TP-rel. address, right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscTprel14R: Self = Self::_PariscTlsLe14ROrPariscTprel14R;
    #[allow(non_upper_case_globals)]
    pub const PariscTlsIe21L: Self = Self::_PariscTlsIe21LOrPariscLtoffTp21LOrShCopy;
    /// LT-TP-rel. address, left 21 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoffTp21L: Self = Self::_PariscTlsIe21LOrPariscLtoffTp21LOrShCopy;
    #[allow(non_upper_case_globals)]
    pub const ShCopy: Self = Self::_PariscTlsIe21LOrPariscLtoffTp21LOrShCopy;
    #[allow(non_upper_case_globals)]
    pub const PariscTlsIe14R: Self =
        Self::_PariscTlsIe14ROrMicromipsTlsGottprelOrPariscLtoffTp14ROrShGotoffOrIa64Dtpmod64Msb;
    #[allow(non_upper_case_globals)]
    pub const MicromipsTlsGottprel: Self =
        Self::_PariscTlsIe14ROrMicromipsTlsGottprelOrPariscLtoffTp14ROrShGotoffOrIa64Dtpmod64Msb;
    /// LT-TP-rel. address, right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoffTp14R: Self =
        Self::_PariscTlsIe14ROrMicromipsTlsGottprelOrPariscLtoffTp14ROrShGotoffOrIa64Dtpmod64Msb;
    #[allow(non_upper_case_globals)]
    pub const ShGotoff: Self =
        Self::_PariscTlsIe14ROrMicromipsTlsGottprelOrPariscLtoffTp14ROrShGotoffOrIa64Dtpmod64Msb;
    /// @dtpmod(sym + add), data8 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Dtpmod64Msb: Self =
        Self::_PariscTlsIe14ROrMicromipsTlsGottprelOrPariscLtoffTp14ROrShGotoffOrIa64Dtpmod64Msb;
    #[allow(non_upper_case_globals)]
    pub const PariscTlsTprel32: Self = Self::_PariscTlsTprel32OrMicromipsCallHi16OrPariscTprel32;
    #[allow(non_upper_case_globals)]
    pub const MicromipsCallHi16: Self = Self::_PariscTlsTprel32OrMicromipsCallHi16OrPariscTprel32;
    /// 32 bits TP-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscTprel32: Self = Self::_PariscTlsTprel32OrMicromipsCallHi16OrPariscTprel32;
    #[allow(non_upper_case_globals)]
    pub const PariscTlsTprel64: Self = Self::_PariscTlsTprel64OrPpcVleRel8OrPariscTprel64;
    #[allow(non_upper_case_globals)]
    pub const PpcVleRel8: Self = Self::_PariscTlsTprel64OrPpcVleRel8OrPariscTprel64;
    /// 64 bits TP-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscTprel64: Self = Self::_PariscTlsTprel64OrPpcVleRel8OrPariscTprel64;
    #[allow(non_upper_case_globals)]
    pub const S39012 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    #[allow(non_upper_case_globals)]
    pub const PowerpcAddr24 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    #[allow(non_upper_case_globals)]
    pub const Tilegx32 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    #[allow(non_upper_case_globals)]
    pub const Mips32 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    #[allow(non_upper_case_globals)]
    pub const X8664Pc32 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    #[allow(non_upper_case_globals)]
    pub const Sparc16 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    #[allow(non_upper_case_globals)]
    pub const ArmAbs32 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    #[allow(non_upper_case_globals)]
    pub const I386Pc32 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// Direct 16 bit
    #[allow(non_upper_case_globals)]
    pub const M68K16 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// Left 21 bits of eff. address.
    #[allow(non_upper_case_globals)]
    pub const PariscDir21L : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// Direct 64 bit
    #[allow(non_upper_case_globals)]
    pub const AlphaRefquad : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// 26bit address, 2 bits ignored.
    #[allow(non_upper_case_globals)]
    pub const PpcAddr24 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// disp ((S + A - P) >> 2) & 0xff
    #[allow(non_upper_case_globals)]
    pub const CkcorePcrelimm8By4 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    #[allow(non_upper_case_globals)]
    pub const ShRel32 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    #[allow(non_upper_case_globals)]
    pub const Cris16 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// Direct 16 bit.
    #[allow(non_upper_case_globals)]
    pub const Mn1030016 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// Direct 32 bit.
    #[allow(non_upper_case_globals)]
    pub const M32R32 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// PC relative 32 bit.
    #[allow(non_upper_case_globals)]
    pub const Microblaze32Pcrel : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// Direct unsigned 16 bit.
    #[allow(non_upper_case_globals)]
    pub const Nios2U16 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// Direct 16 bit
    #[allow(non_upper_case_globals)]
    pub const Tilepro16 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    #[allow(non_upper_case_globals)]
    pub const Riscv64 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// 32bit absolute address
    #[allow(non_upper_case_globals)]
    pub const MetagAddr32 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    #[allow(non_upper_case_globals)]
    pub const Or1K16 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    /// 26bit address, word aligned
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr24 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    #[allow(non_upper_case_globals)]
    pub const Arc16 : Self = Self::_S39012OrPowerpcAddr24OrTilegx32OrMips32OrX8664Pc32OrSparc16OrArmAbs32OrI386Pc32OrM68K16OrPariscDir21LOrAlphaRefquadOrPpcAddr24OrCkcorePcrelimm8By4OrShRel32OrCris16OrMn1030016OrM32R32OrMicroblaze32PcrelOrNios2U16OrTilepro16OrRiscv64OrMetagAddr32OrOr1K16OrPpc64Addr24OrArc16;
    #[allow(non_upper_case_globals)]
    pub const S39016 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    #[allow(non_upper_case_globals)]
    pub const PowerpcAddr16 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    #[allow(non_upper_case_globals)]
    pub const Tilegx16 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    #[allow(non_upper_case_globals)]
    pub const MipsRel32 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    #[allow(non_upper_case_globals)]
    pub const X8664Got32 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    #[allow(non_upper_case_globals)]
    pub const Sparc32 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    #[allow(non_upper_case_globals)]
    pub const ArmRel32 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    #[allow(non_upper_case_globals)]
    pub const I386Got32 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// Direct 8 bit
    #[allow(non_upper_case_globals)]
    pub const M68K8 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// Right 17 bits of eff. address.
    #[allow(non_upper_case_globals)]
    pub const PariscDir17R : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// GP relative 32 bit
    #[allow(non_upper_case_globals)]
    pub const AlphaGprel32 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// 16bit absolute address
    #[allow(non_upper_case_globals)]
    pub const PpcAddr16 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// disp ((S + A - P) >> 1) & 0x7ff
    #[allow(non_upper_case_globals)]
    pub const CkcorePcrelimm11By2 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    #[allow(non_upper_case_globals)]
    pub const ShDir8Wpn : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    #[allow(non_upper_case_globals)]
    pub const Cris32 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// Direct 8 bit.
    #[allow(non_upper_case_globals)]
    pub const Mn103008 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// Direct 24 bit.
    #[allow(non_upper_case_globals)]
    pub const M32R24 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// PC relative 64 bit.
    #[allow(non_upper_case_globals)]
    pub const Microblaze64Pcrel : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// PC relative 16 bit.
    #[allow(non_upper_case_globals)]
    pub const Nios2Pcrel16 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// Direct 8 bit
    #[allow(non_upper_case_globals)]
    pub const Tilepro8 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    #[allow(non_upper_case_globals)]
    pub const RiscvRelative : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// No reloc
    #[allow(non_upper_case_globals)]
    pub const MetagNone : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    #[allow(non_upper_case_globals)]
    pub const Or1K8 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    /// 16bit absolute address
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr16 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    #[allow(non_upper_case_globals)]
    pub const Arc24 : Self = Self::_S39016OrPowerpcAddr16OrTilegx16OrMipsRel32OrX8664Got32OrSparc32OrArmRel32OrI386Got32OrM68K8OrPariscDir17ROrAlphaGprel32OrPpcAddr16OrCkcorePcrelimm11By2OrShDir8WpnOrCris32OrMn103008OrM32R24OrMicroblaze64PcrelOrNios2Pcrel16OrTilepro8OrRiscvRelativeOrMetagNoneOrOr1K8OrPpc64Addr16OrArc24;
    #[allow(non_upper_case_globals)]
    pub const S39032 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    #[allow(non_upper_case_globals)]
    pub const PowerpcAddr16Lo : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    #[allow(non_upper_case_globals)]
    pub const Mips26 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    #[allow(non_upper_case_globals)]
    pub const Tilegx8 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    #[allow(non_upper_case_globals)]
    pub const X8664Plt32 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    #[allow(non_upper_case_globals)]
    pub const SparcDisp8 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    #[allow(non_upper_case_globals)]
    pub const ArmLdrPcG0 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    #[allow(non_upper_case_globals)]
    pub const I386Plt32 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// PC relative 32 bit
    #[allow(non_upper_case_globals)]
    pub const M68KPc32 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// 17 bits of eff. address.
    #[allow(non_upper_case_globals)]
    pub const PariscDir17F : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// GP relative 16 bit w/optimization
    #[allow(non_upper_case_globals)]
    pub const AlphaLiteral : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// lower 16bit of absolute address
    #[allow(non_upper_case_globals)]
    pub const PpcAddr16Lo : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    #[allow(non_upper_case_globals)]
    pub const ArmPc13 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    #[allow(non_upper_case_globals)]
    pub const ShInd12W : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    #[allow(non_upper_case_globals)]
    pub const Cris8Pcrel : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// PC-relative 32-bit.
    #[allow(non_upper_case_globals)]
    pub const Mn10300Pcrel32 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// PC relative 10 bit shifted.
    #[allow(non_upper_case_globals)]
    pub const M32R10Pcrel : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// Low 16 bits of PCREL32.
    #[allow(non_upper_case_globals)]
    pub const Microblaze32PcrelLo : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// Direct call.
    #[allow(non_upper_case_globals)]
    pub const Nios2Call26 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// PC relative 32 bit
    #[allow(non_upper_case_globals)]
    pub const Tilepro32Pcrel : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    #[allow(non_upper_case_globals)]
    pub const RiscvCopy : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    #[allow(non_upper_case_globals)]
    pub const MetagRelbranch : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    #[allow(non_upper_case_globals)]
    pub const Or1KLo16InInsn : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    /// lower 16bits of address
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr16Lo : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    #[allow(non_upper_case_globals)]
    pub const Arc32 : Self = Self::_S39032OrPowerpcAddr16LoOrMips26OrTilegx8OrX8664Plt32OrSparcDisp8OrArmLdrPcG0OrI386Plt32OrM68KPc32OrPariscDir17FOrAlphaLiteralOrPpcAddr16LoOrArmPc13OrShInd12WOrCris8PcrelOrMn10300Pcrel32OrM32R10PcrelOrMicroblaze32PcrelLoOrNios2Call26OrTilepro32PcrelOrRiscvCopyOrMetagRelbranchOrOr1KLo16InInsnOrPpc64Addr16LoOrArc32;
    #[allow(non_upper_case_globals)]
    pub const S390Pc32 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    #[allow(non_upper_case_globals)]
    pub const PowerpcAddr16Hi : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    #[allow(non_upper_case_globals)]
    pub const MipsHi16 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    #[allow(non_upper_case_globals)]
    pub const Tilegx64Pcrel : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    #[allow(non_upper_case_globals)]
    pub const X8664Copy : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    #[allow(non_upper_case_globals)]
    pub const SparcDisp16 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    #[allow(non_upper_case_globals)]
    pub const ArmAbs16 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    #[allow(non_upper_case_globals)]
    pub const I386Copy : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// PC relative 16 bit
    #[allow(non_upper_case_globals)]
    pub const M68KPc16 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// Optimization hint for LITERAL
    #[allow(non_upper_case_globals)]
    pub const AlphaLituse : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// high 16bit of absolute address
    #[allow(non_upper_case_globals)]
    pub const PpcAddr16Hi : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// 32-bit rel (S + A - P)
    #[allow(non_upper_case_globals)]
    pub const CkcorePcrel32 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    #[allow(non_upper_case_globals)]
    pub const ShDir8Wpl : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    #[allow(non_upper_case_globals)]
    pub const Cris16Pcrel : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// PC-relative 16-bit signed.
    #[allow(non_upper_case_globals)]
    pub const Mn10300Pcrel16 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// PC relative 18 bit shifted.
    #[allow(non_upper_case_globals)]
    pub const M32R18Pcrel : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// Direct 64 bit.
    #[allow(non_upper_case_globals)]
    pub const Microblaze64 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// 5 bit constant expression.
    #[allow(non_upper_case_globals)]
    pub const Nios2Imm5 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// PC relative 16 bit
    #[allow(non_upper_case_globals)]
    pub const Tilepro16Pcrel : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    #[allow(non_upper_case_globals)]
    pub const RiscvJumpSlot : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    #[allow(non_upper_case_globals)]
    pub const MetagGetsetoff : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    #[allow(non_upper_case_globals)]
    pub const Or1KHi16InInsn : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    /// high 16bits of address.
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr16Hi : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    #[allow(non_upper_case_globals)]
    pub const ArcB26 : Self = Self::_S390Pc32OrPowerpcAddr16HiOrMipsHi16OrTilegx64PcrelOrX8664CopyOrSparcDisp16OrArmAbs16OrI386CopyOrM68KPc16OrAlphaLituseOrPpcAddr16HiOrCkcorePcrel32OrShDir8WplOrCris16PcrelOrMn10300Pcrel16OrM32R18PcrelOrMicroblaze64OrNios2Imm5OrTilepro16PcrelOrRiscvJumpSlotOrMetagGetsetoffOrOr1KHi16InInsnOrPpc64Addr16HiOrArcB26;
    #[allow(non_upper_case_globals)]
    pub const S390Got12 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    #[allow(non_upper_case_globals)]
    pub const PowerpcAddr16Ha : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    #[allow(non_upper_case_globals)]
    pub const MipsLo16 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    #[allow(non_upper_case_globals)]
    pub const Tilegx32Pcrel : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    #[allow(non_upper_case_globals)]
    pub const X8664GlobDat : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    #[allow(non_upper_case_globals)]
    pub const SparcDisp32 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    #[allow(non_upper_case_globals)]
    pub const ArmAbs12 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    #[allow(non_upper_case_globals)]
    pub const I386GlobDat : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// PC relative 8 bit
    #[allow(non_upper_case_globals)]
    pub const M68KPc8 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// Right 14 bits of eff. address.
    #[allow(non_upper_case_globals)]
    pub const PariscDir14R : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// Add displacement to GP
    #[allow(non_upper_case_globals)]
    pub const AlphaGpdisp : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// adjusted high 16bit
    #[allow(non_upper_case_globals)]
    pub const PpcAddr16Ha : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// disp ((S + A - P) >>1) & 0x7ff
    #[allow(non_upper_case_globals)]
    pub const CkcorePcreljsrImm11By2 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    #[allow(non_upper_case_globals)]
    pub const ShDir8Wpz : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    #[allow(non_upper_case_globals)]
    pub const Cris32Pcrel : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// PC-relative 8-bit signed.
    #[allow(non_upper_case_globals)]
    pub const Mn10300Pcrel8 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// PC relative 26 bit shifted.
    #[allow(non_upper_case_globals)]
    pub const M32R26Pcrel : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// Low 16 bit.
    #[allow(non_upper_case_globals)]
    pub const Microblaze32Lo : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// 5 bit expression, shift 22.
    #[allow(non_upper_case_globals)]
    pub const Nios2CacheOpx : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// PC relative 8 bit
    #[allow(non_upper_case_globals)]
    pub const Tilepro8Pcrel : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    #[allow(non_upper_case_globals)]
    pub const RiscvTlsDtpmod32 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    #[allow(non_upper_case_globals)]
    pub const MetagReg32Op1 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    #[allow(non_upper_case_globals)]
    pub const Or1KInsnRel26 : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    /// adjusted high 16bits.
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr16Ha : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    #[allow(non_upper_case_globals)]
    pub const ArcB22Pcrel : Self = Self::_S390Got12OrPowerpcAddr16HaOrMipsLo16OrTilegx32PcrelOrX8664GlobDatOrSparcDisp32OrArmAbs12OrI386GlobDatOrM68KPc8OrPariscDir14ROrAlphaGpdispOrPpcAddr16HaOrCkcorePcreljsrImm11By2OrShDir8WpzOrCris32PcrelOrMn10300Pcrel8OrM32R26PcrelOrMicroblaze32LoOrNios2CacheOpxOrTilepro8PcrelOrRiscvTlsDtpmod32OrMetagReg32Op1OrOr1KInsnRel26OrPpc64Addr16HaOrArcB22Pcrel;
    #[allow(non_upper_case_globals)]
    pub const S390Got32 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    #[allow(non_upper_case_globals)]
    pub const PowerpcAddr14 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    #[allow(non_upper_case_globals)]
    pub const Tilegx16Pcrel : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    #[allow(non_upper_case_globals)]
    pub const MipsGprel16 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    #[allow(non_upper_case_globals)]
    pub const X8664JumpSlot : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    #[allow(non_upper_case_globals)]
    pub const SparcWdisp30 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    #[allow(non_upper_case_globals)]
    pub const ArmThmAbs5 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    #[allow(non_upper_case_globals)]
    pub const I386JumpSlot : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// 32 bit PC relative GOT entry
    #[allow(non_upper_case_globals)]
    pub const M68KGot32 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// Create PLT entry
    #[allow(non_upper_case_globals)]
    pub const I386JmpSlot : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// PC+4 relative 23 bit shifted
    #[allow(non_upper_case_globals)]
    pub const AlphaBraddr : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// 16bit address, 2 bits ignored
    #[allow(non_upper_case_globals)]
    pub const PpcAddr14 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    #[allow(non_upper_case_globals)]
    pub const ShDir8Bp : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    #[allow(non_upper_case_globals)]
    pub const CrisGnuVtinherit : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// Ancient C++ vtable garbage...
    #[allow(non_upper_case_globals)]
    pub const Mn10300GnuVtinherit : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// High 16 bit with unsigned low.
    #[allow(non_upper_case_globals)]
    pub const M32RHi16Ulo : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// Read-only small data area.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeSro32 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// 6 bit constant expression.
    #[allow(non_upper_case_globals)]
    pub const Nios2Imm6 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// Low 16 bit
    #[allow(non_upper_case_globals)]
    pub const TileproLo16 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    #[allow(non_upper_case_globals)]
    pub const RiscvTlsDtpmod64 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    #[allow(non_upper_case_globals)]
    pub const MetagReg32Op2 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    #[allow(non_upper_case_globals)]
    pub const Or1KGnuVtentry : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    /// 16bit address, word aligned
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr14 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    #[allow(non_upper_case_globals)]
    pub const ArcH30 : Self = Self::_S390Got32OrPowerpcAddr14OrTilegx16PcrelOrMipsGprel16OrX8664JumpSlotOrSparcWdisp30OrArmThmAbs5OrI386JumpSlotOrM68KGot32OrI386JmpSlotOrAlphaBraddrOrPpcAddr14OrShDir8BpOrCrisGnuVtinheritOrMn10300GnuVtinheritOrM32RHi16UloOrMicroblazeSro32OrNios2Imm6OrTileproLo16OrRiscvTlsDtpmod64OrMetagReg32Op2OrOr1KGnuVtentryOrPpc64Addr14OrArcH30;
    #[allow(non_upper_case_globals)]
    pub const S390Plt32 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    #[allow(non_upper_case_globals)]
    pub const PowerpcAddr14Brtaken : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    #[allow(non_upper_case_globals)]
    pub const MipsLiteral : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    #[allow(non_upper_case_globals)]
    pub const Tilegx8Pcrel : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    #[allow(non_upper_case_globals)]
    pub const X8664Relative : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    #[allow(non_upper_case_globals)]
    pub const SparcWdisp22 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    #[allow(non_upper_case_globals)]
    pub const ArmAbs8 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    #[allow(non_upper_case_globals)]
    pub const I386Relative : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    /// 16 bit PC relative GOT entry
    #[allow(non_upper_case_globals)]
    pub const M68KGot16 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    /// PC+4 relative 16 bit shifted
    #[allow(non_upper_case_globals)]
    pub const AlphaHint : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    #[allow(non_upper_case_globals)]
    pub const PpcAddr14Brtaken : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    #[allow(non_upper_case_globals)]
    pub const ShDir8W : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    #[allow(non_upper_case_globals)]
    pub const CrisGnuVtentry : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    /// ... collection annotation.
    #[allow(non_upper_case_globals)]
    pub const Mn10300GnuVtentry : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    /// High 16 bit with signed low.
    #[allow(non_upper_case_globals)]
    pub const M32RHi16Slo : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    /// Read-write small data area.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeSrw32 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    /// 8 bit constant expression.
    #[allow(non_upper_case_globals)]
    pub const Nios2Imm8 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    /// High 16 bit
    #[allow(non_upper_case_globals)]
    pub const TileproHi16 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    #[allow(non_upper_case_globals)]
    pub const RiscvTlsDtprel32 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    #[allow(non_upper_case_globals)]
    pub const MetagReg32Op3 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    #[allow(non_upper_case_globals)]
    pub const Or1KGnuVtinherit : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr14Brtaken : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    #[allow(non_upper_case_globals)]
    pub const ArcN8 : Self = Self::_S390Plt32OrPowerpcAddr14BrtakenOrMipsLiteralOrTilegx8PcrelOrX8664RelativeOrSparcWdisp22OrArmAbs8OrI386RelativeOrM68KGot16OrAlphaHintOrPpcAddr14BrtakenOrShDir8WOrCrisGnuVtentryOrMn10300GnuVtentryOrM32RHi16SloOrMicroblazeSrw32OrNios2Imm8OrTileproHi16OrRiscvTlsDtprel32OrMetagReg32Op3OrOr1KGnuVtinheritOrPpc64Addr14BrtakenOrArcN8;
    #[allow(non_upper_case_globals)]
    pub const S390GlobDat : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    #[allow(non_upper_case_globals)]
    pub const PowerpcRel24 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    #[allow(non_upper_case_globals)]
    pub const MipsPc16 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    #[allow(non_upper_case_globals)]
    pub const TilegxHw1 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    #[allow(non_upper_case_globals)]
    pub const X866432 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    #[allow(non_upper_case_globals)]
    pub const Sparc22 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    #[allow(non_upper_case_globals)]
    pub const ArmThmCall : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    #[allow(non_upper_case_globals)]
    pub const I386Gotpc : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// 32 bit GOT offset
    #[allow(non_upper_case_globals)]
    pub const M68KGot32O : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// Left 21 bits of rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscPcrel21L : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// PC relative 32 bit
    #[allow(non_upper_case_globals)]
    pub const AlphaSrel32 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// PC relative 26 bit
    #[allow(non_upper_case_globals)]
    pub const PpcRel24 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// PC relative 24 bit (Thumb32 BL).
    #[allow(non_upper_case_globals)]
    pub const ArmThmPc22 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// 32 bit adjust by program base
    #[allow(non_upper_case_globals)]
    pub const CkcoreCopy : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    #[allow(non_upper_case_globals)]
    pub const CrisGlobDat : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// 32-bit PCrel offset to GOT.
    #[allow(non_upper_case_globals)]
    pub const Mn10300Gotpc32 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// 16 bit offset in SDA.
    #[allow(non_upper_case_globals)]
    pub const M32RSda16 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// Symbol Op Symbol relocation.
    #[allow(non_upper_case_globals)]
    pub const Microblaze32SymOpSym : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// Low 16 bit.
    #[allow(non_upper_case_globals)]
    pub const Nios2Lo16 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// Copy relocation
    #[allow(non_upper_case_globals)]
    pub const TileproCopy : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    #[allow(non_upper_case_globals)]
    pub const RiscvTlsTprel32 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    #[allow(non_upper_case_globals)]
    pub const Bpf6432 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    #[allow(non_upper_case_globals)]
    pub const MetagReg16Op2 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    #[allow(non_upper_case_globals)]
    pub const Or1K16Pcrel : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    /// PC-rel. 26 bit, word aligned
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel24 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    #[allow(non_upper_case_globals)]
    pub const ArcN24 : Self = Self::_S390GlobDatOrPowerpcRel24OrMipsPc16OrTilegxHw1OrX866432OrSparc22OrArmThmCallOrI386GotpcOrM68KGot32OOrPariscPcrel21LOrAlphaSrel32OrPpcRel24OrArmThmPc22OrCkcoreCopyOrCrisGlobDatOrMn10300Gotpc32OrM32RSda16OrMicroblaze32SymOpSymOrNios2Lo16OrTileproCopyOrRiscvTlsTprel32OrBpf6432OrMetagReg16Op2OrOr1K16PcrelOrPpc64Rel24OrArcN24;
    #[allow(non_upper_case_globals)]
    pub const S390JmpSlot : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    #[allow(non_upper_case_globals)]
    pub const PowerpcRel14 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    #[allow(non_upper_case_globals)]
    pub const TilegxHw2 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    #[allow(non_upper_case_globals)]
    pub const MipsCall16 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    #[allow(non_upper_case_globals)]
    pub const X866432S : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    #[allow(non_upper_case_globals)]
    pub const Sparc13 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    #[allow(non_upper_case_globals)]
    pub const ArmThmPc8 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// 16 bit GOT offset
    #[allow(non_upper_case_globals)]
    pub const M68KGot16O : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    #[allow(non_upper_case_globals)]
    pub const I38632Plt : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// Right 17 bits of rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscPcrel17R : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// PC relative 64 bit
    #[allow(non_upper_case_globals)]
    pub const AlphaSrel64 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// PC relative 16 bit
    #[allow(non_upper_case_globals)]
    pub const PpcRel14 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// off between got and sym (S)
    #[allow(non_upper_case_globals)]
    pub const CkcoreGlobDat : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    #[allow(non_upper_case_globals)]
    pub const CrisJumpSlot : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// 16-bit PCrel offset to GOT.
    #[allow(non_upper_case_globals)]
    pub const Mn10300Gotpc16 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    #[allow(non_upper_case_globals)]
    pub const M32RGnuVtinherit : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// GNU C++ vtable hierarchy.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeGnuVtinherit : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// High 16 bit, adjusted.
    #[allow(non_upper_case_globals)]
    pub const Nios2Hiadj16 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// Create GOT entry
    #[allow(non_upper_case_globals)]
    pub const TileproGlobDat : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    #[allow(non_upper_case_globals)]
    pub const RiscvTlsTprel64 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    #[allow(non_upper_case_globals)]
    pub const MetagReg16Op3 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    #[allow(non_upper_case_globals)]
    pub const Or1K8Pcrel : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    /// PC relative 16 bit
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel14 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    #[allow(non_upper_case_globals)]
    pub const ArcN32 : Self = Self::_S390JmpSlotOrPowerpcRel14OrTilegxHw2OrMipsCall16OrX866432SOrSparc13OrArmThmPc8OrM68KGot16OOrI38632PltOrPariscPcrel17ROrAlphaSrel64OrPpcRel14OrCkcoreGlobDatOrCrisJumpSlotOrMn10300Gotpc16OrM32RGnuVtinheritOrMicroblazeGnuVtinheritOrNios2Hiadj16OrTileproGlobDatOrRiscvTlsTprel64OrMetagReg16Op3OrOr1K8PcrelOrPpc64Rel14OrArcN32;
    #[allow(non_upper_case_globals)]
    pub const S390Relative : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    #[allow(non_upper_case_globals)]
    pub const PowerpcRel14Brtaken : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    #[allow(non_upper_case_globals)]
    pub const MipsGprel32 : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    #[allow(non_upper_case_globals)]
    pub const TilegxHw3 : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    #[allow(non_upper_case_globals)]
    pub const X866416 : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    #[allow(non_upper_case_globals)]
    pub const SparcLo10 : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    #[allow(non_upper_case_globals)]
    pub const ArmBrelAdj : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    /// 8 bit GOT offset
    #[allow(non_upper_case_globals)]
    pub const M68KGot8O : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    /// 17 bits of rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscPcrel17F : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    #[allow(non_upper_case_globals)]
    pub const PpcRel14Brtaken : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    #[allow(non_upper_case_globals)]
    pub const ArmAmpVcall9 : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    /// PLT entry (S)
    #[allow(non_upper_case_globals)]
    pub const CkcoreJumpSlot : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    #[allow(non_upper_case_globals)]
    pub const CrisRelative : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    /// 32-bit offset from GOT.
    #[allow(non_upper_case_globals)]
    pub const Mn10300Gotoff32 : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    #[allow(non_upper_case_globals)]
    pub const M32RGnuVtentry : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    /// GNU C++ vtable member usage.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeGnuVtentry : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    /// 32 bit symbol value + addend.
    #[allow(non_upper_case_globals)]
    pub const Nios2BfdReloc32 : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    /// Create PLT entry
    #[allow(non_upper_case_globals)]
    pub const TileproJmpSlot : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    #[allow(non_upper_case_globals)]
    pub const MetagReg32Op4 : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    #[allow(non_upper_case_globals)]
    pub const Or1KGotpcHi16 : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel14Brtaken : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    #[allow(non_upper_case_globals)]
    pub const ArcSda : Self = Self::_S390RelativeOrPowerpcRel14BrtakenOrMipsGprel32OrTilegxHw3OrX866416OrSparcLo10OrArmBrelAdjOrM68KGot8OOrPariscPcrel17FOrPpcRel14BrtakenOrArmAmpVcall9OrCkcoreJumpSlotOrCrisRelativeOrMn10300Gotoff32OrM32RGnuVtentryOrMicroblazeGnuVtentryOrNios2BfdReloc32OrTileproJmpSlotOrMetagReg32Op4OrOr1KGotpcHi16OrPpc64Rel14BrtakenOrArcSda;
    #[allow(non_upper_case_globals)]
    pub const S390Gotoff32 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    #[allow(non_upper_case_globals)]
    pub const PowerpcRel14Brntaken : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    #[allow(non_upper_case_globals)]
    pub const MipsUnused1 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    #[allow(non_upper_case_globals)]
    pub const TilegxHw0Last : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    #[allow(non_upper_case_globals)]
    pub const X8664Pc16 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    #[allow(non_upper_case_globals)]
    pub const SparcGot10 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    #[allow(non_upper_case_globals)]
    pub const ArmTlsDesc : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    /// 32 bit PC relative PLT address
    #[allow(non_upper_case_globals)]
    pub const M68KPlt32 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    #[allow(non_upper_case_globals)]
    pub const PpcRel14Brntaken : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    /// Obsolete static relocation.
    #[allow(non_upper_case_globals)]
    pub const ArmSwi24 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    /// offset to GOT (S + A - GOT)
    #[allow(non_upper_case_globals)]
    pub const CkcoreGotoff : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    #[allow(non_upper_case_globals)]
    pub const Cris16Got : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    /// 24-bit offset from GOT.
    #[allow(non_upper_case_globals)]
    pub const Mn10300Gotoff24 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    /// PC-relative GOT offset.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeGotpc64 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    /// 16 bit symbol value + addend.
    #[allow(non_upper_case_globals)]
    pub const Nios2BfdReloc16 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    /// Adjust by program base
    #[allow(non_upper_case_globals)]
    pub const TileproRelative : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    #[allow(non_upper_case_globals)]
    pub const MetagHiog : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    #[allow(non_upper_case_globals)]
    pub const Or1KGotpcLo16 : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel14Brntaken : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    #[allow(non_upper_case_globals)]
    pub const ArcSectoff : Self = Self::_S390Gotoff32OrPowerpcRel14BrntakenOrMipsUnused1OrTilegxHw0LastOrX8664Pc16OrSparcGot10OrArmTlsDescOrM68KPlt32OrPpcRel14BrntakenOrArmSwi24OrCkcoreGotoffOrCris16GotOrMn10300Gotoff24OrMicroblazeGotpc64OrNios2BfdReloc16OrTileproRelativeOrMetagHiogOrOr1KGotpcLo16OrPpc64Rel14BrntakenOrArcSectoff;
    #[allow(non_upper_case_globals)]
    pub const S390Gotpc : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGot16 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    #[allow(non_upper_case_globals)]
    pub const MipsUnused2 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    #[allow(non_upper_case_globals)]
    pub const TilegxHw1Last : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    #[allow(non_upper_case_globals)]
    pub const X86648 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    #[allow(non_upper_case_globals)]
    pub const SparcGot13 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    #[allow(non_upper_case_globals)]
    pub const ArmThmSwi8 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    /// 16 bit PC relative PLT address
    #[allow(non_upper_case_globals)]
    pub const M68KPlt16 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    /// Offset in static TLS block
    #[allow(non_upper_case_globals)]
    pub const I386TlsTpoff : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    /// Right 14 bits of rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscPcrel14R : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    #[allow(non_upper_case_globals)]
    pub const PpcGot16 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    /// PC offset to GOT (GOT + A - P)
    #[allow(non_upper_case_globals)]
    pub const CkcoreGotpc : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    #[allow(non_upper_case_globals)]
    pub const Cris32Got : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    /// 16-bit offset from GOT.
    #[allow(non_upper_case_globals)]
    pub const Mn10300Gotoff16 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    /// GOT entry offset.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeGot64 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    /// 8 bit symbol value + addend.
    #[allow(non_upper_case_globals)]
    pub const Nios2BfdReloc8 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    /// X1 pipe branch offset
    #[allow(non_upper_case_globals)]
    pub const TileproBroffX1 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    #[allow(non_upper_case_globals)]
    pub const MetagLoog : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    #[allow(non_upper_case_globals)]
    pub const Or1KGot16 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Got16 : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    #[allow(non_upper_case_globals)]
    pub const ArcS21HPcrel : Self = Self::_S390GotpcOrPowerpcGot16OrMipsUnused2OrTilegxHw1LastOrX86648OrSparcGot13OrArmThmSwi8OrM68KPlt16OrI386TlsTpoffOrPariscPcrel14ROrPpcGot16OrCkcoreGotpcOrCris32GotOrMn10300Gotoff16OrMicroblazeGot64OrNios2BfdReloc8OrTileproBroffX1OrMetagLoogOrOr1KGot16OrPpc64Got16OrArcS21HPcrel;
    #[allow(non_upper_case_globals)]
    pub const S390Got16 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGot16Lo : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    #[allow(non_upper_case_globals)]
    pub const MipsUnused3 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    #[allow(non_upper_case_globals)]
    pub const TilegxHw2Last : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    #[allow(non_upper_case_globals)]
    pub const X8664Pc8 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    #[allow(non_upper_case_globals)]
    pub const SparcGot22 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    #[allow(non_upper_case_globals)]
    pub const ArmXpc25 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    #[allow(non_upper_case_globals)]
    pub const I386TlsIe : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    /// 8 bit PC relative PLT address
    #[allow(non_upper_case_globals)]
    pub const M68KPlt8 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    #[allow(non_upper_case_globals)]
    pub const PpcGot16Lo : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    /// 32 bit GOT entry (G)
    #[allow(non_upper_case_globals)]
    pub const CkcoreGot32 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    #[allow(non_upper_case_globals)]
    pub const Cris16Gotplt : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    /// 32-bit PCrel to PLT entry.
    #[allow(non_upper_case_globals)]
    pub const Mn10300Plt32 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    /// PLT offset (PC-relative).
    #[allow(non_upper_case_globals)]
    pub const MicroblazePlt64 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    /// 16 bit GP pointer offset.
    #[allow(non_upper_case_globals)]
    pub const Nios2Gprel : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    /// X1 pipe jump offset
    #[allow(non_upper_case_globals)]
    pub const TileproJofflongX1 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    #[allow(non_upper_case_globals)]
    pub const MetagRel8 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    #[allow(non_upper_case_globals)]
    pub const Or1KPlt26 : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Got16Lo : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    #[allow(non_upper_case_globals)]
    pub const ArcS21WPcrel : Self = Self::_S390Got16OrPowerpcGot16LoOrMipsUnused3OrTilegxHw2LastOrX8664Pc8OrSparcGot22OrArmXpc25OrI386TlsIeOrM68KPlt8OrPpcGot16LoOrCkcoreGot32OrCris16GotpltOrMn10300Plt32OrMicroblazePlt64OrNios2GprelOrTileproJofflongX1OrMetagRel8OrOr1KPlt26OrPpc64Got16LoOrArcS21WPcrel;
    #[allow(non_upper_case_globals)]
    pub const S390Pc16 : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGot16Hi : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    #[allow(non_upper_case_globals)]
    pub const MipsShift5 : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    #[allow(non_upper_case_globals)]
    pub const TilegxCopy : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    #[allow(non_upper_case_globals)]
    pub const SparcPc10 : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    #[allow(non_upper_case_globals)]
    pub const ArmThmXpc22 : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    #[allow(non_upper_case_globals)]
    pub const I386TlsGotie : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    /// 32 bit PLT offset
    #[allow(non_upper_case_globals)]
    pub const M68KPlt32O : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    #[allow(non_upper_case_globals)]
    pub const PpcGot16Hi : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    /// 32 bit PLT entry (G)
    #[allow(non_upper_case_globals)]
    pub const CkcorePlt32 : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    #[allow(non_upper_case_globals)]
    pub const Cris32Gotplt : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    /// ID of module containing symbol
    #[allow(non_upper_case_globals)]
    pub const X8664Dtpmod64 : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    /// 16-bit PCrel to PLT entry.
    #[allow(non_upper_case_globals)]
    pub const Mn10300Plt16 : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    /// Adjust by program base.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeRel : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    /// GNU C++ vtable hierarchy.
    #[allow(non_upper_case_globals)]
    pub const Nios2GnuVtinherit : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    /// X1 pipe jump offset to PLT
    #[allow(non_upper_case_globals)]
    pub const TileproJofflongX1Plt : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    #[allow(non_upper_case_globals)]
    pub const RiscvBranch : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    #[allow(non_upper_case_globals)]
    pub const MetagRel16 : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    #[allow(non_upper_case_globals)]
    pub const Or1KGotoffHi16 : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Got16Hi : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    #[allow(non_upper_case_globals)]
    pub const ArcS25HPcrel : Self = Self::_S390Pc16OrPowerpcGot16HiOrMipsShift5OrTilegxCopyOrSparcPc10OrArmThmXpc22OrI386TlsGotieOrM68KPlt32OOrPpcGot16HiOrCkcorePlt32OrCris32GotpltOrX8664Dtpmod64OrMn10300Plt16OrMicroblazeRelOrNios2GnuVtinheritOrTileproJofflongX1PltOrRiscvBranchOrMetagRel16OrOr1KGotoffHi16OrPpc64Got16HiOrArcS25HPcrel;
    #[allow(non_upper_case_globals)]
    pub const S390Pc16Dbl : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGot16Ha : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    #[allow(non_upper_case_globals)]
    pub const MipsShift6 : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    #[allow(non_upper_case_globals)]
    pub const TilegxGlobDat : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    #[allow(non_upper_case_globals)]
    pub const X8664Dtpoff64 : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    #[allow(non_upper_case_globals)]
    pub const SparcPc22 : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    #[allow(non_upper_case_globals)]
    pub const ArmTlsDtpmod32 : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    #[allow(non_upper_case_globals)]
    pub const I386TlsLe : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    /// 16 bit PLT offset
    #[allow(non_upper_case_globals)]
    pub const M68KPlt16O : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    /// GP relative 32 bit, high 16 bits
    #[allow(non_upper_case_globals)]
    pub const AlphaGprelhigh : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    #[allow(non_upper_case_globals)]
    pub const PpcGot16Ha : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    /// GOT entry in GLOB_DAT (GOT + G)
    #[allow(non_upper_case_globals)]
    pub const CkcoreAddrgot : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    #[allow(non_upper_case_globals)]
    pub const Cris32Gotrel : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    /// 32-bit offset to GOT entry.
    #[allow(non_upper_case_globals)]
    pub const Mn10300Got32 : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    /// Create PLT entry.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeJumpSlot : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    /// GNU C++ vtable member usage.
    #[allow(non_upper_case_globals)]
    pub const Nios2GnuVtentry : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    /// X0 pipe 8-bit
    #[allow(non_upper_case_globals)]
    pub const TileproImm8X0 : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    #[allow(non_upper_case_globals)]
    pub const RiscvJal : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    #[allow(non_upper_case_globals)]
    pub const Or1KGotoffLo16 : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Got16Ha : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    #[allow(non_upper_case_globals)]
    pub const ArcS25WPcrel : Self = Self::_S390Pc16DblOrPowerpcGot16HaOrMipsShift6OrTilegxGlobDatOrX8664Dtpoff64OrSparcPc22OrArmTlsDtpmod32OrI386TlsLeOrM68KPlt16OOrAlphaGprelhighOrPpcGot16HaOrCkcoreAddrgotOrCris32GotrelOrMn10300Got32OrMicroblazeJumpSlotOrNios2GnuVtentryOrTileproImm8X0OrRiscvJalOrOr1KGotoffLo16OrPpc64Got16HaOrArcS25WPcrel;
    #[allow(non_upper_case_globals)]
    pub const S390Plt16Dbl : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    #[allow(non_upper_case_globals)]
    pub const PpcPltrel24 : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    #[allow(non_upper_case_globals)]
    pub const Mips64 : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    #[allow(non_upper_case_globals)]
    pub const TilegxJmpSlot : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    #[allow(non_upper_case_globals)]
    pub const X8664Tpoff64 : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    #[allow(non_upper_case_globals)]
    pub const SparcWplt30 : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    #[allow(non_upper_case_globals)]
    pub const ArmTlsDtpoff32 : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    #[allow(non_upper_case_globals)]
    pub const I386TlsGd : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    /// 8 bit PLT offset
    #[allow(non_upper_case_globals)]
    pub const M68KPlt8O : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    /// Left 21 bits of rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscDprel21L : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    /// GP relative 32 bit, low 16 bits
    #[allow(non_upper_case_globals)]
    pub const AlphaGprellow : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    /// PLT entry in GLOB_DAT (GOT + G)
    #[allow(non_upper_case_globals)]
    pub const CkcoreAddrplt : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    #[allow(non_upper_case_globals)]
    pub const Cris32PltGotrel : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    /// 24-bit offset to GOT entry.
    #[allow(non_upper_case_globals)]
    pub const Mn10300Got24 : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    /// Create GOT entry.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeGlobDat : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    /// Unconditional branch.
    #[allow(non_upper_case_globals)]
    pub const Nios2Ujmp : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    /// Y0 pipe 8-bit
    #[allow(non_upper_case_globals)]
    pub const TileproImm8Y0 : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    #[allow(non_upper_case_globals)]
    pub const RiscvCall : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    #[allow(non_upper_case_globals)]
    pub const Or1KCopy : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    #[allow(non_upper_case_globals)]
    pub const ArcSda32 : Self = Self::_S390Plt16DblOrPpcPltrel24OrMips64OrTilegxJmpSlotOrX8664Tpoff64OrSparcWplt30OrArmTlsDtpoff32OrI386TlsGdOrM68KPlt8OOrPariscDprel21LOrAlphaGprellowOrCkcoreAddrpltOrCris32PltGotrelOrMn10300Got24OrMicroblazeGlobDatOrNios2UjmpOrTileproImm8Y0OrRiscvCallOrOr1KCopyOrArcSda32;
    #[allow(non_upper_case_globals)]
    pub const S390Plt32Dbl : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGlobDat : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    #[allow(non_upper_case_globals)]
    pub const MipsGotPage : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    #[allow(non_upper_case_globals)]
    pub const TilegxBroffX1 : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    #[allow(non_upper_case_globals)]
    pub const SparcGlobDat : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    #[allow(non_upper_case_globals)]
    pub const ArmCopy : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    /// Create GOT entry
    #[allow(non_upper_case_globals)]
    pub const M68KGlobDat : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    #[allow(non_upper_case_globals)]
    pub const I38616 : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    #[allow(non_upper_case_globals)]
    pub const PpcGlobDat : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    /// disp ((S + A - P) >> 1) & 0xffff
    #[allow(non_upper_case_globals)]
    pub const CkcorePcrelImm16By2 : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    #[allow(non_upper_case_globals)]
    pub const CrisNum : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    #[allow(non_upper_case_globals)]
    pub const X8664Tlsld : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    /// Copy symbol at runtime.
    #[allow(non_upper_case_globals)]
    pub const Mn10300Copy : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    /// 32 bit offset to GOT.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeGotoff32 : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    /// Indirect call through register.
    #[allow(non_upper_case_globals)]
    pub const Nios2Callr : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    /// Y1 pipe 8-bit
    #[allow(non_upper_case_globals)]
    pub const TileproImm8Y1 : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    #[allow(non_upper_case_globals)]
    pub const RiscvGotHi20 : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    #[allow(non_upper_case_globals)]
    pub const Nds3232Rela : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    #[allow(non_upper_case_globals)]
    pub const Or1KJmpSlot : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    #[allow(non_upper_case_globals)]
    pub const Ppc64GlobDat : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    #[allow(non_upper_case_globals)]
    pub const ArcSdaLdst1 : Self = Self::_S390Plt32DblOrPowerpcGlobDatOrMipsGotPageOrTilegxBroffX1OrSparcGlobDatOrArmCopyOrM68KGlobDatOrI38616OrPpcGlobDatOrCkcorePcrelImm16By2OrCrisNumOrX8664TlsldOrMn10300CopyOrMicroblazeGotoff32OrNios2CallrOrTileproImm8Y1OrRiscvGotHi20OrNds3232RelaOrOr1KJmpSlotOrPpc64GlobDatOrArcSdaLdst1;
    #[allow(non_upper_case_globals)]
    pub const S390Gotpcdbl : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    #[allow(non_upper_case_globals)]
    pub const PowerpcJmpSlot : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    #[allow(non_upper_case_globals)]
    pub const MipsGotOfst : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    #[allow(non_upper_case_globals)]
    pub const TilegxJumpoffX1 : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    #[allow(non_upper_case_globals)]
    pub const SparcJmpSlot : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    #[allow(non_upper_case_globals)]
    pub const ArmGlobDat : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    #[allow(non_upper_case_globals)]
    pub const I386Pc16 : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    /// Create PLT entry
    #[allow(non_upper_case_globals)]
    pub const M68KJmpSlot : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    #[allow(non_upper_case_globals)]
    pub const PpcJmpSlot : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    /// disp ((S + A - P) >> 2) & 0xffff
    #[allow(non_upper_case_globals)]
    pub const CkcorePcrelImm16By4 : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    /// Offset in TLS block
    #[allow(non_upper_case_globals)]
    pub const X8664Dtpoff32 : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    /// Create GOT entry.
    #[allow(non_upper_case_globals)]
    pub const Mn10300GlobDat : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    /// Runtime copy.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeCopy : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    #[allow(non_upper_case_globals)]
    pub const Nios2Align : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    /// X1 pipe mtspr
    #[allow(non_upper_case_globals)]
    pub const TileproMtImm15X1 : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    #[allow(non_upper_case_globals)]
    pub const RiscvTlsGotHi20 : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    #[allow(non_upper_case_globals)]
    pub const Or1KRelative : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    #[allow(non_upper_case_globals)]
    pub const Ppc64JmpSlot : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    #[allow(non_upper_case_globals)]
    pub const ArcSdaLdst2 : Self = Self::_S390GotpcdblOrPowerpcJmpSlotOrMipsGotOfstOrTilegxJumpoffX1OrSparcJmpSlotOrArmGlobDatOrI386Pc16OrM68KJmpSlotOrPpcJmpSlotOrCkcorePcrelImm16By4OrX8664Dtpoff32OrMn10300GlobDatOrMicroblazeCopyOrNios2AlignOrTileproMtImm15X1OrRiscvTlsGotHi20OrOr1KRelativeOrPpc64JmpSlotOrArcSdaLdst2;
    #[allow(non_upper_case_globals)]
    pub const S39064 : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    #[allow(non_upper_case_globals)]
    pub const PowerpcRelative : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    #[allow(non_upper_case_globals)]
    pub const MipsGotHi16 : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    #[allow(non_upper_case_globals)]
    pub const TilegxJumpoffX1Plt : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    #[allow(non_upper_case_globals)]
    pub const X8664Gottpoff : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    #[allow(non_upper_case_globals)]
    pub const SparcRelative : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    #[allow(non_upper_case_globals)]
    pub const ArmJumpSlot : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    #[allow(non_upper_case_globals)]
    pub const I3868 : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    /// Adjust by program base
    #[allow(non_upper_case_globals)]
    pub const M68KRelative : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    /// Right 14 bits of rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscDprel14R : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    #[allow(non_upper_case_globals)]
    pub const PpcRelative : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    /// disp ((S + A - P) >> 1) & 0x3ff
    #[allow(non_upper_case_globals)]
    pub const CkcorePcrelImm10By2 : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    /// Create PLT entry.
    #[allow(non_upper_case_globals)]
    pub const Mn10300JmpSlot : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    /// TLS Reloc.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeTls : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    /// 16 bit GOT entry.
    #[allow(non_upper_case_globals)]
    pub const Nios2Got16 : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    /// X1 pipe mfspr
    #[allow(non_upper_case_globals)]
    pub const TileproMfImm15X1 : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    #[allow(non_upper_case_globals)]
    pub const RiscvTlsGdHi20 : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    #[allow(non_upper_case_globals)]
    pub const Or1KTlsGdHi16 : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Relative : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    #[allow(non_upper_case_globals)]
    pub const ArcSda16Ld : Self = Self::_S39064OrPowerpcRelativeOrMipsGotHi16OrTilegxJumpoffX1PltOrX8664GottpoffOrSparcRelativeOrArmJumpSlotOrI3868OrM68KRelativeOrPariscDprel14ROrPpcRelativeOrCkcorePcrelImm10By2OrMn10300JmpSlotOrMicroblazeTlsOrNios2Got16OrTileproMfImm15X1OrRiscvTlsGdHi20OrOr1KTlsGdHi16OrPpc64RelativeOrArcSda16Ld;
    #[allow(non_upper_case_globals)]
    pub const S390Pc64 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrX8664Tpoff32OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    #[allow(non_upper_case_globals)]
    pub const PpcLocal24Pc : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrX8664Tpoff32OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    #[allow(non_upper_case_globals)]
    pub const MipsGotLo16 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrX8664Tpoff32OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm8X0 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrX8664Tpoff32OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    #[allow(non_upper_case_globals)]
    pub const SparcUa32 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrX8664Tpoff32OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    #[allow(non_upper_case_globals)]
    pub const ArmRelative : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrX8664Tpoff32OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    #[allow(non_upper_case_globals)]
    pub const I386Pc8 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrX8664Tpoff32OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    /// disp ((S + A - P) >> 2) & 0x3ff
    #[allow(non_upper_case_globals)]
    pub const CkcorePcrelImm10By4 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrX8664Tpoff32OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    /// Offset in initial TLS block
    #[allow(non_upper_case_globals)]
    pub const X8664Tpoff32 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrX8664Tpoff32OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    /// Adjust by program base.
    #[allow(non_upper_case_globals)]
    pub const Mn10300Relative : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrX8664Tpoff32OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    /// TLS General Dynamic.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeTlsgd : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrX8664Tpoff32OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    /// 16 bit GOT entry for function.
    #[allow(non_upper_case_globals)]
    pub const Nios2Call16 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrX8664Tpoff32OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    /// X0 pipe 16-bit
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrX8664Tpoff32OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    #[allow(non_upper_case_globals)]
    pub const RiscvPcrelHi20 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrX8664Tpoff32OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    #[allow(non_upper_case_globals)]
    pub const Or1KTlsGdLo16 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrX8664Tpoff32OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    #[allow(non_upper_case_globals)]
    pub const ArcSda16Ld1 : Self = Self::_S390Pc64OrPpcLocal24PcOrMipsGotLo16OrTilegxImm8X0OrSparcUa32OrArmRelativeOrI386Pc8OrCkcorePcrelImm10By4OrX8664Tpoff32OrMn10300RelativeOrMicroblazeTlsgdOrNios2Call16OrTileproImm16X0OrRiscvPcrelHi20OrOr1KTlsGdLo16OrArcSda16Ld1;
    #[allow(non_upper_case_globals)]
    pub const S390Got64 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    #[allow(non_upper_case_globals)]
    pub const PowerpcUaddr32 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    #[allow(non_upper_case_globals)]
    pub const MipsSub : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm8Y0 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    #[allow(non_upper_case_globals)]
    pub const X8664Pc64 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    #[allow(non_upper_case_globals)]
    pub const SparcPlt32 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    #[allow(non_upper_case_globals)]
    pub const ArmGotoff32 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    #[allow(non_upper_case_globals)]
    pub const I386TlsGd32 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    /// Copy symbol at runtime
    #[allow(non_upper_case_globals)]
    pub const AlphaCopy : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    #[allow(non_upper_case_globals)]
    pub const PpcUaddr32 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    /// 32 bit offset to GOT
    #[allow(non_upper_case_globals)]
    pub const ArmGotoff : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    /// high & low 16 bit ADDR
    #[allow(non_upper_case_globals)]
    pub const CkcoreAddrHi16 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    /// 32-bit offset for global dynamic.
    #[allow(non_upper_case_globals)]
    pub const Mn10300TlsGd : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    /// TLS Local Dynamic.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeTlsld : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    /// %lo of offset to GOT pointer.
    #[allow(non_upper_case_globals)]
    pub const Nios2GotoffLo : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    /// X1 pipe 16-bit
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    #[allow(non_upper_case_globals)]
    pub const RiscvPcrelLo12I : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    #[allow(non_upper_case_globals)]
    pub const Or1KTlsLdmHi16 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Uaddr32 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    #[allow(non_upper_case_globals)]
    pub const ArcSda16Ld2 : Self = Self::_S390Got64OrPowerpcUaddr32OrMipsSubOrTilegxImm8Y0OrX8664Pc64OrSparcPlt32OrArmGotoff32OrI386TlsGd32OrAlphaCopyOrPpcUaddr32OrArmGotoffOrCkcoreAddrHi16OrMn10300TlsGdOrMicroblazeTlsldOrNios2GotoffLoOrTileproImm16X1OrRiscvPcrelLo12IOrOr1KTlsLdmHi16OrPpc64Uaddr32OrArcSda16Ld2;
    #[allow(non_upper_case_globals)]
    pub const S390Plt64 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    #[allow(non_upper_case_globals)]
    pub const PowerpcUaddr16 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    #[allow(non_upper_case_globals)]
    pub const MipsInsertA : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm8X1 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    #[allow(non_upper_case_globals)]
    pub const X8664Gotoff64 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    #[allow(non_upper_case_globals)]
    pub const SparcHiplt22 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    #[allow(non_upper_case_globals)]
    pub const ArmBasePrel : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    #[allow(non_upper_case_globals)]
    pub const I386TlsGdPush : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    /// 32 bit GOT offset for GD
    #[allow(non_upper_case_globals)]
    pub const M68KTlsGd32 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    /// Create GOT entry
    #[allow(non_upper_case_globals)]
    pub const AlphaGlobDat : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    #[allow(non_upper_case_globals)]
    pub const PpcUaddr16 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    /// 32 bit PC relative offset to GOT
    #[allow(non_upper_case_globals)]
    pub const ArmGotpc : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    /// (S + A) & 0xffff
    #[allow(non_upper_case_globals)]
    pub const CkcoreAddrLo16 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    #[allow(non_upper_case_globals)]
    pub const ShSwitch16 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    /// 32-bit offset for local dynamic.
    #[allow(non_upper_case_globals)]
    pub const Mn10300TlsLd : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    /// TLS Module ID.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeTlsdtpmod32 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    /// %hiadj of offset to GOT pointer.
    #[allow(non_upper_case_globals)]
    pub const Nios2GotoffHa : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    /// X0 pipe low 16-bit
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0Lo : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    #[allow(non_upper_case_globals)]
    pub const RiscvPcrelLo12S : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    #[allow(non_upper_case_globals)]
    pub const Or1KTlsLdmLo16 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Uaddr16 : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    #[allow(non_upper_case_globals)]
    pub const ArcS13Pcrel : Self = Self::_S390Plt64OrPowerpcUaddr16OrMipsInsertAOrTilegxImm8X1OrX8664Gotoff64OrSparcHiplt22OrArmBasePrelOrI386TlsGdPushOrM68KTlsGd32OrAlphaGlobDatOrPpcUaddr16OrArmGotpcOrCkcoreAddrLo16OrShSwitch16OrMn10300TlsLdOrMicroblazeTlsdtpmod32OrNios2GotoffHaOrTileproImm16X0LoOrRiscvPcrelLo12SOrOr1KTlsLdmLo16OrPpc64Uaddr16OrArcS13Pcrel;
    #[allow(non_upper_case_globals)]
    pub const S390Gotent : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    #[allow(non_upper_case_globals)]
    pub const PowerpcRel32 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    #[allow(non_upper_case_globals)]
    pub const MipsInsertB : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm8Y1 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    #[allow(non_upper_case_globals)]
    pub const X8664Gotpc32 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    #[allow(non_upper_case_globals)]
    pub const SparcLoplt10 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    #[allow(non_upper_case_globals)]
    pub const ArmGotBrel : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    #[allow(non_upper_case_globals)]
    pub const I386TlsGdCall : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    /// 16 bit GOT offset for GD
    #[allow(non_upper_case_globals)]
    pub const M68KTlsGd16 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    /// GP-relative, left 21 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscGprel21L : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    /// Create PLT entry
    #[allow(non_upper_case_globals)]
    pub const AlphaJmpSlot : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    #[allow(non_upper_case_globals)]
    pub const PpcRel32 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    /// 32 bit GOT entry
    #[allow(non_upper_case_globals)]
    pub const ArmGot32 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    /// high & low 16 bit GOTPC
    #[allow(non_upper_case_globals)]
    pub const CkcoreGotpcHi16 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    #[allow(non_upper_case_globals)]
    pub const ShSwitch32 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    /// Module-relative offset.
    #[allow(non_upper_case_globals)]
    pub const Mn10300TlsLdo : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    /// TLS Offset Within TLS Block.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeTlsdtprel32 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    /// %lo of PC relative offset.
    #[allow(non_upper_case_globals)]
    pub const Nios2PcrelLo : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    /// X1 pipe low 16-bit
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1Lo : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    #[allow(non_upper_case_globals)]
    pub const RiscvHi20 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    #[allow(non_upper_case_globals)]
    pub const Or1KTlsLdoHi16 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel32 : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    #[allow(non_upper_case_globals)]
    pub const ArcW : Self = Self::_S390GotentOrPowerpcRel32OrMipsInsertBOrTilegxImm8Y1OrX8664Gotpc32OrSparcLoplt10OrArmGotBrelOrI386TlsGdCallOrM68KTlsGd16OrPariscGprel21LOrAlphaJmpSlotOrPpcRel32OrArmGot32OrCkcoreGotpcHi16OrShSwitch32OrMn10300TlsLdoOrMicroblazeTlsdtprel32OrNios2PcrelLoOrTileproImm16X1LoOrRiscvHi20OrOr1KTlsLdoHi16OrPpc64Rel32OrArcW;
    #[allow(non_upper_case_globals)]
    pub const S390Gotoff64 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    #[allow(non_upper_case_globals)]
    pub const PowerpcPltrel32 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    #[allow(non_upper_case_globals)]
    pub const MipsHigher : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    #[allow(non_upper_case_globals)]
    pub const TilegxMtImm14X1 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    #[allow(non_upper_case_globals)]
    pub const X8664Gotpcrel64 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    #[allow(non_upper_case_globals)]
    pub const SparcPcplt22 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    #[allow(non_upper_case_globals)]
    pub const ArmCall : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    #[allow(non_upper_case_globals)]
    pub const I386TlsLdm32 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    /// 32 bit GOT offset for LDM
    #[allow(non_upper_case_globals)]
    pub const M68KTlsLdm32 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    #[allow(non_upper_case_globals)]
    pub const AlphaTlsGdHi : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    #[allow(non_upper_case_globals)]
    pub const PpcPltrel32 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    /// high & low 16 bit GOTOFF
    #[allow(non_upper_case_globals)]
    pub const CkcoreGotoffHi16 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    #[allow(non_upper_case_globals)]
    pub const ShCount : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    #[allow(non_upper_case_globals)]
    pub const Mn10300TlsIe : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    /// TLS Offset From Thread Pointer.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeTlsgottprel32 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    /// 16 bit GOT offset for TLS GD.
    #[allow(non_upper_case_globals)]
    pub const Nios2TlsGd16 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    /// X1 pipe high 16-bit
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1Hi : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    #[allow(non_upper_case_globals)]
    pub const RiscvLo12S : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    #[allow(non_upper_case_globals)]
    pub const Or1KTlsIeHi16 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Pltrel32 : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    #[allow(non_upper_case_globals)]
    pub const ArcN32Me : Self = Self::_S390Gotoff64OrPowerpcPltrel32OrMipsHigherOrTilegxMtImm14X1OrX8664Gotpcrel64OrSparcPcplt22OrArmCallOrI386TlsLdm32OrM68KTlsLdm32OrAlphaTlsGdHiOrPpcPltrel32OrCkcoreGotoffHi16OrShCountOrMn10300TlsIeOrMicroblazeTlsgottprel32OrNios2TlsGd16OrTileproImm16X1HiOrRiscvLo12SOrOr1KTlsIeHi16OrPpc64Pltrel32OrArcN32Me;
    #[allow(non_upper_case_globals)]
    pub const S390Gotplt12 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    #[allow(non_upper_case_globals)]
    pub const PowerpcPlt16Lo : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    #[allow(non_upper_case_globals)]
    pub const MipsHighest : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    #[allow(non_upper_case_globals)]
    pub const TilegxMfImm14X1 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    #[allow(non_upper_case_globals)]
    pub const X8664Gotpc64 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    #[allow(non_upper_case_globals)]
    pub const SparcPcplt10 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    #[allow(non_upper_case_globals)]
    pub const ArmJump24 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    #[allow(non_upper_case_globals)]
    pub const I386TlsLdmPush : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    /// 16 bit GOT offset for LDM
    #[allow(non_upper_case_globals)]
    pub const M68KTlsLdm16 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    #[allow(non_upper_case_globals)]
    pub const AlphaTlsgd : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    #[allow(non_upper_case_globals)]
    pub const PpcPlt16Lo : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    /// (S + A - GOT) & 0xffff
    #[allow(non_upper_case_globals)]
    pub const CkcoreGotoffLo16 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    #[allow(non_upper_case_globals)]
    pub const ShAlign : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    #[allow(non_upper_case_globals)]
    pub const Mn10300TlsLe : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    /// TLS Offset From Thread Pointer.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeTlstprel32 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    /// 16 bit GOT offset for TLS LDM.
    #[allow(non_upper_case_globals)]
    pub const Nios2TlsLdm16 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    /// X0 pipe high 16-bit, adjusted
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0Ha : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    #[allow(non_upper_case_globals)]
    pub const RiscvTprelHi20 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    #[allow(non_upper_case_globals)]
    pub const Or1KTlsIeLo16 : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Plt16Lo : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    #[allow(non_upper_case_globals)]
    pub const ArcSectoffMe : Self = Self::_S390Gotplt12OrPowerpcPlt16LoOrMipsHighestOrTilegxMfImm14X1OrX8664Gotpc64OrSparcPcplt10OrArmJump24OrI386TlsLdmPushOrM68KTlsLdm16OrAlphaTlsgdOrPpcPlt16LoOrCkcoreGotoffLo16OrShAlignOrMn10300TlsLeOrMicroblazeTlstprel32OrNios2TlsLdm16OrTileproImm16X0HaOrRiscvTprelHi20OrOr1KTlsIeLo16OrPpc64Plt16LoOrArcSectoffMe;
    #[allow(non_upper_case_globals)]
    pub const S390Gotplt16 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    #[allow(non_upper_case_globals)]
    pub const PowerpcPlt16Hi : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    #[allow(non_upper_case_globals)]
    pub const MipsCallHi16 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    #[allow(non_upper_case_globals)]
    pub const TilegxMmstartX0 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    #[allow(non_upper_case_globals)]
    pub const X8664Gotplt64 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    #[allow(non_upper_case_globals)]
    pub const Sparc10 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    #[allow(non_upper_case_globals)]
    pub const ArmThmJump24 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    #[allow(non_upper_case_globals)]
    pub const I386TlsLdmCall : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    /// 8 bit GOT offset for LDM
    #[allow(non_upper_case_globals)]
    pub const M68KTlsLdm8 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    /// GP-relative, right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscGprel14R : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    #[allow(non_upper_case_globals)]
    pub const AlphaTlsLdm : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    #[allow(non_upper_case_globals)]
    pub const PpcPlt16Hi : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    /// 12 bit disp GOT entry (G)
    #[allow(non_upper_case_globals)]
    pub const CkcoreGot12 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    #[allow(non_upper_case_globals)]
    pub const ShCode : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    /// ID of module containing symbol.
    #[allow(non_upper_case_globals)]
    pub const Mn10300TlsDtpmod : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    /// 16 bit module relative offset.
    #[allow(non_upper_case_globals)]
    pub const Nios2TlsLdo16 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    /// X1 pipe high 16-bit, adjusted
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1Ha : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    #[allow(non_upper_case_globals)]
    pub const RiscvTprelLo12I : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    #[allow(non_upper_case_globals)]
    pub const MetagGnuVtinherit : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    #[allow(non_upper_case_globals)]
    pub const Or1KTlsLeHi16 : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Plt16Hi : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    #[allow(non_upper_case_globals)]
    pub const ArcSda32Me : Self = Self::_S390Gotplt16OrPowerpcPlt16HiOrMipsCallHi16OrTilegxMmstartX0OrX8664Gotplt64OrSparc10OrArmThmJump24OrI386TlsLdmCallOrM68KTlsLdm8OrPariscGprel14ROrAlphaTlsLdmOrPpcPlt16HiOrCkcoreGot12OrShCodeOrMn10300TlsDtpmodOrNios2TlsLdo16OrTileproImm16X1HaOrRiscvTprelLo12IOrMetagGnuVtinheritOrOr1KTlsLeHi16OrPpc64Plt16HiOrArcSda32Me;
    #[allow(non_upper_case_globals)]
    pub const S390Gotplt32 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    #[allow(non_upper_case_globals)]
    pub const PowerpcPlt16Ha : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    #[allow(non_upper_case_globals)]
    pub const MipsCallLo16 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    #[allow(non_upper_case_globals)]
    pub const TilegxMmendX0 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    #[allow(non_upper_case_globals)]
    pub const X8664Pltoff64 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    #[allow(non_upper_case_globals)]
    pub const Sparc11 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    #[allow(non_upper_case_globals)]
    pub const ArmBaseAbs : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    #[allow(non_upper_case_globals)]
    pub const I386TlsLdmPop : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    /// 32 bit module-relative offset
    #[allow(non_upper_case_globals)]
    pub const M68KTlsLdo32 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    #[allow(non_upper_case_globals)]
    pub const AlphaDtpmod64 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    #[allow(non_upper_case_globals)]
    pub const PpcPlt16Ha : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    /// high & low 16 bit GOT
    #[allow(non_upper_case_globals)]
    pub const CkcoreGotHi16 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    #[allow(non_upper_case_globals)]
    pub const ShData : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    /// Offset in module TLS block.
    #[allow(non_upper_case_globals)]
    pub const Mn10300TlsDtpoff : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    /// 16 bit GOT offset for TLS IE.
    #[allow(non_upper_case_globals)]
    pub const Nios2TlsIe16 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    /// X0 pipe PC relative 16 bit
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0Pcrel : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    #[allow(non_upper_case_globals)]
    pub const RiscvTprelLo12S : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    #[allow(non_upper_case_globals)]
    pub const MetagGnuVtentry : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    #[allow(non_upper_case_globals)]
    pub const Or1KTlsLeLo16 : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Plt16Ha : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    #[allow(non_upper_case_globals)]
    pub const ArcWMe : Self = Self::_S390Gotplt32OrPowerpcPlt16HaOrMipsCallLo16OrTilegxMmendX0OrX8664Pltoff64OrSparc11OrArmBaseAbsOrI386TlsLdmPopOrM68KTlsLdo32OrAlphaDtpmod64OrPpcPlt16HaOrCkcoreGotHi16OrShDataOrMn10300TlsDtpoffOrNios2TlsIe16OrTileproImm16X0PcrelOrRiscvTprelLo12SOrMetagGnuVtentryOrOr1KTlsLeLo16OrPpc64Plt16HaOrArcWMe;
    #[allow(non_upper_case_globals)]
    pub const S390Gotplt64 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    #[allow(non_upper_case_globals)]
    pub const PpcSdarel16 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    #[allow(non_upper_case_globals)]
    pub const MipsScnDisp : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    #[allow(non_upper_case_globals)]
    pub const X8664Size32 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    #[allow(non_upper_case_globals)]
    pub const TilegxShamtX0 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    #[allow(non_upper_case_globals)]
    pub const Sparc64 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    #[allow(non_upper_case_globals)]
    pub const ArmAluPcrel70 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    #[allow(non_upper_case_globals)]
    pub const I386TlsLdo32 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    /// 16 bit module-relative offset
    #[allow(non_upper_case_globals)]
    pub const M68KTlsLdo16 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    #[allow(non_upper_case_globals)]
    pub const AlphaGotdtprel : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    /// (G & 0xffff)
    #[allow(non_upper_case_globals)]
    pub const CkcoreGotLo16 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    #[allow(non_upper_case_globals)]
    pub const ShLabel : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    /// Offset in static TLS block.
    #[allow(non_upper_case_globals)]
    pub const Mn10300TlsTpoff : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    /// 16 bit LE TP-relative offset.
    #[allow(non_upper_case_globals)]
    pub const Nios2TlsLe16 : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    /// X1 pipe PC relative 16 bit
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1Pcrel : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    #[allow(non_upper_case_globals)]
    pub const RiscvTprelAdd : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    #[allow(non_upper_case_globals)]
    pub const MetagHi16Gotoff : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    #[allow(non_upper_case_globals)]
    pub const Or1KTlsTpoff : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    #[allow(non_upper_case_globals)]
    pub const ArcH30Me : Self = Self::_S390Gotplt64OrPpcSdarel16OrMipsScnDispOrX8664Size32OrTilegxShamtX0OrSparc64OrArmAluPcrel70OrI386TlsLdo32OrM68KTlsLdo16OrAlphaGotdtprelOrCkcoreGotLo16OrShLabelOrMn10300TlsTpoffOrNios2TlsLe16OrTileproImm16X1PcrelOrRiscvTprelAddOrMetagHi16GotoffOrOr1KTlsTpoffOrArcH30Me;
    #[allow(non_upper_case_globals)]
    pub const S390Gotpltent : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const PowerpcSectoff : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const MipsRel16 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const X8664Size64 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const TilegxShamtX1 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const SparcOlo10 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const ArmAluPcrel158 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const I386TlsIe32 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    /// 8 bit module-relative offset
    #[allow(non_upper_case_globals)]
    pub const M68KTlsLdo8 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const AlphaDtprel64 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const PpcSectoff : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    /// 12 bit disp PLT entry (G)
    #[allow(non_upper_case_globals)]
    pub const CkcorePlt12 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const ShSwitch8 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const Mn10300SymDiff : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    /// Direct 16 bit.
    #[allow(non_upper_case_globals)]
    pub const M32R16Rela : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    /// Module number.
    #[allow(non_upper_case_globals)]
    pub const Nios2TlsDtpmod : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    /// X0 pipe PC relative low 16 bit
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0LoPcrel : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const RiscvAdd8 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const MetagLo16Gotoff : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const Or1KTlsDtpoff : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Sectoff : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    /// symbol + addend, add imm14
    #[allow(non_upper_case_globals)]
    pub const Ia64Imm14 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const ArcSectoffU8 : Self = Self::_S390GotpltentOrPowerpcSectoffOrMipsRel16OrX8664Size64OrTilegxShamtX1OrSparcOlo10OrArmAluPcrel158OrI386TlsIe32OrM68KTlsLdo8OrAlphaDtprel64OrPpcSectoffOrCkcorePlt12OrShSwitch8OrMn10300SymDiffOrM32R16RelaOrNios2TlsDtpmodOrTileproImm16X0LoPcrelOrRiscvAdd8OrMetagLo16GotoffOrOr1KTlsDtpoffOrPpc64SectoffOrIa64Imm14OrArcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const S390Pltoff16 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const PowerpcSectoffLo : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const MipsAddImmediate : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const TilegxShamtY0 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const X8664Gotpc32Tlsdesc : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const SparcHh22 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const ArmAluPcrel2315 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const I386TlsLe32 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    /// 32 bit GOT offset for IE
    #[allow(non_upper_case_globals)]
    pub const M68KTlsIe32 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    /// LT-relative, left 21 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoff21L : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const AlphaDtprelhi : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const PpcSectoffLo : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    /// high & low 16 bit PLT
    #[allow(non_upper_case_globals)]
    pub const CkcorePltHi16 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const ShGnuVtinherit : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const Mn10300Align : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    /// Direct 32 bit.
    #[allow(non_upper_case_globals)]
    pub const M32R32Rela : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    /// Module-relative offset.
    #[allow(non_upper_case_globals)]
    pub const Nios2TlsDtprel : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    /// X1 pipe PC relative low 16 bit
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1LoPcrel : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const RiscvAdd16 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const MetagGetsetGotoff : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const Or1KTlsDtpmod : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const Ppc64SectoffLo : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    /// symbol + addend, add imm22
    #[allow(non_upper_case_globals)]
    pub const Ia64Imm22 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const ArcSectoffS9 : Self = Self::_S390Pltoff16OrPowerpcSectoffLoOrMipsAddImmediateOrTilegxShamtY0OrX8664Gotpc32TlsdescOrSparcHh22OrArmAluPcrel2315OrI386TlsLe32OrM68KTlsIe32OrPariscLtoff21LOrAlphaDtprelhiOrPpcSectoffLoOrCkcorePltHi16OrShGnuVtinheritOrMn10300AlignOrM32R32RelaOrNios2TlsDtprelOrTileproImm16X1LoPcrelOrRiscvAdd16OrMetagGetsetGotoffOrOr1KTlsDtpmodOrPpc64SectoffLoOrIa64Imm22OrArcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const S390Pltoff32 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const PowerpcSectoffHi : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const MipsPjump : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const TilegxShamtY1 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const X8664TlsdescCall : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const SparcHm10 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const ArmLdrSbrel110Nc : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const I386TlsDtpmod32 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    /// 16 bit GOT offset for IE
    #[allow(non_upper_case_globals)]
    pub const M68KTlsIe16 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const AlphaDtprello : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const PpcSectoffHi : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    /// Deprecated, prog. base relative.
    #[allow(non_upper_case_globals)]
    pub const ArmLdrSbrel110 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    /// G & 0xffff
    #[allow(non_upper_case_globals)]
    pub const CkcorePltLo16 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const ShGnuVtentry : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const Mn10300Num : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    /// Direct 24 bit.
    #[allow(non_upper_case_globals)]
    pub const M32R24Rela : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    /// TP-relative offset.
    #[allow(non_upper_case_globals)]
    pub const Nios2TlsTprel : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    /// X0 pipe PC relative high 16 bit
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0HiPcrel : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const RiscvAdd32 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const MetagGetsetGot : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const Ppc64SectoffHi : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    /// symbol + addend, mov imm64
    #[allow(non_upper_case_globals)]
    pub const Ia64Imm64 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const AcSectoffU8 : Self = Self::_S390Pltoff32OrPowerpcSectoffHiOrMipsPjumpOrTilegxShamtY1OrX8664TlsdescCallOrSparcHm10OrArmLdrSbrel110NcOrI386TlsDtpmod32OrM68KTlsIe16OrAlphaDtprelloOrPpcSectoffHiOrArmLdrSbrel110OrCkcorePltLo16OrShGnuVtentryOrMn10300NumOrM32R24RelaOrNios2TlsTprelOrTileproImm16X0HiPcrelOrRiscvAdd32OrMetagGetsetGotOrPpc64SectoffHiOrIa64Imm64OrAcSectoffU8;
    #[allow(non_upper_case_globals)]
    pub const S390Pltoff64 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    #[allow(non_upper_case_globals)]
    pub const PowerpcSectoffHa : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    #[allow(non_upper_case_globals)]
    pub const MipsRelgot : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw0 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    #[allow(non_upper_case_globals)]
    pub const X8664Tlsdesc : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    #[allow(non_upper_case_globals)]
    pub const SparcLm22 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    #[allow(non_upper_case_globals)]
    pub const ArmAluSbrel1912Nc : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    #[allow(non_upper_case_globals)]
    pub const I386TlsDtpoff32 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    /// 8 bit GOT offset for IE
    #[allow(non_upper_case_globals)]
    pub const M68KTlsIe8 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    #[allow(non_upper_case_globals)]
    pub const AlphaDtprel16 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    #[allow(non_upper_case_globals)]
    pub const PpcSectoffHa : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    /// Deprecated, prog. base relative.
    #[allow(non_upper_case_globals)]
    pub const ArmAluSbrel1912 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    /// high & low 16 bit ADDRGOT
    #[allow(non_upper_case_globals)]
    pub const CkcoreAddrgotHi16 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    /// PC relative 10 bit shifted.
    #[allow(non_upper_case_globals)]
    pub const M32R10PcrelRela : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    /// Copy symbol at runtime.
    #[allow(non_upper_case_globals)]
    pub const Nios2Copy : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    /// X1 pipe PC relative high 16 bit
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1HiPcrel : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    #[allow(non_upper_case_globals)]
    pub const RiscvAdd64 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    #[allow(non_upper_case_globals)]
    pub const MetagHi16Gotpc : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    #[allow(non_upper_case_globals)]
    pub const Ppc64SectoffHa : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    /// symbol + addend, data4 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Dir32Msb : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    #[allow(non_upper_case_globals)]
    pub const AcSectoffU81 : Self = Self::_S390Pltoff64OrPowerpcSectoffHaOrMipsRelgotOrTilegxImm16X0Hw0OrX8664TlsdescOrSparcLm22OrArmAluSbrel1912NcOrI386TlsDtpoff32OrM68KTlsIe8OrAlphaDtprel16OrPpcSectoffHaOrArmAluSbrel1912OrCkcoreAddrgotHi16OrM32R10PcrelRelaOrNios2CopyOrTileproImm16X1HiPcrelOrRiscvAdd64OrMetagHi16GotpcOrPpc64SectoffHaOrIa64Dir32MsbOrAcSectoffU81;
    #[allow(non_upper_case_globals)]
    pub const S390TlsGdcall : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr64 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw1 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const X8664Relative64 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const SparcPcHm10 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const ArmTarget1 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const M68KTlsLe16 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    /// 32-bit symbol size
    #[allow(non_upper_case_globals)]
    pub const I386Size32 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    /// Module number 32 bit
    #[allow(non_upper_case_globals)]
    pub const MipsTlsDtpmod32 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    /// LT-relative, right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoff14R : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const AlphaTprel64 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    /// high & low 16 bit ADDRPLT
    #[allow(non_upper_case_globals)]
    pub const CkcoreAddrpltHi16 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    /// PC relative 26 bit shifted.
    #[allow(non_upper_case_globals)]
    pub const M32R26PcrelRela : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    /// Create PLT entry.
    #[allow(non_upper_case_globals)]
    pub const Nios2JumpSlot : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    /// X1 pipe PC relative ha() 16 bit
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1HaPcrel : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const RiscvSub16 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const MetagHi16Plt : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    /// symbol + addend, data8 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Dir64Msb : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const AcSectoffS9 : Self = Self::_S390TlsGdcallOrPpc64Addr64OrTilegxImm16X0Hw1OrX8664Relative64OrSparcPcHm10OrArmTarget1OrM68KTlsLe16OrI386Size32OrMipsTlsDtpmod32OrPariscLtoff14ROrAlphaTprel64OrCkcoreAddrpltHi16OrM32R26PcrelRelaOrNios2JumpSlotOrTileproImm16X1HaPcrelOrRiscvSub16OrMetagHi16PltOrIa64Dir64MsbOrAcSectoffS9;
    #[allow(non_upper_case_globals)]
    pub const S390TlsLdcall : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr16Higher : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    #[allow(non_upper_case_globals)]
    pub const MipsTlsDtprel32 : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw1 : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    #[allow(non_upper_case_globals)]
    pub const X8664Pc32Bnd : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    #[allow(non_upper_case_globals)]
    pub const SparcPcLm22 : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    #[allow(non_upper_case_globals)]
    pub const I386TlsGotdesc : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    #[allow(non_upper_case_globals)]
    pub const M68KTlsLe8 : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    #[allow(non_upper_case_globals)]
    pub const AlphaTprelhi : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    /// Program base relative.
    #[allow(non_upper_case_globals)]
    pub const ArmSbrel31 : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    /// (GOT+G*4) & 0xffff
    #[allow(non_upper_case_globals)]
    pub const CkcoreAddrpltLo16 : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    /// High 16 bit with unsigned low
    #[allow(non_upper_case_globals)]
    pub const M32RHi16UloRela : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    /// Adjust by program base.
    #[allow(non_upper_case_globals)]
    pub const Nios2Relative : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    /// X0 pipe 16-bit GOT offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0Got : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    #[allow(non_upper_case_globals)]
    pub const RiscvSub32 : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    #[allow(non_upper_case_globals)]
    pub const MetagLo16Plt : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    #[allow(non_upper_case_globals)]
    pub const Nds32Copy : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    /// symbol + addend, data8 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Dir64Lsb : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    #[allow(non_upper_case_globals)]
    pub const AcSectoffS91 : Self = Self::_S390TlsLdcallOrPpc64Addr16HigherOrMipsTlsDtprel32OrTilegxImm16X1Hw1OrX8664Pc32BndOrSparcPcLm22OrI386TlsGotdescOrM68KTlsLe8OrAlphaTprelhiOrArmSbrel31OrCkcoreAddrpltLo16OrM32RHi16UloRelaOrNios2RelativeOrTileproImm16X0GotOrRiscvSub32OrMetagLo16PltOrNds32CopyOrIa64Dir64LsbOrAcSectoffS91;
    #[allow(non_upper_case_globals)]
    pub const S390TlsGd32 : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr16Highera : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    #[allow(non_upper_case_globals)]
    pub const MipsTlsDtpmod64 : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw2 : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    #[allow(non_upper_case_globals)]
    pub const X8664Plt32Bnd : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    #[allow(non_upper_case_globals)]
    pub const SparcWdisp16 : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    #[allow(non_upper_case_globals)]
    pub const ArmV4Bx : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    #[allow(non_upper_case_globals)]
    pub const I386TlsDescCall : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    /// 32 bit module number
    #[allow(non_upper_case_globals)]
    pub const M68KTlsDtpmod32 : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    #[allow(non_upper_case_globals)]
    pub const AlphaTprello : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    /// disp ((S+A-P) >>1) & x3ffffff
    #[allow(non_upper_case_globals)]
    pub const CkcorePcrelJsrImm26By2 : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    /// High 16 bit with signed low
    #[allow(non_upper_case_globals)]
    pub const M32RHi16SloRela : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    /// 16 bit offset to GOT pointer.
    #[allow(non_upper_case_globals)]
    pub const Nios2Gotoff : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    /// X1 pipe 16-bit GOT offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1Got : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    #[allow(non_upper_case_globals)]
    pub const RiscvSub64 : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    #[allow(non_upper_case_globals)]
    pub const MetagRelbranchPlt : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    #[allow(non_upper_case_globals)]
    pub const Nds32GlobDat : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    #[allow(non_upper_case_globals)]
    pub const AcSectoffS92 : Self = Self::_S390TlsGd32OrPpc64Addr16HigheraOrMipsTlsDtpmod64OrTilegxImm16X0Hw2OrX8664Plt32BndOrSparcWdisp16OrArmV4BxOrI386TlsDescCallOrM68KTlsDtpmod32OrAlphaTprelloOrCkcorePcrelJsrImm26By2OrM32RHi16SloRelaOrNios2GotoffOrTileproImm16X1GotOrRiscvSub64OrMetagRelbranchPltOrNds32GlobDatOrAcSectoffS92;
    #[allow(non_upper_case_globals)]
    pub const S390TlsGd64 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr16Highest : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    #[allow(non_upper_case_globals)]
    pub const MipsTlsDtprel64 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw2 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    #[allow(non_upper_case_globals)]
    pub const X8664Gotpcrelx : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    #[allow(non_upper_case_globals)]
    pub const SparcWdisp19 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    #[allow(non_upper_case_globals)]
    pub const ArmTarget2 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    #[allow(non_upper_case_globals)]
    pub const I386TlsDesc : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    /// 32 bit module-relative offset
    #[allow(non_upper_case_globals)]
    pub const M68KTlsDtprel32 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    /// 32 bits section rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscSecrel32 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    #[allow(non_upper_case_globals)]
    pub const AlphaTprel16 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    /// (S+A-BTEXT) & 0xffff
    #[allow(non_upper_case_globals)]
    pub const CkcoreToffsetLo16 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    /// Low 16 bit
    #[allow(non_upper_case_globals)]
    pub const M32RLo16Rela : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    /// Direct call in .noat section.
    #[allow(non_upper_case_globals)]
    pub const Nios2Call26Noat : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    /// X0 pipe low 16-bit GOT offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0GotLo : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    #[allow(non_upper_case_globals)]
    pub const RiscvGnuVtinherit : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    #[allow(non_upper_case_globals)]
    pub const MetagGotoff : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    #[allow(non_upper_case_globals)]
    pub const Nds32JmpSlot : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    #[allow(non_upper_case_globals)]
    pub const ArcSectoffMe1 : Self = Self::_S390TlsGd64OrPpc64Addr16HighestOrMipsTlsDtprel64OrTilegxImm16X1Hw2OrX8664GotpcrelxOrSparcWdisp19OrArmTarget2OrI386TlsDescOrM68KTlsDtprel32OrPariscSecrel32OrAlphaTprel16OrCkcoreToffsetLo16OrM32RLo16RelaOrNios2Call26NoatOrTileproImm16X0GotLoOrRiscvGnuVtinheritOrMetagGotoffOrNds32JmpSlotOrArcSectoffMe1;
    #[allow(non_upper_case_globals)]
    pub const S390TlsGotie12 : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr16Highesta : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    #[allow(non_upper_case_globals)]
    pub const MipsTlsGd : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw3 : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    #[allow(non_upper_case_globals)]
    pub const SparcGlobJmp : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    #[allow(non_upper_case_globals)]
    pub const ArmPrel31 : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    /// 32 bit TP-relative offset
    #[allow(non_upper_case_globals)]
    pub const M68KTlsTprel32 : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    /// Adjust indirectly by program base
    #[allow(non_upper_case_globals)]
    pub const I386Irelative : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    /// (S+A-BTEXT) & 0xffff
    #[allow(non_upper_case_globals)]
    pub const CkcoreDoffsetLo16 : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    #[allow(non_upper_case_globals)]
    pub const X8664RexGotpcrelx : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    /// 16 bit offset in SDA
    #[allow(non_upper_case_globals)]
    pub const M32RSda16Rela : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    /// %lo() of GOT entry.
    #[allow(non_upper_case_globals)]
    pub const Nios2GotLo : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    /// X1 pipe low 16-bit GOT offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1GotLo : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    #[allow(non_upper_case_globals)]
    pub const RiscvGnuVtentry : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    #[allow(non_upper_case_globals)]
    pub const MetagPlt : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    #[allow(non_upper_case_globals)]
    pub const Nds32Relative : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    /// @gprel(sym + add), add imm22
    #[allow(non_upper_case_globals)]
    pub const Ia64Gprel22 : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    #[allow(non_upper_case_globals)]
    pub const ArcSectoffMe2 : Self = Self::_S390TlsGotie12OrPpc64Addr16HighestaOrMipsTlsGdOrTilegxImm16X0Hw3OrSparcGlobJmpOrArmPrel31OrM68KTlsTprel32OrI386IrelativeOrCkcoreDoffsetLo16OrX8664RexGotpcrelxOrM32RSda16RelaOrNios2GotLoOrTileproImm16X1GotLoOrRiscvGnuVtentryOrMetagPltOrNds32RelativeOrIa64Gprel22OrArcSectoffMe2;
    #[allow(non_upper_case_globals)]
    pub const S390TlsGotie32 : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Uaddr64 : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    #[allow(non_upper_case_globals)]
    pub const MipsTlsLdm : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw3 : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    #[allow(non_upper_case_globals)]
    pub const Sparc7 : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    #[allow(non_upper_case_globals)]
    pub const ArmMovwAbsNc : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    #[allow(non_upper_case_globals)]
    pub const I386Got32X : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    #[allow(non_upper_case_globals)]
    pub const M68KNum : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    /// disp ((S+A-P) >>1) & 0x3ffff
    #[allow(non_upper_case_globals)]
    pub const CkcorePcrelImm18By2 : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    #[allow(non_upper_case_globals)]
    pub const X8664Num : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    #[allow(non_upper_case_globals)]
    pub const M32RRelaGnuVtinherit : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    /// %hiadj() of GOT entry.
    #[allow(non_upper_case_globals)]
    pub const Nios2GotHa : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    /// X0 pipe high 16-bit GOT offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0GotHi : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    #[allow(non_upper_case_globals)]
    pub const RiscvAlign : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    #[allow(non_upper_case_globals)]
    pub const MetagCopy : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    /// @gprel(sym + add), mov imm64
    #[allow(non_upper_case_globals)]
    pub const Ia64Gprel64I : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    #[allow(non_upper_case_globals)]
    pub const ArcSectoff1 : Self = Self::_S390TlsGotie32OrPpc64Uaddr64OrMipsTlsLdmOrTilegxImm16X1Hw3OrSparc7OrArmMovwAbsNcOrI386Got32XOrM68KNumOrCkcorePcrelImm18By2OrX8664NumOrM32RRelaGnuVtinheritOrNios2GotHaOrTileproImm16X0GotHiOrRiscvAlignOrMetagCopyOrIa64Gprel64IOrArcSectoff1;
    #[allow(non_upper_case_globals)]
    pub const S390TlsGotie64 : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel64 : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    #[allow(non_upper_case_globals)]
    pub const MipsTlsDtprelHi16 : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw0Last : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    #[allow(non_upper_case_globals)]
    pub const Sparc5 : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    #[allow(non_upper_case_globals)]
    pub const ArmMovtAbs : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    #[allow(non_upper_case_globals)]
    pub const I386Num : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    /// disp (S+A-BDATA) & 0x3ffff
    #[allow(non_upper_case_globals)]
    pub const CkcoreDoffsetImm18 : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    #[allow(non_upper_case_globals)]
    pub const M32RRelaGnuVtentry : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    /// %lo() of function GOT entry.
    #[allow(non_upper_case_globals)]
    pub const Nios2CallLo : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    /// X1 pipe high 16-bit GOT offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1GotHi : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    #[allow(non_upper_case_globals)]
    pub const RiscvRvcBranch : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    #[allow(non_upper_case_globals)]
    pub const MetagJmpSlot : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    /// @gprel(sym + add), data4 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Gprel32Msb : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    #[allow(non_upper_case_globals)]
    pub const ArcSectoff2 : Self = Self::_S390TlsGotie64OrPpc64Rel64OrMipsTlsDtprelHi16OrTilegxImm16X0Hw0LastOrSparc5OrArmMovtAbsOrI386NumOrCkcoreDoffsetImm18OrM32RRelaGnuVtentryOrNios2CallLoOrTileproImm16X1GotHiOrRiscvRvcBranchOrMetagJmpSlotOrIa64Gprel32MsbOrArcSectoff2;
    #[allow(non_upper_case_globals)]
    pub const S390TlsLdm32 : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Plt64 : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const MipsTlsDtprelLo16 : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw0Last : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const Sparc6 : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const ArmMovwPrelNc : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    /// disp ((S+A-BDATA)>>1) & 0x3ffff
    #[allow(non_upper_case_globals)]
    pub const CkcoreDoffsetImm18By2 : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    /// PC relative 32 bit.
    #[allow(non_upper_case_globals)]
    pub const M32RRel32 : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    /// %hiadj() of function GOT entry.
    #[allow(non_upper_case_globals)]
    pub const Nios2CallHa : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    /// X0 pipe ha() 16-bit GOT offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0GotHa : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const RiscvRvcJump : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const MetagRelative : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    /// @gprel(sym + add), data4 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Gprel32Lsb : Self = Self::_S390TlsLdm32OrPpc64Plt64OrMipsTlsDtprelLo16OrTilegxImm16X1Hw0LastOrSparc6OrArmMovwPrelNcOrCkcoreDoffsetImm18By2OrM32RRel32OrNios2CallHaOrTileproImm16X0GotHaOrRiscvRvcJumpOrMetagRelativeOrIa64Gprel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const S390TlsLdm64 : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Pltrel64 : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    #[allow(non_upper_case_globals)]
    pub const MipsTlsGottprel : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw1Last : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    #[allow(non_upper_case_globals)]
    pub const SparcDisp64 : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    #[allow(non_upper_case_globals)]
    pub const ArmMovtPrel : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    #[allow(non_upper_case_globals)]
    pub const AlphaNum : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    /// disp ((S+A-BDATA)>>2) & 0x3ffff
    #[allow(non_upper_case_globals)]
    pub const CkcoreDoffsetImm18By4 : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    /// X1 pipe ha() 16-bit GOT offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1GotHa : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    #[allow(non_upper_case_globals)]
    pub const RiscvRvcLui : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    #[allow(non_upper_case_globals)]
    pub const MetagGlobDat : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    /// @gprel(sym + add), data8 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Gprel64Msb : Self = Self::_S390TlsLdm64OrPpc64Pltrel64OrMipsTlsGottprelOrTilegxImm16X0Hw1LastOrSparcDisp64OrArmMovtPrelOrAlphaNumOrCkcoreDoffsetImm18By4OrTileproImm16X1GotHaOrRiscvRvcLuiOrMetagGlobDatOrIa64Gprel64Msb;
    #[allow(non_upper_case_globals)]
    pub const S390TlsIe32 : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrIa64Gprel64LsbOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGd;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Toc16 : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrIa64Gprel64LsbOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGd;
    #[allow(non_upper_case_globals)]
    pub const MipsTlsTprel32 : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrIa64Gprel64LsbOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGd;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw1Last : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrIa64Gprel64LsbOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGd;
    #[allow(non_upper_case_globals)]
    pub const SparcPlt64 : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrIa64Gprel64LsbOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGd;
    #[allow(non_upper_case_globals)]
    pub const ArmThmMovwAbsNc : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrIa64Gprel64LsbOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGd;
    /// @gprel(sym + add), data8 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Gprel64Lsb : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrIa64Gprel64LsbOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGd;
    /// X0 pipe mm "start"
    #[allow(non_upper_case_globals)]
    pub const TileproMmstartX0 : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrIa64Gprel64LsbOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGd;
    #[allow(non_upper_case_globals)]
    pub const RiscvGprelI : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrIa64Gprel64LsbOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGd;
    #[allow(non_upper_case_globals)]
    pub const MetagTlsGd : Self = Self::_S390TlsIe32OrPpc64Toc16OrMipsTlsTprel32OrTilegxImm16X1Hw1LastOrSparcPlt64OrArmThmMovwAbsNcOrIa64Gprel64LsbOrTileproMmstartX0OrRiscvGprelIOrMetagTlsGd;
    #[allow(non_upper_case_globals)]
    pub const S390TlsIe64 : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Toc16Lo : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    #[allow(non_upper_case_globals)]
    pub const MipsTlsTprel64 : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw2Last : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    #[allow(non_upper_case_globals)]
    pub const SparcHix22 : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    #[allow(non_upper_case_globals)]
    pub const ArmThmMovtAbs : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    /// No relocation, set segment base.
    #[allow(non_upper_case_globals)]
    pub const PariscSegbase : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    /// disp (G >> 2)
    #[allow(non_upper_case_globals)]
    pub const CkcoreGotImm18By4 : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    /// 24 bit GOT entry
    #[allow(non_upper_case_globals)]
    pub const M32RGot24 : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    /// X0 pipe mm "end"
    #[allow(non_upper_case_globals)]
    pub const TileproMmendX0 : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    #[allow(non_upper_case_globals)]
    pub const RiscvGprelS : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    #[allow(non_upper_case_globals)]
    pub const MetagTlsLdm : Self = Self::_S390TlsIe64OrPpc64Toc16LoOrMipsTlsTprel64OrTilegxImm16X0Hw2LastOrSparcHix22OrArmThmMovtAbsOrPariscSegbaseOrCkcoreGotImm18By4OrM32RGot24OrTileproMmendX0OrRiscvGprelSOrMetagTlsLdm;
    #[allow(non_upper_case_globals)]
    pub const S390TlsIeent : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Toc16Hi : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    #[allow(non_upper_case_globals)]
    pub const MipsTlsTprelHi16 : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw2Last : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    #[allow(non_upper_case_globals)]
    pub const SparcLox10 : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    #[allow(non_upper_case_globals)]
    pub const ArmThmMovwPrelNc : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    /// 32 bits segment rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscSegrel32 : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    /// disp (G >> 2)
    #[allow(non_upper_case_globals)]
    pub const CkcorePltImm18By4 : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    /// 26 bit PC relative to PLT shifted
    #[allow(non_upper_case_globals)]
    pub const M32R26Pltrel : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    /// X1 pipe mm "start"
    #[allow(non_upper_case_globals)]
    pub const TileproMmstartX1 : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    #[allow(non_upper_case_globals)]
    pub const RiscvTprelI : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    #[allow(non_upper_case_globals)]
    pub const MetagTlsLdoHi16 : Self = Self::_S390TlsIeentOrPpc64Toc16HiOrMipsTlsTprelHi16OrTilegxImm16X1Hw2LastOrSparcLox10OrArmThmMovwPrelNcOrPariscSegrel32OrCkcorePltImm18By4OrM32R26PltrelOrTileproMmstartX1OrRiscvTprelIOrMetagTlsLdoHi16;
    #[allow(non_upper_case_globals)]
    pub const S390TlsLe32 : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Toc16Ha : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    #[allow(non_upper_case_globals)]
    pub const MipsTlsTprelLo16 : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw0Pcrel : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    #[allow(non_upper_case_globals)]
    pub const SparcH44 : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    #[allow(non_upper_case_globals)]
    pub const ArmThmMovtPrel : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    /// PLT rel. address, left 21 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscPltoff21L : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    /// disp ((S+A-P) >>2) & 0x7f
    #[allow(non_upper_case_globals)]
    pub const CkcorePcrelImm7By4 : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    /// Copy symbol at runtime
    #[allow(non_upper_case_globals)]
    pub const M32RCopy : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    /// X1 pipe mm "end"
    #[allow(non_upper_case_globals)]
    pub const TileproMmendX1 : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    #[allow(non_upper_case_globals)]
    pub const RiscvTprelS : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    #[allow(non_upper_case_globals)]
    pub const MetagTlsLdoLo16 : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    /// @ltoff(sym + add), add imm22
    #[allow(non_upper_case_globals)]
    pub const Ia64Ltoff22 : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    #[allow(non_upper_case_globals)]
    pub const ArcPc32 : Self = Self::_S390TlsLe32OrPpc64Toc16HaOrMipsTlsTprelLo16OrTilegxImm16X0Hw0PcrelOrSparcH44OrArmThmMovtPrelOrPariscPltoff21LOrCkcorePcrelImm7By4OrM32RCopyOrTileproMmendX1OrRiscvTprelSOrMetagTlsLdoLo16OrIa64Ltoff22OrArcPc32;
    #[allow(non_upper_case_globals)]
    pub const S390TlsLe64 : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Toc : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    #[allow(non_upper_case_globals)]
    pub const MipsGlobDat : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw0Pcrel : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    #[allow(non_upper_case_globals)]
    pub const SparcM44 : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    #[allow(non_upper_case_globals)]
    pub const ArmThmJump19 : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    /// 32 bit offset to TLS block
    #[allow(non_upper_case_globals)]
    pub const CkcoreTlsLe32 : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    /// Create GOT entry
    #[allow(non_upper_case_globals)]
    pub const M32RGlobDat : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    /// X0 pipe shift amount
    #[allow(non_upper_case_globals)]
    pub const TileproShamtX0 : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    #[allow(non_upper_case_globals)]
    pub const RiscvRelax : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    #[allow(non_upper_case_globals)]
    pub const MetagTlsLdo : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    /// @ltoff(sym + add), mov imm64
    #[allow(non_upper_case_globals)]
    pub const Ia64Ltoff64I : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    #[allow(non_upper_case_globals)]
    pub const ArcGotpc32 : Self = Self::_S390TlsLe64OrPpc64TocOrMipsGlobDatOrTilegxImm16X1Hw0PcrelOrSparcM44OrArmThmJump19OrCkcoreTlsLe32OrM32RGlobDatOrTileproShamtX0OrRiscvRelaxOrMetagTlsLdoOrIa64Ltoff64IOrArcGotpc32;
    #[allow(non_upper_case_globals)]
    pub const S390TlsLdo32 : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Pltgot16 : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw1Pcrel : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    #[allow(non_upper_case_globals)]
    pub const SparcL44 : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    #[allow(non_upper_case_globals)]
    pub const ArmThmJump6 : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    #[allow(non_upper_case_globals)]
    pub const CkcoreTlsIe32 : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    /// Create PLT entry
    #[allow(non_upper_case_globals)]
    pub const M32RJmpSlot : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    /// X1 pipe shift amount
    #[allow(non_upper_case_globals)]
    pub const TileproShamtX1 : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    #[allow(non_upper_case_globals)]
    pub const RiscvSub6 : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    #[allow(non_upper_case_globals)]
    pub const MetagTlsIe : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    #[allow(non_upper_case_globals)]
    pub const ArcPlt32 : Self = Self::_S390TlsLdo32OrPpc64Pltgot16OrTilegxImm16X0Hw1PcrelOrSparcL44OrArmThmJump6OrCkcoreTlsIe32OrM32RJmpSlotOrTileproShamtX1OrRiscvSub6OrMetagTlsIeOrArcPlt32;
    #[allow(non_upper_case_globals)]
    pub const S390TlsLdo64 : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Pltgot16Lo : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw1Pcrel : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    #[allow(non_upper_case_globals)]
    pub const SparcRegister : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    #[allow(non_upper_case_globals)]
    pub const ArmThmAluPrel110 : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    #[allow(non_upper_case_globals)]
    pub const CkcoreTlsGd32 : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    /// Adjust by program base
    #[allow(non_upper_case_globals)]
    pub const M32RRelative : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    /// Y0 pipe shift amount
    #[allow(non_upper_case_globals)]
    pub const TileproShamtY0 : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    #[allow(non_upper_case_globals)]
    pub const RiscvSet6 : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    #[allow(non_upper_case_globals)]
    pub const MetagTlsIenonpic : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    #[allow(non_upper_case_globals)]
    pub const ArcCopy : Self = Self::_S390TlsLdo64OrPpc64Pltgot16LoOrTilegxImm16X1Hw1PcrelOrSparcRegisterOrArmThmAluPrel110OrCkcoreTlsGd32OrM32RRelativeOrTileproShamtY0OrRiscvSet6OrMetagTlsIenonpicOrArcCopy;
    #[allow(non_upper_case_globals)]
    pub const S390TlsDtpmod : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Pltgot16Hi : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw2Pcrel : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    #[allow(non_upper_case_globals)]
    pub const SparcUa64 : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    #[allow(non_upper_case_globals)]
    pub const ArmThmPc12 : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    /// PLT rel. address, right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscPltoff14R : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    #[allow(non_upper_case_globals)]
    pub const CkcoreTlsLdm32 : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    /// 24 bit offset to GOT
    #[allow(non_upper_case_globals)]
    pub const M32RGotoff : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    /// Y1 pipe shift amount
    #[allow(non_upper_case_globals)]
    pub const TileproShamtY1 : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    #[allow(non_upper_case_globals)]
    pub const RiscvSet8 : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    #[allow(non_upper_case_globals)]
    pub const MetagTlsIenonpicHi16 : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    #[allow(non_upper_case_globals)]
    pub const ArcGlobDat : Self = Self::_S390TlsDtpmodOrPpc64Pltgot16HiOrTilegxImm16X0Hw2PcrelOrSparcUa64OrArmThmPc12OrPariscPltoff14ROrCkcoreTlsLdm32OrM32RGotoffOrTileproShamtY1OrRiscvSet8OrMetagTlsIenonpicHi16OrArcGlobDat;
    #[allow(non_upper_case_globals)]
    pub const S390TlsDtpoff : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Pltgot16Ha : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw2Pcrel : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    #[allow(non_upper_case_globals)]
    pub const SparcUa16 : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    #[allow(non_upper_case_globals)]
    pub const ArmAbs32Noi : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    #[allow(non_upper_case_globals)]
    pub const CkcoreTlsLdo32 : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    /// 24 bit PC relative offset to GOT
    #[allow(non_upper_case_globals)]
    pub const M32RGotpc24 : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    /// X1 pipe destination 8-bit
    #[allow(non_upper_case_globals)]
    pub const TileproDestImm8X1 : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    #[allow(non_upper_case_globals)]
    pub const RiscvSet16 : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    #[allow(non_upper_case_globals)]
    pub const MetagTlsIenonpicLo16 : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    #[allow(non_upper_case_globals)]
    pub const ArcJumpSlot : Self = Self::_S390TlsDtpoffOrPpc64Pltgot16HaOrTilegxImm16X1Hw2PcrelOrSparcUa16OrArmAbs32NoiOrCkcoreTlsLdo32OrM32RGotpc24OrTileproDestImm8X1OrRiscvSet16OrMetagTlsIenonpicLo16OrArcJumpSlot;
    #[allow(non_upper_case_globals)]
    pub const S390TlsTpoff : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr16Ds : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw3Pcrel : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsGdHi22 : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    #[allow(non_upper_case_globals)]
    pub const ArmRel32Noi : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    #[allow(non_upper_case_globals)]
    pub const CkcoreTlsDtpmod32 : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    #[allow(non_upper_case_globals)]
    pub const M32RGot16HiUlo : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    #[allow(non_upper_case_globals)]
    pub const RiscvSet32 : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    #[allow(non_upper_case_globals)]
    pub const MetagTlsTpoff : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    #[allow(non_upper_case_globals)]
    pub const ArcRelative : Self = Self::_S390TlsTpoffOrPpc64Addr16DsOrTilegxImm16X0Hw3PcrelOrSparcTlsGdHi22OrArmRel32NoiOrCkcoreTlsDtpmod32OrM32RGot16HiUloOrRiscvSet32OrMetagTlsTpoffOrArcRelative;
    #[allow(non_upper_case_globals)]
    pub const S390Got20 : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Got16Ds : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw0LastPcrel : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsGdAdd : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    #[allow(non_upper_case_globals)]
    pub const ArmAluPcG0 : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    /// LT-rel. fct ptr, left 21 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoffFptr21L : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    #[allow(non_upper_case_globals)]
    pub const CkcoreTlsTpoff32 : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    /// Low 16 bit GOT entry
    #[allow(non_upper_case_globals)]
    pub const M32RGot16Lo : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    #[allow(non_upper_case_globals)]
    pub const RiscvIrelative : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    #[allow(non_upper_case_globals)]
    pub const MetagTlsDtpoff : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    /// @pltoff(sym + add), add imm22
    #[allow(non_upper_case_globals)]
    pub const Ia64Pltoff22 : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    #[allow(non_upper_case_globals)]
    pub const ArcGotpc : Self = Self::_S390Got20OrPpc64Got16DsOrTilegxImm16X0Hw0LastPcrelOrSparcTlsGdAddOrArmAluPcG0OrPariscLtoffFptr21LOrCkcoreTlsTpoff32OrM32RGot16LoOrRiscvIrelativeOrMetagTlsDtpoffOrIa64Pltoff22OrArcGotpc;
    #[allow(non_upper_case_globals)]
    pub const S390Gotplt20 : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrIa64Pltoff64IOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrArcGot32;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Got16LoDs : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrIa64Pltoff64IOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrArcGot32;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw0LastPcrel : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrIa64Pltoff64IOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrArcGot32;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsGdCall : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrIa64Pltoff64IOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrArcGot32;
    #[allow(non_upper_case_globals)]
    pub const ArmAluPcG1Nc : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrIa64Pltoff64IOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrArcGot32;
    /// @pltoff(sym + add), mov imm64
    #[allow(non_upper_case_globals)]
    pub const Ia64Pltoff64I : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrIa64Pltoff64IOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrArcGot32;
    #[allow(non_upper_case_globals)]
    pub const M32RGotpcHiUlo : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrIa64Pltoff64IOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrArcGot32;
    #[allow(non_upper_case_globals)]
    pub const RiscvNum : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrIa64Pltoff64IOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrArcGot32;
    #[allow(non_upper_case_globals)]
    pub const MetagTlsLe : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrIa64Pltoff64IOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrArcGot32;
    #[allow(non_upper_case_globals)]
    pub const ArcGot32 : Self = Self::_S390Gotplt20OrPpc64Got16LoDsOrTilegxImm16X1Hw0LastPcrelOrSparcTlsGdCallOrArmAluPcG1NcOrIa64Pltoff64IOrM32RGotpcHiUloOrRiscvNumOrMetagTlsLeOrArcGot32;
    #[allow(non_upper_case_globals)]
    pub const S390TlsGotie20 : Self = Self::_S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Plt16LoDs : Self = Self::_S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16;
    #[allow(non_upper_case_globals)]
    pub const MipsPc21S2 : Self = Self::_S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw1LastPcrel : Self = Self::_S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsLdmHi22 : Self = Self::_S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16;
    #[allow(non_upper_case_globals)]
    pub const ArmAluPcG1 : Self = Self::_S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16;
    #[allow(non_upper_case_globals)]
    pub const M32RGotpcHiSlo : Self = Self::_S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16;
    /// "jal" for TLS GD
    #[allow(non_upper_case_globals)]
    pub const TileproTlsGdCall : Self = Self::_S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16;
    #[allow(non_upper_case_globals)]
    pub const MetagTlsLeHi16 : Self = Self::_S390TlsGotie20OrPpc64Plt16LoDsOrMipsPc21S2OrTilegxImm16X0Hw1LastPcrelOrSparcTlsLdmHi22OrArmAluPcG1OrM32RGotpcHiSloOrTileproTlsGdCallOrMetagTlsLeHi16;
    #[allow(non_upper_case_globals)]
    pub const S390Irelative : Self = Self::_S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16;
    #[allow(non_upper_case_globals)]
    pub const Ppc64SectoffDs : Self = Self::_S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16;
    #[allow(non_upper_case_globals)]
    pub const MipsPc26S2 : Self = Self::_S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw1LastPcrel : Self = Self::_S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsLdmLo10 : Self = Self::_S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16;
    #[allow(non_upper_case_globals)]
    pub const ArmAluPcG2 : Self = Self::_S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16;
    #[allow(non_upper_case_globals)]
    pub const M32RGotpcLo : Self = Self::_S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16;
    /// X0 pipe "addi" for TLS GD
    #[allow(non_upper_case_globals)]
    pub const TileproImm8X0TlsGdAdd : Self = Self::_S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16;
    #[allow(non_upper_case_globals)]
    pub const MetagTlsLeLo16 : Self = Self::_S390IrelativeOrPpc64SectoffDsOrMipsPc26S2OrTilegxImm16X1Hw1LastPcrelOrSparcTlsLdmLo10OrArmAluPcG2OrM32RGotpcLoOrTileproImm8X0TlsGdAddOrMetagTlsLeLo16;
    #[allow(non_upper_case_globals)]
    pub const S390Pc12Dbl : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    #[allow(non_upper_case_globals)]
    pub const Ppc64SectoffLoDs : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    #[allow(non_upper_case_globals)]
    pub const MipsPc18S3 : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw2LastPcrel : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsLdmAdd : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    #[allow(non_upper_case_globals)]
    pub const ArmLdrPcG1 : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    /// LT-rel. fct ptr, right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoffFptr14R : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    #[allow(non_upper_case_globals)]
    pub const S390Num : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    #[allow(non_upper_case_globals)]
    pub const M32RGotoffHiUlo : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    /// X1 pipe "addi" for TLS GD
    #[allow(non_upper_case_globals)]
    pub const TileproImm8X1TlsGdAdd : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    /// @pltoff(sym + add), data8 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Pltoff64Msb : Self = Self::_S390Pc12DblOrPpc64SectoffLoDsOrMipsPc18S3OrTilegxImm16X0Hw2LastPcrelOrSparcTlsLdmAddOrArmLdrPcG1OrPariscLtoffFptr14ROrS390NumOrM32RGotoffHiUloOrTileproImm8X1TlsGdAddOrIa64Pltoff64Msb;
    #[allow(non_upper_case_globals)]
    pub const S390Plt12Dbl : Self = Self::_S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrIa64Pltoff64LsbOrM32RGotoffHiSloOrTileproImm8Y0TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Toc16Ds : Self = Self::_S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrIa64Pltoff64LsbOrM32RGotoffHiSloOrTileproImm8Y0TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const MipsPc19S2 : Self = Self::_S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrIa64Pltoff64LsbOrM32RGotoffHiSloOrTileproImm8Y0TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw2LastPcrel : Self = Self::_S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrIa64Pltoff64LsbOrM32RGotoffHiSloOrTileproImm8Y0TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsLdmCall : Self = Self::_S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrIa64Pltoff64LsbOrM32RGotoffHiSloOrTileproImm8Y0TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const ArmLdrPcG2 : Self = Self::_S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrIa64Pltoff64LsbOrM32RGotoffHiSloOrTileproImm8Y0TlsGdAdd;
    /// @pltoff(sym + add), data8 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Pltoff64Lsb : Self = Self::_S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrIa64Pltoff64LsbOrM32RGotoffHiSloOrTileproImm8Y0TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const M32RGotoffHiSlo : Self = Self::_S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrIa64Pltoff64LsbOrM32RGotoffHiSloOrTileproImm8Y0TlsGdAdd;
    /// Y0 pipe "addi" for TLS GD
    #[allow(non_upper_case_globals)]
    pub const TileproImm8Y0TlsGdAdd : Self = Self::_S390Plt12DblOrPpc64Toc16DsOrMipsPc19S2OrTilegxImm16X1Hw2LastPcrelOrSparcTlsLdmCallOrArmLdrPcG2OrIa64Pltoff64LsbOrM32RGotoffHiSloOrTileproImm8Y0TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const S390Pc24Dbl : Self = Self::_S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Toc16LoDs : Self = Self::_S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const MipsPchi16 : Self = Self::_S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw0Got : Self = Self::_S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsLdoHix22 : Self = Self::_S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const ArmLdrsPcG0 : Self = Self::_S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd;
    /// 64 bits function address.
    #[allow(non_upper_case_globals)]
    pub const PariscFptr64 : Self = Self::_S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd;
    /// Low 16 bit offset to GOT
    #[allow(non_upper_case_globals)]
    pub const M32RGotoffLo : Self = Self::_S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd;
    /// Y1 pipe "addi" for TLS GD
    #[allow(non_upper_case_globals)]
    pub const TileproImm8Y1TlsGdAdd : Self = Self::_S390Pc24DblOrPpc64Toc16LoDsOrMipsPchi16OrTilegxImm16X0Hw0GotOrSparcTlsLdoHix22OrArmLdrsPcG0OrPariscFptr64OrM32RGotoffLoOrTileproImm8Y1TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const S390Plt24Dbl : Self = Self::_S390Plt24DblOrPpc64Pltgot16DsOrMipsPclo16OrTilegxImm16X1Hw0GotOrSparcTlsLdoLox10OrArmLdrsPcG1OrPariscPlabel32OrTileproTlsIeLoad;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Pltgot16Ds : Self = Self::_S390Plt24DblOrPpc64Pltgot16DsOrMipsPclo16OrTilegxImm16X1Hw0GotOrSparcTlsLdoLox10OrArmLdrsPcG1OrPariscPlabel32OrTileproTlsIeLoad;
    #[allow(non_upper_case_globals)]
    pub const MipsPclo16 : Self = Self::_S390Plt24DblOrPpc64Pltgot16DsOrMipsPclo16OrTilegxImm16X1Hw0GotOrSparcTlsLdoLox10OrArmLdrsPcG1OrPariscPlabel32OrTileproTlsIeLoad;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw0Got : Self = Self::_S390Plt24DblOrPpc64Pltgot16DsOrMipsPclo16OrTilegxImm16X1Hw0GotOrSparcTlsLdoLox10OrArmLdrsPcG1OrPariscPlabel32OrTileproTlsIeLoad;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsLdoLox10 : Self = Self::_S390Plt24DblOrPpc64Pltgot16DsOrMipsPclo16OrTilegxImm16X1Hw0GotOrSparcTlsLdoLox10OrArmLdrsPcG1OrPariscPlabel32OrTileproTlsIeLoad;
    #[allow(non_upper_case_globals)]
    pub const ArmLdrsPcG1 : Self = Self::_S390Plt24DblOrPpc64Pltgot16DsOrMipsPclo16OrTilegxImm16X1Hw0GotOrSparcTlsLdoLox10OrArmLdrsPcG1OrPariscPlabel32OrTileproTlsIeLoad;
    /// 32 bits function address.
    #[allow(non_upper_case_globals)]
    pub const PariscPlabel32 : Self = Self::_S390Plt24DblOrPpc64Pltgot16DsOrMipsPclo16OrTilegxImm16X1Hw0GotOrSparcTlsLdoLox10OrArmLdrsPcG1OrPariscPlabel32OrTileproTlsIeLoad;
    /// "lw_tls" for TLS IE
    #[allow(non_upper_case_globals)]
    pub const TileproTlsIeLoad : Self = Self::_S390Plt24DblOrPpc64Pltgot16DsOrMipsPclo16OrTilegxImm16X1Hw0GotOrSparcTlsLdoLox10OrArmLdrsPcG1OrPariscPlabel32OrTileproTlsIeLoad;
    #[allow(non_upper_case_globals)]
    pub const S390GnuVtentry : Self = Self::_S390GnuVtentryOrPowerpcRel16HiOrSparcGnuVtentryOrX8664GnuVtentryOrI386GnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22;
    #[allow(non_upper_case_globals)]
    pub const PowerpcRel16Hi : Self = Self::_S390GnuVtentryOrPowerpcRel16HiOrSparcGnuVtentryOrX8664GnuVtentryOrI386GnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22;
    #[allow(non_upper_case_globals)]
    pub const SparcGnuVtentry : Self = Self::_S390GnuVtentryOrPowerpcRel16HiOrSparcGnuVtentryOrX8664GnuVtentryOrI386GnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22;
    #[allow(non_upper_case_globals)]
    pub const X8664GnuVtentry : Self = Self::_S390GnuVtentryOrPowerpcRel16HiOrSparcGnuVtentryOrX8664GnuVtentryOrI386GnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22;
    #[allow(non_upper_case_globals)]
    pub const I386GnuVtentry : Self = Self::_S390GnuVtentryOrPowerpcRel16HiOrSparcGnuVtentryOrX8664GnuVtentryOrI386GnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22;
    /// half16   (sym+add-.)@h
    #[allow(non_upper_case_globals)]
    pub const PpcRel16Hi : Self = Self::_S390GnuVtentryOrPowerpcRel16HiOrSparcGnuVtentryOrX8664GnuVtentryOrI386GnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22;
    /// half16   (sym+add-.)@h
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel16Hi : Self = Self::_S390GnuVtentryOrPowerpcRel16HiOrSparcGnuVtentryOrX8664GnuVtentryOrI386GnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22;
    #[allow(non_upper_case_globals)]
    pub const ArmThmRpc22 : Self = Self::_S390GnuVtentryOrPowerpcRel16HiOrSparcGnuVtentryOrX8664GnuVtentryOrI386GnuVtentryOrPpcRel16HiOrPpc64Rel16HiOrArmThmRpc22;
    #[allow(non_upper_case_globals)]
    pub const PowerpcCopy : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    #[allow(non_upper_case_globals)]
    pub const MipsGotDisp : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    #[allow(non_upper_case_globals)]
    pub const TilegxRelative : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    #[allow(non_upper_case_globals)]
    pub const X8664Tlsgd : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    #[allow(non_upper_case_globals)]
    pub const SparcCopy : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    #[allow(non_upper_case_globals)]
    pub const ArmTlsTpoff32 : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    #[allow(non_upper_case_globals)]
    pub const I386TlsLdm : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    /// Copy symbol at runtime
    #[allow(non_upper_case_globals)]
    pub const M68KCopy : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    /// GP relative 16 bit
    #[allow(non_upper_case_globals)]
    pub const AlphaGprel16 : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    #[allow(non_upper_case_globals)]
    pub const PpcCopy : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    /// ((S + A - P) >> 1) & 0x3ffffff
    #[allow(non_upper_case_globals)]
    pub const CkcorePcrelImm26By2 : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    /// PC relative 32 bit shifted by 1.
    #[allow(non_upper_case_globals)]
    pub const S390Pc32Dbl : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    #[allow(non_upper_case_globals)]
    pub const Cris32PltPcrel : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    /// 16-bit offset to GOT entry.
    #[allow(non_upper_case_globals)]
    pub const Mn10300Got16 : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    /// 64 bit offset to GOT.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeGotoff64 : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    /// Conditional branch.
    #[allow(non_upper_case_globals)]
    pub const Nios2Cjmp : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    /// X1 pipe 8-bit
    #[allow(non_upper_case_globals)]
    pub const TileproImm8X1 : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    #[allow(non_upper_case_globals)]
    pub const RiscvCallPlt : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    #[allow(non_upper_case_globals)]
    pub const Or1KGlobDat : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Copy : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    #[allow(non_upper_case_globals)]
    pub const ArcSdaLdst : Self = Self::_PowerpcCopyOrMipsGotDispOrTilegxRelativeOrX8664TlsgdOrSparcCopyOrArmTlsTpoff32OrI386TlsLdmOrM68KCopyOrAlphaGprel16OrPpcCopyOrCkcorePcrelImm26By2OrS390Pc32DblOrCris32PltPcrelOrMn10300Got16OrMicroblazeGotoff64OrNios2CjmpOrTileproImm8X1OrRiscvCallPltOrOr1KGlobDatOrPpc64CopyOrArcSdaLdst;
    #[allow(non_upper_case_globals)]
    pub const PowerpcPlt32 : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    #[allow(non_upper_case_globals)]
    pub const MipsDelete : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    #[allow(non_upper_case_globals)]
    pub const TilegxDestImm8X1 : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    #[allow(non_upper_case_globals)]
    pub const X8664Got64 : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    #[allow(non_upper_case_globals)]
    pub const SparcPcplt32 : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    #[allow(non_upper_case_globals)]
    pub const ArmPlt32 : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    #[allow(non_upper_case_globals)]
    pub const I386TlsGdPop : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    /// 8 bit GOT offset for GD
    #[allow(non_upper_case_globals)]
    pub const M68KTlsGd8 : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    /// Adjust by program base
    #[allow(non_upper_case_globals)]
    pub const AlphaRelative : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    #[allow(non_upper_case_globals)]
    pub const PpcPlt32 : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    /// (GOT + A - P) & 0xffff
    #[allow(non_upper_case_globals)]
    pub const CkcoreGotpcLo16 : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    #[allow(non_upper_case_globals)]
    pub const ShUses : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    /// 16 bit offset to GOT.
    #[allow(non_upper_case_globals)]
    pub const S390Gotoff16 : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    #[allow(non_upper_case_globals)]
    pub const Mn10300TlsGotie : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    /// TLS Offset Within TLS Block.
    #[allow(non_upper_case_globals)]
    pub const MicroblazeTlsdtprel64 : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    /// %hiadj of PC relative offset.
    #[allow(non_upper_case_globals)]
    pub const Nios2PcrelHa : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    /// X0 pipe high 16-bit
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0Hi : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    #[allow(non_upper_case_globals)]
    pub const RiscvLo12I : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    #[allow(non_upper_case_globals)]
    pub const Or1KTlsLdoLo16 : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Plt32 : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    #[allow(non_upper_case_globals)]
    pub const Arc32Me : Self = Self::_PowerpcPlt32OrMipsDeleteOrTilegxDestImm8X1OrX8664Got64OrSparcPcplt32OrArmPlt32OrI386TlsGdPopOrM68KTlsGd8OrAlphaRelativeOrPpcPlt32OrCkcoreGotpcLo16OrShUsesOrS390Gotoff16OrMn10300TlsGotieOrMicroblazeTlsdtprel64OrNios2PcrelHaOrTileproImm16X0HiOrRiscvLo12IOrOr1KTlsLdoLo16OrPpc64Plt32OrArc32Me;
    #[allow(non_upper_case_globals)]
    pub const PowerpcAddr30 : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    #[allow(non_upper_case_globals)]
    pub const MipsJalr : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw0 : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    #[allow(non_upper_case_globals)]
    pub const X8664Irelative : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    #[allow(non_upper_case_globals)]
    pub const SparcPcHh22 : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    #[allow(non_upper_case_globals)]
    pub const ArmAluSbrel2720Ck : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    #[allow(non_upper_case_globals)]
    pub const I386TlsTpoff32 : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    #[allow(non_upper_case_globals)]
    pub const M68KTlsLe32 : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    #[allow(non_upper_case_globals)]
    pub const AlphaGottprel : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    /// word30 (S + A - P) >> 2
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr30 : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    /// Deprecated, prog. base relative.
    #[allow(non_upper_case_globals)]
    pub const ArmAluSbrel2720 : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    /// (GOT + G * 4) & 0xffff
    #[allow(non_upper_case_globals)]
    pub const CkcoreAddrgotLo16 : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    /// Tag for load insn in TLS code.
    #[allow(non_upper_case_globals)]
    pub const S390TlsLoad : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    /// PC relative 18 bit shifted.
    #[allow(non_upper_case_globals)]
    pub const M32R18PcrelRela : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    /// Create GOT entry.
    #[allow(non_upper_case_globals)]
    pub const Nios2GlobDat : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    /// X0 pipe PC relative ha() 16 bit
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0HaPcrel : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    #[allow(non_upper_case_globals)]
    pub const RiscvSub8 : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    #[allow(non_upper_case_globals)]
    pub const MetagLo16Gotpc : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    /// symbol + addend, data4 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Dir32Lsb : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    #[allow(non_upper_case_globals)]
    pub const AcSectoffU82 : Self = Self::_PowerpcAddr30OrMipsJalrOrTilegxImm16X1Hw0OrX8664IrelativeOrSparcPcHh22OrArmAluSbrel2720CkOrI386TlsTpoff32OrM68KTlsLe32OrAlphaGottprelOrPpc64Addr30OrArmAluSbrel2720OrCkcoreAddrgotLo16OrS390TlsLoadOrM32R18PcrelRelaOrNios2GlobDatOrTileproImm16X0HaPcrelOrRiscvSub8OrMetagLo16GotpcOrIa64Dir32LsbOrAcSectoffU82;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr16LoDs : Self = Self::_Ppc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrS39020OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw3Pcrel : Self = Self::_Ppc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrS39020OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsGdLo10 : Self = Self::_Ppc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrS39020OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    #[allow(non_upper_case_globals)]
    pub const ArmAluPcG0Nc : Self = Self::_Ppc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrS39020OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    /// 32 bits LT-rel. function pointer.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoffFptr32 : Self = Self::_Ppc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrS39020OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    #[allow(non_upper_case_globals)]
    pub const CkcoreTlsDtpoff32 : Self = Self::_Ppc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrS39020OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    /// Direct 20 bit.
    #[allow(non_upper_case_globals)]
    pub const S39020 : Self = Self::_Ppc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrS39020OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    #[allow(non_upper_case_globals)]
    pub const M32RGot16HiSlo : Self = Self::_Ppc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrS39020OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    #[allow(non_upper_case_globals)]
    pub const Riscv32Pcrel : Self = Self::_Ppc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrS39020OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    #[allow(non_upper_case_globals)]
    pub const MetagTlsDtpmod : Self = Self::_Ppc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrS39020OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    #[allow(non_upper_case_globals)]
    pub const ArcGotoff : Self = Self::_Ppc64Addr16LoDsOrTilegxImm16X1Hw3PcrelOrSparcTlsGdLo10OrArmAluPcG0NcOrPariscLtoffFptr32OrCkcoreTlsDtpoff32OrS39020OrM32RGot16HiSloOrRiscv32PcrelOrMetagTlsDtpmodOrArcGotoff;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Pltgot16LoDs : Self = Self::_Ppc64Pltgot16LoDsOrTilegxImm16X0Hw0PltPcrelOrSparcTlsLdoAddOrArmLdrsPcG2OrPariscPlabel21LOrTileproImm16X0TlsGdOrArcTlsDtpmod;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw0PltPcrel : Self = Self::_Ppc64Pltgot16LoDsOrTilegxImm16X0Hw0PltPcrelOrSparcTlsLdoAddOrArmLdrsPcG2OrPariscPlabel21LOrTileproImm16X0TlsGdOrArcTlsDtpmod;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsLdoAdd : Self = Self::_Ppc64Pltgot16LoDsOrTilegxImm16X0Hw0PltPcrelOrSparcTlsLdoAddOrArmLdrsPcG2OrPariscPlabel21LOrTileproImm16X0TlsGdOrArcTlsDtpmod;
    #[allow(non_upper_case_globals)]
    pub const ArmLdrsPcG2 : Self = Self::_Ppc64Pltgot16LoDsOrTilegxImm16X0Hw0PltPcrelOrSparcTlsLdoAddOrArmLdrsPcG2OrPariscPlabel21LOrTileproImm16X0TlsGdOrArcTlsDtpmod;
    /// Left 21 bits of fdesc address.
    #[allow(non_upper_case_globals)]
    pub const PariscPlabel21L : Self = Self::_Ppc64Pltgot16LoDsOrTilegxImm16X0Hw0PltPcrelOrSparcTlsLdoAddOrArmLdrsPcG2OrPariscPlabel21LOrTileproImm16X0TlsGdOrArcTlsDtpmod;
    /// X0 pipe 16-bit TLS GD offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0TlsGd : Self = Self::_Ppc64Pltgot16LoDsOrTilegxImm16X0Hw0PltPcrelOrSparcTlsLdoAddOrArmLdrsPcG2OrPariscPlabel21LOrTileproImm16X0TlsGdOrArcTlsDtpmod;
    #[allow(non_upper_case_globals)]
    pub const ArcTlsDtpmod : Self = Self::_Ppc64Pltgot16LoDsOrTilegxImm16X0Hw0PltPcrelOrSparcTlsLdoAddOrArmLdrsPcG2OrPariscPlabel21LOrTileproImm16X0TlsGdOrArcTlsDtpmod;
    #[allow(non_upper_case_globals)]
    pub const PowerpcTls : Self = Self::_PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw0PltPcrel : Self = Self::_PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsIeHi22 : Self = Self::_PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff;
    #[allow(non_upper_case_globals)]
    pub const ArmLdcPcG0 : Self = Self::_PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff;
    /// none	(sym+add)@tls
    #[allow(non_upper_case_globals)]
    pub const PpcTls : Self = Self::_PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff;
    /// none	(sym+add)@tls
    #[allow(non_upper_case_globals)]
    pub const Ppc64Tls : Self = Self::_PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff;
    /// X1 pipe 16-bit TLS GD offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1TlsGd : Self = Self::_PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff;
    /// @fptr(sym + add), mov imm64
    #[allow(non_upper_case_globals)]
    pub const Ia64Fptr64I : Self = Self::_PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff;
    #[allow(non_upper_case_globals)]
    pub const ArcTlsDtpoff : Self = Self::_PowerpcTlsOrTilegxImm16X1Hw0PltPcrelOrSparcTlsIeHi22OrArmLdcPcG0OrPpcTlsOrPpc64TlsOrTileproImm16X1TlsGdOrIa64Fptr64IOrArcTlsDtpoff;
    #[allow(non_upper_case_globals)]
    pub const PowerpcDtpmod : Self = Self::_PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw1PltPcrel : Self = Self::_PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsIeLo10 : Self = Self::_PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff;
    #[allow(non_upper_case_globals)]
    pub const ArmLdcPcG1 : Self = Self::_PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff;
    /// word32	(sym+add)@dtpmod
    #[allow(non_upper_case_globals)]
    pub const PpcDtpmod32 : Self = Self::_PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff;
    /// doubleword64 (sym+add)@dtpmod
    #[allow(non_upper_case_globals)]
    pub const Ppc64Dtpmod64 : Self = Self::_PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff;
    /// X0 pipe low 16-bit TLS GD offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0TlsGdLo : Self = Self::_PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff;
    /// @fptr(sym + add), data4 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Fptr32Msb : Self = Self::_PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff;
    #[allow(non_upper_case_globals)]
    pub const ArcTlsTpoff : Self = Self::_PowerpcDtpmodOrTilegxImm16X0Hw1PltPcrelOrSparcTlsIeLo10OrArmLdcPcG1OrPpcDtpmod32OrPpc64Dtpmod64OrTileproImm16X0TlsGdLoOrIa64Fptr32MsbOrArcTlsTpoff;
    #[allow(non_upper_case_globals)]
    pub const PowerpcTprel16 : Self = Self::_PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw1PltPcrel : Self = Self::_PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsIeLd : Self = Self::_PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot;
    #[allow(non_upper_case_globals)]
    pub const ArmLdcPcG2 : Self = Self::_PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot;
    /// half16*	(sym+add)@tprel
    #[allow(non_upper_case_globals)]
    pub const PpcTprel16 : Self = Self::_PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot;
    /// half16*	(sym+add)@tprel
    #[allow(non_upper_case_globals)]
    pub const Ppc64Tprel16 : Self = Self::_PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot;
    /// X1 pipe low 16-bit TLS GD offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1TlsGdLo : Self = Self::_PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot;
    /// @fptr(sym + add), data4 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Fptr32Lsb : Self = Self::_PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot;
    #[allow(non_upper_case_globals)]
    pub const ArcTlsGdGot : Self = Self::_PowerpcTprel16OrTilegxImm16X1Hw1PltPcrelOrSparcTlsIeLdOrArmLdcPcG2OrPpcTprel16OrPpc64Tprel16OrTileproImm16X1TlsGdLoOrIa64Fptr32LsbOrArcTlsGdGot;
    #[allow(non_upper_case_globals)]
    pub const PowerpcTprel16Lo : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw2PltPcrel : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsIeLdx : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    #[allow(non_upper_case_globals)]
    pub const ArmAluSbG0Nc : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    /// Right 14 bits of fdesc address.
    #[allow(non_upper_case_globals)]
    pub const PariscPlabel14R : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    /// half16	(sym+add)@tprel@l
    #[allow(non_upper_case_globals)]
    pub const PpcTprel16Lo : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    /// half16	(sym+add)@tprel@l
    #[allow(non_upper_case_globals)]
    pub const Ppc64Tprel16Lo : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    /// X0 pipe high 16-bit TLS GD offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0TlsGdHi : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    /// @fptr(sym + add), data8 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Fptr64Msb : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    #[allow(non_upper_case_globals)]
    pub const ArcTlsGdLd : Self = Self::_PowerpcTprel16LoOrTilegxImm16X0Hw2PltPcrelOrSparcTlsIeLdxOrArmAluSbG0NcOrPariscPlabel14ROrPpcTprel16LoOrPpc64Tprel16LoOrTileproImm16X0TlsGdHiOrIa64Fptr64MsbOrArcTlsGdLd;
    #[allow(non_upper_case_globals)]
    pub const PowerpcTprel16Hi : Self = Self::_PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw2PltPcrel : Self = Self::_PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsIeAdd : Self = Self::_PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall;
    #[allow(non_upper_case_globals)]
    pub const ArmAluSbG0 : Self = Self::_PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall;
    /// half16	(sym+add)@tprel@h
    #[allow(non_upper_case_globals)]
    pub const PpcTprel16Hi : Self = Self::_PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall;
    /// half16	(sym+add)@tprel@h
    #[allow(non_upper_case_globals)]
    pub const Ppc64Tprel16Hi : Self = Self::_PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall;
    /// X1 pipe high 16-bit TLS GD offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1TlsGdHi : Self = Self::_PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall;
    /// @fptr(sym + add), data8 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Fptr64Lsb : Self = Self::_PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall;
    #[allow(non_upper_case_globals)]
    pub const ArcTlsGdCall : Self = Self::_PowerpcTprel16HiOrTilegxImm16X1Hw2PltPcrelOrSparcTlsIeAddOrArmAluSbG0OrPpcTprel16HiOrPpc64Tprel16HiOrTileproImm16X1TlsGdHiOrIa64Fptr64LsbOrArcTlsGdCall;
    #[allow(non_upper_case_globals)]
    pub const PowerpcTprel16Ha : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw0LastGot : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsLeHix22 : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    #[allow(non_upper_case_globals)]
    pub const ArmAluSbG1Nc : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    /// 64 bits PC-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscPcrel64 : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    /// half16	(sym+add)@tprel@ha
    #[allow(non_upper_case_globals)]
    pub const PpcTprel16Ha : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    /// half16	(sym+add)@tprel@ha
    #[allow(non_upper_case_globals)]
    pub const Ppc64Tprel16Ha : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    /// X0 pipe ha() 16-bit TLS GD offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0TlsGdHa : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    /// @pcrel(sym + add), brl
    #[allow(non_upper_case_globals)]
    pub const Ia64Pcrel60B : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    #[allow(non_upper_case_globals)]
    pub const ArcTlsIeGot : Self = Self::_PowerpcTprel16HaOrTilegxImm16X0Hw0LastGotOrSparcTlsLeHix22OrArmAluSbG1NcOrPariscPcrel64OrPpcTprel16HaOrPpc64Tprel16HaOrTileproImm16X0TlsGdHaOrIa64Pcrel60BOrArcTlsIeGot;
    #[allow(non_upper_case_globals)]
    pub const PowerpcTprel : Self = Self::_PowerpcTprelOrTilegxImm16X1Hw0LastGotOrSparcTlsLeLox10OrArmAluSbG1OrPpcTprel32OrPpc64Tprel64OrTileproImm16X1TlsGdHaOrIa64Pcrel21B;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw0LastGot : Self = Self::_PowerpcTprelOrTilegxImm16X1Hw0LastGotOrSparcTlsLeLox10OrArmAluSbG1OrPpcTprel32OrPpc64Tprel64OrTileproImm16X1TlsGdHaOrIa64Pcrel21B;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsLeLox10 : Self = Self::_PowerpcTprelOrTilegxImm16X1Hw0LastGotOrSparcTlsLeLox10OrArmAluSbG1OrPpcTprel32OrPpc64Tprel64OrTileproImm16X1TlsGdHaOrIa64Pcrel21B;
    #[allow(non_upper_case_globals)]
    pub const ArmAluSbG1 : Self = Self::_PowerpcTprelOrTilegxImm16X1Hw0LastGotOrSparcTlsLeLox10OrArmAluSbG1OrPpcTprel32OrPpc64Tprel64OrTileproImm16X1TlsGdHaOrIa64Pcrel21B;
    /// word32	(sym+add)@tprel
    #[allow(non_upper_case_globals)]
    pub const PpcTprel32 : Self = Self::_PowerpcTprelOrTilegxImm16X1Hw0LastGotOrSparcTlsLeLox10OrArmAluSbG1OrPpcTprel32OrPpc64Tprel64OrTileproImm16X1TlsGdHaOrIa64Pcrel21B;
    /// doubleword64 (sym+add)@tprel
    #[allow(non_upper_case_globals)]
    pub const Ppc64Tprel64 : Self = Self::_PowerpcTprelOrTilegxImm16X1Hw0LastGotOrSparcTlsLeLox10OrArmAluSbG1OrPpcTprel32OrPpc64Tprel64OrTileproImm16X1TlsGdHaOrIa64Pcrel21B;
    /// X1 pipe ha() 16-bit TLS GD offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1TlsGdHa : Self = Self::_PowerpcTprelOrTilegxImm16X1Hw0LastGotOrSparcTlsLeLox10OrArmAluSbG1OrPpcTprel32OrPpc64Tprel64OrTileproImm16X1TlsGdHaOrIa64Pcrel21B;
    /// @pcrel(sym + add), ptb, call
    #[allow(non_upper_case_globals)]
    pub const Ia64Pcrel21B : Self = Self::_PowerpcTprelOrTilegxImm16X1Hw0LastGotOrSparcTlsLeLox10OrArmAluSbG1OrPpcTprel32OrPpc64Tprel64OrTileproImm16X1TlsGdHaOrIa64Pcrel21B;
    #[allow(non_upper_case_globals)]
    pub const PowerpcDtprel16 : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw1LastGot : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsDtpmod32 : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    #[allow(non_upper_case_globals)]
    pub const ArmAluSbG2 : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    /// 22 bits PC-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscPcrel22F : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    /// half16*	(sym+add)@dtprel
    #[allow(non_upper_case_globals)]
    pub const PpcDtprel16 : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    /// half16*	(sym+add)@dtprel
    #[allow(non_upper_case_globals)]
    pub const Ppc64Dtprel16 : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    /// X0 pipe 16-bit TLS IE offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0TlsIe : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    /// @pcrel(sym + add), chk.s
    #[allow(non_upper_case_globals)]
    pub const Ia64Pcrel21M : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    #[allow(non_upper_case_globals)]
    pub const ArcTlsDtpoffS9 : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    #[allow(non_upper_case_globals)]
    pub const ArcTlsLeS9 : Self = Self::_PowerpcDtprel16OrTilegxImm16X0Hw1LastGotOrSparcTlsDtpmod32OrArmAluSbG2OrPariscPcrel22FOrPpcDtprel16OrPpc64Dtprel16OrTileproImm16X0TlsIeOrIa64Pcrel21MOrArcTlsDtpoffS9OrArcTlsLeS9;
    #[allow(non_upper_case_globals)]
    pub const PowerpcDtprel16Lo : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw1LastGot : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsDtpmod64 : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    #[allow(non_upper_case_globals)]
    pub const ArmLdrSbG0 : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    /// PC-rel. address, right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscPcrel14Wr : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    /// half16	(sym+add)@dtprel@l
    #[allow(non_upper_case_globals)]
    pub const PpcDtprel16Lo : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    /// half16	(sym+add)@dtprel@l
    #[allow(non_upper_case_globals)]
    pub const Ppc64Dtprel16Lo : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    /// X1 pipe 16-bit TLS IE offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1TlsIe : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    /// @pcrel(sym + add), fchkf
    #[allow(non_upper_case_globals)]
    pub const Ia64Pcrel21F : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    #[allow(non_upper_case_globals)]
    pub const ArcTlsLe32 : Self = Self::_PowerpcDtprel16LoOrTilegxImm16X1Hw1LastGotOrSparcTlsDtpmod64OrArmLdrSbG0OrPariscPcrel14WrOrPpcDtprel16LoOrPpc64Dtprel16LoOrTileproImm16X1TlsIeOrIa64Pcrel21FOrArcTlsLe32;
    #[allow(non_upper_case_globals)]
    pub const PowerpcDtprel16Hi : Self = Self::_PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsDtpoff32 : Self = Self::_PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb;
    #[allow(non_upper_case_globals)]
    pub const ArmLdrSbG1 : Self = Self::_PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb;
    /// PC rel. address, right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscPcrel14Dr : Self = Self::_PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb;
    /// half16	(sym+add)@dtprel@h
    #[allow(non_upper_case_globals)]
    pub const PpcDtprel16Hi : Self = Self::_PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb;
    /// half16	(sym+add)@dtprel@h
    #[allow(non_upper_case_globals)]
    pub const Ppc64Dtprel16Hi : Self = Self::_PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb;
    /// X0 pipe low 16-bit TLS IE offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0TlsIeLo : Self = Self::_PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb;
    /// X0 pipe PC-rel PLT hword 3
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw3PltPcrel : Self = Self::_PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb;
    /// @pcrel(sym + add), data4 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Pcrel32Msb : Self = Self::_PowerpcDtprel16HiOrSparcTlsDtpoff32OrArmLdrSbG1OrPariscPcrel14DrOrPpcDtprel16HiOrPpc64Dtprel16HiOrTileproImm16X0TlsIeLoOrTilegxImm16X0Hw3PltPcrelOrIa64Pcrel32Msb;
    #[allow(non_upper_case_globals)]
    pub const PowerpcDtprel16Ha : Self = Self::_PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsDtpoff64 : Self = Self::_PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const ArmLdrSbG2 : Self = Self::_PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb;
    /// 16 bits PC-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscPcrel16F : Self = Self::_PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb;
    /// half16	(sym+add)@dtprel@ha
    #[allow(non_upper_case_globals)]
    pub const PpcDtprel16Ha : Self = Self::_PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb;
    /// half16	(sym+add)@dtprel@ha
    #[allow(non_upper_case_globals)]
    pub const Ppc64Dtprel16Ha : Self = Self::_PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb;
    /// X1 pipe low 16-bit TLS IE offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1TlsIeLo : Self = Self::_PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb;
    /// X1 pipe PC-rel PLT hword 3
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw3PltPcrel : Self = Self::_PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb;
    /// @pcrel(sym + add), data4 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Pcrel32Lsb : Self = Self::_PowerpcDtprel16HaOrSparcTlsDtpoff64OrArmLdrSbG2OrPariscPcrel16FOrPpcDtprel16HaOrPpc64Dtprel16HaOrTileproImm16X1TlsIeLoOrTilegxImm16X1Hw3PltPcrelOrIa64Pcrel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const PowerpcDtprel : Self = Self::_PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw0TlsGd : Self = Self::_PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsTpoff32 : Self = Self::_PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb;
    #[allow(non_upper_case_globals)]
    pub const ArmLdrsSbG0 : Self = Self::_PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb;
    /// 16 bits PC-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscPcrel16Wf : Self = Self::_PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb;
    /// word32	(sym+add)@dtprel
    #[allow(non_upper_case_globals)]
    pub const PpcDtprel32 : Self = Self::_PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb;
    /// doubleword64 (sym+add)@dtprel
    #[allow(non_upper_case_globals)]
    pub const Ppc64Dtprel64 : Self = Self::_PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb;
    /// X0 pipe high 16-bit TLS IE offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0TlsIeHi : Self = Self::_PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb;
    /// @pcrel(sym + add), data8 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Pcrel64Msb : Self = Self::_PowerpcDtprelOrTilegxImm16X0Hw0TlsGdOrSparcTlsTpoff32OrArmLdrsSbG0OrPariscPcrel16WfOrPpcDtprel32OrPpc64Dtprel64OrTileproImm16X0TlsIeHiOrIa64Pcrel64Msb;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGotTlsgd16 : Self = Self::_PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw0TlsGd : Self = Self::_PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const SparcTlsTpoff64 : Self = Self::_PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const ArmLdrsSbG1 : Self = Self::_PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb;
    /// 16 bits PC-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscPcrel16Df : Self = Self::_PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb;
    /// half16*	(sym+add)@got@tlsgd
    #[allow(non_upper_case_globals)]
    pub const PpcGotTlsgd16 : Self = Self::_PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb;
    /// half16*	(sym+add)@got@tlsgd
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotTlsgd16 : Self = Self::_PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb;
    /// X1 pipe high 16-bit TLS IE offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1TlsIeHi : Self = Self::_PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb;
    /// @pcrel(sym + add), data8 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Pcrel64Lsb : Self = Self::_PowerpcGotTlsgd16OrTilegxImm16X1Hw0TlsGdOrSparcTlsTpoff64OrArmLdrsSbG1OrPariscPcrel16DfOrPpcGotTlsgd16OrPpc64GotTlsgd16OrTileproImm16X1TlsIeHiOrIa64Pcrel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGotTlsgd16Lo : Self = Self::_PowerpcGotTlsgd16LoOrTilegxImm16X0Hw0TlsLeOrArmLdrsSbG2OrSparcGotdataHix22OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrTileproImm16X0TlsIeHa;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw0TlsLe : Self = Self::_PowerpcGotTlsgd16LoOrTilegxImm16X0Hw0TlsLeOrArmLdrsSbG2OrSparcGotdataHix22OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrTileproImm16X0TlsIeHa;
    #[allow(non_upper_case_globals)]
    pub const ArmLdrsSbG2 : Self = Self::_PowerpcGotTlsgd16LoOrTilegxImm16X0Hw0TlsLeOrArmLdrsSbG2OrSparcGotdataHix22OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrTileproImm16X0TlsIeHa;
    #[allow(non_upper_case_globals)]
    pub const SparcGotdataHix22 : Self = Self::_PowerpcGotTlsgd16LoOrTilegxImm16X0Hw0TlsLeOrArmLdrsSbG2OrSparcGotdataHix22OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrTileproImm16X0TlsIeHa;
    /// 64 bits of eff. address.
    #[allow(non_upper_case_globals)]
    pub const PariscDir64 : Self = Self::_PowerpcGotTlsgd16LoOrTilegxImm16X0Hw0TlsLeOrArmLdrsSbG2OrSparcGotdataHix22OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrTileproImm16X0TlsIeHa;
    /// half16	(sym+add)@got@tlsgd@l
    #[allow(non_upper_case_globals)]
    pub const PpcGotTlsgd16Lo : Self = Self::_PowerpcGotTlsgd16LoOrTilegxImm16X0Hw0TlsLeOrArmLdrsSbG2OrSparcGotdataHix22OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrTileproImm16X0TlsIeHa;
    /// half16	(sym+add)@got@tlsgd@l
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotTlsgd16Lo : Self = Self::_PowerpcGotTlsgd16LoOrTilegxImm16X0Hw0TlsLeOrArmLdrsSbG2OrSparcGotdataHix22OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrTileproImm16X0TlsIeHa;
    /// X0 pipe ha() 16-bit TLS IE offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0TlsIeHa : Self = Self::_PowerpcGotTlsgd16LoOrTilegxImm16X0Hw0TlsLeOrArmLdrsSbG2OrSparcGotdataHix22OrPariscDir64OrPpcGotTlsgd16LoOrPpc64GotTlsgd16LoOrTileproImm16X0TlsIeHa;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGotTlsgd16Hi : Self = Self::_PowerpcGotTlsgd16HiOrSparcGotdataLox10OrTilegxImm16X1Hw0TlsLeOrArmLdcSbG0OrPpcGotTlsgd16HiOrPpc64GotTlsgd16HiOrTileproImm16X1TlsIeHa;
    #[allow(non_upper_case_globals)]
    pub const SparcGotdataLox10 : Self = Self::_PowerpcGotTlsgd16HiOrSparcGotdataLox10OrTilegxImm16X1Hw0TlsLeOrArmLdcSbG0OrPpcGotTlsgd16HiOrPpc64GotTlsgd16HiOrTileproImm16X1TlsIeHa;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw0TlsLe : Self = Self::_PowerpcGotTlsgd16HiOrSparcGotdataLox10OrTilegxImm16X1Hw0TlsLeOrArmLdcSbG0OrPpcGotTlsgd16HiOrPpc64GotTlsgd16HiOrTileproImm16X1TlsIeHa;
    #[allow(non_upper_case_globals)]
    pub const ArmLdcSbG0 : Self = Self::_PowerpcGotTlsgd16HiOrSparcGotdataLox10OrTilegxImm16X1Hw0TlsLeOrArmLdcSbG0OrPpcGotTlsgd16HiOrPpc64GotTlsgd16HiOrTileproImm16X1TlsIeHa;
    /// half16	(sym+add)@got@tlsgd@h
    #[allow(non_upper_case_globals)]
    pub const PpcGotTlsgd16Hi : Self = Self::_PowerpcGotTlsgd16HiOrSparcGotdataLox10OrTilegxImm16X1Hw0TlsLeOrArmLdcSbG0OrPpcGotTlsgd16HiOrPpc64GotTlsgd16HiOrTileproImm16X1TlsIeHa;
    /// half16	(sym+add)@got@tlsgd@h
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotTlsgd16Hi : Self = Self::_PowerpcGotTlsgd16HiOrSparcGotdataLox10OrTilegxImm16X1Hw0TlsLeOrArmLdcSbG0OrPpcGotTlsgd16HiOrPpc64GotTlsgd16HiOrTileproImm16X1TlsIeHa;
    /// X1 pipe ha() 16-bit TLS IE offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1TlsIeHa : Self = Self::_PowerpcGotTlsgd16HiOrSparcGotdataLox10OrTilegxImm16X1Hw0TlsLeOrArmLdcSbG0OrPpcGotTlsgd16HiOrPpc64GotTlsgd16HiOrTileproImm16X1TlsIeHa;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGotTlsgd16Ha : Self = Self::_PowerpcGotTlsgd16HaOrSparcGotdataOpHix22OrTilegxImm16X0Hw0LastTlsLeOrArmLdcSbG1OrPpcGotTlsgd16HaOrPpc64GotTlsgd16HaOrTileproTlsDtpmod32OrIa64LtoffFptr22;
    #[allow(non_upper_case_globals)]
    pub const SparcGotdataOpHix22 : Self = Self::_PowerpcGotTlsgd16HaOrSparcGotdataOpHix22OrTilegxImm16X0Hw0LastTlsLeOrArmLdcSbG1OrPpcGotTlsgd16HaOrPpc64GotTlsgd16HaOrTileproTlsDtpmod32OrIa64LtoffFptr22;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw0LastTlsLe : Self = Self::_PowerpcGotTlsgd16HaOrSparcGotdataOpHix22OrTilegxImm16X0Hw0LastTlsLeOrArmLdcSbG1OrPpcGotTlsgd16HaOrPpc64GotTlsgd16HaOrTileproTlsDtpmod32OrIa64LtoffFptr22;
    #[allow(non_upper_case_globals)]
    pub const ArmLdcSbG1 : Self = Self::_PowerpcGotTlsgd16HaOrSparcGotdataOpHix22OrTilegxImm16X0Hw0LastTlsLeOrArmLdcSbG1OrPpcGotTlsgd16HaOrPpc64GotTlsgd16HaOrTileproTlsDtpmod32OrIa64LtoffFptr22;
    /// half16	(sym+add)@got@tlsgd@ha
    #[allow(non_upper_case_globals)]
    pub const PpcGotTlsgd16Ha : Self = Self::_PowerpcGotTlsgd16HaOrSparcGotdataOpHix22OrTilegxImm16X0Hw0LastTlsLeOrArmLdcSbG1OrPpcGotTlsgd16HaOrPpc64GotTlsgd16HaOrTileproTlsDtpmod32OrIa64LtoffFptr22;
    /// half16	(sym+add)@got@tlsgd@ha
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotTlsgd16Ha : Self = Self::_PowerpcGotTlsgd16HaOrSparcGotdataOpHix22OrTilegxImm16X0Hw0LastTlsLeOrArmLdcSbG1OrPpcGotTlsgd16HaOrPpc64GotTlsgd16HaOrTileproTlsDtpmod32OrIa64LtoffFptr22;
    /// ID of module containing symbol
    #[allow(non_upper_case_globals)]
    pub const TileproTlsDtpmod32 : Self = Self::_PowerpcGotTlsgd16HaOrSparcGotdataOpHix22OrTilegxImm16X0Hw0LastTlsLeOrArmLdcSbG1OrPpcGotTlsgd16HaOrPpc64GotTlsgd16HaOrTileproTlsDtpmod32OrIa64LtoffFptr22;
    /// @ltoff(@fptr(s+a)), imm22
    #[allow(non_upper_case_globals)]
    pub const Ia64LtoffFptr22 : Self = Self::_PowerpcGotTlsgd16HaOrSparcGotdataOpHix22OrTilegxImm16X0Hw0LastTlsLeOrArmLdcSbG1OrPpcGotTlsgd16HaOrPpc64GotTlsgd16HaOrTileproTlsDtpmod32OrIa64LtoffFptr22;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGotTlsld16 : Self = Self::_PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I;
    #[allow(non_upper_case_globals)]
    pub const SparcGotdataOpLox10 : Self = Self::_PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw0LastTlsLe : Self = Self::_PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I;
    #[allow(non_upper_case_globals)]
    pub const ArmLdcSbG2 : Self = Self::_PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I;
    /// 14 bits of eff. address.
    #[allow(non_upper_case_globals)]
    pub const PariscDir14Wr : Self = Self::_PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I;
    /// half16*	(sym+add)@got@tlsld
    #[allow(non_upper_case_globals)]
    pub const PpcGotTlsld16 : Self = Self::_PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I;
    /// half16*	(sym+add)@got@tlsld
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotTlsld16 : Self = Self::_PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I;
    /// Offset in TLS block
    #[allow(non_upper_case_globals)]
    pub const TileproTlsDtpoff32 : Self = Self::_PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I;
    /// @ltoff(@fptr(s+a)), imm64
    #[allow(non_upper_case_globals)]
    pub const Ia64LtoffFptr64I : Self = Self::_PowerpcGotTlsld16OrSparcGotdataOpLox10OrTilegxImm16X1Hw0LastTlsLeOrArmLdcSbG2OrPariscDir14WrOrPpcGotTlsld16OrPpc64GotTlsld16OrTileproTlsDtpoff32OrIa64LtoffFptr64I;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGotTlsld16Lo : Self = Self::_PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb;
    #[allow(non_upper_case_globals)]
    pub const SparcGotdataOp : Self = Self::_PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw1LastTlsLe : Self = Self::_PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb;
    #[allow(non_upper_case_globals)]
    pub const ArmMovwBrelNc : Self = Self::_PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb;
    /// 14 bits of eff. address.
    #[allow(non_upper_case_globals)]
    pub const PariscDir14Dr : Self = Self::_PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb;
    /// half16	(sym+add)@got@tlsld@l
    #[allow(non_upper_case_globals)]
    pub const PpcGotTlsld16Lo : Self = Self::_PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb;
    /// half16	(sym+add)@got@tlsld@l
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotTlsld16Lo : Self = Self::_PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb;
    /// Offset in static TLS block
    #[allow(non_upper_case_globals)]
    pub const TileproTlsTpoff32 : Self = Self::_PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb;
    /// @ltoff(@fptr(s+a)), data4 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64LtoffFptr32Msb : Self = Self::_PowerpcGotTlsld16LoOrSparcGotdataOpOrTilegxImm16X0Hw1LastTlsLeOrArmMovwBrelNcOrPariscDir14DrOrPpcGotTlsld16LoOrPpc64GotTlsld16LoOrTileproTlsTpoff32OrIa64LtoffFptr32Msb;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGotTlsld16Hi : Self = Self::_PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw1LastTlsLe : Self = Self::_PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb;
    #[allow(non_upper_case_globals)]
    pub const SparcH34 : Self = Self::_PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb;
    #[allow(non_upper_case_globals)]
    pub const ArmMovtBrel : Self = Self::_PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb;
    /// 16 bits of eff. address.
    #[allow(non_upper_case_globals)]
    pub const PariscDir16F : Self = Self::_PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb;
    /// half16	(sym+add)@got@tlsld@h
    #[allow(non_upper_case_globals)]
    pub const PpcGotTlsld16Hi : Self = Self::_PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb;
    /// half16	(sym+add)@got@tlsld@h
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotTlsld16Hi : Self = Self::_PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb;
    /// X0 pipe 16-bit TLS LE offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0TlsLe : Self = Self::_PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb;
    /// @ltoff(@fptr(s+a)), data4 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64LtoffFptr32Lsb : Self = Self::_PowerpcGotTlsld16HiOrTilegxImm16X1Hw1LastTlsLeOrSparcH34OrArmMovtBrelOrPariscDir16FOrPpcGotTlsld16HiOrPpc64GotTlsld16HiOrTileproImm16X0TlsLeOrIa64LtoffFptr32Lsb;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGotTlsld16Ha : Self = Self::_PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw0LastTlsGd : Self = Self::_PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb;
    #[allow(non_upper_case_globals)]
    pub const SparcSize32 : Self = Self::_PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb;
    #[allow(non_upper_case_globals)]
    pub const ArmMovwBrel : Self = Self::_PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb;
    /// 16 bits of eff. address.
    #[allow(non_upper_case_globals)]
    pub const PariscDir16Wf : Self = Self::_PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb;
    /// half16	(sym+add)@got@tlsld@ha
    #[allow(non_upper_case_globals)]
    pub const PpcGotTlsld16Ha : Self = Self::_PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb;
    /// half16	(sym+add)@got@tlsld@ha
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotTlsld16Ha : Self = Self::_PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb;
    /// X1 pipe 16-bit TLS LE offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1TlsLe : Self = Self::_PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb;
    /// @ltoff(@fptr(s+a)), data8 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64LtoffFptr64Msb : Self = Self::_PowerpcGotTlsld16HaOrTilegxImm16X0Hw0LastTlsGdOrSparcSize32OrArmMovwBrelOrPariscDir16WfOrPpcGotTlsld16HaOrPpc64GotTlsld16HaOrTileproImm16X1TlsLeOrIa64LtoffFptr64Msb;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGotTprel16 : Self = Self::_PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw0LastTlsGd : Self = Self::_PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb;
    #[allow(non_upper_case_globals)]
    pub const SparcSize64 : Self = Self::_PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb;
    #[allow(non_upper_case_globals)]
    pub const ArmThmMovwBrelNc : Self = Self::_PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb;
    /// 16 bits of eff. address.
    #[allow(non_upper_case_globals)]
    pub const PariscDir16Df : Self = Self::_PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb;
    /// half16*	(sym+add)@got@tprel
    #[allow(non_upper_case_globals)]
    pub const PpcGotTprel16 : Self = Self::_PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb;
    /// half16ds*	(sym+add)@got@tprel
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotTprel16Ds : Self = Self::_PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb;
    /// X0 pipe low 16-bit TLS LE offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0TlsLeLo : Self = Self::_PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb;
    /// @ltoff(@fptr(s+a)), data8 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64LtoffFptr64Lsb : Self = Self::_PowerpcGotTprel16OrTilegxImm16X1Hw0LastTlsGdOrSparcSize64OrArmThmMovwBrelNcOrPariscDir16DfOrPpcGotTprel16OrPpc64GotTprel16DsOrTileproImm16X0TlsLeLoOrIa64LtoffFptr64Lsb;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGotTprel16Lo : Self = Self::_PowerpcGotTprel16LoOrTilegxImm16X0Hw1LastTlsGdOrSparcWdisp10OrArmThmMovtBrelOrPariscGprel64OrPpcGotTprel16LoOrPpc64GotTprel16LoDsOrTileproImm16X1TlsLeLo;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw1LastTlsGd : Self = Self::_PowerpcGotTprel16LoOrTilegxImm16X0Hw1LastTlsGdOrSparcWdisp10OrArmThmMovtBrelOrPariscGprel64OrPpcGotTprel16LoOrPpc64GotTprel16LoDsOrTileproImm16X1TlsLeLo;
    #[allow(non_upper_case_globals)]
    pub const SparcWdisp10 : Self = Self::_PowerpcGotTprel16LoOrTilegxImm16X0Hw1LastTlsGdOrSparcWdisp10OrArmThmMovtBrelOrPariscGprel64OrPpcGotTprel16LoOrPpc64GotTprel16LoDsOrTileproImm16X1TlsLeLo;
    #[allow(non_upper_case_globals)]
    pub const ArmThmMovtBrel : Self = Self::_PowerpcGotTprel16LoOrTilegxImm16X0Hw1LastTlsGdOrSparcWdisp10OrArmThmMovtBrelOrPariscGprel64OrPpcGotTprel16LoOrPpc64GotTprel16LoDsOrTileproImm16X1TlsLeLo;
    /// 64 bits of GP-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscGprel64 : Self = Self::_PowerpcGotTprel16LoOrTilegxImm16X0Hw1LastTlsGdOrSparcWdisp10OrArmThmMovtBrelOrPariscGprel64OrPpcGotTprel16LoOrPpc64GotTprel16LoDsOrTileproImm16X1TlsLeLo;
    /// half16	(sym+add)@got@tprel@l
    #[allow(non_upper_case_globals)]
    pub const PpcGotTprel16Lo : Self = Self::_PowerpcGotTprel16LoOrTilegxImm16X0Hw1LastTlsGdOrSparcWdisp10OrArmThmMovtBrelOrPariscGprel64OrPpcGotTprel16LoOrPpc64GotTprel16LoDsOrTileproImm16X1TlsLeLo;
    /// half16ds (sym+add)@got@tprel@l
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotTprel16LoDs : Self = Self::_PowerpcGotTprel16LoOrTilegxImm16X0Hw1LastTlsGdOrSparcWdisp10OrArmThmMovtBrelOrPariscGprel64OrPpcGotTprel16LoOrPpc64GotTprel16LoDsOrTileproImm16X1TlsLeLo;
    /// X1 pipe low 16-bit TLS LE offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1TlsLeLo : Self = Self::_PowerpcGotTprel16LoOrTilegxImm16X0Hw1LastTlsGdOrSparcWdisp10OrArmThmMovtBrelOrPariscGprel64OrPpcGotTprel16LoOrPpc64GotTprel16LoDsOrTileproImm16X1TlsLeLo;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGotTprel16Hi : Self = Self::_PowerpcGotTprel16HiOrTilegxImm16X1Hw1LastTlsGdOrArmThmMovwBrelOrPpcGotTprel16HiOrPpc64GotTprel16HiOrTileproImm16X0TlsLeHi;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw1LastTlsGd : Self = Self::_PowerpcGotTprel16HiOrTilegxImm16X1Hw1LastTlsGdOrArmThmMovwBrelOrPpcGotTprel16HiOrPpc64GotTprel16HiOrTileproImm16X0TlsLeHi;
    #[allow(non_upper_case_globals)]
    pub const ArmThmMovwBrel : Self = Self::_PowerpcGotTprel16HiOrTilegxImm16X1Hw1LastTlsGdOrArmThmMovwBrelOrPpcGotTprel16HiOrPpc64GotTprel16HiOrTileproImm16X0TlsLeHi;
    /// half16	(sym+add)@got@tprel@h
    #[allow(non_upper_case_globals)]
    pub const PpcGotTprel16Hi : Self = Self::_PowerpcGotTprel16HiOrTilegxImm16X1Hw1LastTlsGdOrArmThmMovwBrelOrPpcGotTprel16HiOrPpc64GotTprel16HiOrTileproImm16X0TlsLeHi;
    /// half16	(sym+add)@got@tprel@h
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotTprel16Hi : Self = Self::_PowerpcGotTprel16HiOrTilegxImm16X1Hw1LastTlsGdOrArmThmMovwBrelOrPpcGotTprel16HiOrPpc64GotTprel16HiOrTileproImm16X0TlsLeHi;
    /// X0 pipe high 16-bit TLS LE offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0TlsLeHi : Self = Self::_PowerpcGotTprel16HiOrTilegxImm16X1Hw1LastTlsGdOrArmThmMovwBrelOrPpcGotTprel16HiOrPpc64GotTprel16HiOrTileproImm16X0TlsLeHi;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGotTprel16Ha : Self = Self::_PowerpcGotTprel16HaOrTilegxIrelativeOrArmTlsGotdescOrPpcGotTprel16HaOrPpc64GotTprel16HaOrTileproImm16X1TlsLeHi;
    #[allow(non_upper_case_globals)]
    pub const TilegxIrelative : Self = Self::_PowerpcGotTprel16HaOrTilegxIrelativeOrArmTlsGotdescOrPpcGotTprel16HaOrPpc64GotTprel16HaOrTileproImm16X1TlsLeHi;
    #[allow(non_upper_case_globals)]
    pub const ArmTlsGotdesc : Self = Self::_PowerpcGotTprel16HaOrTilegxIrelativeOrArmTlsGotdescOrPpcGotTprel16HaOrPpc64GotTprel16HaOrTileproImm16X1TlsLeHi;
    /// half16	(sym+add)@got@tprel@ha
    #[allow(non_upper_case_globals)]
    pub const PpcGotTprel16Ha : Self = Self::_PowerpcGotTprel16HaOrTilegxIrelativeOrArmTlsGotdescOrPpcGotTprel16HaOrPpc64GotTprel16HaOrTileproImm16X1TlsLeHi;
    /// half16	(sym+add)@got@tprel@ha
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotTprel16Ha : Self = Self::_PowerpcGotTprel16HaOrTilegxIrelativeOrArmTlsGotdescOrPpcGotTprel16HaOrPpc64GotTprel16HaOrTileproImm16X1TlsLeHi;
    /// X1 pipe high 16-bit TLS LE offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1TlsLeHi : Self = Self::_PowerpcGotTprel16HaOrTilegxIrelativeOrArmTlsGotdescOrPpcGotTprel16HaOrPpc64GotTprel16HaOrTileproImm16X1TlsLeHi;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGotDtprel16 : Self = Self::_PowerpcGotDtprel16OrArmTlsCallOrPariscGprel14WrOrPpcGotDtprel16OrPpc64GotDtprel16DsOrTileproImm16X0TlsLeHa;
    #[allow(non_upper_case_globals)]
    pub const ArmTlsCall : Self = Self::_PowerpcGotDtprel16OrArmTlsCallOrPariscGprel14WrOrPpcGotDtprel16OrPpc64GotDtprel16DsOrTileproImm16X0TlsLeHa;
    /// GP-rel. address, right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscGprel14Wr : Self = Self::_PowerpcGotDtprel16OrArmTlsCallOrPariscGprel14WrOrPpcGotDtprel16OrPpc64GotDtprel16DsOrTileproImm16X0TlsLeHa;
    /// half16*	(sym+add)@got@dtprel
    #[allow(non_upper_case_globals)]
    pub const PpcGotDtprel16 : Self = Self::_PowerpcGotDtprel16OrArmTlsCallOrPariscGprel14WrOrPpcGotDtprel16OrPpc64GotDtprel16DsOrTileproImm16X0TlsLeHa;
    /// half16ds*	(sym+add)@got@dtprel
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotDtprel16Ds : Self = Self::_PowerpcGotDtprel16OrArmTlsCallOrPariscGprel14WrOrPpcGotDtprel16OrPpc64GotDtprel16DsOrTileproImm16X0TlsLeHa;
    /// X0 pipe ha() 16-bit TLS LE offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X0TlsLeHa : Self = Self::_PowerpcGotDtprel16OrArmTlsCallOrPariscGprel14WrOrPpcGotDtprel16OrPpc64GotDtprel16DsOrTileproImm16X0TlsLeHa;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGotDtprel16Lo : Self = Self::_PowerpcGotDtprel16LoOrTilegxImm16X0Hw0TlsIeOrArmTlsDescseqOrPariscGprel14DrOrPpcGotDtprel16LoOrPpc64GotDtprel16LoDsOrTileproImm16X1TlsLeHaOrIa64Segrel32Msb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw0TlsIe : Self = Self::_PowerpcGotDtprel16LoOrTilegxImm16X0Hw0TlsIeOrArmTlsDescseqOrPariscGprel14DrOrPpcGotDtprel16LoOrPpc64GotDtprel16LoDsOrTileproImm16X1TlsLeHaOrIa64Segrel32Msb;
    #[allow(non_upper_case_globals)]
    pub const ArmTlsDescseq : Self = Self::_PowerpcGotDtprel16LoOrTilegxImm16X0Hw0TlsIeOrArmTlsDescseqOrPariscGprel14DrOrPpcGotDtprel16LoOrPpc64GotDtprel16LoDsOrTileproImm16X1TlsLeHaOrIa64Segrel32Msb;
    /// GP-rel. address, right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscGprel14Dr : Self = Self::_PowerpcGotDtprel16LoOrTilegxImm16X0Hw0TlsIeOrArmTlsDescseqOrPariscGprel14DrOrPpcGotDtprel16LoOrPpc64GotDtprel16LoDsOrTileproImm16X1TlsLeHaOrIa64Segrel32Msb;
    /// half16*	(sym+add)@got@dtprel@l
    #[allow(non_upper_case_globals)]
    pub const PpcGotDtprel16Lo : Self = Self::_PowerpcGotDtprel16LoOrTilegxImm16X0Hw0TlsIeOrArmTlsDescseqOrPariscGprel14DrOrPpcGotDtprel16LoOrPpc64GotDtprel16LoDsOrTileproImm16X1TlsLeHaOrIa64Segrel32Msb;
    /// half16ds (sym+add)@got@dtprel@l
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotDtprel16LoDs : Self = Self::_PowerpcGotDtprel16LoOrTilegxImm16X0Hw0TlsIeOrArmTlsDescseqOrPariscGprel14DrOrPpcGotDtprel16LoOrPpc64GotDtprel16LoDsOrTileproImm16X1TlsLeHaOrIa64Segrel32Msb;
    /// X1 pipe ha() 16-bit TLS LE offset
    #[allow(non_upper_case_globals)]
    pub const TileproImm16X1TlsLeHa : Self = Self::_PowerpcGotDtprel16LoOrTilegxImm16X0Hw0TlsIeOrArmTlsDescseqOrPariscGprel14DrOrPpcGotDtprel16LoOrPpc64GotDtprel16LoDsOrTileproImm16X1TlsLeHaOrIa64Segrel32Msb;
    /// @segrel(sym + add), data4 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Segrel32Msb : Self = Self::_PowerpcGotDtprel16LoOrTilegxImm16X0Hw0TlsIeOrArmTlsDescseqOrPariscGprel14DrOrPpcGotDtprel16LoOrPpc64GotDtprel16LoDsOrTileproImm16X1TlsLeHaOrIa64Segrel32Msb;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGotDtprel16Hi : Self = Self::_PowerpcGotDtprel16HiOrTilegxImm16X1Hw0TlsIeOrArmThmTlsCallOrPariscGprel16FOrPpcGotDtprel16HiOrPpc64GotDtprel16HiOrIa64Segrel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw0TlsIe : Self = Self::_PowerpcGotDtprel16HiOrTilegxImm16X1Hw0TlsIeOrArmThmTlsCallOrPariscGprel16FOrPpcGotDtprel16HiOrPpc64GotDtprel16HiOrIa64Segrel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const ArmThmTlsCall : Self = Self::_PowerpcGotDtprel16HiOrTilegxImm16X1Hw0TlsIeOrArmThmTlsCallOrPariscGprel16FOrPpcGotDtprel16HiOrPpc64GotDtprel16HiOrIa64Segrel32Lsb;
    /// 16 bits GP-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscGprel16F : Self = Self::_PowerpcGotDtprel16HiOrTilegxImm16X1Hw0TlsIeOrArmThmTlsCallOrPariscGprel16FOrPpcGotDtprel16HiOrPpc64GotDtprel16HiOrIa64Segrel32Lsb;
    /// half16*	(sym+add)@got@dtprel@h
    #[allow(non_upper_case_globals)]
    pub const PpcGotDtprel16Hi : Self = Self::_PowerpcGotDtprel16HiOrTilegxImm16X1Hw0TlsIeOrArmThmTlsCallOrPariscGprel16FOrPpcGotDtprel16HiOrPpc64GotDtprel16HiOrIa64Segrel32Lsb;
    /// half16	(sym+add)@got@dtprel@h
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotDtprel16Hi : Self = Self::_PowerpcGotDtprel16HiOrTilegxImm16X1Hw0TlsIeOrArmThmTlsCallOrPariscGprel16FOrPpcGotDtprel16HiOrPpc64GotDtprel16HiOrIa64Segrel32Lsb;
    /// @segrel(sym + add), data4 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Segrel32Lsb : Self = Self::_PowerpcGotDtprel16HiOrTilegxImm16X1Hw0TlsIeOrArmThmTlsCallOrPariscGprel16FOrPpcGotDtprel16HiOrPpc64GotDtprel16HiOrIa64Segrel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGotDtprel16Ha : Self = Self::_PowerpcGotDtprel16HaOrTilegxImm16X0Hw0LastPltPcrelOrArmPlt32AbsOrPariscGprel16WfOrPpcGotDtprel16HaOrPpc64GotDtprel16HaOrIa64Segrel64Msb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw0LastPltPcrel : Self = Self::_PowerpcGotDtprel16HaOrTilegxImm16X0Hw0LastPltPcrelOrArmPlt32AbsOrPariscGprel16WfOrPpcGotDtprel16HaOrPpc64GotDtprel16HaOrIa64Segrel64Msb;
    #[allow(non_upper_case_globals)]
    pub const ArmPlt32Abs : Self = Self::_PowerpcGotDtprel16HaOrTilegxImm16X0Hw0LastPltPcrelOrArmPlt32AbsOrPariscGprel16WfOrPpcGotDtprel16HaOrPpc64GotDtprel16HaOrIa64Segrel64Msb;
    /// 16 bits GP-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscGprel16Wf : Self = Self::_PowerpcGotDtprel16HaOrTilegxImm16X0Hw0LastPltPcrelOrArmPlt32AbsOrPariscGprel16WfOrPpcGotDtprel16HaOrPpc64GotDtprel16HaOrIa64Segrel64Msb;
    /// half16*	(sym+add)@got@dtprel@ha
    #[allow(non_upper_case_globals)]
    pub const PpcGotDtprel16Ha : Self = Self::_PowerpcGotDtprel16HaOrTilegxImm16X0Hw0LastPltPcrelOrArmPlt32AbsOrPariscGprel16WfOrPpcGotDtprel16HaOrPpc64GotDtprel16HaOrIa64Segrel64Msb;
    /// half16	(sym+add)@got@dtprel@ha
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotDtprel16Ha : Self = Self::_PowerpcGotDtprel16HaOrTilegxImm16X0Hw0LastPltPcrelOrArmPlt32AbsOrPariscGprel16WfOrPpcGotDtprel16HaOrPpc64GotDtprel16HaOrIa64Segrel64Msb;
    /// @segrel(sym + add), data8 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Segrel64Msb : Self = Self::_PowerpcGotDtprel16HaOrTilegxImm16X0Hw0LastPltPcrelOrArmPlt32AbsOrPariscGprel16WfOrPpcGotDtprel16HaOrPpc64GotDtprel16HaOrIa64Segrel64Msb;
    #[allow(non_upper_case_globals)]
    pub const PpcTlsgd : Self = Self::_PpcTlsgdOrPpc64Tprel16DsOrTilegxImm16X1Hw0LastPltPcrelOrArmGotAbsOrPariscGprel16DfOrIa64Segrel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Tprel16Ds : Self = Self::_PpcTlsgdOrPpc64Tprel16DsOrTilegxImm16X1Hw0LastPltPcrelOrArmGotAbsOrPariscGprel16DfOrIa64Segrel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw0LastPltPcrel : Self = Self::_PpcTlsgdOrPpc64Tprel16DsOrTilegxImm16X1Hw0LastPltPcrelOrArmGotAbsOrPariscGprel16DfOrIa64Segrel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const ArmGotAbs : Self = Self::_PpcTlsgdOrPpc64Tprel16DsOrTilegxImm16X1Hw0LastPltPcrelOrArmGotAbsOrPariscGprel16DfOrIa64Segrel64Lsb;
    /// 16 bits GP-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscGprel16Df : Self = Self::_PpcTlsgdOrPpc64Tprel16DsOrTilegxImm16X1Hw0LastPltPcrelOrArmGotAbsOrPariscGprel16DfOrIa64Segrel64Lsb;
    /// @segrel(sym + add), data8 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Segrel64Lsb : Self = Self::_PpcTlsgdOrPpc64Tprel16DsOrTilegxImm16X1Hw0LastPltPcrelOrArmGotAbsOrPariscGprel16DfOrIa64Segrel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const PpcTlsld: Self =
        Self::_PpcTlsldOrPpc64Tprel16LoDsOrTilegxImm16X0Hw1LastPltPcrelOrArmGotPrelOrPariscLtoff64;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Tprel16LoDs: Self =
        Self::_PpcTlsldOrPpc64Tprel16LoDsOrTilegxImm16X0Hw1LastPltPcrelOrArmGotPrelOrPariscLtoff64;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw1LastPltPcrel: Self =
        Self::_PpcTlsldOrPpc64Tprel16LoDsOrTilegxImm16X0Hw1LastPltPcrelOrArmGotPrelOrPariscLtoff64;
    #[allow(non_upper_case_globals)]
    pub const ArmGotPrel: Self =
        Self::_PpcTlsldOrPpc64Tprel16LoDsOrTilegxImm16X0Hw1LastPltPcrelOrArmGotPrelOrPariscLtoff64;
    /// 64 bits LT-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoff64: Self =
        Self::_PpcTlsldOrPpc64Tprel16LoDsOrTilegxImm16X0Hw1LastPltPcrelOrArmGotPrelOrPariscLtoff64;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Tprel16Higher: Self =
        Self::_Ppc64Tprel16HigherOrTilegxImm16X1Hw1LastPltPcrelOrArmGotBrel12;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw1LastPltPcrel: Self =
        Self::_Ppc64Tprel16HigherOrTilegxImm16X1Hw1LastPltPcrelOrArmGotBrel12;
    #[allow(non_upper_case_globals)]
    pub const ArmGotBrel12: Self =
        Self::_Ppc64Tprel16HigherOrTilegxImm16X1Hw1LastPltPcrelOrArmGotBrel12;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Tprel16Highera: Self =
        Self::_Ppc64Tprel16HigheraOrTilegxImm16X0Hw2LastPltPcrelOrArmGotoff12;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw2LastPltPcrel: Self =
        Self::_Ppc64Tprel16HigheraOrTilegxImm16X0Hw2LastPltPcrelOrArmGotoff12;
    #[allow(non_upper_case_globals)]
    pub const ArmGotoff12: Self =
        Self::_Ppc64Tprel16HigheraOrTilegxImm16X0Hw2LastPltPcrelOrArmGotoff12;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Tprel16Highest: Self =
        Self::_Ppc64Tprel16HighestOrTilegxImm16X1Hw2LastPltPcrelOrArmGotrelaxOrPariscLtoff14Wr;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw2LastPltPcrel: Self =
        Self::_Ppc64Tprel16HighestOrTilegxImm16X1Hw2LastPltPcrelOrArmGotrelaxOrPariscLtoff14Wr;
    #[allow(non_upper_case_globals)]
    pub const ArmGotrelax: Self =
        Self::_Ppc64Tprel16HighestOrTilegxImm16X1Hw2LastPltPcrelOrArmGotrelaxOrPariscLtoff14Wr;
    /// LT-rel. address, right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoff14Wr: Self =
        Self::_Ppc64Tprel16HighestOrTilegxImm16X1Hw2LastPltPcrelOrArmGotrelaxOrPariscLtoff14Wr;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Tprel16Highesta : Self = Self::_Ppc64Tprel16HighestaOrTilegxImm16X0Hw0LastTlsIeOrArmGnuVtentryOrPariscLtoff14DrOrIa64Secrel32Msb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw0LastTlsIe : Self = Self::_Ppc64Tprel16HighestaOrTilegxImm16X0Hw0LastTlsIeOrArmGnuVtentryOrPariscLtoff14DrOrIa64Secrel32Msb;
    #[allow(non_upper_case_globals)]
    pub const ArmGnuVtentry : Self = Self::_Ppc64Tprel16HighestaOrTilegxImm16X0Hw0LastTlsIeOrArmGnuVtentryOrPariscLtoff14DrOrIa64Secrel32Msb;
    /// LT-rel. address, right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoff14Dr : Self = Self::_Ppc64Tprel16HighestaOrTilegxImm16X0Hw0LastTlsIeOrArmGnuVtentryOrPariscLtoff14DrOrIa64Secrel32Msb;
    /// @secrel(sym + add), data4 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Secrel32Msb : Self = Self::_Ppc64Tprel16HighestaOrTilegxImm16X0Hw0LastTlsIeOrArmGnuVtentryOrPariscLtoff14DrOrIa64Secrel32Msb;
    #[allow(non_upper_case_globals)]
    pub const PpcEmbNaddr32 : Self = Self::_PpcEmbNaddr32OrPpc64Dtprel16DsOrMips16GprelOrTilegxImm16X1Hw0LastTlsIeOrArmGnuVtinheritOrPariscLtoff16FOrIa64Secrel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Dtprel16Ds : Self = Self::_PpcEmbNaddr32OrPpc64Dtprel16DsOrMips16GprelOrTilegxImm16X1Hw0LastTlsIeOrArmGnuVtinheritOrPariscLtoff16FOrIa64Secrel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const Mips16Gprel : Self = Self::_PpcEmbNaddr32OrPpc64Dtprel16DsOrMips16GprelOrTilegxImm16X1Hw0LastTlsIeOrArmGnuVtinheritOrPariscLtoff16FOrIa64Secrel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw0LastTlsIe : Self = Self::_PpcEmbNaddr32OrPpc64Dtprel16DsOrMips16GprelOrTilegxImm16X1Hw0LastTlsIeOrArmGnuVtinheritOrPariscLtoff16FOrIa64Secrel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const ArmGnuVtinherit : Self = Self::_PpcEmbNaddr32OrPpc64Dtprel16DsOrMips16GprelOrTilegxImm16X1Hw0LastTlsIeOrArmGnuVtinheritOrPariscLtoff16FOrIa64Secrel32Lsb;
    /// 16 bits LT-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoff16F : Self = Self::_PpcEmbNaddr32OrPpc64Dtprel16DsOrMips16GprelOrTilegxImm16X1Hw0LastTlsIeOrArmGnuVtinheritOrPariscLtoff16FOrIa64Secrel32Lsb;
    /// @secrel(sym + add), data4 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Secrel32Lsb : Self = Self::_PpcEmbNaddr32OrPpc64Dtprel16DsOrMips16GprelOrTilegxImm16X1Hw0LastTlsIeOrArmGnuVtinheritOrPariscLtoff16FOrIa64Secrel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const PpcEmbNaddr16 : Self = Self::_PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Dtprel16LoDs : Self = Self::_PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb;
    #[allow(non_upper_case_globals)]
    pub const Mips16Got16 : Self = Self::_PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X0Hw1LastTlsIe : Self = Self::_PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb;
    #[allow(non_upper_case_globals)]
    pub const ArmThmJump11 : Self = Self::_PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb;
    /// 16 bits LT-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoff16Wf : Self = Self::_PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb;
    /// PC relative & 0xFFE (Thumb16 B).
    #[allow(non_upper_case_globals)]
    pub const ArmThmPc11 : Self = Self::_PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb;
    #[allow(non_upper_case_globals)]
    pub const Nds32TlsTpoff : Self = Self::_PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb;
    /// @secrel(sym + add), data8 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Secrel64Msb : Self = Self::_PpcEmbNaddr16OrPpc64Dtprel16LoDsOrMips16Got16OrTilegxImm16X0Hw1LastTlsIeOrArmThmJump11OrPariscLtoff16WfOrArmThmPc11OrNds32TlsTpoffOrIa64Secrel64Msb;
    #[allow(non_upper_case_globals)]
    pub const PpcEmbNaddr16Lo : Self = Self::_PpcEmbNaddr16LoOrPpc64Dtprel16HigherOrMips16Call16OrTilegxImm16X1Hw1LastTlsIeOrArmThmJump8OrPariscLtoff16DfOrArmThmPc9OrIa64Secrel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Dtprel16Higher : Self = Self::_PpcEmbNaddr16LoOrPpc64Dtprel16HigherOrMips16Call16OrTilegxImm16X1Hw1LastTlsIeOrArmThmJump8OrPariscLtoff16DfOrArmThmPc9OrIa64Secrel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const Mips16Call16 : Self = Self::_PpcEmbNaddr16LoOrPpc64Dtprel16HigherOrMips16Call16OrTilegxImm16X1Hw1LastTlsIeOrArmThmJump8OrPariscLtoff16DfOrArmThmPc9OrIa64Secrel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm16X1Hw1LastTlsIe : Self = Self::_PpcEmbNaddr16LoOrPpc64Dtprel16HigherOrMips16Call16OrTilegxImm16X1Hw1LastTlsIeOrArmThmJump8OrPariscLtoff16DfOrArmThmPc9OrIa64Secrel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const ArmThmJump8 : Self = Self::_PpcEmbNaddr16LoOrPpc64Dtprel16HigherOrMips16Call16OrTilegxImm16X1Hw1LastTlsIeOrArmThmJump8OrPariscLtoff16DfOrArmThmPc9OrIa64Secrel64Lsb;
    /// 16 bits LT-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoff16Df : Self = Self::_PpcEmbNaddr16LoOrPpc64Dtprel16HigherOrMips16Call16OrTilegxImm16X1Hw1LastTlsIeOrArmThmJump8OrPariscLtoff16DfOrArmThmPc9OrIa64Secrel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const ArmThmPc9 : Self = Self::_PpcEmbNaddr16LoOrPpc64Dtprel16HigherOrMips16Call16OrTilegxImm16X1Hw1LastTlsIeOrArmThmJump8OrPariscLtoff16DfOrArmThmPc9OrIa64Secrel64Lsb;
    /// @secrel(sym + add), data8 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Secrel64Lsb : Self = Self::_PpcEmbNaddr16LoOrPpc64Dtprel16HigherOrMips16Call16OrTilegxImm16X1Hw1LastTlsIeOrArmThmJump8OrPariscLtoff16DfOrArmThmPc9OrIa64Secrel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const PpcEmbNaddr16Hi: Self =
        Self::_PpcEmbNaddr16HiOrPpc64Dtprel16HigheraOrMips16Hi16OrArmTlsGd32OrPariscSecrel64;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Dtprel16Highera: Self =
        Self::_PpcEmbNaddr16HiOrPpc64Dtprel16HigheraOrMips16Hi16OrArmTlsGd32OrPariscSecrel64;
    #[allow(non_upper_case_globals)]
    pub const Mips16Hi16: Self =
        Self::_PpcEmbNaddr16HiOrPpc64Dtprel16HigheraOrMips16Hi16OrArmTlsGd32OrPariscSecrel64;
    #[allow(non_upper_case_globals)]
    pub const ArmTlsGd32: Self =
        Self::_PpcEmbNaddr16HiOrPpc64Dtprel16HigheraOrMips16Hi16OrArmTlsGd32OrPariscSecrel64;
    /// 64 bits section rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscSecrel64: Self =
        Self::_PpcEmbNaddr16HiOrPpc64Dtprel16HigheraOrMips16Hi16OrArmTlsGd32OrPariscSecrel64;
    #[allow(non_upper_case_globals)]
    pub const PpcEmbNaddr16Ha: Self =
        Self::_PpcEmbNaddr16HaOrPpc64Dtprel16HighestOrMips16Lo16OrArmTlsLdm32;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Dtprel16Highest: Self =
        Self::_PpcEmbNaddr16HaOrPpc64Dtprel16HighestOrMips16Lo16OrArmTlsLdm32;
    #[allow(non_upper_case_globals)]
    pub const Mips16Lo16: Self =
        Self::_PpcEmbNaddr16HaOrPpc64Dtprel16HighestOrMips16Lo16OrArmTlsLdm32;
    #[allow(non_upper_case_globals)]
    pub const ArmTlsLdm32: Self =
        Self::_PpcEmbNaddr16HaOrPpc64Dtprel16HighestOrMips16Lo16OrArmTlsLdm32;
    #[allow(non_upper_case_globals)]
    pub const PpcEmbSdai16: Self =
        Self::_PpcEmbSdai16OrPpc64Dtprel16HighestaOrMips16TlsGdOrTilegxTlsDtpmod64OrArmTlsLdo32;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Dtprel16Highesta: Self =
        Self::_PpcEmbSdai16OrPpc64Dtprel16HighestaOrMips16TlsGdOrTilegxTlsDtpmod64OrArmTlsLdo32;
    #[allow(non_upper_case_globals)]
    pub const Mips16TlsGd: Self =
        Self::_PpcEmbSdai16OrPpc64Dtprel16HighestaOrMips16TlsGdOrTilegxTlsDtpmod64OrArmTlsLdo32;
    #[allow(non_upper_case_globals)]
    pub const TilegxTlsDtpmod64: Self =
        Self::_PpcEmbSdai16OrPpc64Dtprel16HighestaOrMips16TlsGdOrTilegxTlsDtpmod64OrArmTlsLdo32;
    #[allow(non_upper_case_globals)]
    pub const ArmTlsLdo32: Self =
        Self::_PpcEmbSdai16OrPpc64Dtprel16HighestaOrMips16TlsGdOrTilegxTlsDtpmod64OrArmTlsLdo32;
    #[allow(non_upper_case_globals)]
    pub const PpcEmbSda2I16: Self =
        Self::_PpcEmbSda2I16OrPpc64TlsgdOrMips16TlsLdmOrTilegxTlsDtpoff64OrArmTlsIe32;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Tlsgd: Self =
        Self::_PpcEmbSda2I16OrPpc64TlsgdOrMips16TlsLdmOrTilegxTlsDtpoff64OrArmTlsIe32;
    #[allow(non_upper_case_globals)]
    pub const Mips16TlsLdm: Self =
        Self::_PpcEmbSda2I16OrPpc64TlsgdOrMips16TlsLdmOrTilegxTlsDtpoff64OrArmTlsIe32;
    #[allow(non_upper_case_globals)]
    pub const TilegxTlsDtpoff64: Self =
        Self::_PpcEmbSda2I16OrPpc64TlsgdOrMips16TlsLdmOrTilegxTlsDtpoff64OrArmTlsIe32;
    #[allow(non_upper_case_globals)]
    pub const ArmTlsIe32: Self =
        Self::_PpcEmbSda2I16OrPpc64TlsgdOrMips16TlsLdmOrTilegxTlsDtpoff64OrArmTlsIe32;
    #[allow(non_upper_case_globals)]
    pub const PpcEmbSda2Rel : Self = Self::_PpcEmbSda2RelOrPpc64TlsldOrMips16TlsDtprelHi16OrTilegxTlsTpoff64OrArmTlsLe32OrIa64Rel32Msb;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Tlsld : Self = Self::_PpcEmbSda2RelOrPpc64TlsldOrMips16TlsDtprelHi16OrTilegxTlsTpoff64OrArmTlsLe32OrIa64Rel32Msb;
    #[allow(non_upper_case_globals)]
    pub const Mips16TlsDtprelHi16 : Self = Self::_PpcEmbSda2RelOrPpc64TlsldOrMips16TlsDtprelHi16OrTilegxTlsTpoff64OrArmTlsLe32OrIa64Rel32Msb;
    #[allow(non_upper_case_globals)]
    pub const TilegxTlsTpoff64 : Self = Self::_PpcEmbSda2RelOrPpc64TlsldOrMips16TlsDtprelHi16OrTilegxTlsTpoff64OrArmTlsLe32OrIa64Rel32Msb;
    #[allow(non_upper_case_globals)]
    pub const ArmTlsLe32 : Self = Self::_PpcEmbSda2RelOrPpc64TlsldOrMips16TlsDtprelHi16OrTilegxTlsTpoff64OrArmTlsLe32OrIa64Rel32Msb;
    /// data 4 + REL
    #[allow(non_upper_case_globals)]
    pub const Ia64Rel32Msb : Self = Self::_PpcEmbSda2RelOrPpc64TlsldOrMips16TlsDtprelHi16OrTilegxTlsTpoff64OrArmTlsLe32OrIa64Rel32Msb;
    #[allow(non_upper_case_globals)]
    pub const PpcEmbSda21 : Self = Self::_PpcEmbSda21OrPpc64TocsaveOrMips16TlsDtprelLo16OrTilegxTlsDtpmod32OrArmTlsLdo12OrIa64Rel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Tocsave : Self = Self::_PpcEmbSda21OrPpc64TocsaveOrMips16TlsDtprelLo16OrTilegxTlsDtpmod32OrArmTlsLdo12OrIa64Rel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const Mips16TlsDtprelLo16 : Self = Self::_PpcEmbSda21OrPpc64TocsaveOrMips16TlsDtprelLo16OrTilegxTlsDtpmod32OrArmTlsLdo12OrIa64Rel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const TilegxTlsDtpmod32 : Self = Self::_PpcEmbSda21OrPpc64TocsaveOrMips16TlsDtprelLo16OrTilegxTlsDtpmod32OrArmTlsLdo12OrIa64Rel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const ArmTlsLdo12 : Self = Self::_PpcEmbSda21OrPpc64TocsaveOrMips16TlsDtprelLo16OrTilegxTlsDtpmod32OrArmTlsLdo12OrIa64Rel32Lsb;
    /// data 4 + REL
    #[allow(non_upper_case_globals)]
    pub const Ia64Rel32Lsb : Self = Self::_PpcEmbSda21OrPpc64TocsaveOrMips16TlsDtprelLo16OrTilegxTlsDtpmod32OrArmTlsLdo12OrIa64Rel32Lsb;
    #[allow(non_upper_case_globals)]
    pub const PpcEmbMrkref : Self = Self::_PpcEmbMrkrefOrPpc64Addr16HighOrMips16TlsGottprelOrTilegxTlsDtpoff32OrArmTlsLe12OrIa64Rel64Msb;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr16High : Self = Self::_PpcEmbMrkrefOrPpc64Addr16HighOrMips16TlsGottprelOrTilegxTlsDtpoff32OrArmTlsLe12OrIa64Rel64Msb;
    #[allow(non_upper_case_globals)]
    pub const Mips16TlsGottprel : Self = Self::_PpcEmbMrkrefOrPpc64Addr16HighOrMips16TlsGottprelOrTilegxTlsDtpoff32OrArmTlsLe12OrIa64Rel64Msb;
    #[allow(non_upper_case_globals)]
    pub const TilegxTlsDtpoff32 : Self = Self::_PpcEmbMrkrefOrPpc64Addr16HighOrMips16TlsGottprelOrTilegxTlsDtpoff32OrArmTlsLe12OrIa64Rel64Msb;
    #[allow(non_upper_case_globals)]
    pub const ArmTlsLe12 : Self = Self::_PpcEmbMrkrefOrPpc64Addr16HighOrMips16TlsGottprelOrTilegxTlsDtpoff32OrArmTlsLe12OrIa64Rel64Msb;
    /// data 8 + REL
    #[allow(non_upper_case_globals)]
    pub const Ia64Rel64Msb : Self = Self::_PpcEmbMrkrefOrPpc64Addr16HighOrMips16TlsGottprelOrTilegxTlsDtpoff32OrArmTlsLe12OrIa64Rel64Msb;
    #[allow(non_upper_case_globals)]
    pub const PpcEmbRelsec16 : Self = Self::_PpcEmbRelsec16OrPpc64Addr16HighaOrMips16TlsTprelHi16OrTilegxTlsTpoff32OrArmTlsIe12GpOrIa64Rel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr16Higha : Self = Self::_PpcEmbRelsec16OrPpc64Addr16HighaOrMips16TlsTprelHi16OrTilegxTlsTpoff32OrArmTlsIe12GpOrIa64Rel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const Mips16TlsTprelHi16 : Self = Self::_PpcEmbRelsec16OrPpc64Addr16HighaOrMips16TlsTprelHi16OrTilegxTlsTpoff32OrArmTlsIe12GpOrIa64Rel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const TilegxTlsTpoff32 : Self = Self::_PpcEmbRelsec16OrPpc64Addr16HighaOrMips16TlsTprelHi16OrTilegxTlsTpoff32OrArmTlsIe12GpOrIa64Rel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const ArmTlsIe12Gp : Self = Self::_PpcEmbRelsec16OrPpc64Addr16HighaOrMips16TlsTprelHi16OrTilegxTlsTpoff32OrArmTlsIe12GpOrIa64Rel64Lsb;
    /// data 8 + REL
    #[allow(non_upper_case_globals)]
    pub const Ia64Rel64Lsb : Self = Self::_PpcEmbRelsec16OrPpc64Addr16HighaOrMips16TlsTprelHi16OrTilegxTlsTpoff32OrArmTlsIe12GpOrIa64Rel64Lsb;
    #[allow(non_upper_case_globals)]
    pub const PpcEmbRelstLo : Self = Self::_PpcEmbRelstLoOrPpc64Tprel16HighOrMips16TlsTprelLo16OrTilegxTlsGdCallOrArmPrivate0OrPariscSegrel64;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Tprel16High : Self = Self::_PpcEmbRelstLoOrPpc64Tprel16HighOrMips16TlsTprelLo16OrTilegxTlsGdCallOrArmPrivate0OrPariscSegrel64;
    #[allow(non_upper_case_globals)]
    pub const Mips16TlsTprelLo16 : Self = Self::_PpcEmbRelstLoOrPpc64Tprel16HighOrMips16TlsTprelLo16OrTilegxTlsGdCallOrArmPrivate0OrPariscSegrel64;
    #[allow(non_upper_case_globals)]
    pub const TilegxTlsGdCall : Self = Self::_PpcEmbRelstLoOrPpc64Tprel16HighOrMips16TlsTprelLo16OrTilegxTlsGdCallOrArmPrivate0OrPariscSegrel64;
    #[allow(non_upper_case_globals)]
    pub const ArmPrivate0 : Self = Self::_PpcEmbRelstLoOrPpc64Tprel16HighOrMips16TlsTprelLo16OrTilegxTlsGdCallOrArmPrivate0OrPariscSegrel64;
    /// 64 bits segment rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscSegrel64 : Self = Self::_PpcEmbRelstLoOrPpc64Tprel16HighOrMips16TlsTprelLo16OrTilegxTlsGdCallOrArmPrivate0OrPariscSegrel64;
    #[allow(non_upper_case_globals)]
    pub const PpcEmbRelstHi: Self =
        Self::_PpcEmbRelstHiOrPpc64Tprel16HighaOrArmPrivate1OrTilegxImm8X0TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Tprel16Higha: Self =
        Self::_PpcEmbRelstHiOrPpc64Tprel16HighaOrArmPrivate1OrTilegxImm8X0TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const ArmPrivate1: Self =
        Self::_PpcEmbRelstHiOrPpc64Tprel16HighaOrArmPrivate1OrTilegxImm8X0TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm8X0TlsGdAdd: Self =
        Self::_PpcEmbRelstHiOrPpc64Tprel16HighaOrArmPrivate1OrTilegxImm8X0TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const PpcEmbRelstHa: Self =
        Self::_PpcEmbRelstHaOrPpc64Dtprel16HighOrArmPrivate2OrTilegxImm8X1TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Dtprel16High: Self =
        Self::_PpcEmbRelstHaOrPpc64Dtprel16HighOrArmPrivate2OrTilegxImm8X1TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const ArmPrivate2: Self =
        Self::_PpcEmbRelstHaOrPpc64Dtprel16HighOrArmPrivate2OrTilegxImm8X1TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm8X1TlsGdAdd: Self =
        Self::_PpcEmbRelstHaOrPpc64Dtprel16HighOrArmPrivate2OrTilegxImm8X1TlsGdAdd;
    #[allow(non_upper_case_globals)]
    pub const PpcEmbBitFld : Self = Self::_PpcEmbBitFldOrPpc64Dtprel16HighaOrArmPrivate3OrTilegxImm8Y0TlsGdAddOrPariscPltoff14Wr;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Dtprel16Higha : Self = Self::_PpcEmbBitFldOrPpc64Dtprel16HighaOrArmPrivate3OrTilegxImm8Y0TlsGdAddOrPariscPltoff14Wr;
    #[allow(non_upper_case_globals)]
    pub const ArmPrivate3 : Self = Self::_PpcEmbBitFldOrPpc64Dtprel16HighaOrArmPrivate3OrTilegxImm8Y0TlsGdAddOrPariscPltoff14Wr;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm8Y0TlsGdAdd : Self = Self::_PpcEmbBitFldOrPpc64Dtprel16HighaOrArmPrivate3OrTilegxImm8Y0TlsGdAddOrPariscPltoff14Wr;
    /// PLT-rel. address, right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscPltoff14Wr : Self = Self::_PpcEmbBitFldOrPpc64Dtprel16HighaOrArmPrivate3OrTilegxImm8Y0TlsGdAddOrPariscPltoff14Wr;
    #[allow(non_upper_case_globals)]
    pub const PpcEmbRelsda : Self = Self::_PpcEmbRelsdaOrPpc64Rel24NotocOrArmPrivate4OrTilegxImm8Y1TlsGdAddOrPariscPltoff14DrOrIa64Ltv32Msb;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel24Notoc : Self = Self::_PpcEmbRelsdaOrPpc64Rel24NotocOrArmPrivate4OrTilegxImm8Y1TlsGdAddOrPariscPltoff14DrOrIa64Ltv32Msb;
    #[allow(non_upper_case_globals)]
    pub const ArmPrivate4 : Self = Self::_PpcEmbRelsdaOrPpc64Rel24NotocOrArmPrivate4OrTilegxImm8Y1TlsGdAddOrPariscPltoff14DrOrIa64Ltv32Msb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm8Y1TlsGdAdd : Self = Self::_PpcEmbRelsdaOrPpc64Rel24NotocOrArmPrivate4OrTilegxImm8Y1TlsGdAddOrPariscPltoff14DrOrIa64Ltv32Msb;
    /// PLT-rel. address, right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscPltoff14Dr : Self = Self::_PpcEmbRelsdaOrPpc64Rel24NotocOrArmPrivate4OrTilegxImm8Y1TlsGdAddOrPariscPltoff14DrOrIa64Ltv32Msb;
    /// symbol + addend, data4 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Ltv32Msb : Self = Self::_PpcEmbRelsdaOrPpc64Rel24NotocOrArmPrivate4OrTilegxImm8Y1TlsGdAddOrPariscPltoff14DrOrIa64Ltv32Msb;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr64Local: Self =
        Self::_Ppc64Addr64LocalOrArmPrivate5OrTilegxTlsIeLoadOrPariscPltoff16FOrIa64Ltv32Lsb;
    #[allow(non_upper_case_globals)]
    pub const ArmPrivate5: Self =
        Self::_Ppc64Addr64LocalOrArmPrivate5OrTilegxTlsIeLoadOrPariscPltoff16FOrIa64Ltv32Lsb;
    #[allow(non_upper_case_globals)]
    pub const TilegxTlsIeLoad: Self =
        Self::_Ppc64Addr64LocalOrArmPrivate5OrTilegxTlsIeLoadOrPariscPltoff16FOrIa64Ltv32Lsb;
    /// 16 bits LT-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscPltoff16F: Self =
        Self::_Ppc64Addr64LocalOrArmPrivate5OrTilegxTlsIeLoadOrPariscPltoff16FOrIa64Ltv32Lsb;
    /// symbol + addend, data4 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Ltv32Lsb: Self =
        Self::_Ppc64Addr64LocalOrArmPrivate5OrTilegxTlsIeLoadOrPariscPltoff16FOrIa64Ltv32Lsb;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Entry: Self =
        Self::_Ppc64EntryOrArmPrivate6OrTilegxImm8X0TlsAddOrPariscPltoff16WfOrIa64Ltv64Msb;
    #[allow(non_upper_case_globals)]
    pub const ArmPrivate6: Self =
        Self::_Ppc64EntryOrArmPrivate6OrTilegxImm8X0TlsAddOrPariscPltoff16WfOrIa64Ltv64Msb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm8X0TlsAdd: Self =
        Self::_Ppc64EntryOrArmPrivate6OrTilegxImm8X0TlsAddOrPariscPltoff16WfOrIa64Ltv64Msb;
    /// 16 bits PLT-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscPltoff16Wf: Self =
        Self::_Ppc64EntryOrArmPrivate6OrTilegxImm8X0TlsAddOrPariscPltoff16WfOrIa64Ltv64Msb;
    /// symbol + addend, data8 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Ltv64Msb: Self =
        Self::_Ppc64EntryOrArmPrivate6OrTilegxImm8X0TlsAddOrPariscPltoff16WfOrIa64Ltv64Msb;
    #[allow(non_upper_case_globals)]
    pub const PowerpcPltseq : Self = Self::_PowerpcPltseqOrArmPrivate7OrTilegxImm8X1TlsAddOrPariscPltoff16DfOrNds32TlsDescOrIa64Ltv64Lsb;
    #[allow(non_upper_case_globals)]
    pub const ArmPrivate7 : Self = Self::_PowerpcPltseqOrArmPrivate7OrTilegxImm8X1TlsAddOrPariscPltoff16DfOrNds32TlsDescOrIa64Ltv64Lsb;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm8X1TlsAdd : Self = Self::_PowerpcPltseqOrArmPrivate7OrTilegxImm8X1TlsAddOrPariscPltoff16DfOrNds32TlsDescOrIa64Ltv64Lsb;
    /// 16 bits PLT-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscPltoff16Df : Self = Self::_PowerpcPltseqOrArmPrivate7OrTilegxImm8X1TlsAddOrPariscPltoff16DfOrNds32TlsDescOrIa64Ltv64Lsb;
    #[allow(non_upper_case_globals)]
    pub const Nds32TlsDesc : Self = Self::_PowerpcPltseqOrArmPrivate7OrTilegxImm8X1TlsAddOrPariscPltoff16DfOrNds32TlsDescOrIa64Ltv64Lsb;
    /// symbol + addend, data8 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Ltv64Lsb : Self = Self::_PowerpcPltseqOrArmPrivate7OrTilegxImm8X1TlsAddOrPariscPltoff16DfOrNds32TlsDescOrIa64Ltv64Lsb;
    #[allow(non_upper_case_globals)]
    pub const PowerpcPltcall: Self =
        Self::_PowerpcPltcallOrArmPrivate8OrTilegxImm8Y0TlsAddOrPariscLtoffFptr64;
    #[allow(non_upper_case_globals)]
    pub const ArmPrivate8: Self =
        Self::_PowerpcPltcallOrArmPrivate8OrTilegxImm8Y0TlsAddOrPariscLtoffFptr64;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm8Y0TlsAdd: Self =
        Self::_PowerpcPltcallOrArmPrivate8OrTilegxImm8Y0TlsAddOrPariscLtoffFptr64;
    /// 64 bits LT-rel. function ptr.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoffFptr64: Self =
        Self::_PowerpcPltcallOrArmPrivate8OrTilegxImm8Y0TlsAddOrPariscLtoffFptr64;
    #[allow(non_upper_case_globals)]
    pub const Ppc64PltseqNotoc: Self =
        Self::_Ppc64PltseqNotocOrArmPrivate9OrTilegxImm8Y1TlsAddOrIa64Pcrel21Bi;
    #[allow(non_upper_case_globals)]
    pub const ArmPrivate9: Self =
        Self::_Ppc64PltseqNotocOrArmPrivate9OrTilegxImm8Y1TlsAddOrIa64Pcrel21Bi;
    #[allow(non_upper_case_globals)]
    pub const TilegxImm8Y1TlsAdd: Self =
        Self::_Ppc64PltseqNotocOrArmPrivate9OrTilegxImm8Y1TlsAddOrIa64Pcrel21Bi;
    /// @pcrel(sym + add), 21bit inst
    #[allow(non_upper_case_globals)]
    pub const Ia64Pcrel21Bi: Self =
        Self::_Ppc64PltseqNotocOrArmPrivate9OrTilegxImm8Y1TlsAddOrIa64Pcrel21Bi;
    #[allow(non_upper_case_globals)]
    pub const Ppc64PltcallNotoc: Self = Self::_Ppc64PltcallNotocOrArmPrivate10OrIa64Pcrel22;
    #[allow(non_upper_case_globals)]
    pub const ArmPrivate10: Self = Self::_Ppc64PltcallNotocOrArmPrivate10OrIa64Pcrel22;
    /// @pcrel(sym + add), 22bit inst
    #[allow(non_upper_case_globals)]
    pub const Ia64Pcrel22: Self = Self::_Ppc64PltcallNotocOrArmPrivate10OrIa64Pcrel22;
    #[allow(non_upper_case_globals)]
    pub const Ppc64PcrelOpt: Self =
        Self::_Ppc64PcrelOptOrArmPrivate11OrPariscLtoffFptr14WrOrIa64Pcrel64I;
    #[allow(non_upper_case_globals)]
    pub const ArmPrivate11: Self =
        Self::_Ppc64PcrelOptOrArmPrivate11OrPariscLtoffFptr14WrOrIa64Pcrel64I;
    /// LT-rel. fct. ptr., right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoffFptr14Wr: Self =
        Self::_Ppc64PcrelOptOrArmPrivate11OrPariscLtoffFptr14WrOrIa64Pcrel64I;
    /// @pcrel(sym + add), 64bit inst
    #[allow(non_upper_case_globals)]
    pub const Ia64Pcrel64I: Self =
        Self::_Ppc64PcrelOptOrArmPrivate11OrPariscLtoffFptr14WrOrIa64Pcrel64I;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel24P9Notoc: Self = Self::_Ppc64Rel24P9NotocOrArmPrivate12OrPariscLtoffFptr14Dr;
    #[allow(non_upper_case_globals)]
    pub const ArmPrivate12: Self = Self::_Ppc64Rel24P9NotocOrArmPrivate12OrPariscLtoffFptr14Dr;
    /// LT-rel. fct. ptr., right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoffFptr14Dr: Self =
        Self::_Ppc64Rel24P9NotocOrArmPrivate12OrPariscLtoffFptr14Dr;
    #[allow(non_upper_case_globals)]
    pub const Ppc64D34 : Self = Self::_Ppc64D34OrTilegxGnuVtinheritOrArmMeTooOrMipsNumOrPariscLoreserveOrPariscCopyOrTileproGnuVtinheritOrIa64Ipltmsb;
    #[allow(non_upper_case_globals)]
    pub const TilegxGnuVtinherit : Self = Self::_Ppc64D34OrTilegxGnuVtinheritOrArmMeTooOrMipsNumOrPariscLoreserveOrPariscCopyOrTileproGnuVtinheritOrIa64Ipltmsb;
    #[allow(non_upper_case_globals)]
    pub const ArmMeToo : Self = Self::_Ppc64D34OrTilegxGnuVtinheritOrArmMeTooOrMipsNumOrPariscLoreserveOrPariscCopyOrTileproGnuVtinheritOrIa64Ipltmsb;
    #[allow(non_upper_case_globals)]
    pub const MipsNum : Self = Self::_Ppc64D34OrTilegxGnuVtinheritOrArmMeTooOrMipsNumOrPariscLoreserveOrPariscCopyOrTileproGnuVtinheritOrIa64Ipltmsb;
    #[allow(non_upper_case_globals)]
    pub const PariscLoreserve : Self = Self::_Ppc64D34OrTilegxGnuVtinheritOrArmMeTooOrMipsNumOrPariscLoreserveOrPariscCopyOrTileproGnuVtinheritOrIa64Ipltmsb;
    /// Copy relocation.
    #[allow(non_upper_case_globals)]
    pub const PariscCopy : Self = Self::_Ppc64D34OrTilegxGnuVtinheritOrArmMeTooOrMipsNumOrPariscLoreserveOrPariscCopyOrTileproGnuVtinheritOrIa64Ipltmsb;
    /// GNU C++ vtable hierarchy
    #[allow(non_upper_case_globals)]
    pub const TileproGnuVtinherit : Self = Self::_Ppc64D34OrTilegxGnuVtinheritOrArmMeTooOrMipsNumOrPariscLoreserveOrPariscCopyOrTileproGnuVtinheritOrIa64Ipltmsb;
    /// dynamic reloc, imported PLT, MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Ipltmsb : Self = Self::_Ppc64D34OrTilegxGnuVtinheritOrArmMeTooOrMipsNumOrPariscLoreserveOrPariscCopyOrTileproGnuVtinheritOrIa64Ipltmsb;
    #[allow(non_upper_case_globals)]
    pub const Ppc64D34Lo : Self = Self::_Ppc64D34LoOrTilegxGnuVtentryOrArmThmTlsDescseq16OrPariscIpltOrArmThmTlsDescseqOrTileproGnuVtentryOrIa64Ipltlsb;
    #[allow(non_upper_case_globals)]
    pub const TilegxGnuVtentry : Self = Self::_Ppc64D34LoOrTilegxGnuVtentryOrArmThmTlsDescseq16OrPariscIpltOrArmThmTlsDescseqOrTileproGnuVtentryOrIa64Ipltlsb;
    #[allow(non_upper_case_globals)]
    pub const ArmThmTlsDescseq16 : Self = Self::_Ppc64D34LoOrTilegxGnuVtentryOrArmThmTlsDescseq16OrPariscIpltOrArmThmTlsDescseqOrTileproGnuVtentryOrIa64Ipltlsb;
    /// Dynamic reloc, imported PLT
    #[allow(non_upper_case_globals)]
    pub const PariscIplt : Self = Self::_Ppc64D34LoOrTilegxGnuVtentryOrArmThmTlsDescseq16OrPariscIpltOrArmThmTlsDescseqOrTileproGnuVtentryOrIa64Ipltlsb;
    #[allow(non_upper_case_globals)]
    pub const ArmThmTlsDescseq : Self = Self::_Ppc64D34LoOrTilegxGnuVtentryOrArmThmTlsDescseq16OrPariscIpltOrArmThmTlsDescseqOrTileproGnuVtentryOrIa64Ipltlsb;
    /// GNU C++ vtable member usage
    #[allow(non_upper_case_globals)]
    pub const TileproGnuVtentry : Self = Self::_Ppc64D34LoOrTilegxGnuVtentryOrArmThmTlsDescseq16OrPariscIpltOrArmThmTlsDescseqOrTileproGnuVtentryOrIa64Ipltlsb;
    /// dynamic reloc, imported PLT, LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Ipltlsb : Self = Self::_Ppc64D34LoOrTilegxGnuVtentryOrArmThmTlsDescseq16OrPariscIpltOrArmThmTlsDescseqOrTileproGnuVtentryOrIa64Ipltlsb;
    #[allow(non_upper_case_globals)]
    pub const Ppc64D34Hi30: Self =
        Self::_Ppc64D34Hi30OrArmThmTlsDescseq32OrTilegxNumOrPariscEpltOrTileproNum;
    #[allow(non_upper_case_globals)]
    pub const ArmThmTlsDescseq32: Self =
        Self::_Ppc64D34Hi30OrArmThmTlsDescseq32OrTilegxNumOrPariscEpltOrTileproNum;
    #[allow(non_upper_case_globals)]
    pub const TilegxNum: Self =
        Self::_Ppc64D34Hi30OrArmThmTlsDescseq32OrTilegxNumOrPariscEpltOrTileproNum;
    /// Dynamic reloc, exported PLT
    #[allow(non_upper_case_globals)]
    pub const PariscEplt: Self =
        Self::_Ppc64D34Hi30OrArmThmTlsDescseq32OrTilegxNumOrPariscEpltOrTileproNum;
    #[allow(non_upper_case_globals)]
    pub const TileproNum: Self =
        Self::_Ppc64D34Hi30OrArmThmTlsDescseq32OrTilegxNumOrPariscEpltOrTileproNum;
    #[allow(non_upper_case_globals)]
    pub const Ppc64D34Ha30: Self = Self::_Ppc64D34Ha30OrArmThmGotBrel12;
    #[allow(non_upper_case_globals)]
    pub const ArmThmGotBrel12: Self = Self::_Ppc64D34Ha30OrArmThmGotBrel12;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Pcrel34: Self = Self::_Ppc64Pcrel34OrIa64Copy;
    /// copy relocation
    #[allow(non_upper_case_globals)]
    pub const Ia64Copy: Self = Self::_Ppc64Pcrel34OrIa64Copy;
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotPcrel34: Self = Self::_Ppc64GotPcrel34OrIa64Sub;
    /// Addend and symbol difference
    #[allow(non_upper_case_globals)]
    pub const Ia64Sub: Self = Self::_Ppc64GotPcrel34OrIa64Sub;
    #[allow(non_upper_case_globals)]
    pub const Ppc64PltPcrel34: Self = Self::_Ppc64PltPcrel34OrMicromipsHi16OrIa64Ltoff22X;
    #[allow(non_upper_case_globals)]
    pub const MicromipsHi16: Self = Self::_Ppc64PltPcrel34OrMicromipsHi16OrIa64Ltoff22X;
    /// LTOFF22, relaxable.
    #[allow(non_upper_case_globals)]
    pub const Ia64Ltoff22X: Self = Self::_Ppc64PltPcrel34OrMicromipsHi16OrIa64Ltoff22X;
    #[allow(non_upper_case_globals)]
    pub const Ppc64PltPcrel34Notoc: Self = Self::_Ppc64PltPcrel34NotocOrMicromipsLo16OrIa64Ldxmov;
    #[allow(non_upper_case_globals)]
    pub const MicromipsLo16: Self = Self::_Ppc64PltPcrel34NotocOrMicromipsLo16OrIa64Ldxmov;
    /// Use of LTOFF22X.
    #[allow(non_upper_case_globals)]
    pub const Ia64Ldxmov: Self = Self::_Ppc64PltPcrel34NotocOrMicromipsLo16OrIa64Ldxmov;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr16Higher34: Self = Self::_Ppc64Addr16Higher34OrMicromipsGprel16;
    #[allow(non_upper_case_globals)]
    pub const MicromipsGprel16: Self = Self::_Ppc64Addr16Higher34OrMicromipsGprel16;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr16Highera34: Self =
        Self::_Ppc64Addr16Highera34OrMicromipsLiteralOrArmThmBf12;
    #[allow(non_upper_case_globals)]
    pub const MicromipsLiteral: Self = Self::_Ppc64Addr16Highera34OrMicromipsLiteralOrArmThmBf12;
    #[allow(non_upper_case_globals)]
    pub const ArmThmBf12: Self = Self::_Ppc64Addr16Highera34OrMicromipsLiteralOrArmThmBf12;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr16Highest34: Self = Self::_Ppc64Addr16Highest34OrMicromipsGot16OrArmThmBf18;
    #[allow(non_upper_case_globals)]
    pub const MicromipsGot16: Self = Self::_Ppc64Addr16Highest34OrMicromipsGot16OrArmThmBf18;
    #[allow(non_upper_case_globals)]
    pub const ArmThmBf18: Self = Self::_Ppc64Addr16Highest34OrMicromipsGot16OrArmThmBf18;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Addr16Highesta34: Self = Self::_Ppc64Addr16Highesta34OrMicromipsPc7S1;
    #[allow(non_upper_case_globals)]
    pub const MicromipsPc7S1: Self = Self::_Ppc64Addr16Highesta34OrMicromipsPc7S1;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel16Higher34: Self = Self::_Ppc64Rel16Higher34OrMicromipsPc10S1;
    #[allow(non_upper_case_globals)]
    pub const MicromipsPc10S1: Self = Self::_Ppc64Rel16Higher34OrMicromipsPc10S1;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel16Highera34: Self = Self::_Ppc64Rel16Highera34OrMicromipsPc16S1;
    #[allow(non_upper_case_globals)]
    pub const MicromipsPc16S1: Self = Self::_Ppc64Rel16Highera34OrMicromipsPc16S1;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel16Highest34: Self = Self::_Ppc64Rel16Highest34OrMicromipsCall16;
    #[allow(non_upper_case_globals)]
    pub const MicromipsCall16: Self = Self::_Ppc64Rel16Highest34OrMicromipsCall16;
    #[allow(non_upper_case_globals)]
    pub const Ppc64D28: Self = Self::_Ppc64D28OrShTlsGd32;
    #[allow(non_upper_case_globals)]
    pub const ShTlsGd32: Self = Self::_Ppc64D28OrShTlsGd32;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Pcrel28: Self = Self::_Ppc64Pcrel28OrMicromipsGotDispOrIa64Tprel14OrShTlsLd32;
    #[allow(non_upper_case_globals)]
    pub const MicromipsGotDisp: Self =
        Self::_Ppc64Pcrel28OrMicromipsGotDispOrIa64Tprel14OrShTlsLd32;
    /// @tprel(sym + add), imm14
    #[allow(non_upper_case_globals)]
    pub const Ia64Tprel14: Self = Self::_Ppc64Pcrel28OrMicromipsGotDispOrIa64Tprel14OrShTlsLd32;
    #[allow(non_upper_case_globals)]
    pub const ShTlsLd32: Self = Self::_Ppc64Pcrel28OrMicromipsGotDispOrIa64Tprel14OrShTlsLd32;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Tprel34: Self = Self::_Ppc64Tprel34OrMicromipsGotPageOrIa64Tprel22OrShTlsLdo32;
    #[allow(non_upper_case_globals)]
    pub const MicromipsGotPage: Self =
        Self::_Ppc64Tprel34OrMicromipsGotPageOrIa64Tprel22OrShTlsLdo32;
    /// @tprel(sym + add), imm22
    #[allow(non_upper_case_globals)]
    pub const Ia64Tprel22: Self = Self::_Ppc64Tprel34OrMicromipsGotPageOrIa64Tprel22OrShTlsLdo32;
    #[allow(non_upper_case_globals)]
    pub const ShTlsLdo32: Self = Self::_Ppc64Tprel34OrMicromipsGotPageOrIa64Tprel22OrShTlsLdo32;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Dtprel34: Self = Self::_Ppc64Dtprel34OrMicromipsGotOfstOrIa64Tprel64IOrShTlsIe32;
    #[allow(non_upper_case_globals)]
    pub const MicromipsGotOfst: Self =
        Self::_Ppc64Dtprel34OrMicromipsGotOfstOrIa64Tprel64IOrShTlsIe32;
    /// @tprel(sym + add), imm64
    #[allow(non_upper_case_globals)]
    pub const Ia64Tprel64I: Self = Self::_Ppc64Dtprel34OrMicromipsGotOfstOrIa64Tprel64IOrShTlsIe32;
    #[allow(non_upper_case_globals)]
    pub const ShTlsIe32: Self = Self::_Ppc64Dtprel34OrMicromipsGotOfstOrIa64Tprel64IOrShTlsIe32;
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotTlsgdPcrel34: Self = Self::_Ppc64GotTlsgdPcrel34OrMicromipsGotHi16OrShTlsLe32;
    #[allow(non_upper_case_globals)]
    pub const MicromipsGotHi16: Self = Self::_Ppc64GotTlsgdPcrel34OrMicromipsGotHi16OrShTlsLe32;
    #[allow(non_upper_case_globals)]
    pub const ShTlsLe32: Self = Self::_Ppc64GotTlsgdPcrel34OrMicromipsGotHi16OrShTlsLe32;
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotTlsldPcrel34: Self =
        Self::_Ppc64GotTlsldPcrel34OrMicromipsGotLo16OrShTlsDtpmod32;
    #[allow(non_upper_case_globals)]
    pub const MicromipsGotLo16: Self = Self::_Ppc64GotTlsldPcrel34OrMicromipsGotLo16OrShTlsDtpmod32;
    #[allow(non_upper_case_globals)]
    pub const ShTlsDtpmod32: Self = Self::_Ppc64GotTlsldPcrel34OrMicromipsGotLo16OrShTlsDtpmod32;
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotTprelPcrel34: Self =
        Self::_Ppc64GotTprelPcrel34OrMicromipsSubOrIa64Tprel64MsbOrShTlsDtpoff32;
    #[allow(non_upper_case_globals)]
    pub const MicromipsSub: Self =
        Self::_Ppc64GotTprelPcrel34OrMicromipsSubOrIa64Tprel64MsbOrShTlsDtpoff32;
    /// @tprel(sym + add), data8 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Tprel64Msb: Self =
        Self::_Ppc64GotTprelPcrel34OrMicromipsSubOrIa64Tprel64MsbOrShTlsDtpoff32;
    #[allow(non_upper_case_globals)]
    pub const ShTlsDtpoff32: Self =
        Self::_Ppc64GotTprelPcrel34OrMicromipsSubOrIa64Tprel64MsbOrShTlsDtpoff32;
    #[allow(non_upper_case_globals)]
    pub const Ppc64GotDtprelPcrel34: Self =
        Self::_Ppc64GotDtprelPcrel34OrMicromipsHigherOrIa64Tprel64LsbOrShTlsTpoff32;
    #[allow(non_upper_case_globals)]
    pub const MicromipsHigher: Self =
        Self::_Ppc64GotDtprelPcrel34OrMicromipsHigherOrIa64Tprel64LsbOrShTlsTpoff32;
    /// @tprel(sym + add), data8 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Tprel64Lsb: Self =
        Self::_Ppc64GotDtprelPcrel34OrMicromipsHigherOrIa64Tprel64LsbOrShTlsTpoff32;
    #[allow(non_upper_case_globals)]
    pub const ShTlsTpoff32: Self =
        Self::_Ppc64GotDtprelPcrel34OrMicromipsHigherOrIa64Tprel64LsbOrShTlsTpoff32;
    #[allow(non_upper_case_globals)]
    pub const PpcVleLo16A: Self = Self::_PpcVleLo16AOrPariscTprel14Wr;
    /// TP-rel. address, right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscTprel14Wr: Self = Self::_PpcVleLo16AOrPariscTprel14Wr;
    #[allow(non_upper_case_globals)]
    pub const PpcVleLo16D: Self = Self::_PpcVleLo16DOrPariscTprel14Dr;
    /// TP-rel. address, right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscTprel14Dr: Self = Self::_PpcVleLo16DOrPariscTprel14Dr;
    #[allow(non_upper_case_globals)]
    pub const PpcVleHi16A: Self = Self::_PpcVleHi16AOrPariscTprel16F;
    /// 16 bits TP-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscTprel16F: Self = Self::_PpcVleHi16AOrPariscTprel16F;
    #[allow(non_upper_case_globals)]
    pub const PpcVleHi16D: Self = Self::_PpcVleHi16DOrPariscTprel16Wf;
    /// 16 bits TP-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscTprel16Wf: Self = Self::_PpcVleHi16DOrPariscTprel16Wf;
    #[allow(non_upper_case_globals)]
    pub const PpcVleHa16A: Self = Self::_PpcVleHa16AOrPariscTprel16Df;
    /// 16 bits TP-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscTprel16Df: Self = Self::_PpcVleHa16AOrPariscTprel16Df;
    #[allow(non_upper_case_globals)]
    pub const PpcVleHa16D: Self = Self::_PpcVleHa16DOrPariscLtoffTp64;
    /// 64 bits LT-TP-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoffTp64: Self = Self::_PpcVleHa16DOrPariscLtoffTp64;
    #[allow(non_upper_case_globals)]
    pub const PpcVleSdarelLo16A: Self = Self::_PpcVleSdarelLo16AOrPariscLtoffTp14Wr;
    /// LT-TP-rel. address, right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoffTp14Wr: Self = Self::_PpcVleSdarelLo16AOrPariscLtoffTp14Wr;
    #[allow(non_upper_case_globals)]
    pub const PpcVleSdarelLo16D: Self = Self::_PpcVleSdarelLo16DOrPariscLtoffTp14Dr;
    /// LT-TP-rel. address, right 14 bits.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoffTp14Dr: Self = Self::_PpcVleSdarelLo16DOrPariscLtoffTp14Dr;
    #[allow(non_upper_case_globals)]
    pub const PpcVleSdarelHi16A: Self = Self::_PpcVleSdarelHi16AOrPariscLtoffTp16F;
    /// 16 bits LT-TP-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoffTp16F: Self = Self::_PpcVleSdarelHi16AOrPariscLtoffTp16F;
    #[allow(non_upper_case_globals)]
    pub const PpcVleSdarelHi16D: Self = Self::_PpcVleSdarelHi16DOrPariscLtoffTp16Wf;
    /// 16 bits LT-TP-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoffTp16Wf: Self = Self::_PpcVleSdarelHi16DOrPariscLtoffTp16Wf;
    #[allow(non_upper_case_globals)]
    pub const PpcVleSdarelHa16A: Self = Self::_PpcVleSdarelHa16AOrPariscLtoffTp16Df;
    /// 16 bits LT-TP-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoffTp16Df: Self = Self::_PpcVleSdarelHa16AOrPariscLtoffTp16Df;
    #[allow(non_upper_case_globals)]
    pub const PpcVleSdarelHa16D: Self = Self::_PpcVleSdarelHa16DOrPariscGnuVtentry;
    #[allow(non_upper_case_globals)]
    pub const PariscGnuVtentry: Self = Self::_PpcVleSdarelHa16DOrPariscGnuVtentry;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel16High: Self = Self::_Ppc64Rel16HighOrPariscTlsLdo21L;
    /// LD offset 21-bit left.
    #[allow(non_upper_case_globals)]
    pub const PariscTlsLdo21L: Self = Self::_Ppc64Rel16HighOrPariscTlsLdo21L;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel16Higha: Self = Self::_Ppc64Rel16HighaOrPariscTlsLdo14R;
    /// LD offset 14-bit right.
    #[allow(non_upper_case_globals)]
    pub const PariscTlsLdo14R: Self = Self::_Ppc64Rel16HighaOrPariscTlsLdo14R;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel16Higher: Self = Self::_Ppc64Rel16HigherOrPariscTlsDtpmod32;
    /// DTP module 32-bit.
    #[allow(non_upper_case_globals)]
    pub const PariscTlsDtpmod32: Self = Self::_Ppc64Rel16HigherOrPariscTlsDtpmod32;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel16Highera: Self = Self::_Ppc64Rel16HigheraOrPariscTlsDtpmod64;
    /// DTP module 64-bit.
    #[allow(non_upper_case_globals)]
    pub const PariscTlsDtpmod64: Self = Self::_Ppc64Rel16HigheraOrPariscTlsDtpmod64;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel16Highest: Self = Self::_Ppc64Rel16HighestOrPariscTlsDtpoff32;
    /// DTP offset 32-bit.
    #[allow(non_upper_case_globals)]
    pub const PariscTlsDtpoff32: Self = Self::_Ppc64Rel16HighestOrPariscTlsDtpoff32;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel16Highesta: Self = Self::_Ppc64Rel16HighestaOrPariscTlsDtpoff64;
    /// DTP offset 32-bit.
    #[allow(non_upper_case_globals)]
    pub const PariscTlsDtpoff64: Self = Self::_Ppc64Rel16HighestaOrPariscTlsDtpoff64;
    #[allow(non_upper_case_globals)]
    pub const PowerpcIrelative: Self =
        Self::_PowerpcIrelativeOrSparcJmpIrelOrPpcIrelativeOrPpc64Irelative;
    #[allow(non_upper_case_globals)]
    pub const SparcJmpIrel: Self =
        Self::_PowerpcIrelativeOrSparcJmpIrelOrPpcIrelativeOrPpc64Irelative;
    #[allow(non_upper_case_globals)]
    pub const PpcIrelative: Self =
        Self::_PowerpcIrelativeOrSparcJmpIrelOrPpcIrelativeOrPpc64Irelative;
    #[allow(non_upper_case_globals)]
    pub const Ppc64Irelative: Self =
        Self::_PowerpcIrelativeOrSparcJmpIrelOrPpcIrelativeOrPpc64Irelative;
    #[allow(non_upper_case_globals)]
    pub const PowerpcRel16: Self =
        Self::_PowerpcRel16OrMipsEhOrSparcIrelativeOrPpcRel16OrPpc64Rel16OrArmRxpc25;
    #[allow(non_upper_case_globals)]
    pub const MipsEh: Self =
        Self::_PowerpcRel16OrMipsEhOrSparcIrelativeOrPpcRel16OrPpc64Rel16OrArmRxpc25;
    #[allow(non_upper_case_globals)]
    pub const SparcIrelative: Self =
        Self::_PowerpcRel16OrMipsEhOrSparcIrelativeOrPpcRel16OrPpc64Rel16OrArmRxpc25;
    /// half16   (sym+add-.)
    #[allow(non_upper_case_globals)]
    pub const PpcRel16: Self =
        Self::_PowerpcRel16OrMipsEhOrSparcIrelativeOrPpcRel16OrPpc64Rel16OrArmRxpc25;
    /// half16   (sym+add-.)
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel16: Self =
        Self::_PowerpcRel16OrMipsEhOrSparcIrelativeOrPpcRel16OrPpc64Rel16OrArmRxpc25;
    #[allow(non_upper_case_globals)]
    pub const ArmRxpc25: Self =
        Self::_PowerpcRel16OrMipsEhOrSparcIrelativeOrPpcRel16OrPpc64Rel16OrArmRxpc25;
    #[allow(non_upper_case_globals)]
    pub const PowerpcRel16Lo: Self =
        Self::_PowerpcRel16LoOrSparcGnuVtinheritOrPpcRel16LoOrPpc64Rel16LoOrArmRsbrel32;
    #[allow(non_upper_case_globals)]
    pub const SparcGnuVtinherit: Self =
        Self::_PowerpcRel16LoOrSparcGnuVtinheritOrPpcRel16LoOrPpc64Rel16LoOrArmRsbrel32;
    /// half16   (sym+add-.)@l
    #[allow(non_upper_case_globals)]
    pub const PpcRel16Lo: Self =
        Self::_PowerpcRel16LoOrSparcGnuVtinheritOrPpcRel16LoOrPpc64Rel16LoOrArmRsbrel32;
    /// half16   (sym+add-.)@l
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel16Lo: Self =
        Self::_PowerpcRel16LoOrSparcGnuVtinheritOrPpcRel16LoOrPpc64Rel16LoOrArmRsbrel32;
    #[allow(non_upper_case_globals)]
    pub const ArmRsbrel32: Self =
        Self::_PowerpcRel16LoOrSparcGnuVtinheritOrPpcRel16LoOrPpc64Rel16LoOrArmRsbrel32;
    #[allow(non_upper_case_globals)]
    pub const PowerpcRel16Ha: Self =
        Self::_PowerpcRel16HaOrSparcRev32OrPpcRel16HaOrPpc64Rel16HaOrArmRrel32;
    #[allow(non_upper_case_globals)]
    pub const SparcRev32: Self =
        Self::_PowerpcRel16HaOrSparcRev32OrPpcRel16HaOrPpc64Rel16HaOrArmRrel32;
    /// half16   (sym+add-.)@ha
    #[allow(non_upper_case_globals)]
    pub const PpcRel16Ha: Self =
        Self::_PowerpcRel16HaOrSparcRev32OrPpcRel16HaOrPpc64Rel16HaOrArmRrel32;
    /// half16   (sym+add-.)@ha
    #[allow(non_upper_case_globals)]
    pub const Ppc64Rel16Ha: Self =
        Self::_PowerpcRel16HaOrSparcRev32OrPpcRel16HaOrPpc64Rel16HaOrArmRrel32;
    #[allow(non_upper_case_globals)]
    pub const ArmRrel32: Self =
        Self::_PowerpcRel16HaOrSparcRev32OrPpcRel16HaOrPpc64Rel16HaOrArmRrel32;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGnuVtinherit: Self = Self::_PowerpcGnuVtinheritOrSparcNumOrArmRabs22;
    #[allow(non_upper_case_globals)]
    pub const SparcNum: Self = Self::_PowerpcGnuVtinheritOrSparcNumOrArmRabs22;
    #[allow(non_upper_case_globals)]
    pub const ArmRabs22: Self = Self::_PowerpcGnuVtinheritOrSparcNumOrArmRabs22;
    #[allow(non_upper_case_globals)]
    pub const PowerpcGnuVtentry: Self = Self::_PowerpcGnuVtentryOrMipsGnuVtentryOrArmRpc24;
    #[allow(non_upper_case_globals)]
    pub const MipsGnuVtentry: Self = Self::_PowerpcGnuVtentryOrMipsGnuVtentryOrArmRpc24;
    #[allow(non_upper_case_globals)]
    pub const ArmRpc24: Self = Self::_PowerpcGnuVtentryOrMipsGnuVtentryOrArmRpc24;
    #[allow(non_upper_case_globals)]
    pub const PpcToc16: Self = Self::_PpcToc16OrPariscHireserveOrArmRbase;
    #[allow(non_upper_case_globals)]
    pub const PariscHireserve: Self = Self::_PpcToc16OrPariscHireserveOrArmRbase;
    #[allow(non_upper_case_globals)]
    pub const ArmRbase: Self = Self::_PpcToc16OrPariscHireserveOrArmRbase;
    #[allow(non_upper_case_globals)]
    pub const MipsCopy: Self = Self::_MipsCopyOrArmPrivate14OrPariscLtoffFptr16Wf;
    #[allow(non_upper_case_globals)]
    pub const ArmPrivate14: Self = Self::_MipsCopyOrArmPrivate14OrPariscLtoffFptr16Wf;
    /// 16 bits LT-rel. function ptr.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoffFptr16Wf: Self = Self::_MipsCopyOrArmPrivate14OrPariscLtoffFptr16Wf;
    #[allow(non_upper_case_globals)]
    pub const MipsJumpSlot: Self = Self::_MipsJumpSlotOrArmPrivate15OrPariscLtoffFptr16Df;
    #[allow(non_upper_case_globals)]
    pub const ArmPrivate15: Self = Self::_MipsJumpSlotOrArmPrivate15OrPariscLtoffFptr16Df;
    /// 16 bits LT-rel. function ptr.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoffFptr16Df: Self = Self::_MipsJumpSlotOrArmPrivate15OrPariscLtoffFptr16Df;
    #[allow(non_upper_case_globals)]
    pub const MicromipsTlsLdm: Self = Self::_MicromipsTlsLdmOrShGlobDat;
    #[allow(non_upper_case_globals)]
    pub const ShGlobDat: Self = Self::_MicromipsTlsLdmOrShGlobDat;
    #[allow(non_upper_case_globals)]
    pub const MicromipsTlsDtprelHi16: Self = Self::_MicromipsTlsDtprelHi16OrShJmpSlot;
    #[allow(non_upper_case_globals)]
    pub const ShJmpSlot: Self = Self::_MicromipsTlsDtprelHi16OrShJmpSlot;
    #[allow(non_upper_case_globals)]
    pub const MicromipsTlsDtprelLo16: Self = Self::_MicromipsTlsDtprelLo16OrShRelative;
    #[allow(non_upper_case_globals)]
    pub const ShRelative: Self = Self::_MicromipsTlsDtprelLo16OrShRelative;
    #[allow(non_upper_case_globals)]
    pub const MicromipsTlsTprelLo16: Self = Self::_MicromipsTlsTprelLo16OrIa64LtoffDtpmod22;
    /// @ltoff(@dtpmod(sym + add)), imm22
    #[allow(non_upper_case_globals)]
    pub const Ia64LtoffDtpmod22: Self = Self::_MicromipsTlsTprelLo16OrIa64LtoffDtpmod22;
    #[allow(non_upper_case_globals)]
    pub const ArmPrivate13: Self = Self::_ArmPrivate13OrPariscLtoffFptr16F;
    /// 16 bits LT-rel. function ptr.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoffFptr16F: Self = Self::_ArmPrivate13OrPariscLtoffFptr16F;
    #[allow(non_upper_case_globals)]
    pub const AArch64Withdrawn: Self = Self::_AArch64WithdrawnOrArmNumOrShNumOrM32RNum;
    #[allow(non_upper_case_globals)]
    pub const ArmNum: Self = Self::_AArch64WithdrawnOrArmNumOrShNumOrM32RNum;
    #[allow(non_upper_case_globals)]
    pub const ShNum: Self = Self::_AArch64WithdrawnOrArmNumOrShNumOrM32RNum;
    /// Keep this the last entry.
    #[allow(non_upper_case_globals)]
    pub const M32RNum: Self = Self::_AArch64WithdrawnOrArmNumOrShNumOrM32RNum;
    #[allow(non_upper_case_globals)]
    pub const AArch64TlsDtprel64: Self = Self::_AArch64TlsDtprel64OrAArch64TlsDtprel;
    /// Module-relative offset, 64 bit.
    #[allow(non_upper_case_globals)]
    pub const AArch64TlsDtprel: Self = Self::_AArch64TlsDtprel64OrAArch64TlsDtprel;
    #[allow(non_upper_case_globals)]
    pub const AArch64TlsTprel64: Self = Self::_AArch64TlsTprel64OrAArch64TlsTprel;
    /// TP-relative offset, 64 bit.
    #[allow(non_upper_case_globals)]
    pub const AArch64TlsTprel: Self = Self::_AArch64TlsTprel64OrAArch64TlsTprel;
    /// 14 bits LT-TP-rel. address.
    #[allow(non_upper_case_globals)]
    pub const PariscLtoffTp14F: Self = Self::_PariscLtoffTp14FOrShGotpcOrIa64Dtpmod64Lsb;
    #[allow(non_upper_case_globals)]
    pub const ShGotpc: Self = Self::_PariscLtoffTp14FOrShGotpcOrIa64Dtpmod64Lsb;
    /// @dtpmod(sym + add), data8 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Dtpmod64Lsb: Self = Self::_PariscLtoffTp14FOrShGotpcOrIa64Dtpmod64Lsb;
    /// like EMB_SDA21, but lower 16 bit
    #[allow(non_upper_case_globals)]
    pub const PpcDiabSda21Lo: Self = Self::_PpcDiabSda21LoOrAArch64P32CopyOrIa64Dtprel32Msb;
    /// Copy symbol at runtime.
    #[allow(non_upper_case_globals)]
    pub const AArch64P32Copy: Self = Self::_PpcDiabSda21LoOrAArch64P32CopyOrIa64Dtprel32Msb;
    /// @dtprel(sym + add), data4 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Dtprel32Msb: Self = Self::_PpcDiabSda21LoOrAArch64P32CopyOrIa64Dtprel32Msb;
    /// like EMB_SDA21, but high 16 bit
    #[allow(non_upper_case_globals)]
    pub const PpcDiabSda21Hi: Self = Self::_PpcDiabSda21HiOrAArch64P32GlobDatOrIa64Dtprel32Lsb;
    /// Create GOT entry.
    #[allow(non_upper_case_globals)]
    pub const AArch64P32GlobDat: Self = Self::_PpcDiabSda21HiOrAArch64P32GlobDatOrIa64Dtprel32Lsb;
    /// @dtprel(sym + add), data4 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Dtprel32Lsb: Self = Self::_PpcDiabSda21HiOrAArch64P32GlobDatOrIa64Dtprel32Lsb;
    /// like EMB_SDA21, adjusted high 16
    #[allow(non_upper_case_globals)]
    pub const PpcDiabSda21Ha: Self = Self::_PpcDiabSda21HaOrAArch64P32JumpSlotOrIa64Dtprel64Msb;
    /// Create PLT entry.
    #[allow(non_upper_case_globals)]
    pub const AArch64P32JumpSlot: Self = Self::_PpcDiabSda21HaOrAArch64P32JumpSlotOrIa64Dtprel64Msb;
    /// @dtprel(sym + add), data8 MSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Dtprel64Msb: Self = Self::_PpcDiabSda21HaOrAArch64P32JumpSlotOrIa64Dtprel64Msb;
    /// like EMB_RELSDA, but lower 16 bit
    #[allow(non_upper_case_globals)]
    pub const PpcDiabRelsdaLo: Self = Self::_PpcDiabRelsdaLoOrAArch64P32RelativeOrIa64Dtprel64Lsb;
    /// Adjust by program base.
    #[allow(non_upper_case_globals)]
    pub const AArch64P32Relative: Self =
        Self::_PpcDiabRelsdaLoOrAArch64P32RelativeOrIa64Dtprel64Lsb;
    /// @dtprel(sym + add), data8 LSB
    #[allow(non_upper_case_globals)]
    pub const Ia64Dtprel64Lsb: Self = Self::_PpcDiabRelsdaLoOrAArch64P32RelativeOrIa64Dtprel64Lsb;
    /// like EMB_RELSDA, but high 16 bit
    #[allow(non_upper_case_globals)]
    pub const PpcDiabRelsdaHi: Self = Self::_PpcDiabRelsdaHiOrAArch64P32TlsDtpmod;
    /// Module number, 32 bit.
    #[allow(non_upper_case_globals)]
    pub const AArch64P32TlsDtpmod: Self = Self::_PpcDiabRelsdaHiOrAArch64P32TlsDtpmod;
    /// like EMB_RELSDA, adjusted high 16
    #[allow(non_upper_case_globals)]
    pub const PpcDiabRelsdaHa: Self = Self::_PpcDiabRelsdaHaOrAArch64P32TlsDtprel;
    /// Module-relative offset, 32 bit.
    #[allow(non_upper_case_globals)]
    pub const AArch64P32TlsDtprel: Self = Self::_PpcDiabRelsdaHaOrAArch64P32TlsDtprel;
    /// TP-relative offset, 32 bit.
    #[allow(non_upper_case_globals)]
    pub const AArch64P32TlsTprel: Self = Self::_AArch64P32TlsTprelOrIa64LtoffDtprel22;
    /// @ltoff(@dtprel(s+a)), imm22
    #[allow(non_upper_case_globals)]
    pub const Ia64LtoffDtprel22: Self = Self::_AArch64P32TlsTprelOrIa64LtoffDtprel22;
    #[allow(non_upper_case_globals)]
    pub const ArmIrelative: Self = Self::_ArmIrelativeOrShGot32;
    #[allow(non_upper_case_globals)]
    pub const ShGot32: Self = Self::_ArmIrelativeOrShGot32;
}
