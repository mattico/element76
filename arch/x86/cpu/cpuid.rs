use core::str::from_utf8_unchecked;
use core::slice::SliceExt;
use core::mem;

// These globals are populated in setup() so they can be queried quickly
// without having to run CPUID every time
static mut VENDOR_STRING: &'static str = VENDOR_INVALID;
static mut FEATURES_ECX: u32 = 0;
static mut FEATURES_EDX: u32 = 0;


const VENDOR_OLDAMD:		&'static str = "AMDisbetter!";
const VENDOR_AMD:			&'static str = "AuthenticAMD";
const VENDOR_INTEL:			&'static str = "GenuineIntel";
const VENDOR_VIA:			&'static str = "CentaurHauls";
const VENDOR_OLDTRANSMETA:	&'static str = "TransmetaCPU";
const VENDOR_TRANSMETA:		&'static str = "GenuineTMx86";
const VENDOR_CYRIX:			&'static str = "CyrixInstead";
const VENDOR_CENTAUR:		&'static str = "CentaurHauls";
const VENDOR_NEXGEN:		&'static str = "NexGenDriven";
const VENDOR_UMC:			&'static str = "UMC UMC UMC ";
const VENDOR_SIS:			&'static str = "SiS SiS SiS ";
const VENDOR_NSC:			&'static str = "Geode by NSC";
const VENDOR_RISE:			&'static str = "RiseRiseRise";
const VENDOR_INVALID:		&'static str = "INVALIDVEND!";

pub enum FeatureECX {
	SSE3		= 1 << 0,
	PCLMUL		= 1 << 1,
	DTES64		= 1 << 2,
	MONITOR		= 1 << 3,
	DS_CPL		= 1 << 4,
	VMX			= 1 << 5,
	SMX			= 1 << 6,
	EST			= 1 << 7,
	TM2			= 1 << 8,
	SSSE3		= 1 << 9,
	CID			= 1 << 10,
	FMA			= 1 << 12,
	CX16		= 1 << 13,
	ETPRD		= 1 << 14,
	PDCM		= 1 << 15,
	DCA			= 1 << 18,
	SSE4_1		= 1 << 19,
	SSE4_2		= 1 << 20,
	x2APIC		= 1 << 21,
	MOVBE		= 1 << 22,
	POPCNT		= 1 << 23,
	AES			= 1 << 25,
	XSAVE		= 1 << 26,
	OSXSAVE		= 1 << 27,
	AVX			= 1 << 28,
}

pub enum FeatureEDX {
	FPU		= 1 << 0,
	VME		= 1 << 1,
	DE		= 1 << 2,
	PSE		= 1 << 3,
	TSC		= 1 << 4,
	MSR		= 1 << 5,
	PAE		= 1 << 6,
	MCE		= 1 << 7,
	CX8		= 1 << 8,
	APIC	= 1 << 9,
	SEP		= 1 << 11,
	MTRR	= 1 << 12,
	PGE		= 1 << 13,
	MCA		= 1 << 14,
	CMOV	= 1 << 15,
	PAT		= 1 << 16,
	PSE36	= 1 << 17,
	PSN		= 1 << 18,
	CLF		= 1 << 19,
	DTES	= 1 << 21,
	ACPI	= 1 << 22,
	MMX		= 1 << 23,
	FXSR	= 1 << 24,
	SSE		= 1 << 25,
	SSE2	= 1 << 26,
	SS		= 1 << 27,
	HTT		= 1 << 28,
	TM1		= 1 << 29,
	IA64	= 1 << 30,
	PBE		= 1 << 31,
}

pub fn setup() {
	if get_cpuid_supported() {
		unsafe {
			VENDOR_STRING = _get_vendor();
			let (ecx, edx) = _get_features();
			FEATURES_ECX = ecx;
			FEATURES_EDX = edx;
		}
	}
}

pub fn get_cpuid_supported() -> bool {
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