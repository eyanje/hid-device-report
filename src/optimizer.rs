use std::iter::FromIterator;

use super::field_types::{LogicalValue, PhysicalValue, ReportCount, ReportId, ReportSize, Unit, UnitExponent};
use super::tag::{Tag, TagType};
use super::usage::{UsagePage};

fn replace_option_bool<T: PartialEq>(opt: &mut Option<T>, value: T) -> bool {
    let item_is_new = match opt.as_ref() {
        Some(t) => t != &value,
        None => true,
    };
    *opt = Some(value);
    item_is_new
}


#[derive(Clone, Debug, Default)]
pub struct TagOptimizer(Vec<Tag>);

impl FromIterator<Tag> for TagOptimizer {
    /// Create a TagOptimizer from an iterable of tags.
    fn from_iter<T: IntoIterator<Item = Tag>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for TagOptimizer {
    type Item = Tag;
    type IntoIter = <Vec<Tag> as IntoIterator>::IntoIter;

    /// Create an iterator through all tags of this optimizer.
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}


/// Global state table
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct GlobalTable {
    usage_page: Option<UsagePage>,
    logical_minimum: Option<LogicalValue>,
    logical_maximum: Option<LogicalValue>,
    physical_minimum: Option<PhysicalValue>,
    physical_maximum: Option<PhysicalValue>,
    unit_exponent: Option<UnitExponent>,
    unit: Option<Unit>,
    report_size: Option<ReportSize>,
    report_id: Option<ReportId>,
    report_count: Option<ReportCount>,
}

impl GlobalTable {
    /// Construct a new empty GlobalTable.
    pub fn new() -> Self {
        Self::default()
    }

    /// Incorporate a tag into the table. Returns true if the tag was changed.
    /// Panics if the tag is not recognized.
    pub fn set_tag(&mut self, tag: Tag) -> bool {
        match tag {
            Tag::UsagePage(usage_page) =>
                replace_option_bool(&mut self.usage_page, usage_page),
            Tag::LogicalMinimum(logical_minimum) =>
                replace_option_bool(&mut self.logical_minimum, logical_minimum),
            Tag::LogicalMaximum(logical_maximum) =>
                replace_option_bool(&mut self.logical_maximum, logical_maximum),
            Tag::PhysicalMinimum(physical_minimum) =>
                replace_option_bool(&mut self.physical_minimum, physical_minimum),
            Tag::PhysicalMaximum(physical_maximum) =>
                replace_option_bool(&mut self.physical_maximum, physical_maximum),
            Tag::UnitExponent(unit_exponent) =>
                replace_option_bool(&mut self.unit_exponent, unit_exponent),
            Tag::Unit(unit) =>
                replace_option_bool(&mut self.unit, unit),
            Tag::ReportSize(report_size) =>
                replace_option_bool(&mut self.report_size, report_size),
            Tag::ReportId(report_id) =>
                replace_option_bool(&mut self.report_id, report_id),
            Tag::ReportCount(report_count) =>
                replace_option_bool(&mut self.report_count, report_count),
            _ => panic!("Unrecognized tag"),
        }
    }
}

impl TagOptimizer {
    /// Remove duplicate global attributes.
    /// Cannot yet handle push and pop tags.
    pub fn remove_duplicates(mut self) -> Self {
        let mut is_duplicate = Vec::new();
        is_duplicate.resize(self.0.len(), false);

        // Maintain a state table of all global items.
        let mut global_table = GlobalTable::new();
        // Note: we don't have a local table at this time.
        // Not sure how to handle multiple usages

        for (tag, is_duplicate) in self.0.iter().zip(is_duplicate.iter_mut()) {
            match tag.tag_type() {
                TagType::Global => {
                    *is_duplicate = !global_table.set_tag(*tag);
                },
                _ => (),
            }
        }

        // Delete all tags marked as duplicate
        self.0 = self.0.into_iter().zip(is_duplicate.iter())
            .filter(|(_, &is_duplicate)| !is_duplicate)
            .map(|(tag, _)| tag)
            .collect();

        self
    }
}






