// Generated from the official PDFium public C headers. Do not edit by hand.
// Source: f51279699cb59754d1f49aef10eb8e825fe56532; configuration: linux-arm64-default.
pub const FPDF_OBJECT_UNKNOWN: u32 = 0;
pub const FPDF_OBJECT_BOOLEAN: u32 = 1;
pub const FPDF_OBJECT_NUMBER: u32 = 2;
pub const FPDF_OBJECT_STRING: u32 = 3;
pub const FPDF_OBJECT_NAME: u32 = 4;
pub const FPDF_OBJECT_ARRAY: u32 = 5;
pub const FPDF_OBJECT_DICTIONARY: u32 = 6;
pub const FPDF_OBJECT_STREAM: u32 = 7;
pub const FPDF_OBJECT_NULLOBJ: u32 = 8;
pub const FPDF_OBJECT_REFERENCE: u32 = 9;
pub const FPDF_POLICY_MACHINETIME_ACCESS: u32 = 0;
pub const FPDF_ERR_SUCCESS: u32 = 0;
pub const FPDF_ERR_UNKNOWN: u32 = 1;
pub const FPDF_ERR_FILE: u32 = 2;
pub const FPDF_ERR_FORMAT: u32 = 3;
pub const FPDF_ERR_PASSWORD: u32 = 4;
pub const FPDF_ERR_SECURITY: u32 = 5;
pub const FPDF_ERR_PAGE: u32 = 6;
pub const FPDF_ANNOT: u32 = 1;
pub const FPDF_LCD_TEXT: u32 = 2;
pub const FPDF_NO_NATIVETEXT: u32 = 4;
pub const FPDF_GRAYSCALE: u32 = 8;
pub const FPDF_DEBUG_INFO: u32 = 128;
pub const FPDF_NO_CATCH: u32 = 256;
pub const FPDF_RENDER_LIMITEDIMAGECACHE: u32 = 512;
pub const FPDF_RENDER_FORCEHALFTONE: u32 = 1024;
pub const FPDF_PRINTING: u32 = 2048;
pub const FPDF_RENDER_NO_SMOOTHTEXT: u32 = 4096;
pub const FPDF_RENDER_NO_SMOOTHIMAGE: u32 = 8192;
pub const FPDF_RENDER_NO_SMOOTHPATH: u32 = 16384;
pub const FPDF_REVERSE_BYTE_ORDER: u32 = 16;
pub const FPDF_CONVERT_FILL_TO_STROKE: u32 = 32;
pub const FPDFBitmap_Unknown: u32 = 0;
pub const FPDFBitmap_Gray: u32 = 1;
pub const FPDFBitmap_BGR: u32 = 2;
pub const FPDFBitmap_BGRx: u32 = 3;
pub const FPDFBitmap_BGRA: u32 = 4;
pub const FPDFBitmap_BGRA_Premul: u32 = 5;
pub const FORMTYPE_NONE: u32 = 0;
pub const FORMTYPE_ACRO_FORM: u32 = 1;
pub const FORMTYPE_XFA_FULL: u32 = 2;
pub const FORMTYPE_XFA_FOREGROUND: u32 = 3;
pub const FORMTYPE_COUNT: u32 = 4;
pub const FPDFDOC_AACTION_WC: u32 = 16;
pub const FPDFDOC_AACTION_WS: u32 = 17;
pub const FPDFDOC_AACTION_DS: u32 = 18;
pub const FPDFDOC_AACTION_WP: u32 = 19;
pub const FPDFDOC_AACTION_DP: u32 = 20;
pub const FPDFPAGE_AACTION_OPEN: u32 = 0;
pub const FPDFPAGE_AACTION_CLOSE: u32 = 1;
pub const FPDF_FORMFIELD_UNKNOWN: u32 = 0;
pub const FPDF_FORMFIELD_PUSHBUTTON: u32 = 1;
pub const FPDF_FORMFIELD_CHECKBOX: u32 = 2;
pub const FPDF_FORMFIELD_RADIOBUTTON: u32 = 3;
pub const FPDF_FORMFIELD_COMBOBOX: u32 = 4;
pub const FPDF_FORMFIELD_LISTBOX: u32 = 5;
pub const FPDF_FORMFIELD_TEXTFIELD: u32 = 6;
pub const FPDF_FORMFIELD_SIGNATURE: u32 = 7;
pub const FPDF_FORMFIELD_COUNT: u32 = 8;
pub const FPDF_ANNOT_UNKNOWN: u32 = 0;
pub const FPDF_ANNOT_TEXT: u32 = 1;
pub const FPDF_ANNOT_LINK: u32 = 2;
pub const FPDF_ANNOT_FREETEXT: u32 = 3;
pub const FPDF_ANNOT_LINE: u32 = 4;
pub const FPDF_ANNOT_SQUARE: u32 = 5;
pub const FPDF_ANNOT_CIRCLE: u32 = 6;
pub const FPDF_ANNOT_POLYGON: u32 = 7;
pub const FPDF_ANNOT_POLYLINE: u32 = 8;
pub const FPDF_ANNOT_HIGHLIGHT: u32 = 9;
pub const FPDF_ANNOT_UNDERLINE: u32 = 10;
pub const FPDF_ANNOT_SQUIGGLY: u32 = 11;
pub const FPDF_ANNOT_STRIKEOUT: u32 = 12;
pub const FPDF_ANNOT_STAMP: u32 = 13;
pub const FPDF_ANNOT_CARET: u32 = 14;
pub const FPDF_ANNOT_INK: u32 = 15;
pub const FPDF_ANNOT_POPUP: u32 = 16;
pub const FPDF_ANNOT_FILEATTACHMENT: u32 = 17;
pub const FPDF_ANNOT_SOUND: u32 = 18;
pub const FPDF_ANNOT_MOVIE: u32 = 19;
pub const FPDF_ANNOT_WIDGET: u32 = 20;
pub const FPDF_ANNOT_SCREEN: u32 = 21;
pub const FPDF_ANNOT_PRINTERMARK: u32 = 22;
pub const FPDF_ANNOT_TRAPNET: u32 = 23;
pub const FPDF_ANNOT_WATERMARK: u32 = 24;
pub const FPDF_ANNOT_THREED: u32 = 25;
pub const FPDF_ANNOT_RICHMEDIA: u32 = 26;
pub const FPDF_ANNOT_XFAWIDGET: u32 = 27;
pub const FPDF_ANNOT_REDACT: u32 = 28;
pub const FPDF_ANNOT_FLAG_NONE: u32 = 0;
pub const FPDF_ANNOT_FLAG_INVISIBLE: u32 = 1;
pub const FPDF_ANNOT_FLAG_HIDDEN: u32 = 2;
pub const FPDF_ANNOT_FLAG_PRINT: u32 = 4;
pub const FPDF_ANNOT_FLAG_NOZOOM: u32 = 8;
pub const FPDF_ANNOT_FLAG_NOROTATE: u32 = 16;
pub const FPDF_ANNOT_FLAG_NOVIEW: u32 = 32;
pub const FPDF_ANNOT_FLAG_READONLY: u32 = 64;
pub const FPDF_ANNOT_FLAG_LOCKED: u32 = 128;
pub const FPDF_ANNOT_FLAG_TOGGLENOVIEW: u32 = 256;
pub const FPDF_ANNOT_APPEARANCEMODE_NORMAL: u32 = 0;
pub const FPDF_ANNOT_APPEARANCEMODE_ROLLOVER: u32 = 1;
pub const FPDF_ANNOT_APPEARANCEMODE_DOWN: u32 = 2;
pub const FPDF_ANNOT_APPEARANCEMODE_COUNT: u32 = 3;
pub const FPDF_FORMFLAG_NONE: u32 = 0;
pub const FPDF_FORMFLAG_READONLY: u32 = 1;
pub const FPDF_FORMFLAG_REQUIRED: u32 = 2;
pub const FPDF_FORMFLAG_NOEXPORT: u32 = 4;
pub const FPDF_FORMFLAG_TEXT_MULTILINE: u32 = 4096;
pub const FPDF_FORMFLAG_TEXT_PASSWORD: u32 = 8192;
pub const FPDF_FORMFLAG_CHOICE_COMBO: u32 = 131072;
pub const FPDF_FORMFLAG_CHOICE_EDIT: u32 = 262144;
pub const FPDF_FORMFLAG_CHOICE_MULTI_SELECT: u32 = 2097152;
pub const FPDF_ANNOT_AACTION_KEY_STROKE: u32 = 12;
pub const FPDF_ANNOT_AACTION_FORMAT: u32 = 13;
pub const FPDF_ANNOT_AACTION_VALIDATE: u32 = 14;
pub const FPDF_ANNOT_AACTION_CALCULATE: u32 = 15;
pub const FPDF_COLORSPACE_UNKNOWN: u32 = 0;
pub const FPDF_COLORSPACE_DEVICEGRAY: u32 = 1;
pub const FPDF_COLORSPACE_DEVICERGB: u32 = 2;
pub const FPDF_COLORSPACE_DEVICECMYK: u32 = 3;
pub const FPDF_COLORSPACE_CALGRAY: u32 = 4;
pub const FPDF_COLORSPACE_CALRGB: u32 = 5;
pub const FPDF_COLORSPACE_LAB: u32 = 6;
pub const FPDF_COLORSPACE_ICCBASED: u32 = 7;
pub const FPDF_COLORSPACE_SEPARATION: u32 = 8;
pub const FPDF_COLORSPACE_DEVICEN: u32 = 9;
pub const FPDF_COLORSPACE_INDEXED: u32 = 10;
pub const FPDF_COLORSPACE_PATTERN: u32 = 11;
pub const FPDF_PAGEOBJ_UNKNOWN: u32 = 0;
pub const FPDF_PAGEOBJ_TEXT: u32 = 1;
pub const FPDF_PAGEOBJ_PATH: u32 = 2;
pub const FPDF_PAGEOBJ_IMAGE: u32 = 3;
pub const FPDF_PAGEOBJ_SHADING: u32 = 4;
pub const FPDF_PAGEOBJ_FORM: u32 = 5;
pub const FPDF_SEGMENT_UNKNOWN: i32 = -1;
pub const FPDF_SEGMENT_LINETO: u32 = 0;
pub const FPDF_SEGMENT_BEZIERTO: u32 = 1;
pub const FPDF_SEGMENT_MOVETO: u32 = 2;
pub const FPDF_FILLMODE_NONE: u32 = 0;
pub const FPDF_FILLMODE_ALTERNATE: u32 = 1;
pub const FPDF_FILLMODE_WINDING: u32 = 2;
pub const FPDF_FONT_TYPE1: u32 = 1;
pub const FPDF_FONT_TRUETYPE: u32 = 2;
pub const FPDF_LINECAP_BUTT: u32 = 0;
pub const FPDF_LINECAP_ROUND: u32 = 1;
pub const FPDF_LINECAP_PROJECTING_SQUARE: u32 = 2;
pub const FPDF_LINEJOIN_MITER: u32 = 0;
pub const FPDF_LINEJOIN_ROUND: u32 = 1;
pub const FPDF_LINEJOIN_BEVEL: u32 = 2;
pub const FPDF_PRINTMODE_EMF: u32 = 0;
pub const FPDF_PRINTMODE_TEXTONLY: u32 = 1;
pub const FPDF_PRINTMODE_POSTSCRIPT2: u32 = 2;
pub const FPDF_PRINTMODE_POSTSCRIPT3: u32 = 3;
pub const FPDF_PRINTMODE_POSTSCRIPT2_PASSTHROUGH: u32 = 4;
pub const FPDF_PRINTMODE_POSTSCRIPT3_PASSTHROUGH: u32 = 5;
pub const FPDF_PRINTMODE_EMF_IMAGE_MASKS: u32 = 6;
pub const FPDF_PRINTMODE_POSTSCRIPT3_TYPE42: u32 = 7;
pub const FPDF_PRINTMODE_POSTSCRIPT3_TYPE42_PASSTHROUGH: u32 = 8;
pub const FPDF_UNSP_DOC_XFAFORM: u32 = 1;
pub const FPDF_UNSP_DOC_PORTABLECOLLECTION: u32 = 2;
pub const FPDF_UNSP_DOC_ATTACHMENT: u32 = 3;
pub const FPDF_UNSP_DOC_SECURITY: u32 = 4;
pub const FPDF_UNSP_DOC_SHAREDREVIEW: u32 = 5;
pub const FPDF_UNSP_DOC_SHAREDFORM_ACROBAT: u32 = 6;
pub const FPDF_UNSP_DOC_SHAREDFORM_FILESYSTEM: u32 = 7;
pub const FPDF_UNSP_DOC_SHAREDFORM_EMAIL: u32 = 8;
pub const FPDF_UNSP_ANNOT_3DANNOT: u32 = 11;
pub const FPDF_UNSP_ANNOT_MOVIE: u32 = 12;
pub const FPDF_UNSP_ANNOT_SOUND: u32 = 13;
pub const FPDF_UNSP_ANNOT_SCREEN_MEDIA: u32 = 14;
pub const FPDF_UNSP_ANNOT_SCREEN_RICHMEDIA: u32 = 15;
pub const FPDF_UNSP_ANNOT_ATTACHMENT: u32 = 16;
pub const FPDF_UNSP_ANNOT_SIG: u32 = 17;
pub const FPDF_RENDER_READY: u32 = 0;
pub const FPDF_RENDER_TOBECONTINUED: u32 = 1;
pub const FPDF_RENDER_DONE: u32 = 2;
pub const FPDF_RENDER_FAILED: u32 = 3;
pub const FPDF_INCREMENTAL: u32 = 1;
pub const FPDF_NO_INCREMENTAL: u32 = 2;
pub const FPDF_REMOVE_SECURITY_DEPRECATED: u32 = 3;
pub const FPDF_REMOVE_SECURITY: u32 = 4;
pub const FPDF_SUBSET_NEW_FONTS: u32 = 8;
pub const FPDF_MATCHCASE: u32 = 1;
pub const FPDF_MATCHWHOLEWORD: u32 = 2;
pub const FPDF_CONSECUTIVE: u32 = 4;
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_UNKNOWN: FPDF_TEXT_RENDERMODE = -1;
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL: FPDF_TEXT_RENDERMODE = 0;
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_STROKE: FPDF_TEXT_RENDERMODE = 1;
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_STROKE: FPDF_TEXT_RENDERMODE = 2;
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_INVISIBLE: FPDF_TEXT_RENDERMODE = 3;
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_CLIP: FPDF_TEXT_RENDERMODE = 4;
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_STROKE_CLIP: FPDF_TEXT_RENDERMODE = 5;
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_STROKE_CLIP: FPDF_TEXT_RENDERMODE = 6;
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_CLIP: FPDF_TEXT_RENDERMODE = 7;
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_LAST: FPDF_TEXT_RENDERMODE = 7;
pub type FPDF_TEXT_RENDERMODE = ::core::ffi::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_action_t__ {
    _unused: [u8; 0],
}
pub type FPDF_ACTION = *mut fpdf_action_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_annotation_t__ {
    _unused: [u8; 0],
}
pub type FPDF_ANNOTATION = *mut fpdf_annotation_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_attachment_t__ {
    _unused: [u8; 0],
}
pub type FPDF_ATTACHMENT = *mut fpdf_attachment_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_avail_t__ {
    _unused: [u8; 0],
}
pub type FPDF_AVAIL = *mut fpdf_avail_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_bitmap_t__ {
    _unused: [u8; 0],
}
pub type FPDF_BITMAP = *mut fpdf_bitmap_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_bookmark_t__ {
    _unused: [u8; 0],
}
pub type FPDF_BOOKMARK = *mut fpdf_bookmark_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_clippath_t__ {
    _unused: [u8; 0],
}
pub type FPDF_CLIPPATH = *mut fpdf_clippath_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_dest_t__ {
    _unused: [u8; 0],
}
pub type FPDF_DEST = *mut fpdf_dest_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_document_t__ {
    _unused: [u8; 0],
}
pub type FPDF_DOCUMENT = *mut fpdf_document_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_font_t__ {
    _unused: [u8; 0],
}
pub type FPDF_FONT = *mut fpdf_font_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_form_handle_t__ {
    _unused: [u8; 0],
}
pub type FPDF_FORMHANDLE = *mut fpdf_form_handle_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_glyphpath_t__ {
    _unused: [u8; 0],
}
pub type FPDF_GLYPHPATH = *const fpdf_glyphpath_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_javascript_action_t {
    _unused: [u8; 0],
}
pub type FPDF_JAVASCRIPT_ACTION = *mut fpdf_javascript_action_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_link_t__ {
    _unused: [u8; 0],
}
pub type FPDF_LINK = *mut fpdf_link_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_page_t__ {
    _unused: [u8; 0],
}
pub type FPDF_PAGE = *mut fpdf_page_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_pagelink_t__ {
    _unused: [u8; 0],
}
pub type FPDF_PAGELINK = *mut fpdf_pagelink_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_pageobject_t__ {
    _unused: [u8; 0],
}
pub type FPDF_PAGEOBJECT = *mut fpdf_pageobject_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_pageobjectmark_t__ {
    _unused: [u8; 0],
}
pub type FPDF_PAGEOBJECTMARK = *mut fpdf_pageobjectmark_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_pagerange_t__ {
    _unused: [u8; 0],
}
pub type FPDF_PAGERANGE = *const fpdf_pagerange_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_pathsegment_t {
    _unused: [u8; 0],
}
pub type FPDF_PATHSEGMENT = *const fpdf_pathsegment_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_schhandle_t__ {
    _unused: [u8; 0],
}
pub type FPDF_SCHHANDLE = *mut fpdf_schhandle_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_signature_t__ {
    _unused: [u8; 0],
}
pub type FPDF_SIGNATURE = *const fpdf_signature_t__;
pub type FPDF_SKIA_CANVAS = *mut ::core::ffi::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_structelement_t__ {
    _unused: [u8; 0],
}
pub type FPDF_STRUCTELEMENT = *mut fpdf_structelement_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_structelement_attr_t__ {
    _unused: [u8; 0],
}
pub type FPDF_STRUCTELEMENT_ATTR = *const fpdf_structelement_attr_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_structelement_attr_value_t__ {
    _unused: [u8; 0],
}
pub type FPDF_STRUCTELEMENT_ATTR_VALUE = *const fpdf_structelement_attr_value_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_structtree_t__ {
    _unused: [u8; 0],
}
pub type FPDF_STRUCTTREE = *mut fpdf_structtree_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_textpage_t__ {
    _unused: [u8; 0],
}
pub type FPDF_TEXTPAGE = *mut fpdf_textpage_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_widget_t__ {
    _unused: [u8; 0],
}
pub type FPDF_WIDGET = *mut fpdf_widget_t__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdf_xobject_t__ {
    _unused: [u8; 0],
}
pub type FPDF_XOBJECT = *mut fpdf_xobject_t__;
pub type FPDF_BOOL = ::core::ffi::c_int;
pub type FPDF_RESULT = ::core::ffi::c_int;
pub type FPDF_DWORD = ::core::ffi::c_ulong;
pub type FS_FLOAT = f32;
pub const _FPDF_DUPLEXTYPE__DuplexUndefined: _FPDF_DUPLEXTYPE_ = 0;
pub const _FPDF_DUPLEXTYPE__Simplex: _FPDF_DUPLEXTYPE_ = 1;
pub const _FPDF_DUPLEXTYPE__DuplexFlipShortEdge: _FPDF_DUPLEXTYPE_ = 2;
pub const _FPDF_DUPLEXTYPE__DuplexFlipLongEdge: _FPDF_DUPLEXTYPE_ = 3;
pub type _FPDF_DUPLEXTYPE_ = ::core::ffi::c_uint;
pub use self::_FPDF_DUPLEXTYPE_ as FPDF_DUPLEXTYPE;
pub type FPDF_WCHAR = ::core::ffi::c_ushort;
pub type FPDF_BYTESTRING = *const ::core::ffi::c_char;
pub type FPDF_WIDESTRING = *const FPDF_WCHAR;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FPDF_BSTR_ {
    pub str_: *mut ::core::ffi::c_char,
    pub len: ::core::ffi::c_int,
}
pub type FPDF_BSTR = FPDF_BSTR_;
pub type FPDF_STRING = *const ::core::ffi::c_char;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _FS_MATRIX_ {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}
pub type FS_MATRIX = _FS_MATRIX_;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _FS_RECTF_ {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
pub type FS_LPRECTF = *mut _FS_RECTF_;
pub type FS_RECTF = _FS_RECTF_;
pub type FS_LPCRECTF = *const FS_RECTF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FS_SIZEF_ {
    pub width: f32,
    pub height: f32,
}
pub type FS_LPSIZEF = *mut FS_SIZEF_;
pub type FS_SIZEF = FS_SIZEF_;
pub type FS_LPCSIZEF = *const FS_SIZEF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FS_POINTF_ {
    pub x: f32,
    pub y: f32,
}
pub type FS_LPPOINTF = *mut FS_POINTF_;
pub type FS_POINTF = FS_POINTF_;
pub type FS_LPCPOINTF = *const FS_POINTF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _FS_QUADPOINTSF {
    pub x1: FS_FLOAT,
    pub y1: FS_FLOAT,
    pub x2: FS_FLOAT,
    pub y2: FS_FLOAT,
    pub x3: FS_FLOAT,
    pub y3: FS_FLOAT,
    pub x4: FS_FLOAT,
    pub y4: FS_FLOAT,
}
pub type FS_QUADPOINTSF = _FS_QUADPOINTSF;
pub type FPDF_ANNOTATION_SUBTYPE = ::core::ffi::c_int;
pub type FPDF_ANNOT_APPEARANCEMODE = ::core::ffi::c_int;
pub type FPDF_OBJECT_TYPE = ::core::ffi::c_int;
pub const FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_AGG: FPDF_RENDERER_TYPE = 0;
pub const FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_SKIA: FPDF_RENDERER_TYPE = 1;
pub type FPDF_RENDERER_TYPE = ::core::ffi::c_uint;
pub const FPDF_FONT_BACKEND_TYPE_FPDF_FONTBACKENDTYPE_FREETYPE: FPDF_FONT_BACKEND_TYPE = 0;
pub const FPDF_FONT_BACKEND_TYPE_FPDF_FONTBACKENDTYPE_FONTATIONS: FPDF_FONT_BACKEND_TYPE = 1;
pub type FPDF_FONT_BACKEND_TYPE = ::core::ffi::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FPDF_LIBRARY_CONFIG_ {
    pub version: ::core::ffi::c_int,
    pub m_pUserFontPaths: *mut *const ::core::ffi::c_char,
    pub m_pIsolate: *mut ::core::ffi::c_void,
    pub m_v8EmbedderSlot: ::core::ffi::c_uint,
    pub m_pPlatform: *mut ::core::ffi::c_void,
    pub m_RendererType: FPDF_RENDERER_TYPE,
    pub m_FontLibraryType: FPDF_FONT_BACKEND_TYPE,
    pub m_BrotliEnabled: FPDF_BOOL,
    pub m_IsolatePerDocument: FPDF_BOOL,
}
pub type FPDF_LIBRARY_CONFIG = FPDF_LIBRARY_CONFIG_;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FPDF_FILEACCESS {
    pub m_FileLen: ::core::ffi::c_ulong,
    pub m_GetBlock: ::core::option::Option<
        unsafe extern "C" fn(
            param: *mut ::core::ffi::c_void,
            position: ::core::ffi::c_ulong,
            pBuf: *mut ::core::ffi::c_uchar,
            size: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_int,
    >,
    pub m_Param: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FPDF_FILEHANDLER_ {
    pub clientData: *mut ::core::ffi::c_void,
    pub Release: ::core::option::Option<unsafe extern "C" fn(clientData: *mut ::core::ffi::c_void)>,
    pub GetSize: ::core::option::Option<
        unsafe extern "C" fn(clientData: *mut ::core::ffi::c_void) -> FPDF_DWORD,
    >,
    pub ReadBlock: ::core::option::Option<
        unsafe extern "C" fn(
            clientData: *mut ::core::ffi::c_void,
            offset: FPDF_DWORD,
            buffer: *mut ::core::ffi::c_void,
            size: FPDF_DWORD,
        ) -> FPDF_RESULT,
    >,
    pub WriteBlock: ::core::option::Option<
        unsafe extern "C" fn(
            clientData: *mut ::core::ffi::c_void,
            offset: FPDF_DWORD,
            buffer: *const ::core::ffi::c_void,
            size: FPDF_DWORD,
        ) -> FPDF_RESULT,
    >,
    pub Flush: ::core::option::Option<
        unsafe extern "C" fn(clientData: *mut ::core::ffi::c_void) -> FPDF_RESULT,
    >,
    pub Truncate: ::core::option::Option<
        unsafe extern "C" fn(clientData: *mut ::core::ffi::c_void, size: FPDF_DWORD) -> FPDF_RESULT,
    >,
}
pub type FPDF_FILEHANDLER = FPDF_FILEHANDLER_;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FPDF_COLORSCHEME_ {
    pub path_fill_color: FPDF_DWORD,
    pub path_stroke_color: FPDF_DWORD,
    pub text_fill_color: FPDF_DWORD,
    pub text_stroke_color: FPDF_DWORD,
}
pub type FPDF_COLORSCHEME = FPDF_COLORSCHEME_;
pub const FPDF_TEXT_DIRECTION_FPDF_TEXTDIR_UNKNOWN: FPDF_TEXT_DIRECTION = 0;
pub const FPDF_TEXT_DIRECTION_FPDF_TEXTDIR_AUTO: FPDF_TEXT_DIRECTION = 1;
pub const FPDF_TEXT_DIRECTION_FPDF_TEXTDIR_LTR: FPDF_TEXT_DIRECTION = 2;
pub const FPDF_TEXT_DIRECTION_FPDF_TEXTDIR_RTL: FPDF_TEXT_DIRECTION = 3;
pub type FPDF_TEXT_DIRECTION = ::core::ffi::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IPDF_JsPlatform {
    pub version: ::core::ffi::c_int,
    pub app_alert: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _IPDF_JsPlatform,
            Msg: FPDF_WIDESTRING,
            Title: FPDF_WIDESTRING,
            Type: ::core::ffi::c_int,
            Icon: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub app_beep: ::core::option::Option<
        unsafe extern "C" fn(pThis: *mut _IPDF_JsPlatform, nType: ::core::ffi::c_int),
    >,
    pub app_response: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _IPDF_JsPlatform,
            Question: FPDF_WIDESTRING,
            Title: FPDF_WIDESTRING,
            Default: FPDF_WIDESTRING,
            cLabel: FPDF_WIDESTRING,
            bPassword: FPDF_BOOL,
            response: *mut ::core::ffi::c_void,
            length: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub Doc_getFilePath: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _IPDF_JsPlatform,
            filePath: *mut ::core::ffi::c_void,
            length: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub Doc_mail: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _IPDF_JsPlatform,
            mailData: *mut ::core::ffi::c_void,
            length: ::core::ffi::c_int,
            bUI: FPDF_BOOL,
            To: FPDF_WIDESTRING,
            Subject: FPDF_WIDESTRING,
            CC: FPDF_WIDESTRING,
            BCC: FPDF_WIDESTRING,
            Msg: FPDF_WIDESTRING,
        ),
    >,
    pub Doc_print: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _IPDF_JsPlatform,
            bUI: FPDF_BOOL,
            nStart: ::core::ffi::c_int,
            nEnd: ::core::ffi::c_int,
            bSilent: FPDF_BOOL,
            bShrinkToFit: FPDF_BOOL,
            bPrintAsImage: FPDF_BOOL,
            bReverse: FPDF_BOOL,
            bAnnotations: FPDF_BOOL,
        ),
    >,
    pub Doc_submitForm: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _IPDF_JsPlatform,
            formData: *mut ::core::ffi::c_void,
            length: ::core::ffi::c_int,
            URL: FPDF_WIDESTRING,
        ),
    >,
    pub Doc_gotoPage: ::core::option::Option<
        unsafe extern "C" fn(pThis: *mut _IPDF_JsPlatform, nPageNum: ::core::ffi::c_int),
    >,
    pub Field_browse: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _IPDF_JsPlatform,
            filePath: *mut ::core::ffi::c_void,
            length: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub m_pFormfillinfo: *mut ::core::ffi::c_void,
    pub m_isolate: *mut ::core::ffi::c_void,
    pub m_v8EmbedderSlot: ::core::ffi::c_uint,
}
pub type IPDF_JSPLATFORM = _IPDF_JsPlatform;
pub type TimerCallback = ::core::option::Option<unsafe extern "C" fn(idEvent: ::core::ffi::c_int)>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _FPDF_SYSTEMTIME {
    pub wYear: ::core::ffi::c_ushort,
    pub wMonth: ::core::ffi::c_ushort,
    pub wDayOfWeek: ::core::ffi::c_ushort,
    pub wDay: ::core::ffi::c_ushort,
    pub wHour: ::core::ffi::c_ushort,
    pub wMinute: ::core::ffi::c_ushort,
    pub wSecond: ::core::ffi::c_ushort,
    pub wMilliseconds: ::core::ffi::c_ushort,
}
pub type FPDF_SYSTEMTIME = _FPDF_SYSTEMTIME;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _FPDF_FORMFILLINFO {
    pub version: ::core::ffi::c_int,
    pub Release: ::core::option::Option<unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO)>,
    pub FFI_Invalidate: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            page: FPDF_PAGE,
            left: f64,
            top: f64,
            right: f64,
            bottom: f64,
        ),
    >,
    pub FFI_OutputSelectedRect: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            page: FPDF_PAGE,
            left: f64,
            top: f64,
            right: f64,
            bottom: f64,
        ),
    >,
    pub FFI_SetCursor: ::core::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO, nCursorType: ::core::ffi::c_int),
    >,
    pub FFI_SetTimer: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            uElapse: ::core::ffi::c_int,
            lpTimerFunc: TimerCallback,
        ) -> ::core::ffi::c_int,
    >,
    pub FFI_KillTimer: ::core::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO, nTimerID: ::core::ffi::c_int),
    >,
    pub FFI_GetLocalTime: ::core::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO) -> FPDF_SYSTEMTIME,
    >,
    pub FFI_OnChange: ::core::option::Option<unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO)>,
    pub FFI_GetPage: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            document: FPDF_DOCUMENT,
            nPageIndex: ::core::ffi::c_int,
        ) -> FPDF_PAGE,
    >,
    pub FFI_GetCurrentPage: ::core::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO, document: FPDF_DOCUMENT) -> FPDF_PAGE,
    >,
    pub FFI_GetRotation: ::core::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO, page: FPDF_PAGE) -> ::core::ffi::c_int,
    >,
    pub FFI_ExecuteNamedAction: ::core::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO, namedAction: FPDF_BYTESTRING),
    >,
    pub FFI_SetTextFieldFocus: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            value: FPDF_WIDESTRING,
            valueLen: FPDF_DWORD,
            is_focus: FPDF_BOOL,
        ),
    >,
    pub FFI_DoURIAction: ::core::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO, bsURI: FPDF_BYTESTRING),
    >,
    pub FFI_DoGoToAction: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            nPageIndex: ::core::ffi::c_int,
            zoomMode: ::core::ffi::c_int,
            fPosArray: *mut f32,
            sizeofArray: ::core::ffi::c_int,
        ),
    >,
    pub m_pJsPlatform: *mut IPDF_JSPLATFORM,
    pub xfa_disabled: FPDF_BOOL,
    pub FFI_DisplayCaret: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            page: FPDF_PAGE,
            bVisible: FPDF_BOOL,
            left: f64,
            top: f64,
            right: f64,
            bottom: f64,
        ),
    >,
    pub FFI_GetCurrentPageIndex: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            document: FPDF_DOCUMENT,
        ) -> ::core::ffi::c_int,
    >,
    pub FFI_SetCurrentPage: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            document: FPDF_DOCUMENT,
            iCurPage: ::core::ffi::c_int,
        ),
    >,
    pub FFI_GotoURL: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            document: FPDF_DOCUMENT,
            wsURL: FPDF_WIDESTRING,
        ),
    >,
    pub FFI_GetPageViewRect: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            page: FPDF_PAGE,
            left: *mut f64,
            top: *mut f64,
            right: *mut f64,
            bottom: *mut f64,
        ),
    >,
    pub FFI_PageEvent: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            page_count: ::core::ffi::c_int,
            event_type: FPDF_DWORD,
        ),
    >,
    pub FFI_PopupMenu: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            page: FPDF_PAGE,
            hWidget: FPDF_WIDGET,
            menuFlag: ::core::ffi::c_int,
            x: f32,
            y: f32,
        ) -> FPDF_BOOL,
    >,
    pub FFI_OpenFile: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            fileFlag: ::core::ffi::c_int,
            wsURL: FPDF_WIDESTRING,
            mode: *const ::core::ffi::c_char,
        ) -> *mut FPDF_FILEHANDLER,
    >,
    pub FFI_EmailTo: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            fileHandler: *mut FPDF_FILEHANDLER,
            pTo: FPDF_WIDESTRING,
            pSubject: FPDF_WIDESTRING,
            pCC: FPDF_WIDESTRING,
            pBcc: FPDF_WIDESTRING,
            pMsg: FPDF_WIDESTRING,
        ),
    >,
    pub FFI_UploadTo: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            fileHandler: *mut FPDF_FILEHANDLER,
            fileFlag: ::core::ffi::c_int,
            uploadTo: FPDF_WIDESTRING,
        ),
    >,
    pub FFI_GetPlatform: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            platform: *mut ::core::ffi::c_void,
            length: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub FFI_GetLanguage: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            language: *mut ::core::ffi::c_void,
            length: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub FFI_DownloadFromURL: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            URL: FPDF_WIDESTRING,
        ) -> *mut FPDF_FILEHANDLER,
    >,
    pub FFI_PostRequestURL: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            wsURL: FPDF_WIDESTRING,
            wsData: FPDF_WIDESTRING,
            wsContentType: FPDF_WIDESTRING,
            wsEncode: FPDF_WIDESTRING,
            wsHeader: FPDF_WIDESTRING,
            response: *mut FPDF_BSTR,
        ) -> FPDF_BOOL,
    >,
    pub FFI_PutRequestURL: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            wsURL: FPDF_WIDESTRING,
            wsData: FPDF_WIDESTRING,
            wsEncode: FPDF_WIDESTRING,
        ) -> FPDF_BOOL,
    >,
    pub FFI_OnFocusChange: ::core::option::Option<
        unsafe extern "C" fn(
            param: *mut _FPDF_FORMFILLINFO,
            annot: FPDF_ANNOTATION,
            page_index: ::core::ffi::c_int,
        ),
    >,
    pub FFI_DoURIActionWithKeyboardModifier: ::core::option::Option<
        unsafe extern "C" fn(
            param: *mut _FPDF_FORMFILLINFO,
            uri: FPDF_BYTESTRING,
            modifiers: ::core::ffi::c_int,
        ),
    >,
}
pub type FPDF_FORMFILLINFO = _FPDF_FORMFILLINFO;
pub const FPDFANNOT_COLORTYPE_FPDFANNOT_COLORTYPE_Color: FPDFANNOT_COLORTYPE = 0;
pub const FPDFANNOT_COLORTYPE_FPDFANNOT_COLORTYPE_InteriorColor: FPDFANNOT_COLORTYPE = 1;
pub type FPDFANNOT_COLORTYPE = ::core::ffi::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _FX_FILEAVAIL {
    pub version: ::core::ffi::c_int,
    pub IsDataAvail: ::core::option::Option<
        unsafe extern "C" fn(pThis: *mut _FX_FILEAVAIL, offset: usize, size: usize) -> FPDF_BOOL,
    >,
}
pub type FX_FILEAVAIL = _FX_FILEAVAIL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _FX_DOWNLOADHINTS {
    pub version: ::core::ffi::c_int,
    pub AddSegment: ::core::option::Option<
        unsafe extern "C" fn(pThis: *mut _FX_DOWNLOADHINTS, offset: usize, size: usize),
    >,
}
pub type FX_DOWNLOADHINTS = _FX_DOWNLOADHINTS;
pub const FPDF_FILEIDTYPE_FILEIDTYPE_PERMANENT: FPDF_FILEIDTYPE = 0;
pub const FPDF_FILEIDTYPE_FILEIDTYPE_CHANGING: FPDF_FILEIDTYPE = 1;
pub type FPDF_FILEIDTYPE = ::core::ffi::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FPDF_IMAGEOBJ_METADATA {
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
    pub horizontal_dpi: f32,
    pub vertical_dpi: f32,
    pub bits_per_pixel: ::core::ffi::c_uint,
    pub colorspace: ::core::ffi::c_int,
    pub marked_content_id: ::core::ffi::c_int,
}
pub type time_t = ::core::ffi::c_long;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tm {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _UNSUPPORT_INFO {
    pub version: ::core::ffi::c_int,
    pub FSDK_UnSupport_Handler: ::core::option::Option<
        unsafe extern "C" fn(pThis: *mut _UNSUPPORT_INFO, nType: ::core::ffi::c_int),
    >,
}
pub type UNSUPPORT_INFO = _UNSUPPORT_INFO;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IFSDK_PAUSE {
    pub version: ::core::ffi::c_int,
    pub NeedToPauseNow:
        ::core::option::Option<unsafe extern "C" fn(pThis: *mut _IFSDK_PAUSE) -> FPDF_BOOL>,
    pub user: *mut ::core::ffi::c_void,
}
pub type IFSDK_PAUSE = _IFSDK_PAUSE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FPDF_FILEWRITE_ {
    pub version: ::core::ffi::c_int,
    pub WriteBlock: ::core::option::Option<
        unsafe extern "C" fn(
            self_: *mut FPDF_FILEWRITE_,
            data: *const ::core::ffi::c_void,
            size: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_int,
    >,
}
pub type FPDF_FILEWRITE = FPDF_FILEWRITE_;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _FPDF_SYSFONTINFO {
    pub version: ::core::ffi::c_int,
    pub Release: ::core::option::Option<unsafe extern "C" fn(pThis: *mut _FPDF_SYSFONTINFO)>,
    pub EnumFonts: ::core::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_SYSFONTINFO, pMapper: *mut ::core::ffi::c_void),
    >,
    pub MapFont: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_SYSFONTINFO,
            weight: ::core::ffi::c_int,
            bItalic: FPDF_BOOL,
            charset: ::core::ffi::c_int,
            pitch_family: ::core::ffi::c_int,
            face: *const ::core::ffi::c_char,
            bExact: *mut FPDF_BOOL,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub GetFont: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_SYSFONTINFO,
            face: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub GetFontData: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_SYSFONTINFO,
            hFont: *mut ::core::ffi::c_void,
            table: ::core::ffi::c_uint,
            buffer: *mut ::core::ffi::c_uchar,
            buf_size: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
    >,
    pub GetFaceName: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_SYSFONTINFO,
            hFont: *mut ::core::ffi::c_void,
            buffer: *mut ::core::ffi::c_char,
            buf_size: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
    >,
    pub GetFontCharset: ::core::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_SYSFONTINFO,
            hFont: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub DeleteFont: ::core::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_SYSFONTINFO, hFont: *mut ::core::ffi::c_void),
    >,
}
pub type FPDF_SYSFONTINFO = _FPDF_SYSFONTINFO;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FPDF_CharsetFontMap_ {
    pub charset: ::core::ffi::c_int,
    pub fontname: *const ::core::ffi::c_char,
}
pub type FPDF_CharsetFontMap = FPDF_CharsetFontMap_;

