use super::field_types::{Delimiter, DesignatorIndex, LogicalValue, PhysicalValue, ReportCount, ReportFlags, ReportId, ReportSize, StringIndex, Unit, UnitExponent};
use super::usage::UsageSet;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReportType {
    Input,
    Output,
    Feature,
}

impl ReportType {
    /// Returns true if this ReportType is an Input.
    pub const fn is_input(self) -> bool {
        match self {
            ReportType::Input => true,
            _ => false,
        }
    }

    /// Returns true if this ReportType is an Output.
    pub const fn is_output(self) -> bool {
        match self {
            ReportType::Output => true,
            _ => false,
        }
    }

    /// Returns true if this ReportType is a Feature.
    pub const fn is_feature(self) -> bool {
        match self {
            ReportType::Feature => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ReportMain {
    pub report_type: ReportType,
    pub report_flags: ReportFlags,
}

impl ReportMain {
    /// Construct a new ReportMain with the given type and flags.
    pub fn new(report_type: ReportType, report_flags: ReportFlags) -> Self {
        Self { report_type, report_flags }
    }

    /// Construct a new input ReportMain with the given flags.
    pub fn new_input(report_flags: ReportFlags) -> Self {
        Self::new(ReportType::Input, report_flags)
    }

    /// Construct a new output ReportMain with the given flags.
    pub fn new_output(report_flags: ReportFlags) -> Self {
        Self::new(ReportType::Output, report_flags)
    }
    
    /// Construct a new feature ReportMain with the given flags.
    pub fn new_feature(report_flags: ReportFlags) -> Self {
        Self::new(ReportType::Feature, report_flags)
    }
}


// Required items are specified in 6.2.2, page 25.
// All other local and global items are optional modifiers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Report {
    /// Main report item, denoting either an input, output, or feature, with bit flags.
    pub main: ReportMain,
    /// Usage. Also contains the usage page. (?)
    /// Usages are local, but the usage page is global.
    pub usage_set: UsageSet,
    pub logical_minimum: LogicalValue, // Global
    pub logical_maximum: LogicalValue, // Global
    /// Report size in bits.
    pub report_size: ReportSize, // Global
    pub report_count: ReportCount, // Global

    // Additional optional items
    
    // Global
    
    pub physical_minimum: Option<PhysicalValue>,
    pub physical_maximum: Option<PhysicalValue>,
    pub unit_exponent: Option<UnitExponent>,
    pub unit: Option<Unit>,
    pub report_id: Option<ReportId>,
    
    // Local
    
    pub designator_index: Option<DesignatorIndex>,
    pub designator_minimum: Option<DesignatorIndex>,
    pub designator_maximum: Option<DesignatorIndex>,
    pub string_index: Option<StringIndex>, // strings?
    pub string_minimum: Option<StringIndex>,
    pub string_maximum: Option<StringIndex>,
    pub delimiter: Option<Delimiter>,
}




impl Report {
    /// Create a new report with no optional values.
    pub fn new(
        main: ReportMain,
        usage_set: UsageSet,
        logical_minimum: LogicalValue,
        logical_maximum: LogicalValue,
        report_size: ReportSize,
        report_count: ReportCount
    ) -> Self {
        Self {
            main,
            usage_set: usage_set,
            logical_minimum,
            logical_maximum,
            report_size,
            report_count,

            physical_minimum: None,
            physical_maximum: None,
            unit_exponent: None,
            unit: None,
            report_id: None,
            
            designator_index: None,
            designator_minimum: None,
            designator_maximum: None,
            string_index: None,
            string_minimum: None,
            string_maximum: None,
            delimiter: None,
        }
    }

    /// Construct a new input Report.
    pub fn new_input(
        report_flags: ReportFlags,
        usage_set: UsageSet,
        logical_minimum: LogicalValue,
        logical_maximum: LogicalValue,
        report_size: ReportSize,
        report_count: ReportCount
    ) -> Self {
        Self::new(
            ReportMain::new_input(report_flags),
            usage_set,
            logical_minimum,
            logical_maximum,
            report_size,
            report_count)
    }

    /// Construct a new output Report.
    pub fn new_output(
        report_flags: ReportFlags,
        usage_set: UsageSet,
        logical_minimum: LogicalValue,
        logical_maximum: LogicalValue,
        report_size: ReportSize,
        report_count: ReportCount
    ) -> Self {
        Self::new(
            ReportMain::new_output(report_flags),
            usage_set,
            logical_minimum,
            logical_maximum,
            report_size,
            report_count)
    }

    /// Construct a new feature Report.
    pub fn new_feature(
        report_flags: ReportFlags,
        usage_set: UsageSet,
        logical_minimum: LogicalValue,
        logical_maximum: LogicalValue,
        report_size: ReportSize,
        report_count: ReportCount
    ) -> Self {
        Self::new(
            ReportMain::new_feature(report_flags),
            usage_set,
            logical_minimum,
            logical_maximum,
            report_size,
            report_count)
    }

    pub const fn report_type(&self) -> ReportType {
        self.main.report_type
    }

    /// Returns whether this report is an input.
    pub const fn is_input(&self) -> bool {
        self.report_type().is_input()
    }

    /// Returns whether this report is an output.
    pub const fn is_output(&self) -> bool {
        self.report_type().is_output()
    }

    /// Returns whether this report is a feature.
    pub const fn is_feature(&self) -> bool {
        self.report_type().is_feature()
    }

    /// Construct a new report descriptor by removing the report ID.
    pub fn without_report_id(mut self) -> Self {
        self.report_id = None;
        self
    }

    /// Construct a new report descriptor by adding a report ID.
    pub fn with_report_id(mut self, report_id: ReportId) -> Self {
        self.report_id = Some(report_id);
        self
    }
}
