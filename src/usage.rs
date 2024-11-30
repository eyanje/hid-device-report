use std::cmp::Ordering;
use std::mem::discriminant;
use std::vec::IntoIter;

pub type UsagePage = u16;
pub type UsageId = u16;

/// Struct to hold a usage page and usage ID as a single integer.
/// When constructed as an ExtendedUsage, a usage ID will not be separated into separate Usage Page
/// and Usage ID tags.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ExtendedUsage(u32);

impl ExtendedUsage {
    pub const fn new(id: u32) -> Self {
        Self(id)
    }
    pub const fn as_u32(self) -> u32 {
        self.0
    }
    pub const fn page(self) -> u16 {
        ((self.0 >> 0x10) & 0xFFFF) as u16
    }
    pub const fn id(self) -> u16 {
        (self.0 & 0xFFFF) as u16
    }
}
impl From<ExtendedUsage> for u32 {
    fn from(eu: ExtendedUsage) -> u32 {
        eu.as_u32()
    }
}

/// Also used to represent Usage Minimum and Usage Maximum tags.
/// Contains the Usage in its higher-order form.
#[derive(Copy, Clone, Debug)]
pub enum Usage {
    Standard(UsagePage, UsageId),
    Extended(ExtendedUsage),
}

impl Usage {
    /// Construct a Usage from a UsagePage and a UsageId.
    pub const fn new(page: UsagePage, id: UsageId) -> Self {
        Self::Standard(page, id)
    }

    /// Construct a Usage from a u32 representing an extended usage.
    pub const fn extended(id: u32) -> Self {
        Self::Extended(ExtendedUsage::new(id))
    }

    /// Construct a Usage from an ExtendedUsage.
    pub const fn from_extended(usage: ExtendedUsage) -> Self {
        Self::Extended(usage)
    }

    /// Return the page of this Usage.
    pub const fn page(&self) -> UsagePage {
        match self {
            Self::Standard(page, _id) => *page,
            Self::Extended(usage) => usage.page(),
        }
    }

    /// Return the ID of this Usage.
    pub const fn id(&self) -> UsageId {
        match self {
            Self::Standard(_page, id) => *id,
            Self::Extended(usage) => usage.id(),
        }
    }

    /// Convert this Usage into a u32, where the higher-order bits contain the page and the
    /// lower-order bits contain the ID.
    pub const fn as_u32(self) -> u32 {
        match self {
            Self::Standard(page, id) => ((page as u32) << 0x10) | (id as u32),
            Self::Extended(usage) => usage.as_u32(),
        }
    }
}


// Implementation of equality and comparison for Usage.

impl PartialEq for Usage {
    fn eq(&self, other: &Self) -> bool {
        self.as_u32() == other.as_u32()
    }
}

impl Eq for Usage {}

impl PartialOrd for Usage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_u32().partial_cmp(&other.as_u32())
    }
}

impl Ord for Usage {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_u32().cmp(&other.as_u32())
    }
}


/// A UsageRange represents a contiguous range of usage values.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct UsageRange {
    pub min: Usage, 
    pub max: Usage,
}

impl UsageRange {
    /// Construct a UsageRange as an interval over two Usages.
    pub fn new(min: Usage, max: Usage) -> Self {
        if min > max {
            panic!("Max should be greater than min");
        }
        if discriminant(&min) != discriminant(&max) {
            panic!("Range should not mix extended usages");
        }
        if let (Usage::Standard(page1, _), Usage::Standard(page2, _)) = (min, max) {
            if page1 != page2 {
                panic!("Pages should match");
            }
        }
        UsageRange { min, max }
    }

    /// Construct a UsageRange from a single Usage.
    pub const fn single(usage: Usage) -> Self {
        Self {
            min: usage,
            max: usage,
        }
    }

    /// Returns the number of usages included in this range.
    pub const fn len(&self) -> u32 {
        self.max.as_u32() + 1 - self.min.as_u32()
    }
}

impl From<Usage> for UsageRange {
    /// Convert a single Usage to a UsageRange
    fn from(usage: Usage) -> Self {
        Self::new(usage, usage)
    }
}

/// A UsageSet is an ordered collection of Usages in a report.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UsageSet(Vec<UsageRange>);

impl UsageSet {
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    pub fn into_boxed_slice(self) -> Box<[UsageRange]> {
        self.0.into_boxed_slice()
    }

    /// Add a Usage to this UsageSet.
    pub fn push_usage(&mut self, usage: Usage) {
        self.0.push(UsageRange::single(usage))
    }

    /// Add a Usage to this UsageSet, returning this UsageSet.
    pub fn with_usage(mut self, usage: Usage) -> Self {
        self.push_usage(usage);
        self
    }

    /// Add a Usage range to this UsageSet.
    pub fn push_usage_range(&mut self, usage_range: UsageRange) {
        self.0.push(usage_range)
    }

    /// Add a Usage range to this UsageSet, returning this UsageSet.
    pub fn with_usage_range(mut self, usage_range: UsageRange) -> Self {
        self.push_usage_range(usage_range);
        self
    }

    /// Add a Usage range, specified by a minimum and maximum Usage, to this UsageSet.
    pub fn push_usage_bounds(&mut self, min: Usage, max: Usage) {
        self.0.push(UsageRange::new(min, max))
    }

    /// Add a Usage range, specified by a minimum and maximum Usage, to this UsageSet, and return
    /// this UsageSet.
    pub fn with_usage_bounds(mut self, min: Usage, max: Usage) -> Self {
        self.push_usage_bounds(min, max);
        self
    }
}

impl IntoIterator for UsageSet {
    type Item = UsageRange;
    type IntoIter = IntoIter<UsageRange>;

    /// Consume this UsageSet and produce an interator over its UsageRanges.
    fn into_iter(self) -> IntoIter<UsageRange> {
        self.0.into_iter()
    }
}

