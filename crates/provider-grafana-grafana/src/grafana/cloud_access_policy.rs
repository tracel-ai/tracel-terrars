use super::provider::ProviderGrafana;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudAccessPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    region: PrimField<String>,
    scopes: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<Vec<CloudAccessPolicyConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    realm: Option<Vec<CloudAccessPolicyRealmEl>>,
    dynamic: CloudAccessPolicyDynamic,
}
struct CloudAccessPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudAccessPolicyData>,
}
#[derive(Clone)]
pub struct CloudAccessPolicy(Rc<CloudAccessPolicy_>);
impl CloudAccessPolicy {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGrafana) -> Self {
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
    #[doc = "Set the field `display_name`.\nDisplay name of the access policy. Defaults to the name."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        self,
        v: impl Into<BlockAssignable<CloudAccessPolicyConditionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.conditions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `realm`.\n"]
    pub fn set_realm(self, v: impl Into<BlockAssignable<CloudAccessPolicyRealmEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().realm = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.realm = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nCreation date of the access policy."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the access policy. Defaults to the name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the access policy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\nID of the access policy."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where the API is deployed. Generally where the stack is deployed. Use the region list API to get the list of available regions: https://grafana.com/docs/grafana-cloud/developer-resources/api-reference/cloud-api/#list-regions."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nScopes of the access policy. See https://grafana.com/docs/grafana-cloud/security-and-account-management/authentication-and-permissions/access-policies/#scopes for possible values."]
    pub fn scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.scopes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\nLast update date of the access policy."]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `realm` after provisioning.\n"]
    pub fn realm(&self) -> ListRef<CloudAccessPolicyRealmElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.realm", self.extract_ref()),
        )
    }
}
impl Referable for CloudAccessPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudAccessPolicy {}
impl ToListMappable for CloudAccessPolicy {
    type O = ListRef<CloudAccessPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudAccessPolicy_ {
    fn extract_resource_type(&self) -> String {
        "grafana_cloud_access_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudAccessPolicy {
    pub tf_id: String,
    #[doc = "Name of the access policy."]
    pub name: PrimField<String>,
    #[doc = "Region where the API is deployed. Generally where the stack is deployed. Use the region list API to get the list of available regions: https://grafana.com/docs/grafana-cloud/developer-resources/api-reference/cloud-api/#list-regions."]
    pub region: PrimField<String>,
    #[doc = "Scopes of the access policy. See https://grafana.com/docs/grafana-cloud/security-and-account-management/authentication-and-permissions/access-policies/#scopes for possible values."]
    pub scopes: SetField<PrimField<String>>,
}
impl BuildCloudAccessPolicy {
    pub fn build(self, stack: &mut Stack) -> CloudAccessPolicy {
        let out = CloudAccessPolicy(Rc::new(CloudAccessPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudAccessPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: self.region,
                scopes: self.scopes,
                conditions: core::default::Default::default(),
                realm: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudAccessPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudAccessPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudAccessPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nCreation date of the access policy."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the access policy. Defaults to the name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the access policy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\nID of the access policy."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where the API is deployed. Generally where the stack is deployed. Use the region list API to get the list of available regions: https://grafana.com/docs/grafana-cloud/developer-resources/api-reference/cloud-api/#list-regions."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nScopes of the access policy. See https://grafana.com/docs/grafana-cloud/security-and-account-management/authentication-and-permissions/access-policies/#scopes for possible values."]
    pub fn scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.scopes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\nLast update date of the access policy."]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `realm` after provisioning.\n"]
    pub fn realm(&self) -> ListRef<CloudAccessPolicyRealmElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.realm", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudAccessPolicyConditionsEl {
    allowed_subnets: SetField<PrimField<String>>,
}
impl CloudAccessPolicyConditionsEl {}
impl ToListMappable for CloudAccessPolicyConditionsEl {
    type O = BlockAssignable<CloudAccessPolicyConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudAccessPolicyConditionsEl {
    #[doc = "Conditions that apply to the access policy,such as IP Allow lists."]
    pub allowed_subnets: SetField<PrimField<String>>,
}
impl BuildCloudAccessPolicyConditionsEl {
    pub fn build(self) -> CloudAccessPolicyConditionsEl {
        CloudAccessPolicyConditionsEl {
            allowed_subnets: self.allowed_subnets,
        }
    }
}
pub struct CloudAccessPolicyConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudAccessPolicyConditionsElRef {
    fn new(shared: StackShared, base: String) -> CloudAccessPolicyConditionsElRef {
        CloudAccessPolicyConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudAccessPolicyConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_subnets` after provisioning.\nConditions that apply to the access policy,such as IP Allow lists."]
    pub fn allowed_subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.allowed_subnets", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudAccessPolicyRealmElLabelPolicyEl {
    selector: PrimField<String>,
}
impl CloudAccessPolicyRealmElLabelPolicyEl {}
impl ToListMappable for CloudAccessPolicyRealmElLabelPolicyEl {
    type O = BlockAssignable<CloudAccessPolicyRealmElLabelPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudAccessPolicyRealmElLabelPolicyEl {
    #[doc = "The label selector to match in metrics or logs query. Should be in PromQL or LogQL format."]
    pub selector: PrimField<String>,
}
impl BuildCloudAccessPolicyRealmElLabelPolicyEl {
    pub fn build(self) -> CloudAccessPolicyRealmElLabelPolicyEl {
        CloudAccessPolicyRealmElLabelPolicyEl {
            selector: self.selector,
        }
    }
}
pub struct CloudAccessPolicyRealmElLabelPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudAccessPolicyRealmElLabelPolicyElRef {
    fn new(shared: StackShared, base: String) -> CloudAccessPolicyRealmElLabelPolicyElRef {
        CloudAccessPolicyRealmElLabelPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudAccessPolicyRealmElLabelPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `selector` after provisioning.\nThe label selector to match in metrics or logs query. Should be in PromQL or LogQL format."]
    pub fn selector(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.selector", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudAccessPolicyRealmElDynamic {
    label_policy: Option<DynamicBlock<CloudAccessPolicyRealmElLabelPolicyEl>>,
}
#[derive(Serialize)]
pub struct CloudAccessPolicyRealmEl {
    identifier: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label_policy: Option<Vec<CloudAccessPolicyRealmElLabelPolicyEl>>,
    dynamic: CloudAccessPolicyRealmElDynamic,
}
impl CloudAccessPolicyRealmEl {
    #[doc = "Set the field `label_policy`.\n"]
    pub fn set_label_policy(
        mut self,
        v: impl Into<BlockAssignable<CloudAccessPolicyRealmElLabelPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.label_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.label_policy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudAccessPolicyRealmEl {
    type O = BlockAssignable<CloudAccessPolicyRealmEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudAccessPolicyRealmEl {
    #[doc = "The identifier of the org or stack. For orgs, this is the slug, for stacks, this is the stack ID."]
    pub identifier: PrimField<String>,
    #[doc = "Whether a policy applies to a Cloud org or a specific stack. Should be one of `org` or `stack`."]
    pub type_: PrimField<String>,
}
impl BuildCloudAccessPolicyRealmEl {
    pub fn build(self) -> CloudAccessPolicyRealmEl {
        CloudAccessPolicyRealmEl {
            identifier: self.identifier,
            type_: self.type_,
            label_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudAccessPolicyRealmElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudAccessPolicyRealmElRef {
    fn new(shared: StackShared, base: String) -> CloudAccessPolicyRealmElRef {
        CloudAccessPolicyRealmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudAccessPolicyRealmElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identifier` after provisioning.\nThe identifier of the org or stack. For orgs, this is the slug, for stacks, this is the stack ID."]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nWhether a policy applies to a Cloud org or a specific stack. Should be one of `org` or `stack`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudAccessPolicyDynamic {
    conditions: Option<DynamicBlock<CloudAccessPolicyConditionsEl>>,
    realm: Option<DynamicBlock<CloudAccessPolicyRealmEl>>,
}
