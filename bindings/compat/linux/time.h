/*
 * Minimal POSIX declarations required by PDFium's public fpdf_ext.h when
 * generating Linux ABI bindings on a host without a Linux sysroot.
 * Linux x86_64 and aarch64 both use LP64, so time_t is a signed long.
 */
typedef long time_t;
struct tm;
