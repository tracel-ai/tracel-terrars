use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCloudIdentityPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
}
struct DataCloudIdentityPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudIdentityPolicyData>,
}
#[derive(Clone)]
pub struct DataCloudIdentityPolicy(Rc<DataCloudIdentityPolicy_>);
impl DataCloudIdentityPolicy {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `customer` after provisioning.\nThe customer that the policy belongs to."]
    pub fn customer(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the policy to retrieve."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_query` after provisioning.\nThe CEL query that defines which entities the policy applies to."]
    pub fn policy_query(&self) -> ListRef<DataCloudIdentityPolicyPolicyQueryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.policy_query", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `setting` after provisioning.\nThe setting configured by this policy."]
    pub fn setting(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the policy."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
impl Referable for DataCloudIdentityPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCloudIdentityPolicy {}
impl ToListMappable for DataCloudIdentityPolicy {
    type O = ListRef<DataCloudIdentityPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCloudIdentityPolicy_ {
    fn extract_datasource_type(&self) -> String {
        "google_cloud_identity_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCloudIdentityPolicy {
    pub tf_id: String,
    #[doc = "The resource name of the policy to retrieve."]
    pub name: PrimField<String>,
}
impl BuildDataCloudIdentityPolicy {
    pub fn build(self, stack: &mut Stack) -> DataCloudIdentityPolicy {
        let out = DataCloudIdentityPolicy(Rc::new(DataCloudIdentityPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudIdentityPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataCloudIdentityPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCloudIdentityPolicyRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `customer` after provisioning.\nThe customer that the policy belongs to."]
    pub fn customer(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the policy to retrieve."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_query` after provisioning.\nThe CEL query that defines which entities the policy applies to."]
    pub fn policy_query(&self) -> ListRef<DataCloudIdentityPolicyPolicyQueryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.policy_query", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `setting` after provisioning.\nThe setting configured by this policy."]
    pub fn setting(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the policy."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudIdentityPolicyPolicyQueryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org_unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<PrimField<f64>>,
}
impl DataCloudIdentityPolicyPolicyQueryEl {
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
impl ToListMappable for DataCloudIdentityPolicyPolicyQueryEl {
    type O = BlockAssignable<DataCloudIdentityPolicyPolicyQueryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudIdentityPolicyPolicyQueryEl {}
impl BuildDataCloudIdentityPolicyPolicyQueryEl {
    pub fn build(self) -> DataCloudIdentityPolicyPolicyQueryEl {
        DataCloudIdentityPolicyPolicyQueryEl {
            group: core::default::Default::default(),
            org_unit: core::default::Default::default(),
            query: core::default::Default::default(),
            sort_order: core::default::Default::default(),
        }
    }
}
pub struct DataCloudIdentityPolicyPolicyQueryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityPolicyPolicyQueryElRef {
    fn new(shared: StackShared, base: String) -> DataCloudIdentityPolicyPolicyQueryElRef {
        DataCloudIdentityPolicyPolicyQueryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudIdentityPolicyPolicyQueryElRef {
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
