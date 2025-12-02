use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CurReportDefinitionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_artifacts: Option<SetField<PrimField<String>>>,
    additional_schema_elements: SetField<PrimField<String>>,
    compression: PrimField<String>,
    format: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_closed_reports: Option<PrimField<bool>>,
    report_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    report_versioning: Option<PrimField<String>>,
    s3_bucket: PrimField<String>,
    s3_prefix: PrimField<String>,
    s3_region: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    time_unit: PrimField<String>,
}
struct CurReportDefinition_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CurReportDefinitionData>,
}
#[derive(Clone)]
pub struct CurReportDefinition(Rc<CurReportDefinition_>);
impl CurReportDefinition {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
    #[doc = "Set the field `additional_artifacts`.\n"]
    pub fn set_additional_artifacts(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().additional_artifacts = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `refresh_closed_reports`.\n"]
    pub fn set_refresh_closed_reports(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().refresh_closed_reports = Some(v.into());
        self
    }
    #[doc = "Set the field `report_versioning`.\n"]
    pub fn set_report_versioning(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().report_versioning = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `additional_artifacts` after provisioning.\n"]
    pub fn additional_artifacts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.additional_artifacts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `additional_schema_elements` after provisioning.\n"]
    pub fn additional_schema_elements(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.additional_schema_elements", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `compression` after provisioning.\n"]
    pub fn compression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compression", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.format", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `refresh_closed_reports` after provisioning.\n"]
    pub fn refresh_closed_reports(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.refresh_closed_reports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `report_name` after provisioning.\n"]
    pub fn report_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.report_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `report_versioning` after provisioning.\n"]
    pub fn report_versioning(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.report_versioning", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_prefix` after provisioning.\n"]
    pub fn s3_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_region` after provisioning.\n"]
    pub fn s3_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_unit` after provisioning.\n"]
    pub fn time_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_unit", self.extract_ref()),
        )
    }
}
impl Referable for CurReportDefinition {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CurReportDefinition {}
impl ToListMappable for CurReportDefinition {
    type O = ListRef<CurReportDefinitionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CurReportDefinition_ {
    fn extract_resource_type(&self) -> String {
        "aws_cur_report_definition".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCurReportDefinition {
    pub tf_id: String,
    #[doc = ""]
    pub additional_schema_elements: SetField<PrimField<String>>,
    #[doc = ""]
    pub compression: PrimField<String>,
    #[doc = ""]
    pub format: PrimField<String>,
    #[doc = ""]
    pub report_name: PrimField<String>,
    #[doc = ""]
    pub s3_bucket: PrimField<String>,
    #[doc = ""]
    pub s3_prefix: PrimField<String>,
    #[doc = ""]
    pub s3_region: PrimField<String>,
    #[doc = ""]
    pub time_unit: PrimField<String>,
}
impl BuildCurReportDefinition {
    pub fn build(self, stack: &mut Stack) -> CurReportDefinition {
        let out = CurReportDefinition(Rc::new(CurReportDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CurReportDefinitionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                additional_artifacts: core::default::Default::default(),
                additional_schema_elements: self.additional_schema_elements,
                compression: self.compression,
                format: self.format,
                id: core::default::Default::default(),
                refresh_closed_reports: core::default::Default::default(),
                report_name: self.report_name,
                report_versioning: core::default::Default::default(),
                s3_bucket: self.s3_bucket,
                s3_prefix: self.s3_prefix,
                s3_region: self.s3_region,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                time_unit: self.time_unit,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CurReportDefinitionRef {
    shared: StackShared,
    base: String,
}
impl Ref for CurReportDefinitionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CurReportDefinitionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_artifacts` after provisioning.\n"]
    pub fn additional_artifacts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.additional_artifacts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `additional_schema_elements` after provisioning.\n"]
    pub fn additional_schema_elements(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.additional_schema_elements", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `compression` after provisioning.\n"]
    pub fn compression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compression", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.format", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `refresh_closed_reports` after provisioning.\n"]
    pub fn refresh_closed_reports(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.refresh_closed_reports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `report_name` after provisioning.\n"]
    pub fn report_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.report_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `report_versioning` after provisioning.\n"]
    pub fn report_versioning(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.report_versioning", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_prefix` after provisioning.\n"]
    pub fn s3_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_region` after provisioning.\n"]
    pub fn s3_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_unit` after provisioning.\n"]
    pub fn time_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_unit", self.extract_ref()),
        )
    }
}
