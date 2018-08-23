// This is the compelte output of
//
//     bindgen ../RIOT/sys/include/net/gnrc.h --use-core --ctypes-prefix=libc -o gnrc.rs -- -I ../RIOT/sys/include -I ../RIOT/drivers/include -I ../RIOT/core/include -I.
//
// and the difficulty to split out the relevant parts leads me to believe that a more long-term
// viable version of riot-sys will have a fully auto-generated part, and then the checked parts
// (ie. those that are actually relevant and don't directly depend on customizable types) can be
// re-exported again for consumption by idiomatic wrappers.
//
// Note that this breaks badly when using Debug on a gnrc_netif_t, because now is the point where
// we'd really need to know all the involved C defines spat out by the RIOT build system.

use libc;

pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __STDC_NO_THREADS__: u32 = 1;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 27;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub const _INTTYPES_H: u32 = 1;
pub const ____gwchar_t_defined: u32 = 1;
pub const __PRI64_PREFIX: &'static [u8; 2usize] = b"l\0";
pub const __PRIPTR_PREFIX: &'static [u8; 2usize] = b"l\0";
pub const PRId8: &'static [u8; 2usize] = b"d\0";
pub const PRId16: &'static [u8; 2usize] = b"d\0";
pub const PRId32: &'static [u8; 2usize] = b"d\0";
pub const PRId64: &'static [u8; 3usize] = b"ld\0";
pub const PRIdLEAST8: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST16: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST32: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST64: &'static [u8; 3usize] = b"ld\0";
pub const PRIdFAST8: &'static [u8; 2usize] = b"d\0";
pub const PRIdFAST16: &'static [u8; 3usize] = b"ld\0";
pub const PRIdFAST32: &'static [u8; 3usize] = b"ld\0";
pub const PRIdFAST64: &'static [u8; 3usize] = b"ld\0";
pub const PRIi8: &'static [u8; 2usize] = b"i\0";
pub const PRIi16: &'static [u8; 2usize] = b"i\0";
pub const PRIi32: &'static [u8; 2usize] = b"i\0";
pub const PRIi64: &'static [u8; 3usize] = b"li\0";
pub const PRIiLEAST8: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST16: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST32: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST64: &'static [u8; 3usize] = b"li\0";
pub const PRIiFAST8: &'static [u8; 2usize] = b"i\0";
pub const PRIiFAST16: &'static [u8; 3usize] = b"li\0";
pub const PRIiFAST32: &'static [u8; 3usize] = b"li\0";
pub const PRIiFAST64: &'static [u8; 3usize] = b"li\0";
pub const PRIo8: &'static [u8; 2usize] = b"o\0";
pub const PRIo16: &'static [u8; 2usize] = b"o\0";
pub const PRIo32: &'static [u8; 2usize] = b"o\0";
pub const PRIo64: &'static [u8; 3usize] = b"lo\0";
pub const PRIoLEAST8: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST16: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST32: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST64: &'static [u8; 3usize] = b"lo\0";
pub const PRIoFAST8: &'static [u8; 2usize] = b"o\0";
pub const PRIoFAST16: &'static [u8; 3usize] = b"lo\0";
pub const PRIoFAST32: &'static [u8; 3usize] = b"lo\0";
pub const PRIoFAST64: &'static [u8; 3usize] = b"lo\0";
pub const PRIu8: &'static [u8; 2usize] = b"u\0";
pub const PRIu16: &'static [u8; 2usize] = b"u\0";
pub const PRIu32: &'static [u8; 2usize] = b"u\0";
pub const PRIu64: &'static [u8; 3usize] = b"lu\0";
pub const PRIuLEAST8: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST16: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST32: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST64: &'static [u8; 3usize] = b"lu\0";
pub const PRIuFAST8: &'static [u8; 2usize] = b"u\0";
pub const PRIuFAST16: &'static [u8; 3usize] = b"lu\0";
pub const PRIuFAST32: &'static [u8; 3usize] = b"lu\0";
pub const PRIuFAST64: &'static [u8; 3usize] = b"lu\0";
pub const PRIx8: &'static [u8; 2usize] = b"x\0";
pub const PRIx16: &'static [u8; 2usize] = b"x\0";
pub const PRIx32: &'static [u8; 2usize] = b"x\0";
pub const PRIx64: &'static [u8; 3usize] = b"lx\0";
pub const PRIxLEAST8: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST16: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST32: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST64: &'static [u8; 3usize] = b"lx\0";
pub const PRIxFAST8: &'static [u8; 2usize] = b"x\0";
pub const PRIxFAST16: &'static [u8; 3usize] = b"lx\0";
pub const PRIxFAST32: &'static [u8; 3usize] = b"lx\0";
pub const PRIxFAST64: &'static [u8; 3usize] = b"lx\0";
pub const PRIX8: &'static [u8; 2usize] = b"X\0";
pub const PRIX16: &'static [u8; 2usize] = b"X\0";
pub const PRIX32: &'static [u8; 2usize] = b"X\0";
pub const PRIX64: &'static [u8; 3usize] = b"lX\0";
pub const PRIXLEAST8: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST16: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST32: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST64: &'static [u8; 3usize] = b"lX\0";
pub const PRIXFAST8: &'static [u8; 2usize] = b"X\0";
pub const PRIXFAST16: &'static [u8; 3usize] = b"lX\0";
pub const PRIXFAST32: &'static [u8; 3usize] = b"lX\0";
pub const PRIXFAST64: &'static [u8; 3usize] = b"lX\0";
pub const PRIdMAX: &'static [u8; 3usize] = b"ld\0";
pub const PRIiMAX: &'static [u8; 3usize] = b"li\0";
pub const PRIoMAX: &'static [u8; 3usize] = b"lo\0";
pub const PRIuMAX: &'static [u8; 3usize] = b"lu\0";
pub const PRIxMAX: &'static [u8; 3usize] = b"lx\0";
pub const PRIXMAX: &'static [u8; 3usize] = b"lX\0";
pub const PRIdPTR: &'static [u8; 3usize] = b"ld\0";
pub const PRIiPTR: &'static [u8; 3usize] = b"li\0";
pub const PRIoPTR: &'static [u8; 3usize] = b"lo\0";
pub const PRIuPTR: &'static [u8; 3usize] = b"lu\0";
pub const PRIxPTR: &'static [u8; 3usize] = b"lx\0";
pub const PRIXPTR: &'static [u8; 3usize] = b"lX\0";
pub const SCNd8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNd16: &'static [u8; 3usize] = b"hd\0";
pub const SCNd32: &'static [u8; 2usize] = b"d\0";
pub const SCNd64: &'static [u8; 3usize] = b"ld\0";
pub const SCNdLEAST8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNdLEAST16: &'static [u8; 3usize] = b"hd\0";
pub const SCNdLEAST32: &'static [u8; 2usize] = b"d\0";
pub const SCNdLEAST64: &'static [u8; 3usize] = b"ld\0";
pub const SCNdFAST8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNdFAST16: &'static [u8; 3usize] = b"ld\0";
pub const SCNdFAST32: &'static [u8; 3usize] = b"ld\0";
pub const SCNdFAST64: &'static [u8; 3usize] = b"ld\0";
pub const SCNi8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNi16: &'static [u8; 3usize] = b"hi\0";
pub const SCNi32: &'static [u8; 2usize] = b"i\0";
pub const SCNi64: &'static [u8; 3usize] = b"li\0";
pub const SCNiLEAST8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNiLEAST16: &'static [u8; 3usize] = b"hi\0";
pub const SCNiLEAST32: &'static [u8; 2usize] = b"i\0";
pub const SCNiLEAST64: &'static [u8; 3usize] = b"li\0";
pub const SCNiFAST8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNiFAST16: &'static [u8; 3usize] = b"li\0";
pub const SCNiFAST32: &'static [u8; 3usize] = b"li\0";
pub const SCNiFAST64: &'static [u8; 3usize] = b"li\0";
pub const SCNu8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNu16: &'static [u8; 3usize] = b"hu\0";
pub const SCNu32: &'static [u8; 2usize] = b"u\0";
pub const SCNu64: &'static [u8; 3usize] = b"lu\0";
pub const SCNuLEAST8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNuLEAST16: &'static [u8; 3usize] = b"hu\0";
pub const SCNuLEAST32: &'static [u8; 2usize] = b"u\0";
pub const SCNuLEAST64: &'static [u8; 3usize] = b"lu\0";
pub const SCNuFAST8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNuFAST16: &'static [u8; 3usize] = b"lu\0";
pub const SCNuFAST32: &'static [u8; 3usize] = b"lu\0";
pub const SCNuFAST64: &'static [u8; 3usize] = b"lu\0";
pub const SCNo8: &'static [u8; 4usize] = b"hho\0";
pub const SCNo16: &'static [u8; 3usize] = b"ho\0";
pub const SCNo32: &'static [u8; 2usize] = b"o\0";
pub const SCNo64: &'static [u8; 3usize] = b"lo\0";
pub const SCNoLEAST8: &'static [u8; 4usize] = b"hho\0";
pub const SCNoLEAST16: &'static [u8; 3usize] = b"ho\0";
pub const SCNoLEAST32: &'static [u8; 2usize] = b"o\0";
pub const SCNoLEAST64: &'static [u8; 3usize] = b"lo\0";
pub const SCNoFAST8: &'static [u8; 4usize] = b"hho\0";
pub const SCNoFAST16: &'static [u8; 3usize] = b"lo\0";
pub const SCNoFAST32: &'static [u8; 3usize] = b"lo\0";
pub const SCNoFAST64: &'static [u8; 3usize] = b"lo\0";
pub const SCNx8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNx16: &'static [u8; 3usize] = b"hx\0";
pub const SCNx32: &'static [u8; 2usize] = b"x\0";
pub const SCNx64: &'static [u8; 3usize] = b"lx\0";
pub const SCNxLEAST8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNxLEAST16: &'static [u8; 3usize] = b"hx\0";
pub const SCNxLEAST32: &'static [u8; 2usize] = b"x\0";
pub const SCNxLEAST64: &'static [u8; 3usize] = b"lx\0";
pub const SCNxFAST8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNxFAST16: &'static [u8; 3usize] = b"lx\0";
pub const SCNxFAST32: &'static [u8; 3usize] = b"lx\0";
pub const SCNxFAST64: &'static [u8; 3usize] = b"lx\0";
pub const SCNdMAX: &'static [u8; 3usize] = b"ld\0";
pub const SCNiMAX: &'static [u8; 3usize] = b"li\0";
pub const SCNoMAX: &'static [u8; 3usize] = b"lo\0";
pub const SCNuMAX: &'static [u8; 3usize] = b"lu\0";
pub const SCNxMAX: &'static [u8; 3usize] = b"lx\0";
pub const SCNdPTR: &'static [u8; 3usize] = b"ld\0";
pub const SCNiPTR: &'static [u8; 3usize] = b"li\0";
pub const SCNoPTR: &'static [u8; 3usize] = b"lo\0";
pub const SCNuPTR: &'static [u8; 3usize] = b"lu\0";
pub const SCNxPTR: &'static [u8; 3usize] = b"lx\0";
pub const _LIBC_LIMITS_H_: u32 = 1;
pub const MB_LEN_MAX: u32 = 16;
pub const _BITS_POSIX1_LIM_H: u32 = 1;
pub const _POSIX_AIO_LISTIO_MAX: u32 = 2;
pub const _POSIX_AIO_MAX: u32 = 1;
pub const _POSIX_ARG_MAX: u32 = 4096;
pub const _POSIX_CHILD_MAX: u32 = 25;
pub const _POSIX_DELAYTIMER_MAX: u32 = 32;
pub const _POSIX_HOST_NAME_MAX: u32 = 255;
pub const _POSIX_LINK_MAX: u32 = 8;
pub const _POSIX_LOGIN_NAME_MAX: u32 = 9;
pub const _POSIX_MAX_CANON: u32 = 255;
pub const _POSIX_MAX_INPUT: u32 = 255;
pub const _POSIX_MQ_OPEN_MAX: u32 = 8;
pub const _POSIX_MQ_PRIO_MAX: u32 = 32;
pub const _POSIX_NAME_MAX: u32 = 14;
pub const _POSIX_NGROUPS_MAX: u32 = 8;
pub const _POSIX_OPEN_MAX: u32 = 20;
pub const _POSIX_PATH_MAX: u32 = 256;
pub const _POSIX_PIPE_BUF: u32 = 512;
pub const _POSIX_RE_DUP_MAX: u32 = 255;
pub const _POSIX_RTSIG_MAX: u32 = 8;
pub const _POSIX_SEM_NSEMS_MAX: u32 = 256;
pub const _POSIX_SEM_VALUE_MAX: u32 = 32767;
pub const _POSIX_SIGQUEUE_MAX: u32 = 32;
pub const _POSIX_SSIZE_MAX: u32 = 32767;
pub const _POSIX_STREAM_MAX: u32 = 8;
pub const _POSIX_SYMLINK_MAX: u32 = 255;
pub const _POSIX_SYMLOOP_MAX: u32 = 8;
pub const _POSIX_TIMER_MAX: u32 = 32;
pub const _POSIX_TTY_NAME_MAX: u32 = 9;
pub const _POSIX_TZNAME_MAX: u32 = 6;
pub const _POSIX_CLOCKRES_MIN: u32 = 20000000;
pub const NR_OPEN: u32 = 1024;
pub const NGROUPS_MAX: u32 = 65536;
pub const ARG_MAX: u32 = 131072;
pub const LINK_MAX: u32 = 127;
pub const MAX_CANON: u32 = 255;
pub const MAX_INPUT: u32 = 255;
pub const NAME_MAX: u32 = 255;
pub const PATH_MAX: u32 = 4096;
pub const PIPE_BUF: u32 = 4096;
pub const XATTR_NAME_MAX: u32 = 255;
pub const XATTR_SIZE_MAX: u32 = 65536;
pub const XATTR_LIST_MAX: u32 = 65536;
pub const RTSIG_MAX: u32 = 32;
pub const _POSIX_THREAD_KEYS_MAX: u32 = 128;
pub const PTHREAD_KEYS_MAX: u32 = 1024;
pub const _POSIX_THREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const PTHREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const _POSIX_THREAD_THREADS_MAX: u32 = 64;
pub const AIO_PRIO_DELTA_MAX: u32 = 20;
pub const PTHREAD_STACK_MIN: u32 = 16384;
pub const DELAYTIMER_MAX: u32 = 2147483647;
pub const TTY_NAME_MAX: u32 = 32;
pub const LOGIN_NAME_MAX: u32 = 256;
pub const HOST_NAME_MAX: u32 = 64;
pub const MQ_PRIO_MAX: u32 = 32768;
pub const SEM_VALUE_MAX: u32 = 2147483647;
pub const _BITS_POSIX2_LIM_H: u32 = 1;
pub const _POSIX2_BC_BASE_MAX: u32 = 99;
pub const _POSIX2_BC_DIM_MAX: u32 = 2048;
pub const _POSIX2_BC_SCALE_MAX: u32 = 99;
pub const _POSIX2_BC_STRING_MAX: u32 = 1000;
pub const _POSIX2_COLL_WEIGHTS_MAX: u32 = 2;
pub const _POSIX2_EXPR_NEST_MAX: u32 = 32;
pub const _POSIX2_LINE_MAX: u32 = 2048;
pub const _POSIX2_RE_DUP_MAX: u32 = 255;
pub const _POSIX2_CHARCLASS_NAME_MAX: u32 = 14;
pub const BC_BASE_MAX: u32 = 99;
pub const BC_DIM_MAX: u32 = 2048;
pub const BC_SCALE_MAX: u32 = 99;
pub const BC_STRING_MAX: u32 = 1000;
pub const COLL_WEIGHTS_MAX: u32 = 255;
pub const EXPR_NEST_MAX: u32 = 32;
pub const LINE_MAX: u32 = 2048;
pub const CHARCLASS_NAME_MAX: u32 = 2048;
pub const RE_DUP_MAX: u32 = 32767;
pub const _SYS_TYPES_H: u32 = 1;
pub const __clock_t_defined: u32 = 1;
pub const __clockid_t_defined: u32 = 1;
pub const __time_t_defined: u32 = 1;
pub const __timer_t_defined: u32 = 1;
pub const __BIT_TYPES_DEFINED__: u32 = 1;
pub const _ENDIAN_H: u32 = 1;
pub const __LITTLE_ENDIAN: u32 = 1234;
pub const __BIG_ENDIAN: u32 = 4321;
pub const __PDP_ENDIAN: u32 = 3412;
pub const __BYTE_ORDER: u32 = 1234;
pub const __FLOAT_WORD_ORDER: u32 = 1234;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const BYTE_ORDER: u32 = 1234;
pub const _BITS_BYTESWAP_H: u32 = 1;
pub const _BITS_UINTN_IDENTITY_H: u32 = 1;
pub const _SYS_SELECT_H: u32 = 1;
pub const __FD_ZERO_STOS: &'static [u8; 6usize] = b"stosq\0";
pub const __sigset_t_defined: u32 = 1;
pub const __timeval_defined: u32 = 1;
pub const _STRUCT_TIMESPEC: u32 = 1;
pub const FD_SETSIZE: u32 = 1024;
pub const _SYS_SYSMACROS_H: u32 = 1;
pub const _BITS_SYSMACROS_H: u32 = 1;
pub const _BITS_PTHREADTYPES_COMMON_H: u32 = 1;
pub const _THREAD_SHARED_TYPES_H: u32 = 1;
pub const _BITS_PTHREADTYPES_ARCH_H: u32 = 1;
pub const __SIZEOF_PTHREAD_MUTEX_T: u32 = 40;
pub const __SIZEOF_PTHREAD_ATTR_T: u32 = 56;
pub const __SIZEOF_PTHREAD_RWLOCK_T: u32 = 56;
pub const __SIZEOF_PTHREAD_BARRIER_T: u32 = 32;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_COND_T: u32 = 48;
pub const __SIZEOF_PTHREAD_CONDATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: u32 = 8;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: u32 = 4;
pub const __PTHREAD_MUTEX_LOCK_ELISION: u32 = 1;
pub const __PTHREAD_MUTEX_NUSERS_AFTER_KIND: u32 = 0;
pub const __PTHREAD_MUTEX_USE_UNION: u32 = 0;
pub const __PTHREAD_RWLOCK_INT_FLAGS_SHARED: u32 = 1;
pub const __PTHREAD_MUTEX_HAVE_PREV: u32 = 1;
pub const __have_pthread_attr_t: u32 = 1;
pub const MAXTHREADS: u32 = 32;
pub const KERNEL_PID_UNDEF: u32 = 0;
pub const KERNEL_PID_FIRST: u32 = 1;
pub const KERNEL_PID_LAST: u32 = 32;
pub const PRIkernel_pid: &'static [u8; 2usize] = b"i\0";
pub const KERNEL_PID_ISR: u32 = 33;
pub const BIT0: u32 = 1;
pub const BIT1: u32 = 2;
pub const BIT2: u32 = 4;
pub const BIT3: u32 = 8;
pub const BIT4: u32 = 16;
pub const BIT5: u32 = 32;
pub const BIT6: u32 = 64;
pub const BIT7: u32 = 128;
pub const BIT8: u32 = 256;
pub const BIT9: u32 = 512;
pub const BIT10: u32 = 1024;
pub const BIT11: u32 = 2048;
pub const BIT12: u32 = 4096;
pub const BIT13: u32 = 8192;
pub const BIT14: u32 = 16384;
pub const BIT15: u32 = 32768;
pub const BIT16: u32 = 65536;
pub const BIT17: u32 = 131072;
pub const BIT18: u32 = 262144;
pub const BIT19: u32 = 524288;
pub const BIT20: u32 = 1048576;
pub const BIT21: u32 = 2097152;
pub const BIT22: u32 = 4194304;
pub const BIT23: u32 = 8388608;
pub const BIT24: u32 = 16777216;
pub const BIT25: u32 = 33554432;
pub const BIT26: u32 = 67108864;
pub const BIT27: u32 = 134217728;
pub const BIT28: u32 = 268435456;
pub const BIT29: u32 = 536870912;
pub const BIT30: u32 = 1073741824;
pub const BIT31: u32 = 2147483648;
pub const SCHED_PRIO_LEVELS: u32 = 16;
pub const STATUS_NOT_FOUND: i32 = -1;
pub const STATUS_STOPPED: u32 = 0;
pub const STATUS_SLEEPING: u32 = 1;
pub const STATUS_MUTEX_BLOCKED: u32 = 2;
pub const STATUS_RECEIVE_BLOCKED: u32 = 3;
pub const STATUS_SEND_BLOCKED: u32 = 4;
pub const STATUS_REPLY_BLOCKED: u32 = 5;
pub const STATUS_FLAG_BLOCKED_ANY: u32 = 6;
pub const STATUS_FLAG_BLOCKED_ALL: u32 = 7;
pub const STATUS_MBOX_BLOCKED: u32 = 8;
pub const STATUS_RUNNING: u32 = 9;
pub const STATUS_PENDING: u32 = 10;
pub const THREAD_PRIORITY_MIN: u32 = 15;
pub const THREAD_PRIORITY_IDLE: u32 = 15;
pub const THREAD_PRIORITY_MAIN: u32 = 7;
pub const THREAD_CREATE_SLEEPING: u32 = 1;
pub const THREAD_AUTO_FREE: u32 = 2;
pub const THREAD_CREATE_WOUT_YIELD: u32 = 4;
pub const THREAD_CREATE_STACKTEST: u32 = 8;
pub const ETHERTYPE_RESERVED: u32 = 0;
pub const ETHERTYPE_IPV4: u32 = 2048;
pub const ETHERTYPE_ARP: u32 = 2054;
pub const ETHERTYPE_CCNX: u32 = 2049;
pub const ETHERTYPE_NDN: u32 = 34340;
pub const ETHERTYPE_IPV6: u32 = 34525;
pub const ETHERTYPE_UNKNOWN: u32 = 65535;
pub const PROTNUM_IPV6_EXT_HOPOPT: u32 = 0;
pub const PROTNUM_ICMP: u32 = 1;
pub const PROTNUM_IGMP: u32 = 2;
pub const PROTNUM_GGP: u32 = 3;
pub const PROTNUM_IPV4: u32 = 4;
pub const PROTNUM_ST: u32 = 5;
pub const PROTNUM_TCP: u32 = 6;
pub const PROTNUM_CBT: u32 = 7;
pub const PROTNUM_EGP: u32 = 8;
pub const PROTNUM_IGP: u32 = 9;
pub const PROTNUM_BBN_RCC_MON: u32 = 10;
pub const PROTNUM_NVP_II: u32 = 11;
pub const PROTNUM_PUP: u32 = 12;
pub const PROTNUM_ARGUS: u32 = 13;
pub const PROTNUM_EMCON: u32 = 14;
pub const PROTNUM_XNET: u32 = 15;
pub const PROTNUM_CHAOS: u32 = 16;
pub const PROTNUM_UDP: u32 = 17;
pub const PROTNUM_MUX: u32 = 18;
pub const PROTNUM_DCN_MEAS: u32 = 19;
pub const PROTNUM_HMP: u32 = 20;
pub const PROTNUM_PRM: u32 = 21;
pub const PROTNUM_XNS_IDP: u32 = 22;
pub const PROTNUM_TRUNK_1: u32 = 23;
pub const PROTNUM_TRUNK_2: u32 = 24;
pub const PROTNUM_LEAF_1: u32 = 25;
pub const PROTNUM_LEAF_2: u32 = 26;
pub const PROTNUM_RDP: u32 = 27;
pub const PROTNUM_IRTP: u32 = 28;
pub const PROTNUM_ISO_TP4: u32 = 29;
pub const PROTNUM_NETBLT: u32 = 30;
pub const PROTNUM_MFE_NSP: u32 = 31;
pub const PROTNUM_MERIT_INP: u32 = 32;
pub const PROTNUM_DCCP: u32 = 33;
pub const PROTNUM_3PC: u32 = 34;
pub const PROTNUM_IDPR: u32 = 35;
pub const PROTNUM_XTP: u32 = 36;
pub const PROTNUM_DDP: u32 = 37;
pub const PROTNUM_IDPR_CMTP: u32 = 38;
pub const PROTNUM_TPPLUSPLUS: u32 = 39;
pub const PROTNUM_IL: u32 = 40;
pub const PROTNUM_IPV6: u32 = 41;
pub const PROTNUM_SDRP: u32 = 42;
pub const PROTNUM_IPV6_EXT_RH: u32 = 43;
pub const PROTNUM_IPV6_EXT_FRAG: u32 = 44;
pub const PROTNUM_IDRP: u32 = 45;
pub const PROTNUM_RSVP: u32 = 46;
pub const PROTNUM_GRE: u32 = 47;
pub const PROTNUM_DSR: u32 = 48;
pub const PROTNUM_BNA: u32 = 49;
pub const PROTNUM_IPV6_EXT_ESP: u32 = 50;
pub const PROTNUM_IPV6_EXT_AH: u32 = 51;
pub const PROTNUM_I_NLSP: u32 = 52;
pub const PROTNUM_SWIPE: u32 = 53;
pub const PROTNUM_NARP: u32 = 54;
pub const PROTNUM_MOBILE: u32 = 55;
pub const PROTNUM_TLSP: u32 = 56;
pub const PROTNUM_SKIP: u32 = 57;
pub const PROTNUM_ICMPV6: u32 = 58;
pub const PROTNUM_IPV6_NONXT: u32 = 59;
pub const PROTNUM_IPV6_EXT_DST: u32 = 60;
pub const PROTNUM_CFTP: u32 = 62;
pub const PROTNUM_SAT_EXPAK: u32 = 64;
pub const PROTNUM_KRYPTOLAN: u32 = 65;
pub const PROTNUM_RVD: u32 = 66;
pub const PROTNUM_IPPC: u32 = 67;
pub const PROTNUM_SAT_MON: u32 = 69;
pub const PROTNUM_VISA: u32 = 70;
pub const PROTNUM_IPCV: u32 = 71;
pub const PROTNUM_CPNX: u32 = 72;
pub const PROTNUM_CPHB: u32 = 73;
pub const PROTNUM_WSN: u32 = 74;
pub const PROTNUM_PVP: u32 = 75;
pub const PROTNUM_BR_SAT_MON: u32 = 76;
pub const PROTNUM_SUN_ND: u32 = 77;
pub const PROTNUM_WB_MON: u32 = 78;
pub const PROTNUM_WB_EXPAK: u32 = 79;
pub const PROTNUM_ISO_IP: u32 = 80;
pub const PROTNUM_VMTP: u32 = 81;
pub const PROTNUM_SECURE_VMTP: u32 = 82;
pub const PROTNUM_VINES: u32 = 83;
pub const PROTNUM_TTP: u32 = 84;
pub const PROTNUM_IPTM: u32 = 84;
pub const PROTNUM_NSFNET_IGP: u32 = 85;
pub const PROTNUM_DGP: u32 = 86;
pub const PROTNUM_TCF: u32 = 87;
pub const PROTNUM_EIGRP: u32 = 88;
pub const PROTNUM_OSPFIGP: u32 = 89;
pub const PROTNUM_SPRITE_RPC: u32 = 90;
pub const PROTNUM_LARP: u32 = 91;
pub const PROTNUM_MTP: u32 = 92;
pub const PROTNUM_AX_25: u32 = 93;
pub const PROTNUM_IPIP: u32 = 94;
pub const PROTNUM_MICP: u32 = 95;
pub const PROTNUM_SCC_SP: u32 = 96;
pub const PROTNUM_ETHERIP: u32 = 97;
pub const PROTNUM_ENCAP: u32 = 98;
pub const PROTNUM_GMTP: u32 = 100;
pub const PROTNUM_IFMP: u32 = 101;
pub const PROTNUM_PNNI: u32 = 102;
pub const PROTNUM_PIM: u32 = 103;
pub const PROTNUM_ARIS: u32 = 104;
pub const PROTNUM_SCPS: u32 = 105;
pub const PROTNUM_QNX: u32 = 106;
pub const PROTNUM_A_N: u32 = 107;
pub const PROTNUM_IPCOMP: u32 = 108;
pub const PROTNUM_SNP: u32 = 109;
pub const PROTNUM_COMPAQ_PEER: u32 = 110;
pub const PROTNUM_IPX_IN_IP: u32 = 111;
pub const PROTNUM_VRRP: u32 = 112;
pub const PROTNUM_PGM: u32 = 113;
pub const PROTNUM_L2TP: u32 = 115;
pub const PROTNUM_DDX: u32 = 116;
pub const PROTNUM_IATP: u32 = 117;
pub const PROTNUM_STP: u32 = 118;
pub const PROTNUM_SRP: u32 = 119;
pub const PROTNUM_UTI: u32 = 120;
pub const PROTNUM_SMP: u32 = 121;
pub const PROTNUM_SM: u32 = 122;
pub const PROTNUM_PTP: u32 = 123;
pub const PROTNUM_ISIS_OVER_IPV4: u32 = 124;
pub const PROTNUM_FIRE: u32 = 125;
pub const PROTNUM_CRTP: u32 = 126;
pub const PROTNUM_CRUDP: u32 = 127;
pub const PROTNUM_SSCOPMCE: u32 = 128;
pub const PROTNUM_IPLT: u32 = 129;
pub const PROTNUM_SPS: u32 = 130;
pub const PROTNUM_PIPE: u32 = 131;
pub const PROTNUM_SCTP: u32 = 132;
pub const PROTNUM_FC: u32 = 133;
pub const PROTNUM_RSVP_E2E_IGNORE: u32 = 134;
pub const PROTNUM_IPV6_EXT_MOB: u32 = 135;
pub const PROTNUM_UDPLITE: u32 = 136;
pub const PROTNUM_MPLS_IN_IP: u32 = 137;
pub const PROTNUM_MANET: u32 = 138;
pub const PROTNUM_HIP: u32 = 139;
pub const PROTNUM_SHIM6: u32 = 140;
pub const PROTNUM_WESP: u32 = 141;
pub const PROTNUM_ROHC: u32 = 142;
pub const PROTNUM_RESERVED: u32 = 255;
pub const _STDLIB_H: u32 = 1;
pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;
pub const WSTOPPED: u32 = 2;
pub const WEXITED: u32 = 4;
pub const WCONTINUED: u32 = 8;
pub const WNOWAIT: u32 = 16777216;
pub const __WNOTHREAD: u32 = 536870912;
pub const __WALL: u32 = 1073741824;
pub const __WCLONE: u32 = 2147483648;
pub const __ENUM_IDTYPE_T: u32 = 1;
pub const __W_CONTINUED: u32 = 65535;
pub const __WCOREFLAG: u32 = 128;
pub const __HAVE_FLOAT128: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128: u32 = 0;
pub const __HAVE_FLOAT64X: u32 = 1;
pub const __HAVE_FLOAT64X_LONG_DOUBLE: u32 = 1;
pub const __HAVE_FLOAT16: u32 = 0;
pub const __HAVE_FLOAT32: u32 = 1;
pub const __HAVE_FLOAT64: u32 = 1;
pub const __HAVE_FLOAT32X: u32 = 1;
pub const __HAVE_FLOAT128X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT16: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128X: u32 = 0;
pub const __HAVE_FLOATN_NOT_TYPEDEF: u32 = 0;
pub const __ldiv_t_defined: u32 = 1;
pub const __lldiv_t_defined: u32 = 1;
pub const RAND_MAX: u32 = 2147483647;
pub const EXIT_FAILURE: u32 = 1;
pub const EXIT_SUCCESS: u32 = 0;
pub const _ALLOCA_H: u32 = 1;
pub const GNRC_NETAPI_MSG_TYPE_RCV: u32 = 513;
pub const GNRC_NETAPI_MSG_TYPE_SND: u32 = 514;
pub const GNRC_NETAPI_MSG_TYPE_SET: u32 = 515;
pub const GNRC_NETAPI_MSG_TYPE_GET: u32 = 516;
pub const GNRC_NETAPI_MSG_TYPE_ACK: u32 = 517;
pub const GNRC_NETREG_DEMUX_CTX_ALL: u32 = 4294901760;
pub const _STRING_H: u32 = 1;
pub const _BITS_TYPES_LOCALE_T_H: u32 = 1;
pub const _BITS_TYPES___LOCALE_T_H: u32 = 1;
pub const _STRINGS_H: u32 = 1;
pub const IPV6_ADDR_BIT_LEN: u32 = 128;
pub const IPV6_ADDR_SITE_LOCAL_PREFIX: u32 = 65216;
pub const IPV6_ADDR_MCAST_FLAG_TRANSIENT: u32 = 1;
pub const IPV6_ADDR_MCAST_FLAG_PREFIX_BASED: u32 = 2;
pub const IPV6_ADDR_MCAST_FLAG_EMBED_ON_RP: u32 = 4;
pub const IPV6_ADDR_MCAST_SCP_IF_LOCAL: u32 = 1;
pub const IPV6_ADDR_MCAST_SCP_LINK_LOCAL: u32 = 2;
pub const IPV6_ADDR_MCAST_SCP_REALM_LOCAL: u32 = 3;
pub const IPV6_ADDR_MCAST_SCP_ADMIN_LOCAL: u32 = 4;
pub const IPV6_ADDR_MCAST_SCP_SITE_LOCAL: u32 = 5;
pub const IPV6_ADDR_MCAST_SCP_ORG_LOCAL: u32 = 8;
pub const IPV6_ADDR_MCAST_SCP_GLOBAL: u32 = 14;
pub const IEEE802154_SHORT_ADDRESS_LEN: u32 = 2;
pub const IEEE802154_LONG_ADDRESS_LEN: u32 = 8;
pub const IEEE802154_MAX_HDR_LEN: u32 = 23;
pub const IEEE802154_FCF_LEN: u32 = 2;
pub const IEEE802154_FCS_LEN: u32 = 2;
pub const IEEE802154_FCF_TYPE_MASK: u32 = 7;
pub const IEEE802154_FCF_TYPE_BEACON: u32 = 0;
pub const IEEE802154_FCF_TYPE_DATA: u32 = 1;
pub const IEEE802154_FCF_TYPE_ACK: u32 = 2;
pub const IEEE802154_FCF_TYPE_MACCMD: u32 = 3;
pub const IEEE802154_FCF_SECURITY_EN: u32 = 8;
pub const IEEE802154_FCF_FRAME_PEND: u32 = 16;
pub const IEEE802154_FCF_ACK_REQ: u32 = 32;
pub const IEEE802154_FCF_PAN_COMP: u32 = 64;
pub const IEEE802154_FCF_DST_ADDR_MASK: u32 = 12;
pub const IEEE802154_FCF_DST_ADDR_VOID: u32 = 0;
pub const IEEE802154_FCF_DST_ADDR_RESV: u32 = 4;
pub const IEEE802154_FCF_DST_ADDR_SHORT: u32 = 8;
pub const IEEE802154_FCF_DST_ADDR_LONG: u32 = 12;
pub const IEEE802154_FCF_VERS_MASK: u32 = 48;
pub const IEEE802154_FCF_VERS_V0: u32 = 0;
pub const IEEE802154_FCF_VERS_V1: u32 = 16;
pub const IEEE802154_FCF_SRC_ADDR_MASK: u32 = 192;
pub const IEEE802154_FCF_SRC_ADDR_VOID: u32 = 0;
pub const IEEE802154_FCF_SRC_ADDR_RESV: u32 = 64;
pub const IEEE802154_FCF_SRC_ADDR_SHORT: u32 = 128;
pub const IEEE802154_FCF_SRC_ADDR_LONG: u32 = 192;
pub const IEEE802154_CHANNEL_MIN_SUBGHZ: u32 = 0;
pub const IEEE802154_CHANNEL_MAX_SUBGHZ: u32 = 10;
pub const IEEE802154_CHANNEL_MIN: u32 = 11;
pub const IEEE802154_CHANNEL_MAX: u32 = 26;
pub const IEEE802154_FRAME_LEN_MAX: u32 = 127;
pub const IEEE802154_ADDR_BCAST_LEN: u32 = 2;
pub const IEEE802154_DEFAULT_SUBGHZ_CHANNEL: u32 = 5;
pub const IEEE802154_DEFAULT_CHANNEL: u32 = 26;
pub const IEEE802154_DEFAULT_SUBGHZ_PAGE: u32 = 2;
pub const IEEE802154_DEFAULT_PANID: u32 = 35;
pub const IEEE802154_DEFAULT_TXPOWER: u32 = 0;
pub const ETHERNET_ADDR_LEN: u32 = 6;
pub const ETH_ALEN: u32 = 6;
pub const GNRC_IPV6_NIB_CONF_6LBR: u32 = 0;
pub const GNRC_IPV6_NIB_CONF_6LR: u32 = 0;
pub const GNRC_IPV6_NIB_CONF_6LN: u32 = 0;
pub const GNRC_IPV6_NIB_CONF_ROUTER: u32 = 0;
pub const GNRC_IPV6_NIB_CONF_ADV_ROUTER: u32 = 0;
pub const GNRC_IPV6_NIB_CONF_ARSM: u32 = 1;
pub const GNRC_IPV6_NIB_CONF_QUEUE_PKT: u32 = 1;
pub const GNRC_IPV6_NIB_CONF_SLAAC: u32 = 1;
pub const GNRC_IPV6_NIB_CONF_REDIRECT: u32 = 0;
pub const GNRC_IPV6_NIB_CONF_DC: u32 = 0;
pub const GNRC_IPV6_NIB_CONF_DNS: u32 = 0;
pub const GNRC_IPV6_NIB_CONF_MULTIHOP_P6C: u32 = 0;
pub const GNRC_IPV6_NIB_CONF_MULTIHOP_DAD: u32 = 0;
pub const GNRC_IPV6_NIB_CONF_REACH_TIME_RESET: u32 = 7200000;
pub const GNRC_IPV6_NIB_CONF_NO_RTR_SOL: u32 = 0;
pub const GNRC_IPV6_NIB_L2ADDR_MAX_LEN: u32 = 8;
pub const GNRC_IPV6_NIB_DEFAULT_ROUTER_NUMOF: u32 = 1;
pub const GNRC_IPV6_NIB_NUMOF: u32 = 4;
pub const GNRC_IPV6_NIB_OFFL_NUMOF: u32 = 8;
pub const GNRC_NETIF_NUMOF: u32 = 1;
pub const GNRC_NETIF_PRIO: u32 = 2;
pub const GNRC_NETIF_RPL_ADDR: u32 = 0;
pub const GNRC_NETIF_IPV6_RTR_ADDR: u32 = 0;
pub const GNRC_NETIF_IPV6_ADDRS_NUMOF: u32 = 2;
pub const GNRC_NETIF_IPV6_GROUPS_NUMOF: u32 = 3;
pub const GNRC_NETIF_L2ADDR_MAXLEN: u32 = 8;
pub const GNRC_NETIF_DEFAULT_HL: u32 = 64;
pub const GNRC_NETIF_FLAGS_HAS_L2ADDR: u32 = 1;
pub const GNRC_NETIF_FLAGS_IPV6_FORWARDING: u32 = 2;
pub const GNRC_NETIF_FLAGS_IPV6_RTR_ADV: u32 = 4;
pub const GNRC_NETIF_FLAGS_IPV6_ADV_MTU: u32 = 8;
pub const GNRC_NETIF_FLAGS_IPV6_ADV_CUR_HL: u32 = 16;
pub const GNRC_NETIF_FLAGS_IPV6_ADV_REACH_TIME: u32 = 32;
pub const GNRC_NETIF_FLAGS_IPV6_ADV_RETRANS_TIMER: u32 = 64;
pub const GNRC_NETIF_FLAGS_IPV6_ADV_O_FLAG: u32 = 128;
pub const GNRC_NETIF_FLAGS_6LO_HC: u32 = 256;
pub const GNRC_NETIF_FLAGS_6LO_ABR: u32 = 512;
pub const GNRC_NETIF_FLAGS_6LO_MESH: u32 = 1024;
pub const GNRC_NETIF_FLAGS_6LO_BACKBONE: u32 = 2048;
pub const GNRC_NETIF_FLAGS_MAC_TX_FEEDBACK_MASK: u32 = 24576;
pub const GNRC_NETIF_FLAGS_MAC_RX_STARTED: u32 = 32768;
pub const _UNISTD_H: u32 = 1;
pub const _POSIX_VERSION: u32 = 200809;
pub const __POSIX2_THIS_VERSION: u32 = 200809;
pub const _POSIX2_VERSION: u32 = 200809;
pub const _POSIX2_C_VERSION: u32 = 200809;
pub const _POSIX2_C_BIND: u32 = 200809;
pub const _POSIX2_C_DEV: u32 = 200809;
pub const _POSIX2_SW_DEV: u32 = 200809;
pub const _POSIX2_LOCALEDEF: u32 = 200809;
pub const _XOPEN_VERSION: u32 = 700;
pub const _XOPEN_XCU_VERSION: u32 = 4;
pub const _XOPEN_XPG2: u32 = 1;
pub const _XOPEN_XPG3: u32 = 1;
pub const _XOPEN_XPG4: u32 = 1;
pub const _XOPEN_UNIX: u32 = 1;
pub const _XOPEN_CRYPT: u32 = 1;
pub const _XOPEN_ENH_I18N: u32 = 1;
pub const _XOPEN_LEGACY: u32 = 1;
pub const _BITS_POSIX_OPT_H: u32 = 1;
pub const _POSIX_JOB_CONTROL: u32 = 1;
pub const _POSIX_SAVED_IDS: u32 = 1;
pub const _POSIX_PRIORITY_SCHEDULING: u32 = 200809;
pub const _POSIX_SYNCHRONIZED_IO: u32 = 200809;
pub const _POSIX_FSYNC: u32 = 200809;
pub const _POSIX_MAPPED_FILES: u32 = 200809;
pub const _POSIX_MEMLOCK: u32 = 200809;
pub const _POSIX_MEMLOCK_RANGE: u32 = 200809;
pub const _POSIX_MEMORY_PROTECTION: u32 = 200809;
pub const _POSIX_CHOWN_RESTRICTED: u32 = 0;
pub const _POSIX_VDISABLE: u8 = 0u8;
pub const _POSIX_NO_TRUNC: u32 = 1;
pub const _XOPEN_REALTIME: u32 = 1;
pub const _XOPEN_REALTIME_THREADS: u32 = 1;
pub const _XOPEN_SHM: u32 = 1;
pub const _POSIX_THREADS: u32 = 200809;
pub const _POSIX_REENTRANT_FUNCTIONS: u32 = 1;
pub const _POSIX_THREAD_SAFE_FUNCTIONS: u32 = 200809;
pub const _POSIX_THREAD_PRIORITY_SCHEDULING: u32 = 200809;
pub const _POSIX_THREAD_ATTR_STACKSIZE: u32 = 200809;
pub const _POSIX_THREAD_ATTR_STACKADDR: u32 = 200809;
pub const _POSIX_THREAD_PRIO_INHERIT: u32 = 200809;
pub const _POSIX_THREAD_PRIO_PROTECT: u32 = 200809;
pub const _POSIX_THREAD_ROBUST_PRIO_INHERIT: u32 = 200809;
pub const _POSIX_THREAD_ROBUST_PRIO_PROTECT: i32 = -1;
pub const _POSIX_SEMAPHORES: u32 = 200809;
pub const _POSIX_REALTIME_SIGNALS: u32 = 200809;
pub const _POSIX_ASYNCHRONOUS_IO: u32 = 200809;
pub const _POSIX_ASYNC_IO: u32 = 1;
pub const _LFS_ASYNCHRONOUS_IO: u32 = 1;
pub const _POSIX_PRIORITIZED_IO: u32 = 200809;
pub const _LFS64_ASYNCHRONOUS_IO: u32 = 1;
pub const _LFS_LARGEFILE: u32 = 1;
pub const _LFS64_LARGEFILE: u32 = 1;
pub const _LFS64_STDIO: u32 = 1;
pub const _POSIX_SHARED_MEMORY_OBJECTS: u32 = 200809;
pub const _POSIX_CPUTIME: u32 = 0;
pub const _POSIX_THREAD_CPUTIME: u32 = 0;
pub const _POSIX_REGEXP: u32 = 1;
pub const _POSIX_READER_WRITER_LOCKS: u32 = 200809;
pub const _POSIX_SHELL: u32 = 1;
pub const _POSIX_TIMEOUTS: u32 = 200809;
pub const _POSIX_SPIN_LOCKS: u32 = 200809;
pub const _POSIX_SPAWN: u32 = 200809;
pub const _POSIX_TIMERS: u32 = 200809;
pub const _POSIX_BARRIERS: u32 = 200809;
pub const _POSIX_MESSAGE_PASSING: u32 = 200809;
pub const _POSIX_THREAD_PROCESS_SHARED: u32 = 200809;
pub const _POSIX_MONOTONIC_CLOCK: u32 = 0;
pub const _POSIX_CLOCK_SELECTION: u32 = 200809;
pub const _POSIX_ADVISORY_INFO: u32 = 200809;
pub const _POSIX_IPV6: u32 = 200809;
pub const _POSIX_RAW_SOCKETS: u32 = 200809;
pub const _POSIX2_CHAR_TERM: u32 = 200809;
pub const _POSIX_SPORADIC_SERVER: i32 = -1;
pub const _POSIX_THREAD_SPORADIC_SERVER: i32 = -1;
pub const _POSIX_TRACE: i32 = -1;
pub const _POSIX_TRACE_EVENT_FILTER: i32 = -1;
pub const _POSIX_TRACE_INHERIT: i32 = -1;
pub const _POSIX_TRACE_LOG: i32 = -1;
pub const _POSIX_TYPED_MEMORY_OBJECTS: i32 = -1;
pub const _POSIX_V7_LPBIG_OFFBIG: i32 = -1;
pub const _POSIX_V6_LPBIG_OFFBIG: i32 = -1;
pub const _XBS5_LPBIG_OFFBIG: i32 = -1;
pub const _POSIX_V7_LP64_OFF64: u32 = 1;
pub const _POSIX_V6_LP64_OFF64: u32 = 1;
pub const _XBS5_LP64_OFF64: u32 = 1;
pub const __ILP32_OFF32_CFLAGS: &'static [u8; 5usize] = b"-m32\0";
pub const __ILP32_OFF32_LDFLAGS: &'static [u8; 5usize] = b"-m32\0";
pub const __ILP32_OFFBIG_CFLAGS: &'static [u8; 48usize] =
    b"-m32 -D_LARGEFILE_SOURCE -D_FILE_OFFSET_BITS=64\0";
pub const __ILP32_OFFBIG_LDFLAGS: &'static [u8; 5usize] = b"-m32\0";
pub const __LP64_OFF64_CFLAGS: &'static [u8; 5usize] = b"-m64\0";
pub const __LP64_OFF64_LDFLAGS: &'static [u8; 5usize] = b"-m64\0";
pub const STDIN_FILENO: u32 = 0;
pub const STDOUT_FILENO: u32 = 1;
pub const STDERR_FILENO: u32 = 2;
pub const R_OK: u32 = 4;
pub const W_OK: u32 = 2;
pub const X_OK: u32 = 1;
pub const F_OK: u32 = 0;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const L_SET: u32 = 0;
pub const L_INCR: u32 = 1;
pub const L_XTND: u32 = 2;
pub const _GETOPT_POSIX_H: u32 = 1;
pub const _GETOPT_CORE_H: u32 = 1;
pub const F_ULOCK: u32 = 0;
pub const F_LOCK: u32 = 1;
pub const F_TLOCK: u32 = 2;
pub const F_TEST: u32 = 3;
pub const _ERRNO_H: u32 = 1;
pub const _BITS_ERRNO_H: u32 = 1;
pub const EPERM: u32 = 1;
pub const ENOENT: u32 = 2;
pub const ESRCH: u32 = 3;
pub const EINTR: u32 = 4;
pub const EIO: u32 = 5;
pub const ENXIO: u32 = 6;
pub const E2BIG: u32 = 7;
pub const ENOEXEC: u32 = 8;
pub const EBADF: u32 = 9;
pub const ECHILD: u32 = 10;
pub const EAGAIN: u32 = 11;
pub const ENOMEM: u32 = 12;
pub const EACCES: u32 = 13;
pub const EFAULT: u32 = 14;
pub const ENOTBLK: u32 = 15;
pub const EBUSY: u32 = 16;
pub const EEXIST: u32 = 17;
pub const EXDEV: u32 = 18;
pub const ENODEV: u32 = 19;
pub const ENOTDIR: u32 = 20;
pub const EISDIR: u32 = 21;
pub const EINVAL: u32 = 22;
pub const ENFILE: u32 = 23;
pub const EMFILE: u32 = 24;
pub const ENOTTY: u32 = 25;
pub const ETXTBSY: u32 = 26;
pub const EFBIG: u32 = 27;
pub const ENOSPC: u32 = 28;
pub const ESPIPE: u32 = 29;
pub const EROFS: u32 = 30;
pub const EMLINK: u32 = 31;
pub const EPIPE: u32 = 32;
pub const EDOM: u32 = 33;
pub const ERANGE: u32 = 34;
pub const EDEADLK: u32 = 35;
pub const ENAMETOOLONG: u32 = 36;
pub const ENOLCK: u32 = 37;
pub const ENOSYS: u32 = 38;
pub const ENOTEMPTY: u32 = 39;
pub const ELOOP: u32 = 40;
pub const EWOULDBLOCK: u32 = 11;
pub const ENOMSG: u32 = 42;
pub const EIDRM: u32 = 43;
pub const ECHRNG: u32 = 44;
pub const EL2NSYNC: u32 = 45;
pub const EL3HLT: u32 = 46;
pub const EL3RST: u32 = 47;
pub const ELNRNG: u32 = 48;
pub const EUNATCH: u32 = 49;
pub const ENOCSI: u32 = 50;
pub const EL2HLT: u32 = 51;
pub const EBADE: u32 = 52;
pub const EBADR: u32 = 53;
pub const EXFULL: u32 = 54;
pub const ENOANO: u32 = 55;
pub const EBADRQC: u32 = 56;
pub const EBADSLT: u32 = 57;
pub const EDEADLOCK: u32 = 35;
pub const EBFONT: u32 = 59;
pub const ENOSTR: u32 = 60;
pub const ENODATA: u32 = 61;
pub const ETIME: u32 = 62;
pub const ENOSR: u32 = 63;
pub const ENONET: u32 = 64;
pub const ENOPKG: u32 = 65;
pub const EREMOTE: u32 = 66;
pub const ENOLINK: u32 = 67;
pub const EADV: u32 = 68;
pub const ESRMNT: u32 = 69;
pub const ECOMM: u32 = 70;
pub const EPROTO: u32 = 71;
pub const EMULTIHOP: u32 = 72;
pub const EDOTDOT: u32 = 73;
pub const EBADMSG: u32 = 74;
pub const EOVERFLOW: u32 = 75;
pub const ENOTUNIQ: u32 = 76;
pub const EBADFD: u32 = 77;
pub const EREMCHG: u32 = 78;
pub const ELIBACC: u32 = 79;
pub const ELIBBAD: u32 = 80;
pub const ELIBSCN: u32 = 81;
pub const ELIBMAX: u32 = 82;
pub const ELIBEXEC: u32 = 83;
pub const EILSEQ: u32 = 84;
pub const ERESTART: u32 = 85;
pub const ESTRPIPE: u32 = 86;
pub const EUSERS: u32 = 87;
pub const ENOTSOCK: u32 = 88;
pub const EDESTADDRREQ: u32 = 89;
pub const EMSGSIZE: u32 = 90;
pub const EPROTOTYPE: u32 = 91;
pub const ENOPROTOOPT: u32 = 92;
pub const EPROTONOSUPPORT: u32 = 93;
pub const ESOCKTNOSUPPORT: u32 = 94;
pub const EOPNOTSUPP: u32 = 95;
pub const EPFNOSUPPORT: u32 = 96;
pub const EAFNOSUPPORT: u32 = 97;
pub const EADDRINUSE: u32 = 98;
pub const EADDRNOTAVAIL: u32 = 99;
pub const ENETDOWN: u32 = 100;
pub const ENETUNREACH: u32 = 101;
pub const ENETRESET: u32 = 102;
pub const ECONNABORTED: u32 = 103;
pub const ECONNRESET: u32 = 104;
pub const ENOBUFS: u32 = 105;
pub const EISCONN: u32 = 106;
pub const ENOTCONN: u32 = 107;
pub const ESHUTDOWN: u32 = 108;
pub const ETOOMANYREFS: u32 = 109;
pub const ETIMEDOUT: u32 = 110;
pub const ECONNREFUSED: u32 = 111;
pub const EHOSTDOWN: u32 = 112;
pub const EHOSTUNREACH: u32 = 113;
pub const EALREADY: u32 = 114;
pub const EINPROGRESS: u32 = 115;
pub const ESTALE: u32 = 116;
pub const EUCLEAN: u32 = 117;
pub const ENOTNAM: u32 = 118;
pub const ENAVAIL: u32 = 119;
pub const EISNAM: u32 = 120;
pub const EREMOTEIO: u32 = 121;
pub const EDQUOT: u32 = 122;
pub const ENOMEDIUM: u32 = 123;
pub const EMEDIUMTYPE: u32 = 124;
pub const ECANCELED: u32 = 125;
pub const ENOKEY: u32 = 126;
pub const EKEYEXPIRED: u32 = 127;
pub const EKEYREVOKED: u32 = 128;
pub const EKEYREJECTED: u32 = 129;
pub const EOWNERDEAD: u32 = 130;
pub const ENOTRECOVERABLE: u32 = 131;
pub const ERFKILL: u32 = 132;
pub const EHWPOISON: u32 = 133;
pub const ENOTSUP: u32 = 95;
pub const GNRC_NETERR_MSG_TYPE: u32 = 518;
pub const GNRC_NETERR_SUCCESS: u32 = 0;
pub const GNRC_PKTBUF_SIZE: u32 = 6144;
pub const GNRC_NETIF_HDR_L2ADDR_MAX_LEN: u32 = 8;
pub const GNRC_NETIF_HDR_L2ADDR_PRINT_LEN: u32 = 24;
pub const GNRC_NETIF_HDR_FLAGS_BROADCAST: u32 = 128;
pub const GNRC_NETIF_HDR_FLAGS_MULTICAST: u32 = 64;
pub const GNRC_NETIF_HDR_FLAGS_MORE_DATA: u32 = 16;
/// @brief   (uint16_t) channel number
pub const netopt_t_NETOPT_CHANNEL: netopt_t = 0;
/// @brief   (@ref netopt_enable_t) check whether the network medium is clear
///
/// Getting this option can be used to trigger a manual clear channel
/// assessment (CCA) on some wireless devices.
pub const netopt_t_NETOPT_IS_CHANNEL_CLR: netopt_t = 1;
/// @brief   (byte array, see below) link layer address in network byte order
///
/// Device type   | Length | Meaning
/// ------------- | ------ | -----
/// IEEE 802.15.4 | 2      | device short address
/// Ethernet      | 6      | device MAC address
/// nrfmin        | 2      | device short address
/// CC110x        | 1      | device address
pub const netopt_t_NETOPT_ADDRESS: netopt_t = 2;
/// @brief   (byte array, see below) long link layer address in network byte order
///
/// Device type   | Length   | Meaning
/// ------------- | -------- | -----
/// IEEE 802.15.4 | 8        | device long address (EUI-64), @ref eui64_t
/// nrfmin        | 8        | device long address (based on short address)
/// BLE           | 8        | device long address (EUI-64), @ref eui64_t
pub const netopt_t_NETOPT_ADDRESS_LONG: netopt_t = 3;
/// @brief   (uint16_t) get the default address length a network device expects
pub const netopt_t_NETOPT_ADDR_LEN: netopt_t = 4;
/// @brief   (uint16_t) address length to use for the link layer source address
pub const netopt_t_NETOPT_SRC_LEN: netopt_t = 5;
/// @brief   (uint16_t) network ID
///
/// Examples for this include the PAN ID in IEEE 802.15.4
pub const netopt_t_NETOPT_NID: netopt_t = 6;
/// @brief   (uint8_t) hop limit
pub const netopt_t_NETOPT_HOP_LIMIT: netopt_t = 7;
/// @brief   (@ref eui64_t) get the IPv6 interface identifier of a network interface
///
/// @see <a href="https://tools.ietf.org/html/rfc4291#section-2.5.1">
/// RFC 4291, section 2.5.1
/// </a>
///
/// The generation of the interface identifier is dependent on the link-layer.
/// Please refer to the appropriate IPv6 over `<link>` specification for
/// further implementation details (such as
/// <a href="https://tools.ietf.org/html/rfc2464">RFC 2464</a> or
/// <a href="https://tools.ietf.org/html/rfc4944">RFC 4944</a>).
pub const netopt_t_NETOPT_IPV6_IID: netopt_t = 8;
/// @brief   (@ref ipv6_addr_t[]) get IPv6 addresses of an interface as array
/// of @ref ipv6_addr_t or add an IPv6 address as @ref ipv6_addr_t
/// to an interface
///
/// When adding an IPv6 address to a GNRC interface using
/// @ref GNRC_NETAPI_MSG_TYPE_SET, the gnrc_netapi_opt_t::context field can
/// be used to pass the prefix length (8 MSB) and some flags (8 LSB)
/// according to @ref net_gnrc_netif_ipv6_addrs_flags. The address is however
/// always considered to be manually added.
/// When getting the option you can pass an array of @ref ipv6_addr_t of any
/// length greater than 0 to the getter. The array will be filled up to to
/// its maximum and the remaining addresses on the interface will be ignored
pub const netopt_t_NETOPT_IPV6_ADDR: netopt_t = 9;
/// @brief   (@ref ipv6_addr_t) Removes an IPv6 address from an interface
pub const netopt_t_NETOPT_IPV6_ADDR_REMOVE: netopt_t = 10;
/// @brief   (array of uint8_t) get the flags to the addresses returned by
/// @ref NETOPT_IPV6_ADDR as array
///
/// The information contained in the array is very specific to the
/// interface's API. For GNRC e.g. the values are according to
/// @ref net_gnrc_netif_ipv6_addrs_flags.
pub const netopt_t_NETOPT_IPV6_ADDR_FLAGS: netopt_t = 11;
/// @brief   (@ref ipv6_addr_t) get IPv6 multicast groups of an interface as
/// array of @ref ipv6_addr_t or join an IPv6 multicast group as
/// @ref ipv6_addr_t on an interface
///
/// When adding an IPv6 address to a GNRC interface using
/// @ref GNRC_NETAPI_MSG_TYPE_SET, the gnrc_netapi_opt_t::context field can
/// be used to pass the prefix length (8 MSB) and some flags (8 LSB)
/// according to @ref net_gnrc_netif_ipv6_addrs_flags. The address is however always
/// considered to be manually added.
/// When getting the option you can pass an array of @ref ipv6_addr_t of any
/// length greater than 0 to the getter. The array will be filled up to to
/// its maximum and the remaining addresses on the interface will be ignored
pub const netopt_t_NETOPT_IPV6_GROUP: netopt_t = 12;
/// @brief   (@ref ipv6_addr_t) Leave an IPv6 multicast group on an interface
pub const netopt_t_NETOPT_IPV6_GROUP_LEAVE: netopt_t = 13;
/// @brief   (@ref netopt_enable_t) IPv6 forwarding state
pub const netopt_t_NETOPT_IPV6_FORWARDING: netopt_t = 14;
/// @brief   (@ref netopt_enable_t) sending of IPv6 router advertisements
pub const netopt_t_NETOPT_IPV6_SND_RTR_ADV: netopt_t = 15;
/// @brief   (int16_t) transmit power for radio devices in dBm
pub const netopt_t_NETOPT_TX_POWER: netopt_t = 16;
/// @brief   (uint16_t) maximum packet size a network module can handle
pub const netopt_t_NETOPT_MAX_PACKET_SIZE: netopt_t = 17;
/// @brief   (@ref netopt_enable_t) frame preloading
///
/// Preload frame data using gnrc_netdev_driver_t::send_data() or gnrc_netapi_send(),
/// trigger sending by setting state to @ref NETOPT_STATE_TX
pub const netopt_t_NETOPT_PRELOADING: netopt_t = 18;
/// @brief   (@ref netopt_enable_t) promiscuous mode
pub const netopt_t_NETOPT_PROMISCUOUSMODE: netopt_t = 19;
/// @brief   (@ref netopt_enable_t) automatic link layer ACKs
pub const netopt_t_NETOPT_AUTOACK: netopt_t = 20;
/// @brief   (@ref netopt_enable_t) frame pending bit of ACKs
///
/// For IEEE 802.15.4, this bit is copied into the frame pending subfield of
/// the ACK if it is the response to a data request MAC command frame.
pub const netopt_t_NETOPT_ACK_PENDING: netopt_t = 21;
/// @brief   (@ref netopt_enable_t) acknowledgement request on outgoing frames
///
/// For IEEE 802.15.4, this bit is copied into the ACK req subfield of the
/// frame control field.
pub const netopt_t_NETOPT_ACK_REQ: netopt_t = 22;
/// @brief   (uint8_t) maximum number of retransmissions
pub const netopt_t_NETOPT_RETRANS: netopt_t = 23;
/// @brief   (@ref gnrc_nettype_t) the protocol for the layer
pub const netopt_t_NETOPT_PROTO: netopt_t = 24;
/// @brief   (@ref netopt_state_t) state of network device
pub const netopt_t_NETOPT_STATE: netopt_t = 25;
/// @brief   (@ref netopt_enable_t) when enabled, bypass protocol processing of incoming frames
pub const netopt_t_NETOPT_RAWMODE: netopt_t = 26;
/// @brief   (@ref netopt_enable_t) trigger interrupt at reception start
///
/// It is mostly triggered after the preamble is correctly received
///
/// @note not all transceivers may support this interrupt
pub const netopt_t_NETOPT_RX_START_IRQ: netopt_t = 27;
/// @brief   (@ref netopt_enable_t) trigger interrupt after frame reception
///
/// This interrupt is triggered after a complete frame is received.
///
/// @note in case a transceiver does not support this interrupt, the event
/// may be triggered by the driver
pub const netopt_t_NETOPT_RX_END_IRQ: netopt_t = 28;
/// @brief   (@ref netopt_enable_t) trigger interrupt at transmission start
///
/// This interrupt is triggered when the transceiver starts to send out the
/// frame.
///
/// @note in case a transceiver does not support this interrupt, the event
/// may be triggered by the driver
pub const netopt_t_NETOPT_TX_START_IRQ: netopt_t = 29;
/// @brief   (@ref netopt_enable_t) trigger interrupt after frame transmission
///
/// This interrupt is triggered when the full frame has been transmitted.
///
/// @note not all transceivers may support this interrupt
pub const netopt_t_NETOPT_TX_END_IRQ: netopt_t = 30;
/// @brief   (@ref netopt_enable_t) perform channel clear assessment before transmitting
///
/// This may be a hardware feature of the given transceiver, or might be
/// otherwise implemented in software. If the device supports CSMA this
/// option will enable CSMA with a certain set of parameters to emulate the
/// desired behaviour.
///
/// @note Be sure not to set NETOPT_CSMA simultaneously.
///
/// @todo How to get feedback?
pub const netopt_t_NETOPT_AUTOCCA: netopt_t = 31;
/// @brief (@ref netopt_enable_t) Phy link status.
///
/// Returns NETOPT_ENABLE when the the link of the interface is up,
/// NETOPT_DISABLE when the link is down. If the interface is wireless or
/// doesn't support link status detection this function will return
/// -ENOTSUP.
///
/// @note Setting this option will always return -ENOTSUP.
pub const netopt_t_NETOPT_LINK_CONNECTED: netopt_t = 32;
/// @brief   (@ref netopt_enable_t) CSMA/CA support
///
/// If the device supports CSMA in hardware, this option enables it with
/// default parameters. For further configuration refer to the other
/// NETOPT_CSMA_* options.
pub const netopt_t_NETOPT_CSMA: netopt_t = 33;
/// @brief   (uint8_t) maximum number of CSMA retries
///
/// The maximum number of backoffs the CSMA-CA algorithm will attempt before
/// declaring a channel access failure. Named macMaxCsmaBackoffs in
/// IEEE Std 802.15.4-2015.
///
/// IEEE 802.15.4 default: 4
pub const netopt_t_NETOPT_CSMA_RETRIES: netopt_t = 34;
/// @brief   (uint8_t) maximum backoff exponent for the CSMA-CA algorithm
///
/// Named macMaxBE in IEEE Std 802.15.4-2015.
///
/// IEEE 802.15.4 default: 5
pub const netopt_t_NETOPT_CSMA_MAXBE: netopt_t = 35;
/// @brief   (uint8_t) minimum backoff exponent for the CSMA-CA algorithm
///
/// Named macMinBE in IEEE Std 802.15.4-2015.
///
/// IEEE 802.15.4 default: 3
pub const netopt_t_NETOPT_CSMA_MINBE: netopt_t = 36;
/// @brief   (@ref netopt_enable_t) block transceiver sleep
///
/// Enabling this option tells the MAC layer to never put the radio to sleep.
/// Useful in gateways and routers not running on batteries to improve
/// responsiveness and allow battery powered nodes on the same network to
/// sleep more often.
pub const netopt_t_NETOPT_MAC_NO_SLEEP: netopt_t = 37;
/// @brief   (@ref netopt_enable_t) read-only check for a wired interface.
///
/// This option will return -ENOTSUP for wireless interfaces.
///
/// @note Setting this option will always return -ENOTSUP.
pub const netopt_t_NETOPT_IS_WIRED: netopt_t = 38;
/// @brief   (uint16_t) device type
///
/// e.g. NETDEV_TYPE_ETHERNET, NETDEV_TYPE_IEEE802154, etc.
pub const netopt_t_NETOPT_DEVICE_TYPE: netopt_t = 39;
/// @brief   (uint8_t) channel page as defined by IEEE 802.15.4
pub const netopt_t_NETOPT_CHANNEL_PAGE: netopt_t = 40;
/// @brief   (int8_t) CCA threshold for the radio transceiver
///
/// This is the value, in dBm, that the radio transceiver uses to decide
/// whether the channel is clear or not (CCA). If the current signal strength
/// (RSSI/ED) is stronger than this CCA threshold value, the transceiver
/// usually considers that the radio medium is busy. Otherwise, i.e. if
/// RSSI/ED value is less than the CCA threshold value, the radio medium is
/// supposed to be free (the possibly received weak signal is considered to
/// be background, meaningless noise).
///
/// Most transceivers allow to set this CCA threshold value. Some research
/// work has proven that dynamically adapting it to network environment can
/// improve QoS, especially in WSN.
pub const netopt_t_NETOPT_CCA_THRESHOLD: netopt_t = 41;
/// @brief   (uint8_t) CCA mode for the radio transceiver
///
/// Get/set the CCA mode corresponding to the respective PHY standard.
/// - IEEE 802.15.4: @ref netdev_ieee802154_cca_mode_t
pub const netopt_t_NETOPT_CCA_MODE: netopt_t = 42;
/// @brief   (@ref netstats_t*) get statistics about sent and received packets and data of the device or protocol
///
/// Expects a pointer to a @ref netstats_t struct that will be pointed to
/// the corresponding @ref netstats_t of the module.
pub const netopt_t_NETOPT_STATS: netopt_t = 43;
/// @brief   (@ref netopt_enable_t) link layer encryption.
pub const netopt_t_NETOPT_ENCRYPTION: netopt_t = 44;
/// @brief   (byte array) set encryption key
///
/// The required byte array size is dependent on encryption algorithm and device.
pub const netopt_t_NETOPT_ENCRYPTION_KEY: netopt_t = 45;
/// @brief   (@ref netopt_rf_testmode_t) Test mode for the radio, e.g. for CE or FCC certification
///
/// Get/set the test mode as type @ref netopt_rf_testmode_t or as uint8_t if
/// the radio supports other vendor specific test modes.
///
/// @note Setting this option should always return -ENOTSUP, unless it was
/// explicitly allowed at build time, therefore it should be secured with an
/// additional macro in the device driver.
///
/// @attention For development and certification purposes only! These test
/// modes can disturb normal radio communications and exceed the limits
/// established by the regulatory authority.
///
pub const netopt_t_NETOPT_RF_TESTMODE: netopt_t = 46;
/// @brief   (@ref l2filter_t) add an address to a link layer filter list
///
/// Getting this option from a device will return a pointer of type
/// @ref l2filter_t to the first entry of a filter list.
/// When setting this option a pointer to an link layer address as well as
/// the length of the address are expected as parameters.
pub const netopt_t_NETOPT_L2FILTER: netopt_t = 47;
/// @brief   (@ref l2filter_t) remove an address from a link layer filter list
///
/// Getting this value always returns -ENOTSUP.
/// When setting this option a pointer to an link layer address as well as
/// the length of the address are expected as parameters. Setting this
/// option will lead to the given address being removed from the filer list.
pub const netopt_t_NETOPT_L2FILTER_RM: netopt_t = 48;
/// @brief   (int8_t) Energy level during the last performed CCA or RX frame
///
/// Get the last ED level available as an int8_t. The source of the
/// measurement is unspecified and may come from the latest CCA
/// measurement (CCA mode 1), or from the last received frame.
pub const netopt_t_NETOPT_LAST_ED_LEVEL: netopt_t = 49;
/// @brief   (uint16_t) preamble length
pub const netopt_t_NETOPT_PREAMBLE_LENGTH: netopt_t = 50;
/// @brief   (@ref netopt_enable_t) frame integrity check (e.g CRC)
pub const netopt_t_NETOPT_INTEGRITY_CHECK: netopt_t = 51;
/// @brief   (uint32_t) channel center frequency
///
/// For example, with LoRa, this corresponds to the center frequency of
/// each channel (867300000, etc) for a given frequency band
/// (868, 915, etc).
pub const netopt_t_NETOPT_CHANNEL_FREQUENCY: netopt_t = 52;
/// @brief   (@ref netopt_enable_t) channel hopping
pub const netopt_t_NETOPT_CHANNEL_HOP: netopt_t = 53;
/// @brief   (uint8_t) channel hopping period
pub const netopt_t_NETOPT_CHANNEL_HOP_PERIOD: netopt_t = 54;
/// @brief   (@ref netopt_enable_t) single frame reception
///
/// If enabled, RX is turned off upon reception of a frame
pub const netopt_t_NETOPT_SINGLE_RECEIVE: netopt_t = 55;
/// @brief   (uint32_t) reception timeout of a frame
///
/// @todo in what time unit?
pub const netopt_t_NETOPT_RX_TIMEOUT: netopt_t = 56;
/// @brief   (uint32_t) transmission timeout of a frame
///
/// @todo in what time unit?
pub const netopt_t_NETOPT_TX_TIMEOUT: netopt_t = 57;
/// @brief   (uint8_t) radio modulation bandwidth
pub const netopt_t_NETOPT_BANDWIDTH: netopt_t = 58;
/// @brief   (uint8_t) radio spreading factor
pub const netopt_t_NETOPT_SPREADING_FACTOR: netopt_t = 59;
/// @brief   (uint8_t) radio coding rate
pub const netopt_t_NETOPT_CODING_RATE: netopt_t = 60;
/// @brief   (@ref netopt_enable_t) fixed header mode
pub const netopt_t_NETOPT_FIXED_HEADER: netopt_t = 61;
/// @brief   (@ref netopt_enable_t) IQ inverted
pub const netopt_t_NETOPT_IQ_INVERT: netopt_t = 62;
/// @brief   (@ref netopt_enable_t) header compression
///
/// @see [RFC 6282](https://tools.ietf.org/html/rfc6282)
pub const netopt_t_NETOPT_6LO_IPHC: netopt_t = 63;
/// @brief   (uint8_t) retry amount from missing ACKs of the last transmission
///
/// This retrieves the number of retries needed for the last transmission.
/// Only retransmissions due to missing ACK frames are considered, retries
/// due to CCA failures are not counted.
pub const netopt_t_NETOPT_TX_RETRIES_NEEDED: netopt_t = 64;
/// @brief   (netdev_ble_ctx_t) set BLE radio context (channel, CRC, AA)
///
/// @warning As @ref drivers_netdev_ble is still experimental, use with care!
pub const netopt_t_NETOPT_BLE_CTX: netopt_t = 65;
/// @brief   (@ref netopt_enable_t) enable hardware checksumming
///
/// If enabled, enable hardware checksumming of incoming frames.
pub const netopt_t_NETOPT_CHECKSUM: netopt_t = 66;
/// @brief   (@ref netopt_enable_t) enable busy mode
///
/// When set, the PHY will enter busy mode, in which it will not accept
/// incoming frames until unset.
pub const netopt_t_NETOPT_PHY_BUSY: netopt_t = 67;
/// @brief   maximum number of options defined here.
///
/// @note    Interfaces are not meant to respond to this option
pub const netopt_t_NETOPT_NUMOF: netopt_t = 68;
/// @brief   Global list of configuration options available throughout the
/// network stack, e.g. by netdev and netapi
///
/// The data type specified in parentheses for each individual option is the
/// data type to use for the argument when getting/setting the value of the option.
///
/// All arguments longer than 1 byte (e.g. uint16_t) are given in host byte order
/// unless anything else is specified below.
pub type netopt_t = u32;
/// < disable a given option
pub const netopt_enable_t_NETOPT_DISABLE: netopt_enable_t = 0;
/// < enable a given option
pub const netopt_enable_t_NETOPT_ENABLE: netopt_enable_t = 1;
/// @brief   Binary parameter for enabling and disabling options
pub type netopt_enable_t = u32;
/// < powered off
pub const netopt_state_t_NETOPT_STATE_OFF: netopt_state_t = 0;
/// < sleep mode
pub const netopt_state_t_NETOPT_STATE_SLEEP: netopt_state_t = 1;
/// < idle mode,
/// the device listens to receive packets
pub const netopt_state_t_NETOPT_STATE_IDLE: netopt_state_t = 2;
/// < receive mode,
/// the device currently receives a packet
pub const netopt_state_t_NETOPT_STATE_RX: netopt_state_t = 3;
/// < transmit mode,
/// set: triggers transmission of a preloaded packet
/// (see @ref NETOPT_PRELOADING*). The resulting
/// state of the network device is @ref NETOPT_STATE_IDLE
/// get: the network device is in the process of
/// transmitting a packet
pub const netopt_state_t_NETOPT_STATE_TX: netopt_state_t = 4;
/// < triggers a hardware reset. The resulting
/// state of the network device is @ref NETOPT_STATE_IDLE
pub const netopt_state_t_NETOPT_STATE_RESET: netopt_state_t = 5;
/// < standby mode. The devices is awake but
/// not listening to packets.
pub const netopt_state_t_NETOPT_STATE_STANDBY: netopt_state_t = 6;
/// @brief   Option parameter to be used with @ref NETOPT_STATE to set or get
/// the state of a network device or protocol implementation
pub type netopt_state_t = u32;
/// < idle mode, radio off
pub const netopt_rf_testmode_t_NETOPT_RF_TESTMODE_IDLE: netopt_rf_testmode_t = 0;
/// < continuous rx mode
pub const netopt_rf_testmode_t_NETOPT_RF_TESTMODE_CRX: netopt_rf_testmode_t = 1;
/// < carrier wave continuous tx mode
pub const netopt_rf_testmode_t_NETOPT_RF_TESTMODE_CTX_CW: netopt_rf_testmode_t = 2;
/// < PRBS9 continuous tx mode
pub const netopt_rf_testmode_t_NETOPT_RF_TESTMODE_CTX_PRBS9: netopt_rf_testmode_t = 3;
/// @brief   Option parameter to be used with @ref NETOPT_RF_TESTMODE
pub type netopt_rf_testmode_t = u32;
extern "C" {
    /// @brief   Get a string ptr corresponding to opt, for debugging
    ///
    /// @param[in] opt   The option to get a string representation for
    ///
    /// @return          ptr to string representation for given option or "unknown"
    pub fn netopt2str(opt: netopt_t) -> *const libc::c_char;
}
pub type wchar_t = libc::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: libc::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: f64,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(
        ::core::mem::size_of::<max_align_t>(),
        32usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<max_align_t>())).__clang_max_align_nonce1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<max_align_t>())).__clang_max_align_nonce2 as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}
/// @brief List node structure
///
/// Used as is as reference to a list, or as member of any data structure that
/// should be member of a list.
///
/// Actual list objects should have a @c list_node_t as member and then use
/// the container_of() macro in list operations.
/// See @ref thread_add_to_list() as example.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct list_node {
    /// < pointer to next list entry
    pub next: *mut list_node,
}
#[test]
fn bindgen_test_layout_list_node() {
    assert_eq!(
        ::core::mem::size_of::<list_node>(),
        8usize,
        concat!("Size of: ", stringify!(list_node))
    );
    assert_eq!(
        ::core::mem::align_of::<list_node>(),
        8usize,
        concat!("Alignment of ", stringify!(list_node))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<list_node>())).next as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(list_node),
            "::",
            stringify!(next)
        )
    );
}
pub type list_node_t = list_node;
/// @brief List node structure
///
/// Used as is as reference to a list.
///
pub type clist_node_t = list_node_t;
/// @brief Typedef for comparison function used by @ref clist_sort()
///
pub type clist_cmp_func_t = ::core::option::Option<
    unsafe extern "C" fn(a: *mut clist_node_t, b: *mut clist_node_t) -> libc::c_int,
>;
extern "C" {
    /// @brief   List sorting helper function
    ///
    /// @internal
    ///
    /// @param[in]   list    ptr to first element of list
    /// @param[in]   cmp     comparison function
    ///
    /// @returns     ptr to *last* element in list
    pub fn _clist_sort(list_head: *mut clist_node_t, cmp: clist_cmp_func_t) -> *mut clist_node_t;
}
pub const core_panic_t_PANIC_GENERAL_ERROR: core_panic_t = 0;
pub const core_panic_t_PANIC_SOFT_REBOOT: core_panic_t = 1;
pub const core_panic_t_PANIC_HARD_REBOOT: core_panic_t = 2;
pub const core_panic_t_PANIC_ASSERT_FAIL: core_panic_t = 3;
/// < stack smashing protector failure
pub const core_panic_t_PANIC_SSP: core_panic_t = 4;
pub const core_panic_t_PANIC_UNDEFINED: core_panic_t = 5;
/// @brief Definition of available panic modes
pub type core_panic_t = u32;
extern "C" {
    /// @brief Handle an unrecoverable error by halting or rebooting the system
    ///
    /// A numeric code indicating the failure reason can be given
    /// as the *crash_code* parameter.
    ///
    /// Detailing the failure is possible using the *message* parameter.
    /// This function should serve a similar purpose as the panic()
    /// function of Unix/Linux kernels.
    ///
    /// If the DEVELHELP macro is defined, the system will be halted;
    /// the system will be rebooted otherwise.
    ///
    /// @warning this function NEVER returns!
    ///
    /// @param[in] crash_code    a unique code for identifying the crash reason
    /// @param[in] message       a human readable reason for the crash
    ///
    /// @return                  this function never returns
    ///
    pub fn core_panic(crash_code: core_panic_t, message: *const libc::c_char);
}
extern "C" {
    /// @brief architecture dependent handling of a panic case
    ///
    /// This function gives the CPU the possibility to execute architecture
    /// dependent code in case of a severe error.
    pub fn panic_arch();
}
extern "C" {
    #[link_name = "\u{1}assert_crash_message"]
    pub static mut assert_crash_message: [libc::c_char; 0usize];
}
/// @brief circular integer buffer structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cib_t {
    /// < number of (successful) read accesses
    pub read_count: libc::c_uint,
    /// < number of (successful) write accesses
    pub write_count: libc::c_uint,
    /// < Size of buffer -1, i.e. mask of the bits
    pub mask: libc::c_uint,
}
#[test]
fn bindgen_test_layout_cib_t() {
    assert_eq!(
        ::core::mem::size_of::<cib_t>(),
        12usize,
        concat!("Size of: ", stringify!(cib_t))
    );
    assert_eq!(
        ::core::mem::align_of::<cib_t>(),
        4usize,
        concat!("Alignment of ", stringify!(cib_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<cib_t>())).read_count as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cib_t),
            "::",
            stringify!(read_count)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<cib_t>())).write_count as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cib_t),
            "::",
            stringify!(write_count)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<cib_t>())).mask as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cib_t),
            "::",
            stringify!(mask)
        )
    );
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __u_long = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __quad_t = libc::c_long;
pub type __u_quad_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [libc::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___fsid_t() {
    assert_eq!(
        ::core::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        ::core::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__fsid_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__fsid_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type __clock_t = libc::c_long;
pub type __rlim_t = libc::c_ulong;
pub type __rlim64_t = libc::c_ulong;
pub type __id_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __daddr_t = libc::c_int;
pub type __key_t = libc::c_int;
pub type __clockid_t = libc::c_int;
pub type __timer_t = *mut libc::c_void;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __blkcnt64_t = libc::c_long;
pub type __fsblkcnt_t = libc::c_ulong;
pub type __fsblkcnt64_t = libc::c_ulong;
pub type __fsfilcnt_t = libc::c_ulong;
pub type __fsfilcnt64_t = libc::c_ulong;
pub type __fsword_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __syscall_ulong_t = libc::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut libc::c_char;
pub type __intptr_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type __sig_atomic_t = libc::c_int;
pub type int_least8_t = libc::c_schar;
pub type int_least16_t = libc::c_short;
pub type int_least32_t = libc::c_int;
pub type int_least64_t = libc::c_long;
pub type uint_least8_t = libc::c_uchar;
pub type uint_least16_t = libc::c_ushort;
pub type uint_least32_t = libc::c_uint;
pub type uint_least64_t = libc::c_ulong;
pub type int_fast8_t = libc::c_schar;
pub type int_fast16_t = libc::c_long;
pub type int_fast32_t = libc::c_long;
pub type int_fast64_t = libc::c_long;
pub type uint_fast8_t = libc::c_uchar;
pub type uint_fast16_t = libc::c_ulong;
pub type uint_fast32_t = libc::c_ulong;
pub type uint_fast64_t = libc::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type __gwchar_t = libc::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct imaxdiv_t {
    pub quot: libc::c_long,
    pub rem: libc::c_long,
}
#[test]
fn bindgen_test_layout_imaxdiv_t() {
    assert_eq!(
        ::core::mem::size_of::<imaxdiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(imaxdiv_t))
    );
    assert_eq!(
        ::core::mem::align_of::<imaxdiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(imaxdiv_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<imaxdiv_t>())).quot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(imaxdiv_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<imaxdiv_t>())).rem as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(imaxdiv_t),
            "::",
            stringify!(rem)
        )
    );
}
extern "C" {
    pub fn imaxabs(__n: intmax_t) -> intmax_t;
}
extern "C" {
    pub fn imaxdiv(__numer: intmax_t, __denom: intmax_t) -> imaxdiv_t;
}
extern "C" {
    pub fn strtoimax(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> intmax_t;
}
extern "C" {
    pub fn strtoumax(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> uintmax_t;
}
extern "C" {
    pub fn wcstoimax(
        __nptr: *const __gwchar_t,
        __endptr: *mut *mut __gwchar_t,
        __base: libc::c_int,
    ) -> intmax_t;
}
extern "C" {
    pub fn wcstoumax(
        __nptr: *const __gwchar_t,
        __endptr: *mut *mut __gwchar_t,
        __base: libc::c_int,
    ) -> uintmax_t;
}
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type id_t = __id_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type timer_t = __timer_t;
pub type ulong = libc::c_ulong;
pub type ushort = libc::c_ushort;
pub type uint = libc::c_uint;
pub type u_int8_t = libc::c_uchar;
pub type u_int16_t = libc::c_ushort;
pub type u_int32_t = libc::c_uint;
pub type u_int64_t = libc::c_ulong;
pub type register_t = libc::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___sigset_t() {
    assert_eq!(
        ::core::mem::size_of::<__sigset_t>(),
        128usize,
        concat!("Size of: ", stringify!(__sigset_t))
    );
    assert_eq!(
        ::core::mem::align_of::<__sigset_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__sigset_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__sigset_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigset_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type sigset_t = __sigset_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[test]
fn bindgen_test_layout_timeval() {
    assert_eq!(
        ::core::mem::size_of::<timeval>(),
        16usize,
        concat!("Size of: ", stringify!(timeval))
    );
    assert_eq!(
        ::core::mem::align_of::<timeval>(),
        8usize,
        concat!("Alignment of ", stringify!(timeval))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<timeval>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<timeval>())).tv_usec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval),
            "::",
            stringify!(tv_usec)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[test]
fn bindgen_test_layout_timespec() {
    assert_eq!(
        ::core::mem::size_of::<timespec>(),
        16usize,
        concat!("Size of: ", stringify!(timespec))
    );
    assert_eq!(
        ::core::mem::align_of::<timespec>(),
        8usize,
        concat!("Alignment of ", stringify!(timespec))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<timespec>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<timespec>())).tv_nsec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec),
            "::",
            stringify!(tv_nsec)
        )
    );
}
pub type suseconds_t = __suseconds_t;
pub type __fd_mask = libc::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16usize],
}
#[test]
fn bindgen_test_layout_fd_set() {
    assert_eq!(
        ::core::mem::size_of::<fd_set>(),
        128usize,
        concat!("Size of: ", stringify!(fd_set))
    );
    assert_eq!(
        ::core::mem::align_of::<fd_set>(),
        8usize,
        concat!("Alignment of ", stringify!(fd_set))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<fd_set>())).__fds_bits as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fd_set),
            "::",
            stringify!(__fds_bits)
        )
    );
}
pub type fd_mask = __fd_mask;
extern "C" {
    pub fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pselect(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> libc::c_int;
}
extern "C" {
    pub fn gnu_dev_major(__dev: __dev_t) -> libc::c_uint;
}
extern "C" {
    pub fn gnu_dev_minor(__dev: __dev_t) -> libc::c_uint;
}
extern "C" {
    pub fn gnu_dev_makedev(__major: libc::c_uint, __minor: libc::c_uint) -> __dev_t;
}
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: libc::c_uint,
    pub __writers: libc::c_uint,
    pub __wrphase_futex: libc::c_uint,
    pub __writers_futex: libc::c_uint,
    pub __pad3: libc::c_uint,
    pub __pad4: libc::c_uint,
    pub __cur_writer: libc::c_int,
    pub __shared: libc::c_int,
    pub __rwelision: libc::c_schar,
    pub __pad1: [libc::c_uchar; 7usize],
    pub __pad2: libc::c_ulong,
    pub __flags: libc::c_uint,
}
#[test]
fn bindgen_test_layout___pthread_rwlock_arch_t() {
    assert_eq!(
        ::core::mem::size_of::<__pthread_rwlock_arch_t>(),
        56usize,
        concat!("Size of: ", stringify!(__pthread_rwlock_arch_t))
    );
    assert_eq!(
        ::core::mem::align_of::<__pthread_rwlock_arch_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_rwlock_arch_t))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__pthread_rwlock_arch_t>())).__readers as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__readers)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__pthread_rwlock_arch_t>())).__writers as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__writers)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__pthread_rwlock_arch_t>())).__wrphase_futex as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__wrphase_futex)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__pthread_rwlock_arch_t>())).__writers_futex as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__writers_futex)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__pthread_rwlock_arch_t>())).__pad3 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad3)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__pthread_rwlock_arch_t>())).__pad4 as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__pthread_rwlock_arch_t>())).__cur_writer as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__cur_writer)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__pthread_rwlock_arch_t>())).__shared as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__shared)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__pthread_rwlock_arch_t>())).__rwelision as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__rwelision)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__pthread_rwlock_arch_t>())).__pad1 as *const _ as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad1)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__pthread_rwlock_arch_t>())).__pad2 as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__pthread_rwlock_arch_t>())).__flags as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
#[test]
fn bindgen_test_layout___pthread_internal_list() {
    assert_eq!(
        ::core::mem::size_of::<__pthread_internal_list>(),
        16usize,
        concat!("Size of: ", stringify!(__pthread_internal_list))
    );
    assert_eq!(
        ::core::mem::align_of::<__pthread_internal_list>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_internal_list))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__pthread_internal_list>())).__prev as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_internal_list),
            "::",
            stringify!(__prev)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__pthread_internal_list>())).__next as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_internal_list),
            "::",
            stringify!(__next)
        )
    );
}
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[test]
fn bindgen_test_layout___pthread_mutex_s() {
    assert_eq!(
        ::core::mem::size_of::<__pthread_mutex_s>(),
        40usize,
        concat!("Size of: ", stringify!(__pthread_mutex_s))
    );
    assert_eq!(
        ::core::mem::align_of::<__pthread_mutex_s>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_mutex_s))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__pthread_mutex_s>())).__lock as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__lock)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__pthread_mutex_s>())).__count as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__count)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__pthread_mutex_s>())).__owner as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__owner)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__pthread_mutex_s>())).__nusers as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__nusers)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__pthread_mutex_s>())).__kind as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__kind)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__pthread_mutex_s>())).__spins as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__spins)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__pthread_mutex_s>())).__elision as *const _ as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__elision)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__pthread_mutex_s>())).__list as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__list)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
    pub __bindgen_anon_1: __pthread_cond_s__bindgen_ty_1,
    pub __bindgen_anon_2: __pthread_cond_s__bindgen_ty_2,
    pub __g_refs: [libc::c_uint; 2usize],
    pub __g_size: [libc::c_uint; 2usize],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: __pthread_cond_s__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::core::mem::size_of::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::core::mem::align_of::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>())).__low
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(__low)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>())).__high
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(__high)
        )
    );
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_1() {
    assert_eq!(
        ::core::mem::size_of::<__pthread_cond_s__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(__pthread_cond_s__bindgen_ty_1))
    );
    assert_eq!(
        ::core::mem::align_of::<__pthread_cond_s__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cond_s__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__pthread_cond_s__bindgen_ty_1>())).__wseq as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1),
            "::",
            stringify!(__wseq)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__pthread_cond_s__bindgen_ty_1>())).__wseq32 as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1),
            "::",
            stringify!(__wseq32)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_2 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: __pthread_cond_s__bindgen_ty_2__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_2__bindgen_ty_1() {
    assert_eq!(
        ::core::mem::size_of::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::core::mem::align_of::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>())).__low
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(__low)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>())).__high
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(__high)
        )
    );
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_2() {
    assert_eq!(
        ::core::mem::size_of::<__pthread_cond_s__bindgen_ty_2>(),
        8usize,
        concat!("Size of: ", stringify!(__pthread_cond_s__bindgen_ty_2))
    );
    assert_eq!(
        ::core::mem::align_of::<__pthread_cond_s__bindgen_ty_2>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cond_s__bindgen_ty_2))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__pthread_cond_s__bindgen_ty_2>())).__g1_start as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2),
            "::",
            stringify!(__g1_start)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__pthread_cond_s__bindgen_ty_2>())).__g1_start32 as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2),
            "::",
            stringify!(__g1_start32)
        )
    );
}
#[test]
fn bindgen_test_layout___pthread_cond_s() {
    assert_eq!(
        ::core::mem::size_of::<__pthread_cond_s>(),
        48usize,
        concat!("Size of: ", stringify!(__pthread_cond_s))
    );
    assert_eq!(
        ::core::mem::align_of::<__pthread_cond_s>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cond_s))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__pthread_cond_s>())).__g_refs as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g_refs)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__pthread_cond_s>())).__g_size as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__pthread_cond_s>())).__g1_orig_size as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g1_orig_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__pthread_cond_s>())).__wrefs as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__wrefs)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__pthread_cond_s>())).__g_signals as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g_signals)
        )
    );
}
pub type pthread_t = libc::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4usize],
    pub __align: libc::c_int,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_pthread_mutexattr_t() {
    assert_eq!(
        ::core::mem::size_of::<pthread_mutexattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_mutexattr_t))
    );
    assert_eq!(
        ::core::mem::align_of::<pthread_mutexattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_mutexattr_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_mutexattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutexattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_mutexattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutexattr_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4usize],
    pub __align: libc::c_int,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_pthread_condattr_t() {
    assert_eq!(
        ::core::mem::size_of::<pthread_condattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_condattr_t))
    );
    assert_eq!(
        ::core::mem::align_of::<pthread_condattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_condattr_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_condattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_condattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_condattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_condattr_t),
            "::",
            stringify!(__align)
        )
    );
}
pub type pthread_key_t = libc::c_uint;
pub type pthread_once_t = libc::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56usize],
    pub __align: libc::c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[test]
fn bindgen_test_layout_pthread_attr_t() {
    assert_eq!(
        ::core::mem::size_of::<pthread_attr_t>(),
        56usize,
        concat!("Size of: ", stringify!(pthread_attr_t))
    );
    assert_eq!(
        ::core::mem::align_of::<pthread_attr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_attr_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_attr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_attr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_attr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_attr_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40usize],
    pub __align: libc::c_long,
    _bindgen_union_align: [u64; 5usize],
}
#[test]
fn bindgen_test_layout_pthread_mutex_t() {
    assert_eq!(
        ::core::mem::size_of::<pthread_mutex_t>(),
        40usize,
        concat!("Size of: ", stringify!(pthread_mutex_t))
    );
    assert_eq!(
        ::core::mem::align_of::<pthread_mutex_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_mutex_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_mutex_t>())).__data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutex_t),
            "::",
            stringify!(__data)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_mutex_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutex_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_mutex_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutex_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48usize],
    pub __align: libc::c_longlong,
    _bindgen_union_align: [u64; 6usize],
}
#[test]
fn bindgen_test_layout_pthread_cond_t() {
    assert_eq!(
        ::core::mem::size_of::<pthread_cond_t>(),
        48usize,
        concat!("Size of: ", stringify!(pthread_cond_t))
    );
    assert_eq!(
        ::core::mem::align_of::<pthread_cond_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_cond_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_cond_t>())).__data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_cond_t),
            "::",
            stringify!(__data)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_cond_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_cond_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_cond_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_cond_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56usize],
    pub __align: libc::c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[test]
fn bindgen_test_layout_pthread_rwlock_t() {
    assert_eq!(
        ::core::mem::size_of::<pthread_rwlock_t>(),
        56usize,
        concat!("Size of: ", stringify!(pthread_rwlock_t))
    );
    assert_eq!(
        ::core::mem::align_of::<pthread_rwlock_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_rwlock_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_rwlock_t>())).__data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlock_t),
            "::",
            stringify!(__data)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_rwlock_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlock_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_rwlock_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlock_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlockattr_t {
    pub __size: [libc::c_char; 8usize],
    pub __align: libc::c_long,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_pthread_rwlockattr_t() {
    assert_eq!(
        ::core::mem::size_of::<pthread_rwlockattr_t>(),
        8usize,
        concat!("Size of: ", stringify!(pthread_rwlockattr_t))
    );
    assert_eq!(
        ::core::mem::align_of::<pthread_rwlockattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_rwlockattr_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_rwlockattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlockattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_rwlockattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlockattr_t),
            "::",
            stringify!(__align)
        )
    );
}
pub type pthread_spinlock_t = libc::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrier_t {
    pub __size: [libc::c_char; 32usize],
    pub __align: libc::c_long,
    _bindgen_union_align: [u64; 4usize],
}
#[test]
fn bindgen_test_layout_pthread_barrier_t() {
    assert_eq!(
        ::core::mem::size_of::<pthread_barrier_t>(),
        32usize,
        concat!("Size of: ", stringify!(pthread_barrier_t))
    );
    assert_eq!(
        ::core::mem::align_of::<pthread_barrier_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_barrier_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_barrier_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrier_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_barrier_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrier_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrierattr_t {
    pub __size: [libc::c_char; 4usize],
    pub __align: libc::c_int,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_pthread_barrierattr_t() {
    assert_eq!(
        ::core::mem::size_of::<pthread_barrierattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_barrierattr_t))
    );
    assert_eq!(
        ::core::mem::align_of::<pthread_barrierattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_barrierattr_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_barrierattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrierattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<pthread_barrierattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrierattr_t),
            "::",
            stringify!(__align)
        )
    );
}
/// Unique process identifier
pub type kernel_pid_t = i16;
/// @brief Describes a message object which can be sent between threads.
///
/// User can set type and one of content.ptr and content.value. (content is a union)
/// The meaning of type and the content fields is totally up to the user,
/// the corresponding fields are never read by the kernel.
///
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_t {
    /// < PID of sending thread. Will be filled in
    /// by msg_send.
    pub sender_pid: kernel_pid_t,
    /// < Type field.
    pub type_: u16,
    /// < Content of the message.
    pub content: msg_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union msg_t__bindgen_ty_1 {
    /// < Pointer content field.
    pub ptr: *mut libc::c_void,
    /// < Value content field.
    pub value: u32,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_msg_t__bindgen_ty_1() {
    assert_eq!(
        ::core::mem::size_of::<msg_t__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(msg_t__bindgen_ty_1))
    );
    assert_eq!(
        ::core::mem::align_of::<msg_t__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(msg_t__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<msg_t__bindgen_ty_1>())).ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(msg_t__bindgen_ty_1),
            "::",
            stringify!(ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<msg_t__bindgen_ty_1>())).value as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(msg_t__bindgen_ty_1),
            "::",
            stringify!(value)
        )
    );
}
#[test]
fn bindgen_test_layout_msg_t() {
    assert_eq!(
        ::core::mem::size_of::<msg_t>(),
        16usize,
        concat!("Size of: ", stringify!(msg_t))
    );
    assert_eq!(
        ::core::mem::align_of::<msg_t>(),
        8usize,
        concat!("Alignment of ", stringify!(msg_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<msg_t>())).sender_pid as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(msg_t),
            "::",
            stringify!(sender_pid)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<msg_t>())).type_ as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(msg_t),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<msg_t>())).content as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(msg_t),
            "::",
            stringify!(content)
        )
    );
}
extern "C" {
    /// @brief Send a message (blocking).
    ///
    /// This function sends a message to another thread. The ``msg_t`` structure has
    /// to be allocated (e.g. on the stack) before calling the function and can be
    /// freed afterwards. If called from an interrupt, this function will never
    /// block.
    ///
    /// @param[in] m             Pointer to preallocated ``msg_t`` structure, must
    /// not be NULL.
    /// @param[in] target_pid    PID of target thread
    ///
    /// @return 1, if sending was successful (message delivered directly or to a
    /// queue)
    /// @return 0, if called from ISR and receiver cannot receive the message now
    /// (it is not waiting or it's message queue is full)
    /// @return -1, on error (invalid PID)
    pub fn msg_send(m: *mut msg_t, target_pid: kernel_pid_t) -> libc::c_int;
}
extern "C" {
    /// @brief Send a message (non-blocking).
    ///
    /// This function sends a message to another thread. The ``msg_t`` structure has
    /// to be allocated (e.g. on the stack) before calling the function and can be
    /// freed afterwards. This function will never block.
    ///
    /// @param[in] m             Pointer to preallocated ``msg_t`` structure, must
    /// not be NULL.
    /// @param[in] target_pid    PID of target thread
    ///
    /// @return 1, if sending was successful (message delivered directly or to a
    /// queue)
    /// @return 0, if receiver is not waiting or has a full message queue
    /// @return -1, on error (invalid PID)
    pub fn msg_try_send(m: *mut msg_t, target_pid: kernel_pid_t) -> libc::c_int;
}
extern "C" {
    /// @brief Send a message to the current thread.
    /// @details Will work only if the thread has a message queue.
    ///
    /// Will be automatically chosen instead of @c msg_send
    /// if @c target_pid == @c thread_pid.
    /// This function never blocks.
    ///
    /// @param  m pointer to message structure
    ///
    /// @return 1 if sending was successful
    /// @return 0 if the thread's message queue is full (or inexistent)
    pub fn msg_send_to_self(m: *mut msg_t) -> libc::c_int;
}
extern "C" {
    /// @brief Send message from interrupt.
    ///
    /// Will be automatically chosen instead of msg_send() if called from an
    /// interrupt/ISR.
    ///
    /// The value of ``m->sender_pid`` is set to @ref KERNEL_PID_ISR.
    ///
    /// @see msg_sent_by_int()
    ///
    /// @param[in] m             Pointer to preallocated @ref msg_t structure, must
    /// not be NULL.
    /// @param[in] target_pid    PID of target thread.
    ///
    /// @return 1, if sending was successful
    /// @return 0, if receiver is not waiting and ``block == 0``
    /// @return -1, on error (invalid PID)
    pub fn msg_send_int(m: *mut msg_t, target_pid: kernel_pid_t) -> libc::c_int;
}
extern "C" {
    /// @brief Receive a message.
    ///
    /// This function blocks until a message was received.
    ///
    /// @param[out] m    Pointer to preallocated ``msg_t`` structure, must not be
    /// NULL.
    ///
    /// @return  1, Function always succeeds or blocks forever.
    pub fn msg_receive(m: *mut msg_t) -> libc::c_int;
}
extern "C" {
    /// @brief Try to receive a message.
    ///
    /// This function does not block if no message can be received.
    ///
    /// @param[out] m    Pointer to preallocated ``msg_t`` structure, must not be
    /// NULL.
    ///
    /// @return  1, if a message was received
    /// @return  -1, otherwise.
    pub fn msg_try_receive(m: *mut msg_t) -> libc::c_int;
}
extern "C" {
    /// @brief Send a message, block until reply received.
    ///
    /// This function sends a message to *target_pid* and then blocks until target
    /// has sent a reply which is then stored in *reply*.
    ///
    /// @pre     @p target_pid is not the PID of the current thread.
    ///
    /// @param[in] m             Pointer to preallocated ``msg_t`` structure with
    /// the message to send, must not be NULL.
    /// @param[out] reply        Pointer to preallocated msg. Reply will be written
    /// here, must not be NULL. Can be identical to @p m.
    /// @param[in] target_pid    The PID of the target process
    ///
    /// @return  1, if successful.
    pub fn msg_send_receive(
        m: *mut msg_t,
        reply: *mut msg_t,
        target_pid: kernel_pid_t,
    ) -> libc::c_int;
}
extern "C" {
    /// @brief Replies to a message.
    ///
    /// Sender must have sent the message with msg_send_receive().
    ///
    /// @param[in] m         message to reply to, must not be NULL.
    /// @param[out] reply    message that target will get as reply, must not be NULL.
    ///
    /// @return 1, if successful
    /// @return -1, on error
    pub fn msg_reply(m: *mut msg_t, reply: *mut msg_t) -> libc::c_int;
}
extern "C" {
    /// @brief Replies to a message from interrupt.
    ///
    /// An ISR can obviously not receive messages, however a thread might delegate
    /// replying to a message to an ISR.
    ///
    /// @param[in] m         message to reply to, must not be NULL.
    /// @param[out] reply    message that target will get as reply, must not be NULL.
    ///
    /// @return 1, if successful
    /// @return -1, on error
    pub fn msg_reply_int(m: *mut msg_t, reply: *mut msg_t) -> libc::c_int;
}
extern "C" {
    /// @brief Check how many messages are available in the message queue
    ///
    /// @return Number of messages available in our queue on success
    /// @return -1, if no caller's message queue is initialized
    pub fn msg_avail() -> libc::c_int;
}
extern "C" {
    /// @brief Initialize the current thread's message queue.
    ///
    /// @pre @p num **MUST BE A POWER OF TWO!**
    ///
    /// @param[in] array Pointer to preallocated array of ``msg_t`` structures, must
    /// not be NULL.
    /// @param[in] num   Number of ``msg_t`` structures in array.
    /// **MUST BE POWER OF TWO!**
    pub fn msg_init_queue(array: *mut msg_t, num: libc::c_int);
}
extern "C" {
    /// @brief   Prints the message queue of the current thread.
    pub fn msg_queue_print();
}
extern "C" {
    /// @brief   Returns the number of the highest '1' bit in a value
    /// @param[in]   v   Input value
    /// @return          Bit Number
    ///
    /// Source: http://graphics.stanford.edu/~seander/bithacks.html#IntegerLogObvious
    pub fn bitarithm_msb(v: libc::c_uint) -> libc::c_uint;
}
extern "C" {
    /// @brief   Returns the number of bits set in a value
    /// @param[in]   v   Input value
    /// @return          Number of set bits
    ///
    pub fn bitarithm_bits_set(v: libc::c_uint) -> libc::c_uint;
}
extern "C" {
    /// @brief Compilation with g++ may require the declaration of this function.
    ///
    /// If implementation of this function is required, it can be realized in
    /// thread_arch.c.
    pub fn sched_yield() -> libc::c_int;
}
/// @brief forward declaration for thread_t, defined in thread.h
pub type thread_t = _thread;
extern "C" {
    /// @brief   Triggers the scheduler to schedule the next thread
    /// @returns 1 if sched_active_thread/sched_active_pid was changed, 0 otherwise.
    pub fn sched_run() -> libc::c_int;
}
extern "C" {
    /// @brief   Set the status of the specified process
    ///
    /// @param[in]   process     Pointer to the thread control block of the
    /// targeted process
    /// @param[in]   status      The new status of this thread
    pub fn sched_set_status(process: *mut thread_t, status: libc::c_uint);
}
extern "C" {
    /// @brief       Yield if approriate.
    ///
    /// @details     Either yield if other_prio is higher than the current priority,
    /// or if the current thread is not on the runqueue.
    ///
    /// Depending on whether the current execution is in an ISR (irq_is_in()),
    /// thread_yield_higher() is called or @ref sched_context_switch_request is set,
    /// respectively.
    ///
    /// @param[in]   other_prio      The priority of the target thread.
    pub fn sched_switch(other_prio: u16);
}
extern "C" {
    /// @brief   Call context switching at thread exit
    pub fn cpu_switch_context_exit();
}
extern "C" {
    #[link_name = "\u{1}sched_context_switch_request"]
    pub static mut sched_context_switch_request: libc::c_uint;
}
extern "C" {
    #[link_name = "\u{1}sched_threads"]
    pub static mut sched_threads: [*mut thread_t; 33usize];
}
extern "C" {
    #[link_name = "\u{1}sched_active_thread"]
    pub static mut sched_active_thread: *mut thread_t;
}
extern "C" {
    #[link_name = "\u{1}sched_num_threads"]
    pub static mut sched_num_threads: libc::c_int;
}
extern "C" {
    #[link_name = "\u{1}sched_active_pid"]
    pub static mut sched_active_pid: kernel_pid_t;
}
extern "C" {
    #[link_name = "\u{1}sched_runqueues"]
    pub static mut sched_runqueues: [clist_node_t; 16usize];
}
extern "C" {
    /// @brief  Removes thread from scheduler and set status to #STATUS_STOPPED
    pub fn sched_task_exit();
}
/// @brief Prototype for a thread entry function
pub type thread_task_func_t =
    ::core::option::Option<unsafe extern "C" fn(arg: *mut libc::c_void) -> *mut libc::c_void>;
/// @brief @c thread_t holds thread's context data.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _thread {
    /// < thread's stack pointer
    pub sp: *mut libc::c_char,
    /// < thread's status
    pub status: u8,
    /// < thread's priority
    pub priority: u8,
    /// < thread's process id
    pub pid: kernel_pid_t,
    /// < run queue entry
    pub rq_entry: clist_node_t,
}
#[test]
fn bindgen_test_layout__thread() {
    assert_eq!(
        ::core::mem::size_of::<_thread>(),
        24usize,
        concat!("Size of: ", stringify!(_thread))
    );
    assert_eq!(
        ::core::mem::align_of::<_thread>(),
        8usize,
        concat!("Alignment of ", stringify!(_thread))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<_thread>())).sp as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_thread),
            "::",
            stringify!(sp)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<_thread>())).status as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_thread),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<_thread>())).priority as *const _ as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(_thread),
            "::",
            stringify!(priority)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<_thread>())).pid as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(_thread),
            "::",
            stringify!(pid)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<_thread>())).rq_entry as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_thread),
            "::",
            stringify!(rq_entry)
        )
    );
}
extern "C" {
    /// @brief Creates a new thread.
    ///
    /// For an in-depth discussion of thread priorities, behavior and and flags,
    /// see @ref core_thread.
    ///
    /// @note Avoid assigning the same priority to two or more threads.
    /// @note Creating threads from within an ISR is currently supported, however it
    /// is considered to be a bad programming practice and we strongly
    /// discourage you from doing so.
    ///
    /// @param[out] stack    start address of the preallocated stack memory
    /// @param[in] stacksize the size of the thread's stack in bytes
    /// @param[in] priority  priority of the new thread, lower mean higher priority
    /// @param[in] flags     optional flags for the creation of the new thread
    /// @param[in] task_func pointer to the code that is executed in the new thread
    /// @param[in] arg       the argument to the function
    /// @param[in] name      a human readable descriptor for the thread
    ///
    /// @return              PID of newly created task on success
    /// @return              -EINVAL, if @p priority is greater than or equal to
    /// @ref SCHED_PRIO_LEVELS
    /// @return              -EOVERFLOW, if there are too many threads running already
    pub fn thread_create(
        stack: *mut libc::c_char,
        stacksize: libc::c_int,
        priority: libc::c_char,
        flags: libc::c_int,
        task_func: thread_task_func_t,
        arg: *mut libc::c_void,
        name: *const libc::c_char,
    ) -> kernel_pid_t;
}
extern "C" {
    /// @brief       Retreive a thread control block by PID.
    /// @details     This is a bound-checked variant of accessing `sched_threads[pid]` directly.
    /// If you know that the PID is valid, then don't use this function.
    /// @param[in]   pid   Thread to retreive.
    /// @return      `NULL` if the PID is invalid or there is no such thread.
    pub fn thread_get(pid: kernel_pid_t) -> *mut thread_t;
}
extern "C" {
    /// @brief Returns the status of a process
    ///
    /// @param[in] pid   the PID of the thread to get the status from
    ///
    /// @return          status of the thread
    /// @return          `STATUS_NOT_FOUND` if pid is unknown
    pub fn thread_getstatus(pid: kernel_pid_t) -> libc::c_int;
}
extern "C" {
    /// @brief Puts the current thread into sleep mode. Has to be woken up externally.
    pub fn thread_sleep();
}
extern "C" {
    /// @brief   Lets current thread yield.
    ///
    /// @details The current thread will resume operation immediately,
    /// if there is no other ready thread with the same or a higher priority.
    ///
    /// Differently from thread_yield_higher() the current thread will be put to the
    /// end of the thread's in its priority class.
    ///
    /// @see     thread_yield_higher()
    pub fn thread_yield();
}
extern "C" {
    /// @brief   Lets current thread yield in favor of a higher prioritized thread.
    ///
    /// @details The current thread will resume operation immediately,
    /// if there is no other ready thread with a higher priority.
    ///
    /// Differently from thread_yield() the current thread will be scheduled next
    /// in its own priority class, i.e. it stays the first thread in its
    /// priority class.
    ///
    /// @see     thread_yield()
    pub fn thread_yield_higher();
}
extern "C" {
    /// @brief Wakes up a sleeping thread.
    ///
    /// @param[in] pid   the PID of the thread to be woken up
    ///
    /// @return          `1` on success
    /// @return          `STATUS_NOT_FOUND` if pid is unknown or not sleeping
    pub fn thread_wakeup(pid: kernel_pid_t) -> libc::c_int;
}
extern "C" {
    /// @brief   Gets called upon thread creation to set CPU registers
    ///
    /// @param[in] task_func     First function to call within the thread
    /// @param[in] arg           Argument to supply to task_func
    /// @param[in] stack_start   Start address of the stack
    /// @param[in] stack_size    Stack size
    ///
    /// @return stack pointer
    pub fn thread_stack_init(
        task_func: thread_task_func_t,
        arg: *mut libc::c_void,
        stack_start: *mut libc::c_void,
        stack_size: libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C" {
    /// @brief Add thread to list, sorted by priority (internal)
    ///
    /// This will add @p thread to @p list sorted by the thread priority.
    /// It reuses the thread's rq_entry field.
    /// Used internally by msg and mutex implementations.
    ///
    /// @note Only use for threads *not on any runqueue* and with interrupts
    /// disabled.
    ///
    /// @param[in] list      ptr to list root node
    /// @param[in] thread    thread to add
    pub fn thread_add_to_list(list: *mut list_node_t, thread: *mut thread_t);
}
extern "C" {
    /// @brief Returns the name of a process
    ///
    /// @note when compiling without DEVELHELP, this *always* returns NULL!
    ///
    /// @param[in] pid   the PID of the thread to get the name from
    ///
    /// @return          the threads name
    /// @return          `NULL` if pid is unknown
    pub fn thread_getname(pid: kernel_pid_t) -> *const libc::c_char;
}
extern "C" {
    /// @brief   Get the number of bytes used on the ISR stack
    pub fn thread_isr_stack_usage() -> libc::c_int;
}
extern "C" {
    /// @brief   Get the current ISR stack pointer
    pub fn thread_isr_stack_pointer() -> *mut libc::c_void;
}
extern "C" {
    /// @brief   Get the start of the ISR stack
    pub fn thread_isr_stack_start() -> *mut libc::c_void;
}
extern "C" {
    /// @brief Print the current stack to stdout
    pub fn thread_stack_print();
}
extern "C" {
    /// @brief   Prints human readable, ps-like thread information for debugging purposes
    pub fn thread_print_stack();
}
/// @brief   Not so much protocol but data type that is passed to network
/// devices using the netdev interface
pub const gnrc_nettype_t_GNRC_NETTYPE_IOVEC: gnrc_nettype_t = -2;
/// @brief   Protocol is as defined in @ref gnrc_netif_hdr_t. Not usable with
/// @ref net_gnrc_netreg
pub const gnrc_nettype_t_GNRC_NETTYPE_NETIF: gnrc_nettype_t = -1;
/// < Protocol is undefined
pub const gnrc_nettype_t_GNRC_NETTYPE_UNDEF: gnrc_nettype_t = 0;
/// < maximum number of available protocols
pub const gnrc_nettype_t_GNRC_NETTYPE_NUMOF: gnrc_nettype_t = 1;
/// @brief   Definition of protocol types in the network stack.
///
/// @note    Expand at will.
pub type gnrc_nettype_t = i32;
pub const idtype_t_P_ALL: idtype_t = 0;
pub const idtype_t_P_PID: idtype_t = 1;
pub const idtype_t_P_PGID: idtype_t = 2;
pub type idtype_t = u32;
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct div_t {
    pub quot: libc::c_int,
    pub rem: libc::c_int,
}
#[test]
fn bindgen_test_layout_div_t() {
    assert_eq!(
        ::core::mem::size_of::<div_t>(),
        8usize,
        concat!("Size of: ", stringify!(div_t))
    );
    assert_eq!(
        ::core::mem::align_of::<div_t>(),
        4usize,
        concat!("Alignment of ", stringify!(div_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<div_t>())).quot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(div_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<div_t>())).rem as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(div_t),
            "::",
            stringify!(rem)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ldiv_t {
    pub quot: libc::c_long,
    pub rem: libc::c_long,
}
#[test]
fn bindgen_test_layout_ldiv_t() {
    assert_eq!(
        ::core::mem::size_of::<ldiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(ldiv_t))
    );
    assert_eq!(
        ::core::mem::align_of::<ldiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ldiv_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ldiv_t>())).quot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ldiv_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ldiv_t>())).rem as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ldiv_t),
            "::",
            stringify!(rem)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lldiv_t {
    pub quot: libc::c_longlong,
    pub rem: libc::c_longlong,
}
#[test]
fn bindgen_test_layout_lldiv_t() {
    assert_eq!(
        ::core::mem::size_of::<lldiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(lldiv_t))
    );
    assert_eq!(
        ::core::mem::align_of::<lldiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(lldiv_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lldiv_t>())).quot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(lldiv_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lldiv_t>())).rem as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(lldiv_t),
            "::",
            stringify!(rem)
        )
    );
}
extern "C" {
    pub fn __ctype_get_mb_cur_max() -> usize;
}
extern "C" {
    pub fn atof(__nptr: *const libc::c_char) -> f64;
}
extern "C" {
    pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn atol(__nptr: *const libc::c_char) -> libc::c_long;
}
extern "C" {
    pub fn atoll(__nptr: *const libc::c_char) -> libc::c_longlong;
}
extern "C" {
    pub fn strtod(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char) -> f64;
}
extern "C" {
    pub fn strtof(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char) -> f32;
}
extern "C" {
    pub fn strtold(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char) -> f64;
}
extern "C" {
    pub fn strtol(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_long;
}
extern "C" {
    pub fn strtoul(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_ulong;
}
extern "C" {
    pub fn strtoq(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_longlong;
}
extern "C" {
    pub fn strtouq(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_ulonglong;
}
extern "C" {
    pub fn strtoll(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_longlong;
}
extern "C" {
    pub fn strtoull(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_ulonglong;
}
extern "C" {
    pub fn l64a(__n: libc::c_long) -> *mut libc::c_char;
}
extern "C" {
    pub fn a64l(__s: *const libc::c_char) -> libc::c_long;
}
extern "C" {
    pub fn random() -> libc::c_long;
}
extern "C" {
    pub fn srandom(__seed: libc::c_uint);
}
extern "C" {
    pub fn initstate(
        __seed: libc::c_uint,
        __statebuf: *mut libc::c_char,
        __statelen: usize,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn setstate(__statebuf: *mut libc::c_char) -> *mut libc::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct random_data {
    pub fptr: *mut i32,
    pub rptr: *mut i32,
    pub state: *mut i32,
    pub rand_type: libc::c_int,
    pub rand_deg: libc::c_int,
    pub rand_sep: libc::c_int,
    pub end_ptr: *mut i32,
}
#[test]
fn bindgen_test_layout_random_data() {
    assert_eq!(
        ::core::mem::size_of::<random_data>(),
        48usize,
        concat!("Size of: ", stringify!(random_data))
    );
    assert_eq!(
        ::core::mem::align_of::<random_data>(),
        8usize,
        concat!("Alignment of ", stringify!(random_data))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<random_data>())).fptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(fptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<random_data>())).rptr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<random_data>())).state as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<random_data>())).rand_type as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rand_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<random_data>())).rand_deg as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rand_deg)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<random_data>())).rand_sep as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rand_sep)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<random_data>())).end_ptr as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(end_ptr)
        )
    );
}
extern "C" {
    pub fn random_r(__buf: *mut random_data, __result: *mut i32) -> libc::c_int;
}
extern "C" {
    pub fn srandom_r(__seed: libc::c_uint, __buf: *mut random_data) -> libc::c_int;
}
extern "C" {
    pub fn initstate_r(
        __seed: libc::c_uint,
        __statebuf: *mut libc::c_char,
        __statelen: usize,
        __buf: *mut random_data,
    ) -> libc::c_int;
}
extern "C" {
    pub fn setstate_r(__statebuf: *mut libc::c_char, __buf: *mut random_data) -> libc::c_int;
}
extern "C" {
    pub fn rand() -> libc::c_int;
}
extern "C" {
    pub fn srand(__seed: libc::c_uint);
}
extern "C" {
    pub fn rand_r(__seed: *mut libc::c_uint) -> libc::c_int;
}
extern "C" {
    pub fn drand48() -> f64;
}
extern "C" {
    pub fn erand48(__xsubi: *mut libc::c_ushort) -> f64;
}
extern "C" {
    pub fn lrand48() -> libc::c_long;
}
extern "C" {
    pub fn nrand48(__xsubi: *mut libc::c_ushort) -> libc::c_long;
}
extern "C" {
    pub fn mrand48() -> libc::c_long;
}
extern "C" {
    pub fn jrand48(__xsubi: *mut libc::c_ushort) -> libc::c_long;
}
extern "C" {
    pub fn srand48(__seedval: libc::c_long);
}
extern "C" {
    pub fn seed48(__seed16v: *mut libc::c_ushort) -> *mut libc::c_ushort;
}
extern "C" {
    pub fn lcong48(__param: *mut libc::c_ushort);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drand48_data {
    pub __x: [libc::c_ushort; 3usize],
    pub __old_x: [libc::c_ushort; 3usize],
    pub __c: libc::c_ushort,
    pub __init: libc::c_ushort,
    pub __a: libc::c_ulonglong,
}
#[test]
fn bindgen_test_layout_drand48_data() {
    assert_eq!(
        ::core::mem::size_of::<drand48_data>(),
        24usize,
        concat!("Size of: ", stringify!(drand48_data))
    );
    assert_eq!(
        ::core::mem::align_of::<drand48_data>(),
        8usize,
        concat!("Alignment of ", stringify!(drand48_data))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<drand48_data>())).__x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__x)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<drand48_data>())).__old_x as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__old_x)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<drand48_data>())).__c as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__c)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<drand48_data>())).__init as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__init)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<drand48_data>())).__a as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__a)
        )
    );
}
extern "C" {
    pub fn drand48_r(__buffer: *mut drand48_data, __result: *mut f64) -> libc::c_int;
}
extern "C" {
    pub fn erand48_r(
        __xsubi: *mut libc::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut f64,
    ) -> libc::c_int;
}
extern "C" {
    pub fn lrand48_r(__buffer: *mut drand48_data, __result: *mut libc::c_long) -> libc::c_int;
}
extern "C" {
    pub fn nrand48_r(
        __xsubi: *mut libc::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut libc::c_long,
    ) -> libc::c_int;
}
extern "C" {
    pub fn mrand48_r(__buffer: *mut drand48_data, __result: *mut libc::c_long) -> libc::c_int;
}
extern "C" {
    pub fn jrand48_r(
        __xsubi: *mut libc::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut libc::c_long,
    ) -> libc::c_int;
}
extern "C" {
    pub fn srand48_r(__seedval: libc::c_long, __buffer: *mut drand48_data) -> libc::c_int;
}
extern "C" {
    pub fn seed48_r(__seed16v: *mut libc::c_ushort, __buffer: *mut drand48_data) -> libc::c_int;
}
extern "C" {
    pub fn lcong48_r(__param: *mut libc::c_ushort, __buffer: *mut drand48_data) -> libc::c_int;
}
extern "C" {
    pub fn malloc(__size: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn calloc(__nmemb: usize, __size: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn realloc(__ptr: *mut libc::c_void, __size: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn free(__ptr: *mut libc::c_void);
}
extern "C" {
    pub fn alloca(__size: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn valloc(__size: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn posix_memalign(
        __memptr: *mut *mut libc::c_void,
        __alignment: usize,
        __size: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn aligned_alloc(__alignment: usize, __size: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn abort();
}
extern "C" {
    pub fn atexit(__func: ::core::option::Option<unsafe extern "C" fn()>) -> libc::c_int;
}
extern "C" {
    pub fn at_quick_exit(__func: ::core::option::Option<unsafe extern "C" fn()>) -> libc::c_int;
}
extern "C" {
    pub fn on_exit(
        __func: ::core::option::Option<
            unsafe extern "C" fn(__status: libc::c_int, __arg: *mut libc::c_void),
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C" {
    pub fn exit(__status: libc::c_int);
}
extern "C" {
    pub fn quick_exit(__status: libc::c_int);
}
extern "C" {
    pub fn _Exit(__status: libc::c_int);
}
extern "C" {
    pub fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn putenv(__string: *mut libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn clearenv() -> libc::c_int;
}
extern "C" {
    pub fn mktemp(__template: *mut libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn mkstemps(__template: *mut libc::c_char, __suffixlen: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn mkdtemp(__template: *mut libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn system(__command: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
}
pub type __compar_fn_t = ::core::option::Option<
    unsafe extern "C" fn(arg1: *const libc::c_void, arg2: *const libc::c_void) -> libc::c_int,
>;
extern "C" {
    pub fn bsearch(
        __key: *const libc::c_void,
        __base: *const libc::c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_fn_t,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn qsort(__base: *mut libc::c_void, __nmemb: usize, __size: usize, __compar: __compar_fn_t);
}
extern "C" {
    pub fn abs(__x: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn labs(__x: libc::c_long) -> libc::c_long;
}
extern "C" {
    pub fn llabs(__x: libc::c_longlong) -> libc::c_longlong;
}
extern "C" {
    pub fn div(__numer: libc::c_int, __denom: libc::c_int) -> div_t;
}
extern "C" {
    pub fn ldiv(__numer: libc::c_long, __denom: libc::c_long) -> ldiv_t;
}
extern "C" {
    pub fn lldiv(__numer: libc::c_longlong, __denom: libc::c_longlong) -> lldiv_t;
}
extern "C" {
    pub fn ecvt(
        __value: f64,
        __ndigit: libc::c_int,
        __decpt: *mut libc::c_int,
        __sign: *mut libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn fcvt(
        __value: f64,
        __ndigit: libc::c_int,
        __decpt: *mut libc::c_int,
        __sign: *mut libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn gcvt(__value: f64, __ndigit: libc::c_int, __buf: *mut libc::c_char)
        -> *mut libc::c_char;
}
extern "C" {
    pub fn qecvt(
        __value: f64,
        __ndigit: libc::c_int,
        __decpt: *mut libc::c_int,
        __sign: *mut libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn qfcvt(
        __value: f64,
        __ndigit: libc::c_int,
        __decpt: *mut libc::c_int,
        __sign: *mut libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn qgcvt(
        __value: f64,
        __ndigit: libc::c_int,
        __buf: *mut libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn ecvt_r(
        __value: f64,
        __ndigit: libc::c_int,
        __decpt: *mut libc::c_int,
        __sign: *mut libc::c_int,
        __buf: *mut libc::c_char,
        __len: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn fcvt_r(
        __value: f64,
        __ndigit: libc::c_int,
        __decpt: *mut libc::c_int,
        __sign: *mut libc::c_int,
        __buf: *mut libc::c_char,
        __len: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn qecvt_r(
        __value: f64,
        __ndigit: libc::c_int,
        __decpt: *mut libc::c_int,
        __sign: *mut libc::c_int,
        __buf: *mut libc::c_char,
        __len: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn qfcvt_r(
        __value: f64,
        __ndigit: libc::c_int,
        __decpt: *mut libc::c_int,
        __sign: *mut libc::c_int,
        __buf: *mut libc::c_char,
        __len: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn mblen(__s: *const libc::c_char, __n: usize) -> libc::c_int;
}
extern "C" {
    pub fn mbtowc(__pwc: *mut wchar_t, __s: *const libc::c_char, __n: usize) -> libc::c_int;
}
extern "C" {
    pub fn wctomb(__s: *mut libc::c_char, __wchar: wchar_t) -> libc::c_int;
}
extern "C" {
    pub fn mbstowcs(__pwcs: *mut wchar_t, __s: *const libc::c_char, __n: usize) -> usize;
}
extern "C" {
    pub fn wcstombs(__s: *mut libc::c_char, __pwcs: *const wchar_t, __n: usize) -> usize;
}
extern "C" {
    pub fn rpmatch(__response: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn getsubopt(
        __optionp: *mut *mut libc::c_char,
        __tokens: *const *mut libc::c_char,
        __valuep: *mut *mut libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn getloadavg(__loadavg: *mut f64, __nelem: libc::c_int) -> libc::c_int;
}
/// @brief   Type to represent parts (either headers or payload) of a packet,
/// called snips.
/// @details The idea behind the packet snips is that they either can represent
/// protocol-specific headers or payload. A packet can be comprised of
/// multiple pktsnip_t elements.
///
/// Example:
///
/// buffer
/// +---------------------------+                      +------+
/// | size = 14                 | data +-------------->|      |
/// | type = NETTYPE_ETHERNET   |------+               +------+
/// +---------------------------+                      .      .
/// | next                                       .      .
/// v                                            +------+
/// +---------------------------+         +----------->|      |
/// | size = 40                 | data    |            |      |
/// | type = NETTYPE_IPV6       |---------+            +------+
/// +---------------------------+                      .      .
/// | next                                       .      .
/// v                                            +------+
/// +---------------------------+            +-------->|      |
/// | size = 8                  | data       |         +------+
/// | type = NETTYPE_UDP        |------------+         .      .
/// +---------------------------+                      .      .
/// | next                                       +------+
/// v                                     +----->|      |
/// +---------------------------+               |      |      |
/// | size = 59                 | data          |      .      .
/// | type = NETTYPE_UNDEF      |---------------+      .      .
/// +---------------------------+                      .      .
///
/// To keep data duplication as low as possible the order of the snips
/// in a packet will be reversed depending on if you send the packet or if
/// you received it. For sending the order is from (in the network stack) lowest
/// protocol snip to the highest, for receiving the order is from highest
/// snip to the lowest. This way, if a layer needs to duplicate the packet
/// a tree is created rather than a duplication of the whole package.
///
/// A very extreme example for this (we only expect one or two duplications at
/// maximum per package) can be seen here:
///
/// Sending                          Receiving
/// =======                          =========
///
/// * Payload                        * L2 header
/// ^                                ^
/// |                                |
/// |\                               |\
/// | * L4 header 1                  | * L2.5 header 1
/// | * L3 header 1                  | * L3 header 1
/// | * netif header 1               | * L4 header 1
/// * L4 header 2                    | * Payload 1
/// ^                                * L3 header 2
/// |                                ^
/// |\                               |
/// | * L3 header 2                  |\
/// | * L2 header 2                  | * L4 header 2
/// * L2 header 3                    | * Payload 2
/// |\                               * Payload 3
/// | * L2 header 3
/// * L2 header 4
///
/// The first three fields (next, data, size) match iolist_t (named iol_next,
/// iol_base and iol_len there).  That means that any pktsnip can be casted to
/// iolist_t for direct passing to e.g., netdev send() functions.
///
/// @note    This type has no initializer on purpose. Please use @ref net_gnrc_pktbuf
/// as factory.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gnrc_pktsnip {
    /// < next snip in the packet
    pub next: *mut gnrc_pktsnip,
    /// < pointer to the data of the snip
    pub data: *mut libc::c_void,
    /// < the length of the snip in byte
    pub size: usize,
    /// @brief   Counter of threads currently having control over this packet.
    ///
    /// @internal
    pub users: libc::c_uint,
    /// < protocol of the packet snip
    pub type_: gnrc_nettype_t,
}
#[test]
fn bindgen_test_layout_gnrc_pktsnip() {
    assert_eq!(
        ::core::mem::size_of::<gnrc_pktsnip>(),
        32usize,
        concat!("Size of: ", stringify!(gnrc_pktsnip))
    );
    assert_eq!(
        ::core::mem::align_of::<gnrc_pktsnip>(),
        8usize,
        concat!("Alignment of ", stringify!(gnrc_pktsnip))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_pktsnip>())).next as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_pktsnip),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_pktsnip>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_pktsnip),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_pktsnip>())).size as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_pktsnip),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_pktsnip>())).users as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_pktsnip),
            "::",
            stringify!(users)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_pktsnip>())).type_ as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_pktsnip),
            "::",
            stringify!(type_)
        )
    );
}
pub type gnrc_pktsnip_t = gnrc_pktsnip;
extern "C" {
    /// @brief   Searches the packet for a packet snip of a specific type
    ///
    /// @param[in] pkt   list of packet snips
    /// @param[in] type  the type to search for
    ///
    /// @return  the packet snip in @p pkt with @ref gnrc_nettype_t @p type
    /// @return  NULL, if none of the snips in @p pkt is of @p type
    pub fn gnrc_pktsnip_search_type(
        pkt: *mut gnrc_pktsnip_t,
        type_: gnrc_nettype_t,
    ) -> *mut gnrc_pktsnip_t;
}
/// @brief   Data structure to be send for setting (@ref GNRC_NETAPI_MSG_TYPE_SET)
/// and getting (@ref GNRC_NETAPI_MSG_TYPE_GET) options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gnrc_netapi_opt_t {
    /// < the option to get/set
    pub opt: netopt_t,
    /// < (optional) context for that option
    pub context: u16,
    /// < data to set or buffer to read into
    pub data: *mut libc::c_void,
    /// < size of the data / the buffer
    pub data_len: u16,
}
#[test]
fn bindgen_test_layout_gnrc_netapi_opt_t() {
    assert_eq!(
        ::core::mem::size_of::<gnrc_netapi_opt_t>(),
        24usize,
        concat!("Size of: ", stringify!(gnrc_netapi_opt_t))
    );
    assert_eq!(
        ::core::mem::align_of::<gnrc_netapi_opt_t>(),
        8usize,
        concat!("Alignment of ", stringify!(gnrc_netapi_opt_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netapi_opt_t>())).opt as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netapi_opt_t),
            "::",
            stringify!(opt)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netapi_opt_t>())).context as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netapi_opt_t),
            "::",
            stringify!(context)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netapi_opt_t>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netapi_opt_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netapi_opt_t>())).data_len as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netapi_opt_t),
            "::",
            stringify!(data_len)
        )
    );
}
extern "C" {
    /// @brief   Shortcut function for sending @ref GNRC_NETAPI_MSG_TYPE_SND messages
    ///
    /// @param[in] pid       PID of the targeted network module
    /// @param[in] pkt       pointer into the packet buffer holding the data to send
    ///
    /// @return              1 if packet was successfully delivered
    /// @return              -1 on error (invalid PID or no space in queue)
    pub fn gnrc_netapi_send(pid: kernel_pid_t, pkt: *mut gnrc_pktsnip_t) -> libc::c_int;
}
extern "C" {
    /// @brief   Sends @p cmd to all subscribers to (@p type, @p demux_ctx).
    ///
    /// @param[in] type      protocol type of the targeted network module.
    /// @param[in] demux_ctx demultiplexing context for @p type.
    /// @param[in] cmd       command for all subscribers
    /// @param[in] pkt       pointer into the packet buffer holding the data to send
    ///
    /// @return Number of subscribers to (@p type, @p demux_ctx).
    pub fn gnrc_netapi_dispatch(
        type_: gnrc_nettype_t,
        demux_ctx: u32,
        cmd: u16,
        pkt: *mut gnrc_pktsnip_t,
    ) -> libc::c_int;
}
extern "C" {
    /// @brief   Shortcut function for sending @ref GNRC_NETAPI_MSG_TYPE_RCV messages
    ///
    /// @param[in] pid       PID of the targeted network module
    /// @param[in] pkt       pointer into the packet buffer holding the received data
    ///
    /// @return              1 if packet was successfully delivered
    /// @return              -1 on error (invalid PID or no space in queue)
    pub fn gnrc_netapi_receive(pid: kernel_pid_t, pkt: *mut gnrc_pktsnip_t) -> libc::c_int;
}
extern "C" {
    /// @brief   Shortcut function for sending @ref GNRC_NETAPI_MSG_TYPE_GET messages and
    /// parsing the returned @ref GNRC_NETAPI_MSG_TYPE_ACK message
    ///
    /// @param[in] pid       PID of the targeted network module
    /// @param[in] opt       option to get
    /// @param[in] context   (optional) context to the given option
    /// @param[in] data      pointer to buffer for reading the option's value
    /// @param[in] max_len   maximum number of bytes that fit into @p data
    ///
    /// @return              value returned by the @ref GNRC_NETAPI_MSG_TYPE_ACK message i.e. the actual
    /// length of the resulting data on success, a negative errno on error. The
    /// actual error value is for the implementation to decide but should be
    /// sensible to indicate what went wrong.
    pub fn gnrc_netapi_get(
        pid: kernel_pid_t,
        opt: netopt_t,
        context: u16,
        data: *mut libc::c_void,
        max_len: usize,
    ) -> libc::c_int;
}
extern "C" {
    /// @brief   Shortcut function for sending @ref GNRC_NETAPI_MSG_TYPE_SET messages and
    /// parsing the returned @ref GNRC_NETAPI_MSG_TYPE_ACK message
    ///
    /// @param[in] pid       PID of the targeted network module
    /// @param[in] opt       option to set
    /// @param[in] context   (optional) context to the given option
    /// @param[in] data      data to set the given option to
    /// @param[in] data_len  size of @p data
    ///
    /// @return              value returned by the @ref GNRC_NETAPI_MSG_TYPE_ACK message i.e. 0 on
    /// success, a negative errno on error. The actual error value is for the
    /// implementation to decide but should be sensible to indicate what went
    /// wrong.
    pub fn gnrc_netapi_set(
        pid: kernel_pid_t,
        opt: netopt_t,
        context: u16,
        data: *mut libc::c_void,
        data_len: usize,
    ) -> libc::c_int;
}
/// @brief   Entry to the @ref net_gnrc_netreg
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnrc_netreg_entry {
    /// @brief next element in list
    ///
    /// @internal
    pub next: *mut gnrc_netreg_entry,
    /// @brief   The demultiplexing context for the registering thread.
    ///
    /// @details This can be defined by the network protocol themselves.
    /// E. g. protocol numbers / next header numbers in IPv4/IPv6,
    /// ports in UDP/TCP, or similar.
    pub demux_ctx: u32,
    /// < Target for the registry entry
    pub target: gnrc_netreg_entry__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union gnrc_netreg_entry__bindgen_ty_1 {
    /// < The PID of the registering thread
    pub pid: kernel_pid_t,
    _bindgen_union_align: u16,
}
#[test]
fn bindgen_test_layout_gnrc_netreg_entry__bindgen_ty_1() {
    assert_eq!(
        ::core::mem::size_of::<gnrc_netreg_entry__bindgen_ty_1>(),
        2usize,
        concat!("Size of: ", stringify!(gnrc_netreg_entry__bindgen_ty_1))
    );
    assert_eq!(
        ::core::mem::align_of::<gnrc_netreg_entry__bindgen_ty_1>(),
        2usize,
        concat!("Alignment of ", stringify!(gnrc_netreg_entry__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<gnrc_netreg_entry__bindgen_ty_1>())).pid as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netreg_entry__bindgen_ty_1),
            "::",
            stringify!(pid)
        )
    );
}
#[test]
fn bindgen_test_layout_gnrc_netreg_entry() {
    assert_eq!(
        ::core::mem::size_of::<gnrc_netreg_entry>(),
        16usize,
        concat!("Size of: ", stringify!(gnrc_netreg_entry))
    );
    assert_eq!(
        ::core::mem::align_of::<gnrc_netreg_entry>(),
        8usize,
        concat!("Alignment of ", stringify!(gnrc_netreg_entry))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netreg_entry>())).next as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netreg_entry),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netreg_entry>())).demux_ctx as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netreg_entry),
            "::",
            stringify!(demux_ctx)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netreg_entry>())).target as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netreg_entry),
            "::",
            stringify!(target)
        )
    );
}
pub type gnrc_netreg_entry_t = gnrc_netreg_entry;
extern "C" {
    /// @brief   Initializes module.
    pub fn gnrc_netreg_init();
}
extern "C" {
    /// @brief   Registers a thread to the registry.
    ///
    /// @details The semantics are: Thread gnrc_netreg_entry_t::pid is interested in
    /// packets of protocol @p type with context gnrc_netreg_entry_t::demux_ctx.
    ///
    /// @param[in] type      Type of the protocol. Must not be < GNRC_NETTYPE_UNDEF or
    /// >= GNRC_NETTYPE_NUMOF.
    /// @param[in] entry     An entry you want to add to the registry. This needs to
    /// be initialized before hand using the @ref
    /// net_gnrc_netreg_init_static "static" or @ref
    /// net_gnrc_netreg_init_dyn "dynamic" initialization
    /// helpers.
    ///
    /// @warning Call gnrc_netreg_unregister() *before* you leave the context you
    /// allocated @p entry in. Otherwise it might get overwritten.
    ///
    /// @pre The calling thread must provide a [message queue](@ref msg_init_queue)
    /// when using @ref GNRC_NETREG_TYPE_DEFAULT for gnrc_netreg_entry_t::type
    /// of @p entry.
    ///
    /// @return  0 on success
    /// @return  -EINVAL if @p type was < GNRC_NETTYPE_UNDEF or >= GNRC_NETTYPE_NUMOF
    pub fn gnrc_netreg_register(
        type_: gnrc_nettype_t,
        entry: *mut gnrc_netreg_entry_t,
    ) -> libc::c_int;
}
extern "C" {
    /// @brief   Removes a thread from the registry.
    ///
    /// @param[in] type      Type of the protocol.
    /// @param[in] entry     An entry you want to remove from the registry.
    pub fn gnrc_netreg_unregister(type_: gnrc_nettype_t, entry: *mut gnrc_netreg_entry_t);
}
extern "C" {
    /// @brief   Searches for entries with given parameters in the registry and
    /// returns the first found.
    ///
    /// @param[in] type      Type of the protocol.
    /// @param[in] demux_ctx The demultiplexing context for the registered thread.
    /// See gnrc_netreg_entry_t::demux_ctx.
    ///
    /// @return  The first entry fitting the given parameters on success
    /// @return  NULL if no entry can be found.
    pub fn gnrc_netreg_lookup(type_: gnrc_nettype_t, demux_ctx: u32) -> *mut gnrc_netreg_entry_t;
}
extern "C" {
    /// @brief   Returns number of entries with the same gnrc_netreg_entry_t::type and
    /// gnrc_netreg_entry_t::demux_ctx.
    ///
    /// @param[in] type      Type of the protocol.
    /// @param[in] demux_ctx The demultiplexing context for the registered thread.
    /// See gnrc_netreg_entry_t::demux_ctx.
    ///
    /// @return  Number of entries with the same gnrc_netreg_entry_t::type and
    /// gnrc_netreg_entry_t::demux_ctx as the given parameters.
    pub fn gnrc_netreg_num(type_: gnrc_nettype_t, demux_ctx: u32) -> libc::c_int;
}
extern "C" {
    /// @brief   Returns the next entry after @p entry with the same
    /// gnrc_netreg_entry_t::type and gnrc_netreg_entry_t::demux_ctx as the
    /// given entry.
    ///
    /// @param[in] entry     A registry entry retrieved by gnrc_netreg_lookup() or
    /// gnrc_netreg_getnext(). Must not be NULL.
    ///
    /// @return  The next entry after @p entry fitting the given parameters on success
    /// @return  NULL if no entry new entry can be found.
    pub fn gnrc_netreg_getnext(entry: *mut gnrc_netreg_entry_t) -> *mut gnrc_netreg_entry_t;
}
extern "C" {
    /// @brief   Calculates the checksum for a header.
    ///
    /// @param[in] hdr           The header the checksum should be calculated
    /// for.
    /// @param[in] pseudo_hdr    The header the pseudo header shall be generated
    /// from. NULL if none is needed.
    ///
    /// @return  0, on success.
    /// @return  -EINVAL, if @p pseudo_hdr is NULL but a pseudo header was required.
    /// @return  -ENOENT, if @ref net_gnrc_netreg does not know how to calculate checksum
    /// for gnrc_pktsnip_t::type of @p hdr.
    pub fn gnrc_netreg_calc_csum(
        hdr: *mut gnrc_pktsnip_t,
        pseudo_hdr: *mut gnrc_pktsnip_t,
    ) -> libc::c_int;
}
extern "C" {
    pub fn memcpy(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: usize,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn memmove(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: usize,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn memccpy(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __c: libc::c_int,
        __n: usize,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn memset(__s: *mut libc::c_void, __c: libc::c_int, __n: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn memcmp(__s1: *const libc::c_void, __s2: *const libc::c_void, __n: usize) -> libc::c_int;
}
extern "C" {
    pub fn memchr(__s: *const libc::c_void, __c: libc::c_int, __n: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn strcpy(__dest: *mut libc::c_char, __src: *const libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn strncpy(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: usize,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn strcat(__dest: *mut libc::c_char, __src: *const libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn strncat(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: usize,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn strcmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn strncmp(__s1: *const libc::c_char, __s2: *const libc::c_char, __n: usize)
        -> libc::c_int;
}
extern "C" {
    pub fn strcoll(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn strxfrm(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: usize,
    ) -> libc::c_ulong;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13usize],
    pub __ctype_b: *const libc::c_ushort,
    pub __ctype_tolower: *const libc::c_int,
    pub __ctype_toupper: *const libc::c_int,
    pub __names: [*const libc::c_char; 13usize],
}
#[test]
fn bindgen_test_layout___locale_struct() {
    assert_eq!(
        ::core::mem::size_of::<__locale_struct>(),
        232usize,
        concat!("Size of: ", stringify!(__locale_struct))
    );
    assert_eq!(
        ::core::mem::align_of::<__locale_struct>(),
        8usize,
        concat!("Alignment of ", stringify!(__locale_struct))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__locale_struct>())).__locales as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__locales)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__locale_struct>())).__ctype_b as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__ctype_b)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__locale_struct>())).__ctype_tolower as *const _ as usize
        },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__ctype_tolower)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<__locale_struct>())).__ctype_toupper as *const _ as usize
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__ctype_toupper)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__locale_struct>())).__names as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__names)
        )
    );
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
extern "C" {
    pub fn strcoll_l(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __l: locale_t,
    ) -> libc::c_int;
}
extern "C" {
    pub fn strxfrm_l(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: usize,
        __l: locale_t,
    ) -> usize;
}
extern "C" {
    pub fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn strndup(__string: *const libc::c_char, __n: usize) -> *mut libc::c_char;
}
extern "C" {
    pub fn strchr(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
}
extern "C" {
    pub fn strrchr(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
}
extern "C" {
    pub fn strcspn(__s: *const libc::c_char, __reject: *const libc::c_char) -> libc::c_ulong;
}
extern "C" {
    pub fn strspn(__s: *const libc::c_char, __accept: *const libc::c_char) -> libc::c_ulong;
}
extern "C" {
    pub fn strpbrk(__s: *const libc::c_char, __accept: *const libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn strstr(
        __haystack: *const libc::c_char,
        __needle: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn strtok(__s: *mut libc::c_char, __delim: *const libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn __strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn strlen(__s: *const libc::c_char) -> libc::c_ulong;
}
extern "C" {
    pub fn strnlen(__string: *const libc::c_char, __maxlen: usize) -> usize;
}
extern "C" {
    pub fn strerror(__errnum: libc::c_int) -> *mut libc::c_char;
}
extern "C" {
    #[link_name = "\u{1}__xpg_strerror_r"]
    pub fn strerror_r(
        __errnum: libc::c_int,
        __buf: *mut libc::c_char,
        __buflen: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn strerror_l(__errnum: libc::c_int, __l: locale_t) -> *mut libc::c_char;
}
extern "C" {
    pub fn bcmp(__s1: *const libc::c_void, __s2: *const libc::c_void, __n: usize) -> libc::c_int;
}
extern "C" {
    pub fn bcopy(__src: *const libc::c_void, __dest: *mut libc::c_void, __n: usize);
}
extern "C" {
    pub fn bzero(__s: *mut libc::c_void, __n: usize);
}
extern "C" {
    pub fn index(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
}
extern "C" {
    pub fn rindex(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
}
extern "C" {
    pub fn ffs(__i: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn ffsl(__l: libc::c_long) -> libc::c_int;
}
extern "C" {
    pub fn ffsll(__ll: libc::c_longlong) -> libc::c_int;
}
extern "C" {
    pub fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn strncasecmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn strcasecmp_l(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __loc: locale_t,
    ) -> libc::c_int;
}
extern "C" {
    pub fn strncasecmp_l(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: usize,
        __loc: locale_t,
    ) -> libc::c_int;
}
extern "C" {
    pub fn explicit_bzero(__s: *mut libc::c_void, __n: usize);
}
extern "C" {
    pub fn strsep(
        __stringp: *mut *mut libc::c_char,
        __delim: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn strsignal(__sig: libc::c_int) -> *mut libc::c_char;
}
extern "C" {
    pub fn __stpcpy(__dest: *mut libc::c_char, __src: *const libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn stpcpy(__dest: *mut libc::c_char, __src: *const libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn __stpncpy(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: usize,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn stpncpy(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: usize,
    ) -> *mut libc::c_char;
}
/// @brief          A 16 bit integer in little endian.
/// @details        This is a wrapper around an uint16_t to catch missing conversions
/// between different byte orders at compile time.
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union le_uint16_t {
    /// < 16 bit representation
    pub u16: u16,
    /// < 8 bit representation
    pub u8: [u8; 2usize],
    _bindgen_union_align: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_le_uint16_t() {
    assert_eq!(
        ::core::mem::size_of::<le_uint16_t>(),
        2usize,
        concat!("Size of: ", stringify!(le_uint16_t))
    );
    assert_eq!(
        ::core::mem::align_of::<le_uint16_t>(),
        1usize,
        concat!("Alignment of ", stringify!(le_uint16_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<le_uint16_t>())).u16 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(le_uint16_t),
            "::",
            stringify!(u16)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<le_uint16_t>())).u8 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(le_uint16_t),
            "::",
            stringify!(u8)
        )
    );
}
/// @brief          A 32 bit integer in little endian.
/// @details        This is a wrapper around an uint32_t to catch missing conversions
/// between different byte orders at compile time.
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union le_uint32_t {
    /// < 32 bit representation
    pub u32: u32,
    /// < 8 bit representation
    pub u8: [u8; 4usize],
    /// < 16 bit representation
    pub u16: [u16; 2usize],
    /// < little endian 16 bit representation
    pub l16: [le_uint16_t; 2usize],
    _bindgen_union_align: [u8; 4usize],
}
#[test]
fn bindgen_test_layout_le_uint32_t() {
    assert_eq!(
        ::core::mem::size_of::<le_uint32_t>(),
        4usize,
        concat!("Size of: ", stringify!(le_uint32_t))
    );
    assert_eq!(
        ::core::mem::align_of::<le_uint32_t>(),
        1usize,
        concat!("Alignment of ", stringify!(le_uint32_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<le_uint32_t>())).u32 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(le_uint32_t),
            "::",
            stringify!(u32)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<le_uint32_t>())).u8 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(le_uint32_t),
            "::",
            stringify!(u8)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<le_uint32_t>())).u16 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(le_uint32_t),
            "::",
            stringify!(u16)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<le_uint32_t>())).l16 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(le_uint32_t),
            "::",
            stringify!(l16)
        )
    );
}
/// @brief          A 64 bit integer in little endian.
/// @details        This is a wrapper around an uint64_t to catch missing conversions
/// between different byte orders at compile time.
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union le_uint64_t {
    /// < 64 bit representation
    pub u64: u64,
    /// < 8 bit representation
    pub u8: [u8; 8usize],
    /// < 16 bit representation
    pub u16: [u16; 4usize],
    /// < 32 bit representation
    pub u32: [u32; 2usize],
    /// < little endian 16 bit representation
    pub l16: [le_uint16_t; 4usize],
    /// < little endian 32 bit representation
    pub l32: [le_uint32_t; 2usize],
    _bindgen_union_align: [u8; 8usize],
}
#[test]
fn bindgen_test_layout_le_uint64_t() {
    assert_eq!(
        ::core::mem::size_of::<le_uint64_t>(),
        8usize,
        concat!("Size of: ", stringify!(le_uint64_t))
    );
    assert_eq!(
        ::core::mem::align_of::<le_uint64_t>(),
        1usize,
        concat!("Alignment of ", stringify!(le_uint64_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<le_uint64_t>())).u64 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(le_uint64_t),
            "::",
            stringify!(u64)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<le_uint64_t>())).u8 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(le_uint64_t),
            "::",
            stringify!(u8)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<le_uint64_t>())).u16 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(le_uint64_t),
            "::",
            stringify!(u16)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<le_uint64_t>())).u32 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(le_uint64_t),
            "::",
            stringify!(u32)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<le_uint64_t>())).l16 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(le_uint64_t),
            "::",
            stringify!(l16)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<le_uint64_t>())).l32 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(le_uint64_t),
            "::",
            stringify!(l32)
        )
    );
}
/// @brief          A 16 bit integer in big endian aka network byte order.
/// @details        This is a wrapper around an uint16_t to catch missing conversions
/// between different byte orders at compile time.
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union be_uint16_t {
    /// < 16 bit representation
    pub u16: u16,
    /// < 8 bit representation
    pub u8: [u8; 2usize],
    _bindgen_union_align: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_be_uint16_t() {
    assert_eq!(
        ::core::mem::size_of::<be_uint16_t>(),
        2usize,
        concat!("Size of: ", stringify!(be_uint16_t))
    );
    assert_eq!(
        ::core::mem::align_of::<be_uint16_t>(),
        1usize,
        concat!("Alignment of ", stringify!(be_uint16_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<be_uint16_t>())).u16 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(be_uint16_t),
            "::",
            stringify!(u16)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<be_uint16_t>())).u8 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(be_uint16_t),
            "::",
            stringify!(u8)
        )
    );
}
/// @brief          A 32 bit integer in big endian aka network byte order.
/// @details        This is a wrapper around an uint32_t to catch missing conversions
/// between different byte orders at compile time.
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union be_uint32_t {
    /// < 32 bit representation
    pub u32: u32,
    /// < 8 bit representation
    pub u8: [u8; 4usize],
    /// < 16 bit representation
    pub u16: [u16; 2usize],
    /// < big endian 16 bit representation
    pub b16: [be_uint16_t; 2usize],
    _bindgen_union_align: [u8; 4usize],
}
#[test]
fn bindgen_test_layout_be_uint32_t() {
    assert_eq!(
        ::core::mem::size_of::<be_uint32_t>(),
        4usize,
        concat!("Size of: ", stringify!(be_uint32_t))
    );
    assert_eq!(
        ::core::mem::align_of::<be_uint32_t>(),
        1usize,
        concat!("Alignment of ", stringify!(be_uint32_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<be_uint32_t>())).u32 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(be_uint32_t),
            "::",
            stringify!(u32)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<be_uint32_t>())).u8 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(be_uint32_t),
            "::",
            stringify!(u8)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<be_uint32_t>())).u16 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(be_uint32_t),
            "::",
            stringify!(u16)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<be_uint32_t>())).b16 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(be_uint32_t),
            "::",
            stringify!(b16)
        )
    );
}
/// @brief          A 64 bit integer in big endian aka network byte order.
/// @details        This is a wrapper around an uint64_t to catch missing conversions
/// between different byte orders at compile time.
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union be_uint64_t {
    /// < 64 bit representation
    pub u64: u64,
    /// < 8 bit representation
    pub u8: [u8; 8usize],
    /// < 16 bit representation
    pub u16: [u16; 4usize],
    /// < 32 bit representation
    pub u32: [u32; 2usize],
    /// < big endian 16 bit representation
    pub b16: [be_uint16_t; 4usize],
    /// < big endian 32 bit representation
    pub b32: [be_uint32_t; 2usize],
    _bindgen_union_align: [u8; 8usize],
}
#[test]
fn bindgen_test_layout_be_uint64_t() {
    assert_eq!(
        ::core::mem::size_of::<be_uint64_t>(),
        8usize,
        concat!("Size of: ", stringify!(be_uint64_t))
    );
    assert_eq!(
        ::core::mem::align_of::<be_uint64_t>(),
        1usize,
        concat!("Alignment of ", stringify!(be_uint64_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<be_uint64_t>())).u64 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(be_uint64_t),
            "::",
            stringify!(u64)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<be_uint64_t>())).u8 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(be_uint64_t),
            "::",
            stringify!(u8)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<be_uint64_t>())).u16 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(be_uint64_t),
            "::",
            stringify!(u16)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<be_uint64_t>())).u32 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(be_uint64_t),
            "::",
            stringify!(u32)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<be_uint64_t>())).b16 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(be_uint64_t),
            "::",
            stringify!(b16)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<be_uint64_t>())).b32 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(be_uint64_t),
            "::",
            stringify!(b32)
        )
    );
}
/// @brief A 16 bit integer in network byte order.
pub type network_uint16_t = be_uint16_t;
/// @brief A 32 bit integer in network byte order.
pub type network_uint32_t = be_uint32_t;
/// @brief A 64 bit integer in network byte order.
pub type network_uint64_t = be_uint64_t;
/// @brief Data type to represent an IPv4 address.
#[repr(C)]
#[derive(Copy, Clone)]
pub union ipv4_addr_t {
    /// < as 4 8-bit unsigned integer
    pub u8: [u8; 4usize],
    /// < as 32-bit unsigned integer
    pub u32: network_uint32_t,
    _bindgen_union_align: [u8; 4usize],
}
#[test]
fn bindgen_test_layout_ipv4_addr_t() {
    assert_eq!(
        ::core::mem::size_of::<ipv4_addr_t>(),
        4usize,
        concat!("Size of: ", stringify!(ipv4_addr_t))
    );
    assert_eq!(
        ::core::mem::align_of::<ipv4_addr_t>(),
        1usize,
        concat!("Alignment of ", stringify!(ipv4_addr_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ipv4_addr_t>())).u8 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ipv4_addr_t),
            "::",
            stringify!(u8)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ipv4_addr_t>())).u32 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ipv4_addr_t),
            "::",
            stringify!(u32)
        )
    );
}
extern "C" {
    /// @brief   Converts an IPv4 address to its string representation
    ///
    /// @param[out] result       The resulting string representation of at least
    /// @ref IPV4_ADDR_MAX_STR_LEN.
    /// @param[in] addr          An IPv4 address
    /// @param[in] result_len    Length of @p result
    ///
    /// @return  @p result, on success
    /// @return  NULL, if @p result_len was lesser than IPV4_ADDR_MAX_STR_LEN
    /// @return  NULL, if @p result or @p addr was NULL
    pub fn ipv4_addr_to_str(
        result: *mut libc::c_char,
        addr: *const ipv4_addr_t,
        result_len: u8,
    ) -> *mut libc::c_char;
}
extern "C" {
    /// @brief   Converts an IPv4 address string representation to a byte-represented
    /// IPv4 address
    ///
    /// @param[in] result    The resulting byte representation
    /// @param[in] addr      An IPv4 address string representation
    ///
    /// @return  @p result, on success
    /// @return  NULL, if @p addr was malformed
    /// @return  NULL, if @p result or @p addr was NULL
    pub fn ipv4_addr_from_str(
        result: *mut ipv4_addr_t,
        addr: *const libc::c_char,
    ) -> *mut ipv4_addr_t;
}
/// @brief Data type to represent an IPv6 address.
#[repr(C)]
#[derive(Copy, Clone)]
pub union ipv6_addr_t {
    /// < divided by 16 8-bit words.
    pub u8: [u8; 16usize],
    /// < divided by 8 16-bit words.
    pub u16: [network_uint16_t; 8usize],
    /// < divided by 4 32-bit words.
    pub u32: [network_uint32_t; 4usize],
    /// < divided by 2 64-bit words.
    pub u64: [network_uint64_t; 2usize],
    _bindgen_union_align: [u8; 16usize],
}
#[test]
fn bindgen_test_layout_ipv6_addr_t() {
    assert_eq!(
        ::core::mem::size_of::<ipv6_addr_t>(),
        16usize,
        concat!("Size of: ", stringify!(ipv6_addr_t))
    );
    assert_eq!(
        ::core::mem::align_of::<ipv6_addr_t>(),
        1usize,
        concat!("Alignment of ", stringify!(ipv6_addr_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ipv6_addr_t>())).u8 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ipv6_addr_t),
            "::",
            stringify!(u8)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ipv6_addr_t>())).u16 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ipv6_addr_t),
            "::",
            stringify!(u16)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ipv6_addr_t>())).u32 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ipv6_addr_t),
            "::",
            stringify!(u32)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ipv6_addr_t>())).u64 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ipv6_addr_t),
            "::",
            stringify!(u64)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}ipv6_addr_unspecified"]
    pub static mut ipv6_addr_unspecified: ipv6_addr_t;
}
extern "C" {
    #[link_name = "\u{1}ipv6_addr_loopback"]
    pub static mut ipv6_addr_loopback: ipv6_addr_t;
}
extern "C" {
    #[link_name = "\u{1}ipv6_addr_link_local_prefix"]
    pub static mut ipv6_addr_link_local_prefix: ipv6_addr_t;
}
extern "C" {
    #[link_name = "\u{1}ipv6_addr_all_nodes_if_local"]
    pub static mut ipv6_addr_all_nodes_if_local: ipv6_addr_t;
}
extern "C" {
    #[link_name = "\u{1}ipv6_addr_all_nodes_link_local"]
    pub static mut ipv6_addr_all_nodes_link_local: ipv6_addr_t;
}
extern "C" {
    #[link_name = "\u{1}ipv6_addr_all_routers_if_local"]
    pub static mut ipv6_addr_all_routers_if_local: ipv6_addr_t;
}
extern "C" {
    #[link_name = "\u{1}ipv6_addr_all_routers_link_local"]
    pub static mut ipv6_addr_all_routers_link_local: ipv6_addr_t;
}
extern "C" {
    #[link_name = "\u{1}ipv6_addr_all_routers_site_local"]
    pub static mut ipv6_addr_all_routers_site_local: ipv6_addr_t;
}
extern "C" {
    #[link_name = "\u{1}ipv6_addr_solicited_node_prefix"]
    pub static mut ipv6_addr_solicited_node_prefix: ipv6_addr_t;
}
extern "C" {
    /// @brief   Checks if two IPv6 addresses are equal.
    ///
    /// @param[in] a     An IPv6 address.
    /// @param[in] b     Another IPv6 address.
    ///
    /// @return  true, if @p a and @p b are equal
    /// @return  false, otherwise.
    pub fn ipv6_addr_equal(a: *const ipv6_addr_t, b: *const ipv6_addr_t) -> bool;
}
extern "C" {
    /// @brief   Checks up to which bit-count two IPv6 addresses match in their
    /// prefix.
    ///
    /// @param[in] a An IPv6 address.
    /// @param[in] b Another IPv6 address.
    ///
    /// @return  The number of bits @p a and @p b match in their prefix
    pub fn ipv6_addr_match_prefix(a: *const ipv6_addr_t, b: *const ipv6_addr_t) -> u8;
}
extern "C" {
    /// @brief   Sets IPv6 address @p out with the first @p bits taken
    /// from @p prefix and leaves the remaining bits untouched.
    ///
    /// @param[out]  out     Prefix to be set.
    /// @param[in]   prefix  Address to take prefix from.
    /// @param[in]   bits    Bits to be copied from @p prefix to @p out
    /// (set to 128 when greater than 128).
    pub fn ipv6_addr_init_prefix(out: *mut ipv6_addr_t, prefix: *const ipv6_addr_t, bits: u8);
}
extern "C" {
    /// @brief   Sets the last @p bits of IPv6 address @p out to @p iid.
    /// Leading bits of @p out stay untouched.
    ///
    /// @param[out]  out     IPv6 address to be set.
    /// @param[in]   iid     buffer representing the iid.
    /// @param[in]   bits    Bits to be copied from @p iid to @p out
    /// (set to 128 when greater than 128).
    pub fn ipv6_addr_init_iid(out: *mut ipv6_addr_t, iid: *const u8, bits: u8);
}
extern "C" {
    /// @brief   Converts an IPv6 address to its string representation
    ///
    /// @see <a href="https://tools.ietf.org/html/rfc5952">
    /// RFC 5952
    /// </a>
    ///
    /// @param[out] result       The resulting string representation of at least
    /// @ref IPV6_ADDR_MAX_STR_LEN
    /// @param[in] addr          An IPv6 address
    /// @param[in] result_len    Length of @p result_len
    ///
    /// @return  @p result, on success
    /// @return  NULL, if @p result_len was lesser than IPV6_ADDR_MAX_STR_LEN
    /// @return  NULL, if @p result or @p addr was NULL
    pub fn ipv6_addr_to_str(
        result: *mut libc::c_char,
        addr: *const ipv6_addr_t,
        result_len: u8,
    ) -> *mut libc::c_char;
}
extern "C" {
    /// @brief   Converts an IPv6 address string representation to a byte-represented
    /// IPv6 address
    ///
    /// @see <a href="https://tools.ietf.org/html/rfc5952">
    /// RFC 5952
    /// </a>
    ///
    /// @param[in] result    The resulting byte representation
    /// @param[in] addr      An IPv6 address string representation
    ///
    /// @return  @p result, on success
    /// @return  NULL, if @p addr was malformed
    /// @return  NULL, if @p result or @p addr was NULL
    pub fn ipv6_addr_from_str(
        result: *mut ipv6_addr_t,
        addr: *const libc::c_char,
    ) -> *mut ipv6_addr_t;
}
extern "C" {
    /// @brief split IPv6 address string representation
    ///
    /// @note Will change @p seperator position in @p addr_str to '\0'
    ///
    /// @param[in,out]   addr_str    Address to split
    /// @param[in]       seperator   Seperator char to use
    /// @param[in]       _default    Default value
    ///
    /// @return      atoi(string after split)
    /// @return      @p _default if no string after @p seperator
    pub fn ipv6_addr_split(
        addr_str: *mut libc::c_char,
        seperator: libc::c_char,
        _default: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    /// @brief Print IPv6 address to stdout
    ///
    /// @param[in]   addr  Pointer to ipv6_addr_t to print
    pub fn ipv6_addr_print(addr: *const ipv6_addr_t);
}
/// @brief Data type to represent an EUI-64.
#[repr(C)]
#[derive(Copy, Clone)]
pub union eui64_t {
    /// < represented as 64 bit value
    pub uint64: network_uint64_t,
    /// < split into 8 8-bit words.
    pub uint8: [u8; 8usize],
    /// < split into 4 16-bit words.
    pub uint16: [network_uint16_t; 4usize],
    _bindgen_union_align: [u8; 8usize],
}
#[test]
fn bindgen_test_layout_eui64_t() {
    assert_eq!(
        ::core::mem::size_of::<eui64_t>(),
        8usize,
        concat!("Size of: ", stringify!(eui64_t))
    );
    assert_eq!(
        ::core::mem::align_of::<eui64_t>(),
        1usize,
        concat!("Alignment of ", stringify!(eui64_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<eui64_t>())).uint64 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(eui64_t),
            "::",
            stringify!(uint64)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<eui64_t>())).uint8 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(eui64_t),
            "::",
            stringify!(uint8)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<eui64_t>())).uint16 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(eui64_t),
            "::",
            stringify!(uint16)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}ieee802154_addr_bcast"]
    pub static mut ieee802154_addr_bcast: [u8; 2usize];
}
extern "C" {
    /// @brief   Initializes an IEEE 802.15.4 MAC frame header in @p buf.
    ///
    /// @pre Resulting header must fit in memory allocated at @p buf.
    ///
    /// @see IEEE Std 802.15.4-2011, 5.2.1 General MAC frame format.
    ///
    /// If @p dst is NULL the IEEE802154_FCF_ACK_REQ will be unset to prevent
    /// flooding the network.
    ///
    /// @param[out] buf      Target memory for frame header.
    /// @param[in] src       Source address for frame in network byteorder.
    /// May be NULL if @ref IEEE802154_FCF_SRC_ADDR_VOID is set
    /// in @p flags.
    /// @param[in] src_len   Length of @p src. Legal values are:
    /// * 0 (will set @ref IEEE802154_FCF_SRC_ADDR_VOID in MHR)
    /// * 2 (will set @ref IEEE802154_FCF_SRC_ADDR_SHORT in MHR)
    /// * 8 (will set @ref IEEE802154_FCF_SRC_ADDR_LONG in MHR)
    /// @param[in] dst       Destination address for frame in network byteorder.
    /// May be NULL if @ref IEEE802154_FCF_SRC_ADDR_VOID is set
    /// in @p flags.
    /// @param[in] dst_len   Length of @p dst. Legal values are:
    /// * 0 (will set @ref IEEE802154_FCF_DST_ADDR_VOID in MHR)
    /// * 2 (will set @ref IEEE802154_FCF_DST_ADDR_SHORT in MHR)
    /// * 8 (will set @ref IEEE802154_FCF_DST_ADDR_LONG in MHR)
    /// @param[in] src_pan   Source PAN ID in little-endian. May be 0 if
    /// @ref IEEE802154_FCF_PAN_COMP is set in @p flags.
    /// Otherwise, it will be ignored, when
    /// @ref IEEE802154_FCF_PAN_COMP is set.
    /// @param[in] dst_pan   Destination PAN ID in little-endian.
    /// @param[in] flags     Flags for the frame. These are interchangable with the
    /// first byte of the IEEE 802.15.4 FCF. This means that
    /// it encompasses the type values,
    /// @ref IEEE802154_FCF_SECURITY_EN,
    /// @ref IEEE802154_FCF_FRAME_PEND, and
    /// @ref IEEE802154_FCF_ACK_REQ.
    /// @param[in] seq       Sequence number for frame.
    ///
    /// The version field in the FCF will be set implicitly to version 1.
    ///
    /// @return  Size of frame header on success.
    /// @return  0, on error (flags set to unexpected state).
    pub fn ieee802154_set_frame_hdr(
        buf: *mut u8,
        src: *const u8,
        src_len: usize,
        dst: *const u8,
        dst_len: usize,
        src_pan: le_uint16_t,
        dst_pan: le_uint16_t,
        flags: u8,
        seq: u8,
    ) -> usize;
}
extern "C" {
    /// @brief   Get length of MAC header.
    ///
    /// @todo include security header implications
    ///
    /// @param[in] mhr   MAC header.
    ///
    /// @return  Length of MAC header on success.
    /// @return  0, on error (source mode or destination mode set to reserved).
    pub fn ieee802154_get_frame_hdr_len(mhr: *const u8) -> usize;
}
extern "C" {
    /// @brief   Gets source address from MAC header.
    ///
    /// @pre (@p src != NULL) && (@p src_pan != NULL)
    ///
    /// @param[in] mhr       MAC header.
    /// @param[out] src      Source address in network byte order in MAC header.
    /// @param[out] src_pan  Source PAN little-endian byte order in MAC header.
    ///
    /// @return   Length of source address.
    /// @return  -EINVAL, if @p mhr contains unexpected flags.
    pub fn ieee802154_get_src(
        mhr: *const u8,
        src: *mut u8,
        src_pan: *mut le_uint16_t,
    ) -> libc::c_int;
}
extern "C" {
    /// @brief   Gets destination address from MAC header.
    ///
    /// @pre (@p dst != NULL) && (@p dst_pan != NULL)
    ///
    /// @param[in] mhr       MAC header.
    /// @param[out] dst      Destination address in network byte order in MAC header.
    /// @param[out] dst_pan  Destination PAN in little-endian byte order in MAC header.
    ///
    /// @return   Length of destination address.
    /// @return  -EINVAL, if @p mhr contains unexpected flags.
    pub fn ieee802154_get_dst(
        mhr: *const u8,
        dst: *mut u8,
        dst_pan: *mut le_uint16_t,
    ) -> libc::c_int;
}
/// @brief   Ethernet header
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ethernet_hdr_t {
    /// < destination address
    pub dst: [u8; 6usize],
    /// < source address
    pub src: [u8; 6usize],
    /// < ether type (see @ref net_ethertype)
    pub type_: network_uint16_t,
}
#[test]
fn bindgen_test_layout_ethernet_hdr_t() {
    assert_eq!(
        ::core::mem::size_of::<ethernet_hdr_t>(),
        14usize,
        concat!("Size of: ", stringify!(ethernet_hdr_t))
    );
    assert_eq!(
        ::core::mem::align_of::<ethernet_hdr_t>(),
        1usize,
        concat!("Alignment of ", stringify!(ethernet_hdr_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ethernet_hdr_t>())).dst as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ethernet_hdr_t),
            "::",
            stringify!(dst)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ethernet_hdr_t>())).src as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(ethernet_hdr_t),
            "::",
            stringify!(src)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ethernet_hdr_t>())).type_ as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(ethernet_hdr_t),
            "::",
            stringify!(type_)
        )
    );
}
/// < no configuration
pub const GNRC_NETIF_AAC_NONE: _bindgen_ty_1 = 0;
/// < Use some automatic bootstrapping (e.g. SLAAC with IPv6)
pub const GNRC_NETIF_AAC_AUTO: _bindgen_ty_1 = 1;
/// < Use DHCP(v6)
pub const GNRC_NETIF_AAC_DHCP: _bindgen_ty_1 = 2;
/// @brief   Auto-address configuration modes
/// @anchor  net_gnrc_netif_aac
pub type _bindgen_ty_1 = u32;
pub type useconds_t = __useconds_t;
pub type socklen_t = __socklen_t;
extern "C" {
    pub fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn faccessat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __type: libc::c_int,
        __flag: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
}
extern "C" {
    pub fn close(__fd: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: usize) -> isize;
}
extern "C" {
    pub fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: usize) -> isize;
}
extern "C" {
    pub fn pread(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __nbytes: usize,
        __offset: __off_t,
    ) -> isize;
}
extern "C" {
    pub fn pwrite(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: usize,
        __offset: __off_t,
    ) -> isize;
}
extern "C" {
    pub fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
}
extern "C" {
    pub fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
}
extern "C" {
    pub fn ualarm(__value: __useconds_t, __interval: __useconds_t) -> __useconds_t;
}
extern "C" {
    pub fn usleep(__useconds: __useconds_t) -> libc::c_int;
}
extern "C" {
    pub fn pause() -> libc::c_int;
}
extern "C" {
    pub fn chown(__file: *const libc::c_char, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
}
extern "C" {
    pub fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
}
extern "C" {
    pub fn lchown(__file: *const libc::c_char, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
}
extern "C" {
    pub fn fchownat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
        __flag: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn chdir(__path: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn fchdir(__fd: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn getcwd(__buf: *mut libc::c_char, __size: usize) -> *mut libc::c_char;
}
extern "C" {
    pub fn getwd(__buf: *mut libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn dup(__fd: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
}
extern "C" {
    #[link_name = "\u{1}__environ"]
    pub static mut __environ: *mut *mut libc::c_char;
}
extern "C" {
    pub fn execve(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn fexecve(
        __fd: libc::c_int,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn execv(__path: *const libc::c_char, __argv: *const *mut libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn execle(__path: *const libc::c_char, __arg: *const libc::c_char, ...) -> libc::c_int;
}
extern "C" {
    pub fn execl(__path: *const libc::c_char, __arg: *const libc::c_char, ...) -> libc::c_int;
}
extern "C" {
    pub fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn execlp(__file: *const libc::c_char, __arg: *const libc::c_char, ...) -> libc::c_int;
}
extern "C" {
    pub fn nice(__inc: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn _exit(__status: libc::c_int);
}
pub const _PC_LINK_MAX: _bindgen_ty_2 = 0;
pub const _PC_MAX_CANON: _bindgen_ty_2 = 1;
pub const _PC_MAX_INPUT: _bindgen_ty_2 = 2;
pub const _PC_NAME_MAX: _bindgen_ty_2 = 3;
pub const _PC_PATH_MAX: _bindgen_ty_2 = 4;
pub const _PC_PIPE_BUF: _bindgen_ty_2 = 5;
pub const _PC_CHOWN_RESTRICTED: _bindgen_ty_2 = 6;
pub const _PC_NO_TRUNC: _bindgen_ty_2 = 7;
pub const _PC_VDISABLE: _bindgen_ty_2 = 8;
pub const _PC_SYNC_IO: _bindgen_ty_2 = 9;
pub const _PC_ASYNC_IO: _bindgen_ty_2 = 10;
pub const _PC_PRIO_IO: _bindgen_ty_2 = 11;
pub const _PC_SOCK_MAXBUF: _bindgen_ty_2 = 12;
pub const _PC_FILESIZEBITS: _bindgen_ty_2 = 13;
pub const _PC_REC_INCR_XFER_SIZE: _bindgen_ty_2 = 14;
pub const _PC_REC_MAX_XFER_SIZE: _bindgen_ty_2 = 15;
pub const _PC_REC_MIN_XFER_SIZE: _bindgen_ty_2 = 16;
pub const _PC_REC_XFER_ALIGN: _bindgen_ty_2 = 17;
pub const _PC_ALLOC_SIZE_MIN: _bindgen_ty_2 = 18;
pub const _PC_SYMLINK_MAX: _bindgen_ty_2 = 19;
pub const _PC_2_SYMLINKS: _bindgen_ty_2 = 20;
pub type _bindgen_ty_2 = u32;
pub const _SC_ARG_MAX: _bindgen_ty_3 = 0;
pub const _SC_CHILD_MAX: _bindgen_ty_3 = 1;
pub const _SC_CLK_TCK: _bindgen_ty_3 = 2;
pub const _SC_NGROUPS_MAX: _bindgen_ty_3 = 3;
pub const _SC_OPEN_MAX: _bindgen_ty_3 = 4;
pub const _SC_STREAM_MAX: _bindgen_ty_3 = 5;
pub const _SC_TZNAME_MAX: _bindgen_ty_3 = 6;
pub const _SC_JOB_CONTROL: _bindgen_ty_3 = 7;
pub const _SC_SAVED_IDS: _bindgen_ty_3 = 8;
pub const _SC_REALTIME_SIGNALS: _bindgen_ty_3 = 9;
pub const _SC_PRIORITY_SCHEDULING: _bindgen_ty_3 = 10;
pub const _SC_TIMERS: _bindgen_ty_3 = 11;
pub const _SC_ASYNCHRONOUS_IO: _bindgen_ty_3 = 12;
pub const _SC_PRIORITIZED_IO: _bindgen_ty_3 = 13;
pub const _SC_SYNCHRONIZED_IO: _bindgen_ty_3 = 14;
pub const _SC_FSYNC: _bindgen_ty_3 = 15;
pub const _SC_MAPPED_FILES: _bindgen_ty_3 = 16;
pub const _SC_MEMLOCK: _bindgen_ty_3 = 17;
pub const _SC_MEMLOCK_RANGE: _bindgen_ty_3 = 18;
pub const _SC_MEMORY_PROTECTION: _bindgen_ty_3 = 19;
pub const _SC_MESSAGE_PASSING: _bindgen_ty_3 = 20;
pub const _SC_SEMAPHORES: _bindgen_ty_3 = 21;
pub const _SC_SHARED_MEMORY_OBJECTS: _bindgen_ty_3 = 22;
pub const _SC_AIO_LISTIO_MAX: _bindgen_ty_3 = 23;
pub const _SC_AIO_MAX: _bindgen_ty_3 = 24;
pub const _SC_AIO_PRIO_DELTA_MAX: _bindgen_ty_3 = 25;
pub const _SC_DELAYTIMER_MAX: _bindgen_ty_3 = 26;
pub const _SC_MQ_OPEN_MAX: _bindgen_ty_3 = 27;
pub const _SC_MQ_PRIO_MAX: _bindgen_ty_3 = 28;
pub const _SC_VERSION: _bindgen_ty_3 = 29;
pub const _SC_PAGESIZE: _bindgen_ty_3 = 30;
pub const _SC_RTSIG_MAX: _bindgen_ty_3 = 31;
pub const _SC_SEM_NSEMS_MAX: _bindgen_ty_3 = 32;
pub const _SC_SEM_VALUE_MAX: _bindgen_ty_3 = 33;
pub const _SC_SIGQUEUE_MAX: _bindgen_ty_3 = 34;
pub const _SC_TIMER_MAX: _bindgen_ty_3 = 35;
pub const _SC_BC_BASE_MAX: _bindgen_ty_3 = 36;
pub const _SC_BC_DIM_MAX: _bindgen_ty_3 = 37;
pub const _SC_BC_SCALE_MAX: _bindgen_ty_3 = 38;
pub const _SC_BC_STRING_MAX: _bindgen_ty_3 = 39;
pub const _SC_COLL_WEIGHTS_MAX: _bindgen_ty_3 = 40;
pub const _SC_EQUIV_CLASS_MAX: _bindgen_ty_3 = 41;
pub const _SC_EXPR_NEST_MAX: _bindgen_ty_3 = 42;
pub const _SC_LINE_MAX: _bindgen_ty_3 = 43;
pub const _SC_RE_DUP_MAX: _bindgen_ty_3 = 44;
pub const _SC_CHARCLASS_NAME_MAX: _bindgen_ty_3 = 45;
pub const _SC_2_VERSION: _bindgen_ty_3 = 46;
pub const _SC_2_C_BIND: _bindgen_ty_3 = 47;
pub const _SC_2_C_DEV: _bindgen_ty_3 = 48;
pub const _SC_2_FORT_DEV: _bindgen_ty_3 = 49;
pub const _SC_2_FORT_RUN: _bindgen_ty_3 = 50;
pub const _SC_2_SW_DEV: _bindgen_ty_3 = 51;
pub const _SC_2_LOCALEDEF: _bindgen_ty_3 = 52;
pub const _SC_PII: _bindgen_ty_3 = 53;
pub const _SC_PII_XTI: _bindgen_ty_3 = 54;
pub const _SC_PII_SOCKET: _bindgen_ty_3 = 55;
pub const _SC_PII_INTERNET: _bindgen_ty_3 = 56;
pub const _SC_PII_OSI: _bindgen_ty_3 = 57;
pub const _SC_POLL: _bindgen_ty_3 = 58;
pub const _SC_SELECT: _bindgen_ty_3 = 59;
pub const _SC_UIO_MAXIOV: _bindgen_ty_3 = 60;
pub const _SC_IOV_MAX: _bindgen_ty_3 = 60;
pub const _SC_PII_INTERNET_STREAM: _bindgen_ty_3 = 61;
pub const _SC_PII_INTERNET_DGRAM: _bindgen_ty_3 = 62;
pub const _SC_PII_OSI_COTS: _bindgen_ty_3 = 63;
pub const _SC_PII_OSI_CLTS: _bindgen_ty_3 = 64;
pub const _SC_PII_OSI_M: _bindgen_ty_3 = 65;
pub const _SC_T_IOV_MAX: _bindgen_ty_3 = 66;
pub const _SC_THREADS: _bindgen_ty_3 = 67;
pub const _SC_THREAD_SAFE_FUNCTIONS: _bindgen_ty_3 = 68;
pub const _SC_GETGR_R_SIZE_MAX: _bindgen_ty_3 = 69;
pub const _SC_GETPW_R_SIZE_MAX: _bindgen_ty_3 = 70;
pub const _SC_LOGIN_NAME_MAX: _bindgen_ty_3 = 71;
pub const _SC_TTY_NAME_MAX: _bindgen_ty_3 = 72;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: _bindgen_ty_3 = 73;
pub const _SC_THREAD_KEYS_MAX: _bindgen_ty_3 = 74;
pub const _SC_THREAD_STACK_MIN: _bindgen_ty_3 = 75;
pub const _SC_THREAD_THREADS_MAX: _bindgen_ty_3 = 76;
pub const _SC_THREAD_ATTR_STACKADDR: _bindgen_ty_3 = 77;
pub const _SC_THREAD_ATTR_STACKSIZE: _bindgen_ty_3 = 78;
pub const _SC_THREAD_PRIORITY_SCHEDULING: _bindgen_ty_3 = 79;
pub const _SC_THREAD_PRIO_INHERIT: _bindgen_ty_3 = 80;
pub const _SC_THREAD_PRIO_PROTECT: _bindgen_ty_3 = 81;
pub const _SC_THREAD_PROCESS_SHARED: _bindgen_ty_3 = 82;
pub const _SC_NPROCESSORS_CONF: _bindgen_ty_3 = 83;
pub const _SC_NPROCESSORS_ONLN: _bindgen_ty_3 = 84;
pub const _SC_PHYS_PAGES: _bindgen_ty_3 = 85;
pub const _SC_AVPHYS_PAGES: _bindgen_ty_3 = 86;
pub const _SC_ATEXIT_MAX: _bindgen_ty_3 = 87;
pub const _SC_PASS_MAX: _bindgen_ty_3 = 88;
pub const _SC_XOPEN_VERSION: _bindgen_ty_3 = 89;
pub const _SC_XOPEN_XCU_VERSION: _bindgen_ty_3 = 90;
pub const _SC_XOPEN_UNIX: _bindgen_ty_3 = 91;
pub const _SC_XOPEN_CRYPT: _bindgen_ty_3 = 92;
pub const _SC_XOPEN_ENH_I18N: _bindgen_ty_3 = 93;
pub const _SC_XOPEN_SHM: _bindgen_ty_3 = 94;
pub const _SC_2_CHAR_TERM: _bindgen_ty_3 = 95;
pub const _SC_2_C_VERSION: _bindgen_ty_3 = 96;
pub const _SC_2_UPE: _bindgen_ty_3 = 97;
pub const _SC_XOPEN_XPG2: _bindgen_ty_3 = 98;
pub const _SC_XOPEN_XPG3: _bindgen_ty_3 = 99;
pub const _SC_XOPEN_XPG4: _bindgen_ty_3 = 100;
pub const _SC_CHAR_BIT: _bindgen_ty_3 = 101;
pub const _SC_CHAR_MAX: _bindgen_ty_3 = 102;
pub const _SC_CHAR_MIN: _bindgen_ty_3 = 103;
pub const _SC_INT_MAX: _bindgen_ty_3 = 104;
pub const _SC_INT_MIN: _bindgen_ty_3 = 105;
pub const _SC_LONG_BIT: _bindgen_ty_3 = 106;
pub const _SC_WORD_BIT: _bindgen_ty_3 = 107;
pub const _SC_MB_LEN_MAX: _bindgen_ty_3 = 108;
pub const _SC_NZERO: _bindgen_ty_3 = 109;
pub const _SC_SSIZE_MAX: _bindgen_ty_3 = 110;
pub const _SC_SCHAR_MAX: _bindgen_ty_3 = 111;
pub const _SC_SCHAR_MIN: _bindgen_ty_3 = 112;
pub const _SC_SHRT_MAX: _bindgen_ty_3 = 113;
pub const _SC_SHRT_MIN: _bindgen_ty_3 = 114;
pub const _SC_UCHAR_MAX: _bindgen_ty_3 = 115;
pub const _SC_UINT_MAX: _bindgen_ty_3 = 116;
pub const _SC_ULONG_MAX: _bindgen_ty_3 = 117;
pub const _SC_USHRT_MAX: _bindgen_ty_3 = 118;
pub const _SC_NL_ARGMAX: _bindgen_ty_3 = 119;
pub const _SC_NL_LANGMAX: _bindgen_ty_3 = 120;
pub const _SC_NL_MSGMAX: _bindgen_ty_3 = 121;
pub const _SC_NL_NMAX: _bindgen_ty_3 = 122;
pub const _SC_NL_SETMAX: _bindgen_ty_3 = 123;
pub const _SC_NL_TEXTMAX: _bindgen_ty_3 = 124;
pub const _SC_XBS5_ILP32_OFF32: _bindgen_ty_3 = 125;
pub const _SC_XBS5_ILP32_OFFBIG: _bindgen_ty_3 = 126;
pub const _SC_XBS5_LP64_OFF64: _bindgen_ty_3 = 127;
pub const _SC_XBS5_LPBIG_OFFBIG: _bindgen_ty_3 = 128;
pub const _SC_XOPEN_LEGACY: _bindgen_ty_3 = 129;
pub const _SC_XOPEN_REALTIME: _bindgen_ty_3 = 130;
pub const _SC_XOPEN_REALTIME_THREADS: _bindgen_ty_3 = 131;
pub const _SC_ADVISORY_INFO: _bindgen_ty_3 = 132;
pub const _SC_BARRIERS: _bindgen_ty_3 = 133;
pub const _SC_BASE: _bindgen_ty_3 = 134;
pub const _SC_C_LANG_SUPPORT: _bindgen_ty_3 = 135;
pub const _SC_C_LANG_SUPPORT_R: _bindgen_ty_3 = 136;
pub const _SC_CLOCK_SELECTION: _bindgen_ty_3 = 137;
pub const _SC_CPUTIME: _bindgen_ty_3 = 138;
pub const _SC_THREAD_CPUTIME: _bindgen_ty_3 = 139;
pub const _SC_DEVICE_IO: _bindgen_ty_3 = 140;
pub const _SC_DEVICE_SPECIFIC: _bindgen_ty_3 = 141;
pub const _SC_DEVICE_SPECIFIC_R: _bindgen_ty_3 = 142;
pub const _SC_FD_MGMT: _bindgen_ty_3 = 143;
pub const _SC_FIFO: _bindgen_ty_3 = 144;
pub const _SC_PIPE: _bindgen_ty_3 = 145;
pub const _SC_FILE_ATTRIBUTES: _bindgen_ty_3 = 146;
pub const _SC_FILE_LOCKING: _bindgen_ty_3 = 147;
pub const _SC_FILE_SYSTEM: _bindgen_ty_3 = 148;
pub const _SC_MONOTONIC_CLOCK: _bindgen_ty_3 = 149;
pub const _SC_MULTI_PROCESS: _bindgen_ty_3 = 150;
pub const _SC_SINGLE_PROCESS: _bindgen_ty_3 = 151;
pub const _SC_NETWORKING: _bindgen_ty_3 = 152;
pub const _SC_READER_WRITER_LOCKS: _bindgen_ty_3 = 153;
pub const _SC_SPIN_LOCKS: _bindgen_ty_3 = 154;
pub const _SC_REGEXP: _bindgen_ty_3 = 155;
pub const _SC_REGEX_VERSION: _bindgen_ty_3 = 156;
pub const _SC_SHELL: _bindgen_ty_3 = 157;
pub const _SC_SIGNALS: _bindgen_ty_3 = 158;
pub const _SC_SPAWN: _bindgen_ty_3 = 159;
pub const _SC_SPORADIC_SERVER: _bindgen_ty_3 = 160;
pub const _SC_THREAD_SPORADIC_SERVER: _bindgen_ty_3 = 161;
pub const _SC_SYSTEM_DATABASE: _bindgen_ty_3 = 162;
pub const _SC_SYSTEM_DATABASE_R: _bindgen_ty_3 = 163;
pub const _SC_TIMEOUTS: _bindgen_ty_3 = 164;
pub const _SC_TYPED_MEMORY_OBJECTS: _bindgen_ty_3 = 165;
pub const _SC_USER_GROUPS: _bindgen_ty_3 = 166;
pub const _SC_USER_GROUPS_R: _bindgen_ty_3 = 167;
pub const _SC_2_PBS: _bindgen_ty_3 = 168;
pub const _SC_2_PBS_ACCOUNTING: _bindgen_ty_3 = 169;
pub const _SC_2_PBS_LOCATE: _bindgen_ty_3 = 170;
pub const _SC_2_PBS_MESSAGE: _bindgen_ty_3 = 171;
pub const _SC_2_PBS_TRACK: _bindgen_ty_3 = 172;
pub const _SC_SYMLOOP_MAX: _bindgen_ty_3 = 173;
pub const _SC_STREAMS: _bindgen_ty_3 = 174;
pub const _SC_2_PBS_CHECKPOINT: _bindgen_ty_3 = 175;
pub const _SC_V6_ILP32_OFF32: _bindgen_ty_3 = 176;
pub const _SC_V6_ILP32_OFFBIG: _bindgen_ty_3 = 177;
pub const _SC_V6_LP64_OFF64: _bindgen_ty_3 = 178;
pub const _SC_V6_LPBIG_OFFBIG: _bindgen_ty_3 = 179;
pub const _SC_HOST_NAME_MAX: _bindgen_ty_3 = 180;
pub const _SC_TRACE: _bindgen_ty_3 = 181;
pub const _SC_TRACE_EVENT_FILTER: _bindgen_ty_3 = 182;
pub const _SC_TRACE_INHERIT: _bindgen_ty_3 = 183;
pub const _SC_TRACE_LOG: _bindgen_ty_3 = 184;
pub const _SC_LEVEL1_ICACHE_SIZE: _bindgen_ty_3 = 185;
pub const _SC_LEVEL1_ICACHE_ASSOC: _bindgen_ty_3 = 186;
pub const _SC_LEVEL1_ICACHE_LINESIZE: _bindgen_ty_3 = 187;
pub const _SC_LEVEL1_DCACHE_SIZE: _bindgen_ty_3 = 188;
pub const _SC_LEVEL1_DCACHE_ASSOC: _bindgen_ty_3 = 189;
pub const _SC_LEVEL1_DCACHE_LINESIZE: _bindgen_ty_3 = 190;
pub const _SC_LEVEL2_CACHE_SIZE: _bindgen_ty_3 = 191;
pub const _SC_LEVEL2_CACHE_ASSOC: _bindgen_ty_3 = 192;
pub const _SC_LEVEL2_CACHE_LINESIZE: _bindgen_ty_3 = 193;
pub const _SC_LEVEL3_CACHE_SIZE: _bindgen_ty_3 = 194;
pub const _SC_LEVEL3_CACHE_ASSOC: _bindgen_ty_3 = 195;
pub const _SC_LEVEL3_CACHE_LINESIZE: _bindgen_ty_3 = 196;
pub const _SC_LEVEL4_CACHE_SIZE: _bindgen_ty_3 = 197;
pub const _SC_LEVEL4_CACHE_ASSOC: _bindgen_ty_3 = 198;
pub const _SC_LEVEL4_CACHE_LINESIZE: _bindgen_ty_3 = 199;
pub const _SC_IPV6: _bindgen_ty_3 = 235;
pub const _SC_RAW_SOCKETS: _bindgen_ty_3 = 236;
pub const _SC_V7_ILP32_OFF32: _bindgen_ty_3 = 237;
pub const _SC_V7_ILP32_OFFBIG: _bindgen_ty_3 = 238;
pub const _SC_V7_LP64_OFF64: _bindgen_ty_3 = 239;
pub const _SC_V7_LPBIG_OFFBIG: _bindgen_ty_3 = 240;
pub const _SC_SS_REPL_MAX: _bindgen_ty_3 = 241;
pub const _SC_TRACE_EVENT_NAME_MAX: _bindgen_ty_3 = 242;
pub const _SC_TRACE_NAME_MAX: _bindgen_ty_3 = 243;
pub const _SC_TRACE_SYS_MAX: _bindgen_ty_3 = 244;
pub const _SC_TRACE_USER_EVENT_MAX: _bindgen_ty_3 = 245;
pub const _SC_XOPEN_STREAMS: _bindgen_ty_3 = 246;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: _bindgen_ty_3 = 247;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: _bindgen_ty_3 = 248;
pub type _bindgen_ty_3 = u32;
pub const _CS_PATH: _bindgen_ty_4 = 0;
pub const _CS_V6_WIDTH_RESTRICTED_ENVS: _bindgen_ty_4 = 1;
pub const _CS_GNU_LIBC_VERSION: _bindgen_ty_4 = 2;
pub const _CS_GNU_LIBPTHREAD_VERSION: _bindgen_ty_4 = 3;
pub const _CS_V5_WIDTH_RESTRICTED_ENVS: _bindgen_ty_4 = 4;
pub const _CS_V7_WIDTH_RESTRICTED_ENVS: _bindgen_ty_4 = 5;
pub const _CS_LFS_CFLAGS: _bindgen_ty_4 = 1000;
pub const _CS_LFS_LDFLAGS: _bindgen_ty_4 = 1001;
pub const _CS_LFS_LIBS: _bindgen_ty_4 = 1002;
pub const _CS_LFS_LINTFLAGS: _bindgen_ty_4 = 1003;
pub const _CS_LFS64_CFLAGS: _bindgen_ty_4 = 1004;
pub const _CS_LFS64_LDFLAGS: _bindgen_ty_4 = 1005;
pub const _CS_LFS64_LIBS: _bindgen_ty_4 = 1006;
pub const _CS_LFS64_LINTFLAGS: _bindgen_ty_4 = 1007;
pub const _CS_XBS5_ILP32_OFF32_CFLAGS: _bindgen_ty_4 = 1100;
pub const _CS_XBS5_ILP32_OFF32_LDFLAGS: _bindgen_ty_4 = 1101;
pub const _CS_XBS5_ILP32_OFF32_LIBS: _bindgen_ty_4 = 1102;
pub const _CS_XBS5_ILP32_OFF32_LINTFLAGS: _bindgen_ty_4 = 1103;
pub const _CS_XBS5_ILP32_OFFBIG_CFLAGS: _bindgen_ty_4 = 1104;
pub const _CS_XBS5_ILP32_OFFBIG_LDFLAGS: _bindgen_ty_4 = 1105;
pub const _CS_XBS5_ILP32_OFFBIG_LIBS: _bindgen_ty_4 = 1106;
pub const _CS_XBS5_ILP32_OFFBIG_LINTFLAGS: _bindgen_ty_4 = 1107;
pub const _CS_XBS5_LP64_OFF64_CFLAGS: _bindgen_ty_4 = 1108;
pub const _CS_XBS5_LP64_OFF64_LDFLAGS: _bindgen_ty_4 = 1109;
pub const _CS_XBS5_LP64_OFF64_LIBS: _bindgen_ty_4 = 1110;
pub const _CS_XBS5_LP64_OFF64_LINTFLAGS: _bindgen_ty_4 = 1111;
pub const _CS_XBS5_LPBIG_OFFBIG_CFLAGS: _bindgen_ty_4 = 1112;
pub const _CS_XBS5_LPBIG_OFFBIG_LDFLAGS: _bindgen_ty_4 = 1113;
pub const _CS_XBS5_LPBIG_OFFBIG_LIBS: _bindgen_ty_4 = 1114;
pub const _CS_XBS5_LPBIG_OFFBIG_LINTFLAGS: _bindgen_ty_4 = 1115;
pub const _CS_POSIX_V6_ILP32_OFF32_CFLAGS: _bindgen_ty_4 = 1116;
pub const _CS_POSIX_V6_ILP32_OFF32_LDFLAGS: _bindgen_ty_4 = 1117;
pub const _CS_POSIX_V6_ILP32_OFF32_LIBS: _bindgen_ty_4 = 1118;
pub const _CS_POSIX_V6_ILP32_OFF32_LINTFLAGS: _bindgen_ty_4 = 1119;
pub const _CS_POSIX_V6_ILP32_OFFBIG_CFLAGS: _bindgen_ty_4 = 1120;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS: _bindgen_ty_4 = 1121;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LIBS: _bindgen_ty_4 = 1122;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS: _bindgen_ty_4 = 1123;
pub const _CS_POSIX_V6_LP64_OFF64_CFLAGS: _bindgen_ty_4 = 1124;
pub const _CS_POSIX_V6_LP64_OFF64_LDFLAGS: _bindgen_ty_4 = 1125;
pub const _CS_POSIX_V6_LP64_OFF64_LIBS: _bindgen_ty_4 = 1126;
pub const _CS_POSIX_V6_LP64_OFF64_LINTFLAGS: _bindgen_ty_4 = 1127;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS: _bindgen_ty_4 = 1128;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS: _bindgen_ty_4 = 1129;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LIBS: _bindgen_ty_4 = 1130;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS: _bindgen_ty_4 = 1131;
pub const _CS_POSIX_V7_ILP32_OFF32_CFLAGS: _bindgen_ty_4 = 1132;
pub const _CS_POSIX_V7_ILP32_OFF32_LDFLAGS: _bindgen_ty_4 = 1133;
pub const _CS_POSIX_V7_ILP32_OFF32_LIBS: _bindgen_ty_4 = 1134;
pub const _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS: _bindgen_ty_4 = 1135;
pub const _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS: _bindgen_ty_4 = 1136;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS: _bindgen_ty_4 = 1137;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LIBS: _bindgen_ty_4 = 1138;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS: _bindgen_ty_4 = 1139;
pub const _CS_POSIX_V7_LP64_OFF64_CFLAGS: _bindgen_ty_4 = 1140;
pub const _CS_POSIX_V7_LP64_OFF64_LDFLAGS: _bindgen_ty_4 = 1141;
pub const _CS_POSIX_V7_LP64_OFF64_LIBS: _bindgen_ty_4 = 1142;
pub const _CS_POSIX_V7_LP64_OFF64_LINTFLAGS: _bindgen_ty_4 = 1143;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS: _bindgen_ty_4 = 1144;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS: _bindgen_ty_4 = 1145;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LIBS: _bindgen_ty_4 = 1146;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS: _bindgen_ty_4 = 1147;
pub const _CS_V6_ENV: _bindgen_ty_4 = 1148;
pub const _CS_V7_ENV: _bindgen_ty_4 = 1149;
pub type _bindgen_ty_4 = u32;
extern "C" {
    pub fn pathconf(__path: *const libc::c_char, __name: libc::c_int) -> libc::c_long;
}
extern "C" {
    pub fn fpathconf(__fd: libc::c_int, __name: libc::c_int) -> libc::c_long;
}
extern "C" {
    pub fn sysconf(__name: libc::c_int) -> libc::c_long;
}
extern "C" {
    pub fn confstr(__name: libc::c_int, __buf: *mut libc::c_char, __len: usize) -> usize;
}
extern "C" {
    pub fn getpid() -> __pid_t;
}
extern "C" {
    pub fn getppid() -> __pid_t;
}
extern "C" {
    pub fn getpgrp() -> __pid_t;
}
extern "C" {
    pub fn __getpgid(__pid: __pid_t) -> __pid_t;
}
extern "C" {
    pub fn getpgid(__pid: __pid_t) -> __pid_t;
}
extern "C" {
    pub fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> libc::c_int;
}
extern "C" {
    pub fn setpgrp() -> libc::c_int;
}
extern "C" {
    pub fn setsid() -> __pid_t;
}
extern "C" {
    pub fn getsid(__pid: __pid_t) -> __pid_t;
}
extern "C" {
    pub fn getuid() -> __uid_t;
}
extern "C" {
    pub fn geteuid() -> __uid_t;
}
extern "C" {
    pub fn getgid() -> __gid_t;
}
extern "C" {
    pub fn getegid() -> __gid_t;
}
extern "C" {
    pub fn getgroups(__size: libc::c_int, __list: *mut __gid_t) -> libc::c_int;
}
extern "C" {
    pub fn setuid(__uid: __uid_t) -> libc::c_int;
}
extern "C" {
    pub fn setreuid(__ruid: __uid_t, __euid: __uid_t) -> libc::c_int;
}
extern "C" {
    pub fn seteuid(__uid: __uid_t) -> libc::c_int;
}
extern "C" {
    pub fn setgid(__gid: __gid_t) -> libc::c_int;
}
extern "C" {
    pub fn setregid(__rgid: __gid_t, __egid: __gid_t) -> libc::c_int;
}
extern "C" {
    pub fn setegid(__gid: __gid_t) -> libc::c_int;
}
extern "C" {
    pub fn fork() -> __pid_t;
}
extern "C" {
    pub fn vfork() -> libc::c_int;
}
extern "C" {
    pub fn ttyname(__fd: libc::c_int) -> *mut libc::c_char;
}
extern "C" {
    pub fn ttyname_r(__fd: libc::c_int, __buf: *mut libc::c_char, __buflen: usize) -> libc::c_int;
}
extern "C" {
    pub fn isatty(__fd: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn ttyslot() -> libc::c_int;
}
extern "C" {
    pub fn link(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn linkat(
        __fromfd: libc::c_int,
        __from: *const libc::c_char,
        __tofd: libc::c_int,
        __to: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn symlink(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn readlink(__path: *const libc::c_char, __buf: *mut libc::c_char, __len: usize) -> isize;
}
extern "C" {
    pub fn symlinkat(
        __from: *const libc::c_char,
        __tofd: libc::c_int,
        __to: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn readlinkat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: usize,
    ) -> isize;
}
extern "C" {
    pub fn unlink(__name: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn unlinkat(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __flag: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn rmdir(__path: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn tcgetpgrp(__fd: libc::c_int) -> __pid_t;
}
extern "C" {
    pub fn tcsetpgrp(__fd: libc::c_int, __pgrp_id: __pid_t) -> libc::c_int;
}
extern "C" {
    pub fn getlogin() -> *mut libc::c_char;
}
extern "C" {
    pub fn getlogin_r(__name: *mut libc::c_char, __name_len: usize) -> libc::c_int;
}
extern "C" {
    pub fn setlogin(__name: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    #[link_name = "\u{1}optarg"]
    pub static mut optarg: *mut libc::c_char;
}
extern "C" {
    #[link_name = "\u{1}optind"]
    pub static mut optind: libc::c_int;
}
extern "C" {
    #[link_name = "\u{1}opterr"]
    pub static mut opterr: libc::c_int;
}
extern "C" {
    #[link_name = "\u{1}optopt"]
    pub static mut optopt: libc::c_int;
}
extern "C" {
    pub fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn gethostname(__name: *mut libc::c_char, __len: usize) -> libc::c_int;
}
extern "C" {
    pub fn sethostname(__name: *const libc::c_char, __len: usize) -> libc::c_int;
}
extern "C" {
    pub fn sethostid(__id: libc::c_long) -> libc::c_int;
}
extern "C" {
    pub fn getdomainname(__name: *mut libc::c_char, __len: usize) -> libc::c_int;
}
extern "C" {
    pub fn setdomainname(__name: *const libc::c_char, __len: usize) -> libc::c_int;
}
extern "C" {
    pub fn vhangup() -> libc::c_int;
}
extern "C" {
    pub fn revoke(__file: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn profil(
        __sample_buffer: *mut libc::c_ushort,
        __size: usize,
        __offset: usize,
        __scale: libc::c_uint,
    ) -> libc::c_int;
}
extern "C" {
    pub fn acct(__name: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn getusershell() -> *mut libc::c_char;
}
extern "C" {
    pub fn endusershell();
}
extern "C" {
    pub fn setusershell();
}
extern "C" {
    pub fn daemon(__nochdir: libc::c_int, __noclose: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn chroot(__path: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn getpass(__prompt: *const libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn fsync(__fd: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn gethostid() -> libc::c_long;
}
extern "C" {
    pub fn sync();
}
extern "C" {
    pub fn getpagesize() -> libc::c_int;
}
extern "C" {
    pub fn getdtablesize() -> libc::c_int;
}
extern "C" {
    pub fn truncate(__file: *const libc::c_char, __length: __off_t) -> libc::c_int;
}
extern "C" {
    pub fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
}
extern "C" {
    pub fn brk(__addr: *mut libc::c_void) -> libc::c_int;
}
extern "C" {
    pub fn sbrk(__delta: isize) -> *mut libc::c_void;
}
extern "C" {
    pub fn syscall(__sysno: libc::c_long, ...) -> libc::c_long;
}
extern "C" {
    pub fn lockf(__fd: libc::c_int, __cmd: libc::c_int, __len: __off_t) -> libc::c_int;
}
extern "C" {
    pub fn fdatasync(__fildes: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn getentropy(__buffer: *mut libc::c_void, __length: usize) -> libc::c_int;
}
/// @brief iolist forward declaration
pub type iolist_t = iolist;
/// @brief   iolist structure definition
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iolist {
    /// < ptr to next list entry
    pub iol_next: *mut iolist_t,
    /// < ptr to this list entries data
    pub iol_base: *mut libc::c_void,
    /// < size of data pointet to by ptr
    pub iol_len: usize,
}
#[test]
fn bindgen_test_layout_iolist() {
    assert_eq!(
        ::core::mem::size_of::<iolist>(),
        24usize,
        concat!("Size of: ", stringify!(iolist))
    );
    assert_eq!(
        ::core::mem::align_of::<iolist>(),
        8usize,
        concat!("Alignment of ", stringify!(iolist))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<iolist>())).iol_next as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(iolist),
            "::",
            stringify!(iol_next)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<iolist>())).iol_base as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(iolist),
            "::",
            stringify!(iol_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<iolist>())).iol_len as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(iolist),
            "::",
            stringify!(iol_len)
        )
    );
}
extern "C" {
    /// @brief   Count number of entries in an iolist_t
    ///
    /// @param[in]   iolist  iolist to count
    ///
    /// @returns number of entries (zero for NULL parameter)
    pub fn iolist_count(iolist: *const iolist_t) -> libc::c_uint;
}
extern "C" {
    /// @brief   Sum up number of bytes in iolist
    ///
    /// This function returns the summed ip lenght values of all entries in @p
    /// iolist.
    ///
    /// @param[in]   iolist  iolist to sum up
    ///
    /// @returns summed up number of bytes or zero if @p iolist == NULL
    pub fn iolist_size(iolist: *const iolist_t) -> usize;
}
/// @brief  struct iovec anonymous declaration
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iovec {
    _unused: [u8; 0],
}
extern "C" {
    /// @brief   Create struct iovec from iolist
    ///
    /// This function fills an array of struct iovecs with the contents of @p
    /// iolist. It will write the number of used array entries into @p count.
    ///
    /// The caller *must* ensure that @p iov p points to an array of size >= count!
    ///
    /// @param[in]   iolist  iolist to read from
    /// @param[out]  iov     ptr to array of struct iovec that will be filled
    /// @param[out]  count   number of elements in @p iolist
    ///
    /// @returns iolist_size(iolist)
    pub fn iolist_to_iovec(
        iolist: *const iolist_t,
        iov: *mut iovec,
        count: *mut libc::c_uint,
    ) -> usize;
}
pub const NETDEV_TYPE_UNKNOWN: _bindgen_ty_5 = 0;
pub const NETDEV_TYPE_RAW: _bindgen_ty_5 = 1;
pub const NETDEV_TYPE_ETHERNET: _bindgen_ty_5 = 2;
pub const NETDEV_TYPE_IEEE802154: _bindgen_ty_5 = 3;
pub const NETDEV_TYPE_BLE: _bindgen_ty_5 = 4;
pub const NETDEV_TYPE_CC110X: _bindgen_ty_5 = 5;
pub const NETDEV_TYPE_LORA: _bindgen_ty_5 = 6;
pub const NETDEV_TYPE_NRFMIN: _bindgen_ty_5 = 7;
pub const NETDEV_TYPE_SLIP: _bindgen_ty_5 = 8;
pub type _bindgen_ty_5 = u32;
/// < driver needs it's ISR handled
pub const netdev_event_t_NETDEV_EVENT_ISR: netdev_event_t = 0;
/// < started to receive a packet
pub const netdev_event_t_NETDEV_EVENT_RX_STARTED: netdev_event_t = 1;
/// < finished receiving a packet
pub const netdev_event_t_NETDEV_EVENT_RX_COMPLETE: netdev_event_t = 2;
/// < started to transfer a packet
pub const netdev_event_t_NETDEV_EVENT_TX_STARTED: netdev_event_t = 3;
/// < transfer packet complete
pub const netdev_event_t_NETDEV_EVENT_TX_COMPLETE: netdev_event_t = 4;
/// < transfer packet complete and data pending flag
pub const netdev_event_t_NETDEV_EVENT_TX_COMPLETE_DATA_PENDING: netdev_event_t = 5;
/// < ACK requested but not received
pub const netdev_event_t_NETDEV_EVENT_TX_NOACK: netdev_event_t = 6;
/// < couldn't transfer packet
pub const netdev_event_t_NETDEV_EVENT_TX_MEDIUM_BUSY: netdev_event_t = 7;
/// < link established
pub const netdev_event_t_NETDEV_EVENT_LINK_UP: netdev_event_t = 8;
/// < link gone
pub const netdev_event_t_NETDEV_EVENT_LINK_DOWN: netdev_event_t = 9;
/// < timeout when sending
pub const netdev_event_t_NETDEV_EVENT_TX_TIMEOUT: netdev_event_t = 10;
/// < timeout when receiving
pub const netdev_event_t_NETDEV_EVENT_RX_TIMEOUT: netdev_event_t = 11;
/// < wrong CRC
pub const netdev_event_t_NETDEV_EVENT_CRC_ERROR: netdev_event_t = 12;
/// < channel changed
pub const netdev_event_t_NETDEV_EVENT_FHSS_CHANGE_CHANNEL: netdev_event_t = 13;
/// < channel activity detection done
pub const netdev_event_t_NETDEV_EVENT_CAD_DONE: netdev_event_t = 14;
/// @brief   Possible event types that are send from the device driver to the
/// upper layer
pub type netdev_event_t = u32;
/// @brief   Received packet status information for most radios
///
/// May be different for certain radios.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct netdev_radio_rx_info {
    /// < RSSI of a received packet in dBm
    pub rssi: i16,
    /// < LQI of a received packet
    pub lqi: u8,
}
#[test]
fn bindgen_test_layout_netdev_radio_rx_info() {
    assert_eq!(
        ::core::mem::size_of::<netdev_radio_rx_info>(),
        4usize,
        concat!("Size of: ", stringify!(netdev_radio_rx_info))
    );
    assert_eq!(
        ::core::mem::align_of::<netdev_radio_rx_info>(),
        2usize,
        concat!("Alignment of ", stringify!(netdev_radio_rx_info))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<netdev_radio_rx_info>())).rssi as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(netdev_radio_rx_info),
            "::",
            stringify!(rssi)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<netdev_radio_rx_info>())).lqi as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(netdev_radio_rx_info),
            "::",
            stringify!(lqi)
        )
    );
}
/// @brief   Forward declaration for netdev struct
pub type netdev_t = netdev;
/// @brief   Event callback for signaling event to upper layers
///
/// @param[in] type          type of the event
pub type netdev_event_cb_t =
    ::core::option::Option<unsafe extern "C" fn(dev: *mut netdev_t, event: netdev_event_t)>;
/// @brief Structure to hold driver state
///
/// Supposed to be extended by driver implementations.
/// The extended structure should contain all variable driver state.
///
/// Contains a field @p context which is not used by the drivers, but supposed to
/// be used by upper layers to store reference information.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct netdev {
    /// < ptr to that driver's interface.
    pub driver: *const netdev_driver,
    /// < callback for device events
    pub event_callback: netdev_event_cb_t,
    /// < ptr to network stack context
    pub context: *mut libc::c_void,
}
#[test]
fn bindgen_test_layout_netdev() {
    assert_eq!(
        ::core::mem::size_of::<netdev>(),
        24usize,
        concat!("Size of: ", stringify!(netdev))
    );
    assert_eq!(
        ::core::mem::align_of::<netdev>(),
        8usize,
        concat!("Alignment of ", stringify!(netdev))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<netdev>())).driver as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(netdev),
            "::",
            stringify!(driver)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<netdev>())).event_callback as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(netdev),
            "::",
            stringify!(event_callback)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<netdev>())).context as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(netdev),
            "::",
            stringify!(context)
        )
    );
}
/// @brief Structure to hold driver interface -> function mapping
///
/// The send/receive functions expect/return a full ethernet
/// frame (dst mac, src mac, ethertype, payload, no checksum).
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct netdev_driver {
    /// @brief Send frame
    ///
    /// @pre `(dev != NULL) && (iolist != NULL`
    ///
    /// @param[in] dev       network device descriptor
    /// @param[in] iolist    io vector list to send
    ///
    /// @return number of bytes sent, or `< 0` on error
    pub send: ::core::option::Option<
        unsafe extern "C" fn(dev: *mut netdev_t, iolist: *const iolist_t) -> libc::c_int,
    >,
    /// @brief Get a received frame
    ///
    /// @pre `(dev != NULL)`
    /// @pre `(buf != NULL) && (len > 0)`
    ///
    /// Supposed to be called from
    /// @ref netdev_t::event_callback "netdev->event_callback()"
    ///
    /// If buf == NULL and len == 0, returns the packet size without dropping it.
    /// If buf == NULL and len > 0, drops the packet and returns the packet size.
    ///
    /// @param[in]   dev     network device descriptor
    /// @param[out]  buf     buffer to write into or NULL
    /// @param[in]   len     maximum number of bytes to read
    /// @param[out] info     status information for the received packet. Might
    /// be of different type for different netdev devices.
    /// May be NULL if not needed or applicable.
    ///
    /// @return `< 0` on error
    /// @return number of bytes read if buf != NULL
    /// @return packet size if buf == NULL
    pub recv: ::core::option::Option<
        unsafe extern "C" fn(
            dev: *mut netdev_t,
            buf: *mut libc::c_void,
            len: usize,
            info: *mut libc::c_void,
        ) -> libc::c_int,
    >,
    /// @brief the driver's initialization function
    ///
    /// @pre `(dev != NULL)`
    ///
    /// @return `< 0` on error, 0 on success
    pub init: ::core::option::Option<unsafe extern "C" fn(dev: *mut netdev_t) -> libc::c_int>,
    /// @brief a driver's user-space ISR handler
    ///
    /// @pre `(dev != NULL)`
    ///
    /// This function will be called from a network stack's loop when being
    /// notified by netdev_isr.
    ///
    /// It is supposed to call
    /// @ref netdev_t::event_callback "netdev->event_callback()" for each
    /// occurring event.
    ///
    /// See receive packet flow description for details.
    ///
    /// @param[in]   dev     network device descriptor
    pub isr: ::core::option::Option<unsafe extern "C" fn(dev: *mut netdev_t)>,
    /// @brief   Get an option value from a given network device
    ///
    /// @pre `(dev != NULL)`
    ///
    /// @param[in]   dev     network device descriptor
    /// @param[in]   opt     option type
    /// @param[out]  value   pointer to store the option's value in
    /// @param[in]   max_len maximal amount of byte that fit into @p value
    ///
    /// @return              number of bytes written to @p value
    /// @return              `< 0` on error, 0 on success
    pub get: ::core::option::Option<
        unsafe extern "C" fn(
            dev: *mut netdev_t,
            opt: netopt_t,
            value: *mut libc::c_void,
            max_len: usize,
        ) -> libc::c_int,
    >,
    /// @brief   Set an option value for a given network device
    ///
    /// @pre `(dev != NULL)`
    ///
    /// @param[in] dev       network device descriptor
    /// @param[in] opt       option type
    /// @param[in] value     value to set
    /// @param[in] value_len the length of @p value
    ///
    /// @return              number of bytes used from @p value
    /// @return              `< 0` on error, 0 on success
    pub set: ::core::option::Option<
        unsafe extern "C" fn(
            dev: *mut netdev_t,
            opt: netopt_t,
            value: *const libc::c_void,
            value_len: usize,
        ) -> libc::c_int,
    >,
}
#[test]
fn bindgen_test_layout_netdev_driver() {
    assert_eq!(
        ::core::mem::size_of::<netdev_driver>(),
        48usize,
        concat!("Size of: ", stringify!(netdev_driver))
    );
    assert_eq!(
        ::core::mem::align_of::<netdev_driver>(),
        8usize,
        concat!("Alignment of ", stringify!(netdev_driver))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<netdev_driver>())).send as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(netdev_driver),
            "::",
            stringify!(send)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<netdev_driver>())).recv as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(netdev_driver),
            "::",
            stringify!(recv)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<netdev_driver>())).init as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(netdev_driver),
            "::",
            stringify!(init)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<netdev_driver>())).isr as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(netdev_driver),
            "::",
            stringify!(isr)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<netdev_driver>())).get as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(netdev_driver),
            "::",
            stringify!(get)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<netdev_driver>())).set as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(netdev_driver),
            "::",
            stringify!(set)
        )
    );
}
pub type netdev_driver_t = netdev_driver;
pub const memory_order_memory_order_relaxed: memory_order = 0;
pub const memory_order_memory_order_consume: memory_order = 1;
pub const memory_order_memory_order_acquire: memory_order = 2;
pub const memory_order_memory_order_release: memory_order = 3;
pub const memory_order_memory_order_acq_rel: memory_order = 4;
pub const memory_order_memory_order_seq_cst: memory_order = 5;
pub type memory_order = u32;
extern "C" {
    pub fn atomic_thread_fence(arg1: memory_order);
}
extern "C" {
    pub fn atomic_signal_fence(arg1: memory_order);
}
pub type atomic_bool = u8;
pub type atomic_char = u8;
pub type atomic_schar = u8;
pub type atomic_uchar = u8;
pub type atomic_short = u16;
pub type atomic_ushort = u16;
pub type atomic_int = u32;
pub type atomic_uint = u32;
pub type atomic_long = u64;
pub type atomic_ulong = u64;
pub type atomic_llong = u64;
pub type atomic_ullong = u64;
pub type atomic_char16_t = uint_least16_t;
pub type atomic_char32_t = uint_least32_t;
pub type atomic_wchar_t = wchar_t;
pub type atomic_int_least8_t = int_least8_t;
pub type atomic_uint_least8_t = uint_least8_t;
pub type atomic_int_least16_t = int_least16_t;
pub type atomic_uint_least16_t = uint_least16_t;
pub type atomic_int_least32_t = int_least32_t;
pub type atomic_uint_least32_t = uint_least32_t;
pub type atomic_int_least64_t = int_least64_t;
pub type atomic_uint_least64_t = uint_least64_t;
pub type atomic_int_fast8_t = int_fast8_t;
pub type atomic_uint_fast8_t = uint_fast8_t;
pub type atomic_int_fast16_t = int_fast16_t;
pub type atomic_uint_fast16_t = uint_fast16_t;
pub type atomic_int_fast32_t = int_fast32_t;
pub type atomic_uint_fast32_t = uint_fast32_t;
pub type atomic_int_fast64_t = int_fast64_t;
pub type atomic_uint_fast64_t = uint_fast64_t;
pub type atomic_intptr_t = isize;
pub type atomic_uintptr_t = usize;
pub type atomic_size_t = usize;
pub type atomic_ptrdiff_t = isize;
pub type atomic_intmax_t = intmax_t;
pub type atomic_uintmax_t = uintmax_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct atomic_flag {
    pub _Value: atomic_bool,
}
#[test]
fn bindgen_test_layout_atomic_flag() {
    assert_eq!(
        ::core::mem::size_of::<atomic_flag>(),
        1usize,
        concat!("Size of: ", stringify!(atomic_flag))
    );
    assert_eq!(
        ::core::mem::align_of::<atomic_flag>(),
        1usize,
        concat!("Alignment of ", stringify!(atomic_flag))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<atomic_flag>()))._Value as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(atomic_flag),
            "::",
            stringify!(_Value)
        )
    );
}
extern "C" {
    pub fn atomic_flag_test_and_set(arg1: *mut atomic_flag) -> bool;
}
extern "C" {
    pub fn atomic_flag_test_and_set_explicit(arg1: *mut atomic_flag, arg2: memory_order) -> bool;
}
extern "C" {
    pub fn atomic_flag_clear(arg1: *mut atomic_flag);
}
extern "C" {
    pub fn atomic_flag_clear_explicit(arg1: *mut atomic_flag, arg2: memory_order);
}
/// @brief Mutex structure. Must never be modified by the user.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mutex_t {
    /// @brief   The process waiting queue of the mutex. **Must never be changed
    /// by the user.**
    /// @internal
    pub queue: list_node_t,
}
#[test]
fn bindgen_test_layout_mutex_t() {
    assert_eq!(
        ::core::mem::size_of::<mutex_t>(),
        8usize,
        concat!("Size of: ", stringify!(mutex_t))
    );
    assert_eq!(
        ::core::mem::align_of::<mutex_t>(),
        8usize,
        concat!("Alignment of ", stringify!(mutex_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<mutex_t>())).queue as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mutex_t),
            "::",
            stringify!(queue)
        )
    );
}
extern "C" {
    /// @brief Lock a mutex, blocking or non-blocking.
    ///
    /// @details For commit purposes you should probably use mutex_trylock() and
    /// mutex_lock() instead.
    ///
    /// @param[in] mutex         Mutex object to lock. Has to be initialized first.
    /// Must not be NULL.
    /// @param[in] blocking      if true, block until mutex is available.
    ///
    /// @return 1 if mutex was unlocked, now it is locked.
    /// @return 0 if the mutex was locked.
    pub fn _mutex_lock(mutex: *mut mutex_t, blocking: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// @brief Unlocks the mutex.
    ///
    /// @param[in] mutex Mutex object to unlock, must not be NULL.
    pub fn mutex_unlock(mutex: *mut mutex_t);
}
extern "C" {
    /// @brief Unlocks the mutex and sends the current thread to sleep
    ///
    /// @param[in] mutex Mutex object to unlock, must not be NULL.
    pub fn mutex_unlock_and_sleep(mutex: *mut mutex_t);
}
/// @brief Mutex structure. Must never be modified by the user.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rmutex_t {
    /// @brief The mutex used for locking. **Must never be changed by
    /// the user.**
    /// @internal
    pub mutex: mutex_t,
    /// @brief   Number of locks owned by the thread owner
    /// @internal
    pub refcount: u16,
    /// @brief   Owner thread of the mutex.
    /// @details Owner is written by the mutex holder, and read
    /// concurrently to ensure consistency,
    /// atomic_int_least16_t is used. Note @ref kernel_pid_t is an int16
    /// @internal
    pub owner: atomic_int_least16_t,
}
#[test]
fn bindgen_test_layout_rmutex_t() {
    assert_eq!(
        ::core::mem::size_of::<rmutex_t>(),
        16usize,
        concat!("Size of: ", stringify!(rmutex_t))
    );
    assert_eq!(
        ::core::mem::align_of::<rmutex_t>(),
        8usize,
        concat!("Alignment of ", stringify!(rmutex_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<rmutex_t>())).mutex as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rmutex_t),
            "::",
            stringify!(mutex)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<rmutex_t>())).refcount as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rmutex_t),
            "::",
            stringify!(refcount)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<rmutex_t>())).owner as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(rmutex_t),
            "::",
            stringify!(owner)
        )
    );
}
extern "C" {
    /// @brief Tries to get a recursive mutex, non-blocking.
    ///
    /// @param[in] rmutex Recursive mutex object to lock. Has to be
    /// initialized first. Must not be NULL.
    ///
    /// @return 1 if mutex was unlocked, now it is locked.
    /// @return 0 if the mutex was locked.
    pub fn rmutex_trylock(rmutex: *mut rmutex_t) -> libc::c_int;
}
extern "C" {
    /// @brief Locks a recursive mutex, blocking.
    ///
    /// @param[in] rmutex Recursive mutex object to lock. Has to be
    /// initialized first. Must not be NULL.
    pub fn rmutex_lock(rmutex: *mut rmutex_t);
}
extern "C" {
    /// @brief Unlocks the recursive mutex.
    ///
    /// @param[in] rmutex Recursive mutex object to unlock, must not be NULL.
    pub fn rmutex_unlock(rmutex: *mut rmutex_t);
}
/// @brief   Operations to an interface
pub type gnrc_netif_ops_t = gnrc_netif_ops;
/// @brief   Representation of a network interface
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gnrc_netif_t {
    /// < Operations of the network interface
    pub ops: *const gnrc_netif_ops_t,
    /// < Network device of the network interface
    pub dev: *mut netdev_t,
    /// < Mutex of the interface
    pub mutex: rmutex_t,
    /// @brief   Flags for the interface
    ///
    /// @see net_gnrc_netif_flags
    pub flags: u32,
    /// @brief   The link-layer address currently used as the source address
    /// on this interface.
    ///
    /// @note    Only available if @ref GNRC_NETIF_L2ADDR_MAXLEN > 0
    pub l2addr: [u8; 8usize],
    /// @brief   Length in bytes of gnrc_netif_t::l2addr
    ///
    /// @note    Only available if @ref GNRC_NETIF_L2ADDR_MAXLEN > 0
    pub l2addr_len: u8,
    /// < Current hop-limit for out-going packets
    pub cur_hl: u8,
    /// < Device type
    pub device_type: u8,
    /// < PID of the network interface's thread
    pub pid: kernel_pid_t,
}
#[test]
fn bindgen_test_layout_gnrc_netif_t() {
    assert_eq!(
        ::core::mem::size_of::<gnrc_netif_t>(),
        56usize,
        concat!("Size of: ", stringify!(gnrc_netif_t))
    );
    assert_eq!(
        ::core::mem::align_of::<gnrc_netif_t>(),
        8usize,
        concat!("Alignment of ", stringify!(gnrc_netif_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_t>())).ops as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_t),
            "::",
            stringify!(ops)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_t>())).dev as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_t),
            "::",
            stringify!(dev)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_t>())).mutex as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_t),
            "::",
            stringify!(mutex)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_t>())).flags as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_t),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_t>())).l2addr as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_t),
            "::",
            stringify!(l2addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_t>())).l2addr_len as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_t),
            "::",
            stringify!(l2addr_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_t>())).cur_hl as *const _ as usize },
        45usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_t),
            "::",
            stringify!(cur_hl)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_t>())).device_type as *const _ as usize },
        46usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_t),
            "::",
            stringify!(device_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_t>())).pid as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_t),
            "::",
            stringify!(pid)
        )
    );
}
/// @see gnrc_netif_ops_t
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gnrc_netif_ops {
    /// @brief   Initializes network interface beyond the default settings
    ///
    /// @pre `netif != NULL`
    ///
    /// @param[in] netif The network interface.
    ///
    /// This is called after the default settings were set, right before the
    /// interface's thread starts receiving messages. It is not necessary to lock
    /// the interface's mutex gnrc_netif_t::mutex, since the thread will already
    /// lock it. Leave NULL if you do not need any special initialization.
    pub init: ::core::option::Option<unsafe extern "C" fn(netif: *mut gnrc_netif_t)>,
    /// @brief   Send a @ref net_gnrc_pkt "packet" over the network interface
    ///
    /// @pre `netif != NULL && pkt != NULL`
    ///
    /// @note The function re-formats the content of @p pkt to a format expected
    /// by the netdev_driver_t::send() method of gnrc_netif_t::dev and
    /// releases the packet before returning (so no additional release
    /// should be required after calling this method).
    ///
    /// @param[in] netif The network interface.
    /// @param[in] pkt   A packet to send.
    ///
    /// @return  The number of bytes actually sent on success
    /// @return  -EBADMSG, if the @ref net_gnrc_netif_hdr in @p pkt is missing
    /// or is in an unexpected format.
    /// @return  -ENOTSUP, if sending @p pkt in the given format isn't supported
    /// (e.g. empty payload with Ethernet).
    /// @return  Any negative error code reported by gnrc_netif_t::dev.
    pub send: ::core::option::Option<
        unsafe extern "C" fn(netif: *mut gnrc_netif_t, pkt: *mut gnrc_pktsnip_t) -> libc::c_int,
    >,
    /// @brief   Receives a @ref net_gnrc_pkt "packet" from the network interface
    ///
    /// @pre `netif != NULL`
    ///
    /// @note The function takes the bytes received via netdev_driver_t::recv()
    /// from gnrc_netif_t::dev and re-formats it to a
    /// @ref net_gnrc_pkt "packet" containing a @ref net_gnrc_netif_hdr
    /// and a payload header in receive order.
    ///
    /// @param[in] netif The network interface.
    ///
    /// @return  The packet received. Contains the payload (with the type marked
    /// accordingly) and a @ref net_gnrc_netif_hdr in receive order.
    /// @return  NULL, if @ref net_gnrc_pktbuf was full.
    pub recv: ::core::option::Option<
        unsafe extern "C" fn(netif: *mut gnrc_netif_t) -> *mut gnrc_pktsnip_t,
    >,
    /// @brief   Gets an option from the network interface
    ///
    /// Use gnrc_netif_get_from_netdev() to just get options from
    /// gnrc_netif_t::dev.
    ///
    /// @param[in] netif     The network interface.
    /// @param[in] opt       The option parameters.
    ///
    /// @return  Number of bytes in @p data.
    /// @return  -EOVERFLOW, if @p max_len is lesser than the required space.
    /// @return  -ENOTSUP, if @p opt is not supported to be set.
    /// @return  Any negative error code reported by gnrc_netif_t::dev.
    pub get: ::core::option::Option<
        unsafe extern "C" fn(netif: *mut gnrc_netif_t, opt: *mut gnrc_netapi_opt_t) -> libc::c_int,
    >,
    /// @brief  Sets an option from the network interface
    ///
    /// Use gnrc_netif_set_from_netdev() to just set options from
    /// gnrc_netif_t::dev.
    ///
    /// @param[in] netif     The network interface.
    /// @param[in] opt       The option parameters.
    ///
    /// @return  Number of bytes written to gnrc_netif_t::dev.
    /// @return  -EOVERFLOW, if @p data_len is greater than the allotted space in
    /// gnrc_netif_t::dev or gnrc_netif_t.
    /// @return  -ENOTSUP, if @p opt is not supported to be set.
    /// @return  Any negative error code reported by gnrc_netif_t::dev.
    pub set: ::core::option::Option<
        unsafe extern "C" fn(netif: *mut gnrc_netif_t, opt: *const gnrc_netapi_opt_t)
            -> libc::c_int,
    >,
    /// @brief   Message handler for network interface
    ///
    /// This message handler is used, when the network interface needs to handle
    /// message types beyond the ones defined in @ref net_gnrc_netapi "netapi".
    /// Leave NULL if this is not the case.
    ///
    /// @param[in] netif The network interface.
    /// @param[in] msg   Message to be handled.
    pub msg_handler:
        ::core::option::Option<unsafe extern "C" fn(netif: *mut gnrc_netif_t, msg: *mut msg_t)>,
}
#[test]
fn bindgen_test_layout_gnrc_netif_ops() {
    assert_eq!(
        ::core::mem::size_of::<gnrc_netif_ops>(),
        48usize,
        concat!("Size of: ", stringify!(gnrc_netif_ops))
    );
    assert_eq!(
        ::core::mem::align_of::<gnrc_netif_ops>(),
        8usize,
        concat!("Alignment of ", stringify!(gnrc_netif_ops))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_ops>())).init as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_ops),
            "::",
            stringify!(init)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_ops>())).send as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_ops),
            "::",
            stringify!(send)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_ops>())).recv as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_ops),
            "::",
            stringify!(recv)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_ops>())).get as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_ops),
            "::",
            stringify!(get)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_ops>())).set as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_ops),
            "::",
            stringify!(set)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_ops>())).msg_handler as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_ops),
            "::",
            stringify!(msg_handler)
        )
    );
}
extern "C" {
    /// @brief   Creates a network interface
    ///
    /// @param[in] stack     The stack for the network interface's thread.
    /// @param[in] stacksize Size of @p stack.
    /// @param[in] priority  Priority for the network interface's thread.
    /// @param[in] name      Name for the network interface. May be NULL.
    /// @param[in] dev       Device for the interface.
    /// @param[in] ops       Operations for the network interface.
    ///
    /// @note If @ref DEVELHELP is defined netif_params_t::name is used as the
    /// name of the network interface's thread.
    ///
    /// @attention   Fails and crashes (assertion error with @ref DEVELHELP or
    /// segmentation fault without) if `GNRC_NETIF_NUMOF` is lower than
    /// the number of calls to this function.
    ///
    /// @return  The network interface on success.
    pub fn gnrc_netif_create(
        stack: *mut libc::c_char,
        stacksize: libc::c_int,
        priority: libc::c_char,
        name: *const libc::c_char,
        dev: *mut netdev_t,
        ops: *const gnrc_netif_ops_t,
    ) -> *mut gnrc_netif_t;
}
extern "C" {
    /// @brief   Get number of network interfaces actually allocated
    ///
    /// @return  Number of network interfaces actually allocated
    pub fn gnrc_netif_numof() -> libc::c_uint;
}
extern "C" {
    /// @brief   Iterate over all network interfaces.
    ///
    /// @param[in] prev  previous interface in iteration. NULL to start iteration.
    ///
    /// @return  The next network interface after @p prev.
    /// @return  NULL, if @p prev was the last network interface.
    pub fn gnrc_netif_iter(prev: *const gnrc_netif_t) -> *mut gnrc_netif_t;
}
extern "C" {
    /// @brief   Get network interface by PID
    ///
    /// @param[in] pid   A PID of a network interface.
    ///
    /// @return  The network interface on success.
    /// @return  NULL, if no network interface with PID exists.
    pub fn gnrc_netif_get_by_pid(pid: kernel_pid_t) -> *mut gnrc_netif_t;
}
extern "C" {
    /// @brief   Default operation for gnrc_netif_ops_t::get()
    ///
    /// @note    Can also be used to be called *after* a custom operation.
    ///
    /// @param[in] netif     The network interface.
    /// @param[out] opt      The option parameters.
    ///
    /// @return  Return value of netdev_driver_t::get() of gnrc_netif_t::dev of
    /// @p netif.
    pub fn gnrc_netif_get_from_netdev(
        netif: *mut gnrc_netif_t,
        opt: *mut gnrc_netapi_opt_t,
    ) -> libc::c_int;
}
extern "C" {
    /// @brief   Default operation for gnrc_netif_ops_t::set()
    ///
    /// @note    Can also be used to be called *after* a custom operation.
    ///
    /// @param[in] netif     The network interface.
    /// @param[in] opt       The option parameters.
    ///
    /// @return  Return value of netdev_driver_t::set() of gnrc_netif_t::dev of
    /// @p netif.
    pub fn gnrc_netif_set_from_netdev(
        netif: *mut gnrc_netif_t,
        opt: *const gnrc_netapi_opt_t,
    ) -> libc::c_int;
}
extern "C" {
    /// @brief   Converts a hardware address to a human readable string.
    ///
    /// @details The format will be like `xx:xx:xx:xx` where `xx` are the bytes
    /// of @p addr in hexadecimal representation.
    ///
    /// @pre `(out != NULL) && ((addr != NULL) || (addr_len == 0))`
    /// @pre @p out **MUST** have allocated at least 3 * @p addr_len bytes.
    ///
    /// @param[in] addr      A hardware address.
    /// @param[in] addr_len  Length of @p addr.
    /// @param[out] out      A string to store the output in. Must at least have
    /// 3 * @p addr_len bytes allocated.
    ///
    /// @return  @p out.
    pub fn gnrc_netif_addr_to_str(
        addr: *const u8,
        addr_len: usize,
        out: *mut libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C" {
    /// @brief   Parses a string of colon-separated hexadecimals to a hardware
    /// address.
    ///
    /// @details The input format must be like `xx:xx:xx:xx` where `xx` will be the
    /// bytes of @p addr in hexadecimal representation.
    ///
    /// @pre `(out != NULL)`
    /// @pre @p out **MUST** have allocated at least
    /// @ref GNRC_NETIF_L2ADDR_MAXLEN bytes.
    ///
    /// @param[in] str       A string of colon-separated hexadecimals.
    /// @param[out] out      The resulting hardware address. Must at least have
    /// @ref GNRC_NETIF_L2ADDR_MAXLEN bytes allocated.
    ///
    /// @return  Actual length of @p out on success.
    /// @return  0, on failure.
    pub fn gnrc_netif_addr_from_str(str: *const libc::c_char, out: *mut u8) -> usize;
}
extern "C" {
    pub fn __errno_location() -> *mut libc::c_int;
}
extern "C" {
    /// @brief   Initializes packet buffer module.
    pub fn gnrc_pktbuf_init();
}
extern "C" {
    /// @brief   Adds a new gnrc_pktsnip_t and its packet to the packet buffer.
    ///
    /// @warning **Do not** change the fields of the gnrc_pktsnip_t created by this
    /// function externally. This will most likely create memory leaks or
    /// not allowed memory access.
    ///
    /// @pre size < GNRC_PKTBUF_SIZE
    ///
    /// @param[in] next      Next gnrc_pktsnip_t in the packet. Leave NULL if you
    /// want to create a new packet.
    /// @param[in] data      Data of the new gnrc_pktsnip_t. If @p data is NULL no data
    /// will be inserted into `result`.
    /// @param[in] size      Length of @p data. If this value is 0 the
    /// gnrc_pktsnip::data field of the newly created snip will
    /// be NULL.
    /// @param[in] type      Protocol type of the gnrc_pktsnip_t.
    ///
    /// @return  Pointer to the packet part that represents the new gnrc_pktsnip_t.
    /// @return  NULL, if no space is left in the packet buffer.
    pub fn gnrc_pktbuf_add(
        next: *mut gnrc_pktsnip_t,
        data: *const libc::c_void,
        size: usize,
        type_: gnrc_nettype_t,
    ) -> *mut gnrc_pktsnip_t;
}
extern "C" {
    /// @brief   Marks the first @p size bytes in a received packet with a new
    /// packet snip that is appended to the packet.
    ///
    /// Graphically this can be represented as follows:
    ///
    /// ~~~~~~~~~~~~~~~~~~~
    /// Before                                    After
    /// ======                                    =====
    /// (next)
    /// pkt->data                                 result->data <== pkt->data
    /// v                                         v                v
    /// +--------------------------------+        +----------------+---------------+
    /// +--------------------------------+        +----------------+---------------+
    /// \__________pkt->size___________/          \_result->size_/ \__pkt->size__/
    /// ~~~~~~~~~~~~~~~~~~~
    ///
    /// If `size == pkt->size` then the resulting snip will point to NULL in its
    /// gnrc_pktsnip_t::data field and its gnrc_pktsnip_t::size field will be 0.
    ///
    /// @pre @p pkt != NULL && @p size != 0
    ///
    /// @param[in] pkt   A received packet.
    /// @param[in] size  The size of the new packet snip.
    /// @param[in] type  The type of the new packet snip.
    ///
    /// @return  The new packet snip in @p pkt on success.
    /// @return  NULL, if pkt == NULL or size == 0 or size > pkt->size or pkt->data == NULL.
    /// @return  NULL, if no space is left in the packet buffer.
    pub fn gnrc_pktbuf_mark(
        pkt: *mut gnrc_pktsnip_t,
        size: usize,
        type_: gnrc_nettype_t,
    ) -> *mut gnrc_pktsnip_t;
}
extern "C" {
    /// @brief   Reallocates gnrc_pktsnip_t::data of @p pkt in the packet buffer, without
    /// changing the content.
    ///
    /// @pre `pkt != NULL`
    /// @pre `(pkt->size > 0) <=> (pkt->data != NULL)`
    /// @pre gnrc_pktsnip_t::data of @p pkt is in the packet buffer if it is not NULL.
    ///
    /// @details If enough memory is available behind it or @p size is smaller than
    /// the original size of the packet then gnrc_pktsnip_t::data of @p pkt will
    /// not be moved. Otherwise, it will be moved. If no space is available
    /// nothing happens.
    ///
    /// @param[in] pkt   A packet part.
    /// @param[in] size  The size for @p pkt.
    ///
    /// @return  0, on success
    /// @return  ENOMEM, if no space is left in the packet buffer.
    pub fn gnrc_pktbuf_realloc_data(pkt: *mut gnrc_pktsnip_t, size: usize) -> libc::c_int;
}
extern "C" {
    /// @brief   Increases gnrc_pktsnip_t::users of @p pkt atomically.
    ///
    /// @param[in] pkt   A packet.
    /// @param[in] num   Number you want to increment gnrc_pktsnip_t::users of @p pkt by.
    pub fn gnrc_pktbuf_hold(pkt: *mut gnrc_pktsnip_t, num: libc::c_uint);
}
extern "C" {
    /// @brief   Decreases gnrc_pktsnip_t::users of @p pkt atomically and removes it if it
    /// reaches 0 and reports a possible error through an error code, if
    /// @ref net_gnrc_neterr is included.
    ///
    /// @pre All snips of @p pkt must be in the packet buffer.
    ///
    /// @param[in] pkt   A packet.
    /// @param[in] err   An error code.
    pub fn gnrc_pktbuf_release_error(pkt: *mut gnrc_pktsnip_t, err: u32);
}
extern "C" {
    /// @brief   Must be called once before there is a write operation in a thread.
    ///
    /// @details This function duplicates a packet in the packet buffer if
    /// gnrc_pktsnip_t::users of @p pkt > 1.
    ///
    /// @note    Do *not* call this function in a thread twice on the same packet.
    ///
    /// @param[in] pkt   The packet you want to write into.
    ///
    /// @return  The (new) pointer to the pkt.
    /// @return  NULL, if gnrc_pktsnip_t::users of @p pkt > 1 and if there is not
    /// enough space in the packet buffer.
    pub fn gnrc_pktbuf_start_write(pkt: *mut gnrc_pktsnip_t) -> *mut gnrc_pktsnip_t;
}
extern "C" {
    /// @brief   Create a IOVEC representation of the packet pointed to by *pkt*
    ///
    /// @pre `(len != NULL)`
    ///
    /// @details This function will create a new packet snip in the packet buffer,
    /// which points to the given *pkt* and contains a IOVEC representation
    /// of the referenced packet in its data section.
    ///
    /// @param[in]  pkt  Packet to export as IOVEC
    /// @param[out] len  Number of elements in the IOVEC
    ///
    /// @return  Pointer to the 'IOVEC packet snip'
    /// @return  NULL, if packet is empty of the packet buffer is full
    pub fn gnrc_pktbuf_get_iovec(pkt: *mut gnrc_pktsnip_t, len: *mut usize) -> *mut gnrc_pktsnip_t;
}
extern "C" {
    /// @brief   Deletes a snip from a packet and the packet buffer.
    ///
    /// @param[in] pkt   A packet.
    /// @param[in] snip  A snip in the packet.
    ///
    /// @return  The new reference to @p pkt.
    pub fn gnrc_pktbuf_remove_snip(
        pkt: *mut gnrc_pktsnip_t,
        snip: *mut gnrc_pktsnip_t,
    ) -> *mut gnrc_pktsnip_t;
}
extern "C" {
    /// @brief   Replace a snip from a packet and the packet buffer by another snip.
    ///
    /// @param[in] pkt   A packet
    /// @param[in] old   snip currently in the packet
    /// @param[in] add   snip which will replace old
    ///
    /// @return  The new reference to @p pkt
    pub fn gnrc_pktbuf_replace_snip(
        pkt: *mut gnrc_pktsnip_t,
        old: *mut gnrc_pktsnip_t,
        add: *mut gnrc_pktsnip_t,
    ) -> *mut gnrc_pktsnip_t;
}
extern "C" {
    /// @brief Duplicates pktsnip chain upto (including) a snip with the given type
    /// as a continuous snip.
    ///
    /// Example:
    /// Input:
    /// buffer
    /// +---------------------------+                      +------+
    /// | size = 8                  | data       +-------->|      |
    /// | type = NETTYPE_IPV6_EXT   |------------+         +------+
    /// +---------------------------+                      .      .
    /// | next                                       .      .
    /// v                                            .      .
    /// +---------------------------+                      +------+
    /// | size = 40                 | data    +----------->|      |
    /// | type = NETTYPE_IPV6       |---------+            +------+
    /// +---------------------------+                      .      .
    /// | next                                       .      .
    /// v
    /// +---------------------------+                      +------+
    /// | size = 14                 | data +-------------->|      |
    /// | type = NETTYPE_NETIF      |------+               +------+
    /// +---------------------------+                      .      .
    ///
    ///
    /// Output:
    /// buffer
    /// +---------------------------+                      +------+
    /// | size = 48                 | data       +-------->|      |
    /// | type = NETTYPE_IPV6       |------------+         |      |
    /// +---------------------------+                      |      |
    /// |                                            +------+
    /// |                                            .      .
    /// | next                                       .      .
    /// v
    /// +---------------------------+                      +------+
    /// | size = 14                 | data +-------------->|      |
    /// | type = NETTYPE_NETIF      |------+               +------+
    /// +---------------------------+                      .      .
    ///
    /// The original snip is keeped as is except `users` decremented.
    ///
    /// @param[in,out] pkt   The snip to duplicate.
    /// @param[in]     type  The type of snip to stop duplication.
    ///
    /// @return The duplicated snip, if succeeded.
    /// @return NULL, if no space is left in the packet buffer.
    pub fn gnrc_pktbuf_duplicate_upto(
        pkt: *mut gnrc_pktsnip_t,
        type_: gnrc_nettype_t,
    ) -> *mut gnrc_pktsnip_t;
}
/// @brief   Generic network interface header
///
/// The link layer addresses included in this header are put in memory directly
/// following this struct.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gnrc_netif_hdr_t {
    /// < length of l2 source address in byte
    pub src_l2addr_len: u8,
    /// < length of l2 destination address in byte
    pub dst_l2addr_len: u8,
    /// < PID of network interface
    pub if_pid: kernel_pid_t,
    /// < flags as defined above
    pub flags: u8,
    /// < lqi of received packet (optional)
    pub lqi: u8,
    /// < rssi of received packet in dBm (optional)
    pub rssi: i16,
}
#[test]
fn bindgen_test_layout_gnrc_netif_hdr_t() {
    assert_eq!(
        ::core::mem::size_of::<gnrc_netif_hdr_t>(),
        8usize,
        concat!("Size of: ", stringify!(gnrc_netif_hdr_t))
    );
    assert_eq!(
        ::core::mem::align_of::<gnrc_netif_hdr_t>(),
        2usize,
        concat!("Alignment of ", stringify!(gnrc_netif_hdr_t))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<gnrc_netif_hdr_t>())).src_l2addr_len as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_hdr_t),
            "::",
            stringify!(src_l2addr_len)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<gnrc_netif_hdr_t>())).dst_l2addr_len as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_hdr_t),
            "::",
            stringify!(dst_l2addr_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_hdr_t>())).if_pid as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_hdr_t),
            "::",
            stringify!(if_pid)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_hdr_t>())).flags as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_hdr_t),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_hdr_t>())).lqi as *const _ as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_hdr_t),
            "::",
            stringify!(lqi)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<gnrc_netif_hdr_t>())).rssi as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(gnrc_netif_hdr_t),
            "::",
            stringify!(rssi)
        )
    );
}
extern "C" {
    /// @brief   Builds a generic network interface header for sending and
    /// adds it to the packet buffer.
    ///
    /// @param[in] src       Source address for the header. Can be NULL if not
    /// known or required.
    /// @param[in] src_len   Length of @p src. Can be 0 if not known or required.
    /// @param[in] dst       Destination address for the header. Can be NULL if not
    /// known or required.
    /// @param[in] dst_len   Length of @p dst. Can be 0 if not known or required.
    ///
    /// @return  The generic network layer header on success.
    /// @return  NULL on error.
    pub fn gnrc_netif_hdr_build(
        src: *mut u8,
        src_len: u8,
        dst: *mut u8,
        dst_len: u8,
    ) -> *mut gnrc_pktsnip_t;
}
extern "C" {
    /// @brief   Outputs a generic interface header to stdout.
    ///
    /// @param[in] hdr   A generic interface header.
    pub fn gnrc_netif_hdr_print(hdr: *mut gnrc_netif_hdr_t);
}
extern "C" {
    /// @brief   Fetch the netif header flags of a gnrc packet
    ///
    /// @param[in]   pkt     gnrc packet from whom to fetch
    ///
    /// @return              netif header flags of @p pkt
    /// @return              0, if no header is present
    pub fn gnrc_netif_hdr_get_flag(pkt: *mut gnrc_pktsnip_t) -> u8;
}
extern "C" {
    /// @brief   Extract the destination address out of a gnrc packet
    ///
    /// @param[in]   pkt                 gnrc packet from whom to extract
    /// @param[out]  pointer_to_addr     pointer to address will be stored here
    ///
    /// @return                          length of destination address
    /// @return                          -ENOENT, if no netif header is presented in @p pkt or if no
    /// destination address field presented in netif header.
    pub fn gnrc_netif_hdr_get_dstaddr(
        pkt: *mut gnrc_pktsnip_t,
        pointer_to_addr: *mut *mut u8,
    ) -> libc::c_int;
}
extern "C" {
    /// @brief   Extract the source address out of a gnrc packet
    ///
    /// @param[in]   pkt                 gnrc packet from whom to extract
    /// @param[out]  pointer_to_addr     pointer to address will be stored here
    ///
    /// @return                          length of source address
    /// @return                          -ENOENT, if no netif header is presented in @p pkt or if no
    /// source address field presented in netif header.
    pub fn gnrc_netif_hdr_get_srcaddr(
        pkt: *mut gnrc_pktsnip_t,
        pointer_to_addr: *mut *mut u8,
    ) -> libc::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_data {
    pub _address: u8,
}
