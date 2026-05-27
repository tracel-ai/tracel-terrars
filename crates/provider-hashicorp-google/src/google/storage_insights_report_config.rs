use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct StorageInsightsReportConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csv_options: Option<Vec<StorageInsightsReportConfigCsvOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_options: Option<Vec<StorageInsightsReportConfigFrequencyOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_metadata_report_options:
        Option<Vec<StorageInsightsReportConfigObjectMetadataReportOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parquet_options: Option<Vec<StorageInsightsReportConfigParquetOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<StorageInsightsReportConfigTimeoutsEl>,
    dynamic: StorageInsightsReportConfigDynamic,
}
struct StorageInsightsReportConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StorageInsightsReportConfigData>,
}
#[derive(Clone)]
pub struct StorageInsightsReportConfig(Rc<StorageInsightsReportConfig_>);
impl StorageInsightsReportConfig {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }
    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }
    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }
    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => true,
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    }
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }
    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(r.extract_ref());
        self
    }
    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(attr.to_string());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe editable display name of the inventory report configuration. Has a limit of 256 characters. Can be empty."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `force_destroy`.\nIf set, all the inventory report details associated with this report configuration are deleted."]
    pub fn set_force_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `csv_options`.\n"]
    pub fn set_csv_options(
        self,
        v: impl Into<BlockAssignable<StorageInsightsReportConfigCsvOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().csv_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.csv_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `frequency_options`.\n"]
    pub fn set_frequency_options(
        self,
        v: impl Into<BlockAssignable<StorageInsightsReportConfigFrequencyOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().frequency_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.frequency_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `object_metadata_report_options`.\n"]
    pub fn set_object_metadata_report_options(
        self,
        v: impl Into<BlockAssignable<StorageInsightsReportConfigObjectMetadataReportOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().object_metadata_report_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .object_metadata_report_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `parquet_options`.\n"]
    pub fn set_parquet_options(
        self,
        v: impl Into<BlockAssignable<StorageInsightsReportConfigParquetOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parquet_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parquet_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<StorageInsightsReportConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe editable display name of the inventory report configuration. Has a limit of 256 characters. Can be empty."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\nIf set, all the inventory report details associated with this report configuration are deleted."]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the ReportConfig. The source and destination buckets specified in the ReportConfig\nmust be in the same location."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe UUID of the inventory report configuration."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `csv_options` after provisioning.\n"]
    pub fn csv_options(&self) -> ListRef<StorageInsightsReportConfigCsvOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.csv_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `frequency_options` after provisioning.\n"]
    pub fn frequency_options(&self) -> ListRef<StorageInsightsReportConfigFrequencyOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.frequency_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `object_metadata_report_options` after provisioning.\n"]
    pub fn object_metadata_report_options(
        &self,
    ) -> ListRef<StorageInsightsReportConfigObjectMetadataReportOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.object_metadata_report_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parquet_options` after provisioning.\n"]
    pub fn parquet_options(&self) -> ListRef<StorageInsightsReportConfigParquetOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parquet_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageInsightsReportConfigTimeoutsElRef {
        StorageInsightsReportConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for StorageInsightsReportConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for StorageInsightsReportConfig {}
impl ToListMappable for StorageInsightsReportConfig {
    type O = ListRef<StorageInsightsReportConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for StorageInsightsReportConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_storage_insights_report_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildStorageInsightsReportConfig {
    pub tf_id: String,
    #[doc = "The location of the ReportConfig. The source and destination buckets specified in the ReportConfig\nmust be in the same location."]
    pub location: PrimField<String>,
}
impl BuildStorageInsightsReportConfig {
    pub fn build(self, stack: &mut Stack) -> StorageInsightsReportConfig {
        let out = StorageInsightsReportConfig(Rc::new(StorageInsightsReportConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(StorageInsightsReportConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                csv_options: core::default::Default::default(),
                frequency_options: core::default::Default::default(),
                object_metadata_report_options: core::default::Default::default(),
                parquet_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct StorageInsightsReportConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsReportConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl StorageInsightsReportConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe editable display name of the inventory report configuration. Has a limit of 256 characters. Can be empty."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\nIf set, all the inventory report details associated with this report configuration are deleted."]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the ReportConfig. The source and destination buckets specified in the ReportConfig\nmust be in the same location."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe UUID of the inventory report configuration."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `csv_options` after provisioning.\n"]
    pub fn csv_options(&self) -> ListRef<StorageInsightsReportConfigCsvOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.csv_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `frequency_options` after provisioning.\n"]
    pub fn frequency_options(&self) -> ListRef<StorageInsightsReportConfigFrequencyOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.frequency_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `object_metadata_report_options` after provisioning.\n"]
    pub fn object_metadata_report_options(
        &self,
    ) -> ListRef<StorageInsightsReportConfigObjectMetadataReportOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.object_metadata_report_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parquet_options` after provisioning.\n"]
    pub fn parquet_options(&self) -> ListRef<StorageInsightsReportConfigParquetOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parquet_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageInsightsReportConfigTimeoutsElRef {
        StorageInsightsReportConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct StorageInsightsReportConfigCsvOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_required: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_separator: Option<PrimField<String>>,
}
impl StorageInsightsReportConfigCsvOptionsEl {
    #[doc = "Set the field `delimiter`.\nThe delimiter used to separate the fields in the inventory report CSV file."]
    pub fn set_delimiter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delimiter = Some(v.into());
        self
    }
    #[doc = "Set the field `header_required`.\nThe boolean that indicates whether or not headers are included in the inventory report CSV file."]
    pub fn set_header_required(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.header_required = Some(v.into());
        self
    }
    #[doc = "Set the field `record_separator`.\nThe character used to separate the records in the inventory report CSV file."]
    pub fn set_record_separator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.record_separator = Some(v.into());
        self
    }
}
impl ToListMappable for StorageInsightsReportConfigCsvOptionsEl {
    type O = BlockAssignable<StorageInsightsReportConfigCsvOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsReportConfigCsvOptionsEl {}
impl BuildStorageInsightsReportConfigCsvOptionsEl {
    pub fn build(self) -> StorageInsightsReportConfigCsvOptionsEl {
        StorageInsightsReportConfigCsvOptionsEl {
            delimiter: core::default::Default::default(),
            header_required: core::default::Default::default(),
            record_separator: core::default::Default::default(),
        }
    }
}
pub struct StorageInsightsReportConfigCsvOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsReportConfigCsvOptionsElRef {
    fn new(shared: StackShared, base: String) -> StorageInsightsReportConfigCsvOptionsElRef {
        StorageInsightsReportConfigCsvOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsReportConfigCsvOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delimiter` after provisioning.\nThe delimiter used to separate the fields in the inventory report CSV file."]
    pub fn delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }
    #[doc = "Get a reference to the value of field `header_required` after provisioning.\nThe boolean that indicates whether or not headers are included in the inventory report CSV file."]
    pub fn header_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.header_required", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `record_separator` after provisioning.\nThe character used to separate the records in the inventory report CSV file."]
    pub fn record_separator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_separator", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageInsightsReportConfigFrequencyOptionsElEndDateEl {
    day: PrimField<f64>,
    month: PrimField<f64>,
    year: PrimField<f64>,
}
impl StorageInsightsReportConfigFrequencyOptionsElEndDateEl {}
impl ToListMappable for StorageInsightsReportConfigFrequencyOptionsElEndDateEl {
    type O = BlockAssignable<StorageInsightsReportConfigFrequencyOptionsElEndDateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsReportConfigFrequencyOptionsElEndDateEl {
    #[doc = "The day of the month to stop generating inventory reports."]
    pub day: PrimField<f64>,
    #[doc = "The month to stop generating inventory reports."]
    pub month: PrimField<f64>,
    #[doc = "The year to stop generating inventory reports"]
    pub year: PrimField<f64>,
}
impl BuildStorageInsightsReportConfigFrequencyOptionsElEndDateEl {
    pub fn build(self) -> StorageInsightsReportConfigFrequencyOptionsElEndDateEl {
        StorageInsightsReportConfigFrequencyOptionsElEndDateEl {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}
pub struct StorageInsightsReportConfigFrequencyOptionsElEndDateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsReportConfigFrequencyOptionsElEndDateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageInsightsReportConfigFrequencyOptionsElEndDateElRef {
        StorageInsightsReportConfigFrequencyOptionsElEndDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsReportConfigFrequencyOptionsElEndDateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nThe day of the month to stop generating inventory reports."]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `month` after provisioning.\nThe month to stop generating inventory reports."]
    pub fn month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }
    #[doc = "Get a reference to the value of field `year` after provisioning.\nThe year to stop generating inventory reports"]
    pub fn year(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageInsightsReportConfigFrequencyOptionsElStartDateEl {
    day: PrimField<f64>,
    month: PrimField<f64>,
    year: PrimField<f64>,
}
impl StorageInsightsReportConfigFrequencyOptionsElStartDateEl {}
impl ToListMappable for StorageInsightsReportConfigFrequencyOptionsElStartDateEl {
    type O = BlockAssignable<StorageInsightsReportConfigFrequencyOptionsElStartDateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsReportConfigFrequencyOptionsElStartDateEl {
    #[doc = "The day of the month to start generating inventory reports."]
    pub day: PrimField<f64>,
    #[doc = "The month to start generating inventory reports."]
    pub month: PrimField<f64>,
    #[doc = "The year to start generating inventory reports"]
    pub year: PrimField<f64>,
}
impl BuildStorageInsightsReportConfigFrequencyOptionsElStartDateEl {
    pub fn build(self) -> StorageInsightsReportConfigFrequencyOptionsElStartDateEl {
        StorageInsightsReportConfigFrequencyOptionsElStartDateEl {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}
pub struct StorageInsightsReportConfigFrequencyOptionsElStartDateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsReportConfigFrequencyOptionsElStartDateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageInsightsReportConfigFrequencyOptionsElStartDateElRef {
        StorageInsightsReportConfigFrequencyOptionsElStartDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsReportConfigFrequencyOptionsElStartDateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nThe day of the month to start generating inventory reports."]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `month` after provisioning.\nThe month to start generating inventory reports."]
    pub fn month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }
    #[doc = "Get a reference to the value of field `year` after provisioning.\nThe year to start generating inventory reports"]
    pub fn year(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}
#[derive(Serialize, Default)]
struct StorageInsightsReportConfigFrequencyOptionsElDynamic {
    end_date: Option<DynamicBlock<StorageInsightsReportConfigFrequencyOptionsElEndDateEl>>,
    start_date: Option<DynamicBlock<StorageInsightsReportConfigFrequencyOptionsElStartDateEl>>,
}
#[derive(Serialize)]
pub struct StorageInsightsReportConfigFrequencyOptionsEl {
    frequency: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<Vec<StorageInsightsReportConfigFrequencyOptionsElEndDateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<Vec<StorageInsightsReportConfigFrequencyOptionsElStartDateEl>>,
    dynamic: StorageInsightsReportConfigFrequencyOptionsElDynamic,
}
impl StorageInsightsReportConfigFrequencyOptionsEl {
    #[doc = "Set the field `end_date`.\n"]
    pub fn set_end_date(
        mut self,
        v: impl Into<BlockAssignable<StorageInsightsReportConfigFrequencyOptionsElEndDateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.end_date = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.end_date = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `start_date`.\n"]
    pub fn set_start_date(
        mut self,
        v: impl Into<BlockAssignable<StorageInsightsReportConfigFrequencyOptionsElStartDateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.start_date = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.start_date = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageInsightsReportConfigFrequencyOptionsEl {
    type O = BlockAssignable<StorageInsightsReportConfigFrequencyOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsReportConfigFrequencyOptionsEl {
    #[doc = "The frequency in which inventory reports are generated. Values are DAILY or WEEKLY. Possible values: [\"DAILY\", \"WEEKLY\"]"]
    pub frequency: PrimField<String>,
}
impl BuildStorageInsightsReportConfigFrequencyOptionsEl {
    pub fn build(self) -> StorageInsightsReportConfigFrequencyOptionsEl {
        StorageInsightsReportConfigFrequencyOptionsEl {
            frequency: self.frequency,
            end_date: core::default::Default::default(),
            start_date: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageInsightsReportConfigFrequencyOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsReportConfigFrequencyOptionsElRef {
    fn new(shared: StackShared, base: String) -> StorageInsightsReportConfigFrequencyOptionsElRef {
        StorageInsightsReportConfigFrequencyOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsReportConfigFrequencyOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `frequency` after provisioning.\nThe frequency in which inventory reports are generated. Values are DAILY or WEEKLY. Possible values: [\"DAILY\", \"WEEKLY\"]"]
    pub fn frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.base))
    }
    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> ListRef<StorageInsightsReportConfigFrequencyOptionsElEndDateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.end_date", self.base))
    }
    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(
        &self,
    ) -> ListRef<StorageInsightsReportConfigFrequencyOptionsElStartDateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.start_date", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_path: Option<PrimField<String>>,
}
impl StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl {
    #[doc = "Set the field `destination_path`.\nThe path within the destination bucket to store generated inventory reports."]
    pub fn set_destination_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_path = Some(v.into());
        self
    }
}
impl ToListMappable
    for StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl
{
    type O = BlockAssignable<
        StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl
{
    #[doc = "The destination bucket that stores the generated inventory reports."]
    pub bucket: PrimField<String>,
}
impl BuildStorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl {
    pub fn build(
        self,
    ) -> StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl {
        StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl {
            bucket: self.bucket,
            destination_path: core::default::Default::default(),
        }
    }
}
pub struct StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsElRef
    {
        StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe destination bucket that stores the generated inventory reports."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `destination_path` after provisioning.\nThe path within the destination bucket to store generated inventory reports."]
    pub fn destination_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_path", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
}
impl StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl {
    #[doc = "Set the field `bucket`.\nThe filter to use when specifying which bucket to generate inventory reports for."]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }
}
impl ToListMappable for StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl {
    type O =
        BlockAssignable<StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl {}
impl BuildStorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl {
    pub fn build(self) -> StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl {
        StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl {
            bucket: core::default::Default::default(),
        }
    }
}
pub struct StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersElRef {
        StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe filter to use when specifying which bucket to generate inventory reports for."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
}
#[derive(Serialize, Default)]
struct StorageInsightsReportConfigObjectMetadataReportOptionsElDynamic {
    storage_destination_options: Option<
        DynamicBlock<
            StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl,
        >,
    >,
    storage_filters: Option<
        DynamicBlock<StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl>,
    >,
}
#[derive(Serialize)]
pub struct StorageInsightsReportConfigObjectMetadataReportOptionsEl {
    metadata_fields: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_destination_options: Option<
        Vec<StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_filters:
        Option<Vec<StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl>>,
    dynamic: StorageInsightsReportConfigObjectMetadataReportOptionsElDynamic,
}
impl StorageInsightsReportConfigObjectMetadataReportOptionsEl {
    #[doc = "Set the field `storage_destination_options`.\n"]
    pub fn set_storage_destination_options(
        mut self,
        v: impl Into<
            BlockAssignable<
                StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.storage_destination_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_destination_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `storage_filters`.\n"]
    pub fn set_storage_filters(
        mut self,
        v: impl Into<
            BlockAssignable<
                StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.storage_filters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_filters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageInsightsReportConfigObjectMetadataReportOptionsEl {
    type O = BlockAssignable<StorageInsightsReportConfigObjectMetadataReportOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsReportConfigObjectMetadataReportOptionsEl {
    #[doc = "The metadata fields included in an inventory report."]
    pub metadata_fields: ListField<PrimField<String>>,
}
impl BuildStorageInsightsReportConfigObjectMetadataReportOptionsEl {
    pub fn build(self) -> StorageInsightsReportConfigObjectMetadataReportOptionsEl {
        StorageInsightsReportConfigObjectMetadataReportOptionsEl {
            metadata_fields: self.metadata_fields,
            storage_destination_options: core::default::Default::default(),
            storage_filters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageInsightsReportConfigObjectMetadataReportOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsReportConfigObjectMetadataReportOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageInsightsReportConfigObjectMetadataReportOptionsElRef {
        StorageInsightsReportConfigObjectMetadataReportOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsReportConfigObjectMetadataReportOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metadata_fields` after provisioning.\nThe metadata fields included in an inventory report."]
    pub fn metadata_fields(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata_fields", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `storage_destination_options` after provisioning.\n"]
    pub fn storage_destination_options(
        &self,
    ) -> ListRef<
        StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_destination_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `storage_filters` after provisioning.\n"]
    pub fn storage_filters(
        &self,
    ) -> ListRef<StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_filters", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageInsightsReportConfigParquetOptionsEl {}
impl StorageInsightsReportConfigParquetOptionsEl {}
impl ToListMappable for StorageInsightsReportConfigParquetOptionsEl {
    type O = BlockAssignable<StorageInsightsReportConfigParquetOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsReportConfigParquetOptionsEl {}
impl BuildStorageInsightsReportConfigParquetOptionsEl {
    pub fn build(self) -> StorageInsightsReportConfigParquetOptionsEl {
        StorageInsightsReportConfigParquetOptionsEl {}
    }
}
pub struct StorageInsightsReportConfigParquetOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsReportConfigParquetOptionsElRef {
    fn new(shared: StackShared, base: String) -> StorageInsightsReportConfigParquetOptionsElRef {
        StorageInsightsReportConfigParquetOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsReportConfigParquetOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct StorageInsightsReportConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl StorageInsightsReportConfigTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for StorageInsightsReportConfigTimeoutsEl {
    type O = BlockAssignable<StorageInsightsReportConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageInsightsReportConfigTimeoutsEl {}
impl BuildStorageInsightsReportConfigTimeoutsEl {
    pub fn build(self) -> StorageInsightsReportConfigTimeoutsEl {
        StorageInsightsReportConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct StorageInsightsReportConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageInsightsReportConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> StorageInsightsReportConfigTimeoutsElRef {
        StorageInsightsReportConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageInsightsReportConfigTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct StorageInsightsReportConfigDynamic {
    csv_options: Option<DynamicBlock<StorageInsightsReportConfigCsvOptionsEl>>,
    frequency_options: Option<DynamicBlock<StorageInsightsReportConfigFrequencyOptionsEl>>,
    object_metadata_report_options:
        Option<DynamicBlock<StorageInsightsReportConfigObjectMetadataReportOptionsEl>>,
    parquet_options: Option<DynamicBlock<StorageInsightsReportConfigParquetOptionsEl>>,
}
