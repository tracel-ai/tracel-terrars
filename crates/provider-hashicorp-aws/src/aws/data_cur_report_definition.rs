use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCurReportDefinitionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    report_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataCurReportDefinition_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCurReportDefinitionData>,
}

#[derive(Clone)]
pub struct DataCurReportDefinition(Rc<DataCurReportDefinition_>);

impl DataCurReportDefinition {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `additional_artifacts` after provisioning.\n"]
    pub fn additional_artifacts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.additional_artifacts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `additional_schema_elements` after provisioning.\n"]
    pub fn additional_schema_elements(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.additional_schema_elements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compression` after provisioning.\n"]
    pub fn compression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `refresh_closed_reports` after provisioning.\n"]
    pub fn refresh_closed_reports(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.refresh_closed_reports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_name` after provisioning.\n"]
    pub fn report_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.report_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_versioning` after provisioning.\n"]
    pub fn report_versioning(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.report_versioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_prefix` after provisioning.\n"]
    pub fn s3_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_region` after provisioning.\n"]
    pub fn s3_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_unit` after provisioning.\n"]
    pub fn time_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_unit", self.extract_ref()))
    }
}

impl Referable for DataCurReportDefinition {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataCurReportDefinition { }

impl ToListMappable for DataCurReportDefinition {
    type O = ListRef<DataCurReportDefinitionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCurReportDefinition_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cur_report_definition".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCurReportDefinition {
    pub tf_id: String,
    #[doc = ""]
    pub report_name: PrimField<String>,
}

impl BuildDataCurReportDefinition {
    pub fn build(self, stack: &mut Stack) -> DataCurReportDefinition {
        let out = DataCurReportDefinition(Rc::new(DataCurReportDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCurReportDefinitionData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                report_name: self.report_name,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCurReportDefinitionRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCurReportDefinitionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataCurReportDefinitionRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `additional_artifacts` after provisioning.\n"]
    pub fn additional_artifacts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.additional_artifacts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `additional_schema_elements` after provisioning.\n"]
    pub fn additional_schema_elements(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.additional_schema_elements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compression` after provisioning.\n"]
    pub fn compression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `refresh_closed_reports` after provisioning.\n"]
    pub fn refresh_closed_reports(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.refresh_closed_reports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_name` after provisioning.\n"]
    pub fn report_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.report_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_versioning` after provisioning.\n"]
    pub fn report_versioning(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.report_versioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_prefix` after provisioning.\n"]
    pub fn s3_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_region` after provisioning.\n"]
    pub fn s3_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_unit` after provisioning.\n"]
    pub fn time_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_unit", self.extract_ref()))
    }
}
