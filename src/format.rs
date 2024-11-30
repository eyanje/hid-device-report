use std::cmp::min;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut, SliceIndex};

use super::field_types::ReportId;
use super::report::Report;

type Size = u32;

/// Error type when a given bit slice has bits outside the sliced range.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DataOutOfBoundsError {}

impl Display for DataOutOfBoundsError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        "data bits out of defined size".fmt(fmt)
    }
}
impl Error for DataOutOfBoundsError {}


/// Error type when a report size is larger than allowed.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TooLargeError {}
impl Display for TooLargeError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        "size too large for a report".fmt(fmt)
    }
}
impl Error for TooLargeError {}

/// Take a slice that holds only the bytes in the given bit range.
///
/// The returned slice is guaranteed to be at most the given size. It is possible that the returned
/// slice will be less than the specified size and will need to be later padded with zeros.
fn take_bit_slice<'a>(data: &'a [u8], bit_offset: Size, bit_size: Size) -> &'a [u8] {
    // Cast the start index of the slice to a Size.
    let first_byte: usize = match (bit_offset / 8).try_into() {
        Ok(size) => size,
        Err(_) => {
            // If the offset is too large to be contained in a slice, return an empty slice
            return &[];
        }
    };

    // Byte containing the end (last + 1) bit position.
    // If the slice ends on a byte boundary, this byte is after the slice. However, if the slice
    // does not end on a byte boundary, this byte contains the end of the slice.
    // Guaranteed to fit inside a u32.
    // We cast to u64 to add without overflow, then cast back down after dividing.
    let slice_end_bit_byte = ((bit_offset as u64 + bit_size as u64) / 8) as u32;

    let end_alignment = (bit_offset % 8 + bit_size % 8) % 8;
    // This byte is guaranteed to be the first byte, after the end of the slice, with no bits in
    // the slice.
    let slice_end_byte_u32: u32 = if end_alignment == 0 {
        slice_end_bit_byte
    } else {
        slice_end_bit_byte + 1
    };

    // Cast the slice end byte to usize.
    // Truncate on failure.
    let slice_end_byte: usize = slice_end_byte_u32.try_into().unwrap_or(usize::MAX);
    // Ensure that the slice doesn't extend past the end.
    let safe_slice_end_byte: usize = min(slice_end_byte, data.len());

    &data[first_byte..safe_slice_end_byte]
}

/// Copy the bits from a slice of data.
///
/// This function preserves the alignment within the byte but zeros bits not considered part of the
/// slice.
///
/// If the requested size is too large for the given data, the rest of the data will be filled in
/// with zeros. However, if the requested size is larger than a Size, this function will panic, as
/// it cannot create a boxed slice with the right size.
fn copy_bit_slice(data: &[u8], bit_offset: Size, bit_size: Size) -> Box<[u8]> {
    let slice = take_bit_slice(data, bit_offset, bit_size);

    // Copy the slice into a separate array.
    let mut copy = Vec::from(slice);

    // If copy is zero-length, 
    if copy.len() > 0 {
        // Clean the copy by zeroing extraneous bits
        
        // Remove bits from the start.
        copy[0] &= u8::MAX << bit_offset;
        // Remove bits from the end.
        let end_alignment = (bit_offset % 8 + bit_size % 8) % 8;
        // Number of bits that should be cleared from the end.
        let end_extra_space = if end_alignment == 0 {
            0 // When the end is aligned, no bits need to be cleared.
        } else {
            // When the end is not aligned, clear all bits starting from the end.
            8 - end_alignment
        };
        let last = copy.len() - 1;
        copy[last] &= u8::MAX >> end_extra_space;
    }

    // Pad the copy to occupy the requisite number of bytes.
    let bit_end = bit_offset as u64 + bit_size as u64;
    let desired_size_u32 = if bit_end % 8 == 0 {
        bit_end / 8
    } else {
        // If the end bit is not aligned to byte boundaries, we require an extra byte to hold the value.
        bit_end / 8 + 1
    };

    // Panic if the slice is too large.
    let desired_size = usize::try_from(desired_size_u32).unwrap();
    copy.resize(desired_size, 0);

    // Box the slice
    copy.into()
}


/// Data specifying the byte layout of report data. Can be filled out to create a report, then
/// sent as a report.
/// Lower-order bits are considered first.
/// From 8.4, a report cannot span more than 4 bytes in a report.
#[derive(Clone, Debug)]
pub struct ReportVariable {
    bit_offset: Size,
    bit_size: Size,
    data: u32,
}


impl ReportVariable {
    /// Construct a ReportVariable for a field starting at the given offset and spanning the given
    /// size.
    /// Offset should be a global offset, in bits.
    pub fn new(bit_offset: Size, bit_size: Size) -> Result<Self, TooLargeError> {
        let internal_offset = bit_offset % 8;
        if bit_size + internal_offset <= 32 {
            Ok(Self {
                bit_offset,
                bit_size,
                data: 0,
            })
        } else {
            Err(TooLargeError {})
        }
    }

    /// Clears the written data
    pub fn clear(&mut self) { 
        self.data = 0;
    }

    pub fn data(&self) -> u32 {
        self.data
    }

    /// Copies data from a data slice. The given data slice must not have any bits outside of the
    /// range.
    pub fn set_unsigned(&mut self, data: u32) -> Result<(), DataOutOfBoundsError> {
        // Check that data fits the size
        if data & (u32::MAX << self.bit_size) == 0 {
            let internal_offset = self.bit_offset % 8;
            self.data = data << internal_offset;

            Ok(())
        } else {
            Err(DataOutOfBoundsError {})
        }
    }

    pub fn set_signed(&mut self, data: i32) -> Result<(), DataOutOfBoundsError> {
        let outside_mask = -1i32 << self.bit_size;
        // Check that the data fits the size.
        // For positive values, we expect only 0's outside the space.
        if data > 0 && data & outside_mask != 0 {
            return Err(DataOutOfBoundsError {})
        }
        // For negative values, we expect only 1's outside the space, and the last bit should be 1.
        // This means that the outside is all 1's, even when we shift data out by 1.
        if data < 0 && (data << 1) & outside_mask != outside_mask {
            return Err(DataOutOfBoundsError {})
        }

        // Truncate data, in case it is negative.
        let truncated_data = data & !outside_mask;
        let internal_offset = self.bit_offset % 8;
        self.data = (truncated_data as u32) << internal_offset;

        Ok(())
    }

    /// Construct a ReportVariable by taking the requisite bits from a slice, at an arbitrary starting
    /// position.
    /// Unlike set_data_exact, the given data can have bits out of range.
    pub fn copy_data_from_slice(&mut self, data: &[u8], bit_offset: Size) {
        // Copy bytes from data, with the original offset intact.
        let slice = &copy_bit_slice(data, bit_offset, self.bit_size);
        let mut data_bytes = [0u8; 8];
        data_bytes[..slice.len()].copy_from_slice(slice);
        let unshifted_data = u64::from_le_bytes(data_bytes);
        // Shift bytes 
        let target_internal_shift = self.bit_offset % 8;
        let given_internal_shift = bit_offset % 8;
        let shifted_data = if given_internal_shift < target_internal_shift {
            unshifted_data << (target_internal_shift - given_internal_shift)
        } else {
            unshifted_data >> (given_internal_shift - target_internal_shift)
        };
        self.data = shifted_data.try_into().unwrap();
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ItemType {
    Constant,
    Variable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReportItem {
    pub item_type: ItemType,
    pub bit_size: Size,
}

impl ReportItem {
    /// Construct a ReportItem from a type and size.
    pub fn new(item_type: ItemType, bit_size: Size) -> Self {
        Self {
            item_type,
            bit_size,
        }
    }

    pub fn from_report(report: &Report) -> Self {
        let is_constant = report.main.report_flags.is_constant();
        ReportItem {
            item_type: if is_constant { ItemType::Constant } else { ItemType::Variable },
            bit_size: report.report_size,
        }
    }
}

/// Structure for holding an entire report, broken up into bit-sliced fields.
#[derive(Clone, Debug, Default)]
pub struct ReportFormat {
    report_id: Option<ReportId>,
    // Perhaps also support report type and ID
    reports: Vec<ReportVariable>,
    bit_size: Size,
}

impl ReportFormat {
    /// Construct an empty ReportFormat without an ID.
    pub fn new() -> Self {
        Self {
            report_id: None,
            reports: Vec::new(),
            bit_size: 0,
        }
    }

    /// Construct an empty ReportFormat with an ID.
    pub fn new_with_id(report_id: ReportId) -> Self {
        Self {
            report_id: Some(report_id),
            reports: Vec::new(),
            bit_size: 0,
        }
    }

    /// Construct an empty ReportFormat with an optional ID.
    pub fn new_with_opt_id(report_id: Option<ReportId>) -> Self {
        Self {
            report_id: report_id,
            reports: Vec::new(),
            bit_size: 0,
        }
    }

    /// Returns the ID associated with this report, if there is one.
    pub fn report_id(&self) -> Option<ReportId> {
        self.report_id
    }

    /// Returns a new report format without an ID.
    ///
    /// Use of this method is generally discouraged, as a report format's ID must match with an ID
    /// in the report descriptor, but it can be useful in situations where the ID must be manually
    /// specified or removed.
    pub fn without_report_id(self) -> Self {
        Self {
            report_id: None,
            ..self
        }
    }

    /// Returns a new report format with an ID.
    ///
    /// Use of this method is generally discouraged, as a report format's ID must match with an ID
    /// in the report descriptor, but it can be useful in situations where the ID must be manually
    /// specified or removed.
    pub fn with_report_id(self, report_id: ReportId) -> Self {
        Self {
            report_id: Some(report_id),
            ..self
        }
    }

    /// Add a report variable of the given size
    pub fn push_empty(&mut self, bit_size: Size) -> Result<(), TooLargeError> {
        let bit_offset = self.bit_size;
        self.reports.push(ReportVariable::new(bit_offset, bit_size)?);
        self.bit_size += bit_size;
        Ok(())
    }

    /// Add a constant report variable of the specified size
    pub fn push_constant(&mut self, bit_size: Size) {
        self.bit_size += bit_size;
    }

    pub fn copy_from_iter<I: Iterator<Item = ReportItem>>(mut self, items: I) -> Result<Self, TooLargeError> {
        for item in items {
            match item.item_type {
                ItemType::Constant => {
                    self.push_constant(item.bit_size);
                },
                ItemType::Variable => {
                    self.push_empty(item.bit_size)?;
                },
            }
        }
        Ok(self)
    }

    /// Returns an iterator over the contained reports
    pub fn iter(&self) -> Iter<'_, ReportVariable> {
        self.reports.iter()
    }

    /// Returns an iterator that allows modifying the contained reports.
    pub fn iter_mut(&mut self) -> IterMut<'_, ReportVariable> {
        self.reports.iter_mut()
    }

    /// Returns the number of reports in this ReportFormat.
    pub fn count(&self) -> u32 {
        self.reports.len().try_into().unwrap()
    }

    /// Clears all reports
    pub fn clear(&mut self) {
        for report in self.iter_mut() {
            report.clear()
        }
    }

    /// Convert a filled ReportFormat into bytes.
    /// Unfilled reports items are assigned 0.
    pub fn into_bytes(self) -> Box<[u8]> {
        let id_size: u32 = if self.report_id.is_some() { 1 } else { 0 };
        let size_in_bytes = usize::try_from(
            id_size as u64 + (self.bit_size as u64 + 7) / 8).unwrap();
        let mut storage = [0u8].repeat(size_in_bytes);
        // Prepend the ID
        if let Some(report_id) = self.report_id {
            storage[0] = report_id;
        }
        
        // Copy each report into storage.
        for report_item in self.iter() {
            let report_data = report_item.data();
            // Calculate the starting byte of this item, including any offset due to the report id.
            let start = (id_size + report_item.bit_offset / 8).try_into().unwrap();
            let end = min(start + 4, size_in_bytes);
            let len = end - start;
            // Copy bits from report_data into storage
            // Copy bits until the end of storage is reached
            let data_bytes = report_data.to_le_bytes();
            for (dst, src) in storage[start..end].iter_mut().zip(data_bytes[..len].iter()) {
                *dst |= src;
            }
        }

        storage.into_boxed_slice()
    }
}


impl<I: SliceIndex<[ReportVariable]>> Index<I> for ReportFormat {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        self.reports.index(index)
    }
}

impl<I: SliceIndex<[ReportVariable]>> IndexMut<I> for ReportFormat {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.reports.index_mut(index)
    }
}


