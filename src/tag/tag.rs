use super::super::collection::{Collection, CollectionItem};
use super::super::field_types::{ReportFlags, CollectionType, Delimiter, DesignatorIndex, LogicalValue, PhysicalValue, ReportCount, ReportId, ReportSize, StringIndex, Unit, UnitExponent};
use super::super::report::{Report, ReportMain, ReportType};
use super::super::usage::{ExtendedUsage, Usage, UsageId, UsagePage, UsageRange, UsageSet};

pub enum TagType {
    Main,
    Global,
    Local,
}

/// A tag defines a single statement in a HID report.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tag {
    // Main items
    Input(ReportFlags),
    Output(ReportFlags),
    Feature(ReportFlags),
    Collection(CollectionType),
    EndCollection,

    // Global tags
    UsagePage(UsagePage),
    LogicalMinimum(LogicalValue),
    LogicalMaximum(LogicalValue),
    PhysicalMinimum(PhysicalValue),
    PhysicalMaximum(PhysicalValue),
    UnitExponent(UnitExponent),
    Unit(Unit),
    ReportSize(ReportSize),
    ReportId(ReportId),
    ReportCount(ReportCount),
    Push,
    Pop,

    // Local tags
    ExtendedUsage(ExtendedUsage), // Extended usage
    UsageId(UsageId), // Implicitly equivalent to Usage, but smaller
    ExtendedUsageMinimum(ExtendedUsage),
    UsageMinimumId(UsageId),
    ExtendedUsageMaximum(ExtendedUsage),
    UsageMaximumId(UsageId),

    DesignatorIndex(DesignatorIndex),
    DesignatorMinimum(DesignatorIndex),
    DesignatorMaximum(DesignatorIndex),
    StringIndex(StringIndex),
    StringMinimum(StringIndex),
    StringMaximum(StringIndex),
    Delimiter(Delimiter),
}

impl Tag {
    pub fn tag_type(&self) -> TagType {
        match self {
            Self::Input(..) => TagType::Main,
            Self::Output(..) => TagType::Main,
            Self::Feature(..) => TagType::Main,
            Self::Collection(..) => TagType::Main,
            Self::EndCollection => TagType::Main,

            Self::UsagePage(..) => TagType::Global,
            Self::LogicalMinimum(..) => TagType::Global,
            Self::LogicalMaximum(..) => TagType::Global,
            Self::PhysicalMinimum(..) => TagType::Global,
            Self::PhysicalMaximum(..) => TagType::Global,
            Self::UnitExponent(..) => TagType::Global,
            Self::Unit(..) => TagType::Global,
            Self::ReportSize(..) => TagType::Global,
            Self::ReportId(..) => TagType::Global,
            Self::ReportCount(..) => TagType::Global,
            Self::Push => TagType::Global,
            Self::Pop => TagType::Global,

            Self::ExtendedUsage(..) => TagType::Local,
            Self::UsageId(..) => TagType::Local,
            Self::ExtendedUsageMinimum(..) => TagType::Local,
            Self::UsageMinimumId(..) => TagType::Local,
            Self::ExtendedUsageMaximum(..) => TagType::Local,
            Self::UsageMaximumId(..) => TagType::Local,
            Self::DesignatorIndex(..) => TagType::Local,
            Self::DesignatorMinimum(..) => TagType::Local,
            Self::DesignatorMaximum(..) => TagType::Local,
            Self::StringIndex(..) => TagType::Local,
            Self::StringMinimum(..) => TagType::Local,
            Self::StringMaximum(..) => TagType::Local,
            Self::Delimiter(..) => TagType::Local,
        }
    }
}

/// A TagGroup allows tags to be arranged in a hierarchical structure. However, they are still
/// ordered and can be flattened afterwards into a linear tag document.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TagGroup {
    Tag(Tag),
    Group(Box<[TagGroup]>),
}


impl TagGroup {
    // Main items

    pub const fn input(v: ReportFlags) -> Self {
        Self::Tag(Tag::Input(v))
    }
    pub const fn output(v: ReportFlags) -> Self {
        Self::Tag(Tag::Output(v))
    }
    pub const fn feature(v: ReportFlags) -> Self {
        Self::Tag(Tag::Feature(v))
    }
    pub const fn collection_tag(v: CollectionType) -> Self {
        Self::Tag(Tag::Collection(v))
    }
    pub const fn end_collection() -> Self {
        Self::Tag(Tag::EndCollection)
    }

    // Local tags

    pub const fn usage_page(v: UsagePage) -> Self {
        Self::Tag(Tag::UsagePage(v))
    }
    pub const fn logical_minimum(v: LogicalValue) -> Self {
        Self::Tag(Tag::LogicalMinimum(v))
    }
    pub const fn logical_maximum(v: LogicalValue) -> Self {
        Self::Tag(Tag::LogicalMaximum(v))
    }
    pub const fn physical_minimum(v: PhysicalValue) -> Self {
        Self::Tag(Tag::PhysicalMinimum(v))
    }
    pub const fn physical_maximum(v: PhysicalValue) -> Self {
        Self::Tag(Tag::PhysicalMaximum(v))
    }
    pub const fn unit_exponent(v: UnitExponent) -> Self {
        Self::Tag(Tag::UnitExponent(v))
    }
    pub const fn unit(v: Unit) -> Self {
        Self::Tag(Tag::Unit(v))
    }
    pub const fn report_size(v: ReportSize) -> Self {
        Self::Tag(Tag::ReportSize(v))
    }
    pub const fn report_id(v: ReportId) -> Self {
        Self::Tag(Tag::ReportId(v))
    }
    pub const fn report_count(v: ReportCount) -> Self {
        Self::Tag(Tag::ReportCount(v))
    }
    pub const fn push() -> Self {
        Self::Tag(Tag::Push)
    }
    pub const fn pop() -> Self {
        Self::Tag(Tag::Pop)
    }

    // Local tags

    pub const fn extended_usage(v: ExtendedUsage) -> Self {
        Self::Tag(Tag::ExtendedUsage(v))
    }
    pub const fn usage_id(v: UsageId) -> Self {
        Self::Tag(Tag::UsageId(v))
    }
    pub const fn extended_usage_minimum(v: ExtendedUsage) -> Self {
        Self::Tag(Tag::ExtendedUsageMinimum(v))
    }
    pub const fn usage_minimum_id(v: UsageId) -> Self {
        Self::Tag(Tag::UsageMinimumId(v))
    }
    pub const fn extended_usage_maximum(v: ExtendedUsage) -> Self {
        Self::Tag(Tag::ExtendedUsageMaximum(v))
    }
    pub const fn usage_maximum_id(v: UsageId) -> Self {
        Self::Tag(Tag::UsageMaximumId(v))
    }
    pub const fn designator_index(v: DesignatorIndex) -> Self {
        Self::Tag(Tag::DesignatorIndex(v))
    }
    pub const fn designator_minimum(v: DesignatorIndex) -> Self {
        Self::Tag(Tag::DesignatorMinimum(v))
    }
    pub const fn designator_maximum(v: DesignatorIndex) -> Self {
        Self::Tag(Tag::DesignatorMaximum(v))
    }
    pub const fn string_index(v: StringIndex) -> Self {
        Self::Tag(Tag::StringIndex(v))
    }
    pub const fn string_minimum(v: StringIndex) -> Self {
        Self::Tag(Tag::StringMinimum(v))
    }
    pub const fn string_maximum(v: StringIndex) -> Self {
        Self::Tag(Tag::StringMaximum(v))
    }
    pub const fn delimiter(v: Delimiter) -> Self {
        Self::Tag(Tag::Delimiter(v))
    }
}


impl TagGroup {
    /// Construct a tag group from an iterable of tag groups.
    pub fn group<I: IntoIterator>(it: I) -> Self
    where I::Item: Into<TagGroup> {
        Self::Group(it.into_iter().map(|e| e.into()).collect())
    }

    /// Construct a tag group to represent the page and ID of a Usage.
    pub fn usage(usage: Usage) -> Self {
        let tag_group = match usage {
            Usage::Standard(page, id) => vec![
                Self::usage_page(page),
                Self::usage_id(id),
            ],
            Usage::Extended(id) => vec![Self::extended_usage(id)],
        };
        Self::group(tag_group)
    }

    /// Construct a tag group to represent the page and ID of a UsageMinimum.
    pub fn usage_minimum(usage: Usage) -> Self {
        let tag_group = match usage {
            Usage::Standard(page, id) => vec![
                Self::usage_page(page),
                Self::usage_minimum_id(id),
            ],
            Usage::Extended(id) => vec![Self::extended_usage_minimum(id)],
        };
        Self::group(tag_group)
    }

    /// Construct a tag group to represent the page and ID of a UsageMaximum.
    pub fn usage_maximum(usage: Usage) -> Self {
        let tag_group = match usage {
            Usage::Standard(page, id) => vec![
                Self::usage_page(page),
                Self::usage_maximum_id(id),
            ],
            Usage::Extended(id) => vec![Self::extended_usage_maximum(id)],
        };
        Self::group(tag_group)
    }

    /// Construct a tag group to represent a range of usages.
    pub fn usage_range(usage_range: UsageRange) -> Self {
        let tag_group = if usage_range.len() == 1 {
            vec![Self::usage(usage_range.min)]
        } else {
            vec![
                Self::usage_minimum(usage_range.min),
                Self::usage_maximum(usage_range.max),
            ]
        };
        Self::group(tag_group)
    }

    /// Construct a tag group to represent a UsageSet.
    pub fn usage_set(usage_set: UsageSet) -> Self {
        let tag_group = usage_set.into_iter().map(Self::usage_range);
        Self::group(tag_group)
    }

    /// Construct a tag group to represent a collection.
    pub fn collection(collection: Collection) -> Self {
        // Convert the collection and its attributes to tags
        let mut tag_groups = Vec::new();
        // Specify usage.
        tag_groups.push(Self::usage(collection.usage));
        // Add optional values.
        if let Some(designator_index) = collection.designator_index {
            tag_groups.push(Self::designator_index(designator_index));
        }
        if let Some(string_index) = collection.string_index {
            tag_groups.push(Self::string_index(string_index));
        }
        if let Some(delimiter) = collection.delimiter {
            tag_groups.push(Self::delimiter(delimiter));
        }
        // Start collection.
        tag_groups.push(Self::collection_tag(collection.collection_type));
        // Add collection items.
        tag_groups.push(Self::collection_items(collection.items));
        // End collection.
        tag_groups.push(Self::end_collection());

        Self::group(tag_groups)
    }

    /// Construct a tag group to represent a ReportMain.
    pub fn report_main(main: ReportMain) -> Self {
        match main.report_type {
            ReportType::Feature => Self::feature(main.report_flags),
            ReportType::Input => Self::input(main.report_flags),
            ReportType::Output => Self::output(main.report_flags),
        }
    }

    /// Construct a tag group to represent a report.
    fn report(report: Report) -> Self {
        // Convert the collection and its attributes to tags
        let mut tag_groups = Vec::new();

        tag_groups.append(&mut vec![
            Self::usage_set(report.usage_set),
            Self::logical_minimum(report.logical_minimum),
            Self::logical_maximum(report.logical_maximum),
            Self::report_size(report.report_size),
            Self::report_count(report.report_count),
        ]);

        // TODO: Optionals

        // Optional global fields

        // Physical Minimum
        if let Some(physical_minimum) = report.physical_minimum {
            tag_groups.push(Self::physical_minimum(physical_minimum))
        }

        // Physical Maximum
        if let Some(physical_maximum) = report.physical_maximum {
            tag_groups.push(Self::physical_maximum(physical_maximum))
        }

        // Unit Exponent
        if let Some(unit_exponent) = report.unit_exponent {
            tag_groups.push(Self::unit_exponent(unit_exponent))
        }

        // Unit
        if let Some(unit) = report.unit {
            tag_groups.push(Self::unit(unit))
        }
        
        // Report ID
        if let Some(report_id) = report.report_id {
            tag_groups.push(Self::report_id(report_id))
        }

        // Add the report main
        tag_groups.push(Self::report_main(report.main));

        Self::group(tag_groups)
    }

    pub fn collection_item(item: CollectionItem) -> Self {
        match item {
            CollectionItem::Report(report) => Self::report(report),
            CollectionItem::Collection(collection) => Self::collection(collection),
        }
    }

    pub fn collection_items<I: IntoIterator<Item = CollectionItem>>(item: I) -> Self {
        Self::group(item.into_iter().map(Self::collection_item))
    }
}

