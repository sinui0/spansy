use std::ops::Range;

/// Returns the range within the source string corresponding to the span.
///
/// # Panics
///
/// Panics if the span is not within the source string.
pub(crate) fn get_span_range(src: &[u8], span: &[u8]) -> Range<usize> {
    let src_ptr = src.as_ptr();
    let span_ptr = span.as_ptr();

    assert!(span_ptr >= src_ptr, "span is not within source string");

    let start = unsafe { span_ptr.offset_from(src_ptr) } as usize;

    start..start + span.len()
}
