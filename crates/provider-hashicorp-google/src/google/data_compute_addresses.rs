use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeAddressesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataComputeAddresses_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeAddressesData>,
}
#[derive(Clone)]
pub struct DataComputeAddresses(Rc<DataComputeAddresses_>);
impl DataComputeAddresses {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `filter`.\nFilter sets the optional parameter \"filter\": A filter expression that\nfilters resources listed in the response. The expression must specify\nthe field name, an operator, and the value that you want to use for\nfiltering. The value must be a string, a number, or a boolean. The\noperator must be either \"=\", \"!=\", \">\", \"<\", \"<=\", \">=\" or \":\". For\nexample, if you are filtering Compute Engine instances, you can\nexclude instances named \"example-instance\" by specifying \"name !=\nexample-instance\". The \":\" operator can be used with string fields to\nmatch substrings. For non-string fields it is equivalent to the \"=\"\noperator. The \":*\" comparison can be used to test whether a key has\nbeen defined. For example, to find all objects with \"owner\" label\nuse: \"\"\" labels.owner:* \"\"\" You can also filter nested fields. For\nexample, you could specify \"scheduling.automaticRestart = false\" to\ninclude instances only if they are not scheduled for automatic\nrestarts. You can use filtering on nested fields to filter based on\nresource labels. To filter on multiple expressions, provide each\nseparate expression within parentheses. For example: \"\"\"\n(scheduling.automaticRestart = true) (cpuPlatform = \"Intel Skylake\")\n\"\"\" By default, each expression is an \"AND\" expression. However, you\ncan include \"AND\" and \"OR\" expressions explicitly. For example: \"\"\"\n(cpuPlatform = \"Intel Skylake\") OR (cpuPlatform = \"Intel Broadwell\")\nAND (scheduling.automaticRestart = true) \"\"\""]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe google project in which addresses are listed. Defaults to provider's configuration if missing."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion that should be considered to search addresses. All regions are considered if missing."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `addresses` after provisioning.\n"]
    pub fn addresses(&self) -> ListRef<DataComputeAddressesAddressesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.addresses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nFilter sets the optional parameter \"filter\": A filter expression that\nfilters resources listed in the response. The expression must specify\nthe field name, an operator, and the value that you want to use for\nfiltering. The value must be a string, a number, or a boolean. The\noperator must be either \"=\", \"!=\", \">\", \"<\", \"<=\", \">=\" or \":\". For\nexample, if you are filtering Compute Engine instances, you can\nexclude instances named \"example-instance\" by specifying \"name !=\nexample-instance\". The \":\" operator can be used with string fields to\nmatch substrings. For non-string fields it is equivalent to the \"=\"\noperator. The \":*\" comparison can be used to test whether a key has\nbeen defined. For example, to find all objects with \"owner\" label\nuse: \"\"\" labels.owner:* \"\"\" You can also filter nested fields. For\nexample, you could specify \"scheduling.automaticRestart = false\" to\ninclude instances only if they are not scheduled for automatic\nrestarts. You can use filtering on nested fields to filter based on\nresource labels. To filter on multiple expressions, provide each\nseparate expression within parentheses. For example: \"\"\"\n(scheduling.automaticRestart = true) (cpuPlatform = \"Intel Skylake\")\n\"\"\" By default, each expression is an \"AND\" expression. However, you\ncan include \"AND\" and \"OR\" expressions explicitly. For example: \"\"\"\n(cpuPlatform = \"Intel Skylake\") OR (cpuPlatform = \"Intel Broadwell\")\nAND (scheduling.automaticRestart = true) \"\"\""]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe google project in which addresses are listed. Defaults to provider's configuration if missing."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion that should be considered to search addresses. All regions are considered if missing."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeAddresses {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeAddresses {}
impl ToListMappable for DataComputeAddresses {
    type O = ListRef<DataComputeAddressesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeAddresses_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_addresses".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeAddresses {
    pub tf_id: String,
}
impl BuildDataComputeAddresses {
    pub fn build(self, stack: &mut Stack) -> DataComputeAddresses {
        let out = DataComputeAddresses(Rc::new(DataComputeAddresses_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeAddressesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeAddressesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeAddressesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeAddressesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `addresses` after provisioning.\n"]
    pub fn addresses(&self) -> ListRef<DataComputeAddressesAddressesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.addresses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nFilter sets the optional parameter \"filter\": A filter expression that\nfilters resources listed in the response. The expression must specify\nthe field name, an operator, and the value that you want to use for\nfiltering. The value must be a string, a number, or a boolean. The\noperator must be either \"=\", \"!=\", \">\", \"<\", \"<=\", \">=\" or \":\". For\nexample, if you are filtering Compute Engine instances, you can\nexclude instances named \"example-instance\" by specifying \"name !=\nexample-instance\". The \":\" operator can be used with string fields to\nmatch substrings. For non-string fields it is equivalent to the \"=\"\noperator. The \":*\" comparison can be used to test whether a key has\nbeen defined. For example, to find all objects with \"owner\" label\nuse: \"\"\" labels.owner:* \"\"\" You can also filter nested fields. For\nexample, you could specify \"scheduling.automaticRestart = false\" to\ninclude instances only if they are not scheduled for automatic\nrestarts. You can use filtering on nested fields to filter based on\nresource labels. To filter on multiple expressions, provide each\nseparate expression within parentheses. For example: \"\"\"\n(scheduling.automaticRestart = true) (cpuPlatform = \"Intel Skylake\")\n\"\"\" By default, each expression is an \"AND\" expression. However, you\ncan include \"AND\" and \"OR\" expressions explicitly. For example: \"\"\"\n(cpuPlatform = \"Intel Skylake\") OR (cpuPlatform = \"Intel Broadwell\")\nAND (scheduling.automaticRestart = true) \"\"\""]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe google project in which addresses are listed. Defaults to provider's configuration if missing."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion that should be considered to search addresses. All regions are considered if missing."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeAddressesAddressesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}
impl DataComputeAddressesAddressesEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }
    #[doc = "Set the field `address_type`.\n"]
    pub fn set_address_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_type = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_length`.\n"]
    pub fn set_prefix_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.prefix_length = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeAddressesAddressesEl {
    type O = BlockAssignable<DataComputeAddressesAddressesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeAddressesAddressesEl {}
impl BuildDataComputeAddressesAddressesEl {
    pub fn build(self) -> DataComputeAddressesAddressesEl {
        DataComputeAddressesAddressesEl {
            address: core::default::Default::default(),
            address_type: core::default::Default::default(),
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            prefix_length: core::default::Default::default(),
            region: core::default::Default::default(),
            self_link: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct DataComputeAddressesAddressesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeAddressesAddressesElRef {
    fn new(shared: StackShared, base: String) -> DataComputeAddressesAddressesElRef {
        DataComputeAddressesAddressesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeAddressesAddressesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
    #[doc = "Get a reference to the value of field `address_type` after provisioning.\n"]
    pub fn address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_type", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix_length` after provisioning.\n"]
    pub fn prefix_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prefix_length", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
