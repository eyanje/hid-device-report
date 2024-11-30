/// Types for each field in a report descriptor.

use super::error::TryFromIntError;

// Main item types

const fn bit_at(value: u32, position: u32) -> bool {
    ((value >> position) & 1) > 0
}

const fn set_bit_at(value: u32, position: u32) -> u32 {
    value | (1 << position)
}

const fn unset_bit_at(value: u32, position: u32) -> u32 {
    value & !(1 << position)
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct ReportFlags(u32);

impl ReportFlags {
    /// Construct the default ReportFlags.
    pub const fn new() -> Self {
        Self(0)
    }
}

macro_rules! impl_report_flags(
    ($(($bit:expr, $get:ident, $get_not:ident, $set:ident, $unset:ident)),*
        $(,)?) => {
        impl ReportFlags {
            $(
            pub const fn $get(&self) -> bool {
                bit_at(self.0, $bit)
            }
            pub const fn $get_not(&self) -> bool {
                !self.$get()
            }
            pub const fn $set(self) -> Self {
                Self(set_bit_at(self.0, $bit))
            }
            pub const fn $unset(self) -> Self {
                Self(unset_bit_at(self.0, $bit))
            }
            )*
        }
    }
);

impl_report_flags!(
    (0, is_constant, is_data, as_constant, as_data),
    (1, is_variable, is_array, as_variable, as_array),
    (2, is_relative, is_absolute, as_relative, as_absolute),
    (3, can_wrap, cannot_wrap, with_wrap, without_wrap),
    (4, is_nonlinear, is_linear, as_nonlinear, as_linear),
    (5, has_no_preferred_state, has_preferred_state, with_preferred_state, without_preferred_state),
    (6, has_null_state, has_no_null_position, with_null_state, without_null_position),
    (7, is_volatile, is_nonvolatile, as_volatile, as_nonvolatile),
    (8, is_buffered_bytes, is_bit_field, as_buffered_bytes, as_bit_field),
);

impl From<u32> for ReportFlags {
    /// Construct ReportFlags from a u32.
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<ReportFlags> for u32 {
    /// Convert ReportFlags to a u32.
    fn from(flags: ReportFlags) -> Self {
        flags.0
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CollectionType {
    Physical,
    Application,
    Logical,
    Report,
    NamedArray,
    UsageSwitch,
    UsageModifier,
}

impl CollectionType {
    /// Return the code of this CollectionType.
    pub const fn code(self) -> u8 {
        match self {
            Self::Physical => 0x00,
            Self::Application => 0x01,
            Self::Logical => 0x02,
            Self::Report => 0x03,
            Self::NamedArray => 0x04,
            Self::UsageSwitch => 0x05,
            Self::UsageModifier => 0x06,
        }
    }
}

impl TryFrom<u8> for CollectionType {
    type Error = TryFromIntError;
    /// Attempt to construct a CollectionType for the given u8.
    fn try_from(value: u8) -> Result<Self, TryFromIntError> {
        match value {
            0x00 => Ok(Self::Physical),
            0x01 => Ok(Self::Application),
            0x02 => Ok(Self::Logical),
            0x03 => Ok(Self::Report),
            0x04 => Ok(Self::NamedArray),
            0x05 => Ok(Self::UsageSwitch),
            0x06 => Ok(Self::UsageModifier),
            _ => Err(TryFromIntError {}),
        }
    }
}

impl From<CollectionType> for u8 {
    /// Convert a CollectionType to its u8 code.
    fn from(collection_type: CollectionType) -> Self {
        collection_type.code()
    }
}


// Global item tags: structs.
// 6.2.2.7 of the USB HID specification

// Usage page not listed.

/// Used in logical minima and logical maxima. In logical units.
pub type LogicalValue = i32;

/// Contains units, which should be specified separately in Unit and UnitExponent tags.
pub type PhysicalValue = i32;

/// Value of the unit exponent in base 10.
/// 4-bit 2's-complement value. Needs interpretation.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct UnitExponent(i8);

impl UnitExponent {
    /// Return the value of this exponent as a signed integer.
    pub fn as_i8(&self) -> i8 {
        self.0
    }
    /// Return this exponent in its 4-bit form.
    pub fn as_nibble(&self) -> u8 {
        (self.0 & 0xF).try_into().unwrap()
    }
}

impl TryFrom<i8> for UnitExponent {
    type Error = TryFromIntError;

    fn try_from(value: i8) -> Result<Self, TryFromIntError> {
        if value >= -4 && value < 4 {
            Ok(Self(value))
        } else {
            Err(TryFromIntError {})
        }
    }
}

/// A unit is divided into segments (nibbles) of 4 bits. Each segment corresponds to a different
/// type of unit: system, length, mass, time, temperature, current, and luminosity intensity. The
/// last segment is reserved.
/// The value in each segment is interpreted in the table on page 37. 
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Unit(u32);
 
// Coding all these units is best put into a different file.
impl Unit {
    pub const fn code(self) -> u32 {
        self.0
    }
}

/// A ReportId indicates a prefix that should be added to subsequent reports.
/// 0 should not be used.
/// The presence of at least one Report ID means that all reports will require a prefix.
pub type ReportId = u8;

/// A ReportSize represents the size of a report, in bits.
pub type ReportSize = u32;

/// A ReportCount represents the number of data fields on an item.
pub type ReportCount = u32;



// Local items

/// A DesignatorIndex represents a body part used for a control. This is an index for a physical
/// descriptor, described in the USB HID specification. A DesignatorIndex is also used to represent
/// Designator Minimum and Designator Maximum tags.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct DesignatorIndex(u32);

impl From<u32> for DesignatorIndex {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl From<DesignatorIndex> for u32 {
    fn from(value: DesignatorIndex) -> u32 {
        value.0
    }
}

/// A StringIndex is an index of a string to be associated with an item or control. A StrinIndex is
/// also used to represent String Minimum and String Maximum tags.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct StringIndex(u32);

impl From<u32> for StringIndex {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl From<StringIndex> for u32 {
    fn from(value: StringIndex) -> u32 {
        value.0
    }
}

/// A Delimiter designates the beginning or end of a set of local items.
/// 1 = open set. 0 = close set.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Delimiter(bool);

impl Delimiter {
    pub const fn open() -> Self {
        Delimiter(true)
    }
    pub const fn close() -> Self {
        Delimiter(false)
    }

    pub const fn is_open(&self) -> bool {
        self.0
    }
    pub const fn is_close(&self) -> bool {
        !self.is_open()
    }
}


