use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataServicequotasTemplatesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataServicequotasTemplates_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataServicequotasTemplatesData>,
}

#[derive(Clone)]
pub struct DataServicequotasTemplates(Rc<DataServicequotasTemplates_>);

impl DataServicequotasTemplates {
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

    #[doc = "Set the field `aws_region`.\n"]
    pub fn set_aws_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().aws_region = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `aws_region` after provisioning.\n"]
    pub fn aws_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `templates` after provisioning.\n"]
    pub fn templates(&self) -> ListRef<DataServicequotasTemplatesTemplatesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.templates", self.extract_ref()))
    }
}

impl Referable for DataServicequotasTemplates {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataServicequotasTemplates { }

impl ToListMappable for DataServicequotasTemplates {
    type O = ListRef<DataServicequotasTemplatesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataServicequotasTemplates_ {
    fn extract_datasource_type(&self) -> String {
        "aws_servicequotas_templates".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataServicequotasTemplates {
    pub tf_id: String,
}

impl BuildDataServicequotasTemplates {
    pub fn build(self, stack: &mut Stack) -> DataServicequotasTemplates {
        let out = DataServicequotasTemplates(Rc::new(DataServicequotasTemplates_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataServicequotasTemplatesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                aws_region: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataServicequotasTemplatesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataServicequotasTemplatesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataServicequotasTemplatesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `aws_region` after provisioning.\n"]
    pub fn aws_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `templates` after provisioning.\n"]
    pub fn templates(&self) -> ListRef<DataServicequotasTemplatesTemplatesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.templates", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataServicequotasTemplatesTemplatesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    global_quota: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataServicequotasTemplatesTemplatesEl {
    #[doc = "Set the field `global_quota`.\n"]
    pub fn set_global_quota(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.global_quota = Some(v.into());
        self
    }

    #[doc = "Set the field `quota_code`.\n"]
    pub fn set_quota_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.quota_code = Some(v.into());
        self
    }

    #[doc = "Set the field `quota_name`.\n"]
    pub fn set_quota_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.quota_name = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc = "Set the field `service_code`.\n"]
    pub fn set_service_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_code = Some(v.into());
        self
    }

    #[doc = "Set the field `service_name`.\n"]
    pub fn set_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_name = Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataServicequotasTemplatesTemplatesEl {
    type O = BlockAssignable<DataServicequotasTemplatesTemplatesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServicequotasTemplatesTemplatesEl {}

impl BuildDataServicequotasTemplatesTemplatesEl {
    pub fn build(self) -> DataServicequotasTemplatesTemplatesEl {
        DataServicequotasTemplatesTemplatesEl {
            global_quota: core::default::Default::default(),
            quota_code: core::default::Default::default(),
            quota_name: core::default::Default::default(),
            region: core::default::Default::default(),
            service_code: core::default::Default::default(),
            service_name: core::default::Default::default(),
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataServicequotasTemplatesTemplatesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataServicequotasTemplatesTemplatesElRef {
    fn new(shared: StackShared, base: String) -> DataServicequotasTemplatesTemplatesElRef {
        DataServicequotasTemplatesTemplatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServicequotasTemplatesTemplatesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `global_quota` after provisioning.\n"]
    pub fn global_quota(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_quota", self.base))
    }

    #[doc = "Get a reference to the value of field `quota_code` after provisioning.\n"]
    pub fn quota_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quota_code", self.base))
    }

    #[doc = "Get a reference to the value of field `quota_name` after provisioning.\n"]
    pub fn quota_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quota_name", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `service_code` after provisioning.\n"]
    pub fn service_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_code", self.base))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
