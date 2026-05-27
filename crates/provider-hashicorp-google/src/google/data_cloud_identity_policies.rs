use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCloudIdentityPoliciesData {
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
}
struct DataCloudIdentityPolicies_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudIdentityPoliciesData>,
}
#[derive(Clone)]
pub struct DataCloudIdentityPolicies(Rc<DataCloudIdentityPolicies_>);
impl DataCloudIdentityPolicies {
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
    #[doc = "Set the field `filter`.\nFilter expression for listing policies, as documented in the Cloud Identity Policy API policies.list method"]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nFilter expression for listing policies, as documented in the Cloud Identity Policy API policies.list method"]
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
    #[doc = "Get a reference to the value of field `policies` after provisioning.\nList of Cloud Identity policies that match the filter (or all policies if no filter is provided)."]
    pub fn policies(&self) -> ListRef<DataCloudIdentityPoliciesPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.policies", self.extract_ref()),
        )
    }
}
impl Referable for DataCloudIdentityPolicies {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCloudIdentityPolicies {}
impl ToListMappable for DataCloudIdentityPolicies {
    type O = ListRef<DataCloudIdentityPoliciesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCloudIdentityPolicies_ {
    fn extract_datasource_type(&self) -> String {
        "google_cloud_identity_policies".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCloudIdentityPolicies {
    pub tf_id: String,
}
impl BuildDataCloudIdentityPolicies {
    pub fn build(self, stack: &mut Stack) -> DataCloudIdentityPolicies {
        let out = DataCloudIdentityPolicies(Rc::new(DataCloudIdentityPolicies_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudIdentityPoliciesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataCloudIdentityPoliciesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityPoliciesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCloudIdentityPoliciesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nFilter expression for listing policies, as documented in the Cloud Identity Policy API policies.list method"]
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
    #[doc = "Get a reference to the value of field `policies` after provisioning.\nList of Cloud Identity policies that match the filter (or all policies if no filter is provided)."]
    pub fn policies(&self) -> ListRef<DataCloudIdentityPoliciesPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.policies", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudIdentityPoliciesPoliciesElPolicyQueryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org_unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<PrimField<f64>>,
}
impl DataCloudIdentityPoliciesPoliciesElPolicyQueryEl {
    #[doc = "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group = Some(v.into());
        self
    }
    #[doc = "Set the field `org_unit`.\n"]
    pub fn set_org_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.org_unit = Some(v.into());
        self
    }
    #[doc = "Set the field `query`.\n"]
    pub fn set_query(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query = Some(v.into());
        self
    }
    #[doc = "Set the field `sort_order`.\n"]
    pub fn set_sort_order(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudIdentityPoliciesPoliciesElPolicyQueryEl {
    type O = BlockAssignable<DataCloudIdentityPoliciesPoliciesElPolicyQueryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudIdentityPoliciesPoliciesElPolicyQueryEl {}
impl BuildDataCloudIdentityPoliciesPoliciesElPolicyQueryEl {
    pub fn build(self) -> DataCloudIdentityPoliciesPoliciesElPolicyQueryEl {
        DataCloudIdentityPoliciesPoliciesElPolicyQueryEl {
            group: core::default::Default::default(),
            org_unit: core::default::Default::default(),
            query: core::default::Default::default(),
            sort_order: core::default::Default::default(),
        }
    }
}
pub struct DataCloudIdentityPoliciesPoliciesElPolicyQueryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityPoliciesPoliciesElPolicyQueryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudIdentityPoliciesPoliciesElPolicyQueryElRef {
        DataCloudIdentityPoliciesPoliciesElPolicyQueryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudIdentityPoliciesPoliciesElPolicyQueryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.base))
    }
    #[doc = "Get a reference to the value of field `org_unit` after provisioning.\n"]
    pub fn org_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org_unit", self.base))
    }
    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }
    #[doc = "Get a reference to the value of field `sort_order` after provisioning.\n"]
    pub fn sort_order(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort_order", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudIdentityPoliciesPoliciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_query: Option<ListField<DataCloudIdentityPoliciesPoliciesElPolicyQueryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    setting: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataCloudIdentityPoliciesPoliciesEl {
    #[doc = "Set the field `customer`.\n"]
    pub fn set_customer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.customer = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_query`.\n"]
    pub fn set_policy_query(
        mut self,
        v: impl Into<ListField<DataCloudIdentityPoliciesPoliciesElPolicyQueryEl>>,
    ) -> Self {
        self.policy_query = Some(v.into());
        self
    }
    #[doc = "Set the field `setting`.\n"]
    pub fn set_setting(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.setting = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudIdentityPoliciesPoliciesEl {
    type O = BlockAssignable<DataCloudIdentityPoliciesPoliciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudIdentityPoliciesPoliciesEl {}
impl BuildDataCloudIdentityPoliciesPoliciesEl {
    pub fn build(self) -> DataCloudIdentityPoliciesPoliciesEl {
        DataCloudIdentityPoliciesPoliciesEl {
            customer: core::default::Default::default(),
            name: core::default::Default::default(),
            policy_query: core::default::Default::default(),
            setting: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataCloudIdentityPoliciesPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityPoliciesPoliciesElRef {
    fn new(shared: StackShared, base: String) -> DataCloudIdentityPoliciesPoliciesElRef {
        DataCloudIdentityPoliciesPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudIdentityPoliciesPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `customer` after provisioning.\n"]
    pub fn customer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `policy_query` after provisioning.\n"]
    pub fn policy_query(&self) -> ListRef<DataCloudIdentityPoliciesPoliciesElPolicyQueryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.policy_query", self.base))
    }
    #[doc = "Get a reference to the value of field `setting` after provisioning.\n"]
    pub fn setting(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.setting", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
