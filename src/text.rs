//! Owned text-page handles and UTF-16 text extraction.
use crate::{
    handles::ManagedResource,
    raw::FPDF_TEXTPAGE,
    runtime::{self, loader::LoadedLibrary, Phase},
    PdfiumError,
};
use std::{any::Any, sync::Arc};

/// An owned text span and the bounds of its PDFium text rectangle.
///
/// The bounds are in PDF page user space, measured in PDF points. Their
/// orientation and values are preserved from PDFium, and rectangles retain
/// PDFium's line and text-style grouping and rectangle order. `text` contains
/// characters in document order whose complete character boxes fall within
/// these bounds, including boxes that touch an edge. Characters with an
/// unavailable box or Unicode value are omitted. All fields are owned values
/// and remain usable independently of the originating [`TextPage`].
#[derive(Clone, Debug, PartialEq)]
pub struct TextSegment {
    pub text: String,
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
    pub top: f64,
}

struct TextPageInner {
    resource: ManagedResource<FPDF_TEXTPAGE>,
    source: Option<Arc<dyn crate::callback::SourceFailureCheck>>,
}
// Text pointers are accessed only under the serialized PDFium coordinator.
unsafe impl Send for TextPageInner {}
// All native text operations and finalization are serialized under the coordinator.
unsafe impl Sync for TextPageInner {}
/// Clonable PDFium text page retaining its source page.
#[derive(Clone)]
pub struct TextPage(Arc<TextPageInner>);
fn close_text_page(ctx: &LoadedLibrary, handle: FPDF_TEXTPAGE) {
    unsafe { ctx.dispatch.FPDFText_ClosePage(handle) }
}
impl TextPage {
    pub(crate) fn from_raw(
        handle: FPDF_TEXTPAGE,
        keep: Vec<Arc<dyn Any + Send + Sync>>,
        source: Option<Arc<dyn crate::callback::SourceFailureCheck>>,
    ) -> Self {
        Self(Arc::new(TextPageInner {
            resource: ManagedResource::new(handle, close_text_page, keep),
            source,
        }))
    }
    /// Copies all page text into an owned UTF-8 string.
    pub fn extract_text(&self) -> Result<String, PdfiumError> {
        let handle = self.0.resource.handle();
        let (count, callback_error) = runtime::call_with_source(
            "FPDFText_CountChars",
            Phase::Initialized,
            |ctx| unsafe { ctx.dispatch.FPDFText_CountChars(handle) },
            self.0.source.as_deref(),
        )?;
        if let Some(error) = callback_error {
            return Err(error);
        }
        let count = count.return_value;
        let count = checked_native_count(count)?;
        if count == 0 {
            return Ok(String::new());
        }
        let capacity = count.checked_add(1).ok_or(PdfiumError::Allocation {
            requested_bytes: usize::MAX,
        })?;
        let requested_bytes =
            capacity
                .checked_mul(std::mem::size_of::<u16>())
                .ok_or(PdfiumError::Allocation {
                    requested_bytes: usize::MAX,
                })?;
        let mut units = Vec::<u16>::new();
        units
            .try_reserve_exact(capacity)
            .map_err(|_| PdfiumError::Allocation { requested_bytes })?;
        units.resize(capacity, 0);
        let native_count = checked_native_count_argument(count)?;
        let (returned, callback_error) = runtime::call_with_source(
            "FPDFText_GetText",
            Phase::Initialized,
            |ctx| unsafe {
                ctx.dispatch
                    .FPDFText_GetText(handle, 0, native_count, units.as_mut_ptr())
            },
            self.0.source.as_deref(),
        )?;
        if let Some(error) = callback_error {
            return Err(error);
        }
        let returned = returned.return_value;
        decode_text(count, &units, returned)
    }

    /// Returns owned text and bounds for PDFium's text rectangles.
    ///
    /// Bounds use PDF page user space in PDF points, with the coordinate
    /// orientation and rectangle order returned by PDFium. PDFium's existing
    /// line and text-style grouping is preserved. A character is included
    /// only when its complete box is contained in the rectangle; edges match
    /// inclusively. Characters whose boxes or Unicode values are unavailable
    /// are omitted. Returned strings and bounds are independent owned values
    /// and remain valid after this `TextPage` is dropped.
    pub fn segments(&self) -> Result<Vec<TextSegment>, PdfiumError> {
        let handle = self.0.resource.handle();
        let source = self.0.source.as_deref();
        runtime::call_sequence(Phase::Initialized, source, |calls| {
            let (char_count_call, source_error) = calls
                .call("FPDFText_CountChars", |ctx| unsafe {
                    ctx.dispatch.FPDFText_CountChars(handle)
                })?;
            if let Some(error) = source_error {
                return Err(error);
            }
            let char_count = match segment_counts(char_count_call.return_value, None)? {
                SegmentCounts::Empty => return Ok(Vec::new()),
                SegmentCounts::NeedRectCount(chars) => chars,
                SegmentCounts::Ready { .. } => unreachable!("rect count not queried yet"),
            };
            let char_count_arg = checked_native_count_argument(char_count)?;
            let (rect_count_call, source_error) = calls
                .call("FPDFText_CountRects", |ctx| unsafe {
                    ctx.dispatch.FPDFText_CountRects(handle, 0, char_count_arg)
                })?;
            if let Some(error) = source_error {
                return Err(error);
            }
            let (char_count, rect_count) = match segment_counts(
                char_count_call.return_value,
                Some(rect_count_call.return_value),
            )? {
                SegmentCounts::Empty => return Ok(Vec::new()),
                SegmentCounts::Ready { chars, rects } => (chars, rects),
                SegmentCounts::NeedRectCount(_) => unreachable!("rect count was provided"),
            };
            let rect_count_arg = checked_native_count_argument(rect_count)?;
            let mut segments = Vec::new();
            let requested_bytes =
                checked_allocation_bytes(rect_count, std::mem::size_of::<TextSegment>())?;
            segments
                .try_reserve_exact(rect_count)
                .map_err(|_| PdfiumError::Allocation { requested_bytes })?;
            let characters = collect_character_records(char_count, |index| {
                let index = checked_native_count_argument(index)?;
                let (box_call, source_error) = calls.call("FPDFText_GetCharBox", |ctx| unsafe {
                    let (mut left, mut right, mut bottom, mut top) = (0.0, 0.0, 0.0, 0.0);
                    let ok = ctx.dispatch.FPDFText_GetCharBox(
                        handle,
                        index,
                        &mut left,
                        &mut right,
                        &mut bottom,
                        &mut top,
                    );
                    (ok, left, bottom, right, top)
                })?;
                if let Some(error) = source_error {
                    return Err(error);
                }
                let (ok, left, bottom, right, top) = box_call.return_value;
                if ok == 0 {
                    return Ok((None, None));
                }
                let (unicode_call, source_error) = calls
                    .call("FPDFText_GetUnicode", |ctx| unsafe {
                        ctx.dispatch.FPDFText_GetUnicode(handle, index)
                    })?;
                if let Some(error) = source_error {
                    return Err(error);
                }
                let scalar = char::from_u32(unicode_call.return_value)
                    .filter(|_| unicode_call.return_value != 0);
                Ok((Some((left, bottom, right, top)), scalar))
            })?;
            for rect_index in 0..rect_count_arg {
                let (rect_call, source_error) = calls.call("FPDFText_GetRect", |ctx| unsafe {
                    let (mut left, mut top, mut right, mut bottom) = (0.0, 0.0, 0.0, 0.0);
                    let ok = ctx.dispatch.FPDFText_GetRect(
                        handle,
                        rect_index,
                        &mut left,
                        &mut top,
                        &mut right,
                        &mut bottom,
                    );
                    (ok, left, bottom, right, top)
                })?;
                if let Some(error) = source_error {
                    return Err(error);
                }
                let (ok, left, bottom, right, top) = rect_call.return_value;
                rectangle_result(ok, rect_call.last_error)?;
                let mut text = String::new();
                let estimate = segment_text_capacity((left, bottom, right, top), &characters)?;
                text.try_reserve_exact(estimate)
                    .map_err(|_| PdfiumError::Allocation {
                        requested_bytes: estimate,
                    })?;
                append_segment_text(&mut text, (left, bottom, right, top), &characters);
                segments.push(TextSegment {
                    text,
                    left,
                    bottom,
                    right,
                    top,
                });
            }
            Ok(segments)
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SegmentCounts {
    Empty,
    NeedRectCount(usize),
    Ready { chars: usize, rects: usize },
}

fn segment_counts(char_count: i32, rect_count: Option<i32>) -> Result<SegmentCounts, PdfiumError> {
    let chars = checked_native_count(char_count)?;
    if chars == 0 {
        return Ok(SegmentCounts::Empty);
    }
    let Some(rect_count) = rect_count else {
        return Ok(SegmentCounts::NeedRectCount(chars));
    };
    let rects = checked_native_count(rect_count)?;
    Ok(if rects == 0 {
        SegmentCounts::Empty
    } else {
        SegmentCounts::Ready { chars, rects }
    })
}

fn rectangle_result(ok: i32, last_error: u64) -> Result<(), PdfiumError> {
    if ok != 0 {
        Ok(())
    } else {
        Err(PdfiumError::Operation {
            function: "FPDFText_GetRect".into(),
            last_error,
        })
    }
}

fn checked_allocation_bytes(count: usize, element_size: usize) -> Result<usize, PdfiumError> {
    count
        .checked_mul(element_size)
        .ok_or(PdfiumError::Allocation {
            requested_bytes: usize::MAX,
        })
}

fn collect_character_records<F>(
    count: usize,
    mut fetch: F,
) -> Result<Vec<CharacterRecord>, PdfiumError>
where
    F: FnMut(usize) -> Result<CharacterRecord, PdfiumError>,
{
    let requested_bytes = checked_allocation_bytes(count, std::mem::size_of::<CharacterRecord>())?;
    let mut records = Vec::new();
    records
        .try_reserve_exact(count)
        .map_err(|_| PdfiumError::Allocation { requested_bytes })?;
    for index in 0..count {
        records.push(fetch(index)?);
    }
    Ok(records)
}

fn decode_text(
    character_count: usize,
    units: &[u16],
    returned: i32,
) -> Result<String, PdfiumError> {
    if character_count == 0 {
        return Ok(String::new());
    }
    let returned = usize::try_from(returned)
        .map_err(|_| text_decode_error("PDFium returned a negative UTF-16 unit count"))?;
    if returned == 0 {
        return Err(text_decode_error(
            "PDFium returned no text for a nonempty page",
        ));
    }
    if returned > units.len() {
        return Err(text_decode_error(
            "PDFium returned more UTF-16 units than the output buffer",
        ));
    }
    if units[returned - 1] != 0 {
        return Err(text_decode_error(
            "PDFium omitted the required UTF-16 terminator",
        ));
    }
    let content = &units[..returned - 1];
    let max_bytes = content
        .len()
        .checked_mul(3)
        .ok_or(PdfiumError::Allocation {
            requested_bytes: usize::MAX,
        })?;
    let mut output = String::new();
    output
        .try_reserve_exact(max_bytes)
        .map_err(|_| PdfiumError::Allocation {
            requested_bytes: max_bytes,
        })?;
    for item in char::decode_utf16(content.iter().copied()) {
        output.push(item.map_err(|error| text_decode_error(&error.to_string()))?);
    }
    Ok(output)
}
fn text_decode_error(message: &str) -> PdfiumError {
    PdfiumError::TextDecoding {
        message: message.into(),
    }
}

fn checked_native_count(count: i32) -> Result<usize, PdfiumError> {
    usize::try_from(count).map_err(|_| PdfiumError::InvalidInput {
        operation: "PDFium text count".into(),
        message: "PDFium returned a negative or unrepresentable count".into(),
    })
}

fn checked_native_count_argument(count: usize) -> Result<i32, PdfiumError> {
    i32::try_from(count).map_err(|_| PdfiumError::Allocation {
        requested_bytes: count,
    })
}

#[cfg_attr(not(test), allow(dead_code))]
type Bounds = (f64, f64, f64, f64);
#[cfg_attr(not(test), allow(dead_code))]
type CharacterRecord = (Option<Bounds>, Option<char>);

#[cfg_attr(not(test), allow(dead_code))]
fn valid_bounds((left, bottom, right, top): Bounds) -> bool {
    left.is_finite()
        && bottom.is_finite()
        && right.is_finite()
        && top.is_finite()
        && left <= right
        && bottom <= top
}

#[cfg_attr(not(test), allow(dead_code))]
fn segment_contains_box(rect: Bounds, character: Bounds) -> bool {
    valid_bounds(rect)
        && valid_bounds(character)
        && character.0 >= rect.0
        && character.1 >= rect.1
        && character.2 <= rect.2
        && character.3 <= rect.3
}

#[cfg_attr(not(test), allow(dead_code))]
fn segment_text(rect: Bounds, characters: &[CharacterRecord]) -> String {
    let mut text = String::new();
    append_segment_text(&mut text, rect, characters);
    text
}

fn append_segment_text(text: &mut String, rect: Bounds, characters: &[CharacterRecord]) {
    for (bounds, unicode) in characters {
        if let (Some(bounds), Some(unicode)) = (bounds, unicode) {
            if segment_contains_box(rect, *bounds) {
                text.push(*unicode);
            }
        }
    }
}

fn segment_text_capacity(
    rect: Bounds,
    characters: &[CharacterRecord],
) -> Result<usize, PdfiumError> {
    characters
        .iter()
        .try_fold(0usize, |capacity, (bounds, unicode)| {
            let (Some(bounds), Some(unicode)) = (bounds, unicode) else {
                return Ok(capacity);
            };
            if !segment_contains_box(rect, *bounds) || *unicode == '\0' {
                return Ok(capacity);
            }
            capacity
                .checked_add(unicode.len_utf8())
                .ok_or(PdfiumError::Allocation {
                    requested_bytes: usize::MAX,
                })
        })
}
#[cfg(test)]
mod tests {
    use super::{
        checked_allocation_bytes, checked_native_count, checked_native_count_argument,
        collect_character_records, decode_text, rectangle_result, segment_contains_box,
        segment_counts, segment_text, segment_text_capacity, SegmentCounts,
    };
    use crate::PdfiumError;

    #[test]
    fn segments_includes_fully_contained_boxes_with_inclusive_edges() {
        let bounds = (0.0, 0.0, 10.0, 10.0);
        let chars = [
            (Some((0.0, 0.0, 2.0, 2.0)), Some('A')),
            (Some((8.0, 8.0, 10.0, 10.0)), Some('B')),
            (Some((4.0, 4.0, 6.0, 6.0)), Some('C')),
        ];
        assert!(segment_contains_box(bounds, (0.0, 0.0, 2.0, 2.0)));
        assert_eq!(segment_text(bounds, &chars), "ABC");
    }

    #[test]
    fn segments_omits_partial_invalid_and_unmapped_characters() {
        let bounds = (0.0, 0.0, 10.0, 10.0);
        let chars = [
            (Some((1.0, 1.0, 2.0, 2.0)), Some('A')),
            (Some((-1.0, 1.0, 2.0, 2.0)), Some('x')),
            (Some((f64::NAN, 1.0, 2.0, 2.0)), Some('y')),
            (Some((2.0, 2.0, 3.0, 3.0)), None),
            (None, Some('z')),
            (Some((4.0, 4.0, 3.0, 5.0)), Some('w')),
            (Some((2.0, 2.0, 3.0, 3.0)), Some('B')),
        ];
        assert_eq!(segment_text(bounds, &chars), "AB");
    }

    #[test]
    fn segments_checked_count_accepts_zero_and_positive_and_rejects_negative_and_overflow() {
        assert_eq!(checked_native_count(0).unwrap(), 0);
        assert_eq!(checked_native_count(7).unwrap(), 7);
        assert!(checked_native_count(-1).is_err());
        assert!(checked_native_count_argument(usize::MAX).is_err());
    }

    #[test]
    fn segments_returns_empty_for_empty_text_counts() {
        assert_eq!(segment_counts(0, None).unwrap(), SegmentCounts::Empty);
        assert_eq!(segment_counts(1, Some(0)).unwrap(), SegmentCounts::Empty);
    }

    #[test]
    fn segments_rejects_negative_rectangle_count() {
        assert!(matches!(
            segment_counts(1, Some(-1)),
            Err(PdfiumError::InvalidInput { .. })
        ));
    }

    #[test]
    fn segments_failed_rectangle_retrieval_is_an_operation_error() {
        assert!(matches!(
            rectangle_result(0, 73),
            Err(PdfiumError::Operation { ref function, last_error: 73 }) if function == "FPDFText_GetRect"
        ));
    }

    #[test]
    fn segments_allocation_size_checks_overflow() {
        assert_eq!(checked_allocation_bytes(4, 8).unwrap(), 32);
        assert!(matches!(
            checked_allocation_bytes(usize::MAX, 2),
            Err(PdfiumError::Allocation {
                requested_bytes: usize::MAX
            })
        ));
    }

    #[test]
    fn segments_capacity_counts_only_accepted_utf8_bytes() {
        let rect = (0.0, 0.0, 10.0, 10.0);
        let chars = [
            (Some((1.0, 1.0, 2.0, 2.0)), Some('A')),
            (Some((11.0, 1.0, 12.0, 2.0)), Some('x')),
            (Some((2.0, 2.0, 3.0, 3.0)), None),
            (Some((3.0, 3.0, 4.0, 4.0)), Some('界')),
            (None, Some('😃')),
        ];
        assert_eq!(segment_text_capacity(rect, &chars).unwrap(), 4);
    }

    #[test]
    fn segments_collect_character_records_once_for_multiple_rectangles() {
        let mut fetch_count = 0;
        let characters = collect_character_records(3, |index| {
            fetch_count += 1;
            Ok(match index {
                0 => (Some((0.0, 0.0, 2.0, 2.0)), Some('A')),
                1 => (Some((3.0, 0.0, 5.0, 2.0)), Some('B')),
                _ => (Some((6.0, 0.0, 8.0, 2.0)), Some('C')),
            })
        })
        .unwrap();

        let first_rect = (0.0, 0.0, 5.0, 2.0);
        let second_rect = (6.0, 0.0, 8.0, 2.0);
        assert_eq!(fetch_count, 3);
        assert_eq!(segment_text(first_rect, &characters), "AB");
        assert_eq!(segment_text(second_rect, &characters), "C");
    }

    #[test]
    fn validates_counts_terminator_truncation_and_utf16() {
        assert_eq!(decode_text(0, &[0], 0).unwrap(), "");
        assert_eq!(decode_text(1, &[65, 0], 2).unwrap(), "A");
        assert_eq!(decode_text(1, &[65, 0, 0], 2).unwrap(), "A");
        assert!(decode_text(1, &[65, 0], 0).is_err());
        assert!(decode_text(1, &[65, 0], 3).is_err());
        assert!(decode_text(1, &[65, 66], 2).is_err());
        assert!(decode_text(1, &[0xd800, 0], 2).is_err());
    }

    #[test]
    fn rejects_negative_unit_counts_and_invalid_surrogate_pairs() {
        assert!(matches!(
            decode_text(1, &[65, 0], -1),
            Err(PdfiumError::TextDecoding { .. })
        ));
        assert!(matches!(
            decode_text(2, &[0xdc00, 0], 2),
            Err(PdfiumError::TextDecoding { .. })
        ));
        assert!(matches!(
            decode_text(2, &[65, 0xdc00, 0], 3),
            Err(PdfiumError::TextDecoding { .. })
        ));
    }

    #[test]
    fn decodes_supplementary_unicode_and_ignores_trailing_buffer_units() {
        assert_eq!(decode_text(2, &[0xd83d, 0xde03, 0, 65], 3).unwrap(), "😃");
    }
}
