// Generated lifecycle-gated wrappers; do not edit by hand.
#[allow(non_snake_case)]
impl crate::raw::RawApi<'_> {
    /// Calls `FPDF_SetSandBoxPolicy` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_SetSandBoxPolicy`.
    pub unsafe fn FPDF_SetSandBoxPolicy(&self, policy: FPDF_DWORD, enable: FPDF_BOOL) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDF_SetSandBoxPolicy", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_SetSandBoxPolicy(policy, enable) })
    }
    /// Calls `FPDF_LoadDocument` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_LoadDocument`.
    pub unsafe fn FPDF_LoadDocument(&self, file_path: FPDF_STRING, password: FPDF_BYTESTRING) -> Result<crate::PdfiumCall<FPDF_DOCUMENT>, crate::PdfiumError> {
        crate::runtime::call("FPDF_LoadDocument", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_LoadDocument(file_path, password) })
    }
    /// Calls `FPDF_LoadMemDocument` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_LoadMemDocument`.
    pub unsafe fn FPDF_LoadMemDocument(&self, data_buf: *const ::core::ffi::c_void, size: ::core::ffi::c_int, password: FPDF_BYTESTRING) -> Result<crate::PdfiumCall<FPDF_DOCUMENT>, crate::PdfiumError> {
        crate::runtime::call("FPDF_LoadMemDocument", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_LoadMemDocument(data_buf, size, password) })
    }
    /// Calls `FPDF_LoadMemDocument64` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_LoadMemDocument64`.
    pub unsafe fn FPDF_LoadMemDocument64(&self, data_buf: *const ::core::ffi::c_void, size: usize, password: FPDF_BYTESTRING) -> Result<crate::PdfiumCall<FPDF_DOCUMENT>, crate::PdfiumError> {
        crate::runtime::call("FPDF_LoadMemDocument64", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_LoadMemDocument64(data_buf, size, password) })
    }
    /// Calls `FPDF_LoadCustomDocument` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_LoadCustomDocument`.
    pub unsafe fn FPDF_LoadCustomDocument(&self, pFileAccess: *mut FPDF_FILEACCESS, password: FPDF_BYTESTRING) -> Result<crate::PdfiumCall<FPDF_DOCUMENT>, crate::PdfiumError> {
        crate::runtime::call("FPDF_LoadCustomDocument", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_LoadCustomDocument(pFileAccess, password) })
    }
    /// Calls `FPDF_GetFileVersion` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetFileVersion`.
    pub unsafe fn FPDF_GetFileVersion(&self, doc: FPDF_DOCUMENT, fileVersion: *mut ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetFileVersion", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetFileVersion(doc, fileVersion) })
    }
    /// Calls `FPDF_GetLastError` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetLastError`.
    pub unsafe fn FPDF_GetLastError(&self) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetLastError", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetLastError() })
    }
    /// Calls `FPDF_DocumentHasValidCrossReferenceTable` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_DocumentHasValidCrossReferenceTable`.
    pub unsafe fn FPDF_DocumentHasValidCrossReferenceTable(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_DocumentHasValidCrossReferenceTable", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_DocumentHasValidCrossReferenceTable(document) })
    }
    /// Calls `FPDF_GetTrailerEnds` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetTrailerEnds`.
    pub unsafe fn FPDF_GetTrailerEnds(&self, document: FPDF_DOCUMENT, buffer: *mut ::core::ffi::c_uint, length: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetTrailerEnds", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetTrailerEnds(document, buffer, length) })
    }
    /// Calls `FPDF_GetDocPermissions` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetDocPermissions`.
    pub unsafe fn FPDF_GetDocPermissions(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetDocPermissions", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetDocPermissions(document) })
    }
    /// Calls `FPDF_GetDocUserPermissions` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetDocUserPermissions`.
    pub unsafe fn FPDF_GetDocUserPermissions(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetDocUserPermissions", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetDocUserPermissions(document) })
    }
    /// Calls `FPDF_GetSecurityHandlerRevision` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetSecurityHandlerRevision`.
    pub unsafe fn FPDF_GetSecurityHandlerRevision(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetSecurityHandlerRevision", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetSecurityHandlerRevision(document) })
    }
    /// Calls `FPDF_GetPageCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetPageCount`.
    pub unsafe fn FPDF_GetPageCount(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetPageCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetPageCount(document) })
    }
    /// Calls `FPDF_LoadPage` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_LoadPage`.
    pub unsafe fn FPDF_LoadPage(&self, document: FPDF_DOCUMENT, page_index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_PAGE>, crate::PdfiumError> {
        crate::runtime::call("FPDF_LoadPage", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_LoadPage(document, page_index) })
    }
    /// Calls `FPDF_GetPageWidthF` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetPageWidthF`.
    pub unsafe fn FPDF_GetPageWidthF(&self, page: FPDF_PAGE) -> Result<crate::PdfiumCall<f32>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetPageWidthF", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetPageWidthF(page) })
    }
    /// Calls `FPDF_GetPageWidth` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetPageWidth`.
    pub unsafe fn FPDF_GetPageWidth(&self, page: FPDF_PAGE) -> Result<crate::PdfiumCall<f64>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetPageWidth", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetPageWidth(page) })
    }
    /// Calls `FPDF_GetPageHeightF` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetPageHeightF`.
    pub unsafe fn FPDF_GetPageHeightF(&self, page: FPDF_PAGE) -> Result<crate::PdfiumCall<f32>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetPageHeightF", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetPageHeightF(page) })
    }
    /// Calls `FPDF_GetPageHeight` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetPageHeight`.
    pub unsafe fn FPDF_GetPageHeight(&self, page: FPDF_PAGE) -> Result<crate::PdfiumCall<f64>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetPageHeight", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetPageHeight(page) })
    }
    /// Calls `FPDF_GetPageBoundingBox` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetPageBoundingBox`.
    pub unsafe fn FPDF_GetPageBoundingBox(&self, page: FPDF_PAGE, rect: *mut FS_RECTF) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetPageBoundingBox", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetPageBoundingBox(page, rect) })
    }
    /// Calls `FPDF_GetPageSizeByIndexF` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetPageSizeByIndexF`.
    pub unsafe fn FPDF_GetPageSizeByIndexF(&self, document: FPDF_DOCUMENT, page_index: ::core::ffi::c_int, size: *mut FS_SIZEF) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetPageSizeByIndexF", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetPageSizeByIndexF(document, page_index, size) })
    }
    /// Calls `FPDF_GetPageSizeByIndex` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetPageSizeByIndex`.
    pub unsafe fn FPDF_GetPageSizeByIndex(&self, document: FPDF_DOCUMENT, page_index: ::core::ffi::c_int, width: *mut f64, height: *mut f64) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetPageSizeByIndex", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetPageSizeByIndex(document, page_index, width, height) })
    }
    /// Calls `FPDF_RenderPageBitmap` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_RenderPageBitmap`.
    pub unsafe fn FPDF_RenderPageBitmap(&self, bitmap: FPDF_BITMAP, page: FPDF_PAGE, start_x: ::core::ffi::c_int, start_y: ::core::ffi::c_int, size_x: ::core::ffi::c_int, size_y: ::core::ffi::c_int, rotate: ::core::ffi::c_int, flags: ::core::ffi::c_int) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDF_RenderPageBitmap", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_RenderPageBitmap(bitmap, page, start_x, start_y, size_x, size_y, rotate, flags) })
    }
    /// Calls `FPDF_RenderPageBitmapWithMatrix` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_RenderPageBitmapWithMatrix`.
    pub unsafe fn FPDF_RenderPageBitmapWithMatrix(&self, bitmap: FPDF_BITMAP, page: FPDF_PAGE, matrix: *const FS_MATRIX, clipping: *const FS_RECTF, flags: ::core::ffi::c_int) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDF_RenderPageBitmapWithMatrix", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_RenderPageBitmapWithMatrix(bitmap, page, matrix, clipping, flags) })
    }
    /// Calls `FPDF_ClosePage` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_ClosePage`.
    pub unsafe fn FPDF_ClosePage(&self, page: FPDF_PAGE) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDF_ClosePage", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_ClosePage(page) })
    }
    /// Calls `FPDF_CloseDocument` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_CloseDocument`.
    pub unsafe fn FPDF_CloseDocument(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDF_CloseDocument", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_CloseDocument(document) })
    }
    /// Calls `FPDF_DeviceToPage` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_DeviceToPage`.
    pub unsafe fn FPDF_DeviceToPage(&self, page: FPDF_PAGE, start_x: ::core::ffi::c_int, start_y: ::core::ffi::c_int, size_x: ::core::ffi::c_int, size_y: ::core::ffi::c_int, rotate: ::core::ffi::c_int, device_x: ::core::ffi::c_int, device_y: ::core::ffi::c_int, page_x: *mut f64, page_y: *mut f64) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_DeviceToPage", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_DeviceToPage(page, start_x, start_y, size_x, size_y, rotate, device_x, device_y, page_x, page_y) })
    }
    /// Calls `FPDF_PageToDevice` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_PageToDevice`.
    pub unsafe fn FPDF_PageToDevice(&self, page: FPDF_PAGE, start_x: ::core::ffi::c_int, start_y: ::core::ffi::c_int, size_x: ::core::ffi::c_int, size_y: ::core::ffi::c_int, rotate: ::core::ffi::c_int, page_x: f64, page_y: f64, device_x: *mut ::core::ffi::c_int, device_y: *mut ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_PageToDevice", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_PageToDevice(page, start_x, start_y, size_x, size_y, rotate, page_x, page_y, device_x, device_y) })
    }
    /// Calls `FPDFBitmap_Create` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFBitmap_Create`.
    pub unsafe fn FPDFBitmap_Create(&self, width: ::core::ffi::c_int, height: ::core::ffi::c_int, alpha: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BITMAP>, crate::PdfiumError> {
        crate::runtime::call("FPDFBitmap_Create", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFBitmap_Create(width, height, alpha) })
    }
    /// Calls `FPDFBitmap_CreateEx` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFBitmap_CreateEx`.
    pub unsafe fn FPDFBitmap_CreateEx(&self, width: ::core::ffi::c_int, height: ::core::ffi::c_int, format: ::core::ffi::c_int, first_scan: *mut ::core::ffi::c_void, stride: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BITMAP>, crate::PdfiumError> {
        crate::runtime::call("FPDFBitmap_CreateEx", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFBitmap_CreateEx(width, height, format, first_scan, stride) })
    }
    /// Calls `FPDFBitmap_GetFormat` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFBitmap_GetFormat`.
    pub unsafe fn FPDFBitmap_GetFormat(&self, bitmap: FPDF_BITMAP) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFBitmap_GetFormat", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFBitmap_GetFormat(bitmap) })
    }
    /// Calls `FPDFBitmap_FillRect` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFBitmap_FillRect`.
    pub unsafe fn FPDFBitmap_FillRect(&self, bitmap: FPDF_BITMAP, left: ::core::ffi::c_int, top: ::core::ffi::c_int, width: ::core::ffi::c_int, height: ::core::ffi::c_int, color: FPDF_DWORD) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFBitmap_FillRect", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFBitmap_FillRect(bitmap, left, top, width, height, color) })
    }
    /// Calls `FPDFBitmap_GetBuffer` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFBitmap_GetBuffer`.
    pub unsafe fn FPDFBitmap_GetBuffer(&self, bitmap: FPDF_BITMAP) -> Result<crate::PdfiumCall<*mut ::core::ffi::c_void>, crate::PdfiumError> {
        crate::runtime::call("FPDFBitmap_GetBuffer", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFBitmap_GetBuffer(bitmap) })
    }
    /// Calls `FPDFBitmap_GetWidth` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFBitmap_GetWidth`.
    pub unsafe fn FPDFBitmap_GetWidth(&self, bitmap: FPDF_BITMAP) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFBitmap_GetWidth", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFBitmap_GetWidth(bitmap) })
    }
    /// Calls `FPDFBitmap_GetHeight` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFBitmap_GetHeight`.
    pub unsafe fn FPDFBitmap_GetHeight(&self, bitmap: FPDF_BITMAP) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFBitmap_GetHeight", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFBitmap_GetHeight(bitmap) })
    }
    /// Calls `FPDFBitmap_GetStride` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFBitmap_GetStride`.
    pub unsafe fn FPDFBitmap_GetStride(&self, bitmap: FPDF_BITMAP) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFBitmap_GetStride", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFBitmap_GetStride(bitmap) })
    }
    /// Calls `FPDFBitmap_Destroy` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFBitmap_Destroy`.
    pub unsafe fn FPDFBitmap_Destroy(&self, bitmap: FPDF_BITMAP) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFBitmap_Destroy", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFBitmap_Destroy(bitmap) })
    }
    /// Calls `FPDF_VIEWERREF_GetPrintScaling` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_VIEWERREF_GetPrintScaling`.
    pub unsafe fn FPDF_VIEWERREF_GetPrintScaling(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_VIEWERREF_GetPrintScaling", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_VIEWERREF_GetPrintScaling(document) })
    }
    /// Calls `FPDF_VIEWERREF_GetNumCopies` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_VIEWERREF_GetNumCopies`.
    pub unsafe fn FPDF_VIEWERREF_GetNumCopies(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_VIEWERREF_GetNumCopies", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_VIEWERREF_GetNumCopies(document) })
    }
    /// Calls `FPDF_VIEWERREF_GetPrintPageRange` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_VIEWERREF_GetPrintPageRange`.
    pub unsafe fn FPDF_VIEWERREF_GetPrintPageRange(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<FPDF_PAGERANGE>, crate::PdfiumError> {
        crate::runtime::call("FPDF_VIEWERREF_GetPrintPageRange", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_VIEWERREF_GetPrintPageRange(document) })
    }
    /// Calls `FPDF_VIEWERREF_GetPrintPageRangeCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_VIEWERREF_GetPrintPageRangeCount`.
    pub unsafe fn FPDF_VIEWERREF_GetPrintPageRangeCount(&self, pagerange: FPDF_PAGERANGE) -> Result<crate::PdfiumCall<usize>, crate::PdfiumError> {
        crate::runtime::call("FPDF_VIEWERREF_GetPrintPageRangeCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_VIEWERREF_GetPrintPageRangeCount(pagerange) })
    }
    /// Calls `FPDF_VIEWERREF_GetPrintPageRangeElement` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_VIEWERREF_GetPrintPageRangeElement`.
    pub unsafe fn FPDF_VIEWERREF_GetPrintPageRangeElement(&self, pagerange: FPDF_PAGERANGE, index: usize) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_VIEWERREF_GetPrintPageRangeElement", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_VIEWERREF_GetPrintPageRangeElement(pagerange, index) })
    }
    /// Calls `FPDF_VIEWERREF_GetDuplex` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_VIEWERREF_GetDuplex`.
    pub unsafe fn FPDF_VIEWERREF_GetDuplex(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<FPDF_DUPLEXTYPE>, crate::PdfiumError> {
        crate::runtime::call("FPDF_VIEWERREF_GetDuplex", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_VIEWERREF_GetDuplex(document) })
    }
    /// Calls `FPDF_VIEWERREF_GetName` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_VIEWERREF_GetName`.
    pub unsafe fn FPDF_VIEWERREF_GetName(&self, document: FPDF_DOCUMENT, key: FPDF_BYTESTRING, buffer: *mut ::core::ffi::c_char, length: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_VIEWERREF_GetName", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_VIEWERREF_GetName(document, key, buffer, length) })
    }
    /// Calls `FPDF_CountNamedDests` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_CountNamedDests`.
    pub unsafe fn FPDF_CountNamedDests(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<FPDF_DWORD>, crate::PdfiumError> {
        crate::runtime::call("FPDF_CountNamedDests", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_CountNamedDests(document) })
    }
    /// Calls `FPDF_GetNamedDestByName` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetNamedDestByName`.
    pub unsafe fn FPDF_GetNamedDestByName(&self, document: FPDF_DOCUMENT, name: FPDF_BYTESTRING) -> Result<crate::PdfiumCall<FPDF_DEST>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetNamedDestByName", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetNamedDestByName(document, name) })
    }
    /// Calls `FPDF_GetNamedDest` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetNamedDest`.
    pub unsafe fn FPDF_GetNamedDest(&self, document: FPDF_DOCUMENT, index: ::core::ffi::c_int, buffer: *mut ::core::ffi::c_void, buflen: *mut ::core::ffi::c_long) -> Result<crate::PdfiumCall<FPDF_DEST>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetNamedDest", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetNamedDest(document, index, buffer, buflen) })
    }
    /// Calls `FPDF_GetXFAPacketCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetXFAPacketCount`.
    pub unsafe fn FPDF_GetXFAPacketCount(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetXFAPacketCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetXFAPacketCount(document) })
    }
    /// Calls `FPDF_GetXFAPacketName` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetXFAPacketName`.
    pub unsafe fn FPDF_GetXFAPacketName(&self, document: FPDF_DOCUMENT, index: ::core::ffi::c_int, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetXFAPacketName", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetXFAPacketName(document, index, buffer, buflen) })
    }
    /// Calls `FPDF_GetXFAPacketContent` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetXFAPacketContent`.
    pub unsafe fn FPDF_GetXFAPacketContent(&self, document: FPDF_DOCUMENT, index: ::core::ffi::c_int, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong, out_buflen: *mut ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetXFAPacketContent", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetXFAPacketContent(document, index, buffer, buflen, out_buflen) })
    }
    /// Calls `FPDF_BStr_Init` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_BStr_Init`.
    pub unsafe fn FPDF_BStr_Init(&self, bstr: *mut FPDF_BSTR) -> Result<crate::PdfiumCall<FPDF_RESULT>, crate::PdfiumError> {
        crate::runtime::call("FPDF_BStr_Init", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_BStr_Init(bstr) })
    }
    /// Calls `FPDF_BStr_Set` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_BStr_Set`.
    pub unsafe fn FPDF_BStr_Set(&self, bstr: *mut FPDF_BSTR, cstr: *const ::core::ffi::c_char, length: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_RESULT>, crate::PdfiumError> {
        crate::runtime::call("FPDF_BStr_Set", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_BStr_Set(bstr, cstr, length) })
    }
    /// Calls `FPDF_BStr_Clear` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_BStr_Clear`.
    pub unsafe fn FPDF_BStr_Clear(&self, bstr: *mut FPDF_BSTR) -> Result<crate::PdfiumCall<FPDF_RESULT>, crate::PdfiumError> {
        crate::runtime::call("FPDF_BStr_Clear", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_BStr_Clear(bstr) })
    }
    /// Calls `FPDFDOC_InitFormFillEnvironment` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFDOC_InitFormFillEnvironment`.
    pub unsafe fn FPDFDOC_InitFormFillEnvironment(&self, document: FPDF_DOCUMENT, formInfo: *mut FPDF_FORMFILLINFO) -> Result<crate::PdfiumCall<FPDF_FORMHANDLE>, crate::PdfiumError> {
        crate::runtime::call("FPDFDOC_InitFormFillEnvironment", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFDOC_InitFormFillEnvironment(document, formInfo) })
    }
    /// Calls `FPDFDOC_ExitFormFillEnvironment` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFDOC_ExitFormFillEnvironment`.
    pub unsafe fn FPDFDOC_ExitFormFillEnvironment(&self, hHandle: FPDF_FORMHANDLE) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFDOC_ExitFormFillEnvironment", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFDOC_ExitFormFillEnvironment(hHandle) })
    }
    /// Calls `FORM_OnAfterLoadPage` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_OnAfterLoadPage`.
    pub unsafe fn FORM_OnAfterLoadPage(&self, page: FPDF_PAGE, hHandle: FPDF_FORMHANDLE) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FORM_OnAfterLoadPage", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_OnAfterLoadPage(page, hHandle) })
    }
    /// Calls `FORM_OnBeforeClosePage` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_OnBeforeClosePage`.
    pub unsafe fn FORM_OnBeforeClosePage(&self, page: FPDF_PAGE, hHandle: FPDF_FORMHANDLE) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FORM_OnBeforeClosePage", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_OnBeforeClosePage(page, hHandle) })
    }
    /// Calls `FORM_DoDocumentJSAction` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_DoDocumentJSAction`.
    pub unsafe fn FORM_DoDocumentJSAction(&self, hHandle: FPDF_FORMHANDLE) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FORM_DoDocumentJSAction", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_DoDocumentJSAction(hHandle) })
    }
    /// Calls `FORM_DoDocumentOpenAction` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_DoDocumentOpenAction`.
    pub unsafe fn FORM_DoDocumentOpenAction(&self, hHandle: FPDF_FORMHANDLE) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FORM_DoDocumentOpenAction", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_DoDocumentOpenAction(hHandle) })
    }
    /// Calls `FORM_DoDocumentAAction` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_DoDocumentAAction`.
    pub unsafe fn FORM_DoDocumentAAction(&self, hHandle: FPDF_FORMHANDLE, aaType: ::core::ffi::c_int) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FORM_DoDocumentAAction", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_DoDocumentAAction(hHandle, aaType) })
    }
    /// Calls `FORM_DoPageAAction` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_DoPageAAction`.
    pub unsafe fn FORM_DoPageAAction(&self, page: FPDF_PAGE, hHandle: FPDF_FORMHANDLE, aaType: ::core::ffi::c_int) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FORM_DoPageAAction", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_DoPageAAction(page, hHandle, aaType) })
    }
    /// Calls `FORM_OnMouseMove` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_OnMouseMove`.
    pub unsafe fn FORM_OnMouseMove(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, modifier: ::core::ffi::c_int, page_x: f64, page_y: f64) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_OnMouseMove", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_OnMouseMove(hHandle, page, modifier, page_x, page_y) })
    }
    /// Calls `FORM_OnMouseWheel` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_OnMouseWheel`.
    pub unsafe fn FORM_OnMouseWheel(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, modifier: ::core::ffi::c_int, page_coord: *const FS_POINTF, delta_x: ::core::ffi::c_int, delta_y: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_OnMouseWheel", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_OnMouseWheel(hHandle, page, modifier, page_coord, delta_x, delta_y) })
    }
    /// Calls `FORM_OnFocus` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_OnFocus`.
    pub unsafe fn FORM_OnFocus(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, modifier: ::core::ffi::c_int, page_x: f64, page_y: f64) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_OnFocus", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_OnFocus(hHandle, page, modifier, page_x, page_y) })
    }
    /// Calls `FORM_OnLButtonDown` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_OnLButtonDown`.
    pub unsafe fn FORM_OnLButtonDown(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, modifier: ::core::ffi::c_int, page_x: f64, page_y: f64) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_OnLButtonDown", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_OnLButtonDown(hHandle, page, modifier, page_x, page_y) })
    }
    /// Calls `FORM_OnRButtonDown` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_OnRButtonDown`.
    pub unsafe fn FORM_OnRButtonDown(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, modifier: ::core::ffi::c_int, page_x: f64, page_y: f64) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_OnRButtonDown", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_OnRButtonDown(hHandle, page, modifier, page_x, page_y) })
    }
    /// Calls `FORM_OnLButtonUp` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_OnLButtonUp`.
    pub unsafe fn FORM_OnLButtonUp(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, modifier: ::core::ffi::c_int, page_x: f64, page_y: f64) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_OnLButtonUp", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_OnLButtonUp(hHandle, page, modifier, page_x, page_y) })
    }
    /// Calls `FORM_OnRButtonUp` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_OnRButtonUp`.
    pub unsafe fn FORM_OnRButtonUp(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, modifier: ::core::ffi::c_int, page_x: f64, page_y: f64) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_OnRButtonUp", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_OnRButtonUp(hHandle, page, modifier, page_x, page_y) })
    }
    /// Calls `FORM_OnLButtonDoubleClick` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_OnLButtonDoubleClick`.
    pub unsafe fn FORM_OnLButtonDoubleClick(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, modifier: ::core::ffi::c_int, page_x: f64, page_y: f64) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_OnLButtonDoubleClick", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_OnLButtonDoubleClick(hHandle, page, modifier, page_x, page_y) })
    }
    /// Calls `FORM_OnKeyDown` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_OnKeyDown`.
    pub unsafe fn FORM_OnKeyDown(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, nKeyCode: ::core::ffi::c_int, modifier: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_OnKeyDown", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_OnKeyDown(hHandle, page, nKeyCode, modifier) })
    }
    /// Calls `FORM_OnKeyUp` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_OnKeyUp`.
    pub unsafe fn FORM_OnKeyUp(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, nKeyCode: ::core::ffi::c_int, modifier: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_OnKeyUp", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_OnKeyUp(hHandle, page, nKeyCode, modifier) })
    }
    /// Calls `FORM_OnChar` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_OnChar`.
    pub unsafe fn FORM_OnChar(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, nChar: ::core::ffi::c_int, modifier: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_OnChar", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_OnChar(hHandle, page, nChar, modifier) })
    }
    /// Calls `FORM_GetFocusedText` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_GetFocusedText`.
    pub unsafe fn FORM_GetFocusedText(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FORM_GetFocusedText", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_GetFocusedText(hHandle, page, buffer, buflen) })
    }
    /// Calls `FORM_GetSelectedText` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_GetSelectedText`.
    pub unsafe fn FORM_GetSelectedText(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FORM_GetSelectedText", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_GetSelectedText(hHandle, page, buffer, buflen) })
    }
    /// Calls `FORM_ReplaceAndKeepSelection` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_ReplaceAndKeepSelection`.
    pub unsafe fn FORM_ReplaceAndKeepSelection(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, wsText: FPDF_WIDESTRING) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FORM_ReplaceAndKeepSelection", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_ReplaceAndKeepSelection(hHandle, page, wsText) })
    }
    /// Calls `FORM_ReplaceSelection` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_ReplaceSelection`.
    pub unsafe fn FORM_ReplaceSelection(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, wsText: FPDF_WIDESTRING) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FORM_ReplaceSelection", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_ReplaceSelection(hHandle, page, wsText) })
    }
    /// Calls `FORM_SelectAllText` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_SelectAllText`.
    pub unsafe fn FORM_SelectAllText(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_SelectAllText", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_SelectAllText(hHandle, page) })
    }
    /// Calls `FORM_CanUndo` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_CanUndo`.
    pub unsafe fn FORM_CanUndo(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_CanUndo", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_CanUndo(hHandle, page) })
    }
    /// Calls `FORM_CanRedo` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_CanRedo`.
    pub unsafe fn FORM_CanRedo(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_CanRedo", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_CanRedo(hHandle, page) })
    }
    /// Calls `FORM_Undo` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_Undo`.
    pub unsafe fn FORM_Undo(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_Undo", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_Undo(hHandle, page) })
    }
    /// Calls `FORM_Redo` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_Redo`.
    pub unsafe fn FORM_Redo(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_Redo", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_Redo(hHandle, page) })
    }
    /// Calls `FORM_ForceToKillFocus` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_ForceToKillFocus`.
    pub unsafe fn FORM_ForceToKillFocus(&self, hHandle: FPDF_FORMHANDLE) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_ForceToKillFocus", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_ForceToKillFocus(hHandle) })
    }
    /// Calls `FORM_GetFocusedAnnot` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_GetFocusedAnnot`.
    pub unsafe fn FORM_GetFocusedAnnot(&self, handle: FPDF_FORMHANDLE, page_index: *mut ::core::ffi::c_int, annot: *mut FPDF_ANNOTATION) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_GetFocusedAnnot", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_GetFocusedAnnot(handle, page_index, annot) })
    }
    /// Calls `FORM_SetFocusedAnnot` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_SetFocusedAnnot`.
    pub unsafe fn FORM_SetFocusedAnnot(&self, handle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_SetFocusedAnnot", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_SetFocusedAnnot(handle, annot) })
    }
    /// Calls `FPDFPage_HasFormFieldAtPoint` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_HasFormFieldAtPoint`.
    pub unsafe fn FPDFPage_HasFormFieldAtPoint(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, page_x: f64, page_y: f64) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_HasFormFieldAtPoint", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_HasFormFieldAtPoint(hHandle, page, page_x, page_y) })
    }
    /// Calls `FPDFPage_FormFieldZOrderAtPoint` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_FormFieldZOrderAtPoint`.
    pub unsafe fn FPDFPage_FormFieldZOrderAtPoint(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, page_x: f64, page_y: f64) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_FormFieldZOrderAtPoint", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_FormFieldZOrderAtPoint(hHandle, page, page_x, page_y) })
    }
    /// Calls `FPDF_SetFormFieldHighlightColor` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_SetFormFieldHighlightColor`.
    pub unsafe fn FPDF_SetFormFieldHighlightColor(&self, hHandle: FPDF_FORMHANDLE, fieldType: ::core::ffi::c_int, color: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDF_SetFormFieldHighlightColor", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_SetFormFieldHighlightColor(hHandle, fieldType, color) })
    }
    /// Calls `FPDF_SetFormFieldHighlightAlpha` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_SetFormFieldHighlightAlpha`.
    pub unsafe fn FPDF_SetFormFieldHighlightAlpha(&self, hHandle: FPDF_FORMHANDLE, alpha: ::core::ffi::c_uchar) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDF_SetFormFieldHighlightAlpha", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_SetFormFieldHighlightAlpha(hHandle, alpha) })
    }
    /// Calls `FPDF_RemoveFormFieldHighlight` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_RemoveFormFieldHighlight`.
    pub unsafe fn FPDF_RemoveFormFieldHighlight(&self, hHandle: FPDF_FORMHANDLE) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDF_RemoveFormFieldHighlight", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_RemoveFormFieldHighlight(hHandle) })
    }
    /// Calls `FPDF_FFLDraw` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_FFLDraw`.
    pub unsafe fn FPDF_FFLDraw(&self, hHandle: FPDF_FORMHANDLE, bitmap: FPDF_BITMAP, page: FPDF_PAGE, start_x: ::core::ffi::c_int, start_y: ::core::ffi::c_int, size_x: ::core::ffi::c_int, size_y: ::core::ffi::c_int, rotate: ::core::ffi::c_int, flags: ::core::ffi::c_int) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDF_FFLDraw", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_FFLDraw(hHandle, bitmap, page, start_x, start_y, size_x, size_y, rotate, flags) })
    }
    /// Calls `FPDF_GetFormType` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetFormType`.
    pub unsafe fn FPDF_GetFormType(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetFormType", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetFormType(document) })
    }
    /// Calls `FORM_SetIndexSelected` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_SetIndexSelected`.
    pub unsafe fn FORM_SetIndexSelected(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, index: ::core::ffi::c_int, selected: FPDF_BOOL) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_SetIndexSelected", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_SetIndexSelected(hHandle, page, index, selected) })
    }
    /// Calls `FORM_IsIndexSelected` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_IsIndexSelected`.
    pub unsafe fn FORM_IsIndexSelected(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_IsIndexSelected", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_IsIndexSelected(hHandle, page, index) })
    }
    /// Calls `FORM_GetTextDirection` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_GetTextDirection`.
    pub unsafe fn FORM_GetTextDirection(&self, handle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<FPDF_TEXT_DIRECTION>, crate::PdfiumError> {
        crate::runtime::call("FORM_GetTextDirection", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_GetTextDirection(handle, annot) })
    }
    /// Calls `FORM_SetTextDirection` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FORM_SetTextDirection`.
    pub unsafe fn FORM_SetTextDirection(&self, handle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION, direction: FPDF_TEXT_DIRECTION) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FORM_SetTextDirection", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FORM_SetTextDirection(handle, annot, direction) })
    }
    /// Calls `FPDF_LoadXFA` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_LoadXFA`.
    pub unsafe fn FPDF_LoadXFA(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_LoadXFA", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_LoadXFA(document) })
    }
    /// Calls `FPDFAnnot_IsSupportedSubtype` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_IsSupportedSubtype`.
    pub unsafe fn FPDFAnnot_IsSupportedSubtype(&self, subtype: FPDF_ANNOTATION_SUBTYPE) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_IsSupportedSubtype", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_IsSupportedSubtype(subtype) })
    }
    /// Calls `FPDFPage_CreateAnnot` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_CreateAnnot`.
    pub unsafe fn FPDFPage_CreateAnnot(&self, page: FPDF_PAGE, subtype: FPDF_ANNOTATION_SUBTYPE) -> Result<crate::PdfiumCall<FPDF_ANNOTATION>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_CreateAnnot", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_CreateAnnot(page, subtype) })
    }
    /// Calls `FPDFPage_GetAnnotCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_GetAnnotCount`.
    pub unsafe fn FPDFPage_GetAnnotCount(&self, page: FPDF_PAGE) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_GetAnnotCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_GetAnnotCount(page) })
    }
    /// Calls `FPDFPage_GetAnnot` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_GetAnnot`.
    pub unsafe fn FPDFPage_GetAnnot(&self, page: FPDF_PAGE, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_ANNOTATION>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_GetAnnot", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_GetAnnot(page, index) })
    }
    /// Calls `FPDFPage_GetAnnotIndex` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_GetAnnotIndex`.
    pub unsafe fn FPDFPage_GetAnnotIndex(&self, page: FPDF_PAGE, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_GetAnnotIndex", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_GetAnnotIndex(page, annot) })
    }
    /// Calls `FPDFPage_CloseAnnot` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_CloseAnnot`.
    pub unsafe fn FPDFPage_CloseAnnot(&self, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_CloseAnnot", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_CloseAnnot(annot) })
    }
    /// Calls `FPDFPage_RemoveAnnot` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_RemoveAnnot`.
    pub unsafe fn FPDFPage_RemoveAnnot(&self, page: FPDF_PAGE, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_RemoveAnnot", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_RemoveAnnot(page, index) })
    }
    /// Calls `FPDFAnnot_GetSubtype` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetSubtype`.
    pub unsafe fn FPDFAnnot_GetSubtype(&self, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<FPDF_ANNOTATION_SUBTYPE>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetSubtype", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetSubtype(annot) })
    }
    /// Calls `FPDFAnnot_IsObjectSupportedSubtype` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_IsObjectSupportedSubtype`.
    pub unsafe fn FPDFAnnot_IsObjectSupportedSubtype(&self, subtype: FPDF_ANNOTATION_SUBTYPE) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_IsObjectSupportedSubtype", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_IsObjectSupportedSubtype(subtype) })
    }
    /// Calls `FPDFAnnot_UpdateObject` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_UpdateObject`.
    pub unsafe fn FPDFAnnot_UpdateObject(&self, annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_UpdateObject", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_UpdateObject(annot, obj) })
    }
    /// Calls `FPDFAnnot_AddInkStroke` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_AddInkStroke`.
    pub unsafe fn FPDFAnnot_AddInkStroke(&self, annot: FPDF_ANNOTATION, points: *const FS_POINTF, point_count: usize) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_AddInkStroke", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_AddInkStroke(annot, points, point_count) })
    }
    /// Calls `FPDFAnnot_RemoveInkList` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_RemoveInkList`.
    pub unsafe fn FPDFAnnot_RemoveInkList(&self, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_RemoveInkList", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_RemoveInkList(annot) })
    }
    /// Calls `FPDFAnnot_AppendObject` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_AppendObject`.
    pub unsafe fn FPDFAnnot_AppendObject(&self, annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_AppendObject", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_AppendObject(annot, obj) })
    }
    /// Calls `FPDFAnnot_GetObjectCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetObjectCount`.
    pub unsafe fn FPDFAnnot_GetObjectCount(&self, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetObjectCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetObjectCount(annot) })
    }
    /// Calls `FPDFAnnot_GetObject` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetObject`.
    pub unsafe fn FPDFAnnot_GetObject(&self, annot: FPDF_ANNOTATION, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_PAGEOBJECT>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetObject", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetObject(annot, index) })
    }
    /// Calls `FPDFAnnot_RemoveObject` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_RemoveObject`.
    pub unsafe fn FPDFAnnot_RemoveObject(&self, annot: FPDF_ANNOTATION, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_RemoveObject", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_RemoveObject(annot, index) })
    }
    /// Calls `FPDFAnnot_SetColor` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_SetColor`.
    pub unsafe fn FPDFAnnot_SetColor(&self, annot: FPDF_ANNOTATION, type_: FPDFANNOT_COLORTYPE, R: ::core::ffi::c_uint, G: ::core::ffi::c_uint, B: ::core::ffi::c_uint, A: ::core::ffi::c_uint) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_SetColor", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_SetColor(annot, type_, R, G, B, A) })
    }
    /// Calls `FPDFAnnot_GetColor` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetColor`.
    pub unsafe fn FPDFAnnot_GetColor(&self, annot: FPDF_ANNOTATION, type_: FPDFANNOT_COLORTYPE, R: *mut ::core::ffi::c_uint, G: *mut ::core::ffi::c_uint, B: *mut ::core::ffi::c_uint, A: *mut ::core::ffi::c_uint) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetColor", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetColor(annot, type_, R, G, B, A) })
    }
    /// Calls `FPDFAnnot_HasAttachmentPoints` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_HasAttachmentPoints`.
    pub unsafe fn FPDFAnnot_HasAttachmentPoints(&self, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_HasAttachmentPoints", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_HasAttachmentPoints(annot) })
    }
    /// Calls `FPDFAnnot_SetAttachmentPoints` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_SetAttachmentPoints`.
    pub unsafe fn FPDFAnnot_SetAttachmentPoints(&self, annot: FPDF_ANNOTATION, quad_index: usize, quad_points: *const FS_QUADPOINTSF) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_SetAttachmentPoints", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_SetAttachmentPoints(annot, quad_index, quad_points) })
    }
    /// Calls `FPDFAnnot_AppendAttachmentPoints` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_AppendAttachmentPoints`.
    pub unsafe fn FPDFAnnot_AppendAttachmentPoints(&self, annot: FPDF_ANNOTATION, quad_points: *const FS_QUADPOINTSF) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_AppendAttachmentPoints", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_AppendAttachmentPoints(annot, quad_points) })
    }
    /// Calls `FPDFAnnot_CountAttachmentPoints` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_CountAttachmentPoints`.
    pub unsafe fn FPDFAnnot_CountAttachmentPoints(&self, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<usize>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_CountAttachmentPoints", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_CountAttachmentPoints(annot) })
    }
    /// Calls `FPDFAnnot_GetAttachmentPoints` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetAttachmentPoints`.
    pub unsafe fn FPDFAnnot_GetAttachmentPoints(&self, annot: FPDF_ANNOTATION, quad_index: usize, quad_points: *mut FS_QUADPOINTSF) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetAttachmentPoints", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetAttachmentPoints(annot, quad_index, quad_points) })
    }
    /// Calls `FPDFAnnot_SetRect` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_SetRect`.
    pub unsafe fn FPDFAnnot_SetRect(&self, annot: FPDF_ANNOTATION, rect: *const FS_RECTF) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_SetRect", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_SetRect(annot, rect) })
    }
    /// Calls `FPDFAnnot_GetRect` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetRect`.
    pub unsafe fn FPDFAnnot_GetRect(&self, annot: FPDF_ANNOTATION, rect: *mut FS_RECTF) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetRect", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetRect(annot, rect) })
    }
    /// Calls `FPDFAnnot_GetVertices` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetVertices`.
    pub unsafe fn FPDFAnnot_GetVertices(&self, annot: FPDF_ANNOTATION, buffer: *mut FS_POINTF, length: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetVertices", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetVertices(annot, buffer, length) })
    }
    /// Calls `FPDFAnnot_GetInkListCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetInkListCount`.
    pub unsafe fn FPDFAnnot_GetInkListCount(&self, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetInkListCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetInkListCount(annot) })
    }
    /// Calls `FPDFAnnot_GetInkListPath` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetInkListPath`.
    pub unsafe fn FPDFAnnot_GetInkListPath(&self, annot: FPDF_ANNOTATION, path_index: ::core::ffi::c_ulong, buffer: *mut FS_POINTF, length: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetInkListPath", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetInkListPath(annot, path_index, buffer, length) })
    }
    /// Calls `FPDFAnnot_GetLine` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetLine`.
    pub unsafe fn FPDFAnnot_GetLine(&self, annot: FPDF_ANNOTATION, start: *mut FS_POINTF, end: *mut FS_POINTF) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetLine", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetLine(annot, start, end) })
    }
    /// Calls `FPDFAnnot_SetBorder` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_SetBorder`.
    pub unsafe fn FPDFAnnot_SetBorder(&self, annot: FPDF_ANNOTATION, horizontal_radius: f32, vertical_radius: f32, border_width: f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_SetBorder", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_SetBorder(annot, horizontal_radius, vertical_radius, border_width) })
    }
    /// Calls `FPDFAnnot_GetBorder` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetBorder`.
    pub unsafe fn FPDFAnnot_GetBorder(&self, annot: FPDF_ANNOTATION, horizontal_radius: *mut f32, vertical_radius: *mut f32, border_width: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetBorder", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetBorder(annot, horizontal_radius, vertical_radius, border_width) })
    }
    /// Calls `FPDFAnnot_GetFormAdditionalActionJavaScript` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetFormAdditionalActionJavaScript`.
    pub unsafe fn FPDFAnnot_GetFormAdditionalActionJavaScript(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION, event: ::core::ffi::c_int, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetFormAdditionalActionJavaScript", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetFormAdditionalActionJavaScript(hHandle, annot, event, buffer, buflen) })
    }
    /// Calls `FPDFAnnot_HasKey` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_HasKey`.
    pub unsafe fn FPDFAnnot_HasKey(&self, annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_HasKey", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_HasKey(annot, key) })
    }
    /// Calls `FPDFAnnot_GetValueType` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetValueType`.
    pub unsafe fn FPDFAnnot_GetValueType(&self, annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING) -> Result<crate::PdfiumCall<FPDF_OBJECT_TYPE>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetValueType", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetValueType(annot, key) })
    }
    /// Calls `FPDFAnnot_SetStringValue` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_SetStringValue`.
    pub unsafe fn FPDFAnnot_SetStringValue(&self, annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING, value: FPDF_WIDESTRING) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_SetStringValue", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_SetStringValue(annot, key, value) })
    }
    /// Calls `FPDFAnnot_GetStringValue` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetStringValue`.
    pub unsafe fn FPDFAnnot_GetStringValue(&self, annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetStringValue", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetStringValue(annot, key, buffer, buflen) })
    }
    /// Calls `FPDFAnnot_GetNumberValue` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetNumberValue`.
    pub unsafe fn FPDFAnnot_GetNumberValue(&self, annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING, value: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetNumberValue", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetNumberValue(annot, key, value) })
    }
    /// Calls `FPDFAnnot_SetAP` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_SetAP`.
    pub unsafe fn FPDFAnnot_SetAP(&self, annot: FPDF_ANNOTATION, appearanceMode: FPDF_ANNOT_APPEARANCEMODE, value: FPDF_WIDESTRING) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_SetAP", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_SetAP(annot, appearanceMode, value) })
    }
    /// Calls `FPDFAnnot_GetAP` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetAP`.
    pub unsafe fn FPDFAnnot_GetAP(&self, annot: FPDF_ANNOTATION, appearanceMode: FPDF_ANNOT_APPEARANCEMODE, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetAP", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetAP(annot, appearanceMode, buffer, buflen) })
    }
    /// Calls `FPDFAnnot_GetLinkedAnnot` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetLinkedAnnot`.
    pub unsafe fn FPDFAnnot_GetLinkedAnnot(&self, annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING) -> Result<crate::PdfiumCall<FPDF_ANNOTATION>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetLinkedAnnot", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetLinkedAnnot(annot, key) })
    }
    /// Calls `FPDFAnnot_GetFlags` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetFlags`.
    pub unsafe fn FPDFAnnot_GetFlags(&self, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetFlags", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetFlags(annot) })
    }
    /// Calls `FPDFAnnot_SetFlags` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_SetFlags`.
    pub unsafe fn FPDFAnnot_SetFlags(&self, annot: FPDF_ANNOTATION, flags: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_SetFlags", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_SetFlags(annot, flags) })
    }
    /// Calls `FPDFAnnot_GetFormFieldFlags` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetFormFieldFlags`.
    pub unsafe fn FPDFAnnot_GetFormFieldFlags(&self, handle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetFormFieldFlags", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetFormFieldFlags(handle, annot) })
    }
    /// Calls `FPDFAnnot_SetFormFieldFlags` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_SetFormFieldFlags`.
    pub unsafe fn FPDFAnnot_SetFormFieldFlags(&self, handle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION, flags: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_SetFormFieldFlags", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_SetFormFieldFlags(handle, annot, flags) })
    }
    /// Calls `FPDFAnnot_GetFormFieldAtPoint` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetFormFieldAtPoint`.
    pub unsafe fn FPDFAnnot_GetFormFieldAtPoint(&self, hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, point: *const FS_POINTF) -> Result<crate::PdfiumCall<FPDF_ANNOTATION>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetFormFieldAtPoint", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetFormFieldAtPoint(hHandle, page, point) })
    }
    /// Calls `FPDFAnnot_GetFormFieldName` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetFormFieldName`.
    pub unsafe fn FPDFAnnot_GetFormFieldName(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetFormFieldName", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetFormFieldName(hHandle, annot, buffer, buflen) })
    }
    /// Calls `FPDFAnnot_GetFormFieldAlternateName` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetFormFieldAlternateName`.
    pub unsafe fn FPDFAnnot_GetFormFieldAlternateName(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetFormFieldAlternateName", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetFormFieldAlternateName(hHandle, annot, buffer, buflen) })
    }
    /// Calls `FPDFAnnot_GetFormFieldType` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetFormFieldType`.
    pub unsafe fn FPDFAnnot_GetFormFieldType(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetFormFieldType", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetFormFieldType(hHandle, annot) })
    }
    /// Calls `FPDFAnnot_GetFormFieldValue` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetFormFieldValue`.
    pub unsafe fn FPDFAnnot_GetFormFieldValue(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetFormFieldValue", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetFormFieldValue(hHandle, annot, buffer, buflen) })
    }
    /// Calls `FPDFAnnot_GetOptionCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetOptionCount`.
    pub unsafe fn FPDFAnnot_GetOptionCount(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetOptionCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetOptionCount(hHandle, annot) })
    }
    /// Calls `FPDFAnnot_GetOptionLabel` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetOptionLabel`.
    pub unsafe fn FPDFAnnot_GetOptionLabel(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION, index: ::core::ffi::c_int, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetOptionLabel", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetOptionLabel(hHandle, annot, index, buffer, buflen) })
    }
    /// Calls `FPDFAnnot_IsOptionSelected` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_IsOptionSelected`.
    pub unsafe fn FPDFAnnot_IsOptionSelected(&self, handle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_IsOptionSelected", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_IsOptionSelected(handle, annot, index) })
    }
    /// Calls `FPDFAnnot_GetFontSize` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetFontSize`.
    pub unsafe fn FPDFAnnot_GetFontSize(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION, value: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetFontSize", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetFontSize(hHandle, annot, value) })
    }
    /// Calls `FPDFAnnot_SetFontColor` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_SetFontColor`.
    pub unsafe fn FPDFAnnot_SetFontColor(&self, handle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION, R: ::core::ffi::c_uint, G: ::core::ffi::c_uint, B: ::core::ffi::c_uint) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_SetFontColor", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_SetFontColor(handle, annot, R, G, B) })
    }
    /// Calls `FPDFAnnot_GetFontColor` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetFontColor`.
    pub unsafe fn FPDFAnnot_GetFontColor(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION, R: *mut ::core::ffi::c_uint, G: *mut ::core::ffi::c_uint, B: *mut ::core::ffi::c_uint) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetFontColor", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetFontColor(hHandle, annot, R, G, B) })
    }
    /// Calls `FPDFAnnot_IsChecked` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_IsChecked`.
    pub unsafe fn FPDFAnnot_IsChecked(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_IsChecked", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_IsChecked(hHandle, annot) })
    }
    /// Calls `FPDFAnnot_SetFocusableSubtypes` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_SetFocusableSubtypes`.
    pub unsafe fn FPDFAnnot_SetFocusableSubtypes(&self, hHandle: FPDF_FORMHANDLE, subtypes: *const FPDF_ANNOTATION_SUBTYPE, count: usize) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_SetFocusableSubtypes", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_SetFocusableSubtypes(hHandle, subtypes, count) })
    }
    /// Calls `FPDFAnnot_GetFocusableSubtypesCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetFocusableSubtypesCount`.
    pub unsafe fn FPDFAnnot_GetFocusableSubtypesCount(&self, hHandle: FPDF_FORMHANDLE) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetFocusableSubtypesCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetFocusableSubtypesCount(hHandle) })
    }
    /// Calls `FPDFAnnot_GetFocusableSubtypes` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetFocusableSubtypes`.
    pub unsafe fn FPDFAnnot_GetFocusableSubtypes(&self, hHandle: FPDF_FORMHANDLE, subtypes: *mut FPDF_ANNOTATION_SUBTYPE, count: usize) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetFocusableSubtypes", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetFocusableSubtypes(hHandle, subtypes, count) })
    }
    /// Calls `FPDFAnnot_GetLink` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetLink`.
    pub unsafe fn FPDFAnnot_GetLink(&self, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<FPDF_LINK>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetLink", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetLink(annot) })
    }
    /// Calls `FPDFAnnot_GetFormControlCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetFormControlCount`.
    pub unsafe fn FPDFAnnot_GetFormControlCount(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetFormControlCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetFormControlCount(hHandle, annot) })
    }
    /// Calls `FPDFAnnot_GetFormControlIndex` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetFormControlIndex`.
    pub unsafe fn FPDFAnnot_GetFormControlIndex(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetFormControlIndex", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetFormControlIndex(hHandle, annot) })
    }
    /// Calls `FPDFAnnot_GetFormFieldExportValue` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetFormFieldExportValue`.
    pub unsafe fn FPDFAnnot_GetFormFieldExportValue(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetFormFieldExportValue", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetFormFieldExportValue(hHandle, annot, buffer, buflen) })
    }
    /// Calls `FPDFAnnot_SetURI` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_SetURI`.
    pub unsafe fn FPDFAnnot_SetURI(&self, annot: FPDF_ANNOTATION, uri: *const ::core::ffi::c_char) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_SetURI", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_SetURI(annot, uri) })
    }
    /// Calls `FPDFAnnot_GetFileAttachment` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_GetFileAttachment`.
    pub unsafe fn FPDFAnnot_GetFileAttachment(&self, annot: FPDF_ANNOTATION) -> Result<crate::PdfiumCall<FPDF_ATTACHMENT>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_GetFileAttachment", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_GetFileAttachment(annot) })
    }
    /// Calls `FPDFAnnot_AddFileAttachment` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAnnot_AddFileAttachment`.
    pub unsafe fn FPDFAnnot_AddFileAttachment(&self, annot: FPDF_ANNOTATION, name: FPDF_WIDESTRING) -> Result<crate::PdfiumCall<FPDF_ATTACHMENT>, crate::PdfiumError> {
        crate::runtime::call("FPDFAnnot_AddFileAttachment", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAnnot_AddFileAttachment(annot, name) })
    }
    /// Calls `FPDFDoc_GetAttachmentCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFDoc_GetAttachmentCount`.
    pub unsafe fn FPDFDoc_GetAttachmentCount(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFDoc_GetAttachmentCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFDoc_GetAttachmentCount(document) })
    }
    /// Calls `FPDFDoc_AddAttachment` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFDoc_AddAttachment`.
    pub unsafe fn FPDFDoc_AddAttachment(&self, document: FPDF_DOCUMENT, name: FPDF_WIDESTRING) -> Result<crate::PdfiumCall<FPDF_ATTACHMENT>, crate::PdfiumError> {
        crate::runtime::call("FPDFDoc_AddAttachment", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFDoc_AddAttachment(document, name) })
    }
    /// Calls `FPDFDoc_GetAttachment` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFDoc_GetAttachment`.
    pub unsafe fn FPDFDoc_GetAttachment(&self, document: FPDF_DOCUMENT, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_ATTACHMENT>, crate::PdfiumError> {
        crate::runtime::call("FPDFDoc_GetAttachment", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFDoc_GetAttachment(document, index) })
    }
    /// Calls `FPDFDoc_DeleteAttachment` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFDoc_DeleteAttachment`.
    pub unsafe fn FPDFDoc_DeleteAttachment(&self, document: FPDF_DOCUMENT, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFDoc_DeleteAttachment", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFDoc_DeleteAttachment(document, index) })
    }
    /// Calls `FPDFAttachment_GetName` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAttachment_GetName`.
    pub unsafe fn FPDFAttachment_GetName(&self, attachment: FPDF_ATTACHMENT, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAttachment_GetName", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAttachment_GetName(attachment, buffer, buflen) })
    }
    /// Calls `FPDFAttachment_HasKey` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAttachment_HasKey`.
    pub unsafe fn FPDFAttachment_HasKey(&self, attachment: FPDF_ATTACHMENT, key: FPDF_BYTESTRING) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAttachment_HasKey", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAttachment_HasKey(attachment, key) })
    }
    /// Calls `FPDFAttachment_GetValueType` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAttachment_GetValueType`.
    pub unsafe fn FPDFAttachment_GetValueType(&self, attachment: FPDF_ATTACHMENT, key: FPDF_BYTESTRING) -> Result<crate::PdfiumCall<FPDF_OBJECT_TYPE>, crate::PdfiumError> {
        crate::runtime::call("FPDFAttachment_GetValueType", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAttachment_GetValueType(attachment, key) })
    }
    /// Calls `FPDFAttachment_SetStringValue` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAttachment_SetStringValue`.
    pub unsafe fn FPDFAttachment_SetStringValue(&self, attachment: FPDF_ATTACHMENT, key: FPDF_BYTESTRING, value: FPDF_WIDESTRING) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAttachment_SetStringValue", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAttachment_SetStringValue(attachment, key, value) })
    }
    /// Calls `FPDFAttachment_GetStringValue` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAttachment_GetStringValue`.
    pub unsafe fn FPDFAttachment_GetStringValue(&self, attachment: FPDF_ATTACHMENT, key: FPDF_BYTESTRING, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAttachment_GetStringValue", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAttachment_GetStringValue(attachment, key, buffer, buflen) })
    }
    /// Calls `FPDFAttachment_SetDescription` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAttachment_SetDescription`.
    pub unsafe fn FPDFAttachment_SetDescription(&self, attachment: FPDF_ATTACHMENT, value: FPDF_WIDESTRING) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAttachment_SetDescription", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAttachment_SetDescription(attachment, value) })
    }
    /// Calls `FPDFAttachment_GetDescription` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAttachment_GetDescription`.
    pub unsafe fn FPDFAttachment_GetDescription(&self, attachment: FPDF_ATTACHMENT, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAttachment_GetDescription", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAttachment_GetDescription(attachment, buffer, buflen) })
    }
    /// Calls `FPDFAttachment_SetFile` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAttachment_SetFile`.
    pub unsafe fn FPDFAttachment_SetFile(&self, attachment: FPDF_ATTACHMENT, document: FPDF_DOCUMENT, contents: *const ::core::ffi::c_void, len: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAttachment_SetFile", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAttachment_SetFile(attachment, document, contents, len) })
    }
    /// Calls `FPDFAttachment_GetFile` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAttachment_GetFile`.
    pub unsafe fn FPDFAttachment_GetFile(&self, attachment: FPDF_ATTACHMENT, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong, out_buflen: *mut ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAttachment_GetFile", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAttachment_GetFile(attachment, buffer, buflen, out_buflen) })
    }
    /// Calls `FPDFAttachment_GetSubtype` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAttachment_GetSubtype`.
    pub unsafe fn FPDFAttachment_GetSubtype(&self, attachment: FPDF_ATTACHMENT, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAttachment_GetSubtype", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAttachment_GetSubtype(attachment, buffer, buflen) })
    }
    /// Calls `FPDFAttachment_GetAFRelationship` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAttachment_GetAFRelationship`.
    pub unsafe fn FPDFAttachment_GetAFRelationship(&self, attachment: FPDF_ATTACHMENT, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAttachment_GetAFRelationship", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAttachment_GetAFRelationship(attachment, buffer, buflen) })
    }
    /// Calls `FPDFCatalog_IsTagged` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFCatalog_IsTagged`.
    pub unsafe fn FPDFCatalog_IsTagged(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFCatalog_IsTagged", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFCatalog_IsTagged(document) })
    }
    /// Calls `FPDFCatalog_GetLanguage` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFCatalog_GetLanguage`.
    pub unsafe fn FPDFCatalog_GetLanguage(&self, document: FPDF_DOCUMENT, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFCatalog_GetLanguage", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFCatalog_GetLanguage(document, buffer, buflen) })
    }
    /// Calls `FPDFCatalog_SetLanguage` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFCatalog_SetLanguage`.
    pub unsafe fn FPDFCatalog_SetLanguage(&self, document: FPDF_DOCUMENT, language: FPDF_WIDESTRING) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFCatalog_SetLanguage", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFCatalog_SetLanguage(document, language) })
    }
    /// Calls `FPDFAvail_Create` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAvail_Create`.
    pub unsafe fn FPDFAvail_Create(&self, file_avail: *mut FX_FILEAVAIL, file: *mut FPDF_FILEACCESS) -> Result<crate::PdfiumCall<FPDF_AVAIL>, crate::PdfiumError> {
        crate::runtime::call("FPDFAvail_Create", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAvail_Create(file_avail, file) })
    }
    /// Calls `FPDFAvail_Destroy` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAvail_Destroy`.
    pub unsafe fn FPDFAvail_Destroy(&self, avail: FPDF_AVAIL) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFAvail_Destroy", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAvail_Destroy(avail) })
    }
    /// Calls `FPDFAvail_IsDocAvail` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAvail_IsDocAvail`.
    pub unsafe fn FPDFAvail_IsDocAvail(&self, avail: FPDF_AVAIL, hints: *mut FX_DOWNLOADHINTS) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFAvail_IsDocAvail", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAvail_IsDocAvail(avail, hints) })
    }
    /// Calls `FPDFAvail_GetDocument` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAvail_GetDocument`.
    pub unsafe fn FPDFAvail_GetDocument(&self, avail: FPDF_AVAIL, password: FPDF_BYTESTRING) -> Result<crate::PdfiumCall<FPDF_DOCUMENT>, crate::PdfiumError> {
        crate::runtime::call("FPDFAvail_GetDocument", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAvail_GetDocument(avail, password) })
    }
    /// Calls `FPDFAvail_GetFirstPageNum` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAvail_GetFirstPageNum`.
    pub unsafe fn FPDFAvail_GetFirstPageNum(&self, doc: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFAvail_GetFirstPageNum", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAvail_GetFirstPageNum(doc) })
    }
    /// Calls `FPDFAvail_IsPageAvail` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAvail_IsPageAvail`.
    pub unsafe fn FPDFAvail_IsPageAvail(&self, avail: FPDF_AVAIL, page_index: ::core::ffi::c_int, hints: *mut FX_DOWNLOADHINTS) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFAvail_IsPageAvail", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAvail_IsPageAvail(avail, page_index, hints) })
    }
    /// Calls `FPDFAvail_IsFormAvail` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAvail_IsFormAvail`.
    pub unsafe fn FPDFAvail_IsFormAvail(&self, avail: FPDF_AVAIL, hints: *mut FX_DOWNLOADHINTS) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFAvail_IsFormAvail", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAvail_IsFormAvail(avail, hints) })
    }
    /// Calls `FPDFAvail_IsLinearized` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAvail_IsLinearized`.
    pub unsafe fn FPDFAvail_IsLinearized(&self, avail: FPDF_AVAIL) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFAvail_IsLinearized", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAvail_IsLinearized(avail) })
    }
    /// Calls `FPDFBookmark_GetFirstChild` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFBookmark_GetFirstChild`.
    pub unsafe fn FPDFBookmark_GetFirstChild(&self, document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> Result<crate::PdfiumCall<FPDF_BOOKMARK>, crate::PdfiumError> {
        crate::runtime::call("FPDFBookmark_GetFirstChild", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFBookmark_GetFirstChild(document, bookmark) })
    }
    /// Calls `FPDFBookmark_GetNextSibling` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFBookmark_GetNextSibling`.
    pub unsafe fn FPDFBookmark_GetNextSibling(&self, document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> Result<crate::PdfiumCall<FPDF_BOOKMARK>, crate::PdfiumError> {
        crate::runtime::call("FPDFBookmark_GetNextSibling", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFBookmark_GetNextSibling(document, bookmark) })
    }
    /// Calls `FPDFBookmark_GetTitle` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFBookmark_GetTitle`.
    pub unsafe fn FPDFBookmark_GetTitle(&self, bookmark: FPDF_BOOKMARK, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFBookmark_GetTitle", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFBookmark_GetTitle(bookmark, buffer, buflen) })
    }
    /// Calls `FPDFBookmark_GetCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFBookmark_GetCount`.
    pub unsafe fn FPDFBookmark_GetCount(&self, bookmark: FPDF_BOOKMARK) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFBookmark_GetCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFBookmark_GetCount(bookmark) })
    }
    /// Calls `FPDFBookmark_Find` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFBookmark_Find`.
    pub unsafe fn FPDFBookmark_Find(&self, document: FPDF_DOCUMENT, title: FPDF_WIDESTRING) -> Result<crate::PdfiumCall<FPDF_BOOKMARK>, crate::PdfiumError> {
        crate::runtime::call("FPDFBookmark_Find", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFBookmark_Find(document, title) })
    }
    /// Calls `FPDFBookmark_GetDest` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFBookmark_GetDest`.
    pub unsafe fn FPDFBookmark_GetDest(&self, document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> Result<crate::PdfiumCall<FPDF_DEST>, crate::PdfiumError> {
        crate::runtime::call("FPDFBookmark_GetDest", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFBookmark_GetDest(document, bookmark) })
    }
    /// Calls `FPDFBookmark_GetAction` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFBookmark_GetAction`.
    pub unsafe fn FPDFBookmark_GetAction(&self, bookmark: FPDF_BOOKMARK) -> Result<crate::PdfiumCall<FPDF_ACTION>, crate::PdfiumError> {
        crate::runtime::call("FPDFBookmark_GetAction", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFBookmark_GetAction(bookmark) })
    }
    /// Calls `FPDFBookmark_GetColor` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFBookmark_GetColor`.
    pub unsafe fn FPDFBookmark_GetColor(&self, bookmark: FPDF_BOOKMARK, R: *mut f32, G: *mut f32, B: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFBookmark_GetColor", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFBookmark_GetColor(bookmark, R, G, B) })
    }
    /// Calls `FPDFBookmark_GetStyle` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFBookmark_GetStyle`.
    pub unsafe fn FPDFBookmark_GetStyle(&self, bookmark: FPDF_BOOKMARK) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFBookmark_GetStyle", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFBookmark_GetStyle(bookmark) })
    }
    /// Calls `FPDFAction_GetType` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAction_GetType`.
    pub unsafe fn FPDFAction_GetType(&self, action: FPDF_ACTION) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAction_GetType", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAction_GetType(action) })
    }
    /// Calls `FPDFAction_GetDest` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAction_GetDest`.
    pub unsafe fn FPDFAction_GetDest(&self, document: FPDF_DOCUMENT, action: FPDF_ACTION) -> Result<crate::PdfiumCall<FPDF_DEST>, crate::PdfiumError> {
        crate::runtime::call("FPDFAction_GetDest", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAction_GetDest(document, action) })
    }
    /// Calls `FPDFAction_GetFilePath` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAction_GetFilePath`.
    pub unsafe fn FPDFAction_GetFilePath(&self, action: FPDF_ACTION, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAction_GetFilePath", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAction_GetFilePath(action, buffer, buflen) })
    }
    /// Calls `FPDFAction_GetURIPath` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFAction_GetURIPath`.
    pub unsafe fn FPDFAction_GetURIPath(&self, document: FPDF_DOCUMENT, action: FPDF_ACTION, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFAction_GetURIPath", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFAction_GetURIPath(document, action, buffer, buflen) })
    }
    /// Calls `FPDFDest_GetDestPageIndex` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFDest_GetDestPageIndex`.
    pub unsafe fn FPDFDest_GetDestPageIndex(&self, document: FPDF_DOCUMENT, dest: FPDF_DEST) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFDest_GetDestPageIndex", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFDest_GetDestPageIndex(document, dest) })
    }
    /// Calls `FPDFDest_GetView` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFDest_GetView`.
    pub unsafe fn FPDFDest_GetView(&self, dest: FPDF_DEST, pNumParams: *mut ::core::ffi::c_ulong, pParams: *mut FS_FLOAT) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFDest_GetView", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFDest_GetView(dest, pNumParams, pParams) })
    }
    /// Calls `FPDFDest_GetLocationInPage` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFDest_GetLocationInPage`.
    pub unsafe fn FPDFDest_GetLocationInPage(&self, dest: FPDF_DEST, hasXVal: *mut FPDF_BOOL, hasYVal: *mut FPDF_BOOL, hasZoomVal: *mut FPDF_BOOL, x: *mut FS_FLOAT, y: *mut FS_FLOAT, zoom: *mut FS_FLOAT) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFDest_GetLocationInPage", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFDest_GetLocationInPage(dest, hasXVal, hasYVal, hasZoomVal, x, y, zoom) })
    }
    /// Calls `FPDFLink_GetLinkAtPoint` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFLink_GetLinkAtPoint`.
    pub unsafe fn FPDFLink_GetLinkAtPoint(&self, page: FPDF_PAGE, x: f64, y: f64) -> Result<crate::PdfiumCall<FPDF_LINK>, crate::PdfiumError> {
        crate::runtime::call("FPDFLink_GetLinkAtPoint", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFLink_GetLinkAtPoint(page, x, y) })
    }
    /// Calls `FPDFLink_GetLinkZOrderAtPoint` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFLink_GetLinkZOrderAtPoint`.
    pub unsafe fn FPDFLink_GetLinkZOrderAtPoint(&self, page: FPDF_PAGE, x: f64, y: f64) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFLink_GetLinkZOrderAtPoint", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFLink_GetLinkZOrderAtPoint(page, x, y) })
    }
    /// Calls `FPDFLink_GetDest` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFLink_GetDest`.
    pub unsafe fn FPDFLink_GetDest(&self, document: FPDF_DOCUMENT, link: FPDF_LINK) -> Result<crate::PdfiumCall<FPDF_DEST>, crate::PdfiumError> {
        crate::runtime::call("FPDFLink_GetDest", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFLink_GetDest(document, link) })
    }
    /// Calls `FPDFLink_GetAction` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFLink_GetAction`.
    pub unsafe fn FPDFLink_GetAction(&self, link: FPDF_LINK) -> Result<crate::PdfiumCall<FPDF_ACTION>, crate::PdfiumError> {
        crate::runtime::call("FPDFLink_GetAction", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFLink_GetAction(link) })
    }
    /// Calls `FPDFLink_Enumerate` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFLink_Enumerate`.
    pub unsafe fn FPDFLink_Enumerate(&self, page: FPDF_PAGE, start_pos: *mut ::core::ffi::c_int, link_annot: *mut FPDF_LINK) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFLink_Enumerate", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFLink_Enumerate(page, start_pos, link_annot) })
    }
    /// Calls `FPDFLink_GetAnnot` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFLink_GetAnnot`.
    pub unsafe fn FPDFLink_GetAnnot(&self, page: FPDF_PAGE, link_annot: FPDF_LINK) -> Result<crate::PdfiumCall<FPDF_ANNOTATION>, crate::PdfiumError> {
        crate::runtime::call("FPDFLink_GetAnnot", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFLink_GetAnnot(page, link_annot) })
    }
    /// Calls `FPDFLink_GetAnnotRect` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFLink_GetAnnotRect`.
    pub unsafe fn FPDFLink_GetAnnotRect(&self, link_annot: FPDF_LINK, rect: *mut FS_RECTF) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFLink_GetAnnotRect", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFLink_GetAnnotRect(link_annot, rect) })
    }
    /// Calls `FPDFLink_CountQuadPoints` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFLink_CountQuadPoints`.
    pub unsafe fn FPDFLink_CountQuadPoints(&self, link_annot: FPDF_LINK) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFLink_CountQuadPoints", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFLink_CountQuadPoints(link_annot) })
    }
    /// Calls `FPDFLink_GetQuadPoints` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFLink_GetQuadPoints`.
    pub unsafe fn FPDFLink_GetQuadPoints(&self, link_annot: FPDF_LINK, quad_index: ::core::ffi::c_int, quad_points: *mut FS_QUADPOINTSF) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFLink_GetQuadPoints", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFLink_GetQuadPoints(link_annot, quad_index, quad_points) })
    }
    /// Calls `FPDF_GetPageAAction` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetPageAAction`.
    pub unsafe fn FPDF_GetPageAAction(&self, page: FPDF_PAGE, aa_type: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_ACTION>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetPageAAction", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetPageAAction(page, aa_type) })
    }
    /// Calls `FPDF_GetFileIdentifier` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetFileIdentifier`.
    pub unsafe fn FPDF_GetFileIdentifier(&self, document: FPDF_DOCUMENT, id_type: FPDF_FILEIDTYPE, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetFileIdentifier", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetFileIdentifier(document, id_type, buffer, buflen) })
    }
    /// Calls `FPDF_GetMetaText` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetMetaText`.
    pub unsafe fn FPDF_GetMetaText(&self, document: FPDF_DOCUMENT, tag: FPDF_BYTESTRING, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetMetaText", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetMetaText(document, tag, buffer, buflen) })
    }
    /// Calls `FPDF_GetPageLabel` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetPageLabel`.
    pub unsafe fn FPDF_GetPageLabel(&self, document: FPDF_DOCUMENT, page_index: ::core::ffi::c_int, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetPageLabel", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetPageLabel(document, page_index, buffer, buflen) })
    }
    /// Calls `FPDF_CreateNewDocument` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_CreateNewDocument`.
    pub unsafe fn FPDF_CreateNewDocument(&self) -> Result<crate::PdfiumCall<FPDF_DOCUMENT>, crate::PdfiumError> {
        crate::runtime::call("FPDF_CreateNewDocument", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_CreateNewDocument() })
    }
    /// Calls `FPDFPage_New` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_New`.
    pub unsafe fn FPDFPage_New(&self, document: FPDF_DOCUMENT, page_index: ::core::ffi::c_int, width: f64, height: f64) -> Result<crate::PdfiumCall<FPDF_PAGE>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_New", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_New(document, page_index, width, height) })
    }
    /// Calls `FPDFPage_Delete` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_Delete`.
    pub unsafe fn FPDFPage_Delete(&self, document: FPDF_DOCUMENT, page_index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_Delete", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_Delete(document, page_index) })
    }
    /// Calls `FPDF_MovePages` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_MovePages`.
    pub unsafe fn FPDF_MovePages(&self, document: FPDF_DOCUMENT, page_indices: *const ::core::ffi::c_int, page_indices_len: ::core::ffi::c_ulong, dest_page_index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_MovePages", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_MovePages(document, page_indices, page_indices_len, dest_page_index) })
    }
    /// Calls `FPDFPage_GetRotation` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_GetRotation`.
    pub unsafe fn FPDFPage_GetRotation(&self, page: FPDF_PAGE) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_GetRotation", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_GetRotation(page) })
    }
    /// Calls `FPDFPage_SetRotation` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_SetRotation`.
    pub unsafe fn FPDFPage_SetRotation(&self, page: FPDF_PAGE, rotate: ::core::ffi::c_int) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_SetRotation", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_SetRotation(page, rotate) })
    }
    /// Calls `FPDFPage_InsertObject` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_InsertObject`.
    pub unsafe fn FPDFPage_InsertObject(&self, page: FPDF_PAGE, page_object: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_InsertObject", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_InsertObject(page, page_object) })
    }
    /// Calls `FPDFPage_InsertObjectAtIndex` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_InsertObjectAtIndex`.
    pub unsafe fn FPDFPage_InsertObjectAtIndex(&self, page: FPDF_PAGE, page_object: FPDF_PAGEOBJECT, index: usize) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_InsertObjectAtIndex", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_InsertObjectAtIndex(page, page_object, index) })
    }
    /// Calls `FPDFPage_RemoveObject` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_RemoveObject`.
    pub unsafe fn FPDFPage_RemoveObject(&self, page: FPDF_PAGE, page_object: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_RemoveObject", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_RemoveObject(page, page_object) })
    }
    /// Calls `FPDFPage_CountObjects` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_CountObjects`.
    pub unsafe fn FPDFPage_CountObjects(&self, page: FPDF_PAGE) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_CountObjects", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_CountObjects(page) })
    }
    /// Calls `FPDFPage_GetObject` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_GetObject`.
    pub unsafe fn FPDFPage_GetObject(&self, page: FPDF_PAGE, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_PAGEOBJECT>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_GetObject", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_GetObject(page, index) })
    }
    /// Calls `FPDFPage_HasTransparency` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_HasTransparency`.
    pub unsafe fn FPDFPage_HasTransparency(&self, page: FPDF_PAGE) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_HasTransparency", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_HasTransparency(page) })
    }
    /// Calls `FPDFPage_GenerateContent` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_GenerateContent`.
    pub unsafe fn FPDFPage_GenerateContent(&self, page: FPDF_PAGE) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_GenerateContent", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_GenerateContent(page) })
    }
    /// Calls `FPDFPageObj_Destroy` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_Destroy`.
    pub unsafe fn FPDFPageObj_Destroy(&self, page_object: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_Destroy", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_Destroy(page_object) })
    }
    /// Calls `FPDFPageObj_HasTransparency` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_HasTransparency`.
    pub unsafe fn FPDFPageObj_HasTransparency(&self, page_object: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_HasTransparency", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_HasTransparency(page_object) })
    }
    /// Calls `FPDFPageObj_GetType` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_GetType`.
    pub unsafe fn FPDFPageObj_GetType(&self, page_object: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_GetType", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_GetType(page_object) })
    }
    /// Calls `FPDFPageObj_GetIsActive` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_GetIsActive`.
    pub unsafe fn FPDFPageObj_GetIsActive(&self, page_object: FPDF_PAGEOBJECT, active: *mut FPDF_BOOL) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_GetIsActive", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_GetIsActive(page_object, active) })
    }
    /// Calls `FPDFPageObj_SetIsActive` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_SetIsActive`.
    pub unsafe fn FPDFPageObj_SetIsActive(&self, page_object: FPDF_PAGEOBJECT, active: FPDF_BOOL) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_SetIsActive", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_SetIsActive(page_object, active) })
    }
    /// Calls `FPDFPageObj_Transform` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_Transform`.
    pub unsafe fn FPDFPageObj_Transform(&self, page_object: FPDF_PAGEOBJECT, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_Transform", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_Transform(page_object, a, b, c, d, e, f) })
    }
    /// Calls `FPDFPageObj_TransformF` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_TransformF`.
    pub unsafe fn FPDFPageObj_TransformF(&self, page_object: FPDF_PAGEOBJECT, matrix: *const FS_MATRIX) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_TransformF", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_TransformF(page_object, matrix) })
    }
    /// Calls `FPDFPageObj_GetMatrix` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_GetMatrix`.
    pub unsafe fn FPDFPageObj_GetMatrix(&self, page_object: FPDF_PAGEOBJECT, matrix: *mut FS_MATRIX) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_GetMatrix", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_GetMatrix(page_object, matrix) })
    }
    /// Calls `FPDFPageObj_SetMatrix` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_SetMatrix`.
    pub unsafe fn FPDFPageObj_SetMatrix(&self, page_object: FPDF_PAGEOBJECT, matrix: *const FS_MATRIX) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_SetMatrix", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_SetMatrix(page_object, matrix) })
    }
    /// Calls `FPDFPage_TransformAnnots` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_TransformAnnots`.
    pub unsafe fn FPDFPage_TransformAnnots(&self, page: FPDF_PAGE, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_TransformAnnots", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_TransformAnnots(page, a, b, c, d, e, f) })
    }
    /// Calls `FPDFPageObj_NewImageObj` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_NewImageObj`.
    pub unsafe fn FPDFPageObj_NewImageObj(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<FPDF_PAGEOBJECT>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_NewImageObj", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_NewImageObj(document) })
    }
    /// Calls `FPDFPageObj_GetMarkedContentID` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_GetMarkedContentID`.
    pub unsafe fn FPDFPageObj_GetMarkedContentID(&self, page_object: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_GetMarkedContentID", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_GetMarkedContentID(page_object) })
    }
    /// Calls `FPDFPageObj_CountMarks` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_CountMarks`.
    pub unsafe fn FPDFPageObj_CountMarks(&self, page_object: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_CountMarks", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_CountMarks(page_object) })
    }
    /// Calls `FPDFPageObj_GetMark` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_GetMark`.
    pub unsafe fn FPDFPageObj_GetMark(&self, page_object: FPDF_PAGEOBJECT, index: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<FPDF_PAGEOBJECTMARK>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_GetMark", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_GetMark(page_object, index) })
    }
    /// Calls `FPDFPageObj_AddMark` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_AddMark`.
    pub unsafe fn FPDFPageObj_AddMark(&self, page_object: FPDF_PAGEOBJECT, name: FPDF_BYTESTRING) -> Result<crate::PdfiumCall<FPDF_PAGEOBJECTMARK>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_AddMark", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_AddMark(page_object, name) })
    }
    /// Calls `FPDFPageObj_AddExistingMark` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_AddExistingMark`.
    pub unsafe fn FPDFPageObj_AddExistingMark(&self, page_object: FPDF_PAGEOBJECT, mark: FPDF_PAGEOBJECTMARK) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_AddExistingMark", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_AddExistingMark(page_object, mark) })
    }
    /// Calls `FPDFPageObj_RemoveMark` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_RemoveMark`.
    pub unsafe fn FPDFPageObj_RemoveMark(&self, page_object: FPDF_PAGEOBJECT, mark: FPDF_PAGEOBJECTMARK) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_RemoveMark", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_RemoveMark(page_object, mark) })
    }
    /// Calls `FPDFPageObjMark_GetName` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObjMark_GetName`.
    pub unsafe fn FPDFPageObjMark_GetName(&self, mark: FPDF_PAGEOBJECTMARK, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong, out_buflen: *mut ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObjMark_GetName", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObjMark_GetName(mark, buffer, buflen, out_buflen) })
    }
    /// Calls `FPDFPageObjMark_CountParams` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObjMark_CountParams`.
    pub unsafe fn FPDFPageObjMark_CountParams(&self, mark: FPDF_PAGEOBJECTMARK) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObjMark_CountParams", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObjMark_CountParams(mark) })
    }
    /// Calls `FPDFPageObjMark_GetParamKey` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObjMark_GetParamKey`.
    pub unsafe fn FPDFPageObjMark_GetParamKey(&self, mark: FPDF_PAGEOBJECTMARK, index: ::core::ffi::c_ulong, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong, out_buflen: *mut ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObjMark_GetParamKey", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObjMark_GetParamKey(mark, index, buffer, buflen, out_buflen) })
    }
    /// Calls `FPDFPageObjMark_GetParamValueType` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObjMark_GetParamValueType`.
    pub unsafe fn FPDFPageObjMark_GetParamValueType(&self, mark: FPDF_PAGEOBJECTMARK, key: FPDF_BYTESTRING) -> Result<crate::PdfiumCall<FPDF_OBJECT_TYPE>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObjMark_GetParamValueType", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObjMark_GetParamValueType(mark, key) })
    }
    /// Calls `FPDFPageObjMark_GetParamIntValue` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObjMark_GetParamIntValue`.
    pub unsafe fn FPDFPageObjMark_GetParamIntValue(&self, mark: FPDF_PAGEOBJECTMARK, key: FPDF_BYTESTRING, out_value: *mut ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObjMark_GetParamIntValue", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObjMark_GetParamIntValue(mark, key, out_value) })
    }
    /// Calls `FPDFPageObjMark_GetParamFloatValue` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObjMark_GetParamFloatValue`.
    pub unsafe fn FPDFPageObjMark_GetParamFloatValue(&self, mark: FPDF_PAGEOBJECTMARK, key: FPDF_BYTESTRING, out_value: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObjMark_GetParamFloatValue", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObjMark_GetParamFloatValue(mark, key, out_value) })
    }
    /// Calls `FPDFPageObjMark_GetParamStringValue` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObjMark_GetParamStringValue`.
    pub unsafe fn FPDFPageObjMark_GetParamStringValue(&self, mark: FPDF_PAGEOBJECTMARK, key: FPDF_BYTESTRING, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong, out_buflen: *mut ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObjMark_GetParamStringValue", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObjMark_GetParamStringValue(mark, key, buffer, buflen, out_buflen) })
    }
    /// Calls `FPDFPageObjMark_GetParamBlobValue` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObjMark_GetParamBlobValue`.
    pub unsafe fn FPDFPageObjMark_GetParamBlobValue(&self, mark: FPDF_PAGEOBJECTMARK, key: FPDF_BYTESTRING, buffer: *mut ::core::ffi::c_uchar, buflen: ::core::ffi::c_ulong, out_buflen: *mut ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObjMark_GetParamBlobValue", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObjMark_GetParamBlobValue(mark, key, buffer, buflen, out_buflen) })
    }
    /// Calls `FPDFPageObjMark_SetIntParam` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObjMark_SetIntParam`.
    pub unsafe fn FPDFPageObjMark_SetIntParam(&self, document: FPDF_DOCUMENT, page_object: FPDF_PAGEOBJECT, mark: FPDF_PAGEOBJECTMARK, key: FPDF_BYTESTRING, value: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObjMark_SetIntParam", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObjMark_SetIntParam(document, page_object, mark, key, value) })
    }
    /// Calls `FPDFPageObjMark_SetFloatParam` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObjMark_SetFloatParam`.
    pub unsafe fn FPDFPageObjMark_SetFloatParam(&self, document: FPDF_DOCUMENT, page_object: FPDF_PAGEOBJECT, mark: FPDF_PAGEOBJECTMARK, key: FPDF_BYTESTRING, value: f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObjMark_SetFloatParam", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObjMark_SetFloatParam(document, page_object, mark, key, value) })
    }
    /// Calls `FPDFPageObjMark_SetStringParam` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObjMark_SetStringParam`.
    pub unsafe fn FPDFPageObjMark_SetStringParam(&self, document: FPDF_DOCUMENT, page_object: FPDF_PAGEOBJECT, mark: FPDF_PAGEOBJECTMARK, key: FPDF_BYTESTRING, value: FPDF_BYTESTRING) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObjMark_SetStringParam", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObjMark_SetStringParam(document, page_object, mark, key, value) })
    }
    /// Calls `FPDFPageObjMark_SetBlobParam` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObjMark_SetBlobParam`.
    pub unsafe fn FPDFPageObjMark_SetBlobParam(&self, document: FPDF_DOCUMENT, page_object: FPDF_PAGEOBJECT, mark: FPDF_PAGEOBJECTMARK, key: FPDF_BYTESTRING, value: *const ::core::ffi::c_uchar, value_len: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObjMark_SetBlobParam", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObjMark_SetBlobParam(document, page_object, mark, key, value, value_len) })
    }
    /// Calls `FPDFPageObjMark_RemoveParam` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObjMark_RemoveParam`.
    pub unsafe fn FPDFPageObjMark_RemoveParam(&self, page_object: FPDF_PAGEOBJECT, mark: FPDF_PAGEOBJECTMARK, key: FPDF_BYTESTRING) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObjMark_RemoveParam", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObjMark_RemoveParam(page_object, mark, key) })
    }
    /// Calls `FPDFImageObj_LoadJpegFile` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFImageObj_LoadJpegFile`.
    pub unsafe fn FPDFImageObj_LoadJpegFile(&self, pages: *mut FPDF_PAGE, count: ::core::ffi::c_int, image_object: FPDF_PAGEOBJECT, file_access: *mut FPDF_FILEACCESS) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFImageObj_LoadJpegFile", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFImageObj_LoadJpegFile(pages, count, image_object, file_access) })
    }
    /// Calls `FPDFImageObj_LoadJpegFileInline` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFImageObj_LoadJpegFileInline`.
    pub unsafe fn FPDFImageObj_LoadJpegFileInline(&self, pages: *mut FPDF_PAGE, count: ::core::ffi::c_int, image_object: FPDF_PAGEOBJECT, file_access: *mut FPDF_FILEACCESS) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFImageObj_LoadJpegFileInline", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFImageObj_LoadJpegFileInline(pages, count, image_object, file_access) })
    }
    /// Calls `FPDFImageObj_SetMatrix` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFImageObj_SetMatrix`.
    pub unsafe fn FPDFImageObj_SetMatrix(&self, image_object: FPDF_PAGEOBJECT, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFImageObj_SetMatrix", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFImageObj_SetMatrix(image_object, a, b, c, d, e, f) })
    }
    /// Calls `FPDFImageObj_SetBitmap` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFImageObj_SetBitmap`.
    pub unsafe fn FPDFImageObj_SetBitmap(&self, pages: *mut FPDF_PAGE, count: ::core::ffi::c_int, image_object: FPDF_PAGEOBJECT, bitmap: FPDF_BITMAP) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFImageObj_SetBitmap", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFImageObj_SetBitmap(pages, count, image_object, bitmap) })
    }
    /// Calls `FPDFImageObj_GetBitmap` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFImageObj_GetBitmap`.
    pub unsafe fn FPDFImageObj_GetBitmap(&self, image_object: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<FPDF_BITMAP>, crate::PdfiumError> {
        crate::runtime::call("FPDFImageObj_GetBitmap", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFImageObj_GetBitmap(image_object) })
    }
    /// Calls `FPDFImageObj_GetRenderedBitmap` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFImageObj_GetRenderedBitmap`.
    pub unsafe fn FPDFImageObj_GetRenderedBitmap(&self, document: FPDF_DOCUMENT, page: FPDF_PAGE, image_object: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<FPDF_BITMAP>, crate::PdfiumError> {
        crate::runtime::call("FPDFImageObj_GetRenderedBitmap", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFImageObj_GetRenderedBitmap(document, page, image_object) })
    }
    /// Calls `FPDFImageObj_GetImageDataDecoded` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFImageObj_GetImageDataDecoded`.
    pub unsafe fn FPDFImageObj_GetImageDataDecoded(&self, image_object: FPDF_PAGEOBJECT, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFImageObj_GetImageDataDecoded", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFImageObj_GetImageDataDecoded(image_object, buffer, buflen) })
    }
    /// Calls `FPDFImageObj_GetImageDataRaw` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFImageObj_GetImageDataRaw`.
    pub unsafe fn FPDFImageObj_GetImageDataRaw(&self, image_object: FPDF_PAGEOBJECT, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFImageObj_GetImageDataRaw", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFImageObj_GetImageDataRaw(image_object, buffer, buflen) })
    }
    /// Calls `FPDFImageObj_GetImageFilterCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFImageObj_GetImageFilterCount`.
    pub unsafe fn FPDFImageObj_GetImageFilterCount(&self, image_object: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFImageObj_GetImageFilterCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFImageObj_GetImageFilterCount(image_object) })
    }
    /// Calls `FPDFImageObj_GetImageFilter` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFImageObj_GetImageFilter`.
    pub unsafe fn FPDFImageObj_GetImageFilter(&self, image_object: FPDF_PAGEOBJECT, index: ::core::ffi::c_int, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFImageObj_GetImageFilter", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFImageObj_GetImageFilter(image_object, index, buffer, buflen) })
    }
    /// Calls `FPDFImageObj_GetImageMetadata` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFImageObj_GetImageMetadata`.
    pub unsafe fn FPDFImageObj_GetImageMetadata(&self, image_object: FPDF_PAGEOBJECT, page: FPDF_PAGE, metadata: *mut FPDF_IMAGEOBJ_METADATA) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFImageObj_GetImageMetadata", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFImageObj_GetImageMetadata(image_object, page, metadata) })
    }
    /// Calls `FPDFImageObj_GetImagePixelSize` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFImageObj_GetImagePixelSize`.
    pub unsafe fn FPDFImageObj_GetImagePixelSize(&self, image_object: FPDF_PAGEOBJECT, width: *mut ::core::ffi::c_uint, height: *mut ::core::ffi::c_uint) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFImageObj_GetImagePixelSize", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFImageObj_GetImagePixelSize(image_object, width, height) })
    }
    /// Calls `FPDFImageObj_GetIccProfileDataDecoded` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFImageObj_GetIccProfileDataDecoded`.
    pub unsafe fn FPDFImageObj_GetIccProfileDataDecoded(&self, image_object: FPDF_PAGEOBJECT, page: FPDF_PAGE, buffer: *mut u8, buflen: usize, out_buflen: *mut usize) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFImageObj_GetIccProfileDataDecoded", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFImageObj_GetIccProfileDataDecoded(image_object, page, buffer, buflen, out_buflen) })
    }
    /// Calls `FPDFPageObj_CreateNewPath` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_CreateNewPath`.
    pub unsafe fn FPDFPageObj_CreateNewPath(&self, x: f32, y: f32) -> Result<crate::PdfiumCall<FPDF_PAGEOBJECT>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_CreateNewPath", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_CreateNewPath(x, y) })
    }
    /// Calls `FPDFPageObj_CreateNewRect` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_CreateNewRect`.
    pub unsafe fn FPDFPageObj_CreateNewRect(&self, x: f32, y: f32, w: f32, h: f32) -> Result<crate::PdfiumCall<FPDF_PAGEOBJECT>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_CreateNewRect", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_CreateNewRect(x, y, w, h) })
    }
    /// Calls `FPDFPageObj_GetBounds` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_GetBounds`.
    pub unsafe fn FPDFPageObj_GetBounds(&self, page_object: FPDF_PAGEOBJECT, left: *mut f32, bottom: *mut f32, right: *mut f32, top: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_GetBounds", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_GetBounds(page_object, left, bottom, right, top) })
    }
    /// Calls `FPDFPageObj_GetRotatedBounds` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_GetRotatedBounds`.
    pub unsafe fn FPDFPageObj_GetRotatedBounds(&self, page_object: FPDF_PAGEOBJECT, quad_points: *mut FS_QUADPOINTSF) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_GetRotatedBounds", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_GetRotatedBounds(page_object, quad_points) })
    }
    /// Calls `FPDFPageObj_SetBlendMode` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_SetBlendMode`.
    pub unsafe fn FPDFPageObj_SetBlendMode(&self, page_object: FPDF_PAGEOBJECT, blend_mode: FPDF_BYTESTRING) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_SetBlendMode", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_SetBlendMode(page_object, blend_mode) })
    }
    /// Calls `FPDFPageObj_SetStrokeColor` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_SetStrokeColor`.
    pub unsafe fn FPDFPageObj_SetStrokeColor(&self, page_object: FPDF_PAGEOBJECT, R: ::core::ffi::c_uint, G: ::core::ffi::c_uint, B: ::core::ffi::c_uint, A: ::core::ffi::c_uint) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_SetStrokeColor", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_SetStrokeColor(page_object, R, G, B, A) })
    }
    /// Calls `FPDFPageObj_GetStrokeColor` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_GetStrokeColor`.
    pub unsafe fn FPDFPageObj_GetStrokeColor(&self, page_object: FPDF_PAGEOBJECT, R: *mut ::core::ffi::c_uint, G: *mut ::core::ffi::c_uint, B: *mut ::core::ffi::c_uint, A: *mut ::core::ffi::c_uint) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_GetStrokeColor", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_GetStrokeColor(page_object, R, G, B, A) })
    }
    /// Calls `FPDFPageObj_SetStrokeWidth` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_SetStrokeWidth`.
    pub unsafe fn FPDFPageObj_SetStrokeWidth(&self, page_object: FPDF_PAGEOBJECT, width: f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_SetStrokeWidth", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_SetStrokeWidth(page_object, width) })
    }
    /// Calls `FPDFPageObj_GetStrokeWidth` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_GetStrokeWidth`.
    pub unsafe fn FPDFPageObj_GetStrokeWidth(&self, page_object: FPDF_PAGEOBJECT, width: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_GetStrokeWidth", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_GetStrokeWidth(page_object, width) })
    }
    /// Calls `FPDFPageObj_GetLineJoin` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_GetLineJoin`.
    pub unsafe fn FPDFPageObj_GetLineJoin(&self, page_object: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_GetLineJoin", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_GetLineJoin(page_object) })
    }
    /// Calls `FPDFPageObj_SetLineJoin` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_SetLineJoin`.
    pub unsafe fn FPDFPageObj_SetLineJoin(&self, page_object: FPDF_PAGEOBJECT, line_join: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_SetLineJoin", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_SetLineJoin(page_object, line_join) })
    }
    /// Calls `FPDFPageObj_GetLineCap` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_GetLineCap`.
    pub unsafe fn FPDFPageObj_GetLineCap(&self, page_object: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_GetLineCap", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_GetLineCap(page_object) })
    }
    /// Calls `FPDFPageObj_SetLineCap` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_SetLineCap`.
    pub unsafe fn FPDFPageObj_SetLineCap(&self, page_object: FPDF_PAGEOBJECT, line_cap: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_SetLineCap", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_SetLineCap(page_object, line_cap) })
    }
    /// Calls `FPDFPageObj_SetFillColor` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_SetFillColor`.
    pub unsafe fn FPDFPageObj_SetFillColor(&self, page_object: FPDF_PAGEOBJECT, R: ::core::ffi::c_uint, G: ::core::ffi::c_uint, B: ::core::ffi::c_uint, A: ::core::ffi::c_uint) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_SetFillColor", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_SetFillColor(page_object, R, G, B, A) })
    }
    /// Calls `FPDFPageObj_GetFillColor` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_GetFillColor`.
    pub unsafe fn FPDFPageObj_GetFillColor(&self, page_object: FPDF_PAGEOBJECT, R: *mut ::core::ffi::c_uint, G: *mut ::core::ffi::c_uint, B: *mut ::core::ffi::c_uint, A: *mut ::core::ffi::c_uint) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_GetFillColor", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_GetFillColor(page_object, R, G, B, A) })
    }
    /// Calls `FPDFPageObj_GetDashPhase` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_GetDashPhase`.
    pub unsafe fn FPDFPageObj_GetDashPhase(&self, page_object: FPDF_PAGEOBJECT, phase: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_GetDashPhase", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_GetDashPhase(page_object, phase) })
    }
    /// Calls `FPDFPageObj_SetDashPhase` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_SetDashPhase`.
    pub unsafe fn FPDFPageObj_SetDashPhase(&self, page_object: FPDF_PAGEOBJECT, phase: f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_SetDashPhase", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_SetDashPhase(page_object, phase) })
    }
    /// Calls `FPDFPageObj_GetDashCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_GetDashCount`.
    pub unsafe fn FPDFPageObj_GetDashCount(&self, page_object: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_GetDashCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_GetDashCount(page_object) })
    }
    /// Calls `FPDFPageObj_GetDashArray` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_GetDashArray`.
    pub unsafe fn FPDFPageObj_GetDashArray(&self, page_object: FPDF_PAGEOBJECT, dash_array: *mut f32, dash_count: usize) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_GetDashArray", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_GetDashArray(page_object, dash_array, dash_count) })
    }
    /// Calls `FPDFPageObj_SetDashArray` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_SetDashArray`.
    pub unsafe fn FPDFPageObj_SetDashArray(&self, page_object: FPDF_PAGEOBJECT, dash_array: *const f32, dash_count: usize, phase: f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_SetDashArray", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_SetDashArray(page_object, dash_array, dash_count, phase) })
    }
    /// Calls `FPDFPageObj_GetRenderedStrokePattern` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_GetRenderedStrokePattern`.
    pub unsafe fn FPDFPageObj_GetRenderedStrokePattern(&self, document: FPDF_DOCUMENT, page_object: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<FPDF_BITMAP>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_GetRenderedStrokePattern", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_GetRenderedStrokePattern(document, page_object) })
    }
    /// Calls `FPDFPageObj_GetRenderedFillPattern` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_GetRenderedFillPattern`.
    pub unsafe fn FPDFPageObj_GetRenderedFillPattern(&self, document: FPDF_DOCUMENT, page_object: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<FPDF_BITMAP>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_GetRenderedFillPattern", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_GetRenderedFillPattern(document, page_object) })
    }
    /// Calls `FPDFPath_CountSegments` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPath_CountSegments`.
    pub unsafe fn FPDFPath_CountSegments(&self, path: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFPath_CountSegments", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPath_CountSegments(path) })
    }
    /// Calls `FPDFPath_GetPathSegment` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPath_GetPathSegment`.
    pub unsafe fn FPDFPath_GetPathSegment(&self, path: FPDF_PAGEOBJECT, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_PATHSEGMENT>, crate::PdfiumError> {
        crate::runtime::call("FPDFPath_GetPathSegment", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPath_GetPathSegment(path, index) })
    }
    /// Calls `FPDFPathSegment_GetPoint` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPathSegment_GetPoint`.
    pub unsafe fn FPDFPathSegment_GetPoint(&self, segment: FPDF_PATHSEGMENT, x: *mut f32, y: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPathSegment_GetPoint", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPathSegment_GetPoint(segment, x, y) })
    }
    /// Calls `FPDFPathSegment_GetType` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPathSegment_GetType`.
    pub unsafe fn FPDFPathSegment_GetType(&self, segment: FPDF_PATHSEGMENT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFPathSegment_GetType", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPathSegment_GetType(segment) })
    }
    /// Calls `FPDFPathSegment_GetClose` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPathSegment_GetClose`.
    pub unsafe fn FPDFPathSegment_GetClose(&self, segment: FPDF_PATHSEGMENT) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPathSegment_GetClose", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPathSegment_GetClose(segment) })
    }
    /// Calls `FPDFPath_MoveTo` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPath_MoveTo`.
    pub unsafe fn FPDFPath_MoveTo(&self, path: FPDF_PAGEOBJECT, x: f32, y: f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPath_MoveTo", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPath_MoveTo(path, x, y) })
    }
    /// Calls `FPDFPath_LineTo` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPath_LineTo`.
    pub unsafe fn FPDFPath_LineTo(&self, path: FPDF_PAGEOBJECT, x: f32, y: f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPath_LineTo", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPath_LineTo(path, x, y) })
    }
    /// Calls `FPDFPath_BezierTo` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPath_BezierTo`.
    pub unsafe fn FPDFPath_BezierTo(&self, path: FPDF_PAGEOBJECT, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPath_BezierTo", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPath_BezierTo(path, x1, y1, x2, y2, x3, y3) })
    }
    /// Calls `FPDFPath_GetBezierControlPoints` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPath_GetBezierControlPoints`.
    pub unsafe fn FPDFPath_GetBezierControlPoints(&self, path: FPDF_PAGEOBJECT, index: usize, first_control_point: *mut FS_POINTF, second_control_point: *mut FS_POINTF) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPath_GetBezierControlPoints", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPath_GetBezierControlPoints(path, index, first_control_point, second_control_point) })
    }
    /// Calls `FPDFPath_Close` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPath_Close`.
    pub unsafe fn FPDFPath_Close(&self, path: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPath_Close", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPath_Close(path) })
    }
    /// Calls `FPDFPath_SetDrawMode` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPath_SetDrawMode`.
    pub unsafe fn FPDFPath_SetDrawMode(&self, path: FPDF_PAGEOBJECT, fillmode: ::core::ffi::c_int, stroke: FPDF_BOOL) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPath_SetDrawMode", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPath_SetDrawMode(path, fillmode, stroke) })
    }
    /// Calls `FPDFPath_GetDrawMode` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPath_GetDrawMode`.
    pub unsafe fn FPDFPath_GetDrawMode(&self, path: FPDF_PAGEOBJECT, fillmode: *mut ::core::ffi::c_int, stroke: *mut FPDF_BOOL) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPath_GetDrawMode", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPath_GetDrawMode(path, fillmode, stroke) })
    }
    /// Calls `FPDFPageObj_NewTextObj` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_NewTextObj`.
    pub unsafe fn FPDFPageObj_NewTextObj(&self, document: FPDF_DOCUMENT, font: FPDF_BYTESTRING, font_size: f32) -> Result<crate::PdfiumCall<FPDF_PAGEOBJECT>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_NewTextObj", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_NewTextObj(document, font, font_size) })
    }
    /// Calls `FPDFText_SetText` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_SetText`.
    pub unsafe fn FPDFText_SetText(&self, text_object: FPDF_PAGEOBJECT, text: FPDF_WIDESTRING) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_SetText", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_SetText(text_object, text) })
    }
    /// Calls `FPDFText_SetCharcodes` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_SetCharcodes`.
    pub unsafe fn FPDFText_SetCharcodes(&self, text_object: FPDF_PAGEOBJECT, charcodes: *const u32, count: usize) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_SetCharcodes", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_SetCharcodes(text_object, charcodes, count) })
    }
    /// Calls `FPDFText_SetPositions` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_SetPositions`.
    pub unsafe fn FPDFText_SetPositions(&self, text_object: FPDF_PAGEOBJECT, positions: *const f32, count: usize) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_SetPositions", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_SetPositions(text_object, positions, count) })
    }
    /// Calls `FPDFText_LoadFont` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_LoadFont`.
    pub unsafe fn FPDFText_LoadFont(&self, document: FPDF_DOCUMENT, data: *const u8, size: u32, font_type: ::core::ffi::c_int, cid: FPDF_BOOL) -> Result<crate::PdfiumCall<FPDF_FONT>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_LoadFont", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_LoadFont(document, data, size, font_type, cid) })
    }
    /// Calls `FPDFText_LoadStandardFont` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_LoadStandardFont`.
    pub unsafe fn FPDFText_LoadStandardFont(&self, document: FPDF_DOCUMENT, font: FPDF_BYTESTRING) -> Result<crate::PdfiumCall<FPDF_FONT>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_LoadStandardFont", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_LoadStandardFont(document, font) })
    }
    /// Calls `FPDFText_LoadCidType2Font` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_LoadCidType2Font`.
    pub unsafe fn FPDFText_LoadCidType2Font(&self, document: FPDF_DOCUMENT, font_data: *const u8, font_data_size: u32, to_unicode_cmap: FPDF_BYTESTRING, cid_to_gid_map_data: *const u8, cid_to_gid_map_data_size: u32) -> Result<crate::PdfiumCall<FPDF_FONT>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_LoadCidType2Font", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_LoadCidType2Font(document, font_data, font_data_size, to_unicode_cmap, cid_to_gid_map_data, cid_to_gid_map_data_size) })
    }
    /// Calls `FPDFTextObj_GetFontSize` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFTextObj_GetFontSize`.
    pub unsafe fn FPDFTextObj_GetFontSize(&self, text: FPDF_PAGEOBJECT, size: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFTextObj_GetFontSize", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFTextObj_GetFontSize(text, size) })
    }
    /// Calls `FPDFTextObj_SetFontSize` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFTextObj_SetFontSize`.
    pub unsafe fn FPDFTextObj_SetFontSize(&self, text: FPDF_PAGEOBJECT, size: f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFTextObj_SetFontSize", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFTextObj_SetFontSize(text, size) })
    }
    /// Calls `FPDFFont_Close` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFFont_Close`.
    pub unsafe fn FPDFFont_Close(&self, font: FPDF_FONT) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFFont_Close", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFFont_Close(font) })
    }
    /// Calls `FPDFPageObj_CreateTextObj` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_CreateTextObj`.
    pub unsafe fn FPDFPageObj_CreateTextObj(&self, document: FPDF_DOCUMENT, font: FPDF_FONT, font_size: f32) -> Result<crate::PdfiumCall<FPDF_PAGEOBJECT>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_CreateTextObj", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_CreateTextObj(document, font, font_size) })
    }
    /// Calls `FPDFTextObj_GetTextRenderMode` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFTextObj_GetTextRenderMode`.
    pub unsafe fn FPDFTextObj_GetTextRenderMode(&self, text: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<FPDF_TEXT_RENDERMODE>, crate::PdfiumError> {
        crate::runtime::call("FPDFTextObj_GetTextRenderMode", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFTextObj_GetTextRenderMode(text) })
    }
    /// Calls `FPDFTextObj_SetTextRenderMode` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFTextObj_SetTextRenderMode`.
    pub unsafe fn FPDFTextObj_SetTextRenderMode(&self, text: FPDF_PAGEOBJECT, render_mode: FPDF_TEXT_RENDERMODE) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFTextObj_SetTextRenderMode", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFTextObj_SetTextRenderMode(text, render_mode) })
    }
    /// Calls `FPDFTextObj_GetText` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFTextObj_GetText`.
    pub unsafe fn FPDFTextObj_GetText(&self, text_object: FPDF_PAGEOBJECT, text_page: FPDF_TEXTPAGE, buffer: *mut FPDF_WCHAR, length: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFTextObj_GetText", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFTextObj_GetText(text_object, text_page, buffer, length) })
    }
    /// Calls `FPDFTextObj_GetRenderedBitmap` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFTextObj_GetRenderedBitmap`.
    pub unsafe fn FPDFTextObj_GetRenderedBitmap(&self, document: FPDF_DOCUMENT, page: FPDF_PAGE, text_object: FPDF_PAGEOBJECT, scale: f32) -> Result<crate::PdfiumCall<FPDF_BITMAP>, crate::PdfiumError> {
        crate::runtime::call("FPDFTextObj_GetRenderedBitmap", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFTextObj_GetRenderedBitmap(document, page, text_object, scale) })
    }
    /// Calls `FPDFTextObj_GetFont` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFTextObj_GetFont`.
    pub unsafe fn FPDFTextObj_GetFont(&self, text: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<FPDF_FONT>, crate::PdfiumError> {
        crate::runtime::call("FPDFTextObj_GetFont", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFTextObj_GetFont(text) })
    }
    /// Calls `FPDFFont_GetBaseFontName` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFFont_GetBaseFontName`.
    pub unsafe fn FPDFFont_GetBaseFontName(&self, font: FPDF_FONT, buffer: *mut ::core::ffi::c_char, length: usize) -> Result<crate::PdfiumCall<usize>, crate::PdfiumError> {
        crate::runtime::call("FPDFFont_GetBaseFontName", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFFont_GetBaseFontName(font, buffer, length) })
    }
    /// Calls `FPDFFont_GetFamilyName` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFFont_GetFamilyName`.
    pub unsafe fn FPDFFont_GetFamilyName(&self, font: FPDF_FONT, buffer: *mut ::core::ffi::c_char, length: usize) -> Result<crate::PdfiumCall<usize>, crate::PdfiumError> {
        crate::runtime::call("FPDFFont_GetFamilyName", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFFont_GetFamilyName(font, buffer, length) })
    }
    /// Calls `FPDFFont_GetFontData` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFFont_GetFontData`.
    pub unsafe fn FPDFFont_GetFontData(&self, font: FPDF_FONT, buffer: *mut u8, buflen: usize, out_buflen: *mut usize) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFFont_GetFontData", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFFont_GetFontData(font, buffer, buflen, out_buflen) })
    }
    /// Calls `FPDFFont_GetIsEmbedded` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFFont_GetIsEmbedded`.
    pub unsafe fn FPDFFont_GetIsEmbedded(&self, font: FPDF_FONT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFFont_GetIsEmbedded", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFFont_GetIsEmbedded(font) })
    }
    /// Calls `FPDFFont_GetFlags` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFFont_GetFlags`.
    pub unsafe fn FPDFFont_GetFlags(&self, font: FPDF_FONT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFFont_GetFlags", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFFont_GetFlags(font) })
    }
    /// Calls `FPDFFont_GetWeight` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFFont_GetWeight`.
    pub unsafe fn FPDFFont_GetWeight(&self, font: FPDF_FONT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFFont_GetWeight", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFFont_GetWeight(font) })
    }
    /// Calls `FPDFFont_GetItalicAngle` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFFont_GetItalicAngle`.
    pub unsafe fn FPDFFont_GetItalicAngle(&self, font: FPDF_FONT, angle: *mut ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFFont_GetItalicAngle", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFFont_GetItalicAngle(font, angle) })
    }
    /// Calls `FPDFFont_GetAscent` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFFont_GetAscent`.
    pub unsafe fn FPDFFont_GetAscent(&self, font: FPDF_FONT, font_size: f32, ascent: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFFont_GetAscent", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFFont_GetAscent(font, font_size, ascent) })
    }
    /// Calls `FPDFFont_GetDescent` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFFont_GetDescent`.
    pub unsafe fn FPDFFont_GetDescent(&self, font: FPDF_FONT, font_size: f32, descent: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFFont_GetDescent", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFFont_GetDescent(font, font_size, descent) })
    }
    /// Calls `FPDFFont_GetGlyphWidth` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFFont_GetGlyphWidth`.
    pub unsafe fn FPDFFont_GetGlyphWidth(&self, font: FPDF_FONT, glyph: u32, font_size: f32, width: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFFont_GetGlyphWidth", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFFont_GetGlyphWidth(font, glyph, font_size, width) })
    }
    /// Calls `FPDFFont_GetGlyphPath` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFFont_GetGlyphPath`.
    pub unsafe fn FPDFFont_GetGlyphPath(&self, font: FPDF_FONT, glyph: u32, font_size: f32) -> Result<crate::PdfiumCall<FPDF_GLYPHPATH>, crate::PdfiumError> {
        crate::runtime::call("FPDFFont_GetGlyphPath", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFFont_GetGlyphPath(font, glyph, font_size) })
    }
    /// Calls `FPDFGlyphPath_CountGlyphSegments` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFGlyphPath_CountGlyphSegments`.
    pub unsafe fn FPDFGlyphPath_CountGlyphSegments(&self, glyphpath: FPDF_GLYPHPATH) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFGlyphPath_CountGlyphSegments", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFGlyphPath_CountGlyphSegments(glyphpath) })
    }
    /// Calls `FPDFGlyphPath_GetGlyphPathSegment` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFGlyphPath_GetGlyphPathSegment`.
    pub unsafe fn FPDFGlyphPath_GetGlyphPathSegment(&self, glyphpath: FPDF_GLYPHPATH, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_PATHSEGMENT>, crate::PdfiumError> {
        crate::runtime::call("FPDFGlyphPath_GetGlyphPathSegment", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFGlyphPath_GetGlyphPathSegment(glyphpath, index) })
    }
    /// Calls `FPDFFormObj_CountObjects` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFFormObj_CountObjects`.
    pub unsafe fn FPDFFormObj_CountObjects(&self, form_object: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFFormObj_CountObjects", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFFormObj_CountObjects(form_object) })
    }
    /// Calls `FPDFFormObj_GetObject` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFFormObj_GetObject`.
    pub unsafe fn FPDFFormObj_GetObject(&self, form_object: FPDF_PAGEOBJECT, index: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<FPDF_PAGEOBJECT>, crate::PdfiumError> {
        crate::runtime::call("FPDFFormObj_GetObject", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFFormObj_GetObject(form_object, index) })
    }
    /// Calls `FPDFFormObj_RemoveObject` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFFormObj_RemoveObject`.
    pub unsafe fn FPDFFormObj_RemoveObject(&self, form_object: FPDF_PAGEOBJECT, page_object: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFFormObj_RemoveObject", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFFormObj_RemoveObject(form_object, page_object) })
    }
    /// Calls `FSDK_SetUnSpObjProcessHandler` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FSDK_SetUnSpObjProcessHandler`.
    pub unsafe fn FSDK_SetUnSpObjProcessHandler(&self, unsp_info: *mut UNSUPPORT_INFO) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FSDK_SetUnSpObjProcessHandler", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FSDK_SetUnSpObjProcessHandler(unsp_info) })
    }
    /// Calls `FSDK_SetTimeFunction` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FSDK_SetTimeFunction`.
    pub unsafe fn FSDK_SetTimeFunction(&self, func: ::core::option::Option<unsafe extern "C" fn() -> time_t>) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FSDK_SetTimeFunction", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FSDK_SetTimeFunction(func) })
    }
    /// Calls `FSDK_SetLocaltimeFunction` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FSDK_SetLocaltimeFunction`.
    pub unsafe fn FSDK_SetLocaltimeFunction(&self, func: ::core::option::Option<unsafe extern "C" fn(arg1: *const time_t) -> *mut tm>) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FSDK_SetLocaltimeFunction", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FSDK_SetLocaltimeFunction(func) })
    }
    /// Calls `FPDFDoc_GetPageMode` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFDoc_GetPageMode`.
    pub unsafe fn FPDFDoc_GetPageMode(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFDoc_GetPageMode", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFDoc_GetPageMode(document) })
    }
    /// Calls `FPDFPage_Flatten` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_Flatten`.
    pub unsafe fn FPDFPage_Flatten(&self, page: FPDF_PAGE, nFlag: ::core::ffi::c_int) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_Flatten", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_Flatten(page, nFlag) })
    }
    /// Calls `FPDFDoc_GetJavaScriptActionCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFDoc_GetJavaScriptActionCount`.
    pub unsafe fn FPDFDoc_GetJavaScriptActionCount(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFDoc_GetJavaScriptActionCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFDoc_GetJavaScriptActionCount(document) })
    }
    /// Calls `FPDFDoc_GetJavaScriptAction` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFDoc_GetJavaScriptAction`.
    pub unsafe fn FPDFDoc_GetJavaScriptAction(&self, document: FPDF_DOCUMENT, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_JAVASCRIPT_ACTION>, crate::PdfiumError> {
        crate::runtime::call("FPDFDoc_GetJavaScriptAction", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFDoc_GetJavaScriptAction(document, index) })
    }
    /// Calls `FPDFDoc_CloseJavaScriptAction` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFDoc_CloseJavaScriptAction`.
    pub unsafe fn FPDFDoc_CloseJavaScriptAction(&self, javascript: FPDF_JAVASCRIPT_ACTION) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFDoc_CloseJavaScriptAction", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFDoc_CloseJavaScriptAction(javascript) })
    }
    /// Calls `FPDFJavaScriptAction_GetName` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFJavaScriptAction_GetName`.
    pub unsafe fn FPDFJavaScriptAction_GetName(&self, javascript: FPDF_JAVASCRIPT_ACTION, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFJavaScriptAction_GetName", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFJavaScriptAction_GetName(javascript, buffer, buflen) })
    }
    /// Calls `FPDFJavaScriptAction_GetScript` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFJavaScriptAction_GetScript`.
    pub unsafe fn FPDFJavaScriptAction_GetScript(&self, javascript: FPDF_JAVASCRIPT_ACTION, buffer: *mut FPDF_WCHAR, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFJavaScriptAction_GetScript", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFJavaScriptAction_GetScript(javascript, buffer, buflen) })
    }
    /// Calls `FPDF_ImportPagesByIndex` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_ImportPagesByIndex`.
    pub unsafe fn FPDF_ImportPagesByIndex(&self, dest_doc: FPDF_DOCUMENT, src_doc: FPDF_DOCUMENT, page_indices: *const ::core::ffi::c_int, length: ::core::ffi::c_ulong, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_ImportPagesByIndex", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_ImportPagesByIndex(dest_doc, src_doc, page_indices, length, index) })
    }
    /// Calls `FPDF_ImportPages` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_ImportPages`.
    pub unsafe fn FPDF_ImportPages(&self, dest_doc: FPDF_DOCUMENT, src_doc: FPDF_DOCUMENT, pagerange: FPDF_BYTESTRING, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_ImportPages", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_ImportPages(dest_doc, src_doc, pagerange, index) })
    }
    /// Calls `FPDF_ImportNPagesToOne` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_ImportNPagesToOne`.
    pub unsafe fn FPDF_ImportNPagesToOne(&self, src_doc: FPDF_DOCUMENT, output_width: f32, output_height: f32, num_pages_on_x_axis: usize, num_pages_on_y_axis: usize) -> Result<crate::PdfiumCall<FPDF_DOCUMENT>, crate::PdfiumError> {
        crate::runtime::call("FPDF_ImportNPagesToOne", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_ImportNPagesToOne(src_doc, output_width, output_height, num_pages_on_x_axis, num_pages_on_y_axis) })
    }
    /// Calls `FPDF_NewXObjectFromPage` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_NewXObjectFromPage`.
    pub unsafe fn FPDF_NewXObjectFromPage(&self, dest_doc: FPDF_DOCUMENT, src_doc: FPDF_DOCUMENT, src_page_index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_XOBJECT>, crate::PdfiumError> {
        crate::runtime::call("FPDF_NewXObjectFromPage", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_NewXObjectFromPage(dest_doc, src_doc, src_page_index) })
    }
    /// Calls `FPDF_CloseXObject` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_CloseXObject`.
    pub unsafe fn FPDF_CloseXObject(&self, xobject: FPDF_XOBJECT) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDF_CloseXObject", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_CloseXObject(xobject) })
    }
    /// Calls `FPDF_NewFormObjectFromXObject` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_NewFormObjectFromXObject`.
    pub unsafe fn FPDF_NewFormObjectFromXObject(&self, xobject: FPDF_XOBJECT) -> Result<crate::PdfiumCall<FPDF_PAGEOBJECT>, crate::PdfiumError> {
        crate::runtime::call("FPDF_NewFormObjectFromXObject", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_NewFormObjectFromXObject(xobject) })
    }
    /// Calls `FPDF_CopyViewerPreferences` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_CopyViewerPreferences`.
    pub unsafe fn FPDF_CopyViewerPreferences(&self, dest_doc: FPDF_DOCUMENT, src_doc: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_CopyViewerPreferences", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_CopyViewerPreferences(dest_doc, src_doc) })
    }
    /// Calls `FPDF_RenderPageBitmapWithColorScheme_Start` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_RenderPageBitmapWithColorScheme_Start`.
    pub unsafe fn FPDF_RenderPageBitmapWithColorScheme_Start(&self, bitmap: FPDF_BITMAP, page: FPDF_PAGE, start_x: ::core::ffi::c_int, start_y: ::core::ffi::c_int, size_x: ::core::ffi::c_int, size_y: ::core::ffi::c_int, rotate: ::core::ffi::c_int, flags: ::core::ffi::c_int, color_scheme: *const FPDF_COLORSCHEME, pause: *mut IFSDK_PAUSE) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_RenderPageBitmapWithColorScheme_Start", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_RenderPageBitmapWithColorScheme_Start(bitmap, page, start_x, start_y, size_x, size_y, rotate, flags, color_scheme, pause) })
    }
    /// Calls `FPDF_RenderPageBitmap_Start` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_RenderPageBitmap_Start`.
    pub unsafe fn FPDF_RenderPageBitmap_Start(&self, bitmap: FPDF_BITMAP, page: FPDF_PAGE, start_x: ::core::ffi::c_int, start_y: ::core::ffi::c_int, size_x: ::core::ffi::c_int, size_y: ::core::ffi::c_int, rotate: ::core::ffi::c_int, flags: ::core::ffi::c_int, pause: *mut IFSDK_PAUSE) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_RenderPageBitmap_Start", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_RenderPageBitmap_Start(bitmap, page, start_x, start_y, size_x, size_y, rotate, flags, pause) })
    }
    /// Calls `FPDF_RenderPage_Continue` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_RenderPage_Continue`.
    pub unsafe fn FPDF_RenderPage_Continue(&self, page: FPDF_PAGE, pause: *mut IFSDK_PAUSE) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_RenderPage_Continue", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_RenderPage_Continue(page, pause) })
    }
    /// Calls `FPDF_RenderPage_Close` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_RenderPage_Close`.
    pub unsafe fn FPDF_RenderPage_Close(&self, page: FPDF_PAGE) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDF_RenderPage_Close", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_RenderPage_Close(page) })
    }
    /// Calls `FPDF_SaveAsCopy` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_SaveAsCopy`.
    pub unsafe fn FPDF_SaveAsCopy(&self, document: FPDF_DOCUMENT, file_write: *mut FPDF_FILEWRITE, flags: FPDF_DWORD) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_SaveAsCopy", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_SaveAsCopy(document, file_write, flags) })
    }
    /// Calls `FPDF_SaveWithVersion` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_SaveWithVersion`.
    pub unsafe fn FPDF_SaveWithVersion(&self, document: FPDF_DOCUMENT, file_write: *mut FPDF_FILEWRITE, flags: FPDF_DWORD, file_version: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_SaveWithVersion", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_SaveWithVersion(document, file_write, flags, file_version) })
    }
    /// Calls `FPDFText_GetCharIndexFromTextIndex` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetCharIndexFromTextIndex`.
    pub unsafe fn FPDFText_GetCharIndexFromTextIndex(&self, text_page: FPDF_TEXTPAGE, nTextIndex: ::core::ffi::c_int) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetCharIndexFromTextIndex", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetCharIndexFromTextIndex(text_page, nTextIndex) })
    }
    /// Calls `FPDFText_GetTextIndexFromCharIndex` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetTextIndexFromCharIndex`.
    pub unsafe fn FPDFText_GetTextIndexFromCharIndex(&self, text_page: FPDF_TEXTPAGE, nCharIndex: ::core::ffi::c_int) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetTextIndexFromCharIndex", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetTextIndexFromCharIndex(text_page, nCharIndex) })
    }
    /// Calls `FPDF_GetSignatureCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetSignatureCount`.
    pub unsafe fn FPDF_GetSignatureCount(&self, document: FPDF_DOCUMENT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetSignatureCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetSignatureCount(document) })
    }
    /// Calls `FPDF_GetSignatureObject` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetSignatureObject`.
    pub unsafe fn FPDF_GetSignatureObject(&self, document: FPDF_DOCUMENT, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_SIGNATURE>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetSignatureObject", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetSignatureObject(document, index) })
    }
    /// Calls `FPDFSignatureObj_GetContents` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFSignatureObj_GetContents`.
    pub unsafe fn FPDFSignatureObj_GetContents(&self, signature: FPDF_SIGNATURE, buffer: *mut ::core::ffi::c_void, length: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFSignatureObj_GetContents", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFSignatureObj_GetContents(signature, buffer, length) })
    }
    /// Calls `FPDFSignatureObj_GetByteRange` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFSignatureObj_GetByteRange`.
    pub unsafe fn FPDFSignatureObj_GetByteRange(&self, signature: FPDF_SIGNATURE, buffer: *mut ::core::ffi::c_int, length: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFSignatureObj_GetByteRange", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFSignatureObj_GetByteRange(signature, buffer, length) })
    }
    /// Calls `FPDFSignatureObj_GetSubFilter` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFSignatureObj_GetSubFilter`.
    pub unsafe fn FPDFSignatureObj_GetSubFilter(&self, signature: FPDF_SIGNATURE, buffer: *mut ::core::ffi::c_char, length: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFSignatureObj_GetSubFilter", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFSignatureObj_GetSubFilter(signature, buffer, length) })
    }
    /// Calls `FPDFSignatureObj_GetReason` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFSignatureObj_GetReason`.
    pub unsafe fn FPDFSignatureObj_GetReason(&self, signature: FPDF_SIGNATURE, buffer: *mut ::core::ffi::c_void, length: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFSignatureObj_GetReason", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFSignatureObj_GetReason(signature, buffer, length) })
    }
    /// Calls `FPDFSignatureObj_GetTime` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFSignatureObj_GetTime`.
    pub unsafe fn FPDFSignatureObj_GetTime(&self, signature: FPDF_SIGNATURE, buffer: *mut ::core::ffi::c_char, length: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFSignatureObj_GetTime", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFSignatureObj_GetTime(signature, buffer, length) })
    }
    /// Calls `FPDFSignatureObj_GetDocMDPPermission` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFSignatureObj_GetDocMDPPermission`.
    pub unsafe fn FPDFSignatureObj_GetDocMDPPermission(&self, signature: FPDF_SIGNATURE) -> Result<crate::PdfiumCall<::core::ffi::c_uint>, crate::PdfiumError> {
        crate::runtime::call("FPDFSignatureObj_GetDocMDPPermission", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFSignatureObj_GetDocMDPPermission(signature) })
    }
    /// Calls `FPDF_StructTree_GetForPage` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructTree_GetForPage`.
    pub unsafe fn FPDF_StructTree_GetForPage(&self, page: FPDF_PAGE) -> Result<crate::PdfiumCall<FPDF_STRUCTTREE>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructTree_GetForPage", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructTree_GetForPage(page) })
    }
    /// Calls `FPDF_StructTree_Close` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructTree_Close`.
    pub unsafe fn FPDF_StructTree_Close(&self, struct_tree: FPDF_STRUCTTREE) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructTree_Close", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructTree_Close(struct_tree) })
    }
    /// Calls `FPDF_StructTree_CountChildren` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructTree_CountChildren`.
    pub unsafe fn FPDF_StructTree_CountChildren(&self, struct_tree: FPDF_STRUCTTREE) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructTree_CountChildren", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructTree_CountChildren(struct_tree) })
    }
    /// Calls `FPDF_StructTree_GetChildAtIndex` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructTree_GetChildAtIndex`.
    pub unsafe fn FPDF_StructTree_GetChildAtIndex(&self, struct_tree: FPDF_STRUCTTREE, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_STRUCTELEMENT>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructTree_GetChildAtIndex", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructTree_GetChildAtIndex(struct_tree, index) })
    }
    /// Calls `FPDF_StructElement_GetAltText` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_GetAltText`.
    pub unsafe fn FPDF_StructElement_GetAltText(&self, struct_element: FPDF_STRUCTELEMENT, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_GetAltText", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_GetAltText(struct_element, buffer, buflen) })
    }
    /// Calls `FPDF_StructElement_GetActualText` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_GetActualText`.
    pub unsafe fn FPDF_StructElement_GetActualText(&self, struct_element: FPDF_STRUCTELEMENT, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_GetActualText", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_GetActualText(struct_element, buffer, buflen) })
    }
    /// Calls `FPDF_StructElement_GetExpansion` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_GetExpansion`.
    pub unsafe fn FPDF_StructElement_GetExpansion(&self, struct_element: FPDF_STRUCTELEMENT, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_GetExpansion", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_GetExpansion(struct_element, buffer, buflen) })
    }
    /// Calls `FPDF_StructElement_GetID` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_GetID`.
    pub unsafe fn FPDF_StructElement_GetID(&self, struct_element: FPDF_STRUCTELEMENT, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_GetID", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_GetID(struct_element, buffer, buflen) })
    }
    /// Calls `FPDF_StructElement_GetLang` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_GetLang`.
    pub unsafe fn FPDF_StructElement_GetLang(&self, struct_element: FPDF_STRUCTELEMENT, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_GetLang", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_GetLang(struct_element, buffer, buflen) })
    }
    /// Calls `FPDF_StructElement_GetStringAttribute` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_GetStringAttribute`.
    pub unsafe fn FPDF_StructElement_GetStringAttribute(&self, struct_element: FPDF_STRUCTELEMENT, attr_name: FPDF_BYTESTRING, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_GetStringAttribute", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_GetStringAttribute(struct_element, attr_name, buffer, buflen) })
    }
    /// Calls `FPDF_StructElement_GetMarkedContentID` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_GetMarkedContentID`.
    pub unsafe fn FPDF_StructElement_GetMarkedContentID(&self, struct_element: FPDF_STRUCTELEMENT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_GetMarkedContentID", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_GetMarkedContentID(struct_element) })
    }
    /// Calls `FPDF_StructElement_GetRawType` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_GetRawType`.
    pub unsafe fn FPDF_StructElement_GetRawType(&self, struct_element: FPDF_STRUCTELEMENT, buffer: *mut FPDF_WCHAR, buflen: usize) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_GetRawType", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_GetRawType(struct_element, buffer, buflen) })
    }
    /// Calls `FPDF_StructElement_GetType` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_GetType`.
    pub unsafe fn FPDF_StructElement_GetType(&self, struct_element: FPDF_STRUCTELEMENT, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_GetType", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_GetType(struct_element, buffer, buflen) })
    }
    /// Calls `FPDF_StructElement_GetObjType` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_GetObjType`.
    pub unsafe fn FPDF_StructElement_GetObjType(&self, struct_element: FPDF_STRUCTELEMENT, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_GetObjType", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_GetObjType(struct_element, buffer, buflen) })
    }
    /// Calls `FPDF_StructElement_GetTitle` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_GetTitle`.
    pub unsafe fn FPDF_StructElement_GetTitle(&self, struct_element: FPDF_STRUCTELEMENT, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_GetTitle", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_GetTitle(struct_element, buffer, buflen) })
    }
    /// Calls `FPDF_StructElement_CountChildren` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_CountChildren`.
    pub unsafe fn FPDF_StructElement_CountChildren(&self, struct_element: FPDF_STRUCTELEMENT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_CountChildren", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_CountChildren(struct_element) })
    }
    /// Calls `FPDF_StructElement_GetChildAtIndex` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_GetChildAtIndex`.
    pub unsafe fn FPDF_StructElement_GetChildAtIndex(&self, struct_element: FPDF_STRUCTELEMENT, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_STRUCTELEMENT>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_GetChildAtIndex", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_GetChildAtIndex(struct_element, index) })
    }
    /// Calls `FPDF_StructElement_GetChildMarkedContentID` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_GetChildMarkedContentID`.
    pub unsafe fn FPDF_StructElement_GetChildMarkedContentID(&self, struct_element: FPDF_STRUCTELEMENT, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_GetChildMarkedContentID", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_GetChildMarkedContentID(struct_element, index) })
    }
    /// Calls `FPDF_StructElement_GetParent` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_GetParent`.
    pub unsafe fn FPDF_StructElement_GetParent(&self, struct_element: FPDF_STRUCTELEMENT) -> Result<crate::PdfiumCall<FPDF_STRUCTELEMENT>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_GetParent", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_GetParent(struct_element) })
    }
    /// Calls `FPDF_StructElement_GetAttributeCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_GetAttributeCount`.
    pub unsafe fn FPDF_StructElement_GetAttributeCount(&self, struct_element: FPDF_STRUCTELEMENT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_GetAttributeCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_GetAttributeCount(struct_element) })
    }
    /// Calls `FPDF_StructElement_GetAttributeAtIndex` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_GetAttributeAtIndex`.
    pub unsafe fn FPDF_StructElement_GetAttributeAtIndex(&self, struct_element: FPDF_STRUCTELEMENT, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_STRUCTELEMENT_ATTR>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_GetAttributeAtIndex", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_GetAttributeAtIndex(struct_element, index) })
    }
    /// Calls `FPDF_StructElement_Attr_GetCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_Attr_GetCount`.
    pub unsafe fn FPDF_StructElement_Attr_GetCount(&self, struct_attribute: FPDF_STRUCTELEMENT_ATTR) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_Attr_GetCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_Attr_GetCount(struct_attribute) })
    }
    /// Calls `FPDF_StructElement_Attr_GetName` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_Attr_GetName`.
    pub unsafe fn FPDF_StructElement_Attr_GetName(&self, struct_attribute: FPDF_STRUCTELEMENT_ATTR, index: ::core::ffi::c_int, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong, out_buflen: *mut ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_Attr_GetName", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_Attr_GetName(struct_attribute, index, buffer, buflen, out_buflen) })
    }
    /// Calls `FPDF_StructElement_Attr_GetValue` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_Attr_GetValue`.
    pub unsafe fn FPDF_StructElement_Attr_GetValue(&self, struct_attribute: FPDF_STRUCTELEMENT_ATTR, name: FPDF_BYTESTRING) -> Result<crate::PdfiumCall<FPDF_STRUCTELEMENT_ATTR_VALUE>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_Attr_GetValue", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_Attr_GetValue(struct_attribute, name) })
    }
    /// Calls `FPDF_StructElement_Attr_GetType` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_Attr_GetType`.
    pub unsafe fn FPDF_StructElement_Attr_GetType(&self, value: FPDF_STRUCTELEMENT_ATTR_VALUE) -> Result<crate::PdfiumCall<FPDF_OBJECT_TYPE>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_Attr_GetType", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_Attr_GetType(value) })
    }
    /// Calls `FPDF_StructElement_Attr_GetBooleanValue` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_Attr_GetBooleanValue`.
    pub unsafe fn FPDF_StructElement_Attr_GetBooleanValue(&self, value: FPDF_STRUCTELEMENT_ATTR_VALUE, out_value: *mut FPDF_BOOL) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_Attr_GetBooleanValue", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_Attr_GetBooleanValue(value, out_value) })
    }
    /// Calls `FPDF_StructElement_Attr_GetNumberValue` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_Attr_GetNumberValue`.
    pub unsafe fn FPDF_StructElement_Attr_GetNumberValue(&self, value: FPDF_STRUCTELEMENT_ATTR_VALUE, out_value: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_Attr_GetNumberValue", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_Attr_GetNumberValue(value, out_value) })
    }
    /// Calls `FPDF_StructElement_Attr_GetStringValue` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_Attr_GetStringValue`.
    pub unsafe fn FPDF_StructElement_Attr_GetStringValue(&self, value: FPDF_STRUCTELEMENT_ATTR_VALUE, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong, out_buflen: *mut ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_Attr_GetStringValue", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_Attr_GetStringValue(value, buffer, buflen, out_buflen) })
    }
    /// Calls `FPDF_StructElement_Attr_GetBlobValue` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_Attr_GetBlobValue`.
    pub unsafe fn FPDF_StructElement_Attr_GetBlobValue(&self, value: FPDF_STRUCTELEMENT_ATTR_VALUE, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong, out_buflen: *mut ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_Attr_GetBlobValue", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_Attr_GetBlobValue(value, buffer, buflen, out_buflen) })
    }
    /// Calls `FPDF_StructElement_Attr_CountChildren` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_Attr_CountChildren`.
    pub unsafe fn FPDF_StructElement_Attr_CountChildren(&self, value: FPDF_STRUCTELEMENT_ATTR_VALUE) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_Attr_CountChildren", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_Attr_CountChildren(value) })
    }
    /// Calls `FPDF_StructElement_Attr_GetChildAtIndex` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_Attr_GetChildAtIndex`.
    pub unsafe fn FPDF_StructElement_Attr_GetChildAtIndex(&self, value: FPDF_STRUCTELEMENT_ATTR_VALUE, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_STRUCTELEMENT_ATTR_VALUE>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_Attr_GetChildAtIndex", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_Attr_GetChildAtIndex(value, index) })
    }
    /// Calls `FPDF_StructElement_GetMarkedContentIdCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_GetMarkedContentIdCount`.
    pub unsafe fn FPDF_StructElement_GetMarkedContentIdCount(&self, struct_element: FPDF_STRUCTELEMENT) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_GetMarkedContentIdCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_GetMarkedContentIdCount(struct_element) })
    }
    /// Calls `FPDF_StructElement_GetMarkedContentIdAtIndex` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_StructElement_GetMarkedContentIdAtIndex`.
    pub unsafe fn FPDF_StructElement_GetMarkedContentIdAtIndex(&self, struct_element: FPDF_STRUCTELEMENT, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDF_StructElement_GetMarkedContentIdAtIndex", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_StructElement_GetMarkedContentIdAtIndex(struct_element, index) })
    }
    /// Calls `FPDF_GetDefaultTTFMap` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetDefaultTTFMap`.
    pub unsafe fn FPDF_GetDefaultTTFMap(&self) -> Result<crate::PdfiumCall<*const FPDF_CharsetFontMap>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetDefaultTTFMap", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetDefaultTTFMap() })
    }
    /// Calls `FPDF_GetDefaultTTFMapCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetDefaultTTFMapCount`.
    pub unsafe fn FPDF_GetDefaultTTFMapCount(&self) -> Result<crate::PdfiumCall<usize>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetDefaultTTFMapCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetDefaultTTFMapCount() })
    }
    /// Calls `FPDF_GetDefaultTTFMapEntry` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetDefaultTTFMapEntry`.
    pub unsafe fn FPDF_GetDefaultTTFMapEntry(&self, index: usize) -> Result<crate::PdfiumCall<*const FPDF_CharsetFontMap>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetDefaultTTFMapEntry", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetDefaultTTFMapEntry(index) })
    }
    /// Calls `FPDF_AddInstalledFont` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_AddInstalledFont`.
    pub unsafe fn FPDF_AddInstalledFont(&self, mapper: *mut ::core::ffi::c_void, face: *const ::core::ffi::c_char, charset: ::core::ffi::c_int) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDF_AddInstalledFont", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_AddInstalledFont(mapper, face, charset) })
    }
    /// Calls `FPDF_SetSystemFontInfo` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_SetSystemFontInfo`.
    pub unsafe fn FPDF_SetSystemFontInfo(&self, font_info: *mut FPDF_SYSFONTINFO) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDF_SetSystemFontInfo", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_SetSystemFontInfo(font_info) })
    }
    /// Calls `FPDF_GetDefaultSystemFontInfo` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetDefaultSystemFontInfo`.
    pub unsafe fn FPDF_GetDefaultSystemFontInfo(&self) -> Result<crate::PdfiumCall<*mut FPDF_SYSFONTINFO>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetDefaultSystemFontInfo", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_GetDefaultSystemFontInfo() })
    }
    /// Calls `FPDF_FreeDefaultSystemFontInfo` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_FreeDefaultSystemFontInfo`.
    pub unsafe fn FPDF_FreeDefaultSystemFontInfo(&self, font_info: *mut FPDF_SYSFONTINFO) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDF_FreeDefaultSystemFontInfo", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_FreeDefaultSystemFontInfo(font_info) })
    }
    /// Calls `FPDFText_LoadPage` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_LoadPage`.
    pub unsafe fn FPDFText_LoadPage(&self, page: FPDF_PAGE) -> Result<crate::PdfiumCall<FPDF_TEXTPAGE>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_LoadPage", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_LoadPage(page) })
    }
    /// Calls `FPDFText_ClosePage` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_ClosePage`.
    pub unsafe fn FPDFText_ClosePage(&self, text_page: FPDF_TEXTPAGE) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_ClosePage", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_ClosePage(text_page) })
    }
    /// Calls `FPDFText_CountChars` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_CountChars`.
    pub unsafe fn FPDFText_CountChars(&self, text_page: FPDF_TEXTPAGE) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_CountChars", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_CountChars(text_page) })
    }
    /// Calls `FPDFText_GetUnicode` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetUnicode`.
    pub unsafe fn FPDFText_GetUnicode(&self, text_page: FPDF_TEXTPAGE, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<::core::ffi::c_uint>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetUnicode", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetUnicode(text_page, index) })
    }
    /// Calls `FPDFText_GetTextObject` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetTextObject`.
    pub unsafe fn FPDFText_GetTextObject(&self, text_page: FPDF_TEXTPAGE, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_PAGEOBJECT>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetTextObject", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetTextObject(text_page, index) })
    }
    /// Calls `FPDFText_IsGenerated` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_IsGenerated`.
    pub unsafe fn FPDFText_IsGenerated(&self, text_page: FPDF_TEXTPAGE, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_IsGenerated", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_IsGenerated(text_page, index) })
    }
    /// Calls `FPDFText_IsHyphen` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_IsHyphen`.
    pub unsafe fn FPDFText_IsHyphen(&self, text_page: FPDF_TEXTPAGE, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_IsHyphen", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_IsHyphen(text_page, index) })
    }
    /// Calls `FPDFText_HasUnicodeMapError` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_HasUnicodeMapError`.
    pub unsafe fn FPDFText_HasUnicodeMapError(&self, text_page: FPDF_TEXTPAGE, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_HasUnicodeMapError", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_HasUnicodeMapError(text_page, index) })
    }
    /// Calls `FPDFText_GetFontSize` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetFontSize`.
    pub unsafe fn FPDFText_GetFontSize(&self, text_page: FPDF_TEXTPAGE, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<f64>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetFontSize", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetFontSize(text_page, index) })
    }
    /// Calls `FPDFText_GetFontInfo` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetFontInfo`.
    pub unsafe fn FPDFText_GetFontInfo(&self, text_page: FPDF_TEXTPAGE, index: ::core::ffi::c_int, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong, flags: *mut ::core::ffi::c_int) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetFontInfo", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetFontInfo(text_page, index, buffer, buflen, flags) })
    }
    /// Calls `FPDFText_GetFontWeight` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetFontWeight`.
    pub unsafe fn FPDFText_GetFontWeight(&self, text_page: FPDF_TEXTPAGE, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetFontWeight", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetFontWeight(text_page, index) })
    }
    /// Calls `FPDFText_GetFillColor` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetFillColor`.
    pub unsafe fn FPDFText_GetFillColor(&self, text_page: FPDF_TEXTPAGE, index: ::core::ffi::c_int, R: *mut ::core::ffi::c_uint, G: *mut ::core::ffi::c_uint, B: *mut ::core::ffi::c_uint, A: *mut ::core::ffi::c_uint) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetFillColor", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetFillColor(text_page, index, R, G, B, A) })
    }
    /// Calls `FPDFText_GetStrokeColor` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetStrokeColor`.
    pub unsafe fn FPDFText_GetStrokeColor(&self, text_page: FPDF_TEXTPAGE, index: ::core::ffi::c_int, R: *mut ::core::ffi::c_uint, G: *mut ::core::ffi::c_uint, B: *mut ::core::ffi::c_uint, A: *mut ::core::ffi::c_uint) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetStrokeColor", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetStrokeColor(text_page, index, R, G, B, A) })
    }
    /// Calls `FPDFText_GetCharAngle` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetCharAngle`.
    pub unsafe fn FPDFText_GetCharAngle(&self, text_page: FPDF_TEXTPAGE, index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<f32>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetCharAngle", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetCharAngle(text_page, index) })
    }
    /// Calls `FPDFText_GetCharBox` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetCharBox`.
    pub unsafe fn FPDFText_GetCharBox(&self, text_page: FPDF_TEXTPAGE, index: ::core::ffi::c_int, left: *mut f64, right: *mut f64, bottom: *mut f64, top: *mut f64) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetCharBox", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetCharBox(text_page, index, left, right, bottom, top) })
    }
    /// Calls `FPDFText_GetLooseCharBox` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetLooseCharBox`.
    pub unsafe fn FPDFText_GetLooseCharBox(&self, text_page: FPDF_TEXTPAGE, index: ::core::ffi::c_int, rect: *mut FS_RECTF) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetLooseCharBox", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetLooseCharBox(text_page, index, rect) })
    }
    /// Calls `FPDFText_GetMatrix` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetMatrix`.
    pub unsafe fn FPDFText_GetMatrix(&self, text_page: FPDF_TEXTPAGE, index: ::core::ffi::c_int, matrix: *mut FS_MATRIX) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetMatrix", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetMatrix(text_page, index, matrix) })
    }
    /// Calls `FPDFText_GetCharOrigin` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetCharOrigin`.
    pub unsafe fn FPDFText_GetCharOrigin(&self, text_page: FPDF_TEXTPAGE, index: ::core::ffi::c_int, x: *mut f64, y: *mut f64) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetCharOrigin", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetCharOrigin(text_page, index, x, y) })
    }
    /// Calls `FPDFText_GetCharIndexAtPos` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetCharIndexAtPos`.
    pub unsafe fn FPDFText_GetCharIndexAtPos(&self, text_page: FPDF_TEXTPAGE, x: f64, y: f64, xTolerance: f64, yTolerance: f64) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetCharIndexAtPos", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetCharIndexAtPos(text_page, x, y, xTolerance, yTolerance) })
    }
    /// Calls `FPDFText_GetText` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetText`.
    pub unsafe fn FPDFText_GetText(&self, text_page: FPDF_TEXTPAGE, start_index: ::core::ffi::c_int, count: ::core::ffi::c_int, result: *mut ::core::ffi::c_ushort) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetText", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetText(text_page, start_index, count, result) })
    }
    /// Calls `FPDFText_CountRects` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_CountRects`.
    pub unsafe fn FPDFText_CountRects(&self, text_page: FPDF_TEXTPAGE, start_index: ::core::ffi::c_int, count: ::core::ffi::c_int) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_CountRects", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_CountRects(text_page, start_index, count) })
    }
    /// Calls `FPDFText_GetRect` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetRect`.
    pub unsafe fn FPDFText_GetRect(&self, text_page: FPDF_TEXTPAGE, rect_index: ::core::ffi::c_int, left: *mut f64, top: *mut f64, right: *mut f64, bottom: *mut f64) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetRect", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetRect(text_page, rect_index, left, top, right, bottom) })
    }
    /// Calls `FPDFText_GetBoundedText` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetBoundedText`.
    pub unsafe fn FPDFText_GetBoundedText(&self, text_page: FPDF_TEXTPAGE, left: f64, top: f64, right: f64, bottom: f64, buffer: *mut ::core::ffi::c_ushort, buflen: ::core::ffi::c_int) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetBoundedText", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetBoundedText(text_page, left, top, right, bottom, buffer, buflen) })
    }
    /// Calls `FPDFText_FindStart` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_FindStart`.
    pub unsafe fn FPDFText_FindStart(&self, text_page: FPDF_TEXTPAGE, findwhat: FPDF_WIDESTRING, flags: ::core::ffi::c_ulong, start_index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_SCHHANDLE>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_FindStart", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_FindStart(text_page, findwhat, flags, start_index) })
    }
    /// Calls `FPDFText_FindNext` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_FindNext`.
    pub unsafe fn FPDFText_FindNext(&self, handle: FPDF_SCHHANDLE) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_FindNext", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_FindNext(handle) })
    }
    /// Calls `FPDFText_FindPrev` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_FindPrev`.
    pub unsafe fn FPDFText_FindPrev(&self, handle: FPDF_SCHHANDLE) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_FindPrev", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_FindPrev(handle) })
    }
    /// Calls `FPDFText_GetSchResultIndex` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetSchResultIndex`.
    pub unsafe fn FPDFText_GetSchResultIndex(&self, handle: FPDF_SCHHANDLE) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetSchResultIndex", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetSchResultIndex(handle) })
    }
    /// Calls `FPDFText_GetSchCount` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_GetSchCount`.
    pub unsafe fn FPDFText_GetSchCount(&self, handle: FPDF_SCHHANDLE) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_GetSchCount", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_GetSchCount(handle) })
    }
    /// Calls `FPDFText_FindClose` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFText_FindClose`.
    pub unsafe fn FPDFText_FindClose(&self, handle: FPDF_SCHHANDLE) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFText_FindClose", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFText_FindClose(handle) })
    }
    /// Calls `FPDFLink_LoadWebLinks` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFLink_LoadWebLinks`.
    pub unsafe fn FPDFLink_LoadWebLinks(&self, text_page: FPDF_TEXTPAGE) -> Result<crate::PdfiumCall<FPDF_PAGELINK>, crate::PdfiumError> {
        crate::runtime::call("FPDFLink_LoadWebLinks", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFLink_LoadWebLinks(text_page) })
    }
    /// Calls `FPDFLink_CountWebLinks` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFLink_CountWebLinks`.
    pub unsafe fn FPDFLink_CountWebLinks(&self, link_page: FPDF_PAGELINK) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFLink_CountWebLinks", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFLink_CountWebLinks(link_page) })
    }
    /// Calls `FPDFLink_GetURL` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFLink_GetURL`.
    pub unsafe fn FPDFLink_GetURL(&self, link_page: FPDF_PAGELINK, link_index: ::core::ffi::c_int, buffer: *mut ::core::ffi::c_ushort, buflen: ::core::ffi::c_int) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFLink_GetURL", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFLink_GetURL(link_page, link_index, buffer, buflen) })
    }
    /// Calls `FPDFLink_CountRects` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFLink_CountRects`.
    pub unsafe fn FPDFLink_CountRects(&self, link_page: FPDF_PAGELINK, link_index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFLink_CountRects", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFLink_CountRects(link_page, link_index) })
    }
    /// Calls `FPDFLink_GetRect` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFLink_GetRect`.
    pub unsafe fn FPDFLink_GetRect(&self, link_page: FPDF_PAGELINK, link_index: ::core::ffi::c_int, rect_index: ::core::ffi::c_int, left: *mut f64, top: *mut f64, right: *mut f64, bottom: *mut f64) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFLink_GetRect", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFLink_GetRect(link_page, link_index, rect_index, left, top, right, bottom) })
    }
    /// Calls `FPDFLink_GetTextRange` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFLink_GetTextRange`.
    pub unsafe fn FPDFLink_GetTextRange(&self, link_page: FPDF_PAGELINK, link_index: ::core::ffi::c_int, start_char_index: *mut ::core::ffi::c_int, char_count: *mut ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFLink_GetTextRange", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFLink_GetTextRange(link_page, link_index, start_char_index, char_count) })
    }
    /// Calls `FPDFLink_CloseWebLinks` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFLink_CloseWebLinks`.
    pub unsafe fn FPDFLink_CloseWebLinks(&self, link_page: FPDF_PAGELINK) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFLink_CloseWebLinks", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFLink_CloseWebLinks(link_page) })
    }
    /// Calls `FPDFPage_GetDecodedThumbnailData` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_GetDecodedThumbnailData`.
    pub unsafe fn FPDFPage_GetDecodedThumbnailData(&self, page: FPDF_PAGE, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_GetDecodedThumbnailData", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_GetDecodedThumbnailData(page, buffer, buflen) })
    }
    /// Calls `FPDFPage_GetRawThumbnailData` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_GetRawThumbnailData`.
    pub unsafe fn FPDFPage_GetRawThumbnailData(&self, page: FPDF_PAGE, buffer: *mut ::core::ffi::c_void, buflen: ::core::ffi::c_ulong) -> Result<crate::PdfiumCall<::core::ffi::c_ulong>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_GetRawThumbnailData", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_GetRawThumbnailData(page, buffer, buflen) })
    }
    /// Calls `FPDFPage_GetThumbnailAsBitmap` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_GetThumbnailAsBitmap`.
    pub unsafe fn FPDFPage_GetThumbnailAsBitmap(&self, page: FPDF_PAGE) -> Result<crate::PdfiumCall<FPDF_BITMAP>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_GetThumbnailAsBitmap", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_GetThumbnailAsBitmap(page) })
    }
    /// Calls `FPDFPage_SetMediaBox` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_SetMediaBox`.
    pub unsafe fn FPDFPage_SetMediaBox(&self, page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_SetMediaBox", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_SetMediaBox(page, left, bottom, right, top) })
    }
    /// Calls `FPDFPage_SetCropBox` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_SetCropBox`.
    pub unsafe fn FPDFPage_SetCropBox(&self, page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_SetCropBox", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_SetCropBox(page, left, bottom, right, top) })
    }
    /// Calls `FPDFPage_SetBleedBox` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_SetBleedBox`.
    pub unsafe fn FPDFPage_SetBleedBox(&self, page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_SetBleedBox", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_SetBleedBox(page, left, bottom, right, top) })
    }
    /// Calls `FPDFPage_SetTrimBox` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_SetTrimBox`.
    pub unsafe fn FPDFPage_SetTrimBox(&self, page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_SetTrimBox", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_SetTrimBox(page, left, bottom, right, top) })
    }
    /// Calls `FPDFPage_SetArtBox` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_SetArtBox`.
    pub unsafe fn FPDFPage_SetArtBox(&self, page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_SetArtBox", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_SetArtBox(page, left, bottom, right, top) })
    }
    /// Calls `FPDFPage_GetMediaBox` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_GetMediaBox`.
    pub unsafe fn FPDFPage_GetMediaBox(&self, page: FPDF_PAGE, left: *mut f32, bottom: *mut f32, right: *mut f32, top: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_GetMediaBox", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_GetMediaBox(page, left, bottom, right, top) })
    }
    /// Calls `FPDFPage_GetCropBox` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_GetCropBox`.
    pub unsafe fn FPDFPage_GetCropBox(&self, page: FPDF_PAGE, left: *mut f32, bottom: *mut f32, right: *mut f32, top: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_GetCropBox", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_GetCropBox(page, left, bottom, right, top) })
    }
    /// Calls `FPDFPage_GetBleedBox` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_GetBleedBox`.
    pub unsafe fn FPDFPage_GetBleedBox(&self, page: FPDF_PAGE, left: *mut f32, bottom: *mut f32, right: *mut f32, top: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_GetBleedBox", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_GetBleedBox(page, left, bottom, right, top) })
    }
    /// Calls `FPDFPage_GetTrimBox` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_GetTrimBox`.
    pub unsafe fn FPDFPage_GetTrimBox(&self, page: FPDF_PAGE, left: *mut f32, bottom: *mut f32, right: *mut f32, top: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_GetTrimBox", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_GetTrimBox(page, left, bottom, right, top) })
    }
    /// Calls `FPDFPage_GetArtBox` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_GetArtBox`.
    pub unsafe fn FPDFPage_GetArtBox(&self, page: FPDF_PAGE, left: *mut f32, bottom: *mut f32, right: *mut f32, top: *mut f32) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_GetArtBox", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_GetArtBox(page, left, bottom, right, top) })
    }
    /// Calls `FPDFPage_TransFormWithClip` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_TransFormWithClip`.
    pub unsafe fn FPDFPage_TransFormWithClip(&self, page: FPDF_PAGE, matrix: *const FS_MATRIX, clipRect: *const FS_RECTF) -> Result<crate::PdfiumCall<FPDF_BOOL>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_TransFormWithClip", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_TransFormWithClip(page, matrix, clipRect) })
    }
    /// Calls `FPDFPageObj_TransformClipPath` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_TransformClipPath`.
    pub unsafe fn FPDFPageObj_TransformClipPath(&self, page_object: FPDF_PAGEOBJECT, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_TransformClipPath", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_TransformClipPath(page_object, a, b, c, d, e, f) })
    }
    /// Calls `FPDFPageObj_GetClipPath` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPageObj_GetClipPath`.
    pub unsafe fn FPDFPageObj_GetClipPath(&self, page_object: FPDF_PAGEOBJECT) -> Result<crate::PdfiumCall<FPDF_CLIPPATH>, crate::PdfiumError> {
        crate::runtime::call("FPDFPageObj_GetClipPath", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPageObj_GetClipPath(page_object) })
    }
    /// Calls `FPDFClipPath_CountPaths` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFClipPath_CountPaths`.
    pub unsafe fn FPDFClipPath_CountPaths(&self, clip_path: FPDF_CLIPPATH) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFClipPath_CountPaths", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFClipPath_CountPaths(clip_path) })
    }
    /// Calls `FPDFClipPath_CountPathSegments` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFClipPath_CountPathSegments`.
    pub unsafe fn FPDFClipPath_CountPathSegments(&self, clip_path: FPDF_CLIPPATH, path_index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<::core::ffi::c_int>, crate::PdfiumError> {
        crate::runtime::call("FPDFClipPath_CountPathSegments", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFClipPath_CountPathSegments(clip_path, path_index) })
    }
    /// Calls `FPDFClipPath_GetPathSegment` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFClipPath_GetPathSegment`.
    pub unsafe fn FPDFClipPath_GetPathSegment(&self, clip_path: FPDF_CLIPPATH, path_index: ::core::ffi::c_int, segment_index: ::core::ffi::c_int) -> Result<crate::PdfiumCall<FPDF_PATHSEGMENT>, crate::PdfiumError> {
        crate::runtime::call("FPDFClipPath_GetPathSegment", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFClipPath_GetPathSegment(clip_path, path_index, segment_index) })
    }
    /// Calls `FPDF_CreateClipPath` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_CreateClipPath`.
    pub unsafe fn FPDF_CreateClipPath(&self, left: f32, bottom: f32, right: f32, top: f32) -> Result<crate::PdfiumCall<FPDF_CLIPPATH>, crate::PdfiumError> {
        crate::runtime::call("FPDF_CreateClipPath", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_CreateClipPath(left, bottom, right, top) })
    }
    /// Calls `FPDF_DestroyClipPath` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_DestroyClipPath`.
    pub unsafe fn FPDF_DestroyClipPath(&self, clipPath: FPDF_CLIPPATH) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDF_DestroyClipPath", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDF_DestroyClipPath(clipPath) })
    }
    /// Calls `FPDFPage_InsertClipPath` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDFPage_InsertClipPath`.
    pub unsafe fn FPDFPage_InsertClipPath(&self, page: FPDF_PAGE, clipPath: FPDF_CLIPPATH) -> Result<crate::PdfiumCall<()>, crate::PdfiumError> {
        crate::runtime::call("FPDFPage_InsertClipPath", crate::runtime::Phase::Initialized, |context| unsafe { context.dispatch.FPDFPage_InsertClipPath(page, clipPath) })
    }
}
#[allow(non_snake_case)]
impl crate::raw::PreInitApi<'_> {
    /// Calls `FPDF_GetRecommendedV8Flags` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetRecommendedV8Flags`.
    pub unsafe fn FPDF_GetRecommendedV8Flags(&self) -> Result<crate::PdfiumCall<*const ::core::ffi::c_char>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetRecommendedV8Flags", crate::runtime::Phase::Prepared, |context| unsafe { context.dispatch.FPDF_GetRecommendedV8Flags() })
    }
    /// Calls `FPDF_GetArrayBufferAllocatorSharedInstance` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetArrayBufferAllocatorSharedInstance`.
    pub unsafe fn FPDF_GetArrayBufferAllocatorSharedInstance(&self) -> Result<crate::PdfiumCall<*mut ::core::ffi::c_void>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetArrayBufferAllocatorSharedInstance", crate::runtime::Phase::Prepared, |context| unsafe { context.dispatch.FPDF_GetArrayBufferAllocatorSharedInstance() })
    }
}
#[allow(non_snake_case)]
impl crate::raw::DestroyedApi<'_> {
    /// Calls `FPDF_GetArrayBufferAllocatorSharedInstance` through PDFium's serialized process coordinator.
    ///
    /// # Safety
    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.
    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `FPDF_GetArrayBufferAllocatorSharedInstance`.
    pub unsafe fn FPDF_GetArrayBufferAllocatorSharedInstance(&self) -> Result<crate::PdfiumCall<*mut ::core::ffi::c_void>, crate::PdfiumError> {
        crate::runtime::call("FPDF_GetArrayBufferAllocatorSharedInstance", crate::runtime::Phase::Destroyed, |context| unsafe { context.dispatch.FPDF_GetArrayBufferAllocatorSharedInstance() })
    }
}
