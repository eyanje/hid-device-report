use super::error::TryFromIntError;
use super::tag::Tag;

/// A BSize represents the two-bit size code of a report descriptor item.
pub enum BSize {
    B0,
    B1,
    B2,
    B4,
}

impl BSize {
    /// Returns the code for this size.
    pub const fn code(&self) -> u8 {
        match self {
            BSize::B0 => 0,
            BSize::B1 => 1,
            BSize::B2 => 2,
            BSize::B4 => 3,
        }
    }

    /// Returns the size in bytes.
    pub const fn size(&self) -> u8 {
        match self {
            BSize::B0 => 0,
            BSize::B1 => 1,
            BSize::B2 => 2,
            BSize::B4 => 4,
        }
    }

    pub const fn try_from_code(code: u8) -> Result<Self, TryFromIntError> {
        match code {
            0 => Ok(Self::B0),
            1 => Ok(Self::B1),
            2 => Ok(Self::B2),
            3 => Ok(Self::B4),
            _ => Err(TryFromIntError {}),
        }
    }
}

/// A BType represents the two-bit type code of a report descriptor item.
pub enum BType {
    Main,
    Global,
    Local,
    Reserved
}

impl BType {
    pub const fn code(&self) -> u8 {
        match self {
            BType::Main => 0,
            BType::Global => 1,
            BType::Local => 2,
            BType::Reserved => 3,
        }
    }

    pub const fn from_code(code: u8) -> Option<Self> {
        match code {
            0 => Some(Self::Main),
            1 => Some(Self::Global),
            2 => Some(Self::Local),
            3 => Some(Self::Reserved),
            _ => None,
        }
    }
}

/// 4-bit unsigned integer representing a short item type.
pub struct BTag(u8);

impl TryFrom<u8> for BTag {
    type Error = TryFromIntError;
    /// Try to convert an integer type to a BTag.
    fn try_from(value: u8) -> Result<BTag, TryFromIntError> {
        if value & 0xF == value {
            Ok(BTag(value))
        } else {
            Err(TryFromIntError {})
        }
    }
}

impl BTag {
    /// Convert a BType into a u8.
    pub const fn as_u8(&self) -> u8 {
        self.0
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SizeTypeTag(u8);

impl SizeTypeTag {
    /// Construct a SizeTypeTag from a u8.
    pub const fn from_u8(value: u8) -> Self {
        Self(value)
    }

    /// Construct a SizeTypeTag from a size, type, and tag.
    pub const fn from_size_type_tag(size: BSize, b_type: BType, tag: BTag) -> Self {
        Self(size.code()
             | (b_type.code() << 2)
             | (tag.as_u8() << 4))
    }

    /// Returns the size of this item.
    pub fn size(&self) -> BSize {
        let code = self.0 & 0x03;
        BSize::try_from_code(code).unwrap()
    }

    /// Returns the type of item.
    pub fn b_type(&self) -> BType {
        let type_code = (self.0 >> 2) & 0x03;
        BType::from_code(type_code).unwrap()
    }

    /// Returns the numeric tag
    pub const fn b_tag(&self) -> u8 {
        (self.0 >> 4) & 0x0f
    }

    pub const fn as_u8(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShortItem(SizeTypeTag, u32);

impl ShortItem {
    pub const fn new(size_type_tag: SizeTypeTag, data: u32) -> Self {
        Self(size_type_tag, data)
    }

    pub fn into_bytes(self) -> Box<[u8]> {
        let mut data = [0u8; 5];
        data[0] = self.0.as_u8();
        data[1..].copy_from_slice(&self.1.to_le_bytes());
        match self.0.size() {
            BSize::B0 => Box::from(&data[..1]),
            BSize::B1 => Box::from(&data[..2]),
            BSize::B2 => Box::from(&data[..3]),
            BSize::B4 => Box::from(data),
        }
    }
}


pub enum ShortItemData {
    Empty,
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
}

impl ShortItemData {
    /// Get the size of this item
    pub const fn size(&self) -> BSize {
        match self {
            ShortItemData::Empty => BSize::B0,
            ShortItemData::U8(_) | ShortItemData::I8(_) => BSize::B1,
            ShortItemData::U16(_) | ShortItemData::I16(_) => BSize::B2,
            ShortItemData::U32(_) | ShortItemData::I32(_) => BSize::B4,
        }
    }

    /// Convert this data to u32
    pub const fn as_u32(self) -> u32 {
        match self {
            ShortItemData::Empty => 0,
            ShortItemData::U8(v) => v as u32,
            ShortItemData::I8(v) => v as u32,
            ShortItemData::U16(v) => v as u32,
            ShortItemData::I16(v) => v as u32,
            ShortItemData::U32(v) => v,
            ShortItemData::I32(v) => v as u32,
        }
    }

    fn make_shrunk_unsigned(value: u32) -> Self {
        if let Ok(v) = u8::try_from(value) {
            return ShortItemData::U8(v);
        }
        if let Ok(v) = u16::try_from(value) {
            return ShortItemData::U16(v);
        }
        ShortItemData::U32(value)
    }

    fn make_shrunk_signed(value: i32) -> Self {
        if let Ok(v) = i8::try_from(value) {
            return ShortItemData::I8(v);
        }
        if let Ok(v) = i16::try_from(value) {
            return ShortItemData::I16(v);
        }
        ShortItemData::I32(value)
    }

    /// Convert this ShortItemData to its most compact form.
    pub fn shrink(self) -> Self {
        match self {
            // Don't shrink 0-byte and 1-byte data
            ShortItemData::Empty => self,
            // Unsigned data
            ShortItemData::U8(v) => Self::make_shrunk_unsigned(v.into()),
            ShortItemData::U16(v) => Self::make_shrunk_unsigned(v.into()),
            ShortItemData::U32(v) => Self::make_shrunk_unsigned(v),
            // Signed data
            ShortItemData::I8(v) => Self::make_shrunk_signed(v.into()),
            ShortItemData::I16(v) => Self::make_shrunk_signed(v.into()),
            ShortItemData::I32(v) => Self::make_shrunk_signed(v),
        }
    }
}

impl From<u8> for ShortItemData {
    fn from(data: u8) -> Self {
        Self::U8(data)
    }
}

impl From<u16> for ShortItemData {
    fn from(data: u16) -> Self {
        Self::U16(data)
    }
}

impl From<u32> for ShortItemData {
    fn from(data: u32) -> Self {
        Self::U32(data)
    }
}


/// Prefix containing a type and tag.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct ItemPrefix(u8);

impl ItemPrefix {
    /// Construct an ItemPrefix from a u8.
    pub const fn from_u8(value: u8) -> Self {
        Self(value & 0xfc)
    }

    /// Create a SizeTypeTag by combining this prefix with a size.
    pub const fn with_size(self, size: BSize) -> SizeTypeTag {
        SizeTypeTag::from_u8(self.0 | size.code())
    }

    pub const fn with_data(self, data: ShortItemData) -> ShortItem {
        ShortItem::new(self.with_size(data.size()), data.as_u32())
    }

    pub const fn without_data(self) -> ShortItem {
        self.with_data(ShortItemData::Empty)
    }

    pub const fn with_u8(self, data: u8) -> ShortItem {
        self.with_data(ShortItemData::U8(data))
    }

    pub const fn with_u16(self, data: u16) -> ShortItem {
        self.with_data(ShortItemData::U16(data))
    }

    pub const fn with_u32(self, data: u32) -> ShortItem {
        self.with_data(ShortItemData::U32(data))
    }

    /// Construct a ShortItem with this prefix and a signed integer
    pub const fn with_i32(self, data: i32) -> ShortItem {
        self.with_data(ShortItemData::U32(data as u32))
    }

    pub fn with_shrunk_u32(self, data: u32) -> ShortItem {
        self.with_data(ShortItemData::U32(data).shrink())
    }

    pub fn with_shrunk_i32(self, data: i32) -> ShortItem {
        self.with_data(ShortItemData::I32(data).shrink())
    }
}


// 6.2.2.4 of the USB HID specification
pub mod main_item {
    use super::ItemPrefix;
    pub const INPUT: ItemPrefix = ItemPrefix::from_u8(0x80);
    pub const OUTPUT: ItemPrefix = ItemPrefix::from_u8(0x90);
    pub const FEATURE: ItemPrefix = ItemPrefix::from_u8(0xB0);
    pub const COLLECTION: ItemPrefix = ItemPrefix::from_u8(0xA0);
    pub const END_COLLECTION: ItemPrefix = ItemPrefix::from_u8(0xC0);
}

// 6.2.2.7 of the USB HID specification
pub mod global_item {
    use super::ItemPrefix;
    pub const USAGE_PAGE: ItemPrefix = ItemPrefix::from_u8(0x04);
    pub const LOGICAL_MINIMUM: ItemPrefix = ItemPrefix::from_u8(0x14);
    pub const LOGICAL_MAXIMUM: ItemPrefix = ItemPrefix::from_u8(0x24);
    pub const PHYSICAL_MINIMUM: ItemPrefix = ItemPrefix::from_u8(0x34);
    pub const PHYSICAL_MAXIMUM: ItemPrefix = ItemPrefix::from_u8(0x44);
    pub const UNIT_EXPONENT: ItemPrefix = ItemPrefix::from_u8(0x54);
    pub const UNIT: ItemPrefix = ItemPrefix::from_u8(0x64);
    pub const REPORT_SIZE: ItemPrefix = ItemPrefix::from_u8(0x74);
    pub const REPORT_ID: ItemPrefix = ItemPrefix::from_u8(0x84);
    pub const REPORT_COUNT: ItemPrefix = ItemPrefix::from_u8(0x94);
    pub const PUSH: ItemPrefix = ItemPrefix::from_u8(0xA4);
    pub const POP: ItemPrefix = ItemPrefix::from_u8(0xB4);
}

// 6.2.2.8 of the USB HID specification
pub mod local_item {
    use super::ItemPrefix;
    pub const USAGE: ItemPrefix = ItemPrefix::from_u8(0x08);
    pub const USAGE_MINIMUM: ItemPrefix = ItemPrefix::from_u8(0x18);
    pub const USAGE_MAXIMUM: ItemPrefix = ItemPrefix::from_u8(0x28);
    pub const DESIGNATOR_INDEX: ItemPrefix = ItemPrefix::from_u8(0x38);
    pub const DESIGNATOR_MINIMUM: ItemPrefix = ItemPrefix::from_u8(0x48);
    pub const DESIGNATOR_MAXIMUM: ItemPrefix = ItemPrefix::from_u8(0x58);
    pub const STRING_INDEX: ItemPrefix = ItemPrefix::from_u8(0x78);
    pub const STRING_MINIMUM: ItemPrefix = ItemPrefix::from_u8(0x88);
    pub const STRING_MAXIMUM: ItemPrefix = ItemPrefix::from_u8(0x98);
    pub const DELIMITER: ItemPrefix = ItemPrefix::from_u8(0xA8);
}


impl From<Tag> for ShortItem {
    fn from(tag: Tag) -> Self {
        match tag {
            // Main items
            Tag::Input(input) => main_item::INPUT.with_shrunk_u32(input.into()),
            Tag::Output(output) =>
                main_item::OUTPUT.with_shrunk_u32(output.into()),
            Tag::Feature(feature) =>
                main_item::FEATURE.with_shrunk_u32(feature.into()),
            Tag::Collection(collection_type) =>
                main_item::COLLECTION.with_u8(collection_type.into()),
            Tag::EndCollection =>
                main_item::END_COLLECTION.without_data(),

            // Global tags
            Tag::UsagePage(usage_page) =>
                global_item::USAGE_PAGE.with_shrunk_u32(usage_page.into()),
            Tag::LogicalMinimum(logical_minimum) =>
                global_item::LOGICAL_MINIMUM.with_shrunk_i32(logical_minimum),
            Tag::LogicalMaximum(logical_maximum) =>
                global_item::LOGICAL_MAXIMUM.with_shrunk_i32(logical_maximum),
            Tag::PhysicalMinimum(physical_minimum) =>
                global_item::PHYSICAL_MINIMUM.with_shrunk_i32(physical_minimum),
            Tag::PhysicalMaximum(physical_maximum) =>
                global_item::PHYSICAL_MAXIMUM.with_shrunk_i32(physical_maximum),
            Tag::UnitExponent(unit_exponent) =>
                global_item::UNIT_EXPONENT.with_u8(unit_exponent.as_nibble()),
            Tag::Unit(unit) =>
                global_item::UNIT.with_shrunk_u32(unit.code()),
            Tag::ReportSize(report_size) =>
                global_item::REPORT_SIZE.with_shrunk_u32(report_size),
            Tag::ReportId(report_id) =>
                global_item::REPORT_ID.with_u8(report_id),
            Tag::ReportCount(report_count) =>
                global_item::REPORT_COUNT.with_shrunk_u32(report_count),
            Tag::Push => global_item::PUSH.without_data(), Tag::Pop =>
                global_item::POP.without_data(),
        
            // Local tags
            Tag::ExtendedUsage(extended_usage) =>
                local_item::USAGE.with_u32(extended_usage.into()),
            Tag::UsageId(usage_id) =>
                local_item::USAGE.with_shrunk_u32(usage_id.into()),
            Tag::ExtendedUsageMinimum(extended_usage) =>
                local_item::USAGE_MINIMUM.with_u32(extended_usage.into()),
            Tag::UsageMinimumId(usage_id) =>
                local_item::USAGE_MINIMUM.with_shrunk_u32(usage_id.into()),
            Tag::ExtendedUsageMaximum(extended_usage) =>
                local_item::USAGE_MAXIMUM.with_u32(extended_usage.into()),
            Tag::UsageMaximumId(usage_id) =>
                local_item::USAGE_MAXIMUM.with_shrunk_u32(usage_id.into()),
            Tag::DesignatorIndex(designator_index) =>
                local_item::DESIGNATOR_INDEX.with_shrunk_u32(designator_index.into()),
            Tag::DesignatorMinimum(designator_minimum) =>
                local_item::DESIGNATOR_MINIMUM.with_shrunk_u32(designator_minimum.into()),
            Tag::DesignatorMaximum(designator_maximum) =>
                local_item::DESIGNATOR_MAXIMUM.with_shrunk_u32(designator_maximum.into()),
            Tag::StringIndex(string_index) =>
                local_item::STRING_INDEX.with_shrunk_u32(string_index.into()),
            Tag::StringMinimum(string_minimum) =>
                local_item::STRING_MINIMUM.with_shrunk_u32(string_minimum.into()),
            Tag::StringMaximum(string_maximum) =>
                local_item::STRING_MAXIMUM.with_shrunk_u32(string_maximum.into()),
            Tag::Delimiter(delimiter) =>
                local_item::DELIMITER.with_shrunk_u32(if delimiter.is_open() { 1 } else { 0 }),
        }
    }
}
 
/// A sequence of ShortItems
#[derive(Clone, Debug, Default)]
pub struct ShortItems(Vec<ShortItem>);

impl FromIterator<Tag> for ShortItems {
    /// Construct a ShortItems sequence from an iterator of Tags.
    fn from_iter<I: IntoIterator<Item = Tag>>(iter: I) -> Self {
        Self(iter.into_iter().map(ShortItem::from).collect())
    }
}

impl FromIterator<ShortItem> for ShortItems {
    /// Construct a ShortItems sequence from an iterator of ShortItems.
    fn from_iter<I: IntoIterator<Item = ShortItem>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}


impl ShortItems {
    pub fn into_bytes(self) -> Box<[u8]> {
        self.0.into_iter()
            .flat_map(ShortItem::into_bytes)
            .collect()
    }
}



