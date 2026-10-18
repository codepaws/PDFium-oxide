// Generated from the official PDFium public C headers. Do not edit by hand.
// Source: f51279699cb59754d1f49aef10eb8e825fe56532; configuration: linux-arm64-default.
use super::types::*;
pub(crate) struct pdfium {
    __library: ::libloading::Library,
    pub FPDF_InitLibraryWithConfig:
        Result<unsafe extern "C" fn(config: *const FPDF_LIBRARY_CONFIG), ::libloading::Error>,
    pub FPDF_InitLibrary: Result<unsafe extern "C" fn(), ::libloading::Error>,
    pub FPDF_DestroyLibrary: Result<unsafe extern "C" fn(), ::libloading::Error>,
    pub FPDF_SetSandBoxPolicy:
        Result<unsafe extern "C" fn(policy: FPDF_DWORD, enable: FPDF_BOOL), ::libloading::Error>,
    pub FPDF_LoadDocument: Result<
        unsafe extern "C" fn(file_path: FPDF_STRING, password: FPDF_BYTESTRING) -> FPDF_DOCUMENT,
        ::libloading::Error,
    >,
    pub FPDF_LoadMemDocument: Result<
        unsafe extern "C" fn(
            data_buf: *const ::core::ffi::c_void,
            size: ::core::ffi::c_int,
            password: FPDF_BYTESTRING,
        ) -> FPDF_DOCUMENT,
        ::libloading::Error,
    >,
    pub FPDF_LoadMemDocument64: Result<
        unsafe extern "C" fn(
            data_buf: *const ::core::ffi::c_void,
            size: usize,
            password: FPDF_BYTESTRING,
        ) -> FPDF_DOCUMENT,
        ::libloading::Error,
    >,
    pub FPDF_LoadCustomDocument: Result<
        unsafe extern "C" fn(
            pFileAccess: *mut FPDF_FILEACCESS,
            password: FPDF_BYTESTRING,
        ) -> FPDF_DOCUMENT,
        ::libloading::Error,
    >,
    pub FPDF_GetFileVersion: Result<
        unsafe extern "C" fn(doc: FPDF_DOCUMENT, fileVersion: *mut ::core::ffi::c_int) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDF_GetLastError:
        Result<unsafe extern "C" fn() -> ::core::ffi::c_ulong, ::libloading::Error>,
    pub FPDF_DocumentHasValidCrossReferenceTable:
        Result<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_BOOL, ::libloading::Error>,
    pub FPDF_GetTrailerEnds: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            buffer: *mut ::core::ffi::c_uint,
            length: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_GetDocPermissions: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_GetDocUserPermissions: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_GetSecurityHandlerRevision: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_GetPageCount: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_LoadPage: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT, page_index: ::core::ffi::c_int) -> FPDF_PAGE,
        ::libloading::Error,
    >,
    pub FPDF_GetPageWidthF:
        Result<unsafe extern "C" fn(page: FPDF_PAGE) -> f32, ::libloading::Error>,
    pub FPDF_GetPageWidth:
        Result<unsafe extern "C" fn(page: FPDF_PAGE) -> f64, ::libloading::Error>,
    pub FPDF_GetPageHeightF:
        Result<unsafe extern "C" fn(page: FPDF_PAGE) -> f32, ::libloading::Error>,
    pub FPDF_GetPageHeight:
        Result<unsafe extern "C" fn(page: FPDF_PAGE) -> f64, ::libloading::Error>,
    pub FPDF_GetPageBoundingBox: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, rect: *mut FS_RECTF) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDF_GetPageSizeByIndexF: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            page_index: ::core::ffi::c_int,
            size: *mut FS_SIZEF,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDF_GetPageSizeByIndex: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            page_index: ::core::ffi::c_int,
            width: *mut f64,
            height: *mut f64,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_RenderPageBitmap: Result<
        unsafe extern "C" fn(
            bitmap: FPDF_BITMAP,
            page: FPDF_PAGE,
            start_x: ::core::ffi::c_int,
            start_y: ::core::ffi::c_int,
            size_x: ::core::ffi::c_int,
            size_y: ::core::ffi::c_int,
            rotate: ::core::ffi::c_int,
            flags: ::core::ffi::c_int,
        ),
        ::libloading::Error,
    >,
    pub FPDF_RenderPageBitmapWithMatrix: Result<
        unsafe extern "C" fn(
            bitmap: FPDF_BITMAP,
            page: FPDF_PAGE,
            matrix: *const FS_MATRIX,
            clipping: *const FS_RECTF,
            flags: ::core::ffi::c_int,
        ),
        ::libloading::Error,
    >,
    pub FPDF_ClosePage: Result<unsafe extern "C" fn(page: FPDF_PAGE), ::libloading::Error>,
    pub FPDF_CloseDocument:
        Result<unsafe extern "C" fn(document: FPDF_DOCUMENT), ::libloading::Error>,
    pub FPDF_DeviceToPage: Result<
        unsafe extern "C" fn(
            page: FPDF_PAGE,
            start_x: ::core::ffi::c_int,
            start_y: ::core::ffi::c_int,
            size_x: ::core::ffi::c_int,
            size_y: ::core::ffi::c_int,
            rotate: ::core::ffi::c_int,
            device_x: ::core::ffi::c_int,
            device_y: ::core::ffi::c_int,
            page_x: *mut f64,
            page_y: *mut f64,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDF_PageToDevice: Result<
        unsafe extern "C" fn(
            page: FPDF_PAGE,
            start_x: ::core::ffi::c_int,
            start_y: ::core::ffi::c_int,
            size_x: ::core::ffi::c_int,
            size_y: ::core::ffi::c_int,
            rotate: ::core::ffi::c_int,
            page_x: f64,
            page_y: f64,
            device_x: *mut ::core::ffi::c_int,
            device_y: *mut ::core::ffi::c_int,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFBitmap_Create: Result<
        unsafe extern "C" fn(
            width: ::core::ffi::c_int,
            height: ::core::ffi::c_int,
            alpha: ::core::ffi::c_int,
        ) -> FPDF_BITMAP,
        ::libloading::Error,
    >,
    pub FPDFBitmap_CreateEx: Result<
        unsafe extern "C" fn(
            width: ::core::ffi::c_int,
            height: ::core::ffi::c_int,
            format: ::core::ffi::c_int,
            first_scan: *mut ::core::ffi::c_void,
            stride: ::core::ffi::c_int,
        ) -> FPDF_BITMAP,
        ::libloading::Error,
    >,
    pub FPDFBitmap_GetFormat: Result<
        unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFBitmap_FillRect: Result<
        unsafe extern "C" fn(
            bitmap: FPDF_BITMAP,
            left: ::core::ffi::c_int,
            top: ::core::ffi::c_int,
            width: ::core::ffi::c_int,
            height: ::core::ffi::c_int,
            color: FPDF_DWORD,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFBitmap_GetBuffer: Result<
        unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> *mut ::core::ffi::c_void,
        ::libloading::Error,
    >,
    pub FPDFBitmap_GetWidth: Result<
        unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFBitmap_GetHeight: Result<
        unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFBitmap_GetStride: Result<
        unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFBitmap_Destroy: Result<unsafe extern "C" fn(bitmap: FPDF_BITMAP), ::libloading::Error>,
    pub FPDF_VIEWERREF_GetPrintScaling:
        Result<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_BOOL, ::libloading::Error>,
    pub FPDF_VIEWERREF_GetNumCopies: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_VIEWERREF_GetPrintPageRange: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_PAGERANGE,
        ::libloading::Error,
    >,
    pub FPDF_VIEWERREF_GetPrintPageRangeCount:
        Result<unsafe extern "C" fn(pagerange: FPDF_PAGERANGE) -> usize, ::libloading::Error>,
    pub FPDF_VIEWERREF_GetPrintPageRangeElement: Result<
        unsafe extern "C" fn(pagerange: FPDF_PAGERANGE, index: usize) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_VIEWERREF_GetDuplex: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_DUPLEXTYPE,
        ::libloading::Error,
    >,
    pub FPDF_VIEWERREF_GetName: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            key: FPDF_BYTESTRING,
            buffer: *mut ::core::ffi::c_char,
            length: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_CountNamedDests:
        Result<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_DWORD, ::libloading::Error>,
    pub FPDF_GetNamedDestByName: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT, name: FPDF_BYTESTRING) -> FPDF_DEST,
        ::libloading::Error,
    >,
    pub FPDF_GetNamedDest: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            index: ::core::ffi::c_int,
            buffer: *mut ::core::ffi::c_void,
            buflen: *mut ::core::ffi::c_long,
        ) -> FPDF_DEST,
        ::libloading::Error,
    >,
    pub FPDF_GetXFAPacketCount: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_GetXFAPacketName: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            index: ::core::ffi::c_int,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_GetXFAPacketContent: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            index: ::core::ffi::c_int,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
            out_buflen: *mut ::core::ffi::c_ulong,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFDOC_InitFormFillEnvironment: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            formInfo: *mut FPDF_FORMFILLINFO,
        ) -> FPDF_FORMHANDLE,
        ::libloading::Error,
    >,
    pub FPDFDOC_ExitFormFillEnvironment:
        Result<unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE), ::libloading::Error>,
    pub FORM_OnAfterLoadPage: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, hHandle: FPDF_FORMHANDLE),
        ::libloading::Error,
    >,
    pub FORM_OnBeforeClosePage: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, hHandle: FPDF_FORMHANDLE),
        ::libloading::Error,
    >,
    pub FORM_DoDocumentJSAction:
        Result<unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE), ::libloading::Error>,
    pub FORM_DoDocumentOpenAction:
        Result<unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE), ::libloading::Error>,
    pub FORM_DoDocumentAAction: Result<
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, aaType: ::core::ffi::c_int),
        ::libloading::Error,
    >,
    pub FORM_DoPageAAction: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, hHandle: FPDF_FORMHANDLE, aaType: ::core::ffi::c_int),
        ::libloading::Error,
    >,
    pub FORM_OnMouseMove: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            page: FPDF_PAGE,
            modifier: ::core::ffi::c_int,
            page_x: f64,
            page_y: f64,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_OnMouseWheel: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            page: FPDF_PAGE,
            modifier: ::core::ffi::c_int,
            page_coord: *const FS_POINTF,
            delta_x: ::core::ffi::c_int,
            delta_y: ::core::ffi::c_int,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_OnFocus: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            page: FPDF_PAGE,
            modifier: ::core::ffi::c_int,
            page_x: f64,
            page_y: f64,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_OnLButtonDown: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            page: FPDF_PAGE,
            modifier: ::core::ffi::c_int,
            page_x: f64,
            page_y: f64,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_OnRButtonDown: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            page: FPDF_PAGE,
            modifier: ::core::ffi::c_int,
            page_x: f64,
            page_y: f64,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_OnLButtonUp: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            page: FPDF_PAGE,
            modifier: ::core::ffi::c_int,
            page_x: f64,
            page_y: f64,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_OnRButtonUp: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            page: FPDF_PAGE,
            modifier: ::core::ffi::c_int,
            page_x: f64,
            page_y: f64,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_OnLButtonDoubleClick: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            page: FPDF_PAGE,
            modifier: ::core::ffi::c_int,
            page_x: f64,
            page_y: f64,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_OnKeyDown: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            page: FPDF_PAGE,
            nKeyCode: ::core::ffi::c_int,
            modifier: ::core::ffi::c_int,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_OnKeyUp: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            page: FPDF_PAGE,
            nKeyCode: ::core::ffi::c_int,
            modifier: ::core::ffi::c_int,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_OnChar: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            page: FPDF_PAGE,
            nChar: ::core::ffi::c_int,
            modifier: ::core::ffi::c_int,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_GetFocusedText: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            page: FPDF_PAGE,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FORM_GetSelectedText: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            page: FPDF_PAGE,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FORM_ReplaceAndKeepSelection: Result<
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, wsText: FPDF_WIDESTRING),
        ::libloading::Error,
    >,
    pub FORM_ReplaceSelection: Result<
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, wsText: FPDF_WIDESTRING),
        ::libloading::Error,
    >,
    pub FORM_SelectAllText: Result<
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_CanUndo: Result<
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_CanRedo: Result<
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_Undo: Result<
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_Redo: Result<
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_ForceToKillFocus:
        Result<unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE) -> FPDF_BOOL, ::libloading::Error>,
    pub FORM_GetFocusedAnnot: Result<
        unsafe extern "C" fn(
            handle: FPDF_FORMHANDLE,
            page_index: *mut ::core::ffi::c_int,
            annot: *mut FPDF_ANNOTATION,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_SetFocusedAnnot: Result<
        unsafe extern "C" fn(handle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPage_HasFormFieldAtPoint: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            page: FPDF_PAGE,
            page_x: f64,
            page_y: f64,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFPage_FormFieldZOrderAtPoint: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            page: FPDF_PAGE,
            page_x: f64,
            page_y: f64,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_SetFormFieldHighlightColor: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            fieldType: ::core::ffi::c_int,
            color: ::core::ffi::c_ulong,
        ),
        ::libloading::Error,
    >,
    pub FPDF_SetFormFieldHighlightAlpha: Result<
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, alpha: ::core::ffi::c_uchar),
        ::libloading::Error,
    >,
    pub FPDF_RemoveFormFieldHighlight:
        Result<unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE), ::libloading::Error>,
    pub FPDF_FFLDraw: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            bitmap: FPDF_BITMAP,
            page: FPDF_PAGE,
            start_x: ::core::ffi::c_int,
            start_y: ::core::ffi::c_int,
            size_x: ::core::ffi::c_int,
            size_y: ::core::ffi::c_int,
            rotate: ::core::ffi::c_int,
            flags: ::core::ffi::c_int,
        ),
        ::libloading::Error,
    >,
    pub FPDF_GetFormType: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FORM_SetIndexSelected: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            page: FPDF_PAGE,
            index: ::core::ffi::c_int,
            selected: FPDF_BOOL,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_IsIndexSelected: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            page: FPDF_PAGE,
            index: ::core::ffi::c_int,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FORM_GetTextDirection: Result<
        unsafe extern "C" fn(
            handle: FPDF_FORMHANDLE,
            annot: FPDF_ANNOTATION,
        ) -> FPDF_TEXT_DIRECTION,
        ::libloading::Error,
    >,
    pub FORM_SetTextDirection: Result<
        unsafe extern "C" fn(
            handle: FPDF_FORMHANDLE,
            annot: FPDF_ANNOTATION,
            direction: FPDF_TEXT_DIRECTION,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDF_LoadXFA:
        Result<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_BOOL, ::libloading::Error>,
    pub FPDFAnnot_IsSupportedSubtype: Result<
        unsafe extern "C" fn(subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPage_CreateAnnot: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_ANNOTATION,
        ::libloading::Error,
    >,
    pub FPDFPage_GetAnnotCount:
        Result<unsafe extern "C" fn(page: FPDF_PAGE) -> ::core::ffi::c_int, ::libloading::Error>,
    pub FPDFPage_GetAnnot: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, index: ::core::ffi::c_int) -> FPDF_ANNOTATION,
        ::libloading::Error,
    >,
    pub FPDFPage_GetAnnotIndex: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, annot: FPDF_ANNOTATION) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFPage_CloseAnnot:
        Result<unsafe extern "C" fn(annot: FPDF_ANNOTATION), ::libloading::Error>,
    pub FPDFPage_RemoveAnnot: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, index: ::core::ffi::c_int) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetSubtype: Result<
        unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_ANNOTATION_SUBTYPE,
        ::libloading::Error,
    >,
    pub FPDFAnnot_IsObjectSupportedSubtype: Result<
        unsafe extern "C" fn(subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_UpdateObject: Result<
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_AddInkStroke: Result<
        unsafe extern "C" fn(
            annot: FPDF_ANNOTATION,
            points: *const FS_POINTF,
            point_count: usize,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFAnnot_RemoveInkList:
        Result<unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_BOOL, ::libloading::Error>,
    pub FPDFAnnot_AppendObject: Result<
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetObjectCount: Result<
        unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetObject: Result<
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, index: ::core::ffi::c_int) -> FPDF_PAGEOBJECT,
        ::libloading::Error,
    >,
    pub FPDFAnnot_RemoveObject: Result<
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, index: ::core::ffi::c_int) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_SetColor: Result<
        unsafe extern "C" fn(
            annot: FPDF_ANNOTATION,
            type_: FPDFANNOT_COLORTYPE,
            R: ::core::ffi::c_uint,
            G: ::core::ffi::c_uint,
            B: ::core::ffi::c_uint,
            A: ::core::ffi::c_uint,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetColor: Result<
        unsafe extern "C" fn(
            annot: FPDF_ANNOTATION,
            type_: FPDFANNOT_COLORTYPE,
            R: *mut ::core::ffi::c_uint,
            G: *mut ::core::ffi::c_uint,
            B: *mut ::core::ffi::c_uint,
            A: *mut ::core::ffi::c_uint,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_HasAttachmentPoints:
        Result<unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_BOOL, ::libloading::Error>,
    pub FPDFAnnot_SetAttachmentPoints: Result<
        unsafe extern "C" fn(
            annot: FPDF_ANNOTATION,
            quad_index: usize,
            quad_points: *const FS_QUADPOINTSF,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_AppendAttachmentPoints: Result<
        unsafe extern "C" fn(
            annot: FPDF_ANNOTATION,
            quad_points: *const FS_QUADPOINTSF,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_CountAttachmentPoints:
        Result<unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> usize, ::libloading::Error>,
    pub FPDFAnnot_GetAttachmentPoints: Result<
        unsafe extern "C" fn(
            annot: FPDF_ANNOTATION,
            quad_index: usize,
            quad_points: *mut FS_QUADPOINTSF,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_SetRect: Result<
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, rect: *const FS_RECTF) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetRect: Result<
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, rect: *mut FS_RECTF) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetVertices: Result<
        unsafe extern "C" fn(
            annot: FPDF_ANNOTATION,
            buffer: *mut FS_POINTF,
            length: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetInkListCount: Result<
        unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetInkListPath: Result<
        unsafe extern "C" fn(
            annot: FPDF_ANNOTATION,
            path_index: ::core::ffi::c_ulong,
            buffer: *mut FS_POINTF,
            length: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetLine: Result<
        unsafe extern "C" fn(
            annot: FPDF_ANNOTATION,
            start: *mut FS_POINTF,
            end: *mut FS_POINTF,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_SetBorder: Result<
        unsafe extern "C" fn(
            annot: FPDF_ANNOTATION,
            horizontal_radius: f32,
            vertical_radius: f32,
            border_width: f32,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetBorder: Result<
        unsafe extern "C" fn(
            annot: FPDF_ANNOTATION,
            horizontal_radius: *mut f32,
            vertical_radius: *mut f32,
            border_width: *mut f32,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetFormAdditionalActionJavaScript: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            annot: FPDF_ANNOTATION,
            event: ::core::ffi::c_int,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFAnnot_HasKey: Result<
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetValueType: Result<
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING) -> FPDF_OBJECT_TYPE,
        ::libloading::Error,
    >,
    pub FPDFAnnot_SetStringValue: Result<
        unsafe extern "C" fn(
            annot: FPDF_ANNOTATION,
            key: FPDF_BYTESTRING,
            value: FPDF_WIDESTRING,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetStringValue: Result<
        unsafe extern "C" fn(
            annot: FPDF_ANNOTATION,
            key: FPDF_BYTESTRING,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetNumberValue: Result<
        unsafe extern "C" fn(
            annot: FPDF_ANNOTATION,
            key: FPDF_BYTESTRING,
            value: *mut f32,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_SetAP: Result<
        unsafe extern "C" fn(
            annot: FPDF_ANNOTATION,
            appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
            value: FPDF_WIDESTRING,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetAP: Result<
        unsafe extern "C" fn(
            annot: FPDF_ANNOTATION,
            appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetLinkedAnnot: Result<
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING) -> FPDF_ANNOTATION,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetFlags: Result<
        unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFAnnot_SetFlags: Result<
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, flags: ::core::ffi::c_int) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetFormFieldFlags: Result<
        unsafe extern "C" fn(handle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFAnnot_SetFormFieldFlags: Result<
        unsafe extern "C" fn(
            handle: FPDF_FORMHANDLE,
            annot: FPDF_ANNOTATION,
            flags: ::core::ffi::c_int,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetFormFieldAtPoint: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            page: FPDF_PAGE,
            point: *const FS_POINTF,
        ) -> FPDF_ANNOTATION,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetFormFieldName: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            annot: FPDF_ANNOTATION,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetFormFieldAlternateName: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            annot: FPDF_ANNOTATION,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetFormFieldType: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            annot: FPDF_ANNOTATION,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetFormFieldValue: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            annot: FPDF_ANNOTATION,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetOptionCount: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            annot: FPDF_ANNOTATION,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetOptionLabel: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            annot: FPDF_ANNOTATION,
            index: ::core::ffi::c_int,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFAnnot_IsOptionSelected: Result<
        unsafe extern "C" fn(
            handle: FPDF_FORMHANDLE,
            annot: FPDF_ANNOTATION,
            index: ::core::ffi::c_int,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetFontSize: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            annot: FPDF_ANNOTATION,
            value: *mut f32,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_SetFontColor: Result<
        unsafe extern "C" fn(
            handle: FPDF_FORMHANDLE,
            annot: FPDF_ANNOTATION,
            R: ::core::ffi::c_uint,
            G: ::core::ffi::c_uint,
            B: ::core::ffi::c_uint,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetFontColor: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            annot: FPDF_ANNOTATION,
            R: *mut ::core::ffi::c_uint,
            G: *mut ::core::ffi::c_uint,
            B: *mut ::core::ffi::c_uint,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_IsChecked: Result<
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_SetFocusableSubtypes: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            subtypes: *const FPDF_ANNOTATION_SUBTYPE,
            count: usize,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetFocusableSubtypesCount: Result<
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetFocusableSubtypes: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            subtypes: *mut FPDF_ANNOTATION_SUBTYPE,
            count: usize,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetLink:
        Result<unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_LINK, ::libloading::Error>,
    pub FPDFAnnot_GetFormControlCount: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            annot: FPDF_ANNOTATION,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetFormControlIndex: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            annot: FPDF_ANNOTATION,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetFormFieldExportValue: Result<
        unsafe extern "C" fn(
            hHandle: FPDF_FORMHANDLE,
            annot: FPDF_ANNOTATION,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFAnnot_SetURI: Result<
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, uri: *const ::core::ffi::c_char) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAnnot_GetFileAttachment: Result<
        unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_ATTACHMENT,
        ::libloading::Error,
    >,
    pub FPDFAnnot_AddFileAttachment: Result<
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, name: FPDF_WIDESTRING) -> FPDF_ATTACHMENT,
        ::libloading::Error,
    >,
    pub FPDFDoc_GetAttachmentCount: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFDoc_AddAttachment: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT, name: FPDF_WIDESTRING) -> FPDF_ATTACHMENT,
        ::libloading::Error,
    >,
    pub FPDFDoc_GetAttachment: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT, index: ::core::ffi::c_int) -> FPDF_ATTACHMENT,
        ::libloading::Error,
    >,
    pub FPDFDoc_DeleteAttachment: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT, index: ::core::ffi::c_int) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAttachment_GetName: Result<
        unsafe extern "C" fn(
            attachment: FPDF_ATTACHMENT,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFAttachment_HasKey: Result<
        unsafe extern "C" fn(attachment: FPDF_ATTACHMENT, key: FPDF_BYTESTRING) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAttachment_GetValueType: Result<
        unsafe extern "C" fn(attachment: FPDF_ATTACHMENT, key: FPDF_BYTESTRING) -> FPDF_OBJECT_TYPE,
        ::libloading::Error,
    >,
    pub FPDFAttachment_SetStringValue: Result<
        unsafe extern "C" fn(
            attachment: FPDF_ATTACHMENT,
            key: FPDF_BYTESTRING,
            value: FPDF_WIDESTRING,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAttachment_GetStringValue: Result<
        unsafe extern "C" fn(
            attachment: FPDF_ATTACHMENT,
            key: FPDF_BYTESTRING,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFAttachment_SetDescription: Result<
        unsafe extern "C" fn(attachment: FPDF_ATTACHMENT, value: FPDF_WIDESTRING) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAttachment_GetDescription: Result<
        unsafe extern "C" fn(
            attachment: FPDF_ATTACHMENT,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFAttachment_SetFile: Result<
        unsafe extern "C" fn(
            attachment: FPDF_ATTACHMENT,
            document: FPDF_DOCUMENT,
            contents: *const ::core::ffi::c_void,
            len: ::core::ffi::c_ulong,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAttachment_GetFile: Result<
        unsafe extern "C" fn(
            attachment: FPDF_ATTACHMENT,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
            out_buflen: *mut ::core::ffi::c_ulong,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAttachment_GetSubtype: Result<
        unsafe extern "C" fn(
            attachment: FPDF_ATTACHMENT,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFAttachment_GetAFRelationship: Result<
        unsafe extern "C" fn(
            attachment: FPDF_ATTACHMENT,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFCatalog_IsTagged:
        Result<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_BOOL, ::libloading::Error>,
    pub FPDFCatalog_GetLanguage: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFCatalog_SetLanguage: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT, language: FPDF_WIDESTRING) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFAvail_Create: Result<
        unsafe extern "C" fn(
            file_avail: *mut FX_FILEAVAIL,
            file: *mut FPDF_FILEACCESS,
        ) -> FPDF_AVAIL,
        ::libloading::Error,
    >,
    pub FPDFAvail_Destroy: Result<unsafe extern "C" fn(avail: FPDF_AVAIL), ::libloading::Error>,
    pub FPDFAvail_IsDocAvail: Result<
        unsafe extern "C" fn(avail: FPDF_AVAIL, hints: *mut FX_DOWNLOADHINTS) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFAvail_GetDocument: Result<
        unsafe extern "C" fn(avail: FPDF_AVAIL, password: FPDF_BYTESTRING) -> FPDF_DOCUMENT,
        ::libloading::Error,
    >,
    pub FPDFAvail_GetFirstPageNum:
        Result<unsafe extern "C" fn(doc: FPDF_DOCUMENT) -> ::core::ffi::c_int, ::libloading::Error>,
    pub FPDFAvail_IsPageAvail: Result<
        unsafe extern "C" fn(
            avail: FPDF_AVAIL,
            page_index: ::core::ffi::c_int,
            hints: *mut FX_DOWNLOADHINTS,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFAvail_IsFormAvail: Result<
        unsafe extern "C" fn(avail: FPDF_AVAIL, hints: *mut FX_DOWNLOADHINTS) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFAvail_IsLinearized:
        Result<unsafe extern "C" fn(avail: FPDF_AVAIL) -> ::core::ffi::c_int, ::libloading::Error>,
    pub FPDFBookmark_GetFirstChild: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_BOOKMARK,
        ::libloading::Error,
    >,
    pub FPDFBookmark_GetNextSibling: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_BOOKMARK,
        ::libloading::Error,
    >,
    pub FPDFBookmark_GetTitle: Result<
        unsafe extern "C" fn(
            bookmark: FPDF_BOOKMARK,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFBookmark_GetCount: Result<
        unsafe extern "C" fn(bookmark: FPDF_BOOKMARK) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFBookmark_Find: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT, title: FPDF_WIDESTRING) -> FPDF_BOOKMARK,
        ::libloading::Error,
    >,
    pub FPDFBookmark_GetDest: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_DEST,
        ::libloading::Error,
    >,
    pub FPDFBookmark_GetAction:
        Result<unsafe extern "C" fn(bookmark: FPDF_BOOKMARK) -> FPDF_ACTION, ::libloading::Error>,
    pub FPDFBookmark_GetColor: Result<
        unsafe extern "C" fn(
            bookmark: FPDF_BOOKMARK,
            R: *mut f32,
            G: *mut f32,
            B: *mut f32,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFBookmark_GetStyle: Result<
        unsafe extern "C" fn(bookmark: FPDF_BOOKMARK) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFAction_GetType: Result<
        unsafe extern "C" fn(action: FPDF_ACTION) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFAction_GetDest: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT, action: FPDF_ACTION) -> FPDF_DEST,
        ::libloading::Error,
    >,
    pub FPDFAction_GetFilePath: Result<
        unsafe extern "C" fn(
            action: FPDF_ACTION,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFAction_GetURIPath: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            action: FPDF_ACTION,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFDest_GetDestPageIndex: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT, dest: FPDF_DEST) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFDest_GetView: Result<
        unsafe extern "C" fn(
            dest: FPDF_DEST,
            pNumParams: *mut ::core::ffi::c_ulong,
            pParams: *mut FS_FLOAT,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFDest_GetLocationInPage: Result<
        unsafe extern "C" fn(
            dest: FPDF_DEST,
            hasXVal: *mut FPDF_BOOL,
            hasYVal: *mut FPDF_BOOL,
            hasZoomVal: *mut FPDF_BOOL,
            x: *mut FS_FLOAT,
            y: *mut FS_FLOAT,
            zoom: *mut FS_FLOAT,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFLink_GetLinkAtPoint: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, x: f64, y: f64) -> FPDF_LINK,
        ::libloading::Error,
    >,
    pub FPDFLink_GetLinkZOrderAtPoint: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, x: f64, y: f64) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFLink_GetDest: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT, link: FPDF_LINK) -> FPDF_DEST,
        ::libloading::Error,
    >,
    pub FPDFLink_GetAction:
        Result<unsafe extern "C" fn(link: FPDF_LINK) -> FPDF_ACTION, ::libloading::Error>,
    pub FPDFLink_Enumerate: Result<
        unsafe extern "C" fn(
            page: FPDF_PAGE,
            start_pos: *mut ::core::ffi::c_int,
            link_annot: *mut FPDF_LINK,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFLink_GetAnnot: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, link_annot: FPDF_LINK) -> FPDF_ANNOTATION,
        ::libloading::Error,
    >,
    pub FPDFLink_GetAnnotRect: Result<
        unsafe extern "C" fn(link_annot: FPDF_LINK, rect: *mut FS_RECTF) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFLink_CountQuadPoints: Result<
        unsafe extern "C" fn(link_annot: FPDF_LINK) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFLink_GetQuadPoints: Result<
        unsafe extern "C" fn(
            link_annot: FPDF_LINK,
            quad_index: ::core::ffi::c_int,
            quad_points: *mut FS_QUADPOINTSF,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDF_GetPageAAction: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, aa_type: ::core::ffi::c_int) -> FPDF_ACTION,
        ::libloading::Error,
    >,
    pub FPDF_GetFileIdentifier: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            id_type: FPDF_FILEIDTYPE,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_GetMetaText: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            tag: FPDF_BYTESTRING,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_GetPageLabel: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            page_index: ::core::ffi::c_int,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_CreateNewDocument:
        Result<unsafe extern "C" fn() -> FPDF_DOCUMENT, ::libloading::Error>,
    pub FPDFPage_New: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            page_index: ::core::ffi::c_int,
            width: f64,
            height: f64,
        ) -> FPDF_PAGE,
        ::libloading::Error,
    >,
    pub FPDFPage_Delete: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT, page_index: ::core::ffi::c_int),
        ::libloading::Error,
    >,
    pub FPDF_MovePages: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            page_indices: *const ::core::ffi::c_int,
            page_indices_len: ::core::ffi::c_ulong,
            dest_page_index: ::core::ffi::c_int,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPage_GetRotation:
        Result<unsafe extern "C" fn(page: FPDF_PAGE) -> ::core::ffi::c_int, ::libloading::Error>,
    pub FPDFPage_SetRotation: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, rotate: ::core::ffi::c_int),
        ::libloading::Error,
    >,
    pub FPDFPage_InsertObject: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, page_object: FPDF_PAGEOBJECT) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPage_InsertObjectAtIndex: Result<
        unsafe extern "C" fn(
            page: FPDF_PAGE,
            page_object: FPDF_PAGEOBJECT,
            index: usize,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPage_RemoveObject: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, page_object: FPDF_PAGEOBJECT) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPage_CountObjects:
        Result<unsafe extern "C" fn(page: FPDF_PAGE) -> ::core::ffi::c_int, ::libloading::Error>,
    pub FPDFPage_GetObject: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, index: ::core::ffi::c_int) -> FPDF_PAGEOBJECT,
        ::libloading::Error,
    >,
    pub FPDFPage_HasTransparency:
        Result<unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_BOOL, ::libloading::Error>,
    pub FPDFPage_GenerateContent:
        Result<unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_BOOL, ::libloading::Error>,
    pub FPDFPageObj_Destroy:
        Result<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT), ::libloading::Error>,
    pub FPDFPageObj_HasTransparency: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_GetType: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFPageObj_GetIsActive: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, active: *mut FPDF_BOOL) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_SetIsActive: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, active: FPDF_BOOL) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_Transform: Result<
        unsafe extern "C" fn(
            page_object: FPDF_PAGEOBJECT,
            a: f64,
            b: f64,
            c: f64,
            d: f64,
            e: f64,
            f: f64,
        ),
        ::libloading::Error,
    >,
    pub FPDFPageObj_TransformF: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, matrix: *const FS_MATRIX) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_GetMatrix: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, matrix: *mut FS_MATRIX) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_SetMatrix: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, matrix: *const FS_MATRIX) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPage_TransformAnnots: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64),
        ::libloading::Error,
    >,
    pub FPDFPageObj_NewImageObj: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_PAGEOBJECT,
        ::libloading::Error,
    >,
    pub FPDFPageObj_GetMarkedContentID: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFPageObj_CountMarks: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFPageObj_GetMark: Result<
        unsafe extern "C" fn(
            page_object: FPDF_PAGEOBJECT,
            index: ::core::ffi::c_ulong,
        ) -> FPDF_PAGEOBJECTMARK,
        ::libloading::Error,
    >,
    pub FPDFPageObj_AddMark: Result<
        unsafe extern "C" fn(
            page_object: FPDF_PAGEOBJECT,
            name: FPDF_BYTESTRING,
        ) -> FPDF_PAGEOBJECTMARK,
        ::libloading::Error,
    >,
    pub FPDFPageObj_AddExistingMark: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, mark: FPDF_PAGEOBJECTMARK) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_RemoveMark: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, mark: FPDF_PAGEOBJECTMARK) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObjMark_GetName: Result<
        unsafe extern "C" fn(
            mark: FPDF_PAGEOBJECTMARK,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
            out_buflen: *mut ::core::ffi::c_ulong,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObjMark_CountParams: Result<
        unsafe extern "C" fn(mark: FPDF_PAGEOBJECTMARK) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFPageObjMark_GetParamKey: Result<
        unsafe extern "C" fn(
            mark: FPDF_PAGEOBJECTMARK,
            index: ::core::ffi::c_ulong,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
            out_buflen: *mut ::core::ffi::c_ulong,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObjMark_GetParamValueType: Result<
        unsafe extern "C" fn(mark: FPDF_PAGEOBJECTMARK, key: FPDF_BYTESTRING) -> FPDF_OBJECT_TYPE,
        ::libloading::Error,
    >,
    pub FPDFPageObjMark_GetParamIntValue: Result<
        unsafe extern "C" fn(
            mark: FPDF_PAGEOBJECTMARK,
            key: FPDF_BYTESTRING,
            out_value: *mut ::core::ffi::c_int,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObjMark_GetParamFloatValue: Result<
        unsafe extern "C" fn(
            mark: FPDF_PAGEOBJECTMARK,
            key: FPDF_BYTESTRING,
            out_value: *mut f32,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObjMark_GetParamStringValue: Result<
        unsafe extern "C" fn(
            mark: FPDF_PAGEOBJECTMARK,
            key: FPDF_BYTESTRING,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
            out_buflen: *mut ::core::ffi::c_ulong,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObjMark_GetParamBlobValue: Result<
        unsafe extern "C" fn(
            mark: FPDF_PAGEOBJECTMARK,
            key: FPDF_BYTESTRING,
            buffer: *mut ::core::ffi::c_uchar,
            buflen: ::core::ffi::c_ulong,
            out_buflen: *mut ::core::ffi::c_ulong,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObjMark_SetIntParam: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            page_object: FPDF_PAGEOBJECT,
            mark: FPDF_PAGEOBJECTMARK,
            key: FPDF_BYTESTRING,
            value: ::core::ffi::c_int,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObjMark_SetFloatParam: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            page_object: FPDF_PAGEOBJECT,
            mark: FPDF_PAGEOBJECTMARK,
            key: FPDF_BYTESTRING,
            value: f32,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObjMark_SetStringParam: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            page_object: FPDF_PAGEOBJECT,
            mark: FPDF_PAGEOBJECTMARK,
            key: FPDF_BYTESTRING,
            value: FPDF_BYTESTRING,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObjMark_SetBlobParam: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            page_object: FPDF_PAGEOBJECT,
            mark: FPDF_PAGEOBJECTMARK,
            key: FPDF_BYTESTRING,
            value: *const ::core::ffi::c_uchar,
            value_len: ::core::ffi::c_ulong,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObjMark_RemoveParam: Result<
        unsafe extern "C" fn(
            page_object: FPDF_PAGEOBJECT,
            mark: FPDF_PAGEOBJECTMARK,
            key: FPDF_BYTESTRING,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFImageObj_LoadJpegFile: Result<
        unsafe extern "C" fn(
            pages: *mut FPDF_PAGE,
            count: ::core::ffi::c_int,
            image_object: FPDF_PAGEOBJECT,
            file_access: *mut FPDF_FILEACCESS,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFImageObj_LoadJpegFileInline: Result<
        unsafe extern "C" fn(
            pages: *mut FPDF_PAGE,
            count: ::core::ffi::c_int,
            image_object: FPDF_PAGEOBJECT,
            file_access: *mut FPDF_FILEACCESS,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFImageObj_SetMatrix: Result<
        unsafe extern "C" fn(
            image_object: FPDF_PAGEOBJECT,
            a: f64,
            b: f64,
            c: f64,
            d: f64,
            e: f64,
            f: f64,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFImageObj_SetBitmap: Result<
        unsafe extern "C" fn(
            pages: *mut FPDF_PAGE,
            count: ::core::ffi::c_int,
            image_object: FPDF_PAGEOBJECT,
            bitmap: FPDF_BITMAP,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFImageObj_GetBitmap: Result<
        unsafe extern "C" fn(image_object: FPDF_PAGEOBJECT) -> FPDF_BITMAP,
        ::libloading::Error,
    >,
    pub FPDFImageObj_GetRenderedBitmap: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            page: FPDF_PAGE,
            image_object: FPDF_PAGEOBJECT,
        ) -> FPDF_BITMAP,
        ::libloading::Error,
    >,
    pub FPDFImageObj_GetImageDataDecoded: Result<
        unsafe extern "C" fn(
            image_object: FPDF_PAGEOBJECT,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFImageObj_GetImageDataRaw: Result<
        unsafe extern "C" fn(
            image_object: FPDF_PAGEOBJECT,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFImageObj_GetImageFilterCount: Result<
        unsafe extern "C" fn(image_object: FPDF_PAGEOBJECT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFImageObj_GetImageFilter: Result<
        unsafe extern "C" fn(
            image_object: FPDF_PAGEOBJECT,
            index: ::core::ffi::c_int,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFImageObj_GetImageMetadata: Result<
        unsafe extern "C" fn(
            image_object: FPDF_PAGEOBJECT,
            page: FPDF_PAGE,
            metadata: *mut FPDF_IMAGEOBJ_METADATA,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFImageObj_GetImagePixelSize: Result<
        unsafe extern "C" fn(
            image_object: FPDF_PAGEOBJECT,
            width: *mut ::core::ffi::c_uint,
            height: *mut ::core::ffi::c_uint,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFImageObj_GetIccProfileDataDecoded: Result<
        unsafe extern "C" fn(
            image_object: FPDF_PAGEOBJECT,
            page: FPDF_PAGE,
            buffer: *mut u8,
            buflen: usize,
            out_buflen: *mut usize,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_CreateNewPath:
        Result<unsafe extern "C" fn(x: f32, y: f32) -> FPDF_PAGEOBJECT, ::libloading::Error>,
    pub FPDFPageObj_CreateNewRect: Result<
        unsafe extern "C" fn(x: f32, y: f32, w: f32, h: f32) -> FPDF_PAGEOBJECT,
        ::libloading::Error,
    >,
    pub FPDFPageObj_GetBounds: Result<
        unsafe extern "C" fn(
            page_object: FPDF_PAGEOBJECT,
            left: *mut f32,
            bottom: *mut f32,
            right: *mut f32,
            top: *mut f32,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_GetRotatedBounds: Result<
        unsafe extern "C" fn(
            page_object: FPDF_PAGEOBJECT,
            quad_points: *mut FS_QUADPOINTSF,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_SetBlendMode: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, blend_mode: FPDF_BYTESTRING),
        ::libloading::Error,
    >,
    pub FPDFPageObj_SetStrokeColor: Result<
        unsafe extern "C" fn(
            page_object: FPDF_PAGEOBJECT,
            R: ::core::ffi::c_uint,
            G: ::core::ffi::c_uint,
            B: ::core::ffi::c_uint,
            A: ::core::ffi::c_uint,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_GetStrokeColor: Result<
        unsafe extern "C" fn(
            page_object: FPDF_PAGEOBJECT,
            R: *mut ::core::ffi::c_uint,
            G: *mut ::core::ffi::c_uint,
            B: *mut ::core::ffi::c_uint,
            A: *mut ::core::ffi::c_uint,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_SetStrokeWidth: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, width: f32) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_GetStrokeWidth: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, width: *mut f32) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_GetLineJoin: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFPageObj_SetLineJoin: Result<
        unsafe extern "C" fn(
            page_object: FPDF_PAGEOBJECT,
            line_join: ::core::ffi::c_int,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_GetLineCap: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFPageObj_SetLineCap: Result<
        unsafe extern "C" fn(
            page_object: FPDF_PAGEOBJECT,
            line_cap: ::core::ffi::c_int,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_SetFillColor: Result<
        unsafe extern "C" fn(
            page_object: FPDF_PAGEOBJECT,
            R: ::core::ffi::c_uint,
            G: ::core::ffi::c_uint,
            B: ::core::ffi::c_uint,
            A: ::core::ffi::c_uint,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_GetFillColor: Result<
        unsafe extern "C" fn(
            page_object: FPDF_PAGEOBJECT,
            R: *mut ::core::ffi::c_uint,
            G: *mut ::core::ffi::c_uint,
            B: *mut ::core::ffi::c_uint,
            A: *mut ::core::ffi::c_uint,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_GetDashPhase: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, phase: *mut f32) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_SetDashPhase: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, phase: f32) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_GetDashCount: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFPageObj_GetDashArray: Result<
        unsafe extern "C" fn(
            page_object: FPDF_PAGEOBJECT,
            dash_array: *mut f32,
            dash_count: usize,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_SetDashArray: Result<
        unsafe extern "C" fn(
            page_object: FPDF_PAGEOBJECT,
            dash_array: *const f32,
            dash_count: usize,
            phase: f32,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_GetRenderedStrokePattern: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT, page_object: FPDF_PAGEOBJECT) -> FPDF_BITMAP,
        ::libloading::Error,
    >,
    pub FPDFPageObj_GetRenderedFillPattern: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT, page_object: FPDF_PAGEOBJECT) -> FPDF_BITMAP,
        ::libloading::Error,
    >,
    pub FPDFPath_CountSegments: Result<
        unsafe extern "C" fn(path: FPDF_PAGEOBJECT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFPath_GetPathSegment: Result<
        unsafe extern "C" fn(path: FPDF_PAGEOBJECT, index: ::core::ffi::c_int) -> FPDF_PATHSEGMENT,
        ::libloading::Error,
    >,
    pub FPDFPathSegment_GetPoint: Result<
        unsafe extern "C" fn(segment: FPDF_PATHSEGMENT, x: *mut f32, y: *mut f32) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPathSegment_GetType: Result<
        unsafe extern "C" fn(segment: FPDF_PATHSEGMENT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFPathSegment_GetClose:
        Result<unsafe extern "C" fn(segment: FPDF_PATHSEGMENT) -> FPDF_BOOL, ::libloading::Error>,
    pub FPDFPath_MoveTo: Result<
        unsafe extern "C" fn(path: FPDF_PAGEOBJECT, x: f32, y: f32) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPath_LineTo: Result<
        unsafe extern "C" fn(path: FPDF_PAGEOBJECT, x: f32, y: f32) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPath_BezierTo: Result<
        unsafe extern "C" fn(
            path: FPDF_PAGEOBJECT,
            x1: f32,
            y1: f32,
            x2: f32,
            y2: f32,
            x3: f32,
            y3: f32,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPath_GetBezierControlPoints: Result<
        unsafe extern "C" fn(
            path: FPDF_PAGEOBJECT,
            index: usize,
            first_control_point: *mut FS_POINTF,
            second_control_point: *mut FS_POINTF,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPath_Close:
        Result<unsafe extern "C" fn(path: FPDF_PAGEOBJECT) -> FPDF_BOOL, ::libloading::Error>,
    pub FPDFPath_SetDrawMode: Result<
        unsafe extern "C" fn(
            path: FPDF_PAGEOBJECT,
            fillmode: ::core::ffi::c_int,
            stroke: FPDF_BOOL,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPath_GetDrawMode: Result<
        unsafe extern "C" fn(
            path: FPDF_PAGEOBJECT,
            fillmode: *mut ::core::ffi::c_int,
            stroke: *mut FPDF_BOOL,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_NewTextObj: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            font: FPDF_BYTESTRING,
            font_size: f32,
        ) -> FPDF_PAGEOBJECT,
        ::libloading::Error,
    >,
    pub FPDFText_SetText: Result<
        unsafe extern "C" fn(text_object: FPDF_PAGEOBJECT, text: FPDF_WIDESTRING) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFText_SetCharcodes: Result<
        unsafe extern "C" fn(
            text_object: FPDF_PAGEOBJECT,
            charcodes: *const u32,
            count: usize,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFText_SetPositions: Result<
        unsafe extern "C" fn(
            text_object: FPDF_PAGEOBJECT,
            positions: *const f32,
            count: usize,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFText_LoadFont: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            data: *const u8,
            size: u32,
            font_type: ::core::ffi::c_int,
            cid: FPDF_BOOL,
        ) -> FPDF_FONT,
        ::libloading::Error,
    >,
    pub FPDFText_LoadStandardFont: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT, font: FPDF_BYTESTRING) -> FPDF_FONT,
        ::libloading::Error,
    >,
    pub FPDFText_LoadCidType2Font: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            font_data: *const u8,
            font_data_size: u32,
            to_unicode_cmap: FPDF_BYTESTRING,
            cid_to_gid_map_data: *const u8,
            cid_to_gid_map_data_size: u32,
        ) -> FPDF_FONT,
        ::libloading::Error,
    >,
    pub FPDFTextObj_GetFontSize: Result<
        unsafe extern "C" fn(text: FPDF_PAGEOBJECT, size: *mut f32) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFTextObj_SetFontSize: Result<
        unsafe extern "C" fn(text: FPDF_PAGEOBJECT, size: f32) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFFont_Close: Result<unsafe extern "C" fn(font: FPDF_FONT), ::libloading::Error>,
    pub FPDFPageObj_CreateTextObj: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            font: FPDF_FONT,
            font_size: f32,
        ) -> FPDF_PAGEOBJECT,
        ::libloading::Error,
    >,
    pub FPDFTextObj_GetTextRenderMode: Result<
        unsafe extern "C" fn(text: FPDF_PAGEOBJECT) -> FPDF_TEXT_RENDERMODE,
        ::libloading::Error,
    >,
    pub FPDFTextObj_SetTextRenderMode: Result<
        unsafe extern "C" fn(text: FPDF_PAGEOBJECT, render_mode: FPDF_TEXT_RENDERMODE) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFTextObj_GetText: Result<
        unsafe extern "C" fn(
            text_object: FPDF_PAGEOBJECT,
            text_page: FPDF_TEXTPAGE,
            buffer: *mut FPDF_WCHAR,
            length: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFTextObj_GetRenderedBitmap: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            page: FPDF_PAGE,
            text_object: FPDF_PAGEOBJECT,
            scale: f32,
        ) -> FPDF_BITMAP,
        ::libloading::Error,
    >,
    pub FPDFTextObj_GetFont:
        Result<unsafe extern "C" fn(text: FPDF_PAGEOBJECT) -> FPDF_FONT, ::libloading::Error>,
    pub FPDFFont_GetBaseFontName: Result<
        unsafe extern "C" fn(
            font: FPDF_FONT,
            buffer: *mut ::core::ffi::c_char,
            length: usize,
        ) -> usize,
        ::libloading::Error,
    >,
    pub FPDFFont_GetFamilyName: Result<
        unsafe extern "C" fn(
            font: FPDF_FONT,
            buffer: *mut ::core::ffi::c_char,
            length: usize,
        ) -> usize,
        ::libloading::Error,
    >,
    pub FPDFFont_GetFontData: Result<
        unsafe extern "C" fn(
            font: FPDF_FONT,
            buffer: *mut u8,
            buflen: usize,
            out_buflen: *mut usize,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFFont_GetIsEmbedded:
        Result<unsafe extern "C" fn(font: FPDF_FONT) -> ::core::ffi::c_int, ::libloading::Error>,
    pub FPDFFont_GetFlags:
        Result<unsafe extern "C" fn(font: FPDF_FONT) -> ::core::ffi::c_int, ::libloading::Error>,
    pub FPDFFont_GetWeight:
        Result<unsafe extern "C" fn(font: FPDF_FONT) -> ::core::ffi::c_int, ::libloading::Error>,
    pub FPDFFont_GetItalicAngle: Result<
        unsafe extern "C" fn(font: FPDF_FONT, angle: *mut ::core::ffi::c_int) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFFont_GetAscent: Result<
        unsafe extern "C" fn(font: FPDF_FONT, font_size: f32, ascent: *mut f32) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFFont_GetDescent: Result<
        unsafe extern "C" fn(font: FPDF_FONT, font_size: f32, descent: *mut f32) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFFont_GetGlyphWidth: Result<
        unsafe extern "C" fn(
            font: FPDF_FONT,
            glyph: u32,
            font_size: f32,
            width: *mut f32,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFFont_GetGlyphPath: Result<
        unsafe extern "C" fn(font: FPDF_FONT, glyph: u32, font_size: f32) -> FPDF_GLYPHPATH,
        ::libloading::Error,
    >,
    pub FPDFGlyphPath_CountGlyphSegments: Result<
        unsafe extern "C" fn(glyphpath: FPDF_GLYPHPATH) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFGlyphPath_GetGlyphPathSegment: Result<
        unsafe extern "C" fn(
            glyphpath: FPDF_GLYPHPATH,
            index: ::core::ffi::c_int,
        ) -> FPDF_PATHSEGMENT,
        ::libloading::Error,
    >,
    pub FPDFFormObj_CountObjects: Result<
        unsafe extern "C" fn(form_object: FPDF_PAGEOBJECT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFFormObj_GetObject: Result<
        unsafe extern "C" fn(
            form_object: FPDF_PAGEOBJECT,
            index: ::core::ffi::c_ulong,
        ) -> FPDF_PAGEOBJECT,
        ::libloading::Error,
    >,
    pub FPDFFormObj_RemoveObject: Result<
        unsafe extern "C" fn(
            form_object: FPDF_PAGEOBJECT,
            page_object: FPDF_PAGEOBJECT,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FSDK_SetUnSpObjProcessHandler: Result<
        unsafe extern "C" fn(unsp_info: *mut UNSUPPORT_INFO) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FSDK_SetTimeFunction: Result<
        unsafe extern "C" fn(func: ::core::option::Option<unsafe extern "C" fn() -> time_t>),
        ::libloading::Error,
    >,
    pub FSDK_SetLocaltimeFunction: Result<
        unsafe extern "C" fn(
            func: ::core::option::Option<unsafe extern "C" fn(arg1: *const time_t) -> *mut tm>,
        ),
        ::libloading::Error,
    >,
    pub FPDFDoc_GetPageMode: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFPage_Flatten: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, nFlag: ::core::ffi::c_int) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFDoc_GetJavaScriptActionCount: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFDoc_GetJavaScriptAction: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            index: ::core::ffi::c_int,
        ) -> FPDF_JAVASCRIPT_ACTION,
        ::libloading::Error,
    >,
    pub FPDFDoc_CloseJavaScriptAction:
        Result<unsafe extern "C" fn(javascript: FPDF_JAVASCRIPT_ACTION), ::libloading::Error>,
    pub FPDFJavaScriptAction_GetName: Result<
        unsafe extern "C" fn(
            javascript: FPDF_JAVASCRIPT_ACTION,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFJavaScriptAction_GetScript: Result<
        unsafe extern "C" fn(
            javascript: FPDF_JAVASCRIPT_ACTION,
            buffer: *mut FPDF_WCHAR,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_ImportPagesByIndex: Result<
        unsafe extern "C" fn(
            dest_doc: FPDF_DOCUMENT,
            src_doc: FPDF_DOCUMENT,
            page_indices: *const ::core::ffi::c_int,
            length: ::core::ffi::c_ulong,
            index: ::core::ffi::c_int,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDF_ImportPages: Result<
        unsafe extern "C" fn(
            dest_doc: FPDF_DOCUMENT,
            src_doc: FPDF_DOCUMENT,
            pagerange: FPDF_BYTESTRING,
            index: ::core::ffi::c_int,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDF_ImportNPagesToOne: Result<
        unsafe extern "C" fn(
            src_doc: FPDF_DOCUMENT,
            output_width: f32,
            output_height: f32,
            num_pages_on_x_axis: usize,
            num_pages_on_y_axis: usize,
        ) -> FPDF_DOCUMENT,
        ::libloading::Error,
    >,
    pub FPDF_NewXObjectFromPage: Result<
        unsafe extern "C" fn(
            dest_doc: FPDF_DOCUMENT,
            src_doc: FPDF_DOCUMENT,
            src_page_index: ::core::ffi::c_int,
        ) -> FPDF_XOBJECT,
        ::libloading::Error,
    >,
    pub FPDF_CloseXObject: Result<unsafe extern "C" fn(xobject: FPDF_XOBJECT), ::libloading::Error>,
    pub FPDF_NewFormObjectFromXObject:
        Result<unsafe extern "C" fn(xobject: FPDF_XOBJECT) -> FPDF_PAGEOBJECT, ::libloading::Error>,
    pub FPDF_CopyViewerPreferences: Result<
        unsafe extern "C" fn(dest_doc: FPDF_DOCUMENT, src_doc: FPDF_DOCUMENT) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDF_RenderPageBitmapWithColorScheme_Start: Result<
        unsafe extern "C" fn(
            bitmap: FPDF_BITMAP,
            page: FPDF_PAGE,
            start_x: ::core::ffi::c_int,
            start_y: ::core::ffi::c_int,
            size_x: ::core::ffi::c_int,
            size_y: ::core::ffi::c_int,
            rotate: ::core::ffi::c_int,
            flags: ::core::ffi::c_int,
            color_scheme: *const FPDF_COLORSCHEME,
            pause: *mut IFSDK_PAUSE,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_RenderPageBitmap_Start: Result<
        unsafe extern "C" fn(
            bitmap: FPDF_BITMAP,
            page: FPDF_PAGE,
            start_x: ::core::ffi::c_int,
            start_y: ::core::ffi::c_int,
            size_x: ::core::ffi::c_int,
            size_y: ::core::ffi::c_int,
            rotate: ::core::ffi::c_int,
            flags: ::core::ffi::c_int,
            pause: *mut IFSDK_PAUSE,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_RenderPage_Continue: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, pause: *mut IFSDK_PAUSE) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_RenderPage_Close: Result<unsafe extern "C" fn(page: FPDF_PAGE), ::libloading::Error>,
    pub FPDF_SaveAsCopy: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            file_write: *mut FPDF_FILEWRITE,
            flags: FPDF_DWORD,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDF_SaveWithVersion: Result<
        unsafe extern "C" fn(
            document: FPDF_DOCUMENT,
            file_write: *mut FPDF_FILEWRITE,
            flags: FPDF_DWORD,
            file_version: ::core::ffi::c_int,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFText_GetCharIndexFromTextIndex: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            nTextIndex: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFText_GetTextIndexFromCharIndex: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            nCharIndex: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_GetSignatureCount: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_GetSignatureObject: Result<
        unsafe extern "C" fn(document: FPDF_DOCUMENT, index: ::core::ffi::c_int) -> FPDF_SIGNATURE,
        ::libloading::Error,
    >,
    pub FPDFSignatureObj_GetContents: Result<
        unsafe extern "C" fn(
            signature: FPDF_SIGNATURE,
            buffer: *mut ::core::ffi::c_void,
            length: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFSignatureObj_GetByteRange: Result<
        unsafe extern "C" fn(
            signature: FPDF_SIGNATURE,
            buffer: *mut ::core::ffi::c_int,
            length: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFSignatureObj_GetSubFilter: Result<
        unsafe extern "C" fn(
            signature: FPDF_SIGNATURE,
            buffer: *mut ::core::ffi::c_char,
            length: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFSignatureObj_GetReason: Result<
        unsafe extern "C" fn(
            signature: FPDF_SIGNATURE,
            buffer: *mut ::core::ffi::c_void,
            length: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFSignatureObj_GetTime: Result<
        unsafe extern "C" fn(
            signature: FPDF_SIGNATURE,
            buffer: *mut ::core::ffi::c_char,
            length: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFSignatureObj_GetDocMDPPermission: Result<
        unsafe extern "C" fn(signature: FPDF_SIGNATURE) -> ::core::ffi::c_uint,
        ::libloading::Error,
    >,
    pub FPDF_StructTree_GetForPage:
        Result<unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_STRUCTTREE, ::libloading::Error>,
    pub FPDF_StructTree_Close:
        Result<unsafe extern "C" fn(struct_tree: FPDF_STRUCTTREE), ::libloading::Error>,
    pub FPDF_StructTree_CountChildren: Result<
        unsafe extern "C" fn(struct_tree: FPDF_STRUCTTREE) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_StructTree_GetChildAtIndex: Result<
        unsafe extern "C" fn(
            struct_tree: FPDF_STRUCTTREE,
            index: ::core::ffi::c_int,
        ) -> FPDF_STRUCTELEMENT,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_GetAltText: Result<
        unsafe extern "C" fn(
            struct_element: FPDF_STRUCTELEMENT,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_GetActualText: Result<
        unsafe extern "C" fn(
            struct_element: FPDF_STRUCTELEMENT,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_GetExpansion: Result<
        unsafe extern "C" fn(
            struct_element: FPDF_STRUCTELEMENT,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_GetID: Result<
        unsafe extern "C" fn(
            struct_element: FPDF_STRUCTELEMENT,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_GetLang: Result<
        unsafe extern "C" fn(
            struct_element: FPDF_STRUCTELEMENT,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_GetStringAttribute: Result<
        unsafe extern "C" fn(
            struct_element: FPDF_STRUCTELEMENT,
            attr_name: FPDF_BYTESTRING,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_GetMarkedContentID: Result<
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_GetRawType: Result<
        unsafe extern "C" fn(
            struct_element: FPDF_STRUCTELEMENT,
            buffer: *mut FPDF_WCHAR,
            buflen: usize,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_GetType: Result<
        unsafe extern "C" fn(
            struct_element: FPDF_STRUCTELEMENT,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_GetObjType: Result<
        unsafe extern "C" fn(
            struct_element: FPDF_STRUCTELEMENT,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_GetTitle: Result<
        unsafe extern "C" fn(
            struct_element: FPDF_STRUCTELEMENT,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_CountChildren: Result<
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_GetChildAtIndex: Result<
        unsafe extern "C" fn(
            struct_element: FPDF_STRUCTELEMENT,
            index: ::core::ffi::c_int,
        ) -> FPDF_STRUCTELEMENT,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_GetChildMarkedContentID: Result<
        unsafe extern "C" fn(
            struct_element: FPDF_STRUCTELEMENT,
            index: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_GetParent: Result<
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> FPDF_STRUCTELEMENT,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_GetAttributeCount: Result<
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_GetAttributeAtIndex: Result<
        unsafe extern "C" fn(
            struct_element: FPDF_STRUCTELEMENT,
            index: ::core::ffi::c_int,
        ) -> FPDF_STRUCTELEMENT_ATTR,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_Attr_GetCount: Result<
        unsafe extern "C" fn(struct_attribute: FPDF_STRUCTELEMENT_ATTR) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_Attr_GetName: Result<
        unsafe extern "C" fn(
            struct_attribute: FPDF_STRUCTELEMENT_ATTR,
            index: ::core::ffi::c_int,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
            out_buflen: *mut ::core::ffi::c_ulong,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_Attr_GetValue: Result<
        unsafe extern "C" fn(
            struct_attribute: FPDF_STRUCTELEMENT_ATTR,
            name: FPDF_BYTESTRING,
        ) -> FPDF_STRUCTELEMENT_ATTR_VALUE,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_Attr_GetType: Result<
        unsafe extern "C" fn(value: FPDF_STRUCTELEMENT_ATTR_VALUE) -> FPDF_OBJECT_TYPE,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_Attr_GetBooleanValue: Result<
        unsafe extern "C" fn(
            value: FPDF_STRUCTELEMENT_ATTR_VALUE,
            out_value: *mut FPDF_BOOL,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_Attr_GetNumberValue: Result<
        unsafe extern "C" fn(
            value: FPDF_STRUCTELEMENT_ATTR_VALUE,
            out_value: *mut f32,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_Attr_GetStringValue: Result<
        unsafe extern "C" fn(
            value: FPDF_STRUCTELEMENT_ATTR_VALUE,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
            out_buflen: *mut ::core::ffi::c_ulong,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_Attr_GetBlobValue: Result<
        unsafe extern "C" fn(
            value: FPDF_STRUCTELEMENT_ATTR_VALUE,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
            out_buflen: *mut ::core::ffi::c_ulong,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_Attr_CountChildren: Result<
        unsafe extern "C" fn(value: FPDF_STRUCTELEMENT_ATTR_VALUE) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_Attr_GetChildAtIndex: Result<
        unsafe extern "C" fn(
            value: FPDF_STRUCTELEMENT_ATTR_VALUE,
            index: ::core::ffi::c_int,
        ) -> FPDF_STRUCTELEMENT_ATTR_VALUE,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_GetMarkedContentIdCount: Result<
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_StructElement_GetMarkedContentIdAtIndex: Result<
        unsafe extern "C" fn(
            struct_element: FPDF_STRUCTELEMENT,
            index: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDF_GetDefaultTTFMap:
        Result<unsafe extern "C" fn() -> *const FPDF_CharsetFontMap, ::libloading::Error>,
    pub FPDF_GetDefaultTTFMapCount: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub FPDF_GetDefaultTTFMapEntry: Result<
        unsafe extern "C" fn(index: usize) -> *const FPDF_CharsetFontMap,
        ::libloading::Error,
    >,
    pub FPDF_AddInstalledFont: Result<
        unsafe extern "C" fn(
            mapper: *mut ::core::ffi::c_void,
            face: *const ::core::ffi::c_char,
            charset: ::core::ffi::c_int,
        ),
        ::libloading::Error,
    >,
    pub FPDF_SetSystemFontInfo:
        Result<unsafe extern "C" fn(font_info: *mut FPDF_SYSFONTINFO), ::libloading::Error>,
    pub FPDF_GetDefaultSystemFontInfo:
        Result<unsafe extern "C" fn() -> *mut FPDF_SYSFONTINFO, ::libloading::Error>,
    pub FPDF_FreeDefaultSystemFontInfo:
        Result<unsafe extern "C" fn(font_info: *mut FPDF_SYSFONTINFO), ::libloading::Error>,
    pub FPDFText_LoadPage:
        Result<unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_TEXTPAGE, ::libloading::Error>,
    pub FPDFText_ClosePage:
        Result<unsafe extern "C" fn(text_page: FPDF_TEXTPAGE), ::libloading::Error>,
    pub FPDFText_CountChars: Result<
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFText_GetUnicode: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            index: ::core::ffi::c_int,
        ) -> ::core::ffi::c_uint,
        ::libloading::Error,
    >,
    pub FPDFText_GetTextObject: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            index: ::core::ffi::c_int,
        ) -> FPDF_PAGEOBJECT,
        ::libloading::Error,
    >,
    pub FPDFText_IsGenerated: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            index: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFText_IsHyphen: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            index: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFText_HasUnicodeMapError: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            index: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFText_GetFontSize: Result<
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: ::core::ffi::c_int) -> f64,
        ::libloading::Error,
    >,
    pub FPDFText_GetFontInfo: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            index: ::core::ffi::c_int,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
            flags: *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFText_GetFontWeight: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            index: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFText_GetFillColor: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            index: ::core::ffi::c_int,
            R: *mut ::core::ffi::c_uint,
            G: *mut ::core::ffi::c_uint,
            B: *mut ::core::ffi::c_uint,
            A: *mut ::core::ffi::c_uint,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFText_GetStrokeColor: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            index: ::core::ffi::c_int,
            R: *mut ::core::ffi::c_uint,
            G: *mut ::core::ffi::c_uint,
            B: *mut ::core::ffi::c_uint,
            A: *mut ::core::ffi::c_uint,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFText_GetCharAngle: Result<
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: ::core::ffi::c_int) -> f32,
        ::libloading::Error,
    >,
    pub FPDFText_GetCharBox: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            index: ::core::ffi::c_int,
            left: *mut f64,
            right: *mut f64,
            bottom: *mut f64,
            top: *mut f64,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFText_GetLooseCharBox: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            index: ::core::ffi::c_int,
            rect: *mut FS_RECTF,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFText_GetMatrix: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            index: ::core::ffi::c_int,
            matrix: *mut FS_MATRIX,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFText_GetCharOrigin: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            index: ::core::ffi::c_int,
            x: *mut f64,
            y: *mut f64,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFText_GetCharIndexAtPos: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            x: f64,
            y: f64,
            xTolerance: f64,
            yTolerance: f64,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFText_GetText: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            start_index: ::core::ffi::c_int,
            count: ::core::ffi::c_int,
            result: *mut ::core::ffi::c_ushort,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFText_CountRects: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            start_index: ::core::ffi::c_int,
            count: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFText_GetRect: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            rect_index: ::core::ffi::c_int,
            left: *mut f64,
            top: *mut f64,
            right: *mut f64,
            bottom: *mut f64,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFText_GetBoundedText: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            left: f64,
            top: f64,
            right: f64,
            bottom: f64,
            buffer: *mut ::core::ffi::c_ushort,
            buflen: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFText_FindStart: Result<
        unsafe extern "C" fn(
            text_page: FPDF_TEXTPAGE,
            findwhat: FPDF_WIDESTRING,
            flags: ::core::ffi::c_ulong,
            start_index: ::core::ffi::c_int,
        ) -> FPDF_SCHHANDLE,
        ::libloading::Error,
    >,
    pub FPDFText_FindNext:
        Result<unsafe extern "C" fn(handle: FPDF_SCHHANDLE) -> FPDF_BOOL, ::libloading::Error>,
    pub FPDFText_FindPrev:
        Result<unsafe extern "C" fn(handle: FPDF_SCHHANDLE) -> FPDF_BOOL, ::libloading::Error>,
    pub FPDFText_GetSchResultIndex: Result<
        unsafe extern "C" fn(handle: FPDF_SCHHANDLE) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFText_GetSchCount: Result<
        unsafe extern "C" fn(handle: FPDF_SCHHANDLE) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFText_FindClose:
        Result<unsafe extern "C" fn(handle: FPDF_SCHHANDLE), ::libloading::Error>,
    pub FPDFLink_LoadWebLinks: Result<
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE) -> FPDF_PAGELINK,
        ::libloading::Error,
    >,
    pub FPDFLink_CountWebLinks: Result<
        unsafe extern "C" fn(link_page: FPDF_PAGELINK) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFLink_GetURL: Result<
        unsafe extern "C" fn(
            link_page: FPDF_PAGELINK,
            link_index: ::core::ffi::c_int,
            buffer: *mut ::core::ffi::c_ushort,
            buflen: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFLink_CountRects: Result<
        unsafe extern "C" fn(
            link_page: FPDF_PAGELINK,
            link_index: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFLink_GetRect: Result<
        unsafe extern "C" fn(
            link_page: FPDF_PAGELINK,
            link_index: ::core::ffi::c_int,
            rect_index: ::core::ffi::c_int,
            left: *mut f64,
            top: *mut f64,
            right: *mut f64,
            bottom: *mut f64,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFLink_GetTextRange: Result<
        unsafe extern "C" fn(
            link_page: FPDF_PAGELINK,
            link_index: ::core::ffi::c_int,
            start_char_index: *mut ::core::ffi::c_int,
            char_count: *mut ::core::ffi::c_int,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFLink_CloseWebLinks:
        Result<unsafe extern "C" fn(link_page: FPDF_PAGELINK), ::libloading::Error>,
    pub FPDFPage_GetDecodedThumbnailData: Result<
        unsafe extern "C" fn(
            page: FPDF_PAGE,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFPage_GetRawThumbnailData: Result<
        unsafe extern "C" fn(
            page: FPDF_PAGE,
            buffer: *mut ::core::ffi::c_void,
            buflen: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
        ::libloading::Error,
    >,
    pub FPDFPage_GetThumbnailAsBitmap:
        Result<unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_BITMAP, ::libloading::Error>,
    pub FPDFPage_SetMediaBox: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32),
        ::libloading::Error,
    >,
    pub FPDFPage_SetCropBox: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32),
        ::libloading::Error,
    >,
    pub FPDFPage_SetBleedBox: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32),
        ::libloading::Error,
    >,
    pub FPDFPage_SetTrimBox: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32),
        ::libloading::Error,
    >,
    pub FPDFPage_SetArtBox: Result<
        unsafe extern "C" fn(page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32),
        ::libloading::Error,
    >,
    pub FPDFPage_GetMediaBox: Result<
        unsafe extern "C" fn(
            page: FPDF_PAGE,
            left: *mut f32,
            bottom: *mut f32,
            right: *mut f32,
            top: *mut f32,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPage_GetCropBox: Result<
        unsafe extern "C" fn(
            page: FPDF_PAGE,
            left: *mut f32,
            bottom: *mut f32,
            right: *mut f32,
            top: *mut f32,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPage_GetBleedBox: Result<
        unsafe extern "C" fn(
            page: FPDF_PAGE,
            left: *mut f32,
            bottom: *mut f32,
            right: *mut f32,
            top: *mut f32,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPage_GetTrimBox: Result<
        unsafe extern "C" fn(
            page: FPDF_PAGE,
            left: *mut f32,
            bottom: *mut f32,
            right: *mut f32,
            top: *mut f32,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPage_GetArtBox: Result<
        unsafe extern "C" fn(
            page: FPDF_PAGE,
            left: *mut f32,
            bottom: *mut f32,
            right: *mut f32,
            top: *mut f32,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPage_TransFormWithClip: Result<
        unsafe extern "C" fn(
            page: FPDF_PAGE,
            matrix: *const FS_MATRIX,
            clipRect: *const FS_RECTF,
        ) -> FPDF_BOOL,
        ::libloading::Error,
    >,
    pub FPDFPageObj_TransformClipPath: Result<
        unsafe extern "C" fn(
            page_object: FPDF_PAGEOBJECT,
            a: f64,
            b: f64,
            c: f64,
            d: f64,
            e: f64,
            f: f64,
        ),
        ::libloading::Error,
    >,
    pub FPDFPageObj_GetClipPath: Result<
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> FPDF_CLIPPATH,
        ::libloading::Error,
    >,
    pub FPDFClipPath_CountPaths: Result<
        unsafe extern "C" fn(clip_path: FPDF_CLIPPATH) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFClipPath_CountPathSegments: Result<
        unsafe extern "C" fn(
            clip_path: FPDF_CLIPPATH,
            path_index: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
        ::libloading::Error,
    >,
    pub FPDFClipPath_GetPathSegment: Result<
        unsafe extern "C" fn(
            clip_path: FPDF_CLIPPATH,
            path_index: ::core::ffi::c_int,
            segment_index: ::core::ffi::c_int,
        ) -> FPDF_PATHSEGMENT,
        ::libloading::Error,
    >,
    pub FPDF_CreateClipPath: Result<
        unsafe extern "C" fn(left: f32, bottom: f32, right: f32, top: f32) -> FPDF_CLIPPATH,
        ::libloading::Error,
    >,
    pub FPDF_DestroyClipPath:
        Result<unsafe extern "C" fn(clipPath: FPDF_CLIPPATH), ::libloading::Error>,
    pub FPDFPage_InsertClipPath:
        Result<unsafe extern "C" fn(page: FPDF_PAGE, clipPath: FPDF_CLIPPATH), ::libloading::Error>,
}
impl pdfium {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let library = ::libloading::Library::new(path)?;
        Self::from_library(library)
    }
    pub unsafe fn from_library<L>(library: L) -> Result<Self, ::libloading::Error>
    where
        L: Into<::libloading::Library>,
    {
        let __library = library.into();
        let FPDF_InitLibraryWithConfig = __library
            .get(b"FPDF_InitLibraryWithConfig\0")
            .map(|sym| *sym);
        let FPDF_InitLibrary = __library.get(b"FPDF_InitLibrary\0").map(|sym| *sym);
        let FPDF_DestroyLibrary = __library.get(b"FPDF_DestroyLibrary\0").map(|sym| *sym);
        let FPDF_SetSandBoxPolicy = __library.get(b"FPDF_SetSandBoxPolicy\0").map(|sym| *sym);
        let FPDF_LoadDocument = __library.get(b"FPDF_LoadDocument\0").map(|sym| *sym);
        let FPDF_LoadMemDocument = __library.get(b"FPDF_LoadMemDocument\0").map(|sym| *sym);
        let FPDF_LoadMemDocument64 = __library.get(b"FPDF_LoadMemDocument64\0").map(|sym| *sym);
        let FPDF_LoadCustomDocument = __library.get(b"FPDF_LoadCustomDocument\0").map(|sym| *sym);
        let FPDF_GetFileVersion = __library.get(b"FPDF_GetFileVersion\0").map(|sym| *sym);
        let FPDF_GetLastError = __library.get(b"FPDF_GetLastError\0").map(|sym| *sym);
        let FPDF_DocumentHasValidCrossReferenceTable = __library
            .get(b"FPDF_DocumentHasValidCrossReferenceTable\0")
            .map(|sym| *sym);
        let FPDF_GetTrailerEnds = __library.get(b"FPDF_GetTrailerEnds\0").map(|sym| *sym);
        let FPDF_GetDocPermissions = __library.get(b"FPDF_GetDocPermissions\0").map(|sym| *sym);
        let FPDF_GetDocUserPermissions = __library
            .get(b"FPDF_GetDocUserPermissions\0")
            .map(|sym| *sym);
        let FPDF_GetSecurityHandlerRevision = __library
            .get(b"FPDF_GetSecurityHandlerRevision\0")
            .map(|sym| *sym);
        let FPDF_GetPageCount = __library.get(b"FPDF_GetPageCount\0").map(|sym| *sym);
        let FPDF_LoadPage = __library.get(b"FPDF_LoadPage\0").map(|sym| *sym);
        let FPDF_GetPageWidthF = __library.get(b"FPDF_GetPageWidthF\0").map(|sym| *sym);
        let FPDF_GetPageWidth = __library.get(b"FPDF_GetPageWidth\0").map(|sym| *sym);
        let FPDF_GetPageHeightF = __library.get(b"FPDF_GetPageHeightF\0").map(|sym| *sym);
        let FPDF_GetPageHeight = __library.get(b"FPDF_GetPageHeight\0").map(|sym| *sym);
        let FPDF_GetPageBoundingBox = __library.get(b"FPDF_GetPageBoundingBox\0").map(|sym| *sym);
        let FPDF_GetPageSizeByIndexF = __library.get(b"FPDF_GetPageSizeByIndexF\0").map(|sym| *sym);
        let FPDF_GetPageSizeByIndex = __library.get(b"FPDF_GetPageSizeByIndex\0").map(|sym| *sym);
        let FPDF_RenderPageBitmap = __library.get(b"FPDF_RenderPageBitmap\0").map(|sym| *sym);
        let FPDF_RenderPageBitmapWithMatrix = __library
            .get(b"FPDF_RenderPageBitmapWithMatrix\0")
            .map(|sym| *sym);
        let FPDF_ClosePage = __library.get(b"FPDF_ClosePage\0").map(|sym| *sym);
        let FPDF_CloseDocument = __library.get(b"FPDF_CloseDocument\0").map(|sym| *sym);
        let FPDF_DeviceToPage = __library.get(b"FPDF_DeviceToPage\0").map(|sym| *sym);
        let FPDF_PageToDevice = __library.get(b"FPDF_PageToDevice\0").map(|sym| *sym);
        let FPDFBitmap_Create = __library.get(b"FPDFBitmap_Create\0").map(|sym| *sym);
        let FPDFBitmap_CreateEx = __library.get(b"FPDFBitmap_CreateEx\0").map(|sym| *sym);
        let FPDFBitmap_GetFormat = __library.get(b"FPDFBitmap_GetFormat\0").map(|sym| *sym);
        let FPDFBitmap_FillRect = __library.get(b"FPDFBitmap_FillRect\0").map(|sym| *sym);
        let FPDFBitmap_GetBuffer = __library.get(b"FPDFBitmap_GetBuffer\0").map(|sym| *sym);
        let FPDFBitmap_GetWidth = __library.get(b"FPDFBitmap_GetWidth\0").map(|sym| *sym);
        let FPDFBitmap_GetHeight = __library.get(b"FPDFBitmap_GetHeight\0").map(|sym| *sym);
        let FPDFBitmap_GetStride = __library.get(b"FPDFBitmap_GetStride\0").map(|sym| *sym);
        let FPDFBitmap_Destroy = __library.get(b"FPDFBitmap_Destroy\0").map(|sym| *sym);
        let FPDF_VIEWERREF_GetPrintScaling = __library
            .get(b"FPDF_VIEWERREF_GetPrintScaling\0")
            .map(|sym| *sym);
        let FPDF_VIEWERREF_GetNumCopies = __library
            .get(b"FPDF_VIEWERREF_GetNumCopies\0")
            .map(|sym| *sym);
        let FPDF_VIEWERREF_GetPrintPageRange = __library
            .get(b"FPDF_VIEWERREF_GetPrintPageRange\0")
            .map(|sym| *sym);
        let FPDF_VIEWERREF_GetPrintPageRangeCount = __library
            .get(b"FPDF_VIEWERREF_GetPrintPageRangeCount\0")
            .map(|sym| *sym);
        let FPDF_VIEWERREF_GetPrintPageRangeElement = __library
            .get(b"FPDF_VIEWERREF_GetPrintPageRangeElement\0")
            .map(|sym| *sym);
        let FPDF_VIEWERREF_GetDuplex = __library.get(b"FPDF_VIEWERREF_GetDuplex\0").map(|sym| *sym);
        let FPDF_VIEWERREF_GetName = __library.get(b"FPDF_VIEWERREF_GetName\0").map(|sym| *sym);
        let FPDF_CountNamedDests = __library.get(b"FPDF_CountNamedDests\0").map(|sym| *sym);
        let FPDF_GetNamedDestByName = __library.get(b"FPDF_GetNamedDestByName\0").map(|sym| *sym);
        let FPDF_GetNamedDest = __library.get(b"FPDF_GetNamedDest\0").map(|sym| *sym);
        let FPDF_GetXFAPacketCount = __library.get(b"FPDF_GetXFAPacketCount\0").map(|sym| *sym);
        let FPDF_GetXFAPacketName = __library.get(b"FPDF_GetXFAPacketName\0").map(|sym| *sym);
        let FPDF_GetXFAPacketContent = __library.get(b"FPDF_GetXFAPacketContent\0").map(|sym| *sym);
        let FPDFDOC_InitFormFillEnvironment = __library
            .get(b"FPDFDOC_InitFormFillEnvironment\0")
            .map(|sym| *sym);
        let FPDFDOC_ExitFormFillEnvironment = __library
            .get(b"FPDFDOC_ExitFormFillEnvironment\0")
            .map(|sym| *sym);
        let FORM_OnAfterLoadPage = __library.get(b"FORM_OnAfterLoadPage\0").map(|sym| *sym);
        let FORM_OnBeforeClosePage = __library.get(b"FORM_OnBeforeClosePage\0").map(|sym| *sym);
        let FORM_DoDocumentJSAction = __library.get(b"FORM_DoDocumentJSAction\0").map(|sym| *sym);
        let FORM_DoDocumentOpenAction = __library
            .get(b"FORM_DoDocumentOpenAction\0")
            .map(|sym| *sym);
        let FORM_DoDocumentAAction = __library.get(b"FORM_DoDocumentAAction\0").map(|sym| *sym);
        let FORM_DoPageAAction = __library.get(b"FORM_DoPageAAction\0").map(|sym| *sym);
        let FORM_OnMouseMove = __library.get(b"FORM_OnMouseMove\0").map(|sym| *sym);
        let FORM_OnMouseWheel = __library.get(b"FORM_OnMouseWheel\0").map(|sym| *sym);
        let FORM_OnFocus = __library.get(b"FORM_OnFocus\0").map(|sym| *sym);
        let FORM_OnLButtonDown = __library.get(b"FORM_OnLButtonDown\0").map(|sym| *sym);
        let FORM_OnRButtonDown = __library.get(b"FORM_OnRButtonDown\0").map(|sym| *sym);
        let FORM_OnLButtonUp = __library.get(b"FORM_OnLButtonUp\0").map(|sym| *sym);
        let FORM_OnRButtonUp = __library.get(b"FORM_OnRButtonUp\0").map(|sym| *sym);
        let FORM_OnLButtonDoubleClick = __library
            .get(b"FORM_OnLButtonDoubleClick\0")
            .map(|sym| *sym);
        let FORM_OnKeyDown = __library.get(b"FORM_OnKeyDown\0").map(|sym| *sym);
        let FORM_OnKeyUp = __library.get(b"FORM_OnKeyUp\0").map(|sym| *sym);
        let FORM_OnChar = __library.get(b"FORM_OnChar\0").map(|sym| *sym);
        let FORM_GetFocusedText = __library.get(b"FORM_GetFocusedText\0").map(|sym| *sym);
        let FORM_GetSelectedText = __library.get(b"FORM_GetSelectedText\0").map(|sym| *sym);
        let FORM_ReplaceAndKeepSelection = __library
            .get(b"FORM_ReplaceAndKeepSelection\0")
            .map(|sym| *sym);
        let FORM_ReplaceSelection = __library.get(b"FORM_ReplaceSelection\0").map(|sym| *sym);
        let FORM_SelectAllText = __library.get(b"FORM_SelectAllText\0").map(|sym| *sym);
        let FORM_CanUndo = __library.get(b"FORM_CanUndo\0").map(|sym| *sym);
        let FORM_CanRedo = __library.get(b"FORM_CanRedo\0").map(|sym| *sym);
        let FORM_Undo = __library.get(b"FORM_Undo\0").map(|sym| *sym);
        let FORM_Redo = __library.get(b"FORM_Redo\0").map(|sym| *sym);
        let FORM_ForceToKillFocus = __library.get(b"FORM_ForceToKillFocus\0").map(|sym| *sym);
        let FORM_GetFocusedAnnot = __library.get(b"FORM_GetFocusedAnnot\0").map(|sym| *sym);
        let FORM_SetFocusedAnnot = __library.get(b"FORM_SetFocusedAnnot\0").map(|sym| *sym);
        let FPDFPage_HasFormFieldAtPoint = __library
            .get(b"FPDFPage_HasFormFieldAtPoint\0")
            .map(|sym| *sym);
        let FPDFPage_FormFieldZOrderAtPoint = __library
            .get(b"FPDFPage_FormFieldZOrderAtPoint\0")
            .map(|sym| *sym);
        let FPDF_SetFormFieldHighlightColor = __library
            .get(b"FPDF_SetFormFieldHighlightColor\0")
            .map(|sym| *sym);
        let FPDF_SetFormFieldHighlightAlpha = __library
            .get(b"FPDF_SetFormFieldHighlightAlpha\0")
            .map(|sym| *sym);
        let FPDF_RemoveFormFieldHighlight = __library
            .get(b"FPDF_RemoveFormFieldHighlight\0")
            .map(|sym| *sym);
        let FPDF_FFLDraw = __library.get(b"FPDF_FFLDraw\0").map(|sym| *sym);
        let FPDF_GetFormType = __library.get(b"FPDF_GetFormType\0").map(|sym| *sym);
        let FORM_SetIndexSelected = __library.get(b"FORM_SetIndexSelected\0").map(|sym| *sym);
        let FORM_IsIndexSelected = __library.get(b"FORM_IsIndexSelected\0").map(|sym| *sym);
        let FORM_GetTextDirection = __library.get(b"FORM_GetTextDirection\0").map(|sym| *sym);
        let FORM_SetTextDirection = __library.get(b"FORM_SetTextDirection\0").map(|sym| *sym);
        let FPDF_LoadXFA = __library.get(b"FPDF_LoadXFA\0").map(|sym| *sym);
        let FPDFAnnot_IsSupportedSubtype = __library
            .get(b"FPDFAnnot_IsSupportedSubtype\0")
            .map(|sym| *sym);
        let FPDFPage_CreateAnnot = __library.get(b"FPDFPage_CreateAnnot\0").map(|sym| *sym);
        let FPDFPage_GetAnnotCount = __library.get(b"FPDFPage_GetAnnotCount\0").map(|sym| *sym);
        let FPDFPage_GetAnnot = __library.get(b"FPDFPage_GetAnnot\0").map(|sym| *sym);
        let FPDFPage_GetAnnotIndex = __library.get(b"FPDFPage_GetAnnotIndex\0").map(|sym| *sym);
        let FPDFPage_CloseAnnot = __library.get(b"FPDFPage_CloseAnnot\0").map(|sym| *sym);
        let FPDFPage_RemoveAnnot = __library.get(b"FPDFPage_RemoveAnnot\0").map(|sym| *sym);
        let FPDFAnnot_GetSubtype = __library.get(b"FPDFAnnot_GetSubtype\0").map(|sym| *sym);
        let FPDFAnnot_IsObjectSupportedSubtype = __library
            .get(b"FPDFAnnot_IsObjectSupportedSubtype\0")
            .map(|sym| *sym);
        let FPDFAnnot_UpdateObject = __library.get(b"FPDFAnnot_UpdateObject\0").map(|sym| *sym);
        let FPDFAnnot_AddInkStroke = __library.get(b"FPDFAnnot_AddInkStroke\0").map(|sym| *sym);
        let FPDFAnnot_RemoveInkList = __library.get(b"FPDFAnnot_RemoveInkList\0").map(|sym| *sym);
        let FPDFAnnot_AppendObject = __library.get(b"FPDFAnnot_AppendObject\0").map(|sym| *sym);
        let FPDFAnnot_GetObjectCount = __library.get(b"FPDFAnnot_GetObjectCount\0").map(|sym| *sym);
        let FPDFAnnot_GetObject = __library.get(b"FPDFAnnot_GetObject\0").map(|sym| *sym);
        let FPDFAnnot_RemoveObject = __library.get(b"FPDFAnnot_RemoveObject\0").map(|sym| *sym);
        let FPDFAnnot_SetColor = __library.get(b"FPDFAnnot_SetColor\0").map(|sym| *sym);
        let FPDFAnnot_GetColor = __library.get(b"FPDFAnnot_GetColor\0").map(|sym| *sym);
        let FPDFAnnot_HasAttachmentPoints = __library
            .get(b"FPDFAnnot_HasAttachmentPoints\0")
            .map(|sym| *sym);
        let FPDFAnnot_SetAttachmentPoints = __library
            .get(b"FPDFAnnot_SetAttachmentPoints\0")
            .map(|sym| *sym);
        let FPDFAnnot_AppendAttachmentPoints = __library
            .get(b"FPDFAnnot_AppendAttachmentPoints\0")
            .map(|sym| *sym);
        let FPDFAnnot_CountAttachmentPoints = __library
            .get(b"FPDFAnnot_CountAttachmentPoints\0")
            .map(|sym| *sym);
        let FPDFAnnot_GetAttachmentPoints = __library
            .get(b"FPDFAnnot_GetAttachmentPoints\0")
            .map(|sym| *sym);
        let FPDFAnnot_SetRect = __library.get(b"FPDFAnnot_SetRect\0").map(|sym| *sym);
        let FPDFAnnot_GetRect = __library.get(b"FPDFAnnot_GetRect\0").map(|sym| *sym);
        let FPDFAnnot_GetVertices = __library.get(b"FPDFAnnot_GetVertices\0").map(|sym| *sym);
        let FPDFAnnot_GetInkListCount = __library
            .get(b"FPDFAnnot_GetInkListCount\0")
            .map(|sym| *sym);
        let FPDFAnnot_GetInkListPath = __library.get(b"FPDFAnnot_GetInkListPath\0").map(|sym| *sym);
        let FPDFAnnot_GetLine = __library.get(b"FPDFAnnot_GetLine\0").map(|sym| *sym);
        let FPDFAnnot_SetBorder = __library.get(b"FPDFAnnot_SetBorder\0").map(|sym| *sym);
        let FPDFAnnot_GetBorder = __library.get(b"FPDFAnnot_GetBorder\0").map(|sym| *sym);
        let FPDFAnnot_GetFormAdditionalActionJavaScript = __library
            .get(b"FPDFAnnot_GetFormAdditionalActionJavaScript\0")
            .map(|sym| *sym);
        let FPDFAnnot_HasKey = __library.get(b"FPDFAnnot_HasKey\0").map(|sym| *sym);
        let FPDFAnnot_GetValueType = __library.get(b"FPDFAnnot_GetValueType\0").map(|sym| *sym);
        let FPDFAnnot_SetStringValue = __library.get(b"FPDFAnnot_SetStringValue\0").map(|sym| *sym);
        let FPDFAnnot_GetStringValue = __library.get(b"FPDFAnnot_GetStringValue\0").map(|sym| *sym);
        let FPDFAnnot_GetNumberValue = __library.get(b"FPDFAnnot_GetNumberValue\0").map(|sym| *sym);
        let FPDFAnnot_SetAP = __library.get(b"FPDFAnnot_SetAP\0").map(|sym| *sym);
        let FPDFAnnot_GetAP = __library.get(b"FPDFAnnot_GetAP\0").map(|sym| *sym);
        let FPDFAnnot_GetLinkedAnnot = __library.get(b"FPDFAnnot_GetLinkedAnnot\0").map(|sym| *sym);
        let FPDFAnnot_GetFlags = __library.get(b"FPDFAnnot_GetFlags\0").map(|sym| *sym);
        let FPDFAnnot_SetFlags = __library.get(b"FPDFAnnot_SetFlags\0").map(|sym| *sym);
        let FPDFAnnot_GetFormFieldFlags = __library
            .get(b"FPDFAnnot_GetFormFieldFlags\0")
            .map(|sym| *sym);
        let FPDFAnnot_SetFormFieldFlags = __library
            .get(b"FPDFAnnot_SetFormFieldFlags\0")
            .map(|sym| *sym);
        let FPDFAnnot_GetFormFieldAtPoint = __library
            .get(b"FPDFAnnot_GetFormFieldAtPoint\0")
            .map(|sym| *sym);
        let FPDFAnnot_GetFormFieldName = __library
            .get(b"FPDFAnnot_GetFormFieldName\0")
            .map(|sym| *sym);
        let FPDFAnnot_GetFormFieldAlternateName = __library
            .get(b"FPDFAnnot_GetFormFieldAlternateName\0")
            .map(|sym| *sym);
        let FPDFAnnot_GetFormFieldType = __library
            .get(b"FPDFAnnot_GetFormFieldType\0")
            .map(|sym| *sym);
        let FPDFAnnot_GetFormFieldValue = __library
            .get(b"FPDFAnnot_GetFormFieldValue\0")
            .map(|sym| *sym);
        let FPDFAnnot_GetOptionCount = __library.get(b"FPDFAnnot_GetOptionCount\0").map(|sym| *sym);
        let FPDFAnnot_GetOptionLabel = __library.get(b"FPDFAnnot_GetOptionLabel\0").map(|sym| *sym);
        let FPDFAnnot_IsOptionSelected = __library
            .get(b"FPDFAnnot_IsOptionSelected\0")
            .map(|sym| *sym);
        let FPDFAnnot_GetFontSize = __library.get(b"FPDFAnnot_GetFontSize\0").map(|sym| *sym);
        let FPDFAnnot_SetFontColor = __library.get(b"FPDFAnnot_SetFontColor\0").map(|sym| *sym);
        let FPDFAnnot_GetFontColor = __library.get(b"FPDFAnnot_GetFontColor\0").map(|sym| *sym);
        let FPDFAnnot_IsChecked = __library.get(b"FPDFAnnot_IsChecked\0").map(|sym| *sym);
        let FPDFAnnot_SetFocusableSubtypes = __library
            .get(b"FPDFAnnot_SetFocusableSubtypes\0")
            .map(|sym| *sym);
        let FPDFAnnot_GetFocusableSubtypesCount = __library
            .get(b"FPDFAnnot_GetFocusableSubtypesCount\0")
            .map(|sym| *sym);
        let FPDFAnnot_GetFocusableSubtypes = __library
            .get(b"FPDFAnnot_GetFocusableSubtypes\0")
            .map(|sym| *sym);
        let FPDFAnnot_GetLink = __library.get(b"FPDFAnnot_GetLink\0").map(|sym| *sym);
        let FPDFAnnot_GetFormControlCount = __library
            .get(b"FPDFAnnot_GetFormControlCount\0")
            .map(|sym| *sym);
        let FPDFAnnot_GetFormControlIndex = __library
            .get(b"FPDFAnnot_GetFormControlIndex\0")
            .map(|sym| *sym);
        let FPDFAnnot_GetFormFieldExportValue = __library
            .get(b"FPDFAnnot_GetFormFieldExportValue\0")
            .map(|sym| *sym);
        let FPDFAnnot_SetURI = __library.get(b"FPDFAnnot_SetURI\0").map(|sym| *sym);
        let FPDFAnnot_GetFileAttachment = __library
            .get(b"FPDFAnnot_GetFileAttachment\0")
            .map(|sym| *sym);
        let FPDFAnnot_AddFileAttachment = __library
            .get(b"FPDFAnnot_AddFileAttachment\0")
            .map(|sym| *sym);
        let FPDFDoc_GetAttachmentCount = __library
            .get(b"FPDFDoc_GetAttachmentCount\0")
            .map(|sym| *sym);
        let FPDFDoc_AddAttachment = __library.get(b"FPDFDoc_AddAttachment\0").map(|sym| *sym);
        let FPDFDoc_GetAttachment = __library.get(b"FPDFDoc_GetAttachment\0").map(|sym| *sym);
        let FPDFDoc_DeleteAttachment = __library.get(b"FPDFDoc_DeleteAttachment\0").map(|sym| *sym);
        let FPDFAttachment_GetName = __library.get(b"FPDFAttachment_GetName\0").map(|sym| *sym);
        let FPDFAttachment_HasKey = __library.get(b"FPDFAttachment_HasKey\0").map(|sym| *sym);
        let FPDFAttachment_GetValueType = __library
            .get(b"FPDFAttachment_GetValueType\0")
            .map(|sym| *sym);
        let FPDFAttachment_SetStringValue = __library
            .get(b"FPDFAttachment_SetStringValue\0")
            .map(|sym| *sym);
        let FPDFAttachment_GetStringValue = __library
            .get(b"FPDFAttachment_GetStringValue\0")
            .map(|sym| *sym);
        let FPDFAttachment_SetDescription = __library
            .get(b"FPDFAttachment_SetDescription\0")
            .map(|sym| *sym);
        let FPDFAttachment_GetDescription = __library
            .get(b"FPDFAttachment_GetDescription\0")
            .map(|sym| *sym);
        let FPDFAttachment_SetFile = __library.get(b"FPDFAttachment_SetFile\0").map(|sym| *sym);
        let FPDFAttachment_GetFile = __library.get(b"FPDFAttachment_GetFile\0").map(|sym| *sym);
        let FPDFAttachment_GetSubtype = __library
            .get(b"FPDFAttachment_GetSubtype\0")
            .map(|sym| *sym);
        let FPDFAttachment_GetAFRelationship = __library
            .get(b"FPDFAttachment_GetAFRelationship\0")
            .map(|sym| *sym);
        let FPDFCatalog_IsTagged = __library.get(b"FPDFCatalog_IsTagged\0").map(|sym| *sym);
        let FPDFCatalog_GetLanguage = __library.get(b"FPDFCatalog_GetLanguage\0").map(|sym| *sym);
        let FPDFCatalog_SetLanguage = __library.get(b"FPDFCatalog_SetLanguage\0").map(|sym| *sym);
        let FPDFAvail_Create = __library.get(b"FPDFAvail_Create\0").map(|sym| *sym);
        let FPDFAvail_Destroy = __library.get(b"FPDFAvail_Destroy\0").map(|sym| *sym);
        let FPDFAvail_IsDocAvail = __library.get(b"FPDFAvail_IsDocAvail\0").map(|sym| *sym);
        let FPDFAvail_GetDocument = __library.get(b"FPDFAvail_GetDocument\0").map(|sym| *sym);
        let FPDFAvail_GetFirstPageNum = __library
            .get(b"FPDFAvail_GetFirstPageNum\0")
            .map(|sym| *sym);
        let FPDFAvail_IsPageAvail = __library.get(b"FPDFAvail_IsPageAvail\0").map(|sym| *sym);
        let FPDFAvail_IsFormAvail = __library.get(b"FPDFAvail_IsFormAvail\0").map(|sym| *sym);
        let FPDFAvail_IsLinearized = __library.get(b"FPDFAvail_IsLinearized\0").map(|sym| *sym);
        let FPDFBookmark_GetFirstChild = __library
            .get(b"FPDFBookmark_GetFirstChild\0")
            .map(|sym| *sym);
        let FPDFBookmark_GetNextSibling = __library
            .get(b"FPDFBookmark_GetNextSibling\0")
            .map(|sym| *sym);
        let FPDFBookmark_GetTitle = __library.get(b"FPDFBookmark_GetTitle\0").map(|sym| *sym);
        let FPDFBookmark_GetCount = __library.get(b"FPDFBookmark_GetCount\0").map(|sym| *sym);
        let FPDFBookmark_Find = __library.get(b"FPDFBookmark_Find\0").map(|sym| *sym);
        let FPDFBookmark_GetDest = __library.get(b"FPDFBookmark_GetDest\0").map(|sym| *sym);
        let FPDFBookmark_GetAction = __library.get(b"FPDFBookmark_GetAction\0").map(|sym| *sym);
        let FPDFBookmark_GetColor = __library.get(b"FPDFBookmark_GetColor\0").map(|sym| *sym);
        let FPDFBookmark_GetStyle = __library.get(b"FPDFBookmark_GetStyle\0").map(|sym| *sym);
        let FPDFAction_GetType = __library.get(b"FPDFAction_GetType\0").map(|sym| *sym);
        let FPDFAction_GetDest = __library.get(b"FPDFAction_GetDest\0").map(|sym| *sym);
        let FPDFAction_GetFilePath = __library.get(b"FPDFAction_GetFilePath\0").map(|sym| *sym);
        let FPDFAction_GetURIPath = __library.get(b"FPDFAction_GetURIPath\0").map(|sym| *sym);
        let FPDFDest_GetDestPageIndex = __library
            .get(b"FPDFDest_GetDestPageIndex\0")
            .map(|sym| *sym);
        let FPDFDest_GetView = __library.get(b"FPDFDest_GetView\0").map(|sym| *sym);
        let FPDFDest_GetLocationInPage = __library
            .get(b"FPDFDest_GetLocationInPage\0")
            .map(|sym| *sym);
        let FPDFLink_GetLinkAtPoint = __library.get(b"FPDFLink_GetLinkAtPoint\0").map(|sym| *sym);
        let FPDFLink_GetLinkZOrderAtPoint = __library
            .get(b"FPDFLink_GetLinkZOrderAtPoint\0")
            .map(|sym| *sym);
        let FPDFLink_GetDest = __library.get(b"FPDFLink_GetDest\0").map(|sym| *sym);
        let FPDFLink_GetAction = __library.get(b"FPDFLink_GetAction\0").map(|sym| *sym);
        let FPDFLink_Enumerate = __library.get(b"FPDFLink_Enumerate\0").map(|sym| *sym);
        let FPDFLink_GetAnnot = __library.get(b"FPDFLink_GetAnnot\0").map(|sym| *sym);
        let FPDFLink_GetAnnotRect = __library.get(b"FPDFLink_GetAnnotRect\0").map(|sym| *sym);
        let FPDFLink_CountQuadPoints = __library.get(b"FPDFLink_CountQuadPoints\0").map(|sym| *sym);
        let FPDFLink_GetQuadPoints = __library.get(b"FPDFLink_GetQuadPoints\0").map(|sym| *sym);
        let FPDF_GetPageAAction = __library.get(b"FPDF_GetPageAAction\0").map(|sym| *sym);
        let FPDF_GetFileIdentifier = __library.get(b"FPDF_GetFileIdentifier\0").map(|sym| *sym);
        let FPDF_GetMetaText = __library.get(b"FPDF_GetMetaText\0").map(|sym| *sym);
        let FPDF_GetPageLabel = __library.get(b"FPDF_GetPageLabel\0").map(|sym| *sym);
        let FPDF_CreateNewDocument = __library.get(b"FPDF_CreateNewDocument\0").map(|sym| *sym);
        let FPDFPage_New = __library.get(b"FPDFPage_New\0").map(|sym| *sym);
        let FPDFPage_Delete = __library.get(b"FPDFPage_Delete\0").map(|sym| *sym);
        let FPDF_MovePages = __library.get(b"FPDF_MovePages\0").map(|sym| *sym);
        let FPDFPage_GetRotation = __library.get(b"FPDFPage_GetRotation\0").map(|sym| *sym);
        let FPDFPage_SetRotation = __library.get(b"FPDFPage_SetRotation\0").map(|sym| *sym);
        let FPDFPage_InsertObject = __library.get(b"FPDFPage_InsertObject\0").map(|sym| *sym);
        let FPDFPage_InsertObjectAtIndex = __library
            .get(b"FPDFPage_InsertObjectAtIndex\0")
            .map(|sym| *sym);
        let FPDFPage_RemoveObject = __library.get(b"FPDFPage_RemoveObject\0").map(|sym| *sym);
        let FPDFPage_CountObjects = __library.get(b"FPDFPage_CountObjects\0").map(|sym| *sym);
        let FPDFPage_GetObject = __library.get(b"FPDFPage_GetObject\0").map(|sym| *sym);
        let FPDFPage_HasTransparency = __library.get(b"FPDFPage_HasTransparency\0").map(|sym| *sym);
        let FPDFPage_GenerateContent = __library.get(b"FPDFPage_GenerateContent\0").map(|sym| *sym);
        let FPDFPageObj_Destroy = __library.get(b"FPDFPageObj_Destroy\0").map(|sym| *sym);
        let FPDFPageObj_HasTransparency = __library
            .get(b"FPDFPageObj_HasTransparency\0")
            .map(|sym| *sym);
        let FPDFPageObj_GetType = __library.get(b"FPDFPageObj_GetType\0").map(|sym| *sym);
        let FPDFPageObj_GetIsActive = __library.get(b"FPDFPageObj_GetIsActive\0").map(|sym| *sym);
        let FPDFPageObj_SetIsActive = __library.get(b"FPDFPageObj_SetIsActive\0").map(|sym| *sym);
        let FPDFPageObj_Transform = __library.get(b"FPDFPageObj_Transform\0").map(|sym| *sym);
        let FPDFPageObj_TransformF = __library.get(b"FPDFPageObj_TransformF\0").map(|sym| *sym);
        let FPDFPageObj_GetMatrix = __library.get(b"FPDFPageObj_GetMatrix\0").map(|sym| *sym);
        let FPDFPageObj_SetMatrix = __library.get(b"FPDFPageObj_SetMatrix\0").map(|sym| *sym);
        let FPDFPage_TransformAnnots = __library.get(b"FPDFPage_TransformAnnots\0").map(|sym| *sym);
        let FPDFPageObj_NewImageObj = __library.get(b"FPDFPageObj_NewImageObj\0").map(|sym| *sym);
        let FPDFPageObj_GetMarkedContentID = __library
            .get(b"FPDFPageObj_GetMarkedContentID\0")
            .map(|sym| *sym);
        let FPDFPageObj_CountMarks = __library.get(b"FPDFPageObj_CountMarks\0").map(|sym| *sym);
        let FPDFPageObj_GetMark = __library.get(b"FPDFPageObj_GetMark\0").map(|sym| *sym);
        let FPDFPageObj_AddMark = __library.get(b"FPDFPageObj_AddMark\0").map(|sym| *sym);
        let FPDFPageObj_AddExistingMark = __library
            .get(b"FPDFPageObj_AddExistingMark\0")
            .map(|sym| *sym);
        let FPDFPageObj_RemoveMark = __library.get(b"FPDFPageObj_RemoveMark\0").map(|sym| *sym);
        let FPDFPageObjMark_GetName = __library.get(b"FPDFPageObjMark_GetName\0").map(|sym| *sym);
        let FPDFPageObjMark_CountParams = __library
            .get(b"FPDFPageObjMark_CountParams\0")
            .map(|sym| *sym);
        let FPDFPageObjMark_GetParamKey = __library
            .get(b"FPDFPageObjMark_GetParamKey\0")
            .map(|sym| *sym);
        let FPDFPageObjMark_GetParamValueType = __library
            .get(b"FPDFPageObjMark_GetParamValueType\0")
            .map(|sym| *sym);
        let FPDFPageObjMark_GetParamIntValue = __library
            .get(b"FPDFPageObjMark_GetParamIntValue\0")
            .map(|sym| *sym);
        let FPDFPageObjMark_GetParamFloatValue = __library
            .get(b"FPDFPageObjMark_GetParamFloatValue\0")
            .map(|sym| *sym);
        let FPDFPageObjMark_GetParamStringValue = __library
            .get(b"FPDFPageObjMark_GetParamStringValue\0")
            .map(|sym| *sym);
        let FPDFPageObjMark_GetParamBlobValue = __library
            .get(b"FPDFPageObjMark_GetParamBlobValue\0")
            .map(|sym| *sym);
        let FPDFPageObjMark_SetIntParam = __library
            .get(b"FPDFPageObjMark_SetIntParam\0")
            .map(|sym| *sym);
        let FPDFPageObjMark_SetFloatParam = __library
            .get(b"FPDFPageObjMark_SetFloatParam\0")
            .map(|sym| *sym);
        let FPDFPageObjMark_SetStringParam = __library
            .get(b"FPDFPageObjMark_SetStringParam\0")
            .map(|sym| *sym);
        let FPDFPageObjMark_SetBlobParam = __library
            .get(b"FPDFPageObjMark_SetBlobParam\0")
            .map(|sym| *sym);
        let FPDFPageObjMark_RemoveParam = __library
            .get(b"FPDFPageObjMark_RemoveParam\0")
            .map(|sym| *sym);
        let FPDFImageObj_LoadJpegFile = __library
            .get(b"FPDFImageObj_LoadJpegFile\0")
            .map(|sym| *sym);
        let FPDFImageObj_LoadJpegFileInline = __library
            .get(b"FPDFImageObj_LoadJpegFileInline\0")
            .map(|sym| *sym);
        let FPDFImageObj_SetMatrix = __library.get(b"FPDFImageObj_SetMatrix\0").map(|sym| *sym);
        let FPDFImageObj_SetBitmap = __library.get(b"FPDFImageObj_SetBitmap\0").map(|sym| *sym);
        let FPDFImageObj_GetBitmap = __library.get(b"FPDFImageObj_GetBitmap\0").map(|sym| *sym);
        let FPDFImageObj_GetRenderedBitmap = __library
            .get(b"FPDFImageObj_GetRenderedBitmap\0")
            .map(|sym| *sym);
        let FPDFImageObj_GetImageDataDecoded = __library
            .get(b"FPDFImageObj_GetImageDataDecoded\0")
            .map(|sym| *sym);
        let FPDFImageObj_GetImageDataRaw = __library
            .get(b"FPDFImageObj_GetImageDataRaw\0")
            .map(|sym| *sym);
        let FPDFImageObj_GetImageFilterCount = __library
            .get(b"FPDFImageObj_GetImageFilterCount\0")
            .map(|sym| *sym);
        let FPDFImageObj_GetImageFilter = __library
            .get(b"FPDFImageObj_GetImageFilter\0")
            .map(|sym| *sym);
        let FPDFImageObj_GetImageMetadata = __library
            .get(b"FPDFImageObj_GetImageMetadata\0")
            .map(|sym| *sym);
        let FPDFImageObj_GetImagePixelSize = __library
            .get(b"FPDFImageObj_GetImagePixelSize\0")
            .map(|sym| *sym);
        let FPDFImageObj_GetIccProfileDataDecoded = __library
            .get(b"FPDFImageObj_GetIccProfileDataDecoded\0")
            .map(|sym| *sym);
        let FPDFPageObj_CreateNewPath = __library
            .get(b"FPDFPageObj_CreateNewPath\0")
            .map(|sym| *sym);
        let FPDFPageObj_CreateNewRect = __library
            .get(b"FPDFPageObj_CreateNewRect\0")
            .map(|sym| *sym);
        let FPDFPageObj_GetBounds = __library.get(b"FPDFPageObj_GetBounds\0").map(|sym| *sym);
        let FPDFPageObj_GetRotatedBounds = __library
            .get(b"FPDFPageObj_GetRotatedBounds\0")
            .map(|sym| *sym);
        let FPDFPageObj_SetBlendMode = __library.get(b"FPDFPageObj_SetBlendMode\0").map(|sym| *sym);
        let FPDFPageObj_SetStrokeColor = __library
            .get(b"FPDFPageObj_SetStrokeColor\0")
            .map(|sym| *sym);
        let FPDFPageObj_GetStrokeColor = __library
            .get(b"FPDFPageObj_GetStrokeColor\0")
            .map(|sym| *sym);
        let FPDFPageObj_SetStrokeWidth = __library
            .get(b"FPDFPageObj_SetStrokeWidth\0")
            .map(|sym| *sym);
        let FPDFPageObj_GetStrokeWidth = __library
            .get(b"FPDFPageObj_GetStrokeWidth\0")
            .map(|sym| *sym);
        let FPDFPageObj_GetLineJoin = __library.get(b"FPDFPageObj_GetLineJoin\0").map(|sym| *sym);
        let FPDFPageObj_SetLineJoin = __library.get(b"FPDFPageObj_SetLineJoin\0").map(|sym| *sym);
        let FPDFPageObj_GetLineCap = __library.get(b"FPDFPageObj_GetLineCap\0").map(|sym| *sym);
        let FPDFPageObj_SetLineCap = __library.get(b"FPDFPageObj_SetLineCap\0").map(|sym| *sym);
        let FPDFPageObj_SetFillColor = __library.get(b"FPDFPageObj_SetFillColor\0").map(|sym| *sym);
        let FPDFPageObj_GetFillColor = __library.get(b"FPDFPageObj_GetFillColor\0").map(|sym| *sym);
        let FPDFPageObj_GetDashPhase = __library.get(b"FPDFPageObj_GetDashPhase\0").map(|sym| *sym);
        let FPDFPageObj_SetDashPhase = __library.get(b"FPDFPageObj_SetDashPhase\0").map(|sym| *sym);
        let FPDFPageObj_GetDashCount = __library.get(b"FPDFPageObj_GetDashCount\0").map(|sym| *sym);
        let FPDFPageObj_GetDashArray = __library.get(b"FPDFPageObj_GetDashArray\0").map(|sym| *sym);
        let FPDFPageObj_SetDashArray = __library.get(b"FPDFPageObj_SetDashArray\0").map(|sym| *sym);
        let FPDFPageObj_GetRenderedStrokePattern = __library
            .get(b"FPDFPageObj_GetRenderedStrokePattern\0")
            .map(|sym| *sym);
        let FPDFPageObj_GetRenderedFillPattern = __library
            .get(b"FPDFPageObj_GetRenderedFillPattern\0")
            .map(|sym| *sym);
        let FPDFPath_CountSegments = __library.get(b"FPDFPath_CountSegments\0").map(|sym| *sym);
        let FPDFPath_GetPathSegment = __library.get(b"FPDFPath_GetPathSegment\0").map(|sym| *sym);
        let FPDFPathSegment_GetPoint = __library.get(b"FPDFPathSegment_GetPoint\0").map(|sym| *sym);
        let FPDFPathSegment_GetType = __library.get(b"FPDFPathSegment_GetType\0").map(|sym| *sym);
        let FPDFPathSegment_GetClose = __library.get(b"FPDFPathSegment_GetClose\0").map(|sym| *sym);
        let FPDFPath_MoveTo = __library.get(b"FPDFPath_MoveTo\0").map(|sym| *sym);
        let FPDFPath_LineTo = __library.get(b"FPDFPath_LineTo\0").map(|sym| *sym);
        let FPDFPath_BezierTo = __library.get(b"FPDFPath_BezierTo\0").map(|sym| *sym);
        let FPDFPath_GetBezierControlPoints = __library
            .get(b"FPDFPath_GetBezierControlPoints\0")
            .map(|sym| *sym);
        let FPDFPath_Close = __library.get(b"FPDFPath_Close\0").map(|sym| *sym);
        let FPDFPath_SetDrawMode = __library.get(b"FPDFPath_SetDrawMode\0").map(|sym| *sym);
        let FPDFPath_GetDrawMode = __library.get(b"FPDFPath_GetDrawMode\0").map(|sym| *sym);
        let FPDFPageObj_NewTextObj = __library.get(b"FPDFPageObj_NewTextObj\0").map(|sym| *sym);
        let FPDFText_SetText = __library.get(b"FPDFText_SetText\0").map(|sym| *sym);
        let FPDFText_SetCharcodes = __library.get(b"FPDFText_SetCharcodes\0").map(|sym| *sym);
        let FPDFText_SetPositions = __library.get(b"FPDFText_SetPositions\0").map(|sym| *sym);
        let FPDFText_LoadFont = __library.get(b"FPDFText_LoadFont\0").map(|sym| *sym);
        let FPDFText_LoadStandardFont = __library
            .get(b"FPDFText_LoadStandardFont\0")
            .map(|sym| *sym);
        let FPDFText_LoadCidType2Font = __library
            .get(b"FPDFText_LoadCidType2Font\0")
            .map(|sym| *sym);
        let FPDFTextObj_GetFontSize = __library.get(b"FPDFTextObj_GetFontSize\0").map(|sym| *sym);
        let FPDFTextObj_SetFontSize = __library.get(b"FPDFTextObj_SetFontSize\0").map(|sym| *sym);
        let FPDFFont_Close = __library.get(b"FPDFFont_Close\0").map(|sym| *sym);
        let FPDFPageObj_CreateTextObj = __library
            .get(b"FPDFPageObj_CreateTextObj\0")
            .map(|sym| *sym);
        let FPDFTextObj_GetTextRenderMode = __library
            .get(b"FPDFTextObj_GetTextRenderMode\0")
            .map(|sym| *sym);
        let FPDFTextObj_SetTextRenderMode = __library
            .get(b"FPDFTextObj_SetTextRenderMode\0")
            .map(|sym| *sym);
        let FPDFTextObj_GetText = __library.get(b"FPDFTextObj_GetText\0").map(|sym| *sym);
        let FPDFTextObj_GetRenderedBitmap = __library
            .get(b"FPDFTextObj_GetRenderedBitmap\0")
            .map(|sym| *sym);
        let FPDFTextObj_GetFont = __library.get(b"FPDFTextObj_GetFont\0").map(|sym| *sym);
        let FPDFFont_GetBaseFontName = __library.get(b"FPDFFont_GetBaseFontName\0").map(|sym| *sym);
        let FPDFFont_GetFamilyName = __library.get(b"FPDFFont_GetFamilyName\0").map(|sym| *sym);
        let FPDFFont_GetFontData = __library.get(b"FPDFFont_GetFontData\0").map(|sym| *sym);
        let FPDFFont_GetIsEmbedded = __library.get(b"FPDFFont_GetIsEmbedded\0").map(|sym| *sym);
        let FPDFFont_GetFlags = __library.get(b"FPDFFont_GetFlags\0").map(|sym| *sym);
        let FPDFFont_GetWeight = __library.get(b"FPDFFont_GetWeight\0").map(|sym| *sym);
        let FPDFFont_GetItalicAngle = __library.get(b"FPDFFont_GetItalicAngle\0").map(|sym| *sym);
        let FPDFFont_GetAscent = __library.get(b"FPDFFont_GetAscent\0").map(|sym| *sym);
        let FPDFFont_GetDescent = __library.get(b"FPDFFont_GetDescent\0").map(|sym| *sym);
        let FPDFFont_GetGlyphWidth = __library.get(b"FPDFFont_GetGlyphWidth\0").map(|sym| *sym);
        let FPDFFont_GetGlyphPath = __library.get(b"FPDFFont_GetGlyphPath\0").map(|sym| *sym);
        let FPDFGlyphPath_CountGlyphSegments = __library
            .get(b"FPDFGlyphPath_CountGlyphSegments\0")
            .map(|sym| *sym);
        let FPDFGlyphPath_GetGlyphPathSegment = __library
            .get(b"FPDFGlyphPath_GetGlyphPathSegment\0")
            .map(|sym| *sym);
        let FPDFFormObj_CountObjects = __library.get(b"FPDFFormObj_CountObjects\0").map(|sym| *sym);
        let FPDFFormObj_GetObject = __library.get(b"FPDFFormObj_GetObject\0").map(|sym| *sym);
        let FPDFFormObj_RemoveObject = __library.get(b"FPDFFormObj_RemoveObject\0").map(|sym| *sym);
        let FSDK_SetUnSpObjProcessHandler = __library
            .get(b"FSDK_SetUnSpObjProcessHandler\0")
            .map(|sym| *sym);
        let FSDK_SetTimeFunction = __library.get(b"FSDK_SetTimeFunction\0").map(|sym| *sym);
        let FSDK_SetLocaltimeFunction = __library
            .get(b"FSDK_SetLocaltimeFunction\0")
            .map(|sym| *sym);
        let FPDFDoc_GetPageMode = __library.get(b"FPDFDoc_GetPageMode\0").map(|sym| *sym);
        let FPDFPage_Flatten = __library.get(b"FPDFPage_Flatten\0").map(|sym| *sym);
        let FPDFDoc_GetJavaScriptActionCount = __library
            .get(b"FPDFDoc_GetJavaScriptActionCount\0")
            .map(|sym| *sym);
        let FPDFDoc_GetJavaScriptAction = __library
            .get(b"FPDFDoc_GetJavaScriptAction\0")
            .map(|sym| *sym);
        let FPDFDoc_CloseJavaScriptAction = __library
            .get(b"FPDFDoc_CloseJavaScriptAction\0")
            .map(|sym| *sym);
        let FPDFJavaScriptAction_GetName = __library
            .get(b"FPDFJavaScriptAction_GetName\0")
            .map(|sym| *sym);
        let FPDFJavaScriptAction_GetScript = __library
            .get(b"FPDFJavaScriptAction_GetScript\0")
            .map(|sym| *sym);
        let FPDF_ImportPagesByIndex = __library.get(b"FPDF_ImportPagesByIndex\0").map(|sym| *sym);
        let FPDF_ImportPages = __library.get(b"FPDF_ImportPages\0").map(|sym| *sym);
        let FPDF_ImportNPagesToOne = __library.get(b"FPDF_ImportNPagesToOne\0").map(|sym| *sym);
        let FPDF_NewXObjectFromPage = __library.get(b"FPDF_NewXObjectFromPage\0").map(|sym| *sym);
        let FPDF_CloseXObject = __library.get(b"FPDF_CloseXObject\0").map(|sym| *sym);
        let FPDF_NewFormObjectFromXObject = __library
            .get(b"FPDF_NewFormObjectFromXObject\0")
            .map(|sym| *sym);
        let FPDF_CopyViewerPreferences = __library
            .get(b"FPDF_CopyViewerPreferences\0")
            .map(|sym| *sym);
        let FPDF_RenderPageBitmapWithColorScheme_Start = __library
            .get(b"FPDF_RenderPageBitmapWithColorScheme_Start\0")
            .map(|sym| *sym);
        let FPDF_RenderPageBitmap_Start = __library
            .get(b"FPDF_RenderPageBitmap_Start\0")
            .map(|sym| *sym);
        let FPDF_RenderPage_Continue = __library.get(b"FPDF_RenderPage_Continue\0").map(|sym| *sym);
        let FPDF_RenderPage_Close = __library.get(b"FPDF_RenderPage_Close\0").map(|sym| *sym);
        let FPDF_SaveAsCopy = __library.get(b"FPDF_SaveAsCopy\0").map(|sym| *sym);
        let FPDF_SaveWithVersion = __library.get(b"FPDF_SaveWithVersion\0").map(|sym| *sym);
        let FPDFText_GetCharIndexFromTextIndex = __library
            .get(b"FPDFText_GetCharIndexFromTextIndex\0")
            .map(|sym| *sym);
        let FPDFText_GetTextIndexFromCharIndex = __library
            .get(b"FPDFText_GetTextIndexFromCharIndex\0")
            .map(|sym| *sym);
        let FPDF_GetSignatureCount = __library.get(b"FPDF_GetSignatureCount\0").map(|sym| *sym);
        let FPDF_GetSignatureObject = __library.get(b"FPDF_GetSignatureObject\0").map(|sym| *sym);
        let FPDFSignatureObj_GetContents = __library
            .get(b"FPDFSignatureObj_GetContents\0")
            .map(|sym| *sym);
        let FPDFSignatureObj_GetByteRange = __library
            .get(b"FPDFSignatureObj_GetByteRange\0")
            .map(|sym| *sym);
        let FPDFSignatureObj_GetSubFilter = __library
            .get(b"FPDFSignatureObj_GetSubFilter\0")
            .map(|sym| *sym);
        let FPDFSignatureObj_GetReason = __library
            .get(b"FPDFSignatureObj_GetReason\0")
            .map(|sym| *sym);
        let FPDFSignatureObj_GetTime = __library.get(b"FPDFSignatureObj_GetTime\0").map(|sym| *sym);
        let FPDFSignatureObj_GetDocMDPPermission = __library
            .get(b"FPDFSignatureObj_GetDocMDPPermission\0")
            .map(|sym| *sym);
        let FPDF_StructTree_GetForPage = __library
            .get(b"FPDF_StructTree_GetForPage\0")
            .map(|sym| *sym);
        let FPDF_StructTree_Close = __library.get(b"FPDF_StructTree_Close\0").map(|sym| *sym);
        let FPDF_StructTree_CountChildren = __library
            .get(b"FPDF_StructTree_CountChildren\0")
            .map(|sym| *sym);
        let FPDF_StructTree_GetChildAtIndex = __library
            .get(b"FPDF_StructTree_GetChildAtIndex\0")
            .map(|sym| *sym);
        let FPDF_StructElement_GetAltText = __library
            .get(b"FPDF_StructElement_GetAltText\0")
            .map(|sym| *sym);
        let FPDF_StructElement_GetActualText = __library
            .get(b"FPDF_StructElement_GetActualText\0")
            .map(|sym| *sym);
        let FPDF_StructElement_GetExpansion = __library
            .get(b"FPDF_StructElement_GetExpansion\0")
            .map(|sym| *sym);
        let FPDF_StructElement_GetID = __library.get(b"FPDF_StructElement_GetID\0").map(|sym| *sym);
        let FPDF_StructElement_GetLang = __library
            .get(b"FPDF_StructElement_GetLang\0")
            .map(|sym| *sym);
        let FPDF_StructElement_GetStringAttribute = __library
            .get(b"FPDF_StructElement_GetStringAttribute\0")
            .map(|sym| *sym);
        let FPDF_StructElement_GetMarkedContentID = __library
            .get(b"FPDF_StructElement_GetMarkedContentID\0")
            .map(|sym| *sym);
        let FPDF_StructElement_GetRawType = __library
            .get(b"FPDF_StructElement_GetRawType\0")
            .map(|sym| *sym);
        let FPDF_StructElement_GetType = __library
            .get(b"FPDF_StructElement_GetType\0")
            .map(|sym| *sym);
        let FPDF_StructElement_GetObjType = __library
            .get(b"FPDF_StructElement_GetObjType\0")
            .map(|sym| *sym);
        let FPDF_StructElement_GetTitle = __library
            .get(b"FPDF_StructElement_GetTitle\0")
            .map(|sym| *sym);
        let FPDF_StructElement_CountChildren = __library
            .get(b"FPDF_StructElement_CountChildren\0")
            .map(|sym| *sym);
        let FPDF_StructElement_GetChildAtIndex = __library
            .get(b"FPDF_StructElement_GetChildAtIndex\0")
            .map(|sym| *sym);
        let FPDF_StructElement_GetChildMarkedContentID = __library
            .get(b"FPDF_StructElement_GetChildMarkedContentID\0")
            .map(|sym| *sym);
        let FPDF_StructElement_GetParent = __library
            .get(b"FPDF_StructElement_GetParent\0")
            .map(|sym| *sym);
        let FPDF_StructElement_GetAttributeCount = __library
            .get(b"FPDF_StructElement_GetAttributeCount\0")
            .map(|sym| *sym);
        let FPDF_StructElement_GetAttributeAtIndex = __library
            .get(b"FPDF_StructElement_GetAttributeAtIndex\0")
            .map(|sym| *sym);
        let FPDF_StructElement_Attr_GetCount = __library
            .get(b"FPDF_StructElement_Attr_GetCount\0")
            .map(|sym| *sym);
        let FPDF_StructElement_Attr_GetName = __library
            .get(b"FPDF_StructElement_Attr_GetName\0")
            .map(|sym| *sym);
        let FPDF_StructElement_Attr_GetValue = __library
            .get(b"FPDF_StructElement_Attr_GetValue\0")
            .map(|sym| *sym);
        let FPDF_StructElement_Attr_GetType = __library
            .get(b"FPDF_StructElement_Attr_GetType\0")
            .map(|sym| *sym);
        let FPDF_StructElement_Attr_GetBooleanValue = __library
            .get(b"FPDF_StructElement_Attr_GetBooleanValue\0")
            .map(|sym| *sym);
        let FPDF_StructElement_Attr_GetNumberValue = __library
            .get(b"FPDF_StructElement_Attr_GetNumberValue\0")
            .map(|sym| *sym);
        let FPDF_StructElement_Attr_GetStringValue = __library
            .get(b"FPDF_StructElement_Attr_GetStringValue\0")
            .map(|sym| *sym);
        let FPDF_StructElement_Attr_GetBlobValue = __library
            .get(b"FPDF_StructElement_Attr_GetBlobValue\0")
            .map(|sym| *sym);
        let FPDF_StructElement_Attr_CountChildren = __library
            .get(b"FPDF_StructElement_Attr_CountChildren\0")
            .map(|sym| *sym);
        let FPDF_StructElement_Attr_GetChildAtIndex = __library
            .get(b"FPDF_StructElement_Attr_GetChildAtIndex\0")
            .map(|sym| *sym);
        let FPDF_StructElement_GetMarkedContentIdCount = __library
            .get(b"FPDF_StructElement_GetMarkedContentIdCount\0")
            .map(|sym| *sym);
        let FPDF_StructElement_GetMarkedContentIdAtIndex = __library
            .get(b"FPDF_StructElement_GetMarkedContentIdAtIndex\0")
            .map(|sym| *sym);
        let FPDF_GetDefaultTTFMap = __library.get(b"FPDF_GetDefaultTTFMap\0").map(|sym| *sym);
        let FPDF_GetDefaultTTFMapCount = __library
            .get(b"FPDF_GetDefaultTTFMapCount\0")
            .map(|sym| *sym);
        let FPDF_GetDefaultTTFMapEntry = __library
            .get(b"FPDF_GetDefaultTTFMapEntry\0")
            .map(|sym| *sym);
        let FPDF_AddInstalledFont = __library.get(b"FPDF_AddInstalledFont\0").map(|sym| *sym);
        let FPDF_SetSystemFontInfo = __library.get(b"FPDF_SetSystemFontInfo\0").map(|sym| *sym);
        let FPDF_GetDefaultSystemFontInfo = __library
            .get(b"FPDF_GetDefaultSystemFontInfo\0")
            .map(|sym| *sym);
        let FPDF_FreeDefaultSystemFontInfo = __library
            .get(b"FPDF_FreeDefaultSystemFontInfo\0")
            .map(|sym| *sym);
        let FPDFText_LoadPage = __library.get(b"FPDFText_LoadPage\0").map(|sym| *sym);
        let FPDFText_ClosePage = __library.get(b"FPDFText_ClosePage\0").map(|sym| *sym);
        let FPDFText_CountChars = __library.get(b"FPDFText_CountChars\0").map(|sym| *sym);
        let FPDFText_GetUnicode = __library.get(b"FPDFText_GetUnicode\0").map(|sym| *sym);
        let FPDFText_GetTextObject = __library.get(b"FPDFText_GetTextObject\0").map(|sym| *sym);
        let FPDFText_IsGenerated = __library.get(b"FPDFText_IsGenerated\0").map(|sym| *sym);
        let FPDFText_IsHyphen = __library.get(b"FPDFText_IsHyphen\0").map(|sym| *sym);
        let FPDFText_HasUnicodeMapError = __library
            .get(b"FPDFText_HasUnicodeMapError\0")
            .map(|sym| *sym);
        let FPDFText_GetFontSize = __library.get(b"FPDFText_GetFontSize\0").map(|sym| *sym);
        let FPDFText_GetFontInfo = __library.get(b"FPDFText_GetFontInfo\0").map(|sym| *sym);
        let FPDFText_GetFontWeight = __library.get(b"FPDFText_GetFontWeight\0").map(|sym| *sym);
        let FPDFText_GetFillColor = __library.get(b"FPDFText_GetFillColor\0").map(|sym| *sym);
        let FPDFText_GetStrokeColor = __library.get(b"FPDFText_GetStrokeColor\0").map(|sym| *sym);
        let FPDFText_GetCharAngle = __library.get(b"FPDFText_GetCharAngle\0").map(|sym| *sym);
        let FPDFText_GetCharBox = __library.get(b"FPDFText_GetCharBox\0").map(|sym| *sym);
        let FPDFText_GetLooseCharBox = __library.get(b"FPDFText_GetLooseCharBox\0").map(|sym| *sym);
        let FPDFText_GetMatrix = __library.get(b"FPDFText_GetMatrix\0").map(|sym| *sym);
        let FPDFText_GetCharOrigin = __library.get(b"FPDFText_GetCharOrigin\0").map(|sym| *sym);
        let FPDFText_GetCharIndexAtPos = __library
            .get(b"FPDFText_GetCharIndexAtPos\0")
            .map(|sym| *sym);
        let FPDFText_GetText = __library.get(b"FPDFText_GetText\0").map(|sym| *sym);
        let FPDFText_CountRects = __library.get(b"FPDFText_CountRects\0").map(|sym| *sym);
        let FPDFText_GetRect = __library.get(b"FPDFText_GetRect\0").map(|sym| *sym);
        let FPDFText_GetBoundedText = __library.get(b"FPDFText_GetBoundedText\0").map(|sym| *sym);
        let FPDFText_FindStart = __library.get(b"FPDFText_FindStart\0").map(|sym| *sym);
        let FPDFText_FindNext = __library.get(b"FPDFText_FindNext\0").map(|sym| *sym);
        let FPDFText_FindPrev = __library.get(b"FPDFText_FindPrev\0").map(|sym| *sym);
        let FPDFText_GetSchResultIndex = __library
            .get(b"FPDFText_GetSchResultIndex\0")
            .map(|sym| *sym);
        let FPDFText_GetSchCount = __library.get(b"FPDFText_GetSchCount\0").map(|sym| *sym);
        let FPDFText_FindClose = __library.get(b"FPDFText_FindClose\0").map(|sym| *sym);
        let FPDFLink_LoadWebLinks = __library.get(b"FPDFLink_LoadWebLinks\0").map(|sym| *sym);
        let FPDFLink_CountWebLinks = __library.get(b"FPDFLink_CountWebLinks\0").map(|sym| *sym);
        let FPDFLink_GetURL = __library.get(b"FPDFLink_GetURL\0").map(|sym| *sym);
        let FPDFLink_CountRects = __library.get(b"FPDFLink_CountRects\0").map(|sym| *sym);
        let FPDFLink_GetRect = __library.get(b"FPDFLink_GetRect\0").map(|sym| *sym);
        let FPDFLink_GetTextRange = __library.get(b"FPDFLink_GetTextRange\0").map(|sym| *sym);
        let FPDFLink_CloseWebLinks = __library.get(b"FPDFLink_CloseWebLinks\0").map(|sym| *sym);
        let FPDFPage_GetDecodedThumbnailData = __library
            .get(b"FPDFPage_GetDecodedThumbnailData\0")
            .map(|sym| *sym);
        let FPDFPage_GetRawThumbnailData = __library
            .get(b"FPDFPage_GetRawThumbnailData\0")
            .map(|sym| *sym);
        let FPDFPage_GetThumbnailAsBitmap = __library
            .get(b"FPDFPage_GetThumbnailAsBitmap\0")
            .map(|sym| *sym);
        let FPDFPage_SetMediaBox = __library.get(b"FPDFPage_SetMediaBox\0").map(|sym| *sym);
        let FPDFPage_SetCropBox = __library.get(b"FPDFPage_SetCropBox\0").map(|sym| *sym);
        let FPDFPage_SetBleedBox = __library.get(b"FPDFPage_SetBleedBox\0").map(|sym| *sym);
        let FPDFPage_SetTrimBox = __library.get(b"FPDFPage_SetTrimBox\0").map(|sym| *sym);
        let FPDFPage_SetArtBox = __library.get(b"FPDFPage_SetArtBox\0").map(|sym| *sym);
        let FPDFPage_GetMediaBox = __library.get(b"FPDFPage_GetMediaBox\0").map(|sym| *sym);
        let FPDFPage_GetCropBox = __library.get(b"FPDFPage_GetCropBox\0").map(|sym| *sym);
        let FPDFPage_GetBleedBox = __library.get(b"FPDFPage_GetBleedBox\0").map(|sym| *sym);
        let FPDFPage_GetTrimBox = __library.get(b"FPDFPage_GetTrimBox\0").map(|sym| *sym);
        let FPDFPage_GetArtBox = __library.get(b"FPDFPage_GetArtBox\0").map(|sym| *sym);
        let FPDFPage_TransFormWithClip = __library
            .get(b"FPDFPage_TransFormWithClip\0")
            .map(|sym| *sym);
        let FPDFPageObj_TransformClipPath = __library
            .get(b"FPDFPageObj_TransformClipPath\0")
            .map(|sym| *sym);
        let FPDFPageObj_GetClipPath = __library.get(b"FPDFPageObj_GetClipPath\0").map(|sym| *sym);
        let FPDFClipPath_CountPaths = __library.get(b"FPDFClipPath_CountPaths\0").map(|sym| *sym);
        let FPDFClipPath_CountPathSegments = __library
            .get(b"FPDFClipPath_CountPathSegments\0")
            .map(|sym| *sym);
        let FPDFClipPath_GetPathSegment = __library
            .get(b"FPDFClipPath_GetPathSegment\0")
            .map(|sym| *sym);
        let FPDF_CreateClipPath = __library.get(b"FPDF_CreateClipPath\0").map(|sym| *sym);
        let FPDF_DestroyClipPath = __library.get(b"FPDF_DestroyClipPath\0").map(|sym| *sym);
        let FPDFPage_InsertClipPath = __library.get(b"FPDFPage_InsertClipPath\0").map(|sym| *sym);
        Ok(pdfium {
            __library,
            FPDF_InitLibraryWithConfig,
            FPDF_InitLibrary,
            FPDF_DestroyLibrary,
            FPDF_SetSandBoxPolicy,
            FPDF_LoadDocument,
            FPDF_LoadMemDocument,
            FPDF_LoadMemDocument64,
            FPDF_LoadCustomDocument,
            FPDF_GetFileVersion,
            FPDF_GetLastError,
            FPDF_DocumentHasValidCrossReferenceTable,
            FPDF_GetTrailerEnds,
            FPDF_GetDocPermissions,
            FPDF_GetDocUserPermissions,
            FPDF_GetSecurityHandlerRevision,
            FPDF_GetPageCount,
            FPDF_LoadPage,
            FPDF_GetPageWidthF,
            FPDF_GetPageWidth,
            FPDF_GetPageHeightF,
            FPDF_GetPageHeight,
            FPDF_GetPageBoundingBox,
            FPDF_GetPageSizeByIndexF,
            FPDF_GetPageSizeByIndex,
            FPDF_RenderPageBitmap,
            FPDF_RenderPageBitmapWithMatrix,
            FPDF_ClosePage,
            FPDF_CloseDocument,
            FPDF_DeviceToPage,
            FPDF_PageToDevice,
            FPDFBitmap_Create,
            FPDFBitmap_CreateEx,
            FPDFBitmap_GetFormat,
            FPDFBitmap_FillRect,
            FPDFBitmap_GetBuffer,
            FPDFBitmap_GetWidth,
            FPDFBitmap_GetHeight,
            FPDFBitmap_GetStride,
            FPDFBitmap_Destroy,
            FPDF_VIEWERREF_GetPrintScaling,
            FPDF_VIEWERREF_GetNumCopies,
            FPDF_VIEWERREF_GetPrintPageRange,
            FPDF_VIEWERREF_GetPrintPageRangeCount,
            FPDF_VIEWERREF_GetPrintPageRangeElement,
            FPDF_VIEWERREF_GetDuplex,
            FPDF_VIEWERREF_GetName,
            FPDF_CountNamedDests,
            FPDF_GetNamedDestByName,
            FPDF_GetNamedDest,
            FPDF_GetXFAPacketCount,
            FPDF_GetXFAPacketName,
            FPDF_GetXFAPacketContent,
            FPDFDOC_InitFormFillEnvironment,
            FPDFDOC_ExitFormFillEnvironment,
            FORM_OnAfterLoadPage,
            FORM_OnBeforeClosePage,
            FORM_DoDocumentJSAction,
            FORM_DoDocumentOpenAction,
            FORM_DoDocumentAAction,
            FORM_DoPageAAction,
            FORM_OnMouseMove,
            FORM_OnMouseWheel,
            FORM_OnFocus,
            FORM_OnLButtonDown,
            FORM_OnRButtonDown,
            FORM_OnLButtonUp,
            FORM_OnRButtonUp,
            FORM_OnLButtonDoubleClick,
            FORM_OnKeyDown,
            FORM_OnKeyUp,
            FORM_OnChar,
            FORM_GetFocusedText,
            FORM_GetSelectedText,
            FORM_ReplaceAndKeepSelection,
            FORM_ReplaceSelection,
            FORM_SelectAllText,
            FORM_CanUndo,
            FORM_CanRedo,
            FORM_Undo,
            FORM_Redo,
            FORM_ForceToKillFocus,
            FORM_GetFocusedAnnot,
            FORM_SetFocusedAnnot,
            FPDFPage_HasFormFieldAtPoint,
            FPDFPage_FormFieldZOrderAtPoint,
            FPDF_SetFormFieldHighlightColor,
            FPDF_SetFormFieldHighlightAlpha,
            FPDF_RemoveFormFieldHighlight,
            FPDF_FFLDraw,
            FPDF_GetFormType,
            FORM_SetIndexSelected,
            FORM_IsIndexSelected,
            FORM_GetTextDirection,
            FORM_SetTextDirection,
            FPDF_LoadXFA,
            FPDFAnnot_IsSupportedSubtype,
            FPDFPage_CreateAnnot,
            FPDFPage_GetAnnotCount,
            FPDFPage_GetAnnot,
            FPDFPage_GetAnnotIndex,
            FPDFPage_CloseAnnot,
            FPDFPage_RemoveAnnot,
            FPDFAnnot_GetSubtype,
            FPDFAnnot_IsObjectSupportedSubtype,
            FPDFAnnot_UpdateObject,
            FPDFAnnot_AddInkStroke,
            FPDFAnnot_RemoveInkList,
            FPDFAnnot_AppendObject,
            FPDFAnnot_GetObjectCount,
            FPDFAnnot_GetObject,
            FPDFAnnot_RemoveObject,
            FPDFAnnot_SetColor,
            FPDFAnnot_GetColor,
            FPDFAnnot_HasAttachmentPoints,
            FPDFAnnot_SetAttachmentPoints,
            FPDFAnnot_AppendAttachmentPoints,
            FPDFAnnot_CountAttachmentPoints,
            FPDFAnnot_GetAttachmentPoints,
            FPDFAnnot_SetRect,
            FPDFAnnot_GetRect,
            FPDFAnnot_GetVertices,
            FPDFAnnot_GetInkListCount,
            FPDFAnnot_GetInkListPath,
            FPDFAnnot_GetLine,
            FPDFAnnot_SetBorder,
            FPDFAnnot_GetBorder,
            FPDFAnnot_GetFormAdditionalActionJavaScript,
            FPDFAnnot_HasKey,
            FPDFAnnot_GetValueType,
            FPDFAnnot_SetStringValue,
            FPDFAnnot_GetStringValue,
            FPDFAnnot_GetNumberValue,
            FPDFAnnot_SetAP,
            FPDFAnnot_GetAP,
            FPDFAnnot_GetLinkedAnnot,
            FPDFAnnot_GetFlags,
            FPDFAnnot_SetFlags,
            FPDFAnnot_GetFormFieldFlags,
            FPDFAnnot_SetFormFieldFlags,
            FPDFAnnot_GetFormFieldAtPoint,
            FPDFAnnot_GetFormFieldName,
            FPDFAnnot_GetFormFieldAlternateName,
            FPDFAnnot_GetFormFieldType,
            FPDFAnnot_GetFormFieldValue,
            FPDFAnnot_GetOptionCount,
            FPDFAnnot_GetOptionLabel,
            FPDFAnnot_IsOptionSelected,
            FPDFAnnot_GetFontSize,
            FPDFAnnot_SetFontColor,
            FPDFAnnot_GetFontColor,
            FPDFAnnot_IsChecked,
            FPDFAnnot_SetFocusableSubtypes,
            FPDFAnnot_GetFocusableSubtypesCount,
            FPDFAnnot_GetFocusableSubtypes,
            FPDFAnnot_GetLink,
            FPDFAnnot_GetFormControlCount,
            FPDFAnnot_GetFormControlIndex,
            FPDFAnnot_GetFormFieldExportValue,
            FPDFAnnot_SetURI,
            FPDFAnnot_GetFileAttachment,
            FPDFAnnot_AddFileAttachment,
            FPDFDoc_GetAttachmentCount,
            FPDFDoc_AddAttachment,
            FPDFDoc_GetAttachment,
            FPDFDoc_DeleteAttachment,
            FPDFAttachment_GetName,
            FPDFAttachment_HasKey,
            FPDFAttachment_GetValueType,
            FPDFAttachment_SetStringValue,
            FPDFAttachment_GetStringValue,
            FPDFAttachment_SetDescription,
            FPDFAttachment_GetDescription,
            FPDFAttachment_SetFile,
            FPDFAttachment_GetFile,
            FPDFAttachment_GetSubtype,
            FPDFAttachment_GetAFRelationship,
            FPDFCatalog_IsTagged,
            FPDFCatalog_GetLanguage,
            FPDFCatalog_SetLanguage,
            FPDFAvail_Create,
            FPDFAvail_Destroy,
            FPDFAvail_IsDocAvail,
            FPDFAvail_GetDocument,
            FPDFAvail_GetFirstPageNum,
            FPDFAvail_IsPageAvail,
            FPDFAvail_IsFormAvail,
            FPDFAvail_IsLinearized,
            FPDFBookmark_GetFirstChild,
            FPDFBookmark_GetNextSibling,
            FPDFBookmark_GetTitle,
            FPDFBookmark_GetCount,
            FPDFBookmark_Find,
            FPDFBookmark_GetDest,
            FPDFBookmark_GetAction,
            FPDFBookmark_GetColor,
            FPDFBookmark_GetStyle,
            FPDFAction_GetType,
            FPDFAction_GetDest,
            FPDFAction_GetFilePath,
            FPDFAction_GetURIPath,
            FPDFDest_GetDestPageIndex,
            FPDFDest_GetView,
            FPDFDest_GetLocationInPage,
            FPDFLink_GetLinkAtPoint,
            FPDFLink_GetLinkZOrderAtPoint,
            FPDFLink_GetDest,
            FPDFLink_GetAction,
            FPDFLink_Enumerate,
            FPDFLink_GetAnnot,
            FPDFLink_GetAnnotRect,
            FPDFLink_CountQuadPoints,
            FPDFLink_GetQuadPoints,
            FPDF_GetPageAAction,
            FPDF_GetFileIdentifier,
            FPDF_GetMetaText,
            FPDF_GetPageLabel,
            FPDF_CreateNewDocument,
            FPDFPage_New,
            FPDFPage_Delete,
            FPDF_MovePages,
            FPDFPage_GetRotation,
            FPDFPage_SetRotation,
            FPDFPage_InsertObject,
            FPDFPage_InsertObjectAtIndex,
            FPDFPage_RemoveObject,
            FPDFPage_CountObjects,
            FPDFPage_GetObject,
            FPDFPage_HasTransparency,
            FPDFPage_GenerateContent,
            FPDFPageObj_Destroy,
            FPDFPageObj_HasTransparency,
            FPDFPageObj_GetType,
            FPDFPageObj_GetIsActive,
            FPDFPageObj_SetIsActive,
            FPDFPageObj_Transform,
            FPDFPageObj_TransformF,
            FPDFPageObj_GetMatrix,
            FPDFPageObj_SetMatrix,
            FPDFPage_TransformAnnots,
            FPDFPageObj_NewImageObj,
            FPDFPageObj_GetMarkedContentID,
            FPDFPageObj_CountMarks,
            FPDFPageObj_GetMark,
            FPDFPageObj_AddMark,
            FPDFPageObj_AddExistingMark,
            FPDFPageObj_RemoveMark,
            FPDFPageObjMark_GetName,
            FPDFPageObjMark_CountParams,
            FPDFPageObjMark_GetParamKey,
            FPDFPageObjMark_GetParamValueType,
            FPDFPageObjMark_GetParamIntValue,
            FPDFPageObjMark_GetParamFloatValue,
            FPDFPageObjMark_GetParamStringValue,
            FPDFPageObjMark_GetParamBlobValue,
            FPDFPageObjMark_SetIntParam,
            FPDFPageObjMark_SetFloatParam,
            FPDFPageObjMark_SetStringParam,
            FPDFPageObjMark_SetBlobParam,
            FPDFPageObjMark_RemoveParam,
            FPDFImageObj_LoadJpegFile,
            FPDFImageObj_LoadJpegFileInline,
            FPDFImageObj_SetMatrix,
            FPDFImageObj_SetBitmap,
            FPDFImageObj_GetBitmap,
            FPDFImageObj_GetRenderedBitmap,
            FPDFImageObj_GetImageDataDecoded,
            FPDFImageObj_GetImageDataRaw,
            FPDFImageObj_GetImageFilterCount,
            FPDFImageObj_GetImageFilter,
            FPDFImageObj_GetImageMetadata,
            FPDFImageObj_GetImagePixelSize,
            FPDFImageObj_GetIccProfileDataDecoded,
            FPDFPageObj_CreateNewPath,
            FPDFPageObj_CreateNewRect,
            FPDFPageObj_GetBounds,
            FPDFPageObj_GetRotatedBounds,
            FPDFPageObj_SetBlendMode,
            FPDFPageObj_SetStrokeColor,
            FPDFPageObj_GetStrokeColor,
            FPDFPageObj_SetStrokeWidth,
            FPDFPageObj_GetStrokeWidth,
            FPDFPageObj_GetLineJoin,
            FPDFPageObj_SetLineJoin,
            FPDFPageObj_GetLineCap,
            FPDFPageObj_SetLineCap,
            FPDFPageObj_SetFillColor,
            FPDFPageObj_GetFillColor,
            FPDFPageObj_GetDashPhase,
            FPDFPageObj_SetDashPhase,
            FPDFPageObj_GetDashCount,
            FPDFPageObj_GetDashArray,
            FPDFPageObj_SetDashArray,
            FPDFPageObj_GetRenderedStrokePattern,
            FPDFPageObj_GetRenderedFillPattern,
            FPDFPath_CountSegments,
            FPDFPath_GetPathSegment,
            FPDFPathSegment_GetPoint,
            FPDFPathSegment_GetType,
            FPDFPathSegment_GetClose,
            FPDFPath_MoveTo,
            FPDFPath_LineTo,
            FPDFPath_BezierTo,
            FPDFPath_GetBezierControlPoints,
            FPDFPath_Close,
            FPDFPath_SetDrawMode,
            FPDFPath_GetDrawMode,
            FPDFPageObj_NewTextObj,
            FPDFText_SetText,
            FPDFText_SetCharcodes,
            FPDFText_SetPositions,
            FPDFText_LoadFont,
            FPDFText_LoadStandardFont,
            FPDFText_LoadCidType2Font,
            FPDFTextObj_GetFontSize,
            FPDFTextObj_SetFontSize,
            FPDFFont_Close,
            FPDFPageObj_CreateTextObj,
            FPDFTextObj_GetTextRenderMode,
            FPDFTextObj_SetTextRenderMode,
            FPDFTextObj_GetText,
            FPDFTextObj_GetRenderedBitmap,
            FPDFTextObj_GetFont,
            FPDFFont_GetBaseFontName,
            FPDFFont_GetFamilyName,
            FPDFFont_GetFontData,
            FPDFFont_GetIsEmbedded,
            FPDFFont_GetFlags,
            FPDFFont_GetWeight,
            FPDFFont_GetItalicAngle,
            FPDFFont_GetAscent,
            FPDFFont_GetDescent,
            FPDFFont_GetGlyphWidth,
            FPDFFont_GetGlyphPath,
            FPDFGlyphPath_CountGlyphSegments,
            FPDFGlyphPath_GetGlyphPathSegment,
            FPDFFormObj_CountObjects,
            FPDFFormObj_GetObject,
            FPDFFormObj_RemoveObject,
            FSDK_SetUnSpObjProcessHandler,
            FSDK_SetTimeFunction,
            FSDK_SetLocaltimeFunction,
            FPDFDoc_GetPageMode,
            FPDFPage_Flatten,
            FPDFDoc_GetJavaScriptActionCount,
            FPDFDoc_GetJavaScriptAction,
            FPDFDoc_CloseJavaScriptAction,
            FPDFJavaScriptAction_GetName,
            FPDFJavaScriptAction_GetScript,
            FPDF_ImportPagesByIndex,
            FPDF_ImportPages,
            FPDF_ImportNPagesToOne,
            FPDF_NewXObjectFromPage,
            FPDF_CloseXObject,
            FPDF_NewFormObjectFromXObject,
            FPDF_CopyViewerPreferences,
            FPDF_RenderPageBitmapWithColorScheme_Start,
            FPDF_RenderPageBitmap_Start,
            FPDF_RenderPage_Continue,
            FPDF_RenderPage_Close,
            FPDF_SaveAsCopy,
            FPDF_SaveWithVersion,
            FPDFText_GetCharIndexFromTextIndex,
            FPDFText_GetTextIndexFromCharIndex,
            FPDF_GetSignatureCount,
            FPDF_GetSignatureObject,
            FPDFSignatureObj_GetContents,
            FPDFSignatureObj_GetByteRange,
            FPDFSignatureObj_GetSubFilter,
            FPDFSignatureObj_GetReason,
            FPDFSignatureObj_GetTime,
            FPDFSignatureObj_GetDocMDPPermission,
            FPDF_StructTree_GetForPage,
            FPDF_StructTree_Close,
            FPDF_StructTree_CountChildren,
            FPDF_StructTree_GetChildAtIndex,
            FPDF_StructElement_GetAltText,
            FPDF_StructElement_GetActualText,
            FPDF_StructElement_GetExpansion,
            FPDF_StructElement_GetID,
            FPDF_StructElement_GetLang,
            FPDF_StructElement_GetStringAttribute,
            FPDF_StructElement_GetMarkedContentID,
            FPDF_StructElement_GetRawType,
            FPDF_StructElement_GetType,
            FPDF_StructElement_GetObjType,
            FPDF_StructElement_GetTitle,
            FPDF_StructElement_CountChildren,
            FPDF_StructElement_GetChildAtIndex,
            FPDF_StructElement_GetChildMarkedContentID,
            FPDF_StructElement_GetParent,
            FPDF_StructElement_GetAttributeCount,
            FPDF_StructElement_GetAttributeAtIndex,
            FPDF_StructElement_Attr_GetCount,
            FPDF_StructElement_Attr_GetName,
            FPDF_StructElement_Attr_GetValue,
            FPDF_StructElement_Attr_GetType,
            FPDF_StructElement_Attr_GetBooleanValue,
            FPDF_StructElement_Attr_GetNumberValue,
            FPDF_StructElement_Attr_GetStringValue,
            FPDF_StructElement_Attr_GetBlobValue,
            FPDF_StructElement_Attr_CountChildren,
            FPDF_StructElement_Attr_GetChildAtIndex,
            FPDF_StructElement_GetMarkedContentIdCount,
            FPDF_StructElement_GetMarkedContentIdAtIndex,
            FPDF_GetDefaultTTFMap,
            FPDF_GetDefaultTTFMapCount,
            FPDF_GetDefaultTTFMapEntry,
            FPDF_AddInstalledFont,
            FPDF_SetSystemFontInfo,
            FPDF_GetDefaultSystemFontInfo,
            FPDF_FreeDefaultSystemFontInfo,
            FPDFText_LoadPage,
            FPDFText_ClosePage,
            FPDFText_CountChars,
            FPDFText_GetUnicode,
            FPDFText_GetTextObject,
            FPDFText_IsGenerated,
            FPDFText_IsHyphen,
            FPDFText_HasUnicodeMapError,
            FPDFText_GetFontSize,
            FPDFText_GetFontInfo,
            FPDFText_GetFontWeight,
            FPDFText_GetFillColor,
            FPDFText_GetStrokeColor,
            FPDFText_GetCharAngle,
            FPDFText_GetCharBox,
            FPDFText_GetLooseCharBox,
            FPDFText_GetMatrix,
            FPDFText_GetCharOrigin,
            FPDFText_GetCharIndexAtPos,
            FPDFText_GetText,
            FPDFText_CountRects,
            FPDFText_GetRect,
            FPDFText_GetBoundedText,
            FPDFText_FindStart,
            FPDFText_FindNext,
            FPDFText_FindPrev,
            FPDFText_GetSchResultIndex,
            FPDFText_GetSchCount,
            FPDFText_FindClose,
            FPDFLink_LoadWebLinks,
            FPDFLink_CountWebLinks,
            FPDFLink_GetURL,
            FPDFLink_CountRects,
            FPDFLink_GetRect,
            FPDFLink_GetTextRange,
            FPDFLink_CloseWebLinks,
            FPDFPage_GetDecodedThumbnailData,
            FPDFPage_GetRawThumbnailData,
            FPDFPage_GetThumbnailAsBitmap,
            FPDFPage_SetMediaBox,
            FPDFPage_SetCropBox,
            FPDFPage_SetBleedBox,
            FPDFPage_SetTrimBox,
            FPDFPage_SetArtBox,
            FPDFPage_GetMediaBox,
            FPDFPage_GetCropBox,
            FPDFPage_GetBleedBox,
            FPDFPage_GetTrimBox,
            FPDFPage_GetArtBox,
            FPDFPage_TransFormWithClip,
            FPDFPageObj_TransformClipPath,
            FPDFPageObj_GetClipPath,
            FPDFClipPath_CountPaths,
            FPDFClipPath_CountPathSegments,
            FPDFClipPath_GetPathSegment,
            FPDF_CreateClipPath,
            FPDF_DestroyClipPath,
            FPDFPage_InsertClipPath,
        })
    }
    pub unsafe fn FPDF_InitLibraryWithConfig(&self, config: *const FPDF_LIBRARY_CONFIG) {
        (self
            .FPDF_InitLibraryWithConfig
            .as_ref()
            .expect("Expected function, got error."))(config)
    }
    pub unsafe fn FPDF_InitLibrary(&self) {
        (self
            .FPDF_InitLibrary
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn FPDF_DestroyLibrary(&self) {
        (self
            .FPDF_DestroyLibrary
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn FPDF_SetSandBoxPolicy(&self, policy: FPDF_DWORD, enable: FPDF_BOOL) {
        (self
            .FPDF_SetSandBoxPolicy
            .as_ref()
            .expect("Expected function, got error."))(policy, enable)
    }
    pub unsafe fn FPDF_LoadDocument(
        &self,
        file_path: FPDF_STRING,
        password: FPDF_BYTESTRING,
    ) -> FPDF_DOCUMENT {
        (self
            .FPDF_LoadDocument
            .as_ref()
            .expect("Expected function, got error."))(file_path, password)
    }
    pub unsafe fn FPDF_LoadMemDocument(
        &self,
        data_buf: *const ::core::ffi::c_void,
        size: ::core::ffi::c_int,
        password: FPDF_BYTESTRING,
    ) -> FPDF_DOCUMENT {
        (self
            .FPDF_LoadMemDocument
            .as_ref()
            .expect("Expected function, got error."))(data_buf, size, password)
    }
    pub unsafe fn FPDF_LoadMemDocument64(
        &self,
        data_buf: *const ::core::ffi::c_void,
        size: usize,
        password: FPDF_BYTESTRING,
    ) -> FPDF_DOCUMENT {
        (self
            .FPDF_LoadMemDocument64
            .as_ref()
            .expect("Expected function, got error."))(data_buf, size, password)
    }
    pub unsafe fn FPDF_LoadCustomDocument(
        &self,
        pFileAccess: *mut FPDF_FILEACCESS,
        password: FPDF_BYTESTRING,
    ) -> FPDF_DOCUMENT {
        (self
            .FPDF_LoadCustomDocument
            .as_ref()
            .expect("Expected function, got error."))(pFileAccess, password)
    }
    pub unsafe fn FPDF_GetFileVersion(
        &self,
        doc: FPDF_DOCUMENT,
        fileVersion: *mut ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FPDF_GetFileVersion
            .as_ref()
            .expect("Expected function, got error."))(doc, fileVersion)
    }
    pub unsafe fn FPDF_GetLastError(&self) -> ::core::ffi::c_ulong {
        (self
            .FPDF_GetLastError
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn FPDF_DocumentHasValidCrossReferenceTable(
        &self,
        document: FPDF_DOCUMENT,
    ) -> FPDF_BOOL {
        (self
            .FPDF_DocumentHasValidCrossReferenceTable
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDF_GetTrailerEnds(
        &self,
        document: FPDF_DOCUMENT,
        buffer: *mut ::core::ffi::c_uint,
        length: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDF_GetTrailerEnds
            .as_ref()
            .expect("Expected function, got error."))(document, buffer, length)
    }
    pub unsafe fn FPDF_GetDocPermissions(&self, document: FPDF_DOCUMENT) -> ::core::ffi::c_ulong {
        (self
            .FPDF_GetDocPermissions
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDF_GetDocUserPermissions(
        &self,
        document: FPDF_DOCUMENT,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDF_GetDocUserPermissions
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDF_GetSecurityHandlerRevision(
        &self,
        document: FPDF_DOCUMENT,
    ) -> ::core::ffi::c_int {
        (self
            .FPDF_GetSecurityHandlerRevision
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDF_GetPageCount(&self, document: FPDF_DOCUMENT) -> ::core::ffi::c_int {
        (self
            .FPDF_GetPageCount
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDF_LoadPage(
        &self,
        document: FPDF_DOCUMENT,
        page_index: ::core::ffi::c_int,
    ) -> FPDF_PAGE {
        (self
            .FPDF_LoadPage
            .as_ref()
            .expect("Expected function, got error."))(document, page_index)
    }
    pub unsafe fn FPDF_GetPageWidthF(&self, page: FPDF_PAGE) -> f32 {
        (self
            .FPDF_GetPageWidthF
            .as_ref()
            .expect("Expected function, got error."))(page)
    }
    pub unsafe fn FPDF_GetPageWidth(&self, page: FPDF_PAGE) -> f64 {
        (self
            .FPDF_GetPageWidth
            .as_ref()
            .expect("Expected function, got error."))(page)
    }
    pub unsafe fn FPDF_GetPageHeightF(&self, page: FPDF_PAGE) -> f32 {
        (self
            .FPDF_GetPageHeightF
            .as_ref()
            .expect("Expected function, got error."))(page)
    }
    pub unsafe fn FPDF_GetPageHeight(&self, page: FPDF_PAGE) -> f64 {
        (self
            .FPDF_GetPageHeight
            .as_ref()
            .expect("Expected function, got error."))(page)
    }
    pub unsafe fn FPDF_GetPageBoundingBox(
        &self,
        page: FPDF_PAGE,
        rect: *mut FS_RECTF,
    ) -> FPDF_BOOL {
        (self
            .FPDF_GetPageBoundingBox
            .as_ref()
            .expect("Expected function, got error."))(page, rect)
    }
    pub unsafe fn FPDF_GetPageSizeByIndexF(
        &self,
        document: FPDF_DOCUMENT,
        page_index: ::core::ffi::c_int,
        size: *mut FS_SIZEF,
    ) -> FPDF_BOOL {
        (self
            .FPDF_GetPageSizeByIndexF
            .as_ref()
            .expect("Expected function, got error."))(document, page_index, size)
    }
    pub unsafe fn FPDF_GetPageSizeByIndex(
        &self,
        document: FPDF_DOCUMENT,
        page_index: ::core::ffi::c_int,
        width: *mut f64,
        height: *mut f64,
    ) -> ::core::ffi::c_int {
        (self
            .FPDF_GetPageSizeByIndex
            .as_ref()
            .expect("Expected function, got error."))(document, page_index, width, height)
    }
    pub unsafe fn FPDF_RenderPageBitmap(
        &self,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: ::core::ffi::c_int,
        start_y: ::core::ffi::c_int,
        size_x: ::core::ffi::c_int,
        size_y: ::core::ffi::c_int,
        rotate: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) {
        (self
            .FPDF_RenderPageBitmap
            .as_ref()
            .expect("Expected function, got error."))(
            bitmap, page, start_x, start_y, size_x, size_y, rotate, flags,
        )
    }
    pub unsafe fn FPDF_RenderPageBitmapWithMatrix(
        &self,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        matrix: *const FS_MATRIX,
        clipping: *const FS_RECTF,
        flags: ::core::ffi::c_int,
    ) {
        (self
            .FPDF_RenderPageBitmapWithMatrix
            .as_ref()
            .expect("Expected function, got error."))(bitmap, page, matrix, clipping, flags)
    }
    pub unsafe fn FPDF_ClosePage(&self, page: FPDF_PAGE) {
        (self
            .FPDF_ClosePage
            .as_ref()
            .expect("Expected function, got error."))(page)
    }
    pub unsafe fn FPDF_CloseDocument(&self, document: FPDF_DOCUMENT) {
        (self
            .FPDF_CloseDocument
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDF_DeviceToPage(
        &self,
        page: FPDF_PAGE,
        start_x: ::core::ffi::c_int,
        start_y: ::core::ffi::c_int,
        size_x: ::core::ffi::c_int,
        size_y: ::core::ffi::c_int,
        rotate: ::core::ffi::c_int,
        device_x: ::core::ffi::c_int,
        device_y: ::core::ffi::c_int,
        page_x: *mut f64,
        page_y: *mut f64,
    ) -> FPDF_BOOL {
        (self
            .FPDF_DeviceToPage
            .as_ref()
            .expect("Expected function, got error."))(
            page, start_x, start_y, size_x, size_y, rotate, device_x, device_y, page_x, page_y,
        )
    }
    pub unsafe fn FPDF_PageToDevice(
        &self,
        page: FPDF_PAGE,
        start_x: ::core::ffi::c_int,
        start_y: ::core::ffi::c_int,
        size_x: ::core::ffi::c_int,
        size_y: ::core::ffi::c_int,
        rotate: ::core::ffi::c_int,
        page_x: f64,
        page_y: f64,
        device_x: *mut ::core::ffi::c_int,
        device_y: *mut ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FPDF_PageToDevice
            .as_ref()
            .expect("Expected function, got error."))(
            page, start_x, start_y, size_x, size_y, rotate, page_x, page_y, device_x, device_y,
        )
    }
    pub unsafe fn FPDFBitmap_Create(
        &self,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        alpha: ::core::ffi::c_int,
    ) -> FPDF_BITMAP {
        (self
            .FPDFBitmap_Create
            .as_ref()
            .expect("Expected function, got error."))(width, height, alpha)
    }
    pub unsafe fn FPDFBitmap_CreateEx(
        &self,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        format: ::core::ffi::c_int,
        first_scan: *mut ::core::ffi::c_void,
        stride: ::core::ffi::c_int,
    ) -> FPDF_BITMAP {
        (self
            .FPDFBitmap_CreateEx
            .as_ref()
            .expect("Expected function, got error."))(
            width, height, format, first_scan, stride
        )
    }
    pub unsafe fn FPDFBitmap_GetFormat(&self, bitmap: FPDF_BITMAP) -> ::core::ffi::c_int {
        (self
            .FPDFBitmap_GetFormat
            .as_ref()
            .expect("Expected function, got error."))(bitmap)
    }
    pub unsafe fn FPDFBitmap_FillRect(
        &self,
        bitmap: FPDF_BITMAP,
        left: ::core::ffi::c_int,
        top: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        color: FPDF_DWORD,
    ) -> FPDF_BOOL {
        (self
            .FPDFBitmap_FillRect
            .as_ref()
            .expect("Expected function, got error."))(
            bitmap, left, top, width, height, color
        )
    }
    pub unsafe fn FPDFBitmap_GetBuffer(&self, bitmap: FPDF_BITMAP) -> *mut ::core::ffi::c_void {
        (self
            .FPDFBitmap_GetBuffer
            .as_ref()
            .expect("Expected function, got error."))(bitmap)
    }
    pub unsafe fn FPDFBitmap_GetWidth(&self, bitmap: FPDF_BITMAP) -> ::core::ffi::c_int {
        (self
            .FPDFBitmap_GetWidth
            .as_ref()
            .expect("Expected function, got error."))(bitmap)
    }
    pub unsafe fn FPDFBitmap_GetHeight(&self, bitmap: FPDF_BITMAP) -> ::core::ffi::c_int {
        (self
            .FPDFBitmap_GetHeight
            .as_ref()
            .expect("Expected function, got error."))(bitmap)
    }
    pub unsafe fn FPDFBitmap_GetStride(&self, bitmap: FPDF_BITMAP) -> ::core::ffi::c_int {
        (self
            .FPDFBitmap_GetStride
            .as_ref()
            .expect("Expected function, got error."))(bitmap)
    }
    pub unsafe fn FPDFBitmap_Destroy(&self, bitmap: FPDF_BITMAP) {
        (self
            .FPDFBitmap_Destroy
            .as_ref()
            .expect("Expected function, got error."))(bitmap)
    }
    pub unsafe fn FPDF_VIEWERREF_GetPrintScaling(&self, document: FPDF_DOCUMENT) -> FPDF_BOOL {
        (self
            .FPDF_VIEWERREF_GetPrintScaling
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDF_VIEWERREF_GetNumCopies(
        &self,
        document: FPDF_DOCUMENT,
    ) -> ::core::ffi::c_int {
        (self
            .FPDF_VIEWERREF_GetNumCopies
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDF_VIEWERREF_GetPrintPageRange(
        &self,
        document: FPDF_DOCUMENT,
    ) -> FPDF_PAGERANGE {
        (self
            .FPDF_VIEWERREF_GetPrintPageRange
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDF_VIEWERREF_GetPrintPageRangeCount(&self, pagerange: FPDF_PAGERANGE) -> usize {
        (self
            .FPDF_VIEWERREF_GetPrintPageRangeCount
            .as_ref()
            .expect("Expected function, got error."))(pagerange)
    }
    pub unsafe fn FPDF_VIEWERREF_GetPrintPageRangeElement(
        &self,
        pagerange: FPDF_PAGERANGE,
        index: usize,
    ) -> ::core::ffi::c_int {
        (self
            .FPDF_VIEWERREF_GetPrintPageRangeElement
            .as_ref()
            .expect("Expected function, got error."))(pagerange, index)
    }
    pub unsafe fn FPDF_VIEWERREF_GetDuplex(&self, document: FPDF_DOCUMENT) -> FPDF_DUPLEXTYPE {
        (self
            .FPDF_VIEWERREF_GetDuplex
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDF_VIEWERREF_GetName(
        &self,
        document: FPDF_DOCUMENT,
        key: FPDF_BYTESTRING,
        buffer: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDF_VIEWERREF_GetName
            .as_ref()
            .expect("Expected function, got error."))(document, key, buffer, length)
    }
    pub unsafe fn FPDF_CountNamedDests(&self, document: FPDF_DOCUMENT) -> FPDF_DWORD {
        (self
            .FPDF_CountNamedDests
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDF_GetNamedDestByName(
        &self,
        document: FPDF_DOCUMENT,
        name: FPDF_BYTESTRING,
    ) -> FPDF_DEST {
        (self
            .FPDF_GetNamedDestByName
            .as_ref()
            .expect("Expected function, got error."))(document, name)
    }
    pub unsafe fn FPDF_GetNamedDest(
        &self,
        document: FPDF_DOCUMENT,
        index: ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
        buflen: *mut ::core::ffi::c_long,
    ) -> FPDF_DEST {
        (self
            .FPDF_GetNamedDest
            .as_ref()
            .expect("Expected function, got error."))(document, index, buffer, buflen)
    }
    pub unsafe fn FPDF_GetXFAPacketCount(&self, document: FPDF_DOCUMENT) -> ::core::ffi::c_int {
        (self
            .FPDF_GetXFAPacketCount
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDF_GetXFAPacketName(
        &self,
        document: FPDF_DOCUMENT,
        index: ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDF_GetXFAPacketName
            .as_ref()
            .expect("Expected function, got error."))(document, index, buffer, buflen)
    }
    pub unsafe fn FPDF_GetXFAPacketContent(
        &self,
        document: FPDF_DOCUMENT,
        index: ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
        out_buflen: *mut ::core::ffi::c_ulong,
    ) -> FPDF_BOOL {
        (self
            .FPDF_GetXFAPacketContent
            .as_ref()
            .expect("Expected function, got error."))(
            document, index, buffer, buflen, out_buflen
        )
    }
    pub unsafe fn FPDFDOC_InitFormFillEnvironment(
        &self,
        document: FPDF_DOCUMENT,
        formInfo: *mut FPDF_FORMFILLINFO,
    ) -> FPDF_FORMHANDLE {
        (self
            .FPDFDOC_InitFormFillEnvironment
            .as_ref()
            .expect("Expected function, got error."))(document, formInfo)
    }
    pub unsafe fn FPDFDOC_ExitFormFillEnvironment(&self, hHandle: FPDF_FORMHANDLE) {
        (self
            .FPDFDOC_ExitFormFillEnvironment
            .as_ref()
            .expect("Expected function, got error."))(hHandle)
    }
    pub unsafe fn FORM_OnAfterLoadPage(&self, page: FPDF_PAGE, hHandle: FPDF_FORMHANDLE) {
        (self
            .FORM_OnAfterLoadPage
            .as_ref()
            .expect("Expected function, got error."))(page, hHandle)
    }
    pub unsafe fn FORM_OnBeforeClosePage(&self, page: FPDF_PAGE, hHandle: FPDF_FORMHANDLE) {
        (self
            .FORM_OnBeforeClosePage
            .as_ref()
            .expect("Expected function, got error."))(page, hHandle)
    }
    pub unsafe fn FORM_DoDocumentJSAction(&self, hHandle: FPDF_FORMHANDLE) {
        (self
            .FORM_DoDocumentJSAction
            .as_ref()
            .expect("Expected function, got error."))(hHandle)
    }
    pub unsafe fn FORM_DoDocumentOpenAction(&self, hHandle: FPDF_FORMHANDLE) {
        (self
            .FORM_DoDocumentOpenAction
            .as_ref()
            .expect("Expected function, got error."))(hHandle)
    }
    pub unsafe fn FORM_DoDocumentAAction(
        &self,
        hHandle: FPDF_FORMHANDLE,
        aaType: ::core::ffi::c_int,
    ) {
        (self
            .FORM_DoDocumentAAction
            .as_ref()
            .expect("Expected function, got error."))(hHandle, aaType)
    }
    pub unsafe fn FORM_DoPageAAction(
        &self,
        page: FPDF_PAGE,
        hHandle: FPDF_FORMHANDLE,
        aaType: ::core::ffi::c_int,
    ) {
        (self
            .FORM_DoPageAAction
            .as_ref()
            .expect("Expected function, got error."))(page, hHandle, aaType)
    }
    pub unsafe fn FORM_OnMouseMove(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        modifier: ::core::ffi::c_int,
        page_x: f64,
        page_y: f64,
    ) -> FPDF_BOOL {
        (self
            .FORM_OnMouseMove
            .as_ref()
            .expect("Expected function, got error."))(
            hHandle, page, modifier, page_x, page_y
        )
    }
    pub unsafe fn FORM_OnMouseWheel(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        modifier: ::core::ffi::c_int,
        page_coord: *const FS_POINTF,
        delta_x: ::core::ffi::c_int,
        delta_y: ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FORM_OnMouseWheel
            .as_ref()
            .expect("Expected function, got error."))(
            hHandle, page, modifier, page_coord, delta_x, delta_y,
        )
    }
    pub unsafe fn FORM_OnFocus(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        modifier: ::core::ffi::c_int,
        page_x: f64,
        page_y: f64,
    ) -> FPDF_BOOL {
        (self
            .FORM_OnFocus
            .as_ref()
            .expect("Expected function, got error."))(
            hHandle, page, modifier, page_x, page_y
        )
    }
    pub unsafe fn FORM_OnLButtonDown(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        modifier: ::core::ffi::c_int,
        page_x: f64,
        page_y: f64,
    ) -> FPDF_BOOL {
        (self
            .FORM_OnLButtonDown
            .as_ref()
            .expect("Expected function, got error."))(
            hHandle, page, modifier, page_x, page_y
        )
    }
    pub unsafe fn FORM_OnRButtonDown(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        modifier: ::core::ffi::c_int,
        page_x: f64,
        page_y: f64,
    ) -> FPDF_BOOL {
        (self
            .FORM_OnRButtonDown
            .as_ref()
            .expect("Expected function, got error."))(
            hHandle, page, modifier, page_x, page_y
        )
    }
    pub unsafe fn FORM_OnLButtonUp(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        modifier: ::core::ffi::c_int,
        page_x: f64,
        page_y: f64,
    ) -> FPDF_BOOL {
        (self
            .FORM_OnLButtonUp
            .as_ref()
            .expect("Expected function, got error."))(
            hHandle, page, modifier, page_x, page_y
        )
    }
    pub unsafe fn FORM_OnRButtonUp(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        modifier: ::core::ffi::c_int,
        page_x: f64,
        page_y: f64,
    ) -> FPDF_BOOL {
        (self
            .FORM_OnRButtonUp
            .as_ref()
            .expect("Expected function, got error."))(
            hHandle, page, modifier, page_x, page_y
        )
    }
    pub unsafe fn FORM_OnLButtonDoubleClick(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        modifier: ::core::ffi::c_int,
        page_x: f64,
        page_y: f64,
    ) -> FPDF_BOOL {
        (self
            .FORM_OnLButtonDoubleClick
            .as_ref()
            .expect("Expected function, got error."))(
            hHandle, page, modifier, page_x, page_y
        )
    }
    pub unsafe fn FORM_OnKeyDown(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        nKeyCode: ::core::ffi::c_int,
        modifier: ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FORM_OnKeyDown
            .as_ref()
            .expect("Expected function, got error."))(hHandle, page, nKeyCode, modifier)
    }
    pub unsafe fn FORM_OnKeyUp(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        nKeyCode: ::core::ffi::c_int,
        modifier: ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FORM_OnKeyUp
            .as_ref()
            .expect("Expected function, got error."))(hHandle, page, nKeyCode, modifier)
    }
    pub unsafe fn FORM_OnChar(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        nChar: ::core::ffi::c_int,
        modifier: ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FORM_OnChar
            .as_ref()
            .expect("Expected function, got error."))(hHandle, page, nChar, modifier)
    }
    pub unsafe fn FORM_GetFocusedText(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FORM_GetFocusedText
            .as_ref()
            .expect("Expected function, got error."))(hHandle, page, buffer, buflen)
    }
    pub unsafe fn FORM_GetSelectedText(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FORM_GetSelectedText
            .as_ref()
            .expect("Expected function, got error."))(hHandle, page, buffer, buflen)
    }
    pub unsafe fn FORM_ReplaceAndKeepSelection(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        wsText: FPDF_WIDESTRING,
    ) {
        (self
            .FORM_ReplaceAndKeepSelection
            .as_ref()
            .expect("Expected function, got error."))(hHandle, page, wsText)
    }
    pub unsafe fn FORM_ReplaceSelection(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        wsText: FPDF_WIDESTRING,
    ) {
        (self
            .FORM_ReplaceSelection
            .as_ref()
            .expect("Expected function, got error."))(hHandle, page, wsText)
    }
    pub unsafe fn FORM_SelectAllText(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
    ) -> FPDF_BOOL {
        (self
            .FORM_SelectAllText
            .as_ref()
            .expect("Expected function, got error."))(hHandle, page)
    }
    pub unsafe fn FORM_CanUndo(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> FPDF_BOOL {
        (self
            .FORM_CanUndo
            .as_ref()
            .expect("Expected function, got error."))(hHandle, page)
    }
    pub unsafe fn FORM_CanRedo(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> FPDF_BOOL {
        (self
            .FORM_CanRedo
            .as_ref()
            .expect("Expected function, got error."))(hHandle, page)
    }
    pub unsafe fn FORM_Undo(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> FPDF_BOOL {
        (self
            .FORM_Undo
            .as_ref()
            .expect("Expected function, got error."))(hHandle, page)
    }
    pub unsafe fn FORM_Redo(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> FPDF_BOOL {
        (self
            .FORM_Redo
            .as_ref()
            .expect("Expected function, got error."))(hHandle, page)
    }
    pub unsafe fn FORM_ForceToKillFocus(&self, hHandle: FPDF_FORMHANDLE) -> FPDF_BOOL {
        (self
            .FORM_ForceToKillFocus
            .as_ref()
            .expect("Expected function, got error."))(hHandle)
    }
    pub unsafe fn FORM_GetFocusedAnnot(
        &self,
        handle: FPDF_FORMHANDLE,
        page_index: *mut ::core::ffi::c_int,
        annot: *mut FPDF_ANNOTATION,
    ) -> FPDF_BOOL {
        (self
            .FORM_GetFocusedAnnot
            .as_ref()
            .expect("Expected function, got error."))(handle, page_index, annot)
    }
    pub unsafe fn FORM_SetFocusedAnnot(
        &self,
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> FPDF_BOOL {
        (self
            .FORM_SetFocusedAnnot
            .as_ref()
            .expect("Expected function, got error."))(handle, annot)
    }
    pub unsafe fn FPDFPage_HasFormFieldAtPoint(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        page_x: f64,
        page_y: f64,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFPage_HasFormFieldAtPoint
            .as_ref()
            .expect("Expected function, got error."))(hHandle, page, page_x, page_y)
    }
    pub unsafe fn FPDFPage_FormFieldZOrderAtPoint(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        page_x: f64,
        page_y: f64,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFPage_FormFieldZOrderAtPoint
            .as_ref()
            .expect("Expected function, got error."))(hHandle, page, page_x, page_y)
    }
    pub unsafe fn FPDF_SetFormFieldHighlightColor(
        &self,
        hHandle: FPDF_FORMHANDLE,
        fieldType: ::core::ffi::c_int,
        color: ::core::ffi::c_ulong,
    ) {
        (self
            .FPDF_SetFormFieldHighlightColor
            .as_ref()
            .expect("Expected function, got error."))(hHandle, fieldType, color)
    }
    pub unsafe fn FPDF_SetFormFieldHighlightAlpha(
        &self,
        hHandle: FPDF_FORMHANDLE,
        alpha: ::core::ffi::c_uchar,
    ) {
        (self
            .FPDF_SetFormFieldHighlightAlpha
            .as_ref()
            .expect("Expected function, got error."))(hHandle, alpha)
    }
    pub unsafe fn FPDF_RemoveFormFieldHighlight(&self, hHandle: FPDF_FORMHANDLE) {
        (self
            .FPDF_RemoveFormFieldHighlight
            .as_ref()
            .expect("Expected function, got error."))(hHandle)
    }
    pub unsafe fn FPDF_FFLDraw(
        &self,
        hHandle: FPDF_FORMHANDLE,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: ::core::ffi::c_int,
        start_y: ::core::ffi::c_int,
        size_x: ::core::ffi::c_int,
        size_y: ::core::ffi::c_int,
        rotate: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) {
        (self
            .FPDF_FFLDraw
            .as_ref()
            .expect("Expected function, got error."))(
            hHandle, bitmap, page, start_x, start_y, size_x, size_y, rotate, flags,
        )
    }
    pub unsafe fn FPDF_GetFormType(&self, document: FPDF_DOCUMENT) -> ::core::ffi::c_int {
        (self
            .FPDF_GetFormType
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FORM_SetIndexSelected(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        index: ::core::ffi::c_int,
        selected: FPDF_BOOL,
    ) -> FPDF_BOOL {
        (self
            .FORM_SetIndexSelected
            .as_ref()
            .expect("Expected function, got error."))(hHandle, page, index, selected)
    }
    pub unsafe fn FORM_IsIndexSelected(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        index: ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FORM_IsIndexSelected
            .as_ref()
            .expect("Expected function, got error."))(hHandle, page, index)
    }
    pub unsafe fn FORM_GetTextDirection(
        &self,
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> FPDF_TEXT_DIRECTION {
        (self
            .FORM_GetTextDirection
            .as_ref()
            .expect("Expected function, got error."))(handle, annot)
    }
    pub unsafe fn FORM_SetTextDirection(
        &self,
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        direction: FPDF_TEXT_DIRECTION,
    ) -> FPDF_BOOL {
        (self
            .FORM_SetTextDirection
            .as_ref()
            .expect("Expected function, got error."))(handle, annot, direction)
    }
    pub unsafe fn FPDF_LoadXFA(&self, document: FPDF_DOCUMENT) -> FPDF_BOOL {
        (self
            .FPDF_LoadXFA
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDFAnnot_IsSupportedSubtype(
        &self,
        subtype: FPDF_ANNOTATION_SUBTYPE,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_IsSupportedSubtype
            .as_ref()
            .expect("Expected function, got error."))(subtype)
    }
    pub unsafe fn FPDFPage_CreateAnnot(
        &self,
        page: FPDF_PAGE,
        subtype: FPDF_ANNOTATION_SUBTYPE,
    ) -> FPDF_ANNOTATION {
        (self
            .FPDFPage_CreateAnnot
            .as_ref()
            .expect("Expected function, got error."))(page, subtype)
    }
    pub unsafe fn FPDFPage_GetAnnotCount(&self, page: FPDF_PAGE) -> ::core::ffi::c_int {
        (self
            .FPDFPage_GetAnnotCount
            .as_ref()
            .expect("Expected function, got error."))(page)
    }
    pub unsafe fn FPDFPage_GetAnnot(
        &self,
        page: FPDF_PAGE,
        index: ::core::ffi::c_int,
    ) -> FPDF_ANNOTATION {
        (self
            .FPDFPage_GetAnnot
            .as_ref()
            .expect("Expected function, got error."))(page, index)
    }
    pub unsafe fn FPDFPage_GetAnnotIndex(
        &self,
        page: FPDF_PAGE,
        annot: FPDF_ANNOTATION,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFPage_GetAnnotIndex
            .as_ref()
            .expect("Expected function, got error."))(page, annot)
    }
    pub unsafe fn FPDFPage_CloseAnnot(&self, annot: FPDF_ANNOTATION) {
        (self
            .FPDFPage_CloseAnnot
            .as_ref()
            .expect("Expected function, got error."))(annot)
    }
    pub unsafe fn FPDFPage_RemoveAnnot(
        &self,
        page: FPDF_PAGE,
        index: ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FPDFPage_RemoveAnnot
            .as_ref()
            .expect("Expected function, got error."))(page, index)
    }
    pub unsafe fn FPDFAnnot_GetSubtype(&self, annot: FPDF_ANNOTATION) -> FPDF_ANNOTATION_SUBTYPE {
        (self
            .FPDFAnnot_GetSubtype
            .as_ref()
            .expect("Expected function, got error."))(annot)
    }
    pub unsafe fn FPDFAnnot_IsObjectSupportedSubtype(
        &self,
        subtype: FPDF_ANNOTATION_SUBTYPE,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_IsObjectSupportedSubtype
            .as_ref()
            .expect("Expected function, got error."))(subtype)
    }
    pub unsafe fn FPDFAnnot_UpdateObject(
        &self,
        annot: FPDF_ANNOTATION,
        obj: FPDF_PAGEOBJECT,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_UpdateObject
            .as_ref()
            .expect("Expected function, got error."))(annot, obj)
    }
    pub unsafe fn FPDFAnnot_AddInkStroke(
        &self,
        annot: FPDF_ANNOTATION,
        points: *const FS_POINTF,
        point_count: usize,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFAnnot_AddInkStroke
            .as_ref()
            .expect("Expected function, got error."))(annot, points, point_count)
    }
    pub unsafe fn FPDFAnnot_RemoveInkList(&self, annot: FPDF_ANNOTATION) -> FPDF_BOOL {
        (self
            .FPDFAnnot_RemoveInkList
            .as_ref()
            .expect("Expected function, got error."))(annot)
    }
    pub unsafe fn FPDFAnnot_AppendObject(
        &self,
        annot: FPDF_ANNOTATION,
        obj: FPDF_PAGEOBJECT,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_AppendObject
            .as_ref()
            .expect("Expected function, got error."))(annot, obj)
    }
    pub unsafe fn FPDFAnnot_GetObjectCount(&self, annot: FPDF_ANNOTATION) -> ::core::ffi::c_int {
        (self
            .FPDFAnnot_GetObjectCount
            .as_ref()
            .expect("Expected function, got error."))(annot)
    }
    pub unsafe fn FPDFAnnot_GetObject(
        &self,
        annot: FPDF_ANNOTATION,
        index: ::core::ffi::c_int,
    ) -> FPDF_PAGEOBJECT {
        (self
            .FPDFAnnot_GetObject
            .as_ref()
            .expect("Expected function, got error."))(annot, index)
    }
    pub unsafe fn FPDFAnnot_RemoveObject(
        &self,
        annot: FPDF_ANNOTATION,
        index: ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_RemoveObject
            .as_ref()
            .expect("Expected function, got error."))(annot, index)
    }
    pub unsafe fn FPDFAnnot_SetColor(
        &self,
        annot: FPDF_ANNOTATION,
        type_: FPDFANNOT_COLORTYPE,
        R: ::core::ffi::c_uint,
        G: ::core::ffi::c_uint,
        B: ::core::ffi::c_uint,
        A: ::core::ffi::c_uint,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_SetColor
            .as_ref()
            .expect("Expected function, got error."))(annot, type_, R, G, B, A)
    }
    pub unsafe fn FPDFAnnot_GetColor(
        &self,
        annot: FPDF_ANNOTATION,
        type_: FPDFANNOT_COLORTYPE,
        R: *mut ::core::ffi::c_uint,
        G: *mut ::core::ffi::c_uint,
        B: *mut ::core::ffi::c_uint,
        A: *mut ::core::ffi::c_uint,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_GetColor
            .as_ref()
            .expect("Expected function, got error."))(annot, type_, R, G, B, A)
    }
    pub unsafe fn FPDFAnnot_HasAttachmentPoints(&self, annot: FPDF_ANNOTATION) -> FPDF_BOOL {
        (self
            .FPDFAnnot_HasAttachmentPoints
            .as_ref()
            .expect("Expected function, got error."))(annot)
    }
    pub unsafe fn FPDFAnnot_SetAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_index: usize,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_SetAttachmentPoints
            .as_ref()
            .expect("Expected function, got error."))(annot, quad_index, quad_points)
    }
    pub unsafe fn FPDFAnnot_AppendAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_AppendAttachmentPoints
            .as_ref()
            .expect("Expected function, got error."))(annot, quad_points)
    }
    pub unsafe fn FPDFAnnot_CountAttachmentPoints(&self, annot: FPDF_ANNOTATION) -> usize {
        (self
            .FPDFAnnot_CountAttachmentPoints
            .as_ref()
            .expect("Expected function, got error."))(annot)
    }
    pub unsafe fn FPDFAnnot_GetAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_index: usize,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_GetAttachmentPoints
            .as_ref()
            .expect("Expected function, got error."))(annot, quad_index, quad_points)
    }
    pub unsafe fn FPDFAnnot_SetRect(
        &self,
        annot: FPDF_ANNOTATION,
        rect: *const FS_RECTF,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_SetRect
            .as_ref()
            .expect("Expected function, got error."))(annot, rect)
    }
    pub unsafe fn FPDFAnnot_GetRect(
        &self,
        annot: FPDF_ANNOTATION,
        rect: *mut FS_RECTF,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_GetRect
            .as_ref()
            .expect("Expected function, got error."))(annot, rect)
    }
    pub unsafe fn FPDFAnnot_GetVertices(
        &self,
        annot: FPDF_ANNOTATION,
        buffer: *mut FS_POINTF,
        length: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFAnnot_GetVertices
            .as_ref()
            .expect("Expected function, got error."))(annot, buffer, length)
    }
    pub unsafe fn FPDFAnnot_GetInkListCount(&self, annot: FPDF_ANNOTATION) -> ::core::ffi::c_ulong {
        (self
            .FPDFAnnot_GetInkListCount
            .as_ref()
            .expect("Expected function, got error."))(annot)
    }
    pub unsafe fn FPDFAnnot_GetInkListPath(
        &self,
        annot: FPDF_ANNOTATION,
        path_index: ::core::ffi::c_ulong,
        buffer: *mut FS_POINTF,
        length: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFAnnot_GetInkListPath
            .as_ref()
            .expect("Expected function, got error."))(annot, path_index, buffer, length)
    }
    pub unsafe fn FPDFAnnot_GetLine(
        &self,
        annot: FPDF_ANNOTATION,
        start: *mut FS_POINTF,
        end: *mut FS_POINTF,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_GetLine
            .as_ref()
            .expect("Expected function, got error."))(annot, start, end)
    }
    pub unsafe fn FPDFAnnot_SetBorder(
        &self,
        annot: FPDF_ANNOTATION,
        horizontal_radius: f32,
        vertical_radius: f32,
        border_width: f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_SetBorder
            .as_ref()
            .expect("Expected function, got error."))(
            annot,
            horizontal_radius,
            vertical_radius,
            border_width,
        )
    }
    pub unsafe fn FPDFAnnot_GetBorder(
        &self,
        annot: FPDF_ANNOTATION,
        horizontal_radius: *mut f32,
        vertical_radius: *mut f32,
        border_width: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_GetBorder
            .as_ref()
            .expect("Expected function, got error."))(
            annot,
            horizontal_radius,
            vertical_radius,
            border_width,
        )
    }
    pub unsafe fn FPDFAnnot_GetFormAdditionalActionJavaScript(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        event: ::core::ffi::c_int,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFAnnot_GetFormAdditionalActionJavaScript
            .as_ref()
            .expect("Expected function, got error."))(hHandle, annot, event, buffer, buflen)
    }
    pub unsafe fn FPDFAnnot_HasKey(
        &self,
        annot: FPDF_ANNOTATION,
        key: FPDF_BYTESTRING,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_HasKey
            .as_ref()
            .expect("Expected function, got error."))(annot, key)
    }
    pub unsafe fn FPDFAnnot_GetValueType(
        &self,
        annot: FPDF_ANNOTATION,
        key: FPDF_BYTESTRING,
    ) -> FPDF_OBJECT_TYPE {
        (self
            .FPDFAnnot_GetValueType
            .as_ref()
            .expect("Expected function, got error."))(annot, key)
    }
    pub unsafe fn FPDFAnnot_SetStringValue(
        &self,
        annot: FPDF_ANNOTATION,
        key: FPDF_BYTESTRING,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_SetStringValue
            .as_ref()
            .expect("Expected function, got error."))(annot, key, value)
    }
    pub unsafe fn FPDFAnnot_GetStringValue(
        &self,
        annot: FPDF_ANNOTATION,
        key: FPDF_BYTESTRING,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFAnnot_GetStringValue
            .as_ref()
            .expect("Expected function, got error."))(annot, key, buffer, buflen)
    }
    pub unsafe fn FPDFAnnot_GetNumberValue(
        &self,
        annot: FPDF_ANNOTATION,
        key: FPDF_BYTESTRING,
        value: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_GetNumberValue
            .as_ref()
            .expect("Expected function, got error."))(annot, key, value)
    }
    pub unsafe fn FPDFAnnot_SetAP(
        &self,
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_SetAP
            .as_ref()
            .expect("Expected function, got error."))(annot, appearanceMode, value)
    }
    pub unsafe fn FPDFAnnot_GetAP(
        &self,
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFAnnot_GetAP
            .as_ref()
            .expect("Expected function, got error."))(annot, appearanceMode, buffer, buflen)
    }
    pub unsafe fn FPDFAnnot_GetLinkedAnnot(
        &self,
        annot: FPDF_ANNOTATION,
        key: FPDF_BYTESTRING,
    ) -> FPDF_ANNOTATION {
        (self
            .FPDFAnnot_GetLinkedAnnot
            .as_ref()
            .expect("Expected function, got error."))(annot, key)
    }
    pub unsafe fn FPDFAnnot_GetFlags(&self, annot: FPDF_ANNOTATION) -> ::core::ffi::c_int {
        (self
            .FPDFAnnot_GetFlags
            .as_ref()
            .expect("Expected function, got error."))(annot)
    }
    pub unsafe fn FPDFAnnot_SetFlags(
        &self,
        annot: FPDF_ANNOTATION,
        flags: ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_SetFlags
            .as_ref()
            .expect("Expected function, got error."))(annot, flags)
    }
    pub unsafe fn FPDFAnnot_GetFormFieldFlags(
        &self,
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFAnnot_GetFormFieldFlags
            .as_ref()
            .expect("Expected function, got error."))(handle, annot)
    }
    pub unsafe fn FPDFAnnot_SetFormFieldFlags(
        &self,
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        flags: ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_SetFormFieldFlags
            .as_ref()
            .expect("Expected function, got error."))(handle, annot, flags)
    }
    pub unsafe fn FPDFAnnot_GetFormFieldAtPoint(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        point: *const FS_POINTF,
    ) -> FPDF_ANNOTATION {
        (self
            .FPDFAnnot_GetFormFieldAtPoint
            .as_ref()
            .expect("Expected function, got error."))(hHandle, page, point)
    }
    pub unsafe fn FPDFAnnot_GetFormFieldName(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFAnnot_GetFormFieldName
            .as_ref()
            .expect("Expected function, got error."))(hHandle, annot, buffer, buflen)
    }
    pub unsafe fn FPDFAnnot_GetFormFieldAlternateName(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFAnnot_GetFormFieldAlternateName
            .as_ref()
            .expect("Expected function, got error."))(hHandle, annot, buffer, buflen)
    }
    pub unsafe fn FPDFAnnot_GetFormFieldType(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFAnnot_GetFormFieldType
            .as_ref()
            .expect("Expected function, got error."))(hHandle, annot)
    }
    pub unsafe fn FPDFAnnot_GetFormFieldValue(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFAnnot_GetFormFieldValue
            .as_ref()
            .expect("Expected function, got error."))(hHandle, annot, buffer, buflen)
    }
    pub unsafe fn FPDFAnnot_GetOptionCount(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFAnnot_GetOptionCount
            .as_ref()
            .expect("Expected function, got error."))(hHandle, annot)
    }
    pub unsafe fn FPDFAnnot_GetOptionLabel(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        index: ::core::ffi::c_int,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFAnnot_GetOptionLabel
            .as_ref()
            .expect("Expected function, got error."))(hHandle, annot, index, buffer, buflen)
    }
    pub unsafe fn FPDFAnnot_IsOptionSelected(
        &self,
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        index: ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_IsOptionSelected
            .as_ref()
            .expect("Expected function, got error."))(handle, annot, index)
    }
    pub unsafe fn FPDFAnnot_GetFontSize(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        value: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_GetFontSize
            .as_ref()
            .expect("Expected function, got error."))(hHandle, annot, value)
    }
    pub unsafe fn FPDFAnnot_SetFontColor(
        &self,
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        R: ::core::ffi::c_uint,
        G: ::core::ffi::c_uint,
        B: ::core::ffi::c_uint,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_SetFontColor
            .as_ref()
            .expect("Expected function, got error."))(handle, annot, R, G, B)
    }
    pub unsafe fn FPDFAnnot_GetFontColor(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        R: *mut ::core::ffi::c_uint,
        G: *mut ::core::ffi::c_uint,
        B: *mut ::core::ffi::c_uint,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_GetFontColor
            .as_ref()
            .expect("Expected function, got error."))(hHandle, annot, R, G, B)
    }
    pub unsafe fn FPDFAnnot_IsChecked(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_IsChecked
            .as_ref()
            .expect("Expected function, got error."))(hHandle, annot)
    }
    pub unsafe fn FPDFAnnot_SetFocusableSubtypes(
        &self,
        hHandle: FPDF_FORMHANDLE,
        subtypes: *const FPDF_ANNOTATION_SUBTYPE,
        count: usize,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_SetFocusableSubtypes
            .as_ref()
            .expect("Expected function, got error."))(hHandle, subtypes, count)
    }
    pub unsafe fn FPDFAnnot_GetFocusableSubtypesCount(
        &self,
        hHandle: FPDF_FORMHANDLE,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFAnnot_GetFocusableSubtypesCount
            .as_ref()
            .expect("Expected function, got error."))(hHandle)
    }
    pub unsafe fn FPDFAnnot_GetFocusableSubtypes(
        &self,
        hHandle: FPDF_FORMHANDLE,
        subtypes: *mut FPDF_ANNOTATION_SUBTYPE,
        count: usize,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_GetFocusableSubtypes
            .as_ref()
            .expect("Expected function, got error."))(hHandle, subtypes, count)
    }
    pub unsafe fn FPDFAnnot_GetLink(&self, annot: FPDF_ANNOTATION) -> FPDF_LINK {
        (self
            .FPDFAnnot_GetLink
            .as_ref()
            .expect("Expected function, got error."))(annot)
    }
    pub unsafe fn FPDFAnnot_GetFormControlCount(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFAnnot_GetFormControlCount
            .as_ref()
            .expect("Expected function, got error."))(hHandle, annot)
    }
    pub unsafe fn FPDFAnnot_GetFormControlIndex(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFAnnot_GetFormControlIndex
            .as_ref()
            .expect("Expected function, got error."))(hHandle, annot)
    }
    pub unsafe fn FPDFAnnot_GetFormFieldExportValue(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFAnnot_GetFormFieldExportValue
            .as_ref()
            .expect("Expected function, got error."))(hHandle, annot, buffer, buflen)
    }
    pub unsafe fn FPDFAnnot_SetURI(
        &self,
        annot: FPDF_ANNOTATION,
        uri: *const ::core::ffi::c_char,
    ) -> FPDF_BOOL {
        (self
            .FPDFAnnot_SetURI
            .as_ref()
            .expect("Expected function, got error."))(annot, uri)
    }
    pub unsafe fn FPDFAnnot_GetFileAttachment(&self, annot: FPDF_ANNOTATION) -> FPDF_ATTACHMENT {
        (self
            .FPDFAnnot_GetFileAttachment
            .as_ref()
            .expect("Expected function, got error."))(annot)
    }
    pub unsafe fn FPDFAnnot_AddFileAttachment(
        &self,
        annot: FPDF_ANNOTATION,
        name: FPDF_WIDESTRING,
    ) -> FPDF_ATTACHMENT {
        (self
            .FPDFAnnot_AddFileAttachment
            .as_ref()
            .expect("Expected function, got error."))(annot, name)
    }
    pub unsafe fn FPDFDoc_GetAttachmentCount(&self, document: FPDF_DOCUMENT) -> ::core::ffi::c_int {
        (self
            .FPDFDoc_GetAttachmentCount
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDFDoc_AddAttachment(
        &self,
        document: FPDF_DOCUMENT,
        name: FPDF_WIDESTRING,
    ) -> FPDF_ATTACHMENT {
        (self
            .FPDFDoc_AddAttachment
            .as_ref()
            .expect("Expected function, got error."))(document, name)
    }
    pub unsafe fn FPDFDoc_GetAttachment(
        &self,
        document: FPDF_DOCUMENT,
        index: ::core::ffi::c_int,
    ) -> FPDF_ATTACHMENT {
        (self
            .FPDFDoc_GetAttachment
            .as_ref()
            .expect("Expected function, got error."))(document, index)
    }
    pub unsafe fn FPDFDoc_DeleteAttachment(
        &self,
        document: FPDF_DOCUMENT,
        index: ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FPDFDoc_DeleteAttachment
            .as_ref()
            .expect("Expected function, got error."))(document, index)
    }
    pub unsafe fn FPDFAttachment_GetName(
        &self,
        attachment: FPDF_ATTACHMENT,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFAttachment_GetName
            .as_ref()
            .expect("Expected function, got error."))(attachment, buffer, buflen)
    }
    pub unsafe fn FPDFAttachment_HasKey(
        &self,
        attachment: FPDF_ATTACHMENT,
        key: FPDF_BYTESTRING,
    ) -> FPDF_BOOL {
        (self
            .FPDFAttachment_HasKey
            .as_ref()
            .expect("Expected function, got error."))(attachment, key)
    }
    pub unsafe fn FPDFAttachment_GetValueType(
        &self,
        attachment: FPDF_ATTACHMENT,
        key: FPDF_BYTESTRING,
    ) -> FPDF_OBJECT_TYPE {
        (self
            .FPDFAttachment_GetValueType
            .as_ref()
            .expect("Expected function, got error."))(attachment, key)
    }
    pub unsafe fn FPDFAttachment_SetStringValue(
        &self,
        attachment: FPDF_ATTACHMENT,
        key: FPDF_BYTESTRING,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL {
        (self
            .FPDFAttachment_SetStringValue
            .as_ref()
            .expect("Expected function, got error."))(attachment, key, value)
    }
    pub unsafe fn FPDFAttachment_GetStringValue(
        &self,
        attachment: FPDF_ATTACHMENT,
        key: FPDF_BYTESTRING,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFAttachment_GetStringValue
            .as_ref()
            .expect("Expected function, got error."))(attachment, key, buffer, buflen)
    }
    pub unsafe fn FPDFAttachment_SetDescription(
        &self,
        attachment: FPDF_ATTACHMENT,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL {
        (self
            .FPDFAttachment_SetDescription
            .as_ref()
            .expect("Expected function, got error."))(attachment, value)
    }
    pub unsafe fn FPDFAttachment_GetDescription(
        &self,
        attachment: FPDF_ATTACHMENT,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFAttachment_GetDescription
            .as_ref()
            .expect("Expected function, got error."))(attachment, buffer, buflen)
    }
    pub unsafe fn FPDFAttachment_SetFile(
        &self,
        attachment: FPDF_ATTACHMENT,
        document: FPDF_DOCUMENT,
        contents: *const ::core::ffi::c_void,
        len: ::core::ffi::c_ulong,
    ) -> FPDF_BOOL {
        (self
            .FPDFAttachment_SetFile
            .as_ref()
            .expect("Expected function, got error."))(attachment, document, contents, len)
    }
    pub unsafe fn FPDFAttachment_GetFile(
        &self,
        attachment: FPDF_ATTACHMENT,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
        out_buflen: *mut ::core::ffi::c_ulong,
    ) -> FPDF_BOOL {
        (self
            .FPDFAttachment_GetFile
            .as_ref()
            .expect("Expected function, got error."))(attachment, buffer, buflen, out_buflen)
    }
    pub unsafe fn FPDFAttachment_GetSubtype(
        &self,
        attachment: FPDF_ATTACHMENT,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFAttachment_GetSubtype
            .as_ref()
            .expect("Expected function, got error."))(attachment, buffer, buflen)
    }
    pub unsafe fn FPDFAttachment_GetAFRelationship(
        &self,
        attachment: FPDF_ATTACHMENT,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFAttachment_GetAFRelationship
            .as_ref()
            .expect("Expected function, got error."))(attachment, buffer, buflen)
    }
    pub unsafe fn FPDFCatalog_IsTagged(&self, document: FPDF_DOCUMENT) -> FPDF_BOOL {
        (self
            .FPDFCatalog_IsTagged
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDFCatalog_GetLanguage(
        &self,
        document: FPDF_DOCUMENT,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFCatalog_GetLanguage
            .as_ref()
            .expect("Expected function, got error."))(document, buffer, buflen)
    }
    pub unsafe fn FPDFCatalog_SetLanguage(
        &self,
        document: FPDF_DOCUMENT,
        language: FPDF_WIDESTRING,
    ) -> FPDF_BOOL {
        (self
            .FPDFCatalog_SetLanguage
            .as_ref()
            .expect("Expected function, got error."))(document, language)
    }
    pub unsafe fn FPDFAvail_Create(
        &self,
        file_avail: *mut FX_FILEAVAIL,
        file: *mut FPDF_FILEACCESS,
    ) -> FPDF_AVAIL {
        (self
            .FPDFAvail_Create
            .as_ref()
            .expect("Expected function, got error."))(file_avail, file)
    }
    pub unsafe fn FPDFAvail_Destroy(&self, avail: FPDF_AVAIL) {
        (self
            .FPDFAvail_Destroy
            .as_ref()
            .expect("Expected function, got error."))(avail)
    }
    pub unsafe fn FPDFAvail_IsDocAvail(
        &self,
        avail: FPDF_AVAIL,
        hints: *mut FX_DOWNLOADHINTS,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFAvail_IsDocAvail
            .as_ref()
            .expect("Expected function, got error."))(avail, hints)
    }
    pub unsafe fn FPDFAvail_GetDocument(
        &self,
        avail: FPDF_AVAIL,
        password: FPDF_BYTESTRING,
    ) -> FPDF_DOCUMENT {
        (self
            .FPDFAvail_GetDocument
            .as_ref()
            .expect("Expected function, got error."))(avail, password)
    }
    pub unsafe fn FPDFAvail_GetFirstPageNum(&self, doc: FPDF_DOCUMENT) -> ::core::ffi::c_int {
        (self
            .FPDFAvail_GetFirstPageNum
            .as_ref()
            .expect("Expected function, got error."))(doc)
    }
    pub unsafe fn FPDFAvail_IsPageAvail(
        &self,
        avail: FPDF_AVAIL,
        page_index: ::core::ffi::c_int,
        hints: *mut FX_DOWNLOADHINTS,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFAvail_IsPageAvail
            .as_ref()
            .expect("Expected function, got error."))(avail, page_index, hints)
    }
    pub unsafe fn FPDFAvail_IsFormAvail(
        &self,
        avail: FPDF_AVAIL,
        hints: *mut FX_DOWNLOADHINTS,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFAvail_IsFormAvail
            .as_ref()
            .expect("Expected function, got error."))(avail, hints)
    }
    pub unsafe fn FPDFAvail_IsLinearized(&self, avail: FPDF_AVAIL) -> ::core::ffi::c_int {
        (self
            .FPDFAvail_IsLinearized
            .as_ref()
            .expect("Expected function, got error."))(avail)
    }
    pub unsafe fn FPDFBookmark_GetFirstChild(
        &self,
        document: FPDF_DOCUMENT,
        bookmark: FPDF_BOOKMARK,
    ) -> FPDF_BOOKMARK {
        (self
            .FPDFBookmark_GetFirstChild
            .as_ref()
            .expect("Expected function, got error."))(document, bookmark)
    }
    pub unsafe fn FPDFBookmark_GetNextSibling(
        &self,
        document: FPDF_DOCUMENT,
        bookmark: FPDF_BOOKMARK,
    ) -> FPDF_BOOKMARK {
        (self
            .FPDFBookmark_GetNextSibling
            .as_ref()
            .expect("Expected function, got error."))(document, bookmark)
    }
    pub unsafe fn FPDFBookmark_GetTitle(
        &self,
        bookmark: FPDF_BOOKMARK,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFBookmark_GetTitle
            .as_ref()
            .expect("Expected function, got error."))(bookmark, buffer, buflen)
    }
    pub unsafe fn FPDFBookmark_GetCount(&self, bookmark: FPDF_BOOKMARK) -> ::core::ffi::c_int {
        (self
            .FPDFBookmark_GetCount
            .as_ref()
            .expect("Expected function, got error."))(bookmark)
    }
    pub unsafe fn FPDFBookmark_Find(
        &self,
        document: FPDF_DOCUMENT,
        title: FPDF_WIDESTRING,
    ) -> FPDF_BOOKMARK {
        (self
            .FPDFBookmark_Find
            .as_ref()
            .expect("Expected function, got error."))(document, title)
    }
    pub unsafe fn FPDFBookmark_GetDest(
        &self,
        document: FPDF_DOCUMENT,
        bookmark: FPDF_BOOKMARK,
    ) -> FPDF_DEST {
        (self
            .FPDFBookmark_GetDest
            .as_ref()
            .expect("Expected function, got error."))(document, bookmark)
    }
    pub unsafe fn FPDFBookmark_GetAction(&self, bookmark: FPDF_BOOKMARK) -> FPDF_ACTION {
        (self
            .FPDFBookmark_GetAction
            .as_ref()
            .expect("Expected function, got error."))(bookmark)
    }
    pub unsafe fn FPDFBookmark_GetColor(
        &self,
        bookmark: FPDF_BOOKMARK,
        R: *mut f32,
        G: *mut f32,
        B: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFBookmark_GetColor
            .as_ref()
            .expect("Expected function, got error."))(bookmark, R, G, B)
    }
    pub unsafe fn FPDFBookmark_GetStyle(&self, bookmark: FPDF_BOOKMARK) -> ::core::ffi::c_int {
        (self
            .FPDFBookmark_GetStyle
            .as_ref()
            .expect("Expected function, got error."))(bookmark)
    }
    pub unsafe fn FPDFAction_GetType(&self, action: FPDF_ACTION) -> ::core::ffi::c_ulong {
        (self
            .FPDFAction_GetType
            .as_ref()
            .expect("Expected function, got error."))(action)
    }
    pub unsafe fn FPDFAction_GetDest(
        &self,
        document: FPDF_DOCUMENT,
        action: FPDF_ACTION,
    ) -> FPDF_DEST {
        (self
            .FPDFAction_GetDest
            .as_ref()
            .expect("Expected function, got error."))(document, action)
    }
    pub unsafe fn FPDFAction_GetFilePath(
        &self,
        action: FPDF_ACTION,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFAction_GetFilePath
            .as_ref()
            .expect("Expected function, got error."))(action, buffer, buflen)
    }
    pub unsafe fn FPDFAction_GetURIPath(
        &self,
        document: FPDF_DOCUMENT,
        action: FPDF_ACTION,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFAction_GetURIPath
            .as_ref()
            .expect("Expected function, got error."))(document, action, buffer, buflen)
    }
    pub unsafe fn FPDFDest_GetDestPageIndex(
        &self,
        document: FPDF_DOCUMENT,
        dest: FPDF_DEST,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFDest_GetDestPageIndex
            .as_ref()
            .expect("Expected function, got error."))(document, dest)
    }
    pub unsafe fn FPDFDest_GetView(
        &self,
        dest: FPDF_DEST,
        pNumParams: *mut ::core::ffi::c_ulong,
        pParams: *mut FS_FLOAT,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFDest_GetView
            .as_ref()
            .expect("Expected function, got error."))(dest, pNumParams, pParams)
    }
    pub unsafe fn FPDFDest_GetLocationInPage(
        &self,
        dest: FPDF_DEST,
        hasXVal: *mut FPDF_BOOL,
        hasYVal: *mut FPDF_BOOL,
        hasZoomVal: *mut FPDF_BOOL,
        x: *mut FS_FLOAT,
        y: *mut FS_FLOAT,
        zoom: *mut FS_FLOAT,
    ) -> FPDF_BOOL {
        (self
            .FPDFDest_GetLocationInPage
            .as_ref()
            .expect("Expected function, got error."))(
            dest, hasXVal, hasYVal, hasZoomVal, x, y, zoom,
        )
    }
    pub unsafe fn FPDFLink_GetLinkAtPoint(&self, page: FPDF_PAGE, x: f64, y: f64) -> FPDF_LINK {
        (self
            .FPDFLink_GetLinkAtPoint
            .as_ref()
            .expect("Expected function, got error."))(page, x, y)
    }
    pub unsafe fn FPDFLink_GetLinkZOrderAtPoint(
        &self,
        page: FPDF_PAGE,
        x: f64,
        y: f64,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFLink_GetLinkZOrderAtPoint
            .as_ref()
            .expect("Expected function, got error."))(page, x, y)
    }
    pub unsafe fn FPDFLink_GetDest(&self, document: FPDF_DOCUMENT, link: FPDF_LINK) -> FPDF_DEST {
        (self
            .FPDFLink_GetDest
            .as_ref()
            .expect("Expected function, got error."))(document, link)
    }
    pub unsafe fn FPDFLink_GetAction(&self, link: FPDF_LINK) -> FPDF_ACTION {
        (self
            .FPDFLink_GetAction
            .as_ref()
            .expect("Expected function, got error."))(link)
    }
    pub unsafe fn FPDFLink_Enumerate(
        &self,
        page: FPDF_PAGE,
        start_pos: *mut ::core::ffi::c_int,
        link_annot: *mut FPDF_LINK,
    ) -> FPDF_BOOL {
        (self
            .FPDFLink_Enumerate
            .as_ref()
            .expect("Expected function, got error."))(page, start_pos, link_annot)
    }
    pub unsafe fn FPDFLink_GetAnnot(
        &self,
        page: FPDF_PAGE,
        link_annot: FPDF_LINK,
    ) -> FPDF_ANNOTATION {
        (self
            .FPDFLink_GetAnnot
            .as_ref()
            .expect("Expected function, got error."))(page, link_annot)
    }
    pub unsafe fn FPDFLink_GetAnnotRect(
        &self,
        link_annot: FPDF_LINK,
        rect: *mut FS_RECTF,
    ) -> FPDF_BOOL {
        (self
            .FPDFLink_GetAnnotRect
            .as_ref()
            .expect("Expected function, got error."))(link_annot, rect)
    }
    pub unsafe fn FPDFLink_CountQuadPoints(&self, link_annot: FPDF_LINK) -> ::core::ffi::c_int {
        (self
            .FPDFLink_CountQuadPoints
            .as_ref()
            .expect("Expected function, got error."))(link_annot)
    }
    pub unsafe fn FPDFLink_GetQuadPoints(
        &self,
        link_annot: FPDF_LINK,
        quad_index: ::core::ffi::c_int,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        (self
            .FPDFLink_GetQuadPoints
            .as_ref()
            .expect("Expected function, got error."))(link_annot, quad_index, quad_points)
    }
    pub unsafe fn FPDF_GetPageAAction(
        &self,
        page: FPDF_PAGE,
        aa_type: ::core::ffi::c_int,
    ) -> FPDF_ACTION {
        (self
            .FPDF_GetPageAAction
            .as_ref()
            .expect("Expected function, got error."))(page, aa_type)
    }
    pub unsafe fn FPDF_GetFileIdentifier(
        &self,
        document: FPDF_DOCUMENT,
        id_type: FPDF_FILEIDTYPE,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDF_GetFileIdentifier
            .as_ref()
            .expect("Expected function, got error."))(document, id_type, buffer, buflen)
    }
    pub unsafe fn FPDF_GetMetaText(
        &self,
        document: FPDF_DOCUMENT,
        tag: FPDF_BYTESTRING,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDF_GetMetaText
            .as_ref()
            .expect("Expected function, got error."))(document, tag, buffer, buflen)
    }
    pub unsafe fn FPDF_GetPageLabel(
        &self,
        document: FPDF_DOCUMENT,
        page_index: ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDF_GetPageLabel
            .as_ref()
            .expect("Expected function, got error."))(document, page_index, buffer, buflen)
    }
    pub unsafe fn FPDF_CreateNewDocument(&self) -> FPDF_DOCUMENT {
        (self
            .FPDF_CreateNewDocument
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn FPDFPage_New(
        &self,
        document: FPDF_DOCUMENT,
        page_index: ::core::ffi::c_int,
        width: f64,
        height: f64,
    ) -> FPDF_PAGE {
        (self
            .FPDFPage_New
            .as_ref()
            .expect("Expected function, got error."))(document, page_index, width, height)
    }
    pub unsafe fn FPDFPage_Delete(&self, document: FPDF_DOCUMENT, page_index: ::core::ffi::c_int) {
        (self
            .FPDFPage_Delete
            .as_ref()
            .expect("Expected function, got error."))(document, page_index)
    }
    pub unsafe fn FPDF_MovePages(
        &self,
        document: FPDF_DOCUMENT,
        page_indices: *const ::core::ffi::c_int,
        page_indices_len: ::core::ffi::c_ulong,
        dest_page_index: ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FPDF_MovePages
            .as_ref()
            .expect("Expected function, got error."))(
            document,
            page_indices,
            page_indices_len,
            dest_page_index,
        )
    }
    pub unsafe fn FPDFPage_GetRotation(&self, page: FPDF_PAGE) -> ::core::ffi::c_int {
        (self
            .FPDFPage_GetRotation
            .as_ref()
            .expect("Expected function, got error."))(page)
    }
    pub unsafe fn FPDFPage_SetRotation(&self, page: FPDF_PAGE, rotate: ::core::ffi::c_int) {
        (self
            .FPDFPage_SetRotation
            .as_ref()
            .expect("Expected function, got error."))(page, rotate)
    }
    pub unsafe fn FPDFPage_InsertObject(
        &self,
        page: FPDF_PAGE,
        page_object: FPDF_PAGEOBJECT,
    ) -> FPDF_BOOL {
        (self
            .FPDFPage_InsertObject
            .as_ref()
            .expect("Expected function, got error."))(page, page_object)
    }
    pub unsafe fn FPDFPage_InsertObjectAtIndex(
        &self,
        page: FPDF_PAGE,
        page_object: FPDF_PAGEOBJECT,
        index: usize,
    ) -> FPDF_BOOL {
        (self
            .FPDFPage_InsertObjectAtIndex
            .as_ref()
            .expect("Expected function, got error."))(page, page_object, index)
    }
    pub unsafe fn FPDFPage_RemoveObject(
        &self,
        page: FPDF_PAGE,
        page_object: FPDF_PAGEOBJECT,
    ) -> FPDF_BOOL {
        (self
            .FPDFPage_RemoveObject
            .as_ref()
            .expect("Expected function, got error."))(page, page_object)
    }
    pub unsafe fn FPDFPage_CountObjects(&self, page: FPDF_PAGE) -> ::core::ffi::c_int {
        (self
            .FPDFPage_CountObjects
            .as_ref()
            .expect("Expected function, got error."))(page)
    }
    pub unsafe fn FPDFPage_GetObject(
        &self,
        page: FPDF_PAGE,
        index: ::core::ffi::c_int,
    ) -> FPDF_PAGEOBJECT {
        (self
            .FPDFPage_GetObject
            .as_ref()
            .expect("Expected function, got error."))(page, index)
    }
    pub unsafe fn FPDFPage_HasTransparency(&self, page: FPDF_PAGE) -> FPDF_BOOL {
        (self
            .FPDFPage_HasTransparency
            .as_ref()
            .expect("Expected function, got error."))(page)
    }
    pub unsafe fn FPDFPage_GenerateContent(&self, page: FPDF_PAGE) -> FPDF_BOOL {
        (self
            .FPDFPage_GenerateContent
            .as_ref()
            .expect("Expected function, got error."))(page)
    }
    pub unsafe fn FPDFPageObj_Destroy(&self, page_object: FPDF_PAGEOBJECT) {
        (self
            .FPDFPageObj_Destroy
            .as_ref()
            .expect("Expected function, got error."))(page_object)
    }
    pub unsafe fn FPDFPageObj_HasTransparency(&self, page_object: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        (self
            .FPDFPageObj_HasTransparency
            .as_ref()
            .expect("Expected function, got error."))(page_object)
    }
    pub unsafe fn FPDFPageObj_GetType(&self, page_object: FPDF_PAGEOBJECT) -> ::core::ffi::c_int {
        (self
            .FPDFPageObj_GetType
            .as_ref()
            .expect("Expected function, got error."))(page_object)
    }
    pub unsafe fn FPDFPageObj_GetIsActive(
        &self,
        page_object: FPDF_PAGEOBJECT,
        active: *mut FPDF_BOOL,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_GetIsActive
            .as_ref()
            .expect("Expected function, got error."))(page_object, active)
    }
    pub unsafe fn FPDFPageObj_SetIsActive(
        &self,
        page_object: FPDF_PAGEOBJECT,
        active: FPDF_BOOL,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_SetIsActive
            .as_ref()
            .expect("Expected function, got error."))(page_object, active)
    }
    pub unsafe fn FPDFPageObj_Transform(
        &self,
        page_object: FPDF_PAGEOBJECT,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) {
        (self
            .FPDFPageObj_Transform
            .as_ref()
            .expect("Expected function, got error."))(page_object, a, b, c, d, e, f)
    }
    pub unsafe fn FPDFPageObj_TransformF(
        &self,
        page_object: FPDF_PAGEOBJECT,
        matrix: *const FS_MATRIX,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_TransformF
            .as_ref()
            .expect("Expected function, got error."))(page_object, matrix)
    }
    pub unsafe fn FPDFPageObj_GetMatrix(
        &self,
        page_object: FPDF_PAGEOBJECT,
        matrix: *mut FS_MATRIX,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_GetMatrix
            .as_ref()
            .expect("Expected function, got error."))(page_object, matrix)
    }
    pub unsafe fn FPDFPageObj_SetMatrix(
        &self,
        page_object: FPDF_PAGEOBJECT,
        matrix: *const FS_MATRIX,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_SetMatrix
            .as_ref()
            .expect("Expected function, got error."))(page_object, matrix)
    }
    pub unsafe fn FPDFPage_TransformAnnots(
        &self,
        page: FPDF_PAGE,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) {
        (self
            .FPDFPage_TransformAnnots
            .as_ref()
            .expect("Expected function, got error."))(page, a, b, c, d, e, f)
    }
    pub unsafe fn FPDFPageObj_NewImageObj(&self, document: FPDF_DOCUMENT) -> FPDF_PAGEOBJECT {
        (self
            .FPDFPageObj_NewImageObj
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDFPageObj_GetMarkedContentID(
        &self,
        page_object: FPDF_PAGEOBJECT,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFPageObj_GetMarkedContentID
            .as_ref()
            .expect("Expected function, got error."))(page_object)
    }
    pub unsafe fn FPDFPageObj_CountMarks(
        &self,
        page_object: FPDF_PAGEOBJECT,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFPageObj_CountMarks
            .as_ref()
            .expect("Expected function, got error."))(page_object)
    }
    pub unsafe fn FPDFPageObj_GetMark(
        &self,
        page_object: FPDF_PAGEOBJECT,
        index: ::core::ffi::c_ulong,
    ) -> FPDF_PAGEOBJECTMARK {
        (self
            .FPDFPageObj_GetMark
            .as_ref()
            .expect("Expected function, got error."))(page_object, index)
    }
    pub unsafe fn FPDFPageObj_AddMark(
        &self,
        page_object: FPDF_PAGEOBJECT,
        name: FPDF_BYTESTRING,
    ) -> FPDF_PAGEOBJECTMARK {
        (self
            .FPDFPageObj_AddMark
            .as_ref()
            .expect("Expected function, got error."))(page_object, name)
    }
    pub unsafe fn FPDFPageObj_AddExistingMark(
        &self,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_AddExistingMark
            .as_ref()
            .expect("Expected function, got error."))(page_object, mark)
    }
    pub unsafe fn FPDFPageObj_RemoveMark(
        &self,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_RemoveMark
            .as_ref()
            .expect("Expected function, got error."))(page_object, mark)
    }
    pub unsafe fn FPDFPageObjMark_GetName(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
        out_buflen: *mut ::core::ffi::c_ulong,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObjMark_GetName
            .as_ref()
            .expect("Expected function, got error."))(mark, buffer, buflen, out_buflen)
    }
    pub unsafe fn FPDFPageObjMark_CountParams(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFPageObjMark_CountParams
            .as_ref()
            .expect("Expected function, got error."))(mark)
    }
    pub unsafe fn FPDFPageObjMark_GetParamKey(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        index: ::core::ffi::c_ulong,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
        out_buflen: *mut ::core::ffi::c_ulong,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObjMark_GetParamKey
            .as_ref()
            .expect("Expected function, got error."))(
            mark, index, buffer, buflen, out_buflen
        )
    }
    pub unsafe fn FPDFPageObjMark_GetParamValueType(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
    ) -> FPDF_OBJECT_TYPE {
        (self
            .FPDFPageObjMark_GetParamValueType
            .as_ref()
            .expect("Expected function, got error."))(mark, key)
    }
    pub unsafe fn FPDFPageObjMark_GetParamIntValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        out_value: *mut ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObjMark_GetParamIntValue
            .as_ref()
            .expect("Expected function, got error."))(mark, key, out_value)
    }
    pub unsafe fn FPDFPageObjMark_GetParamFloatValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        out_value: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObjMark_GetParamFloatValue
            .as_ref()
            .expect("Expected function, got error."))(mark, key, out_value)
    }
    pub unsafe fn FPDFPageObjMark_GetParamStringValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
        out_buflen: *mut ::core::ffi::c_ulong,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObjMark_GetParamStringValue
            .as_ref()
            .expect("Expected function, got error."))(mark, key, buffer, buflen, out_buflen)
    }
    pub unsafe fn FPDFPageObjMark_GetParamBlobValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        buffer: *mut ::core::ffi::c_uchar,
        buflen: ::core::ffi::c_ulong,
        out_buflen: *mut ::core::ffi::c_ulong,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObjMark_GetParamBlobValue
            .as_ref()
            .expect("Expected function, got error."))(mark, key, buffer, buflen, out_buflen)
    }
    pub unsafe fn FPDFPageObjMark_SetIntParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        value: ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObjMark_SetIntParam
            .as_ref()
            .expect("Expected function, got error."))(
            document, page_object, mark, key, value
        )
    }
    pub unsafe fn FPDFPageObjMark_SetFloatParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        value: f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObjMark_SetFloatParam
            .as_ref()
            .expect("Expected function, got error."))(
            document, page_object, mark, key, value
        )
    }
    pub unsafe fn FPDFPageObjMark_SetStringParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        value: FPDF_BYTESTRING,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObjMark_SetStringParam
            .as_ref()
            .expect("Expected function, got error."))(
            document, page_object, mark, key, value
        )
    }
    pub unsafe fn FPDFPageObjMark_SetBlobParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        value: *const ::core::ffi::c_uchar,
        value_len: ::core::ffi::c_ulong,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObjMark_SetBlobParam
            .as_ref()
            .expect("Expected function, got error."))(
            document,
            page_object,
            mark,
            key,
            value,
            value_len,
        )
    }
    pub unsafe fn FPDFPageObjMark_RemoveParam(
        &self,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObjMark_RemoveParam
            .as_ref()
            .expect("Expected function, got error."))(page_object, mark, key)
    }
    pub unsafe fn FPDFImageObj_LoadJpegFile(
        &self,
        pages: *mut FPDF_PAGE,
        count: ::core::ffi::c_int,
        image_object: FPDF_PAGEOBJECT,
        file_access: *mut FPDF_FILEACCESS,
    ) -> FPDF_BOOL {
        (self
            .FPDFImageObj_LoadJpegFile
            .as_ref()
            .expect("Expected function, got error."))(
            pages, count, image_object, file_access
        )
    }
    pub unsafe fn FPDFImageObj_LoadJpegFileInline(
        &self,
        pages: *mut FPDF_PAGE,
        count: ::core::ffi::c_int,
        image_object: FPDF_PAGEOBJECT,
        file_access: *mut FPDF_FILEACCESS,
    ) -> FPDF_BOOL {
        (self
            .FPDFImageObj_LoadJpegFileInline
            .as_ref()
            .expect("Expected function, got error."))(
            pages, count, image_object, file_access
        )
    }
    pub unsafe fn FPDFImageObj_SetMatrix(
        &self,
        image_object: FPDF_PAGEOBJECT,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> FPDF_BOOL {
        (self
            .FPDFImageObj_SetMatrix
            .as_ref()
            .expect("Expected function, got error."))(image_object, a, b, c, d, e, f)
    }
    pub unsafe fn FPDFImageObj_SetBitmap(
        &self,
        pages: *mut FPDF_PAGE,
        count: ::core::ffi::c_int,
        image_object: FPDF_PAGEOBJECT,
        bitmap: FPDF_BITMAP,
    ) -> FPDF_BOOL {
        (self
            .FPDFImageObj_SetBitmap
            .as_ref()
            .expect("Expected function, got error."))(pages, count, image_object, bitmap)
    }
    pub unsafe fn FPDFImageObj_GetBitmap(&self, image_object: FPDF_PAGEOBJECT) -> FPDF_BITMAP {
        (self
            .FPDFImageObj_GetBitmap
            .as_ref()
            .expect("Expected function, got error."))(image_object)
    }
    pub unsafe fn FPDFImageObj_GetRenderedBitmap(
        &self,
        document: FPDF_DOCUMENT,
        page: FPDF_PAGE,
        image_object: FPDF_PAGEOBJECT,
    ) -> FPDF_BITMAP {
        (self
            .FPDFImageObj_GetRenderedBitmap
            .as_ref()
            .expect("Expected function, got error."))(document, page, image_object)
    }
    pub unsafe fn FPDFImageObj_GetImageDataDecoded(
        &self,
        image_object: FPDF_PAGEOBJECT,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFImageObj_GetImageDataDecoded
            .as_ref()
            .expect("Expected function, got error."))(image_object, buffer, buflen)
    }
    pub unsafe fn FPDFImageObj_GetImageDataRaw(
        &self,
        image_object: FPDF_PAGEOBJECT,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFImageObj_GetImageDataRaw
            .as_ref()
            .expect("Expected function, got error."))(image_object, buffer, buflen)
    }
    pub unsafe fn FPDFImageObj_GetImageFilterCount(
        &self,
        image_object: FPDF_PAGEOBJECT,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFImageObj_GetImageFilterCount
            .as_ref()
            .expect("Expected function, got error."))(image_object)
    }
    pub unsafe fn FPDFImageObj_GetImageFilter(
        &self,
        image_object: FPDF_PAGEOBJECT,
        index: ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFImageObj_GetImageFilter
            .as_ref()
            .expect("Expected function, got error."))(image_object, index, buffer, buflen)
    }
    pub unsafe fn FPDFImageObj_GetImageMetadata(
        &self,
        image_object: FPDF_PAGEOBJECT,
        page: FPDF_PAGE,
        metadata: *mut FPDF_IMAGEOBJ_METADATA,
    ) -> FPDF_BOOL {
        (self
            .FPDFImageObj_GetImageMetadata
            .as_ref()
            .expect("Expected function, got error."))(image_object, page, metadata)
    }
    pub unsafe fn FPDFImageObj_GetImagePixelSize(
        &self,
        image_object: FPDF_PAGEOBJECT,
        width: *mut ::core::ffi::c_uint,
        height: *mut ::core::ffi::c_uint,
    ) -> FPDF_BOOL {
        (self
            .FPDFImageObj_GetImagePixelSize
            .as_ref()
            .expect("Expected function, got error."))(image_object, width, height)
    }
    pub unsafe fn FPDFImageObj_GetIccProfileDataDecoded(
        &self,
        image_object: FPDF_PAGEOBJECT,
        page: FPDF_PAGE,
        buffer: *mut u8,
        buflen: usize,
        out_buflen: *mut usize,
    ) -> FPDF_BOOL {
        (self
            .FPDFImageObj_GetIccProfileDataDecoded
            .as_ref()
            .expect("Expected function, got error."))(
            image_object,
            page,
            buffer,
            buflen,
            out_buflen,
        )
    }
    pub unsafe fn FPDFPageObj_CreateNewPath(&self, x: f32, y: f32) -> FPDF_PAGEOBJECT {
        (self
            .FPDFPageObj_CreateNewPath
            .as_ref()
            .expect("Expected function, got error."))(x, y)
    }
    pub unsafe fn FPDFPageObj_CreateNewRect(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> FPDF_PAGEOBJECT {
        (self
            .FPDFPageObj_CreateNewRect
            .as_ref()
            .expect("Expected function, got error."))(x, y, w, h)
    }
    pub unsafe fn FPDFPageObj_GetBounds(
        &self,
        page_object: FPDF_PAGEOBJECT,
        left: *mut f32,
        bottom: *mut f32,
        right: *mut f32,
        top: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_GetBounds
            .as_ref()
            .expect("Expected function, got error."))(page_object, left, bottom, right, top)
    }
    pub unsafe fn FPDFPageObj_GetRotatedBounds(
        &self,
        page_object: FPDF_PAGEOBJECT,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_GetRotatedBounds
            .as_ref()
            .expect("Expected function, got error."))(page_object, quad_points)
    }
    pub unsafe fn FPDFPageObj_SetBlendMode(
        &self,
        page_object: FPDF_PAGEOBJECT,
        blend_mode: FPDF_BYTESTRING,
    ) {
        (self
            .FPDFPageObj_SetBlendMode
            .as_ref()
            .expect("Expected function, got error."))(page_object, blend_mode)
    }
    pub unsafe fn FPDFPageObj_SetStrokeColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: ::core::ffi::c_uint,
        G: ::core::ffi::c_uint,
        B: ::core::ffi::c_uint,
        A: ::core::ffi::c_uint,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_SetStrokeColor
            .as_ref()
            .expect("Expected function, got error."))(page_object, R, G, B, A)
    }
    pub unsafe fn FPDFPageObj_GetStrokeColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: *mut ::core::ffi::c_uint,
        G: *mut ::core::ffi::c_uint,
        B: *mut ::core::ffi::c_uint,
        A: *mut ::core::ffi::c_uint,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_GetStrokeColor
            .as_ref()
            .expect("Expected function, got error."))(page_object, R, G, B, A)
    }
    pub unsafe fn FPDFPageObj_SetStrokeWidth(
        &self,
        page_object: FPDF_PAGEOBJECT,
        width: f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_SetStrokeWidth
            .as_ref()
            .expect("Expected function, got error."))(page_object, width)
    }
    pub unsafe fn FPDFPageObj_GetStrokeWidth(
        &self,
        page_object: FPDF_PAGEOBJECT,
        width: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_GetStrokeWidth
            .as_ref()
            .expect("Expected function, got error."))(page_object, width)
    }
    pub unsafe fn FPDFPageObj_GetLineJoin(
        &self,
        page_object: FPDF_PAGEOBJECT,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFPageObj_GetLineJoin
            .as_ref()
            .expect("Expected function, got error."))(page_object)
    }
    pub unsafe fn FPDFPageObj_SetLineJoin(
        &self,
        page_object: FPDF_PAGEOBJECT,
        line_join: ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_SetLineJoin
            .as_ref()
            .expect("Expected function, got error."))(page_object, line_join)
    }
    pub unsafe fn FPDFPageObj_GetLineCap(
        &self,
        page_object: FPDF_PAGEOBJECT,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFPageObj_GetLineCap
            .as_ref()
            .expect("Expected function, got error."))(page_object)
    }
    pub unsafe fn FPDFPageObj_SetLineCap(
        &self,
        page_object: FPDF_PAGEOBJECT,
        line_cap: ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_SetLineCap
            .as_ref()
            .expect("Expected function, got error."))(page_object, line_cap)
    }
    pub unsafe fn FPDFPageObj_SetFillColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: ::core::ffi::c_uint,
        G: ::core::ffi::c_uint,
        B: ::core::ffi::c_uint,
        A: ::core::ffi::c_uint,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_SetFillColor
            .as_ref()
            .expect("Expected function, got error."))(page_object, R, G, B, A)
    }
    pub unsafe fn FPDFPageObj_GetFillColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: *mut ::core::ffi::c_uint,
        G: *mut ::core::ffi::c_uint,
        B: *mut ::core::ffi::c_uint,
        A: *mut ::core::ffi::c_uint,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_GetFillColor
            .as_ref()
            .expect("Expected function, got error."))(page_object, R, G, B, A)
    }
    pub unsafe fn FPDFPageObj_GetDashPhase(
        &self,
        page_object: FPDF_PAGEOBJECT,
        phase: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_GetDashPhase
            .as_ref()
            .expect("Expected function, got error."))(page_object, phase)
    }
    pub unsafe fn FPDFPageObj_SetDashPhase(
        &self,
        page_object: FPDF_PAGEOBJECT,
        phase: f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_SetDashPhase
            .as_ref()
            .expect("Expected function, got error."))(page_object, phase)
    }
    pub unsafe fn FPDFPageObj_GetDashCount(
        &self,
        page_object: FPDF_PAGEOBJECT,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFPageObj_GetDashCount
            .as_ref()
            .expect("Expected function, got error."))(page_object)
    }
    pub unsafe fn FPDFPageObj_GetDashArray(
        &self,
        page_object: FPDF_PAGEOBJECT,
        dash_array: *mut f32,
        dash_count: usize,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_GetDashArray
            .as_ref()
            .expect("Expected function, got error."))(page_object, dash_array, dash_count)
    }
    pub unsafe fn FPDFPageObj_SetDashArray(
        &self,
        page_object: FPDF_PAGEOBJECT,
        dash_array: *const f32,
        dash_count: usize,
        phase: f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFPageObj_SetDashArray
            .as_ref()
            .expect("Expected function, got error."))(
            page_object, dash_array, dash_count, phase
        )
    }
    pub unsafe fn FPDFPageObj_GetRenderedStrokePattern(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
    ) -> FPDF_BITMAP {
        (self
            .FPDFPageObj_GetRenderedStrokePattern
            .as_ref()
            .expect("Expected function, got error."))(document, page_object)
    }
    pub unsafe fn FPDFPageObj_GetRenderedFillPattern(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
    ) -> FPDF_BITMAP {
        (self
            .FPDFPageObj_GetRenderedFillPattern
            .as_ref()
            .expect("Expected function, got error."))(document, page_object)
    }
    pub unsafe fn FPDFPath_CountSegments(&self, path: FPDF_PAGEOBJECT) -> ::core::ffi::c_int {
        (self
            .FPDFPath_CountSegments
            .as_ref()
            .expect("Expected function, got error."))(path)
    }
    pub unsafe fn FPDFPath_GetPathSegment(
        &self,
        path: FPDF_PAGEOBJECT,
        index: ::core::ffi::c_int,
    ) -> FPDF_PATHSEGMENT {
        (self
            .FPDFPath_GetPathSegment
            .as_ref()
            .expect("Expected function, got error."))(path, index)
    }
    pub unsafe fn FPDFPathSegment_GetPoint(
        &self,
        segment: FPDF_PATHSEGMENT,
        x: *mut f32,
        y: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFPathSegment_GetPoint
            .as_ref()
            .expect("Expected function, got error."))(segment, x, y)
    }
    pub unsafe fn FPDFPathSegment_GetType(&self, segment: FPDF_PATHSEGMENT) -> ::core::ffi::c_int {
        (self
            .FPDFPathSegment_GetType
            .as_ref()
            .expect("Expected function, got error."))(segment)
    }
    pub unsafe fn FPDFPathSegment_GetClose(&self, segment: FPDF_PATHSEGMENT) -> FPDF_BOOL {
        (self
            .FPDFPathSegment_GetClose
            .as_ref()
            .expect("Expected function, got error."))(segment)
    }
    pub unsafe fn FPDFPath_MoveTo(&self, path: FPDF_PAGEOBJECT, x: f32, y: f32) -> FPDF_BOOL {
        (self
            .FPDFPath_MoveTo
            .as_ref()
            .expect("Expected function, got error."))(path, x, y)
    }
    pub unsafe fn FPDFPath_LineTo(&self, path: FPDF_PAGEOBJECT, x: f32, y: f32) -> FPDF_BOOL {
        (self
            .FPDFPath_LineTo
            .as_ref()
            .expect("Expected function, got error."))(path, x, y)
    }
    pub unsafe fn FPDFPath_BezierTo(
        &self,
        path: FPDF_PAGEOBJECT,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFPath_BezierTo
            .as_ref()
            .expect("Expected function, got error."))(path, x1, y1, x2, y2, x3, y3)
    }
    pub unsafe fn FPDFPath_GetBezierControlPoints(
        &self,
        path: FPDF_PAGEOBJECT,
        index: usize,
        first_control_point: *mut FS_POINTF,
        second_control_point: *mut FS_POINTF,
    ) -> FPDF_BOOL {
        (self
            .FPDFPath_GetBezierControlPoints
            .as_ref()
            .expect("Expected function, got error."))(
            path,
            index,
            first_control_point,
            second_control_point,
        )
    }
    pub unsafe fn FPDFPath_Close(&self, path: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        (self
            .FPDFPath_Close
            .as_ref()
            .expect("Expected function, got error."))(path)
    }
    pub unsafe fn FPDFPath_SetDrawMode(
        &self,
        path: FPDF_PAGEOBJECT,
        fillmode: ::core::ffi::c_int,
        stroke: FPDF_BOOL,
    ) -> FPDF_BOOL {
        (self
            .FPDFPath_SetDrawMode
            .as_ref()
            .expect("Expected function, got error."))(path, fillmode, stroke)
    }
    pub unsafe fn FPDFPath_GetDrawMode(
        &self,
        path: FPDF_PAGEOBJECT,
        fillmode: *mut ::core::ffi::c_int,
        stroke: *mut FPDF_BOOL,
    ) -> FPDF_BOOL {
        (self
            .FPDFPath_GetDrawMode
            .as_ref()
            .expect("Expected function, got error."))(path, fillmode, stroke)
    }
    pub unsafe fn FPDFPageObj_NewTextObj(
        &self,
        document: FPDF_DOCUMENT,
        font: FPDF_BYTESTRING,
        font_size: f32,
    ) -> FPDF_PAGEOBJECT {
        (self
            .FPDFPageObj_NewTextObj
            .as_ref()
            .expect("Expected function, got error."))(document, font, font_size)
    }
    pub unsafe fn FPDFText_SetText(
        &self,
        text_object: FPDF_PAGEOBJECT,
        text: FPDF_WIDESTRING,
    ) -> FPDF_BOOL {
        (self
            .FPDFText_SetText
            .as_ref()
            .expect("Expected function, got error."))(text_object, text)
    }
    pub unsafe fn FPDFText_SetCharcodes(
        &self,
        text_object: FPDF_PAGEOBJECT,
        charcodes: *const u32,
        count: usize,
    ) -> FPDF_BOOL {
        (self
            .FPDFText_SetCharcodes
            .as_ref()
            .expect("Expected function, got error."))(text_object, charcodes, count)
    }
    pub unsafe fn FPDFText_SetPositions(
        &self,
        text_object: FPDF_PAGEOBJECT,
        positions: *const f32,
        count: usize,
    ) -> FPDF_BOOL {
        (self
            .FPDFText_SetPositions
            .as_ref()
            .expect("Expected function, got error."))(text_object, positions, count)
    }
    pub unsafe fn FPDFText_LoadFont(
        &self,
        document: FPDF_DOCUMENT,
        data: *const u8,
        size: u32,
        font_type: ::core::ffi::c_int,
        cid: FPDF_BOOL,
    ) -> FPDF_FONT {
        (self
            .FPDFText_LoadFont
            .as_ref()
            .expect("Expected function, got error."))(document, data, size, font_type, cid)
    }
    pub unsafe fn FPDFText_LoadStandardFont(
        &self,
        document: FPDF_DOCUMENT,
        font: FPDF_BYTESTRING,
    ) -> FPDF_FONT {
        (self
            .FPDFText_LoadStandardFont
            .as_ref()
            .expect("Expected function, got error."))(document, font)
    }
    pub unsafe fn FPDFText_LoadCidType2Font(
        &self,
        document: FPDF_DOCUMENT,
        font_data: *const u8,
        font_data_size: u32,
        to_unicode_cmap: FPDF_BYTESTRING,
        cid_to_gid_map_data: *const u8,
        cid_to_gid_map_data_size: u32,
    ) -> FPDF_FONT {
        (self
            .FPDFText_LoadCidType2Font
            .as_ref()
            .expect("Expected function, got error."))(
            document,
            font_data,
            font_data_size,
            to_unicode_cmap,
            cid_to_gid_map_data,
            cid_to_gid_map_data_size,
        )
    }
    pub unsafe fn FPDFTextObj_GetFontSize(
        &self,
        text: FPDF_PAGEOBJECT,
        size: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFTextObj_GetFontSize
            .as_ref()
            .expect("Expected function, got error."))(text, size)
    }
    pub unsafe fn FPDFTextObj_SetFontSize(&self, text: FPDF_PAGEOBJECT, size: f32) -> FPDF_BOOL {
        (self
            .FPDFTextObj_SetFontSize
            .as_ref()
            .expect("Expected function, got error."))(text, size)
    }
    pub unsafe fn FPDFFont_Close(&self, font: FPDF_FONT) {
        (self
            .FPDFFont_Close
            .as_ref()
            .expect("Expected function, got error."))(font)
    }
    pub unsafe fn FPDFPageObj_CreateTextObj(
        &self,
        document: FPDF_DOCUMENT,
        font: FPDF_FONT,
        font_size: f32,
    ) -> FPDF_PAGEOBJECT {
        (self
            .FPDFPageObj_CreateTextObj
            .as_ref()
            .expect("Expected function, got error."))(document, font, font_size)
    }
    pub unsafe fn FPDFTextObj_GetTextRenderMode(
        &self,
        text: FPDF_PAGEOBJECT,
    ) -> FPDF_TEXT_RENDERMODE {
        (self
            .FPDFTextObj_GetTextRenderMode
            .as_ref()
            .expect("Expected function, got error."))(text)
    }
    pub unsafe fn FPDFTextObj_SetTextRenderMode(
        &self,
        text: FPDF_PAGEOBJECT,
        render_mode: FPDF_TEXT_RENDERMODE,
    ) -> FPDF_BOOL {
        (self
            .FPDFTextObj_SetTextRenderMode
            .as_ref()
            .expect("Expected function, got error."))(text, render_mode)
    }
    pub unsafe fn FPDFTextObj_GetText(
        &self,
        text_object: FPDF_PAGEOBJECT,
        text_page: FPDF_TEXTPAGE,
        buffer: *mut FPDF_WCHAR,
        length: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFTextObj_GetText
            .as_ref()
            .expect("Expected function, got error."))(text_object, text_page, buffer, length)
    }
    pub unsafe fn FPDFTextObj_GetRenderedBitmap(
        &self,
        document: FPDF_DOCUMENT,
        page: FPDF_PAGE,
        text_object: FPDF_PAGEOBJECT,
        scale: f32,
    ) -> FPDF_BITMAP {
        (self
            .FPDFTextObj_GetRenderedBitmap
            .as_ref()
            .expect("Expected function, got error."))(document, page, text_object, scale)
    }
    pub unsafe fn FPDFTextObj_GetFont(&self, text: FPDF_PAGEOBJECT) -> FPDF_FONT {
        (self
            .FPDFTextObj_GetFont
            .as_ref()
            .expect("Expected function, got error."))(text)
    }
    pub unsafe fn FPDFFont_GetBaseFontName(
        &self,
        font: FPDF_FONT,
        buffer: *mut ::core::ffi::c_char,
        length: usize,
    ) -> usize {
        (self
            .FPDFFont_GetBaseFontName
            .as_ref()
            .expect("Expected function, got error."))(font, buffer, length)
    }
    pub unsafe fn FPDFFont_GetFamilyName(
        &self,
        font: FPDF_FONT,
        buffer: *mut ::core::ffi::c_char,
        length: usize,
    ) -> usize {
        (self
            .FPDFFont_GetFamilyName
            .as_ref()
            .expect("Expected function, got error."))(font, buffer, length)
    }
    pub unsafe fn FPDFFont_GetFontData(
        &self,
        font: FPDF_FONT,
        buffer: *mut u8,
        buflen: usize,
        out_buflen: *mut usize,
    ) -> FPDF_BOOL {
        (self
            .FPDFFont_GetFontData
            .as_ref()
            .expect("Expected function, got error."))(font, buffer, buflen, out_buflen)
    }
    pub unsafe fn FPDFFont_GetIsEmbedded(&self, font: FPDF_FONT) -> ::core::ffi::c_int {
        (self
            .FPDFFont_GetIsEmbedded
            .as_ref()
            .expect("Expected function, got error."))(font)
    }
    pub unsafe fn FPDFFont_GetFlags(&self, font: FPDF_FONT) -> ::core::ffi::c_int {
        (self
            .FPDFFont_GetFlags
            .as_ref()
            .expect("Expected function, got error."))(font)
    }
    pub unsafe fn FPDFFont_GetWeight(&self, font: FPDF_FONT) -> ::core::ffi::c_int {
        (self
            .FPDFFont_GetWeight
            .as_ref()
            .expect("Expected function, got error."))(font)
    }
    pub unsafe fn FPDFFont_GetItalicAngle(
        &self,
        font: FPDF_FONT,
        angle: *mut ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FPDFFont_GetItalicAngle
            .as_ref()
            .expect("Expected function, got error."))(font, angle)
    }
    pub unsafe fn FPDFFont_GetAscent(
        &self,
        font: FPDF_FONT,
        font_size: f32,
        ascent: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFFont_GetAscent
            .as_ref()
            .expect("Expected function, got error."))(font, font_size, ascent)
    }
    pub unsafe fn FPDFFont_GetDescent(
        &self,
        font: FPDF_FONT,
        font_size: f32,
        descent: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFFont_GetDescent
            .as_ref()
            .expect("Expected function, got error."))(font, font_size, descent)
    }
    pub unsafe fn FPDFFont_GetGlyphWidth(
        &self,
        font: FPDF_FONT,
        glyph: u32,
        font_size: f32,
        width: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFFont_GetGlyphWidth
            .as_ref()
            .expect("Expected function, got error."))(font, glyph, font_size, width)
    }
    pub unsafe fn FPDFFont_GetGlyphPath(
        &self,
        font: FPDF_FONT,
        glyph: u32,
        font_size: f32,
    ) -> FPDF_GLYPHPATH {
        (self
            .FPDFFont_GetGlyphPath
            .as_ref()
            .expect("Expected function, got error."))(font, glyph, font_size)
    }
    pub unsafe fn FPDFGlyphPath_CountGlyphSegments(
        &self,
        glyphpath: FPDF_GLYPHPATH,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFGlyphPath_CountGlyphSegments
            .as_ref()
            .expect("Expected function, got error."))(glyphpath)
    }
    pub unsafe fn FPDFGlyphPath_GetGlyphPathSegment(
        &self,
        glyphpath: FPDF_GLYPHPATH,
        index: ::core::ffi::c_int,
    ) -> FPDF_PATHSEGMENT {
        (self
            .FPDFGlyphPath_GetGlyphPathSegment
            .as_ref()
            .expect("Expected function, got error."))(glyphpath, index)
    }
    pub unsafe fn FPDFFormObj_CountObjects(
        &self,
        form_object: FPDF_PAGEOBJECT,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFFormObj_CountObjects
            .as_ref()
            .expect("Expected function, got error."))(form_object)
    }
    pub unsafe fn FPDFFormObj_GetObject(
        &self,
        form_object: FPDF_PAGEOBJECT,
        index: ::core::ffi::c_ulong,
    ) -> FPDF_PAGEOBJECT {
        (self
            .FPDFFormObj_GetObject
            .as_ref()
            .expect("Expected function, got error."))(form_object, index)
    }
    pub unsafe fn FPDFFormObj_RemoveObject(
        &self,
        form_object: FPDF_PAGEOBJECT,
        page_object: FPDF_PAGEOBJECT,
    ) -> FPDF_BOOL {
        (self
            .FPDFFormObj_RemoveObject
            .as_ref()
            .expect("Expected function, got error."))(form_object, page_object)
    }
    pub unsafe fn FSDK_SetUnSpObjProcessHandler(
        &self,
        unsp_info: *mut UNSUPPORT_INFO,
    ) -> FPDF_BOOL {
        (self
            .FSDK_SetUnSpObjProcessHandler
            .as_ref()
            .expect("Expected function, got error."))(unsp_info)
    }
    pub unsafe fn FSDK_SetTimeFunction(
        &self,
        func: ::core::option::Option<unsafe extern "C" fn() -> time_t>,
    ) {
        (self
            .FSDK_SetTimeFunction
            .as_ref()
            .expect("Expected function, got error."))(func)
    }
    pub unsafe fn FSDK_SetLocaltimeFunction(
        &self,
        func: ::core::option::Option<unsafe extern "C" fn(arg1: *const time_t) -> *mut tm>,
    ) {
        (self
            .FSDK_SetLocaltimeFunction
            .as_ref()
            .expect("Expected function, got error."))(func)
    }
    pub unsafe fn FPDFDoc_GetPageMode(&self, document: FPDF_DOCUMENT) -> ::core::ffi::c_int {
        (self
            .FPDFDoc_GetPageMode
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDFPage_Flatten(
        &self,
        page: FPDF_PAGE,
        nFlag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFPage_Flatten
            .as_ref()
            .expect("Expected function, got error."))(page, nFlag)
    }
    pub unsafe fn FPDFDoc_GetJavaScriptActionCount(
        &self,
        document: FPDF_DOCUMENT,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFDoc_GetJavaScriptActionCount
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDFDoc_GetJavaScriptAction(
        &self,
        document: FPDF_DOCUMENT,
        index: ::core::ffi::c_int,
    ) -> FPDF_JAVASCRIPT_ACTION {
        (self
            .FPDFDoc_GetJavaScriptAction
            .as_ref()
            .expect("Expected function, got error."))(document, index)
    }
    pub unsafe fn FPDFDoc_CloseJavaScriptAction(&self, javascript: FPDF_JAVASCRIPT_ACTION) {
        (self
            .FPDFDoc_CloseJavaScriptAction
            .as_ref()
            .expect("Expected function, got error."))(javascript)
    }
    pub unsafe fn FPDFJavaScriptAction_GetName(
        &self,
        javascript: FPDF_JAVASCRIPT_ACTION,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFJavaScriptAction_GetName
            .as_ref()
            .expect("Expected function, got error."))(javascript, buffer, buflen)
    }
    pub unsafe fn FPDFJavaScriptAction_GetScript(
        &self,
        javascript: FPDF_JAVASCRIPT_ACTION,
        buffer: *mut FPDF_WCHAR,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFJavaScriptAction_GetScript
            .as_ref()
            .expect("Expected function, got error."))(javascript, buffer, buflen)
    }
    pub unsafe fn FPDF_ImportPagesByIndex(
        &self,
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        page_indices: *const ::core::ffi::c_int,
        length: ::core::ffi::c_ulong,
        index: ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FPDF_ImportPagesByIndex
            .as_ref()
            .expect("Expected function, got error."))(
            dest_doc,
            src_doc,
            page_indices,
            length,
            index,
        )
    }
    pub unsafe fn FPDF_ImportPages(
        &self,
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        pagerange: FPDF_BYTESTRING,
        index: ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FPDF_ImportPages
            .as_ref()
            .expect("Expected function, got error."))(dest_doc, src_doc, pagerange, index)
    }
    pub unsafe fn FPDF_ImportNPagesToOne(
        &self,
        src_doc: FPDF_DOCUMENT,
        output_width: f32,
        output_height: f32,
        num_pages_on_x_axis: usize,
        num_pages_on_y_axis: usize,
    ) -> FPDF_DOCUMENT {
        (self
            .FPDF_ImportNPagesToOne
            .as_ref()
            .expect("Expected function, got error."))(
            src_doc,
            output_width,
            output_height,
            num_pages_on_x_axis,
            num_pages_on_y_axis,
        )
    }
    pub unsafe fn FPDF_NewXObjectFromPage(
        &self,
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        src_page_index: ::core::ffi::c_int,
    ) -> FPDF_XOBJECT {
        (self
            .FPDF_NewXObjectFromPage
            .as_ref()
            .expect("Expected function, got error."))(dest_doc, src_doc, src_page_index)
    }
    pub unsafe fn FPDF_CloseXObject(&self, xobject: FPDF_XOBJECT) {
        (self
            .FPDF_CloseXObject
            .as_ref()
            .expect("Expected function, got error."))(xobject)
    }
    pub unsafe fn FPDF_NewFormObjectFromXObject(&self, xobject: FPDF_XOBJECT) -> FPDF_PAGEOBJECT {
        (self
            .FPDF_NewFormObjectFromXObject
            .as_ref()
            .expect("Expected function, got error."))(xobject)
    }
    pub unsafe fn FPDF_CopyViewerPreferences(
        &self,
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
    ) -> FPDF_BOOL {
        (self
            .FPDF_CopyViewerPreferences
            .as_ref()
            .expect("Expected function, got error."))(dest_doc, src_doc)
    }
    pub unsafe fn FPDF_RenderPageBitmapWithColorScheme_Start(
        &self,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: ::core::ffi::c_int,
        start_y: ::core::ffi::c_int,
        size_x: ::core::ffi::c_int,
        size_y: ::core::ffi::c_int,
        rotate: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
        color_scheme: *const FPDF_COLORSCHEME,
        pause: *mut IFSDK_PAUSE,
    ) -> ::core::ffi::c_int {
        (self
            .FPDF_RenderPageBitmapWithColorScheme_Start
            .as_ref()
            .expect("Expected function, got error."))(
            bitmap,
            page,
            start_x,
            start_y,
            size_x,
            size_y,
            rotate,
            flags,
            color_scheme,
            pause,
        )
    }
    pub unsafe fn FPDF_RenderPageBitmap_Start(
        &self,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: ::core::ffi::c_int,
        start_y: ::core::ffi::c_int,
        size_x: ::core::ffi::c_int,
        size_y: ::core::ffi::c_int,
        rotate: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
        pause: *mut IFSDK_PAUSE,
    ) -> ::core::ffi::c_int {
        (self
            .FPDF_RenderPageBitmap_Start
            .as_ref()
            .expect("Expected function, got error."))(
            bitmap, page, start_x, start_y, size_x, size_y, rotate, flags, pause,
        )
    }
    pub unsafe fn FPDF_RenderPage_Continue(
        &self,
        page: FPDF_PAGE,
        pause: *mut IFSDK_PAUSE,
    ) -> ::core::ffi::c_int {
        (self
            .FPDF_RenderPage_Continue
            .as_ref()
            .expect("Expected function, got error."))(page, pause)
    }
    pub unsafe fn FPDF_RenderPage_Close(&self, page: FPDF_PAGE) {
        (self
            .FPDF_RenderPage_Close
            .as_ref()
            .expect("Expected function, got error."))(page)
    }
    pub unsafe fn FPDF_SaveAsCopy(
        &self,
        document: FPDF_DOCUMENT,
        file_write: *mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
    ) -> FPDF_BOOL {
        (self
            .FPDF_SaveAsCopy
            .as_ref()
            .expect("Expected function, got error."))(document, file_write, flags)
    }
    pub unsafe fn FPDF_SaveWithVersion(
        &self,
        document: FPDF_DOCUMENT,
        file_write: *mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
        file_version: ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FPDF_SaveWithVersion
            .as_ref()
            .expect("Expected function, got error."))(
            document, file_write, flags, file_version
        )
    }
    pub unsafe fn FPDFText_GetCharIndexFromTextIndex(
        &self,
        text_page: FPDF_TEXTPAGE,
        nTextIndex: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFText_GetCharIndexFromTextIndex
            .as_ref()
            .expect("Expected function, got error."))(text_page, nTextIndex)
    }
    pub unsafe fn FPDFText_GetTextIndexFromCharIndex(
        &self,
        text_page: FPDF_TEXTPAGE,
        nCharIndex: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFText_GetTextIndexFromCharIndex
            .as_ref()
            .expect("Expected function, got error."))(text_page, nCharIndex)
    }
    pub unsafe fn FPDF_GetSignatureCount(&self, document: FPDF_DOCUMENT) -> ::core::ffi::c_int {
        (self
            .FPDF_GetSignatureCount
            .as_ref()
            .expect("Expected function, got error."))(document)
    }
    pub unsafe fn FPDF_GetSignatureObject(
        &self,
        document: FPDF_DOCUMENT,
        index: ::core::ffi::c_int,
    ) -> FPDF_SIGNATURE {
        (self
            .FPDF_GetSignatureObject
            .as_ref()
            .expect("Expected function, got error."))(document, index)
    }
    pub unsafe fn FPDFSignatureObj_GetContents(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut ::core::ffi::c_void,
        length: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFSignatureObj_GetContents
            .as_ref()
            .expect("Expected function, got error."))(signature, buffer, length)
    }
    pub unsafe fn FPDFSignatureObj_GetByteRange(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut ::core::ffi::c_int,
        length: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFSignatureObj_GetByteRange
            .as_ref()
            .expect("Expected function, got error."))(signature, buffer, length)
    }
    pub unsafe fn FPDFSignatureObj_GetSubFilter(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFSignatureObj_GetSubFilter
            .as_ref()
            .expect("Expected function, got error."))(signature, buffer, length)
    }
    pub unsafe fn FPDFSignatureObj_GetReason(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut ::core::ffi::c_void,
        length: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFSignatureObj_GetReason
            .as_ref()
            .expect("Expected function, got error."))(signature, buffer, length)
    }
    pub unsafe fn FPDFSignatureObj_GetTime(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFSignatureObj_GetTime
            .as_ref()
            .expect("Expected function, got error."))(signature, buffer, length)
    }
    pub unsafe fn FPDFSignatureObj_GetDocMDPPermission(
        &self,
        signature: FPDF_SIGNATURE,
    ) -> ::core::ffi::c_uint {
        (self
            .FPDFSignatureObj_GetDocMDPPermission
            .as_ref()
            .expect("Expected function, got error."))(signature)
    }
    pub unsafe fn FPDF_StructTree_GetForPage(&self, page: FPDF_PAGE) -> FPDF_STRUCTTREE {
        (self
            .FPDF_StructTree_GetForPage
            .as_ref()
            .expect("Expected function, got error."))(page)
    }
    pub unsafe fn FPDF_StructTree_Close(&self, struct_tree: FPDF_STRUCTTREE) {
        (self
            .FPDF_StructTree_Close
            .as_ref()
            .expect("Expected function, got error."))(struct_tree)
    }
    pub unsafe fn FPDF_StructTree_CountChildren(
        &self,
        struct_tree: FPDF_STRUCTTREE,
    ) -> ::core::ffi::c_int {
        (self
            .FPDF_StructTree_CountChildren
            .as_ref()
            .expect("Expected function, got error."))(struct_tree)
    }
    pub unsafe fn FPDF_StructTree_GetChildAtIndex(
        &self,
        struct_tree: FPDF_STRUCTTREE,
        index: ::core::ffi::c_int,
    ) -> FPDF_STRUCTELEMENT {
        (self
            .FPDF_StructTree_GetChildAtIndex
            .as_ref()
            .expect("Expected function, got error."))(struct_tree, index)
    }
    pub unsafe fn FPDF_StructElement_GetAltText(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDF_StructElement_GetAltText
            .as_ref()
            .expect("Expected function, got error."))(struct_element, buffer, buflen)
    }
    pub unsafe fn FPDF_StructElement_GetActualText(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDF_StructElement_GetActualText
            .as_ref()
            .expect("Expected function, got error."))(struct_element, buffer, buflen)
    }
    pub unsafe fn FPDF_StructElement_GetExpansion(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDF_StructElement_GetExpansion
            .as_ref()
            .expect("Expected function, got error."))(struct_element, buffer, buflen)
    }
    pub unsafe fn FPDF_StructElement_GetID(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDF_StructElement_GetID
            .as_ref()
            .expect("Expected function, got error."))(struct_element, buffer, buflen)
    }
    pub unsafe fn FPDF_StructElement_GetLang(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDF_StructElement_GetLang
            .as_ref()
            .expect("Expected function, got error."))(struct_element, buffer, buflen)
    }
    pub unsafe fn FPDF_StructElement_GetStringAttribute(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        attr_name: FPDF_BYTESTRING,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDF_StructElement_GetStringAttribute
            .as_ref()
            .expect("Expected function, got error."))(
            struct_element, attr_name, buffer, buflen
        )
    }
    pub unsafe fn FPDF_StructElement_GetMarkedContentID(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
    ) -> ::core::ffi::c_int {
        (self
            .FPDF_StructElement_GetMarkedContentID
            .as_ref()
            .expect("Expected function, got error."))(struct_element)
    }
    pub unsafe fn FPDF_StructElement_GetRawType(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut FPDF_WCHAR,
        buflen: usize,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDF_StructElement_GetRawType
            .as_ref()
            .expect("Expected function, got error."))(struct_element, buffer, buflen)
    }
    pub unsafe fn FPDF_StructElement_GetType(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDF_StructElement_GetType
            .as_ref()
            .expect("Expected function, got error."))(struct_element, buffer, buflen)
    }
    pub unsafe fn FPDF_StructElement_GetObjType(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDF_StructElement_GetObjType
            .as_ref()
            .expect("Expected function, got error."))(struct_element, buffer, buflen)
    }
    pub unsafe fn FPDF_StructElement_GetTitle(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDF_StructElement_GetTitle
            .as_ref()
            .expect("Expected function, got error."))(struct_element, buffer, buflen)
    }
    pub unsafe fn FPDF_StructElement_CountChildren(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
    ) -> ::core::ffi::c_int {
        (self
            .FPDF_StructElement_CountChildren
            .as_ref()
            .expect("Expected function, got error."))(struct_element)
    }
    pub unsafe fn FPDF_StructElement_GetChildAtIndex(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        index: ::core::ffi::c_int,
    ) -> FPDF_STRUCTELEMENT {
        (self
            .FPDF_StructElement_GetChildAtIndex
            .as_ref()
            .expect("Expected function, got error."))(struct_element, index)
    }
    pub unsafe fn FPDF_StructElement_GetChildMarkedContentID(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        (self
            .FPDF_StructElement_GetChildMarkedContentID
            .as_ref()
            .expect("Expected function, got error."))(struct_element, index)
    }
    pub unsafe fn FPDF_StructElement_GetParent(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
    ) -> FPDF_STRUCTELEMENT {
        (self
            .FPDF_StructElement_GetParent
            .as_ref()
            .expect("Expected function, got error."))(struct_element)
    }
    pub unsafe fn FPDF_StructElement_GetAttributeCount(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
    ) -> ::core::ffi::c_int {
        (self
            .FPDF_StructElement_GetAttributeCount
            .as_ref()
            .expect("Expected function, got error."))(struct_element)
    }
    pub unsafe fn FPDF_StructElement_GetAttributeAtIndex(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        index: ::core::ffi::c_int,
    ) -> FPDF_STRUCTELEMENT_ATTR {
        (self
            .FPDF_StructElement_GetAttributeAtIndex
            .as_ref()
            .expect("Expected function, got error."))(struct_element, index)
    }
    pub unsafe fn FPDF_StructElement_Attr_GetCount(
        &self,
        struct_attribute: FPDF_STRUCTELEMENT_ATTR,
    ) -> ::core::ffi::c_int {
        (self
            .FPDF_StructElement_Attr_GetCount
            .as_ref()
            .expect("Expected function, got error."))(struct_attribute)
    }
    pub unsafe fn FPDF_StructElement_Attr_GetName(
        &self,
        struct_attribute: FPDF_STRUCTELEMENT_ATTR,
        index: ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
        out_buflen: *mut ::core::ffi::c_ulong,
    ) -> FPDF_BOOL {
        (self
            .FPDF_StructElement_Attr_GetName
            .as_ref()
            .expect("Expected function, got error."))(
            struct_attribute,
            index,
            buffer,
            buflen,
            out_buflen,
        )
    }
    pub unsafe fn FPDF_StructElement_Attr_GetValue(
        &self,
        struct_attribute: FPDF_STRUCTELEMENT_ATTR,
        name: FPDF_BYTESTRING,
    ) -> FPDF_STRUCTELEMENT_ATTR_VALUE {
        (self
            .FPDF_StructElement_Attr_GetValue
            .as_ref()
            .expect("Expected function, got error."))(struct_attribute, name)
    }
    pub unsafe fn FPDF_StructElement_Attr_GetType(
        &self,
        value: FPDF_STRUCTELEMENT_ATTR_VALUE,
    ) -> FPDF_OBJECT_TYPE {
        (self
            .FPDF_StructElement_Attr_GetType
            .as_ref()
            .expect("Expected function, got error."))(value)
    }
    pub unsafe fn FPDF_StructElement_Attr_GetBooleanValue(
        &self,
        value: FPDF_STRUCTELEMENT_ATTR_VALUE,
        out_value: *mut FPDF_BOOL,
    ) -> FPDF_BOOL {
        (self
            .FPDF_StructElement_Attr_GetBooleanValue
            .as_ref()
            .expect("Expected function, got error."))(value, out_value)
    }
    pub unsafe fn FPDF_StructElement_Attr_GetNumberValue(
        &self,
        value: FPDF_STRUCTELEMENT_ATTR_VALUE,
        out_value: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDF_StructElement_Attr_GetNumberValue
            .as_ref()
            .expect("Expected function, got error."))(value, out_value)
    }
    pub unsafe fn FPDF_StructElement_Attr_GetStringValue(
        &self,
        value: FPDF_STRUCTELEMENT_ATTR_VALUE,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
        out_buflen: *mut ::core::ffi::c_ulong,
    ) -> FPDF_BOOL {
        (self
            .FPDF_StructElement_Attr_GetStringValue
            .as_ref()
            .expect("Expected function, got error."))(value, buffer, buflen, out_buflen)
    }
    pub unsafe fn FPDF_StructElement_Attr_GetBlobValue(
        &self,
        value: FPDF_STRUCTELEMENT_ATTR_VALUE,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
        out_buflen: *mut ::core::ffi::c_ulong,
    ) -> FPDF_BOOL {
        (self
            .FPDF_StructElement_Attr_GetBlobValue
            .as_ref()
            .expect("Expected function, got error."))(value, buffer, buflen, out_buflen)
    }
    pub unsafe fn FPDF_StructElement_Attr_CountChildren(
        &self,
        value: FPDF_STRUCTELEMENT_ATTR_VALUE,
    ) -> ::core::ffi::c_int {
        (self
            .FPDF_StructElement_Attr_CountChildren
            .as_ref()
            .expect("Expected function, got error."))(value)
    }
    pub unsafe fn FPDF_StructElement_Attr_GetChildAtIndex(
        &self,
        value: FPDF_STRUCTELEMENT_ATTR_VALUE,
        index: ::core::ffi::c_int,
    ) -> FPDF_STRUCTELEMENT_ATTR_VALUE {
        (self
            .FPDF_StructElement_Attr_GetChildAtIndex
            .as_ref()
            .expect("Expected function, got error."))(value, index)
    }
    pub unsafe fn FPDF_StructElement_GetMarkedContentIdCount(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
    ) -> ::core::ffi::c_int {
        (self
            .FPDF_StructElement_GetMarkedContentIdCount
            .as_ref()
            .expect("Expected function, got error."))(struct_element)
    }
    pub unsafe fn FPDF_StructElement_GetMarkedContentIdAtIndex(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        (self
            .FPDF_StructElement_GetMarkedContentIdAtIndex
            .as_ref()
            .expect("Expected function, got error."))(struct_element, index)
    }
    pub unsafe fn FPDF_GetDefaultTTFMap(&self) -> *const FPDF_CharsetFontMap {
        (self
            .FPDF_GetDefaultTTFMap
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn FPDF_GetDefaultTTFMapCount(&self) -> usize {
        (self
            .FPDF_GetDefaultTTFMapCount
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn FPDF_GetDefaultTTFMapEntry(&self, index: usize) -> *const FPDF_CharsetFontMap {
        (self
            .FPDF_GetDefaultTTFMapEntry
            .as_ref()
            .expect("Expected function, got error."))(index)
    }
    pub unsafe fn FPDF_AddInstalledFont(
        &self,
        mapper: *mut ::core::ffi::c_void,
        face: *const ::core::ffi::c_char,
        charset: ::core::ffi::c_int,
    ) {
        (self
            .FPDF_AddInstalledFont
            .as_ref()
            .expect("Expected function, got error."))(mapper, face, charset)
    }
    pub unsafe fn FPDF_SetSystemFontInfo(&self, font_info: *mut FPDF_SYSFONTINFO) {
        (self
            .FPDF_SetSystemFontInfo
            .as_ref()
            .expect("Expected function, got error."))(font_info)
    }
    pub unsafe fn FPDF_GetDefaultSystemFontInfo(&self) -> *mut FPDF_SYSFONTINFO {
        (self
            .FPDF_GetDefaultSystemFontInfo
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn FPDF_FreeDefaultSystemFontInfo(&self, font_info: *mut FPDF_SYSFONTINFO) {
        (self
            .FPDF_FreeDefaultSystemFontInfo
            .as_ref()
            .expect("Expected function, got error."))(font_info)
    }
    pub unsafe fn FPDFText_LoadPage(&self, page: FPDF_PAGE) -> FPDF_TEXTPAGE {
        (self
            .FPDFText_LoadPage
            .as_ref()
            .expect("Expected function, got error."))(page)
    }
    pub unsafe fn FPDFText_ClosePage(&self, text_page: FPDF_TEXTPAGE) {
        (self
            .FPDFText_ClosePage
            .as_ref()
            .expect("Expected function, got error."))(text_page)
    }
    pub unsafe fn FPDFText_CountChars(&self, text_page: FPDF_TEXTPAGE) -> ::core::ffi::c_int {
        (self
            .FPDFText_CountChars
            .as_ref()
            .expect("Expected function, got error."))(text_page)
    }
    pub unsafe fn FPDFText_GetUnicode(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uint {
        (self
            .FPDFText_GetUnicode
            .as_ref()
            .expect("Expected function, got error."))(text_page, index)
    }
    pub unsafe fn FPDFText_GetTextObject(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: ::core::ffi::c_int,
    ) -> FPDF_PAGEOBJECT {
        (self
            .FPDFText_GetTextObject
            .as_ref()
            .expect("Expected function, got error."))(text_page, index)
    }
    pub unsafe fn FPDFText_IsGenerated(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFText_IsGenerated
            .as_ref()
            .expect("Expected function, got error."))(text_page, index)
    }
    pub unsafe fn FPDFText_IsHyphen(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFText_IsHyphen
            .as_ref()
            .expect("Expected function, got error."))(text_page, index)
    }
    pub unsafe fn FPDFText_HasUnicodeMapError(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFText_HasUnicodeMapError
            .as_ref()
            .expect("Expected function, got error."))(text_page, index)
    }
    pub unsafe fn FPDFText_GetFontSize(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: ::core::ffi::c_int,
    ) -> f64 {
        (self
            .FPDFText_GetFontSize
            .as_ref()
            .expect("Expected function, got error."))(text_page, index)
    }
    pub unsafe fn FPDFText_GetFontInfo(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
        flags: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFText_GetFontInfo
            .as_ref()
            .expect("Expected function, got error."))(
            text_page, index, buffer, buflen, flags
        )
    }
    pub unsafe fn FPDFText_GetFontWeight(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFText_GetFontWeight
            .as_ref()
            .expect("Expected function, got error."))(text_page, index)
    }
    pub unsafe fn FPDFText_GetFillColor(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: ::core::ffi::c_int,
        R: *mut ::core::ffi::c_uint,
        G: *mut ::core::ffi::c_uint,
        B: *mut ::core::ffi::c_uint,
        A: *mut ::core::ffi::c_uint,
    ) -> FPDF_BOOL {
        (self
            .FPDFText_GetFillColor
            .as_ref()
            .expect("Expected function, got error."))(text_page, index, R, G, B, A)
    }
    pub unsafe fn FPDFText_GetStrokeColor(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: ::core::ffi::c_int,
        R: *mut ::core::ffi::c_uint,
        G: *mut ::core::ffi::c_uint,
        B: *mut ::core::ffi::c_uint,
        A: *mut ::core::ffi::c_uint,
    ) -> FPDF_BOOL {
        (self
            .FPDFText_GetStrokeColor
            .as_ref()
            .expect("Expected function, got error."))(text_page, index, R, G, B, A)
    }
    pub unsafe fn FPDFText_GetCharAngle(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: ::core::ffi::c_int,
    ) -> f32 {
        (self
            .FPDFText_GetCharAngle
            .as_ref()
            .expect("Expected function, got error."))(text_page, index)
    }
    pub unsafe fn FPDFText_GetCharBox(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: ::core::ffi::c_int,
        left: *mut f64,
        right: *mut f64,
        bottom: *mut f64,
        top: *mut f64,
    ) -> FPDF_BOOL {
        (self
            .FPDFText_GetCharBox
            .as_ref()
            .expect("Expected function, got error."))(
            text_page, index, left, right, bottom, top
        )
    }
    pub unsafe fn FPDFText_GetLooseCharBox(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: ::core::ffi::c_int,
        rect: *mut FS_RECTF,
    ) -> FPDF_BOOL {
        (self
            .FPDFText_GetLooseCharBox
            .as_ref()
            .expect("Expected function, got error."))(text_page, index, rect)
    }
    pub unsafe fn FPDFText_GetMatrix(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: ::core::ffi::c_int,
        matrix: *mut FS_MATRIX,
    ) -> FPDF_BOOL {
        (self
            .FPDFText_GetMatrix
            .as_ref()
            .expect("Expected function, got error."))(text_page, index, matrix)
    }
    pub unsafe fn FPDFText_GetCharOrigin(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: ::core::ffi::c_int,
        x: *mut f64,
        y: *mut f64,
    ) -> FPDF_BOOL {
        (self
            .FPDFText_GetCharOrigin
            .as_ref()
            .expect("Expected function, got error."))(text_page, index, x, y)
    }
    pub unsafe fn FPDFText_GetCharIndexAtPos(
        &self,
        text_page: FPDF_TEXTPAGE,
        x: f64,
        y: f64,
        xTolerance: f64,
        yTolerance: f64,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFText_GetCharIndexAtPos
            .as_ref()
            .expect("Expected function, got error."))(
            text_page, x, y, xTolerance, yTolerance
        )
    }
    pub unsafe fn FPDFText_GetText(
        &self,
        text_page: FPDF_TEXTPAGE,
        start_index: ::core::ffi::c_int,
        count: ::core::ffi::c_int,
        result: *mut ::core::ffi::c_ushort,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFText_GetText
            .as_ref()
            .expect("Expected function, got error."))(text_page, start_index, count, result)
    }
    pub unsafe fn FPDFText_CountRects(
        &self,
        text_page: FPDF_TEXTPAGE,
        start_index: ::core::ffi::c_int,
        count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFText_CountRects
            .as_ref()
            .expect("Expected function, got error."))(text_page, start_index, count)
    }
    pub unsafe fn FPDFText_GetRect(
        &self,
        text_page: FPDF_TEXTPAGE,
        rect_index: ::core::ffi::c_int,
        left: *mut f64,
        top: *mut f64,
        right: *mut f64,
        bottom: *mut f64,
    ) -> FPDF_BOOL {
        (self
            .FPDFText_GetRect
            .as_ref()
            .expect("Expected function, got error."))(
            text_page, rect_index, left, top, right, bottom,
        )
    }
    pub unsafe fn FPDFText_GetBoundedText(
        &self,
        text_page: FPDF_TEXTPAGE,
        left: f64,
        top: f64,
        right: f64,
        bottom: f64,
        buffer: *mut ::core::ffi::c_ushort,
        buflen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFText_GetBoundedText
            .as_ref()
            .expect("Expected function, got error."))(
            text_page, left, top, right, bottom, buffer, buflen,
        )
    }
    pub unsafe fn FPDFText_FindStart(
        &self,
        text_page: FPDF_TEXTPAGE,
        findwhat: FPDF_WIDESTRING,
        flags: ::core::ffi::c_ulong,
        start_index: ::core::ffi::c_int,
    ) -> FPDF_SCHHANDLE {
        (self
            .FPDFText_FindStart
            .as_ref()
            .expect("Expected function, got error."))(
            text_page, findwhat, flags, start_index
        )
    }
    pub unsafe fn FPDFText_FindNext(&self, handle: FPDF_SCHHANDLE) -> FPDF_BOOL {
        (self
            .FPDFText_FindNext
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn FPDFText_FindPrev(&self, handle: FPDF_SCHHANDLE) -> FPDF_BOOL {
        (self
            .FPDFText_FindPrev
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn FPDFText_GetSchResultIndex(&self, handle: FPDF_SCHHANDLE) -> ::core::ffi::c_int {
        (self
            .FPDFText_GetSchResultIndex
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn FPDFText_GetSchCount(&self, handle: FPDF_SCHHANDLE) -> ::core::ffi::c_int {
        (self
            .FPDFText_GetSchCount
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn FPDFText_FindClose(&self, handle: FPDF_SCHHANDLE) {
        (self
            .FPDFText_FindClose
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn FPDFLink_LoadWebLinks(&self, text_page: FPDF_TEXTPAGE) -> FPDF_PAGELINK {
        (self
            .FPDFLink_LoadWebLinks
            .as_ref()
            .expect("Expected function, got error."))(text_page)
    }
    pub unsafe fn FPDFLink_CountWebLinks(&self, link_page: FPDF_PAGELINK) -> ::core::ffi::c_int {
        (self
            .FPDFLink_CountWebLinks
            .as_ref()
            .expect("Expected function, got error."))(link_page)
    }
    pub unsafe fn FPDFLink_GetURL(
        &self,
        link_page: FPDF_PAGELINK,
        link_index: ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_ushort,
        buflen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFLink_GetURL
            .as_ref()
            .expect("Expected function, got error."))(link_page, link_index, buffer, buflen)
    }
    pub unsafe fn FPDFLink_CountRects(
        &self,
        link_page: FPDF_PAGELINK,
        link_index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFLink_CountRects
            .as_ref()
            .expect("Expected function, got error."))(link_page, link_index)
    }
    pub unsafe fn FPDFLink_GetRect(
        &self,
        link_page: FPDF_PAGELINK,
        link_index: ::core::ffi::c_int,
        rect_index: ::core::ffi::c_int,
        left: *mut f64,
        top: *mut f64,
        right: *mut f64,
        bottom: *mut f64,
    ) -> FPDF_BOOL {
        (self
            .FPDFLink_GetRect
            .as_ref()
            .expect("Expected function, got error."))(
            link_page, link_index, rect_index, left, top, right, bottom,
        )
    }
    pub unsafe fn FPDFLink_GetTextRange(
        &self,
        link_page: FPDF_PAGELINK,
        link_index: ::core::ffi::c_int,
        start_char_index: *mut ::core::ffi::c_int,
        char_count: *mut ::core::ffi::c_int,
    ) -> FPDF_BOOL {
        (self
            .FPDFLink_GetTextRange
            .as_ref()
            .expect("Expected function, got error."))(
            link_page,
            link_index,
            start_char_index,
            char_count,
        )
    }
    pub unsafe fn FPDFLink_CloseWebLinks(&self, link_page: FPDF_PAGELINK) {
        (self
            .FPDFLink_CloseWebLinks
            .as_ref()
            .expect("Expected function, got error."))(link_page)
    }
    pub unsafe fn FPDFPage_GetDecodedThumbnailData(
        &self,
        page: FPDF_PAGE,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFPage_GetDecodedThumbnailData
            .as_ref()
            .expect("Expected function, got error."))(page, buffer, buflen)
    }
    pub unsafe fn FPDFPage_GetRawThumbnailData(
        &self,
        page: FPDF_PAGE,
        buffer: *mut ::core::ffi::c_void,
        buflen: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong {
        (self
            .FPDFPage_GetRawThumbnailData
            .as_ref()
            .expect("Expected function, got error."))(page, buffer, buflen)
    }
    pub unsafe fn FPDFPage_GetThumbnailAsBitmap(&self, page: FPDF_PAGE) -> FPDF_BITMAP {
        (self
            .FPDFPage_GetThumbnailAsBitmap
            .as_ref()
            .expect("Expected function, got error."))(page)
    }
    pub unsafe fn FPDFPage_SetMediaBox(
        &self,
        page: FPDF_PAGE,
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
    ) {
        (self
            .FPDFPage_SetMediaBox
            .as_ref()
            .expect("Expected function, got error."))(page, left, bottom, right, top)
    }
    pub unsafe fn FPDFPage_SetCropBox(
        &self,
        page: FPDF_PAGE,
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
    ) {
        (self
            .FPDFPage_SetCropBox
            .as_ref()
            .expect("Expected function, got error."))(page, left, bottom, right, top)
    }
    pub unsafe fn FPDFPage_SetBleedBox(
        &self,
        page: FPDF_PAGE,
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
    ) {
        (self
            .FPDFPage_SetBleedBox
            .as_ref()
            .expect("Expected function, got error."))(page, left, bottom, right, top)
    }
    pub unsafe fn FPDFPage_SetTrimBox(
        &self,
        page: FPDF_PAGE,
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
    ) {
        (self
            .FPDFPage_SetTrimBox
            .as_ref()
            .expect("Expected function, got error."))(page, left, bottom, right, top)
    }
    pub unsafe fn FPDFPage_SetArtBox(
        &self,
        page: FPDF_PAGE,
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
    ) {
        (self
            .FPDFPage_SetArtBox
            .as_ref()
            .expect("Expected function, got error."))(page, left, bottom, right, top)
    }
    pub unsafe fn FPDFPage_GetMediaBox(
        &self,
        page: FPDF_PAGE,
        left: *mut f32,
        bottom: *mut f32,
        right: *mut f32,
        top: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFPage_GetMediaBox
            .as_ref()
            .expect("Expected function, got error."))(page, left, bottom, right, top)
    }
    pub unsafe fn FPDFPage_GetCropBox(
        &self,
        page: FPDF_PAGE,
        left: *mut f32,
        bottom: *mut f32,
        right: *mut f32,
        top: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFPage_GetCropBox
            .as_ref()
            .expect("Expected function, got error."))(page, left, bottom, right, top)
    }
    pub unsafe fn FPDFPage_GetBleedBox(
        &self,
        page: FPDF_PAGE,
        left: *mut f32,
        bottom: *mut f32,
        right: *mut f32,
        top: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFPage_GetBleedBox
            .as_ref()
            .expect("Expected function, got error."))(page, left, bottom, right, top)
    }
    pub unsafe fn FPDFPage_GetTrimBox(
        &self,
        page: FPDF_PAGE,
        left: *mut f32,
        bottom: *mut f32,
        right: *mut f32,
        top: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFPage_GetTrimBox
            .as_ref()
            .expect("Expected function, got error."))(page, left, bottom, right, top)
    }
    pub unsafe fn FPDFPage_GetArtBox(
        &self,
        page: FPDF_PAGE,
        left: *mut f32,
        bottom: *mut f32,
        right: *mut f32,
        top: *mut f32,
    ) -> FPDF_BOOL {
        (self
            .FPDFPage_GetArtBox
            .as_ref()
            .expect("Expected function, got error."))(page, left, bottom, right, top)
    }
    pub unsafe fn FPDFPage_TransFormWithClip(
        &self,
        page: FPDF_PAGE,
        matrix: *const FS_MATRIX,
        clipRect: *const FS_RECTF,
    ) -> FPDF_BOOL {
        (self
            .FPDFPage_TransFormWithClip
            .as_ref()
            .expect("Expected function, got error."))(page, matrix, clipRect)
    }
    pub unsafe fn FPDFPageObj_TransformClipPath(
        &self,
        page_object: FPDF_PAGEOBJECT,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) {
        (self
            .FPDFPageObj_TransformClipPath
            .as_ref()
            .expect("Expected function, got error."))(page_object, a, b, c, d, e, f)
    }
    pub unsafe fn FPDFPageObj_GetClipPath(&self, page_object: FPDF_PAGEOBJECT) -> FPDF_CLIPPATH {
        (self
            .FPDFPageObj_GetClipPath
            .as_ref()
            .expect("Expected function, got error."))(page_object)
    }
    pub unsafe fn FPDFClipPath_CountPaths(&self, clip_path: FPDF_CLIPPATH) -> ::core::ffi::c_int {
        (self
            .FPDFClipPath_CountPaths
            .as_ref()
            .expect("Expected function, got error."))(clip_path)
    }
    pub unsafe fn FPDFClipPath_CountPathSegments(
        &self,
        clip_path: FPDF_CLIPPATH,
        path_index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        (self
            .FPDFClipPath_CountPathSegments
            .as_ref()
            .expect("Expected function, got error."))(clip_path, path_index)
    }
    pub unsafe fn FPDFClipPath_GetPathSegment(
        &self,
        clip_path: FPDF_CLIPPATH,
        path_index: ::core::ffi::c_int,
        segment_index: ::core::ffi::c_int,
    ) -> FPDF_PATHSEGMENT {
        (self
            .FPDFClipPath_GetPathSegment
            .as_ref()
            .expect("Expected function, got error."))(clip_path, path_index, segment_index)
    }
    pub unsafe fn FPDF_CreateClipPath(
        &self,
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
    ) -> FPDF_CLIPPATH {
        (self
            .FPDF_CreateClipPath
            .as_ref()
            .expect("Expected function, got error."))(left, bottom, right, top)
    }
    pub unsafe fn FPDF_DestroyClipPath(&self, clipPath: FPDF_CLIPPATH) {
        (self
            .FPDF_DestroyClipPath
            .as_ref()
            .expect("Expected function, got error."))(clipPath)
    }
    pub unsafe fn FPDFPage_InsertClipPath(&self, page: FPDF_PAGE, clipPath: FPDF_CLIPPATH) {
        (self
            .FPDFPage_InsertClipPath
            .as_ref()
            .expect("Expected function, got error."))(page, clipPath)
    }
}

