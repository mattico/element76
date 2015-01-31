use core::str::from_utf8_unchecked;
use core::slice::SliceExt;
use core::mem;
use core::cmp::min;

pub const VENDOR_OLDAMD:		&'static str = "AMDisbetter!";
pub const VENDOR_AMD:			&'static str = "AuthenticAMD";
pub const VENDOR_INTEL:			&'static str = "GenuineIntel";
pub const VENDOR_OLDTRANSMETA:	&'static str = "TransmetaCPU";
pub const VENDOR_TRANSMETA:		&'static str = "GenuineTMx86";
pub const VENDOR_CYRIX:			&'static str = "CyrixInstead";
pub const VENDOR_CENTAUR:		&'static str = "CentaurHauls";
pub const VENDOR_NEXGEN:		&'static str = "NexGenDriven";
pub const VENDOR_UMC:			&'static str = "UMC UMC UMC ";
pub const VENDOR_SIS:			&'static str = "SiS SiS SiS ";
pub const VENDOR_NSC:			&'static str = "Geode by NSC";
pub const VENDOR_RISE:			&'static str = "RiseRiseRise";
pub const VENDOR_VIA:			&'static str = "VIA VIA VIA ";
pub const VENDOR_VORTEX:		&'static str = "Vortex86 SoC";
pub const VENDOR_KVM:			&'static str = "KVMKVMKVMKVM";
pub const VENDOR_HYPERV:		&'static str = "Microsoft Hv";
pub const VENDOR_VMWARE:		&'static str = "VMwareVMware";
pub const VENDOR_XEN:			&'static str = "XenVMMXenVMM";


enum FeatureType {
	PROCESSOR_INFO 				= 0x1,
	EXTENDED_FEATURES			= 0x7,
	EXTENDED_PROCESSOR_INFO		= 0x8000_0001,
}

enum Register {
	EAX,
	EBX,
	ECX,
	EDX,
}

struct Feature {
	feature_type: FeatureType,
	register: Register,
	bitmask: u32,
}

// TODO: Add Features for the rest of the CPUID Instructions (cache, etc)

// This slight ugliness makes things easier from the user side
// since they won't need to know which register the feature is in.
mod features {

	use ::platform::cpu::cpuid::{Feature, FeatureType, Register};

	const STEPPING:			Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EAX, bitmask: 0x000000F };
	const MODEL:			Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EAX, bitmask: 0x00000F0 };
	const FAMILY:			Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EAX, bitmask: 0x0000F00 };
	const PROCESSOR_TYPE:	Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EAX, bitmask: 0x0003000 };
	const EXTENDED_MODEL:	Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EAX, bitmask: 0x00F0000 };
	const EXTENDED_FAMILY:	Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EAX, bitmask: 0xFF00000 };

	const SSE3:			Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 0 };
	const PCLMUL:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 1 };
	const DTES64:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 2 };
	const MONITOR:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 3 };
	const DS_CPL:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 4 };
	const VMX:			Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 5 };
	const SMX:			Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 6 };
	const EST:			Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 7 };
	const TM2:			Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 8 };
	const SSSE3:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 9 };
	const CID:			Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 10 };
	const FMA:			Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 12 };
	const CX16:			Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 13 };
	const ETPRD:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 14 };
	const PDCM:			Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 15 };
	const PCID:			Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 17 };
	const DCA:			Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 18 };
	const SSE4_1:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 19 };
	const SSE4_2:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 20 };
	const X2APIC:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 21 };
	const MOVBE:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 22 };
	const POPCNT:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 23 };
	const TSC_DEADLINE:	Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 24 };
	const AES:			Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 25 };
	const XSAVE:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 26 };
	const OSXSAVE:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 27 };
	const AVX:			Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 28 };
	const F16C:			Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 29 };
	const RDRND:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 30 };
	const HYPERVISOR:	Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 31 };

	const FPU:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 0 };
	const VME:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 1 };
	const DE:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 2 };
	const PSE:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 3 };
	const TSC:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 4 };
	const MSR:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 5 };
	const PAE:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 6 };
	const MCE:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 7 };
	const CX8:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 8 };
	const APIC:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 9 };
	const SEP:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 11 };
	const MTRR:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 12 };
	const PGE:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 13 };
	const MCA:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 14 };
	const CMOV:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 15 };
	const PAT:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 16 };
	const PSE36:	Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 17 };
	const PSN:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 18 };
	const CLF:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 19 };
	const DTES:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 21 };
	const ACPI:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 22 };
	const MMX:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 23 };
	const FXSR:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 24 };
	const SSE:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 25 };
	const SSE2:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 26 };
	const SS:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 27 };
	const HTT:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 28 };
	const TM1:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 29 };
	const IA64:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 30 };
	const PBE:		Feature = Feature { feature_type: FeatureType::PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 31 };

	const FSGSBASE:		Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 0 };
	const BMI1:			Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 3 };
	const HLE:			Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 4 };
	const AVX2:			Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 5 };
	const SMEP:			Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 7 };
	const BMI2:			Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 8 };
	const ERMS:			Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 9 };
	const INVPCID:		Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 10 };
	const RTM:			Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 11 };
	const MPX:			Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 14 };
	const AVX512F:		Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 16 };
	const AVX512DQ:		Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 17 };
	const RDSEED:		Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 18 };
	const ADX:			Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 19 };
	const SMAP:			Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 20 };
	const AVX512IFMA:	Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 21 };
	const PCOMMIT:		Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 22 };
	const CLFLUSHOPT:	Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 23 };
	const CLWB:			Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 24 };
	const INTEL_PT:		Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 25 };
	const AVX512PF:		Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 26 };
	const AVX512ER:		Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 27 };
	const AVX512CD:		Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 28 };
	const SHA:			Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 29 };
	const AVX512BW:		Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 30 };
	const AVX512VL:		Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::EBX, bitmask: 1 << 31 };

	const PREFETCHWT1:	Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::ECX, bitmask: 1 << 0 };
	const AVX512VBMI:	Feature = Feature { feature_type: FeatureType::EXTENDED_FEATURES, register: Register::ECX, bitmask: 1 << 1 };

	// The AMD Features from 0x80000001h are prefixed with AMD_ since some are duplicates.
	const AMD_FPU:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 0 };
	const AMD_VME:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 1 };
	const AMD_DE:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 2 };
	const AMD_PSE:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 3 };
	const AMD_TSC:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 4 };
	const AMD_MSR:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 5 };
	const AMD_PAE:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 6 };
	const AMD_MCE:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 7 };
	const AMD_CX8:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 8 };
	const AMD_APIC:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 9 };
	const AMD_SYSCALL:	Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 11 };
	const AMD_MTRR:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 12 };
	const AMD_PGE:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 13 };
	const AMD_MCA:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 14 };
	const AMD_CMOV:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 15 };
	const AMD_PAT:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 16 };
	const AMD_PSE36:	Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 17 };
	const AMD_MP:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 19 };
	const AMD_NX:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 20 };
	const AMD_MMXEXT:	Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 22 };
	const AMD_MMX:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 23 };
	const AMD_FXSR:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 24 };
	const AMD_FSXR_OPT:	Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 25 };
	const AMD_PDPE1GB:	Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 26 };
	const AMD_RDTSCP:	Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 27 };
	const AMD_LM:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 29 };
	const AMD_3DNOWEXT:	Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 30 };
	const AMD_3DNOW:	Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::EDX, bitmask: 1 << 31 };

	const AMD_LAHF_LM:			Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 0 };
	const AMD_CMP_LEGACY:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 1 };
	const AMD_SVM:				Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 2 };
	const AMD_EXTAPIC:			Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 3 };
	const AMD_CR8_LEGACY:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 4 };
	const AMD_ABM:				Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 5 };
	const AMD_SSE4A:			Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 6 };
	const AMD_MISALIGNSSE:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 7 };
	const AMD_3DNOWPREFETCH:	Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 8 };
	const AMD_OSVW:				Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 9 };
	const AMD_IBS:				Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 10 };
	const AMD_XOP:				Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 11 };
	const AMD_SKINIT:			Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 12 };
	const AMD_WDT:				Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 13 };
	const AMD_LWP:				Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 15 };
	const AMD_FMA4:				Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 13 };
	const AMD_TCE:				Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 17 };
	const AMD_NODEID_MSR:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 19 };
	const AMD_TBM:				Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 21 };
	const AMD_TOPOEXT:			Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 22 };
	const AMD_PERFCTR_CORE:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 23 };
	const AMD_PERFCTR_NB:		Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 24 };
	const AMD_DBX:				Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 26 };
	const AMD_PERFTSC:			Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 27 };
	const AMD_PCX_L2I:			Feature = Feature { feature_type: FeatureType::EXTENDED_PROCESSOR_INFO, register: Register::ECX, bitmask: 1 << 28 };
}


// CPUInfo contains the parsed Vendor String, Brand String (if available), and raw
// values for the registers returned by the other CPUID functions.  These registers
// can then be masked to query specific features.
struct CPUInfo {
	supported: bool,
	vendor_string: [u8; 12],
	processor_brand_string: [u8; 48],
	processor_info_eax: u32,
	processor_info_ecx: u32,
	processor_info_edx: u32,
	extended_features_ebx: u32,
	extended_features_ecx: u32,
	extended_processor_info_ecx: u32,
	extended_processor_info_edx: u32,
	l1_cache_eax: u32,
	l1_cache_ebx: u32,
	l1_cache_ecx: u32,
	l1_cache_edx: u32,
	l2_cache_eax: u32,
	l2_cache_ebx: u32,
	l2_cache_ecx: u32,
	l3_cache_edx: u32,
	apmi_edx: u32, // advanced power management information
	lmasi_eax: u32, // long mode address size identifiers
	apic_ecx: u32, // apic id size and core count
	svm_eax: u32, // svm revision
	svm_ebx: u32,
	svm_edx: u32,
	tlb_eax: u32, // tlb 1gb page identifiers
	tlb_ebx: u32,
	poid_eax: u32, // performance optimization identifiers
	ibsi_eax: u32, // instruction based sampling identifiers
	lpc0_eax: u32, // lightweight profiling capabilities 0
	lpc0_ebx: u32,
	lpc0_ecx: u32,
	lpc0_edx: u32,
	cache_ebx: u32, // cache properties
	cache_ecx: u32,
	cache_edx: u32,
	eapic_eax: u32,
	compute_ebx: u32, // compute unit identifiers
	node_eax: u32, // node Identifiers
}

static mut CPUINFO: CPUInfo = CPUInfo {
	supported: false,
	vendor_string: [0u8; 12],
	processor_brand_string: [0u8; 48],
	processor_info_eax: 0u32,
	processor_info_ecx: 0u32,
	processor_info_edx: 0u32,
	extended_features_ebx: 0u32,
	extended_features_ecx: 0u32,
	extended_processor_info_ecx: 0u32,
	extended_processor_info_edx: 0u32,
	l1_cache_eax: 0u32,
	l1_cache_ebx: 0u32,
	l1_cache_ecx: 0u32,
	l1_cache_edx: 0u32,
	l2_cache_eax: 0u32,
	l2_cache_ebx: 0u32,
	l2_cache_ecx: 0u32,
	l3_cache_edx: 0u32,
	apmi_edx: 0u32,
	lmasi_eax: 0u32,
	apic_ecx: 0u32,
	svm_eax: 0u32,
	svm_ebx: 0u32,
	svm_edx: 0u32,
	tlb_eax: 0u32,
	tlb_ebx: 0u32,
	poid_eax: 0u32,
	ibsi_eax: 0u32,
	lpc0_eax: 0u32,
	lpc0_ebx: 0u32,
	lpc0_ecx: 0u32,
	lpc0_edx: 0u32,
	cache_ebx: 0u32,
	cache_ecx: 0u32,
	cache_edx: 0u32,
	eapic_eax: 0u32,
	compute_ebx: 0u32,
	node_eax: 0u32,
};

pub unsafe fn setup() {
	if _get_cpuid_supported() {
		CPUINFO.supported = true;
	} else {
		return
	}

	// Highest CPUID Parameter and vendor string
	let (highest_parameter, ebx, ecx, edx) = _get_cpuid_result(0);

	let mut bytes = [0u8; 12];
	bytes[0] 	= ((ebx >> 24) & 0xFF) as u8;
	bytes[1] 	= ((ebx >> 16) & 0xFF) as u8;
	bytes[2] 	= ((ebx >> 8) & 0xFF) as u8;
	bytes[3] 	= (ebx & 0xFF) as u8;
	bytes[4] 	= ((edx >> 24) & 0xFF) as u8;
	bytes[5] 	= ((edx >> 16) & 0xFF) as u8;
	bytes[6] 	= ((edx >> 8) & 0xFF) as u8;
	bytes[7] 	= (edx & 0xFF) as u8;
	bytes[8] 	= ((ecx >> 24) & 0xFF) as u8;
	bytes[9] 	= ((ecx >> 16) & 0xFF) as u8;
	bytes[10] 	= ((ecx >> 8) & 0xFF) as u8;
	bytes[11] 	= (ecx & 0xFF) as u8;
	bytes.reverse();

	CPUINFO.vendor_string = bytes;

	if highest_parameter < 1 {
		return
	}
	let (eax, ebx, ecx, edx) = _get_cpuid_result(1);



}

fn _get_cpuid_supported() -> bool {
	let mut res: u32 = 0;
	unsafe {
		asm!("
			pushfl
			pushfl
			xorl $$0x00200000, (%esp)
			popfl
			pushfl
			popl %eax
			xorl (%esp), %eax
			popfl
			andl $$0x00200000, %eax"
			: "={eax}"(res)
			:
			: "esp"
			: "volatile"
		);
	}
	if res == 0 {
		false
	} else {
		true
	}
}

fn _get_cpuid_result(input: u32) -> (u32, u32, u32, u32) {
	let mut eax: u32 = 0;
	let mut ebx: u32 = 0;
	let mut ecx: u32 = 0;
	let mut edx: u32 = 0;
	unsafe {
		asm!("cpuid"
			: "={eax}"(eax), "={ebx}"(ebx), "={ecx}"(ecx), "={edx}"(edx)
			: "{eax}"(input)
			:
			: "volatile"
		);
	}
	(eax, ebx, ecx, edx)
}


/*
pub fn get_vendor() -> &'static str {
	unsafe {
		VENDOR_STRING
	}
}

fn _get_vendor() -> &'static str {
	let mut ebx = 0u32;
	let mut edx = 0u32;
	let mut ecx = 0u32;

	unsafe {
		asm!("
			movl $$0x0, %eax
			cpuid"
			: "={ebx}"(ebx), "={edx}"(edx), "={ecx}"(ecx)
			:
			: "eax"
			: "volatile"
		);
	}

	let bytes: &'static mut [u8; 12] = unsafe { mem::zeroed() };
	bytes[0] 	= ((ebx >> 24) & 0xFF) as u8;
	bytes[1] 	= ((ebx >> 16) & 0xFF) as u8;
	bytes[2] 	= ((ebx >> 8) & 0xFF) as u8;
	bytes[3] 	= (ebx & 0xFF) as u8;
	bytes[4] 	= ((edx >> 24) & 0xFF) as u8;
	bytes[5] 	= ((edx >> 16) & 0xFF) as u8;
	bytes[6] 	= ((edx >> 8) & 0xFF) as u8;
	bytes[7] 	= (edx & 0xFF) as u8;
	bytes[8] 	= ((ecx >> 24) & 0xFF) as u8;
	bytes[9] 	= ((ecx >> 16) & 0xFF) as u8;
	bytes[10] 	= ((ecx >> 8) & 0xFF) as u8;
	bytes[11] 	= (ecx & 0xFF) as u8;
	bytes.reverse();

	unsafe {
		from_utf8_unchecked(bytes)
	}
}

fn _get_features() -> (u32, u32) {
	let mut ecx = 0u32;
	let mut edx = 0u32;
	unsafe {
		asm!("
			movl $$0x1, %eax
			cpuid"
			: "={ecx}"(ecx), "={edx}"(edx)
			:
			: "eax"
			: "volatile"
		);
	}

	(ecx, edx)
}


// Usage:
// FeatureECX::SSE3.get_feature_supported()
pub trait FeatureEnum {
	fn get_feature_supported(self) -> bool;
}

impl FeatureEnum for FeatureECX {
	fn get_feature_supported(self) -> bool {
		unsafe {
			if FEATURES_ECX & (self as u32) > 0 {
				true
			} else {
				false
			}
		}
	}
}

impl FeatureEnum for FeatureEDX {
	fn get_feature_supported(self) -> bool {
		unsafe {
			if FEATURES_EDX & (self as u32) > 0 {
				true
			} else {
				false
			}
		}
	}
}
*/