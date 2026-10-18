/*
 * Minimal Windows SDK declaration needed to audit PDFium's public headers on
 * hosts without a Windows SDK. The symbol audit only needs HDC's opaque handle
 * type for the legacy FPDF_RenderPage declaration; this is not used to build
 * the library or generated bindings.
 */
typedef struct HDC__* HDC;
