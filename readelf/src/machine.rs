use std::fmt;

/// The target instruction set architecture for the ELF file.
///
/// There are various sources of official documentation for the assignment of
/// the machine types.
///
/// - [SCO gABI](https://www.sco.com/developers/gabi/latest/ch4.eheader.html)
///   containing values 0-243.
/// - [Xinuous Registry](https://groups.google.com/g/generic-abi/c/cmq1LFFpWqU)
///   for values 244-250.
/// - Further missing constants obtained by GNU
///   [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=include/elf/common.h;h=78c85bd514b91bd7ff31f387b4dc98e477d10072).
///
/// The value of the constants try to be the same as the official SCO
/// documentation, but if it starts with a number, then it is prepended usually
/// with the manufacturer's name. In some cases, this has been done
/// consistently.
///
/// # Example
/// ```rust
/// use readelf::Machine;
///
/// let m = Machine::from(Machine::NONE);
/// println!("{:?}", m);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Machine {
    machine: u16,
}

impl Machine {
    /// No machine.
    pub const NONE: u16 = 0x0000;

    /// Bellmac 32 AT&T WE 32100.
    pub const M32: u16 = 0x0001;

    /// SPARC.
    pub const SPARC: u16 = 0x0002;

    /// Intel 80386.
    pub const INTEL_386: u16 = 0x0003;

    /// Motorola 68000.
    pub const MOTOROLA_68K: u16 = 0x0004;

    /// Motorola 88000.
    pub const MOTOROLA_88K: u16 = 0x0005;

    /// Intel MCU.
    pub const IAMCU: u16 = 0x0006;

    /// Intel 80860.
    pub const INTEL_860: u16 = 0x0007;

    /// MIPS I Architecture.
    ///
    /// Linux [Elf(5)](https://man7.org/linux/man-pages/man5/elf.5.html)
    /// documents this as being MIPS RS3000 (big-endian only).
    pub const MIPS: u16 = 0x0008;

    /// IBM System/370 Processor.
    pub const S370: u16 = 0x0009;

    /// MIPS RS3000 Little-endian.
    pub const MIPS_RS3_LE: u16 = 0x000A;

    /// Hewlett-Packard PA-RISC.
    pub const PARISC: u16 = 0x000F;

    /// Fujitsu VPP500 / VPP550.
    pub const VPP500: u16 = 0x0011;

    /// Enhanced instruction set SPARC.
    pub const SPARC32PLUS: u16 = 0x0012;

    /// Intel 80960.
    pub const INTEL_960: u16 = 0x0013;

    /// PowerPC.
    ///
    /// This replaces the old value 0x9025, described by
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l297).
    pub const PPC: u16 = 0x0014;

    /// 64-bit PowerPC.
    pub const PPC64: u16 = 0x0015;

    /// IBM System/390 Processor.
    ///
    /// This replaces the old value 0xA390, described by
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l301).
    pub const S390: u16 = 0x0016;

    /// IBM SPU/SPC.
    pub const SPU: u16 = 0x0017;

    /// NEC V800.
    pub const V800: u16 = 0x0024;

    /// Fujitsu FR20.
    pub const FR20: u16 = 0x0025;

    /// TRW RH-32.
    pub const RH32: u16 = 0x0026;

    /// Motorola M*Core.
    pub const MCORE: u16 = 0x0027;

    /// Motorola RCE.
    ///
    /// This is the old name for [Machine::MCORE] described by GNU
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=include/elf/common.h;h=78c85bd514b91bd7ff31f387b4dc98e477d10072;hb=HEAD#l152).
    /// It is the official name given by the SCO documentation.
    pub const RCE: u16 = 0x0027;

    /// ARM 32-bit architecture (AARCH32)
    pub const ARM: u16 = 0x0028;

    /// Digital Alpha.
    ///
    /// The official SCO documentation states this as being `ALPHA`. However,
    /// observed that this value isn't used, and
    /// [glibc](https://sourceware.org/git/?p=glibc.git;a=blob;f=elf/elf.h;h=33aea7f743b885c5d74736276e55ef21756293ee;hb=HEAD#l198)
    /// refers to this as `EM_FAKE_ALPHA`. But we use the correct name as given
    /// in the [SCO documentation
    /// ](https://www.sco.com/developers/gabi/latest/ch4.eheader.html).
    pub const ALPHA: u16 = 0x0029;

    /// Hitachi SH.
    pub const SH: u16 = 0x002A;

    /// SPARC Version 9.
    pub const SPARCV9: u16 = 0x002B;

    /// Siemens TriCore embedded processor.
    pub const TRICORE: u16 = 0x002C;

    /// Argonaut RISC Core, Argonaut Technologies Inc.
    pub const ARC: u16 = 0x002D;

    /// Hitachi H8/300.
    pub const H8_300: u16 = 0x002E;

    /// Hitachi H8/300H.
    pub const H8_300H: u16 = 0x002F;

    /// Hitachi H8S.
    pub const H8S: u16 = 0x0030;

    /// Hitachi H8/500.
    pub const H8_500: u16 = 0x0031;

    /// Intel IA-64 processor architecture.
    pub const IA_64: u16 = 0x0032;

    /// Stanford MIPS-X.
    pub const MIPS_X: u16 = 0x0033;

    /// Motorola ColdFire.
    pub const COLDFIRE: u16 = 0x0034;

    /// Motorola M68HC12.
    pub const MOTOROLA_68HC12: u16 = 0x0035;

    /// Fujitsu MMA Multimedia Accelerator.
    pub const MMA: u16 = 0x0036;

    /// Siemens PCP.
    pub const PCP: u16 = 0x0037;

    /// Sony nCPU embedded RISC processor.
    pub const NCPU: u16 = 0x0038;

    /// Denso NDR1 microprocessor.
    pub const NDR1: u16 = 0x0039;

    /// Motorola Star*Core processor.
    pub const STARCORE: u16 = 0x003A;

    /// Toyota ME16 processor.
    pub const ME16: u16 = 0x003B;

    /// STMicroelectronics ST100 processor.
    pub const ST100: u16 = 0x003C;

    /// Advanced Logic Corp. TinyJ embedded processor family.
    pub const TINYJ: u16 = 0x003D;

    /// AMD x86-64 architecture.
    pub const X86_64: u16 = 0x003E;

    /// Sony DSP Processor.
    pub const PDSP: u16 = 0x003F;

    /// Digital Equipment Corp. PDP-10.
    pub const PDP10: u16 = 0x0040;

    /// Digital Equipment Corp. PDP-11.
    pub const PDP11: u16 = 0x0041;

    /// Siemens FX66 microcontroller.
    pub const FX66: u16 = 0x0042;

    /// STMicroelectronics ST9+ 8/16 bit microcontroller.
    pub const ST9PLUS: u16 = 0x0043;

    /// STMicroelectronics ST7 8-bit microcontroller.
    pub const ST7: u16 = 0x0044;

    /// Motorola MC68HC16 Microcontroller.
    pub const MOTOROLA_68HC16: u16 = 0x0045;

    /// Motorola MC68HC11 Microcontroller.
    pub const MOTOROLA_68HC11: u16 = 0x0046;

    /// Motorola MC68HC08 Microcontroller.
    pub const MOTOROLA_68HC08: u16 = 0x0047;

    /// Motorola MC68HC05 Microcontroller.
    pub const MOTOROLA_68HC05: u16 = 0x0048;

    /// Silicon Graphics SVx.
    pub const SVX: u16 = 0x0049;

    /// STMicroelectronics ST19 8-bit microcontroller.
    pub const ST19: u16 = 0x004A;

    /// Digital VAX.
    pub const VAX: u16 = 0x004B;

    /// Axis Communications 32-bit embedded processor.
    pub const CRIS: u16 = 0x004C;

    /// Infineon Technologies 32-bit embedded processor.
    pub const JAVELIN: u16 = 0x004D;

    /// Element 14 64-bit DSP Processor.
    pub const FIREPATH: u16 = 0x004E;

    /// LSI Logic 16-bit DSP Processor.
    pub const ZSP: u16 = 0x004F;

    /// Donald Knuth's educational 64-bit processor.
    pub const MMIX: u16 = 0x0050;

    /// Harvard University machine-independent object files.
    pub const HUANY: u16 = 0x0051;

    /// SiTera Prism.
    pub const PRISM: u16 = 0x0052;

    /// Atmel AVR 8-bit microcontroller.
    ///
    /// This replaces the old value 0x1057, described by
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l290).
    pub const AVR: u16 = 0x0053;

    /// Fujitsu FR30.
    ///
    /// This replaces the old value 0x3330, described by
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l292).
    pub const FR30: u16 = 0x0054;

    /// Mitsubishi D10V.
    ///
    /// This replaces the old value 0x7650, described by
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l294).
    pub const D10V: u16 = 0x0055;

    /// Mitsubishi D30V.
    ///
    /// This replaces the old value 0x7676, described by
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l295).
    pub const D30V: u16 = 0x0056;

    /// NEC v850.
    ///
    /// This replaces the old value 0x9080, described by
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l300).
    pub const V850: u16 = 0x0057;

    /// Mitsubishi M32R.
    ///
    /// This replaces the old value 0x9041, described by
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l299).
    pub const M32R: u16 = 0x0058;

    /// Matsushita MN10300.
    ///
    /// This replaces the old value 0xBEEF, described by
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l303).
    pub const MN10300: u16 = 0x0059;

    /// Matsushita MN10200.
    ///
    /// This replaces the old value 0xDEAD, described by
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l304).
    pub const MN10200: u16 = 0x005A;

    /// picoJava.
    pub const PJ: u16 = 0x005B;

    /// OpenRISC 32-bit embedded processor.
    ///
    /// Also known as `EM_OR1K` in GNU
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l249).
    /// The official SCO name is used.
    ///
    /// This replaces the old value 0x3426 and 0x8472, described by
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l293).
    pub const OPENRISC: u16 = 0x005C;

    /// OpenRISC 32-bit embedded processor.
    ///
    /// Also known as `EM_OR1K` in GNU
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l249).
    /// The official SCO name is used.
    ///
    /// This replaces the old value 0x3426 and 0x8472, described by
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l293).
    pub const OR1K: u16 = Machine::OPENRISC;

    /// ARC International ARCompact processor (old spelling/synonym: [Machine::ARC_A5]).
    pub const ARC_COMPACT: u16 = 0x005D;

    /// ARC International ARCompact processor (old spelling/synonym, replaced by [Machine::ARC_COMPACT]).
    pub const ARC_A5: u16 = Machine::ARC_COMPACT;

    /// Tensilica Xtensa Architecture.
    ///
    /// This replaces the old value 0xABC7, described by
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l302).
    pub const XTENSA: u16 = 0x005E;

    /// Alphamosaic VideoCore processor.
    pub const VIDEOCORE: u16 = 0x005F;

    /// Thompson Multimedia General Purpose Processor.
    pub const TMM_GPP: u16 = 0x0060;

    /// National Semiconductor 32000 series.
    pub const NS32K: u16 = 0x0061;

    /// Tenor Network TPC processor.
    pub const TPC: u16 = 0x0062;

    /// Trebia SNP 1000 processor.
    pub const SNP1K: u16 = 0x0063;

    /// STMicroelectronics (www.st.com) ST200 microcontroller.
    pub const ST200: u16 = 0x0064;

    /// Ubicom IP2xxx microcontroller family.
    ///
    /// This replaces the old value 0x8217, described by
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l296).
    pub const IP2K: u16 = 0x0065;

    /// MAX Processor.
    pub const MAX: u16 = 0x0066;

    /// National Semiconductor CompactRISC microprocessor.
    pub const CR: u16 = 0x0067;

    /// Fujitsu F2MC16.
    pub const F2MC16: u16 = 0x0068;

    /// Texas Instruments embedded microcontroller msp430.
    ///
    /// This replaces the old value 0x1059, described by
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l291).
    pub const MSP430: u16 = 0x0069;

    /// Analog Devices Blackfin (DSP) processor.
    pub const BLACKFIN: u16 = 0x006A;

    /// S1C33 Family of Seiko Epson processors.
    pub const SE_C33: u16 = 0x006B;

    /// Sharp embedded microprocessor.
    pub const SEP: u16 = 0x006C;

    /// Arca RISC Microprocessor.
    pub const ARCA: u16 = 0x006D;

    /// Microprocessor series from PKU-Unity Ltd. and MPRC of Peking University.
    pub const UNICORE: u16 = 0x006E;

    /// eXcess: 16/32/64-bit configurable embedded CPU.
    pub const EXCESS: u16 = 0x006F;

    /// Icera Semiconductor Inc. Deep Execution Processor.
    pub const DXP: u16 = 0x0070;

    /// Altera Nios II soft-core processor.
    pub const ALTERA_NIOS2: u16 = 0x0071;

    /// National Semiconductor CompactRISC CRX microprocessor.
    pub const CRX: u16 = 0x0072;

    /// Motorola XGATE embedded processor.
    pub const XGATE: u16 = 0x0073;

    /// Infineon C16x/XC16x processor.
    pub const C166: u16 = 0x0074;

    /// Renesas M16C series microprocessors.
    pub const M16C: u16 = 0x0075;

    /// Microchip Technology dsPIC30F Digital Signal Controller.
    pub const DSPIC30F: u16 = 0x0076;

    /// Freescale Communication Engine RISC core.
    pub const CE: u16 = 0x0077;

    /// Renesas M32C series microprocessors.
    pub const M32C: u16 = 0x0078;

    /// Altium TSK3000 core.
    pub const TSK3000: u16 = 0x0083;

    /// Freescale RS08 embedded processor.
    pub const RS08: u16 = 0x0084;

    /// Analog Devices SHARC family of 32-bit DSP processors.
    pub const SHARC: u16 = 0x0085;

    /// Cyan Technology eCOG2 microprocessor.
    pub const ECOG2: u16 = 0x0086;

    /// Sunplus S+core7 RISC processor.
    pub const SCORE7: u16 = 0x0087;

    /// New Japan Radio (NJR) 24-bit DSP Processor.
    pub const DSP24: u16 = 0x0088;

    /// Broadcom VideoCore III processor.
    pub const VIDEOCORE3: u16 = 0x0089;

    /// RISC processor for Lattice FPGA architecture.
    pub const LATTICEMICO32: u16 = 0x008A;

    /// Seiko Epson C17 family.
    pub const SE_C17: u16 = 0x008B;

    /// The Texas Instruments TMS320C6000 DSP family.
    pub const TI_C6000: u16 = 0x008C;

    /// The Texas Instruments TMS320C2000 DSP family.
    pub const TI_C2000: u16 = 0x008D;

    /// The Texas Instruments TMS320C55x DSP family.
    pub const TI_C5500: u16 = 0x008E;

    /// Texas Instruments Application Specific RISC Processor, 32bit fetch.
    pub const TI_ARP32: u16 = 0x008F;

    /// Texas Instruments Programmable Realtime Unit.
    pub const TI_PRU: u16 = 0x0090;

    /// STMicroelectronics 64bit VLIW Data Signal Processor.
    pub const MMDSP_PLUS: u16 = 0x00A0;

    /// Cypress M8C microprocessor.
    pub const CYPRESS_M8C: u16 = 0x00A1;

    /// Renesas R32C series microprocessors.
    pub const R32C: u16 = 0x00A2;

    /// NXP Semiconductors TriMedia architecture family.
    pub const TRIMEDIA: u16 = 0x00A3;

    /// QUALCOMM DSP6 Processor.
    pub const QDSP6: u16 = 0x00A4;

    /// Intel 8051 and variants.
    pub const INTEL_8051: u16 = 0x00A5;

    /// STMicroelectronics STxP7x family of configurable and extensible RISC processors.
    pub const STXP7X: u16 = 0x00A6;

    /// Andes Technology compact code size embedded RISC processor family.
    pub const NDS32: u16 = 0x00A7;

    /// Cyan Technology eCOG1X family.
    pub const ECOG1: u16 = 0x00A8;

    /// Cyan Technology eCOG1X family.
    pub const ECOG1X: u16 = Machine::ECOG1;

    /// Dallas Semiconductor MAXQ30 Core Micro-controllers.
    pub const MAXQ30: u16 = 0x00A9;

    /// New Japan Radio (NJR) 16-bit DSP Processor.
    pub const XIMO16: u16 = 0x00AA;

    /// M2000 Reconfigurable RISC Microprocessor.
    pub const MANIK: u16 = 0x00AB;

    /// Cray Inc. NV2 vector architecture.
    pub const CRAYNV2: u16 = 0x00AC;

    /// Renesas RX family.
    pub const RX: u16 = 0x00AD;

    /// Imagination Technologies META processor architecture.
    pub const METAG: u16 = 0x00AE;

    /// MCST Elbrus general purpose hardware architecture.
    pub const MCST_ELBRUS: u16 = 0x00AF;

    /// Cyan Technology eCOG16 family.
    pub const ECOG16: u16 = 0x00B0;

    /// National Semiconductor CompactRISC CR16 16-bit microprocessor.
    pub const CR16: u16 = 0x00B1;

    /// Freescale Extended Time Processing Unit.
    pub const ETPU: u16 = 0x00B2;

    /// Infineon Technologies SLE9X core.
    pub const SLE9X: u16 = 0x00B3;

    /// Intel L10M.
    pub const L10M: u16 = 0x00B4;

    /// Intel K10M.
    pub const K10M: u16 = 0x00B5;

    /// ARM 64-bit architecture (AARCH64).
    pub const AARCH64: u16 = 0x00B7;

    /// Atmel Corporation 32-bit microprocessor family.
    pub const AVR32: u16 = 0x00B9;

    /// STMicroeletronics STM8 8-bit microcontroller.
    pub const STM8: u16 = 0x00BA;

    /// Tilera TILE64 multicore architecture family.
    pub const TILE64: u16 = 0x00BB;

    /// Tilera TILEPro multicore architecture family.
    pub const TILEPRO: u16 = 0x00BC;

    /// Xilinx MicroBlaze 32-bit RISC soft processor core.
    pub const MICROBLAZE: u16 = 0x00BD;

    /// NVIDIA CUDA architecture.
    pub const CUDA: u16 = 0x00BE;

    /// Tilera TILE-Gx multicore architecture family.
    pub const TILEGX: u16 = 0x00BF;

    /// CloudShield architecture family.
    pub const CLOUDSHIELD: u16 = 0x00C0;

    /// KIPO-KAIST Core-A 1st generation processor family.
    pub const COREA_1ST: u16 = 0x00C1;

    /// KIPO-KAIST Core-A 2nd generation processor family.
    pub const COREA_2ND: u16 = 0x00C2;

    /// Synopsys ARCompact V2.
    pub const ARC_COMPACT2: u16 = 0x00C3;

    /// Open8 8-bit RISC soft processor core.
    pub const OPEN8: u16 = 0x00C4;

    /// Renesas RL78 family.
    pub const RL78: u16 = 0x00C5;

    /// Broadcom VideoCore V processor.
    pub const VIDEOCORE5: u16 = 0x00C6;

    /// Renesas 78KOR family.
    pub const RENESAS_78K0R: u16 = 0x00C7;

    /// Freescale 56800EX Digital Signal Controller (DSC).
    pub const FREESCALE_56800EX: u16 = 0x00C8;

    /// Beyond BA1 CPU architecture.
    pub const BA1: u16 = 0x00C9;

    /// Beyond BA2 CPU architecture.
    pub const BA2: u16 = 0x00CA;

    /// XMOS xCORE processor family.
    pub const XCORE: u16 = 0x00CB;

    /// Microchip 8-bit PIC(r) family.
    pub const MCHP_PIC: u16 = 0x00CC;

    /// Intel Graphics Technology.
    ///
    /// Renamed [2021-03-19](https://groups.google.com/g/generic-abi/c/ofBevXA48dM).
    pub const INTELGT: u16 = 0x00CD;

    /// KM211 KM32 32-bit processor.
    pub const KM32: u16 = 0x00D2;

    /// KM211 KMX32 32-bit processor.
    pub const KMX32: u16 = 0x00D3;

    /// KM211 KMX16 16-bit processor.
    pub const KMX16: u16 = 0x00D4;

    /// KM211 KMX8 8-bit processor.
    pub const KMX8: u16 = 0x00D5;

    /// KM211 KVARC processor.
    pub const KVARC: u16 = 0x00D6;

    /// Paneve CDP architecture family.
    pub const CDP: u16 = 0x00D7;

    /// Cognitive Smart Memory Processor.
    pub const COGE: u16 = 0x00D8;

    /// Bluechip Systems CoolEngine.
    pub const COOL: u16 = 0x00D9;

    /// Nanoradio Optimized RISC.
    pub const NORC: u16 = 0x00DA;

    /// CSR Kalimba architecture family.
    pub const CSR_KALIMBA: u16 = 0x00DB;

    /// Zilog Z80.
    pub const Z80: u16 = 0x00DC;

    /// Controls and Data Services VISIUMcore processor.
    pub const VISIUM: u16 = 0x00DD;

    /// FTDI Chip FT32 high performance 32-bit RISC architecture.
    pub const FT32: u16 = 0x00DE;

    /// Moxie processor family.
    pub const MOXIE: u16 = 0x00DF;

    /// AMD GPU architecture.
    pub const AMDGPU: u16 = 0x00E0;

    /// RISC-V.
    pub const RISCV: u16 = 0x00F3;

    /// Lanai processor.
    pub const LANAI: u16 = 0x00F4;

    /// CEVA Processor Architecture Family.
    pub const CEVA: u16 = 0x00F5;

    /// CEVA X2 Processor Family.
    pub const CEVA_X2: u16 = 0x00F6;

    /// Linux BPF – in-kernel virtual machine.
    pub const BPF: u16 = 0x00F7;

    /// Graphcore Intelligent Processing Unit.
    pub const GRAPHCORE_IPU: u16 = 0x00F8;

    /// Imagination Technologies.
    pub const IMG1: u16 = 0x00F9;

    /// Netronome Flow Processor (NFP).
    pub const NFP: u16 = 0x00FA;

    /// NEC Vector Engine.
    ///
    /// Defined in GNU
    /// [bintuls](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=include/elf/common.h;h=78c85bd514b91bd7ff31f387b4dc98e477d10072;hb=HEAD#l351).
    pub const VE: u16 = 0x00FB;

    /// C-SKY.
    ///
    /// First public at [C-SKY port: ELF and BFD
    /// support](https://sourceware.org/legacy-ml/binutils/2018-05/msg00242.html),
    /// which changed to not conflict with [Machine::MCORE]. Also defined in GNU
    /// [bintuls](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=include/elf/common.h;h=78c85bd514b91bd7ff31f387b4dc98e477d10072;hb=HEAD#l352).
    pub const CSKY: u16 = 0x00FC;

    /// Synopsys ARCv2.3 64-bit.
    ///
    /// Defined in GNU
    /// [bintuls](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=include/elf/common.h;h=78c85bd514b91bd7ff31f387b4dc98e477d10072;hb=HEAD#l353).
    pub const ARC_COMPACT3_64: u16 = 0x00FD;

    /// KMOS Technology MCS 6502 processor.
    ///
    /// Defined in GNU
    /// [bintuls](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=include/elf/common.h;h=78c85bd514b91bd7ff31f387b4dc98e477d10072;hb=HEAD#l354).
    pub const MCS6502: u16 = 0x00FE;

    /// Synopsys ARCv2.3 32-bit.
    ///
    /// Defined in GNU
    /// [bintuls](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=include/elf/common.h;h=78c85bd514b91bd7ff31f387b4dc98e477d10072;hb=HEAD#l355).
    pub const ARC_COMPACT3: u16 = 0x00FF;

    /// Kalray VLIW core of the MPPA processor family.
    ///
    /// Defined in GNU
    /// [bintuls](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=include/elf/common.h;h=78c85bd514b91bd7ff31f387b4dc98e477d10072;hb=HEAD#l356).
    pub const KVX: u16 = 0x0100;

    /// WDC 65816/65C816.
    ///
    /// Assigned [2020-08-18](https://groups.google.com/g/generic-abi/c/qaPzp2lRzDA).
    pub const WDC_65816: u16 = 0x0101;

    /// LoongArch
    ///
    /// Defined in
    /// [glibc](https://sourceware.org/git/?p=glibc.git;a=blob;f=elf/elf.h;h=33aea7f743b885c5d74736276e55ef21756293ee;hb=HEAD#l361).
    pub const LOONGARCH: u16 = 0x0102;

    /// KungFu 32 architecture from ChipON Micro-Electronic Co.
    ///
    /// Assigned [2020-12-11](https://groups.google.com/g/generic-abi/c/n8tLQxj02YY).
    pub const KF32: u16 = 0x0103;

    /// LAPIS nX-U16/U8.
    ///
    /// Assigned [2021-01-15](https://groups.google.com/g/generic-abi/c/57et441Wyho).
    pub const U16_U8CORE: u16 = 0x0104;

    /// Tachyum
    ///
    /// Assigned [2021-08-07](https://groups.google.com/g/generic-abi/c/xGiSaxy5Z4s).
    pub const TACHYUM: u16 = 0x0105;

    /// NXP 56800V4 Digital Signal Controller.
    ///
    /// Assigned
    /// [2022-02-03](https://groups.google.com/g/generic-abi/c/Bwf07opvj4U).
    ///
    /// GNU
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=include/elf/common.h;h=78c85bd514b91bd7ff31f387b4dc98e477d10072;hb=HEAD#l362)
    /// defines this as `EM_56800EF`.
    pub const NXP_56800V4: u16 = 0x0106;

    /// AMD/Xilinx AIEngine.
    ///
    /// Assigned [2023-01-18](https://groups.google.com/g/generic-abi/c/B6kXyqCXVqQ).
    pub const AIENGINE: u16 = 0x0108;

    /// SIMa.ai Neural Network.
    ///
    /// Assigned [2023-02-16](https://groups.google.com/g/generic-abi/c/-fkwJxpG9H0).
    pub const SIMA_MLA: u16 = 0x0109;

    /// Cambricon BANG architecture.
    ///
    /// Assigned [2024-04-03](https://groups.google.com/g/generic-abi/c/vzppcbW_QGQ).
    pub const BANG: u16 = 0x010A;

    /// Loongson LoongGPU.
    ///
    /// Assigned [2024-05-21](https://groups.google.com/g/generic-abi/c/9tr9IcCbWnE)
    pub const LOONGGPU: u16 = 0x010B;

    /// Digital Alpha
    ///
    /// This value does not appear to be official, but is very commonly observed
    /// with modern operating systems (e.g. the BSD variants). It is defined in
    /// [glibc](https://sourceware.org/git/?p=glibc.git;a=blob;f=elf/elf.h;h=33aea7f743b885c5d74736276e55ef21756293ee;hb=HEAD#l373).
    ///
    /// GNU
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l298)
    /// describes this as the "old" ALPHA. We use the term here OLD_ALPHA to
    /// make this clear, although it is commonly used.
    pub const OLD_ALPHA: u16 = 0x9026;

    /// Adapteva Ephiphany architecture.
    ///
    /// Defined by GNU
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=include/elf/common.h;h=78c85bd514b91bd7ff31f387b4dc98e477d10072;hb=HEAD#l453).
    pub const ADAPTEVA_EPIPHANY: u16 = 0x1223;

    /// Morpho MT.
    ///
    /// Defined by GNU
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=include/elf/common.h;h=78c85bd514b91bd7ff31f387b4dc98e477d10072;hb=HEAD#l383).
    pub const MT: u16 = 0x2530;

    /// Webassembly.
    ///
    /// Defined by GNU
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=include/elf/common.h;h=78c85bd514b91bd7ff31f387b4dc98e477d10072;hb=HEAD#l389).
    pub const WEBASSEMBLY: u16 = 0x4157;

    /// Freescale S12Z.
    ///
    /// Defined by GNU
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=include/elf/common.h;h=78c85bd514b91bd7ff31f387b4dc98e477d10072;hb=HEAD#l392).
    pub const S12Z: u16 = 0x4DEF;

    /// DLX.
    ///
    /// Defined by GNU
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l277).
    pub const DLX: u16 = 0x5AA5;

    /// FRV.
    ///
    /// Defined by GNU
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l279).
    pub const FRV: u16 = 0x5441;

    /// Infineon Technologies 16-bit microcontroller with C166-V2 core.
    ///
    /// Defined by GNU
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l281).
    pub const X16X: u16 = 0x4688;

    /// Xstormy16.
    ///
    /// Defined by GNU
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l283).
    pub const XSTORMY16: u16 = 0xAD45;

    /// Vitesse IQ2000.
    ///
    /// Defined by GNU
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l287).
    pub const IQ2000: u16 = 0xFEBA;

    /// NIOS.
    ///
    /// Defined by GNU
    /// [binutils](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l289).
    pub const NIOS32: u16 = 0xFEBB;

    /// Get the byte representation of the OS ABI in the ELF file.
    #[must_use]
    pub const fn machine(&self) -> u16 {
        self.machine
    }

    const fn name(&self) -> Option<&str> {
        match self.machine {
            Machine::NONE => Some("NONE"),
            Machine::M32 => Some("Bellmac 32 AT&T WE 32100"),
            Machine::SPARC => Some("Solaris SPARC"),
            Machine::INTEL_386 => Some("Intel 386"),
            Machine::MOTOROLA_68K => Some("Motorola 68K"),
            Machine::MOTOROLA_88K => Some("Motorola 88K"),
            Machine::IAMCU => Some("Intel MCU"),
            Machine::INTEL_860 => Some("Intel 80860"),
            Machine::MIPS => Some("MIPS"),
            Machine::S370 => Some("IBM System/370"),
            Machine::MIPS_RS3_LE => Some("MIPS RS3000 Little-Endian"),
            Machine::PARISC => Some("Hewlett-Packard PA-RISC"),
            Machine::VPP500 => Some("Fujitsu VPP500/VPP550"),
            Machine::SPARC32PLUS => Some("Solaris SPARC32 V8+"),
            Machine::INTEL_960 => Some("Intel 80960"),
            Machine::PPC => Some("PowerPC"),
            Machine::PPC64 => Some("PowerPC64"),
            Machine::S390 => Some("IBM System/390"),
            Machine::SPU => Some("IBM SPU/SPC"),
            Machine::V800 => Some("NEC V800"),
            Machine::FR20 => Some("Fujitsu FR20"),
            Machine::RH32 => Some("TRW RH-32"),
            Machine::RCE => Some("Motorola M*Core / RCE"),
            Machine::ARM => Some("ARM AArch32"),
            Machine::ALPHA => Some("DEC Alpha"),
            Machine::SH => Some("Hitachi SuperH"),
            Machine::SPARCV9 => Some("Solaris SPARCv9 64-bit"),
            Machine::TRICORE => Some("Siemens TriCore"),
            Machine::ARC => Some("Argonaut RISC Core"),
            Machine::H8_300 => Some("Hitachi H8/300"),
            Machine::H8_300H => Some("Hitachi H8/300H"),
            Machine::H8S => Some("Hitachi H8S"),
            Machine::H8_500 => Some("Hitachi H8/500"),
            Machine::IA_64 => Some("Intel IA-64"),
            Machine::MIPS_X => Some("Stanford MIPS-X"),
            Machine::COLDFIRE => Some("Motorola ColdFire"),
            Machine::MOTOROLA_68HC12 => Some("Motorola 68HC12"),
            Machine::MMA => Some("Fujitsu MMA Multimedia Accelerator"),
            Machine::PCP => Some("Siemens PCP"),
            Machine::NCPU => Some("Sony nCPU RISC"),
            Machine::NDR1 => Some("Denso NDR1"),
            Machine::STARCORE => Some("Motorola Star*Core"),
            Machine::ME16 => Some("Toyota ME16"),
            Machine::ST100 => Some("STMicroelectronics ST100"),
            Machine::TINYJ => Some("Advanced Logic Corp TinyJ"),
            Machine::X86_64 => Some("AMD x86-64"),
            Machine::PDSP => Some("Sony DSP"),
            Machine::PDP10 => Some("DEC PDP-10"),
            Machine::PDP11 => Some("DEC PDP-11"),
            Machine::FX66 => Some("Siemens FX66"),
            Machine::ST9PLUS => Some("STMicroelectronics ST9+ 8/16-bit"),
            Machine::ST7 => Some("STMicroelectronics ST7 8-bit"),
            Machine::MOTOROLA_68HC16 => Some("Motorola 68HC16"),
            Machine::MOTOROLA_68HC11 => Some("Motorola 68HC11"),
            Machine::MOTOROLA_68HC08 => Some("Motorola 68HC08"),
            Machine::MOTOROLA_68HC05 => Some("Motorola 68HC05"),
            Machine::SVX => Some("Silicon Graphics SVx"),
            Machine::ST19 => Some("STMicroelectronics ST19 8-bit"),
            Machine::VAX => Some("Digital VAX"),
            Machine::CRIS => Some("CRIS Axis Communications 32-bit"),
            Machine::JAVELIN => Some("Infineon 32-bit Javelin"),
            Machine::FIREPATH => Some("Element 14 64-bit DSP Firepath"),
            Machine::ZSP => Some("LSI Logic 16-bit DSP ZSP"),
            Machine::MMIX => Some("Donald Knuth's EDU 64-bit"),
            Machine::HUANY => Some("Harvard University Machine-Independent"),
            Machine::PRISM => Some("SiTera Prism"),
            Machine::AVR => Some("Atmel AVR 8-bit"),
            Machine::FR30 => Some("Fujitsu FR30"),
            Machine::D10V => Some("Mitsubishi D10V"),
            Machine::D30V => Some("Mitsubishi D30V"),
            Machine::V850 => Some("NEC v850"),
            Machine::M32R => Some("Mitsubishi M32R"),
            Machine::MN10300 => Some("Matsushita MN10300"),
            Machine::MN10200 => Some("Matsushita MN10200"),
            Machine::PJ => Some("picoJava"),
            Machine::OPENRISC => Some("OpenRISC 32-bit"),
            Machine::ARC_COMPACT => Some("ARCompact"),
            Machine::XTENSA => Some("Tensilica Xtensa"),
            Machine::VIDEOCORE => Some("Alphamosaic VideoCore"),
            Machine::TMM_GPP => Some("Thompson Multimedia General Purpose Processor"),
            Machine::NS32K => Some("National Semiconductor 32000 series"),
            Machine::TPC => Some("Tenor Network TPC"),
            Machine::SNP1K => Some("Trebia SNP 1000"),
            Machine::ST200 => Some("STMicroelectronics ST200"),
            Machine::IP2K => Some("Ubicom IP2xxx"),
            Machine::MAX => Some("MAX"),
            Machine::CR => Some("National Semiconductor CompactRISC"),
            Machine::F2MC16 => Some("Fujitsu F2MC16"),
            Machine::MSP430 => Some("Texas Instruments MSP430"),
            Machine::BLACKFIN => Some("Analog Devices Blackfin DSP"),
            Machine::SE_C33 => Some("Seiko Epson S1C33"),
            Machine::SEP => Some("Sharp embedded"),
            Machine::ARCA => Some("Arca RISC"),
            Machine::UNICORE => Some("PKU-Unity Ltd Peking Unicore"),
            Machine::EXCESS => Some("eXcess 16/32/64-bit"),
            Machine::DXP => Some("Icera Deep Execution Processor"),
            Machine::ALTERA_NIOS2 => Some("Altera Nios II soft-core"),
            Machine::CRX => Some("National Semiconductor CompactRISC CRX"),
            Machine::XGATE => Some("Motorola XGATE"),
            Machine::C166 => Some("Infineon C16x/XC16x"),
            Machine::M16C => Some("Renesas M16C"),
            Machine::DSPIC30F => Some("Microchip Technology dsPIC30F"),
            Machine::CE => Some("Freescale Communication Engine RISC"),
            Machine::M32C => Some("Renesas M32C"),
            Machine::TSK3000 => Some("Altium TSK3000"),
            Machine::RS08 => Some("Freescale RS08"),
            Machine::SHARC => Some("Analog Devices SHARC 32-bit DSP"),
            Machine::ECOG2 => Some("Cyan Technology eCOG2"),
            Machine::SCORE7 => Some("Sunplus S+core7 RISC"),
            Machine::DSP24 => Some("New Japan Radio 24-bit DSP"),
            Machine::VIDEOCORE3 => Some("Broadcome VideoCore III"),
            Machine::LATTICEMICO32 => Some("Lattice FPGA RISC"),
            Machine::SE_C17 => Some("Seiko Epson C17"),
            Machine::TI_C6000 => Some("Texas Instruments TMS320C6000 DSP"),
            Machine::TI_C2000 => Some("Texas Instruments TMS320C2000 DSP"),
            Machine::TI_C5500 => Some("Texas Instruments TMS320C55x DSP"),
            Machine::TI_ARP32 => Some("Texas Instruments Application Specific RISC"),
            Machine::TI_PRU => Some("Texas Instruments Programmable Realtime Unit"),
            Machine::MMDSP_PLUS => Some("STMicroelectronics 64bit VLIW DSP"),
            Machine::CYPRESS_M8C => Some("Cypress M8C"),
            Machine::R32C => Some("Renesas R32C"),
            Machine::TRIMEDIA => Some("NXP Semiconductors TriMedia architecture"),
            Machine::QDSP6 => Some("QUALCOMM DSP6"),
            Machine::INTEL_8051 => Some("Intel 8051"),
            Machine::STXP7X => Some("STMicroelectronics STxP7x RISC"),
            Machine::NDS32 => Some("Andes Technology embedded RISC"),
            Machine::ECOG1 => Some("Cyan Technology eCOG1X"),
            Machine::MAXQ30 => Some("Dallas Semiconductor MAXQ30 Core"),
            Machine::XIMO16 => Some("New Japan Radio 16-bit DSP"),
            Machine::MANIK => Some("M2000 Reconfigurable RISC Manik"),
            Machine::CRAYNV2 => Some("Cray Inc. NV2 vector architecture"),
            Machine::RX => Some("Renesas RX"),
            Machine::METAG => Some("Imagination Technologies META"),
            Machine::MCST_ELBRUS => Some("MCST Elbrus"),
            Machine::ECOG16 => Some("Cyan Technology eCOG16"),
            Machine::CR16 => Some("National Semiconductor CompactRISC CR16 16-bit"),
            Machine::ETPU => Some("Freescale Extended Time Processing Unit"),
            Machine::SLE9X => Some("Infineon Technologies SLE9X"),
            Machine::L10M => Some("Intel L10M"),
            Machine::K10M => Some("Intel K10M"),
            Machine::AARCH64 => Some("ARM 64-bit"),
            Machine::AVR32 => Some("Atmel Corporation 32-bit"),
            Machine::STM8 => Some("STMicroeletronics STM8 8-bit"),
            Machine::TILE64 => Some("Tilera TILE64 multicore"),
            Machine::TILEPRO => Some("Tilera TILEPro multicore"),
            Machine::MICROBLAZE => Some("Xilinx MicroBlaze 32-bit RISC"),
            Machine::CUDA => Some("NVIDIA CUDA"),
            Machine::TILEGX => Some("Tilera TILE-Gx multicore"),
            Machine::CLOUDSHIELD => Some("CloudShield"),
            Machine::COREA_1ST => Some("KIPO-KAIST Core-A 1st generation"),
            Machine::COREA_2ND => Some("KIPO-KAIST Core-A 2nd generation"),
            Machine::ARC_COMPACT2 => Some("Synopsys ARCompact V2"),
            Machine::OPEN8 => Some("Open8 8-bit RISC"),
            Machine::RL78 => Some("Renesas RL78"),
            Machine::VIDEOCORE5 => Some("Broadcom VideoCore V"),
            Machine::RENESAS_78K0R => Some("Renesas 78KOR"),
            Machine::FREESCALE_56800EX => Some("Freescale 56800EX Digital Signal Controller"),
            Machine::BA1 => Some("Beyond BA1 CPU"),
            Machine::BA2 => Some("Beyond BA2 CPU"),
            Machine::XCORE => Some("XMOS xCORE"),
            Machine::MCHP_PIC => Some("Microchip 8-bit PIC(r)"),
            Machine::INTELGT => Some("Intel Graphics Technology"),
            Machine::KM32 => Some("KM211 KM32 32-bit"),
            Machine::KMX32 => Some("KM211 KMX32 32-bit"),
            Machine::KMX16 => Some("KM211 KMX16 16-bit"),
            Machine::KMX8 => Some("KM211 KMX8 8-bit"),
            Machine::KVARC => Some("KM211 KVARC"),
            Machine::CDP => Some("Paneve CDP"),
            Machine::COGE => Some("Cognitive Smart Memory Processor"),
            Machine::COOL => Some("Bluechip Systems CoolEngine"),
            Machine::NORC => Some("Nanoradio Optimized RISC"),
            Machine::CSR_KALIMBA => Some("CSR Kalimba"),
            Machine::Z80 => Some("Zilog Z80"),
            Machine::VISIUM => Some("Controls and Data Services VISIUMcore"),
            Machine::FT32 => Some("FTDI Chip FT32 high performance 32-bit RISC"),
            Machine::MOXIE => Some("Moxie"),
            Machine::AMDGPU => Some("AMD GPU"),
            Machine::RISCV => Some("RISC-V"),
            Machine::LANAI => Some("Lanai"),
            Machine::CEVA => Some("CEVA"),
            Machine::CEVA_X2 => Some("CEVA X2"),
            Machine::BPF => Some("Linux BPF VM"),
            Machine::GRAPHCORE_IPU => Some("Graphcore Intelligent Processing Unit"),
            Machine::IMG1 => Some("Imagination Technologies"),
            Machine::NFP => Some("Netronome Flow Processor"),
            Machine::VE => Some("NEC Vector Engine"),
            Machine::CSKY => Some("C-SKY"),
            Machine::ARC_COMPACT3_64 => Some("Synopsys ARCv2.3 64-bit"),
            Machine::MCS6502 => Some("KMOS Technology MCS 6502"),
            Machine::ARC_COMPACT3 => Some("Synopsys ARCv2.3 32-bit"),
            Machine::KVX => Some("Kalray VLIW core of MPPA"),
            Machine::WDC_65816 => Some("WDC 65816/65C816"),
            Machine::LOONGARCH => Some("LoongArch"),
            Machine::KF32 => Some("ChipON Micro-Electronic Co. KungFu 32"),
            Machine::U16_U8CORE => Some("LAPIS nX-U16/U8"),
            Machine::TACHYUM => Some("Tachyum"),
            Machine::NXP_56800V4 => Some("NXP 56800V4 Digital Signal Controller"),
            Machine::AIENGINE => Some("AMD/Xilinx AIEngine"),
            Machine::SIMA_MLA => Some("SIMa.ai Neural Network"),
            Machine::BANG => Some("Cambricon BANG"),
            Machine::LOONGGPU => Some("Loongson LoongGPU"),

            // Unofficial values. Taken from binutils.
            Machine::OLD_ALPHA => Some("Alpha 9026"),
            Machine::ADAPTEVA_EPIPHANY => Some("Adapteva Ephiphany"),
            Machine::MT => Some("Morpho MT"),
            Machine::WEBASSEMBLY => Some("Webassembly"),
            Machine::S12Z => Some("Freescale S12Z"),
            Machine::DLX => Some("DLX"),
            Machine::FRV => Some("FRV (cygnus)"),
            Machine::X16X => Some("Infineon Technologies 16-bit microcontroller with C166-V2 core"),
            Machine::XSTORMY16 => Some("Xstormy16"),
            Machine::IQ2000 => Some("Vitesse IQ2000"),
            Machine::NIOS32 => Some("NIOS"),
            0x1057 => Some("AVR (old)"),
            0x1059 => Some("MSP430 (old)"),
            0x3330 => Some("FR30 (cygnus)"),
            0x7650 => Some("D10V (cygnus)"),
            0x7676 => Some("D30V (cygnus)"),
            0x8217 => Some("IP2K (old)"),
            0x9025 => Some("PowerPC (cygnus)"),
            0x9041 => Some("M32R (cygnus)"),
            0x9080 => Some("V850 (cygnus)"),
            0xA390 => Some("S/390 (old)"),
            0xABC7 => Some("Xtensa (old)"),
            0xBAAB => Some("Microblaze (old)"),
            0xBEEF => Some("MN10300 (cygnus)"),
            0xDEAD => Some("MN10200 (cygnus)"),
            0xF00D => Some("Toyota MeP"),
            0xFEB0 => Some("Renesas M32C (old)"),
            0xFEED => Some("Moxie (old)"),
            _ => None,
        }
    }
}

impl From<u16> for Machine {
    #[must_use]
    fn from(v: u16) -> Self {
        Machine { machine: v }
    }
}

impl From<Machine> for u16 {
    #[must_use]
    fn from(v: Machine) -> Self {
        v.machine
    }
}

impl fmt::Display for Machine {
    /// Format the machine name to a more descriptive name that can be printed.
    ///
    /// Takes the machine value and prints the name for it. Official names are
    /// preferred over "old" or unofficial names. This means that unofficial
    /// names may change in the future. On conflicts, only the machine value is
    /// printed.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Some(v) => write!(f, "{}", v),
            None => write!(f, "Machine 0x{:0>4X}", self.machine),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Machine;
    use std::ops::Bound::*;
    use std::ops::RangeBounds;

    #[test]
    fn abi_string_sco() {
        // Note the explicit avoidance of the constant, to check against the SCO
        // documentation,
        // https://www.sco.com/developers/gabi/latest/ch4.eheader.html. Values
        // that are out of date are removed. Reserved values are not listed
        // here.

        assert_eq!(Machine::from(0).to_string(), "NONE");
        assert_eq!(Machine::from(1).to_string(), "Bellmac 32 AT&T WE 32100");
        assert_eq!(Machine::from(2).to_string(), "Solaris SPARC");
        assert_eq!(Machine::from(3).to_string(), "Intel 386");
        assert_eq!(Machine::from(4).to_string(), "Motorola 68K");
        assert_eq!(Machine::from(5).to_string(), "Motorola 88K");
        assert_eq!(Machine::from(6).to_string(), "Intel MCU");
        assert_eq!(Machine::from(7).to_string(), "Intel 80860");
        assert_eq!(Machine::from(8).to_string(), "MIPS");
        assert_eq!(Machine::from(9).to_string(), "IBM System/370");
        assert_eq!(Machine::from(10).to_string(), "MIPS RS3000 Little-Endian");
        assert_eq!(Machine::from(15).to_string(), "Hewlett-Packard PA-RISC");
        assert_eq!(Machine::from(17).to_string(), "Fujitsu VPP500/VPP550");
        assert_eq!(Machine::from(18).to_string(), "Solaris SPARC32 V8+");
        assert_eq!(Machine::from(19).to_string(), "Intel 80960");
        assert_eq!(Machine::from(20).to_string(), "PowerPC");
        assert_eq!(Machine::from(21).to_string(), "PowerPC64");
        assert_eq!(Machine::from(22).to_string(), "IBM System/390");
        assert_eq!(Machine::from(23).to_string(), "IBM SPU/SPC");
        assert_eq!(Machine::from(36).to_string(), "NEC V800");
        assert_eq!(Machine::from(37).to_string(), "Fujitsu FR20");
        assert_eq!(Machine::from(38).to_string(), "TRW RH-32");
        assert_eq!(Machine::from(39).to_string(), "Motorola M*Core / RCE");
        assert_eq!(Machine::from(40).to_string(), "ARM AArch32");
        assert_eq!(Machine::from(41).to_string(), "DEC Alpha");
        assert_eq!(Machine::from(42).to_string(), "Hitachi SuperH");
        assert_eq!(Machine::from(43).to_string(), "Solaris SPARCv9 64-bit");
        assert_eq!(Machine::from(44).to_string(), "Siemens TriCore");
        assert_eq!(Machine::from(45).to_string(), "Argonaut RISC Core");
        assert_eq!(Machine::from(46).to_string(), "Hitachi H8/300");
        assert_eq!(Machine::from(47).to_string(), "Hitachi H8/300H");
        assert_eq!(Machine::from(48).to_string(), "Hitachi H8S");
        assert_eq!(Machine::from(49).to_string(), "Hitachi H8/500");
        assert_eq!(Machine::from(50).to_string(), "Intel IA-64");
        assert_eq!(Machine::from(51).to_string(), "Stanford MIPS-X");
        assert_eq!(Machine::from(52).to_string(), "Motorola ColdFire");
        assert_eq!(Machine::from(53).to_string(), "Motorola 68HC12");
        assert_eq!(
            Machine::from(54).to_string(),
            "Fujitsu MMA Multimedia Accelerator"
        );
        assert_eq!(Machine::from(55).to_string(), "Siemens PCP");
        assert_eq!(Machine::from(56).to_string(), "Sony nCPU RISC");
        assert_eq!(Machine::from(57).to_string(), "Denso NDR1");
        assert_eq!(Machine::from(58).to_string(), "Motorola Star*Core");
        assert_eq!(Machine::from(59).to_string(), "Toyota ME16");
        assert_eq!(Machine::from(60).to_string(), "STMicroelectronics ST100");
        assert_eq!(Machine::from(61).to_string(), "Advanced Logic Corp TinyJ");
        assert_eq!(Machine::from(62).to_string(), "AMD x86-64");
        assert_eq!(Machine::from(63).to_string(), "Sony DSP");
        assert_eq!(Machine::from(64).to_string(), "DEC PDP-10");
        assert_eq!(Machine::from(65).to_string(), "DEC PDP-11");
        assert_eq!(Machine::from(66).to_string(), "Siemens FX66");
        assert_eq!(
            Machine::from(67).to_string(),
            "STMicroelectronics ST9+ 8/16-bit"
        );
        assert_eq!(
            Machine::from(68).to_string(),
            "STMicroelectronics ST7 8-bit"
        );
        assert_eq!(Machine::from(69).to_string(), "Motorola 68HC16");
        assert_eq!(Machine::from(70).to_string(), "Motorola 68HC11");
        assert_eq!(Machine::from(71).to_string(), "Motorola 68HC08");
        assert_eq!(Machine::from(72).to_string(), "Motorola 68HC05");
        assert_eq!(Machine::from(73).to_string(), "Silicon Graphics SVx");
        assert_eq!(
            Machine::from(74).to_string(),
            "STMicroelectronics ST19 8-bit"
        );
        assert_eq!(Machine::from(75).to_string(), "Digital VAX");
        assert_eq!(
            Machine::from(76).to_string(),
            "CRIS Axis Communications 32-bit"
        );
        assert_eq!(Machine::from(77).to_string(), "Infineon 32-bit Javelin");
        assert_eq!(
            Machine::from(78).to_string(),
            "Element 14 64-bit DSP Firepath"
        );
        assert_eq!(Machine::from(79).to_string(), "LSI Logic 16-bit DSP ZSP");
        assert_eq!(Machine::from(80).to_string(), "Donald Knuth's EDU 64-bit");
        assert_eq!(
            Machine::from(81).to_string(),
            "Harvard University Machine-Independent"
        );
        assert_eq!(Machine::from(82).to_string(), "SiTera Prism");
        assert_eq!(Machine::from(83).to_string(), "Atmel AVR 8-bit");
        assert_eq!(Machine::from(84).to_string(), "Fujitsu FR30");
        assert_eq!(Machine::from(85).to_string(), "Mitsubishi D10V");
        assert_eq!(Machine::from(86).to_string(), "Mitsubishi D30V");
        assert_eq!(Machine::from(87).to_string(), "NEC v850");
        assert_eq!(Machine::from(88).to_string(), "Mitsubishi M32R");
        assert_eq!(Machine::from(89).to_string(), "Matsushita MN10300");
        assert_eq!(Machine::from(90).to_string(), "Matsushita MN10200");
        assert_eq!(Machine::from(91).to_string(), "picoJava");
        assert_eq!(Machine::from(92).to_string(), "OpenRISC 32-bit");
        assert_eq!(Machine::from(93).to_string(), "ARCompact");
        assert_eq!(Machine::from(94).to_string(), "Tensilica Xtensa");
        assert_eq!(Machine::from(95).to_string(), "Alphamosaic VideoCore");
        assert_eq!(
            Machine::from(96).to_string(),
            "Thompson Multimedia General Purpose Processor"
        );
        assert_eq!(
            Machine::from(97).to_string(),
            "National Semiconductor 32000 series"
        );
        assert_eq!(Machine::from(98).to_string(), "Tenor Network TPC");
        assert_eq!(Machine::from(99).to_string(), "Trebia SNP 1000");
        assert_eq!(Machine::from(100).to_string(), "STMicroelectronics ST200");
        assert_eq!(Machine::from(101).to_string(), "Ubicom IP2xxx");
        assert_eq!(Machine::from(102).to_string(), "MAX");
        assert_eq!(
            Machine::from(103).to_string(),
            "National Semiconductor CompactRISC"
        );
        assert_eq!(Machine::from(104).to_string(), "Fujitsu F2MC16");
        assert_eq!(Machine::from(105).to_string(), "Texas Instruments MSP430");
        assert_eq!(
            Machine::from(106).to_string(),
            "Analog Devices Blackfin DSP"
        );
        assert_eq!(Machine::from(107).to_string(), "Seiko Epson S1C33");
        assert_eq!(Machine::from(108).to_string(), "Sharp embedded");
        assert_eq!(Machine::from(109).to_string(), "Arca RISC");
        assert_eq!(
            Machine::from(110).to_string(),
            "PKU-Unity Ltd Peking Unicore"
        );
        assert_eq!(Machine::from(111).to_string(), "eXcess 16/32/64-bit");
        assert_eq!(
            Machine::from(112).to_string(),
            "Icera Deep Execution Processor"
        );
        assert_eq!(Machine::from(113).to_string(), "Altera Nios II soft-core");
        assert_eq!(
            Machine::from(114).to_string(),
            "National Semiconductor CompactRISC CRX"
        );
        assert_eq!(Machine::from(115).to_string(), "Motorola XGATE");
        assert_eq!(Machine::from(116).to_string(), "Infineon C16x/XC16x");
        assert_eq!(Machine::from(117).to_string(), "Renesas M16C");
        assert_eq!(
            Machine::from(118).to_string(),
            "Microchip Technology dsPIC30F"
        );
        assert_eq!(
            Machine::from(119).to_string(),
            "Freescale Communication Engine RISC"
        );
        assert_eq!(Machine::from(120).to_string(), "Renesas M32C");
        assert_eq!(Machine::from(131).to_string(), "Altium TSK3000");
        assert_eq!(Machine::from(132).to_string(), "Freescale RS08");
        assert_eq!(
            Machine::from(133).to_string(),
            "Analog Devices SHARC 32-bit DSP"
        );
        assert_eq!(Machine::from(134).to_string(), "Cyan Technology eCOG2");
        assert_eq!(Machine::from(135).to_string(), "Sunplus S+core7 RISC");
        assert_eq!(Machine::from(136).to_string(), "New Japan Radio 24-bit DSP");
        assert_eq!(Machine::from(137).to_string(), "Broadcome VideoCore III");
        assert_eq!(Machine::from(138).to_string(), "Lattice FPGA RISC");
        assert_eq!(Machine::from(139).to_string(), "Seiko Epson C17");
        assert_eq!(
            Machine::from(140).to_string(),
            "Texas Instruments TMS320C6000 DSP"
        );
        assert_eq!(
            Machine::from(141).to_string(),
            "Texas Instruments TMS320C2000 DSP"
        );
        assert_eq!(
            Machine::from(142).to_string(),
            "Texas Instruments TMS320C55x DSP"
        );
        assert_eq!(
            Machine::from(143).to_string(),
            "Texas Instruments Application Specific RISC"
        );
        assert_eq!(
            Machine::from(144).to_string(),
            "Texas Instruments Programmable Realtime Unit"
        );
        assert_eq!(
            Machine::from(160).to_string(),
            "STMicroelectronics 64bit VLIW DSP"
        );
        assert_eq!(Machine::from(161).to_string(), "Cypress M8C");
        assert_eq!(Machine::from(162).to_string(), "Renesas R32C");
        assert_eq!(
            Machine::from(163).to_string(),
            "NXP Semiconductors TriMedia architecture"
        );
        assert_eq!(Machine::from(164).to_string(), "QUALCOMM DSP6");
        assert_eq!(Machine::from(165).to_string(), "Intel 8051");
        assert_eq!(
            Machine::from(166).to_string(),
            "STMicroelectronics STxP7x RISC"
        );
        assert_eq!(
            Machine::from(167).to_string(),
            "Andes Technology embedded RISC"
        );
        assert_eq!(Machine::from(168).to_string(), "Cyan Technology eCOG1X");
        assert_eq!(
            Machine::from(169).to_string(),
            "Dallas Semiconductor MAXQ30 Core"
        );
        assert_eq!(Machine::from(170).to_string(), "New Japan Radio 16-bit DSP");
        assert_eq!(
            Machine::from(171).to_string(),
            "M2000 Reconfigurable RISC Manik"
        );
        assert_eq!(
            Machine::from(172).to_string(),
            "Cray Inc. NV2 vector architecture"
        );
        assert_eq!(Machine::from(173).to_string(), "Renesas RX");
        assert_eq!(
            Machine::from(174).to_string(),
            "Imagination Technologies META"
        );
        assert_eq!(Machine::from(175).to_string(), "MCST Elbrus");
        assert_eq!(Machine::from(176).to_string(), "Cyan Technology eCOG16");
        assert_eq!(
            Machine::from(177).to_string(),
            "National Semiconductor CompactRISC CR16 16-bit"
        );
        assert_eq!(
            Machine::from(178).to_string(),
            "Freescale Extended Time Processing Unit"
        );
        assert_eq!(
            Machine::from(179).to_string(),
            "Infineon Technologies SLE9X"
        );
        assert_eq!(Machine::from(180).to_string(), "Intel L10M");
        assert_eq!(Machine::from(181).to_string(), "Intel K10M");
        assert_eq!(Machine::from(183).to_string(), "ARM 64-bit");
        assert_eq!(Machine::from(185).to_string(), "Atmel Corporation 32-bit");
        assert_eq!(
            Machine::from(186).to_string(),
            "STMicroeletronics STM8 8-bit"
        );
        assert_eq!(Machine::from(187).to_string(), "Tilera TILE64 multicore");
        assert_eq!(Machine::from(188).to_string(), "Tilera TILEPro multicore");
        assert_eq!(
            Machine::from(189).to_string(),
            "Xilinx MicroBlaze 32-bit RISC"
        );
        assert_eq!(Machine::from(190).to_string(), "NVIDIA CUDA");
        assert_eq!(Machine::from(191).to_string(), "Tilera TILE-Gx multicore");
        assert_eq!(Machine::from(192).to_string(), "CloudShield");
        assert_eq!(
            Machine::from(193).to_string(),
            "KIPO-KAIST Core-A 1st generation"
        );
        assert_eq!(
            Machine::from(194).to_string(),
            "KIPO-KAIST Core-A 2nd generation"
        );
        assert_eq!(Machine::from(195).to_string(), "Synopsys ARCompact V2");
        assert_eq!(Machine::from(196).to_string(), "Open8 8-bit RISC");
        assert_eq!(Machine::from(197).to_string(), "Renesas RL78");
        assert_eq!(Machine::from(198).to_string(), "Broadcom VideoCore V");
        assert_eq!(Machine::from(199).to_string(), "Renesas 78KOR");
        assert_eq!(
            Machine::from(200).to_string(),
            "Freescale 56800EX Digital Signal Controller"
        );
        assert_eq!(Machine::from(201).to_string(), "Beyond BA1 CPU");
        assert_eq!(Machine::from(202).to_string(), "Beyond BA2 CPU");
        assert_eq!(Machine::from(203).to_string(), "XMOS xCORE");
        assert_eq!(Machine::from(204).to_string(), "Microchip 8-bit PIC(r)");
        assert_eq!(Machine::from(210).to_string(), "KM211 KM32 32-bit");
        assert_eq!(Machine::from(211).to_string(), "KM211 KMX32 32-bit");
        assert_eq!(Machine::from(212).to_string(), "KM211 KMX16 16-bit");
        assert_eq!(Machine::from(213).to_string(), "KM211 KMX8 8-bit");
        assert_eq!(Machine::from(214).to_string(), "KM211 KVARC");
        assert_eq!(Machine::from(215).to_string(), "Paneve CDP");
        assert_eq!(
            Machine::from(216).to_string(),
            "Cognitive Smart Memory Processor"
        );
        assert_eq!(
            Machine::from(217).to_string(),
            "Bluechip Systems CoolEngine"
        );
        assert_eq!(Machine::from(218).to_string(), "Nanoradio Optimized RISC");
        assert_eq!(Machine::from(219).to_string(), "CSR Kalimba");
        assert_eq!(Machine::from(220).to_string(), "Zilog Z80");
        assert_eq!(
            Machine::from(221).to_string(),
            "Controls and Data Services VISIUMcore"
        );
        assert_eq!(
            Machine::from(222).to_string(),
            "FTDI Chip FT32 high performance 32-bit RISC"
        );
        assert_eq!(Machine::from(223).to_string(), "Moxie");
        assert_eq!(Machine::from(224).to_string(), "AMD GPU");
        assert_eq!(Machine::from(243).to_string(), "RISC-V");
    }

    #[test]
    fn abi_string_sco_group() {
        // Note the explicit avoidance of the constant, to check against the
        // Google group "generic-abi"
        // https://groups.google.com/g/generic-abi/c/cmq1LFFpWqU. They may
        // change original documentation, which means test `abi_string_sco` has
        // it removed and added here. This should have precedence over other
        // constants.

        assert_eq!(Machine::from(205).to_string(), "Intel Graphics Technology");
        assert_eq!(Machine::from(244).to_string(), "Lanai");
        assert_eq!(Machine::from(245).to_string(), "CEVA");
        assert_eq!(Machine::from(246).to_string(), "CEVA X2");
        assert_eq!(Machine::from(247).to_string(), "Linux BPF VM");
        assert_eq!(
            Machine::from(248).to_string(),
            "Graphcore Intelligent Processing Unit"
        );
        assert_eq!(Machine::from(249).to_string(), "Imagination Technologies");
        assert_eq!(Machine::from(250).to_string(), "Netronome Flow Processor");
        assert_eq!(Machine::from(252).to_string(), "C-SKY");
        assert_eq!(Machine::from(257).to_string(), "WDC 65816/65C816");
        assert_eq!(Machine::from(258).to_string(), "LoongArch");
        assert_eq!(
            Machine::from(259).to_string(),
            "ChipON Micro-Electronic Co. KungFu 32"
        );
        assert_eq!(Machine::from(260).to_string(), "LAPIS nX-U16/U8");
        assert_eq!(Machine::from(261).to_string(), "Tachyum");
        assert_eq!(
            Machine::from(262).to_string(),
            "NXP 56800V4 Digital Signal Controller"
        );
        assert_eq!(Machine::from(264).to_string(), "AMD/Xilinx AIEngine");
        assert_eq!(Machine::from(265).to_string(), "SIMa.ai Neural Network");
        assert_eq!(Machine::from(266).to_string(), "Cambricon BANG");
        assert_eq!(Machine::from(267).to_string(), "Loongson LoongGPU");
    }

    #[test]
    fn abi_string_binutils() {
        // Note the explicit avoidance of the constant, to check against
        // constants defined in GNU binutils, but don't appear anywhere else.
        // Fills in some gaps and historical information.

        assert_eq!(Machine::from(251).to_string(), "NEC Vector Engine");
        assert_eq!(Machine::from(253).to_string(), "Synopsys ARCv2.3 64-bit");
        assert_eq!(Machine::from(254).to_string(), "KMOS Technology MCS 6502");
        assert_eq!(Machine::from(255).to_string(), "Synopsys ARCv2.3 32-bit");
        assert_eq!(Machine::from(256).to_string(), "Kalray VLIW core of MPPA");

        assert_eq!(Machine::from(0x1057).to_string(), "AVR (old)");
        assert_eq!(Machine::from(0x1059).to_string(), "MSP430 (old)");
        assert_eq!(Machine::from(0x1223).to_string(), "Adapteva Ephiphany");
        assert_eq!(Machine::from(0x2530).to_string(), "Morpho MT");
        assert_eq!(Machine::from(0x3330).to_string(), "FR30 (cygnus)");
        assert_eq!(Machine::from(0x4157).to_string(), "Webassembly");
        assert_eq!(
            Machine::from(0x4688).to_string(),
            "Infineon Technologies 16-bit microcontroller with C166-V2 core"
        );
        assert_eq!(Machine::from(0x4DEF).to_string(), "Freescale S12Z");
        assert_eq!(Machine::from(0x5AA5).to_string(), "DLX");
        assert_eq!(Machine::from(0x5441).to_string(), "FRV (cygnus)");
        assert_eq!(Machine::from(0x7650).to_string(), "D10V (cygnus)");
        assert_eq!(Machine::from(0x7676).to_string(), "D30V (cygnus)");
        assert_eq!(Machine::from(0x8217).to_string(), "IP2K (old)");
        assert_eq!(Machine::from(0x9025).to_string(), "PowerPC (cygnus)");
        assert_eq!(Machine::from(0x9026).to_string(), "Alpha 9026");
        assert_eq!(Machine::from(0x9041).to_string(), "M32R (cygnus)");
        assert_eq!(Machine::from(0x9080).to_string(), "V850 (cygnus)");
        assert_eq!(Machine::from(0xA390).to_string(), "S/390 (old)");
        assert_eq!(Machine::from(0xABC7).to_string(), "Xtensa (old)");
        assert_eq!(Machine::from(0xAD45).to_string(), "Xstormy16");
        assert_eq!(Machine::from(0xBAAB).to_string(), "Microblaze (old)");
        assert_eq!(Machine::from(0xBEEF).to_string(), "MN10300 (cygnus)");
        assert_eq!(Machine::from(0xDEAD).to_string(), "MN10200 (cygnus)");
        assert_eq!(Machine::from(0xF00D).to_string(), "Toyota MeP");
        assert_eq!(Machine::from(0xFEB0).to_string(), "Renesas M32C (old)");
        assert_eq!(Machine::from(0xFEBA).to_string(), "Vitesse IQ2000");
        assert_eq!(Machine::from(0xFEBB).to_string(), "NIOS");
        assert_eq!(Machine::from(0xFEED).to_string(), "Moxie (old)");
    }

    #[test]
    fn abi_string_reserved_values() {
        abi_string_reserved_range(11..=14);
        abi_string_reserved_value(16);
        abi_string_reserved_range(24..=35);
        abi_string_reserved_range(145..=159);
        abi_string_reserved_value(182);
        abi_string_reserved_value(184);
        abi_string_reserved_range(206..=209);
        abi_string_reserved_range(225..=242);
        abi_string_reserved_range(268..=0x1000);
    }

    fn abi_string_reserved_value(machine: u16) {
        let actual = Machine::from(machine).to_string();
        let expected = format!("Machine 0x{:0>4X}", machine);
        assert_eq!(actual, expected, "for value {}", machine);
    }

    fn abi_string_reserved_range<R: RangeBounds<u16>>(range: R) {
        let start: u16 = match range.start_bound() {
            Included(&v) => v,
            Excluded(&v) => v + 1,
            Unbounded => u16::MIN,
        };
        let end: u16 = match range.end_bound() {
            Included(&v) => v,
            Excluded(&v) => v - 1,
            Unbounded => u16::MAX,
        };

        if start > end {
            // In case the test case provided an invalid range - fix the test
            // case.
            panic!("Start {start} is greater than {end}");
        }

        for machine in start..=end {
            abi_string_reserved_value(machine);
        }
    }

    #[test]
    fn from_integer() {
        let machine = Machine::from(Machine::ARM);

        let v: u16 = machine.into();
        assert_eq!(v, Machine::ARM);

        assert_eq!(machine.machine(), Machine::ARM);
    }
}
