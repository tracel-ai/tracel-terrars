use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct OrganizationPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    constraint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    org_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boolean_policy: Option<Vec<OrganizationPolicyBooleanPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    list_policy: Option<Vec<OrganizationPolicyListPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_policy: Option<Vec<OrganizationPolicyRestorePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OrganizationPolicyTimeoutsEl>,
    dynamic: OrganizationPolicyDynamic,
}
struct OrganizationPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OrganizationPolicyData>,
}
#[derive(Clone)]
pub struct OrganizationPolicy(Rc<OrganizationPolicy_>);
impl OrganizationPolicy {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nVersion of the Policy. Default version is 0."]
    pub fn set_version(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().version = Some(v.into());
        self
    }
    #[doc = "Set the field `boolean_policy`.\n"]
    pub fn set_boolean_policy(
        self,
        v: impl Into<BlockAssignable<OrganizationPolicyBooleanPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().boolean_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.boolean_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `list_policy`.\n"]
    pub fn set_list_policy(
        self,
        v: impl Into<BlockAssignable<OrganizationPolicyListPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().list_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.list_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `restore_policy`.\n"]
    pub fn set_restore_policy(
        self,
        v: impl Into<BlockAssignable<OrganizationPolicyRestorePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restore_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restore_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<OrganizationPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `constraint` after provisioning.\nThe name of the Constraint the Policy is configuring, for example, serviceuser.services."]
    pub fn constraint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.constraint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag of the organization policy. etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\n"]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds, representing when the variable was last updated. Example: \"2016-10-09T12:33:37.578138407Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the Policy. Default version is 0."]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `boolean_policy` after provisioning.\n"]
    pub fn boolean_policy(&self) -> ListRef<OrganizationPolicyBooleanPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.boolean_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `list_policy` after provisioning.\n"]
    pub fn list_policy(&self) -> ListRef<OrganizationPolicyListPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.list_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_policy` after provisioning.\n"]
    pub fn restore_policy(&self) -> ListRef<OrganizationPolicyRestorePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OrganizationPolicyTimeoutsElRef {
        OrganizationPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for OrganizationPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for OrganizationPolicy {}
impl ToListMappable for OrganizationPolicy {
    type O = ListRef<OrganizationPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for OrganizationPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_organization_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildOrganizationPolicy {
    pub tf_id: String,
    #[doc = "The name of the Constraint the Policy is configuring, for example, serviceuser.services."]
    pub constraint: PrimField<String>,
    #[doc = ""]
    pub org_id: PrimField<String>,
}
impl BuildOrganizationPolicy {
    pub fn build(self, stack: &mut Stack) -> OrganizationPolicy {
        let out = OrganizationPolicy(Rc::new(OrganizationPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OrganizationPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                constraint: self.constraint,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                org_id: self.org_id,
                version: core::default::Default::default(),
                boolean_policy: core::default::Default::default(),
                list_policy: core::default::Default::default(),
                restore_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct OrganizationPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrganizationPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl OrganizationPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `constraint` after provisioning.\nThe name of the Constraint the Policy is configuring, for example, serviceuser.services."]
    pub fn constraint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.constraint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag of the organization policy. etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\n"]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds, representing when the variable was last updated. Example: \"2016-10-09T12:33:37.578138407Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the Policy. Default version is 0."]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `boolean_policy` after provisioning.\n"]
    pub fn boolean_policy(&self) -> ListRef<OrganizationPolicyBooleanPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.boolean_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `list_policy` after provisioning.\n"]
    pub fn list_policy(&self) -> ListRef<OrganizationPolicyListPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.list_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_policy` after provisioning.\n"]
    pub fn restore_policy(&self) -> ListRef<OrganizationPolicyRestorePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OrganizationPolicyTimeoutsElRef {
        OrganizationPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct OrganizationPolicyBooleanPolicyEl {
    enforced: PrimField<bool>,
}
impl OrganizationPolicyBooleanPolicyEl {}
impl ToListMappable for OrganizationPolicyBooleanPolicyEl {
    type O = BlockAssignable<OrganizationPolicyBooleanPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOrganizationPolicyBooleanPolicyEl {
    #[doc = "If true, then the Policy is enforced. If false, then any configuration is acceptable."]
    pub enforced: PrimField<bool>,
}
impl BuildOrganizationPolicyBooleanPolicyEl {
    pub fn build(self) -> OrganizationPolicyBooleanPolicyEl {
        OrganizationPolicyBooleanPolicyEl {
            enforced: self.enforced,
        }
    }
}
pub struct OrganizationPolicyBooleanPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrganizationPolicyBooleanPolicyElRef {
    fn new(shared: StackShared, base: String) -> OrganizationPolicyBooleanPolicyElRef {
        OrganizationPolicyBooleanPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OrganizationPolicyBooleanPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enforced` after provisioning.\nIf true, then the Policy is enforced. If false, then any configuration is acceptable."]
    pub fn enforced(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforced", self.base))
    }
}
#[derive(Serialize)]
pub struct OrganizationPolicyListPolicyElAllowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}
impl OrganizationPolicyListPolicyElAllowEl {
    #[doc = "Set the field `all`.\nThe policy allows or denies all values."]
    pub fn set_all(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\nThe policy can define specific values that are allowed or denied."]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for OrganizationPolicyListPolicyElAllowEl {
    type O = BlockAssignable<OrganizationPolicyListPolicyElAllowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOrganizationPolicyListPolicyElAllowEl {}
impl BuildOrganizationPolicyListPolicyElAllowEl {
    pub fn build(self) -> OrganizationPolicyListPolicyElAllowEl {
        OrganizationPolicyListPolicyElAllowEl {
            all: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct OrganizationPolicyListPolicyElAllowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrganizationPolicyListPolicyElAllowElRef {
    fn new(shared: StackShared, base: String) -> OrganizationPolicyListPolicyElAllowElRef {
        OrganizationPolicyListPolicyElAllowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OrganizationPolicyListPolicyElAllowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all` after provisioning.\nThe policy allows or denies all values."]
    pub fn all(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\nThe policy can define specific values that are allowed or denied."]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct OrganizationPolicyListPolicyElDenyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}
impl OrganizationPolicyListPolicyElDenyEl {
    #[doc = "Set the field `all`.\nThe policy allows or denies all values."]
    pub fn set_all(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\nThe policy can define specific values that are allowed or denied."]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for OrganizationPolicyListPolicyElDenyEl {
    type O = BlockAssignable<OrganizationPolicyListPolicyElDenyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOrganizationPolicyListPolicyElDenyEl {}
impl BuildOrganizationPolicyListPolicyElDenyEl {
    pub fn build(self) -> OrganizationPolicyListPolicyElDenyEl {
        OrganizationPolicyListPolicyElDenyEl {
            all: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct OrganizationPolicyListPolicyElDenyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrganizationPolicyListPolicyElDenyElRef {
    fn new(shared: StackShared, base: String) -> OrganizationPolicyListPolicyElDenyElRef {
        OrganizationPolicyListPolicyElDenyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OrganizationPolicyListPolicyElDenyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all` after provisioning.\nThe policy allows or denies all values."]
    pub fn all(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\nThe policy can define specific values that are allowed or denied."]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize, Default)]
struct OrganizationPolicyListPolicyElDynamic {
    allow: Option<DynamicBlock<OrganizationPolicyListPolicyElAllowEl>>,
    deny: Option<DynamicBlock<OrganizationPolicyListPolicyElDenyEl>>,
}
#[derive(Serialize)]
pub struct OrganizationPolicyListPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    inherit_from_parent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow: Option<Vec<OrganizationPolicyListPolicyElAllowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deny: Option<Vec<OrganizationPolicyListPolicyElDenyEl>>,
    dynamic: OrganizationPolicyListPolicyElDynamic,
}
impl OrganizationPolicyListPolicyEl {
    #[doc = "Set the field `inherit_from_parent`.\nIf set to true, the values from the effective Policy of the parent resource are inherited, meaning the values set in this Policy are added to the values inherited up the hierarchy."]
    pub fn set_inherit_from_parent(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.inherit_from_parent = Some(v.into());
        self
    }
    #[doc = "Set the field `suggested_value`.\nThe Google Cloud Console will try to default to a configuration that matches the value specified in this field."]
    pub fn set_suggested_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suggested_value = Some(v.into());
        self
    }
    #[doc = "Set the field `allow`.\n"]
    pub fn set_allow(
        mut self,
        v: impl Into<BlockAssignable<OrganizationPolicyListPolicyElAllowEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allow = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allow = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `deny`.\n"]
    pub fn set_deny(
        mut self,
        v: impl Into<BlockAssignable<OrganizationPolicyListPolicyElDenyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.deny = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.deny = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OrganizationPolicyListPolicyEl {
    type O = BlockAssignable<OrganizationPolicyListPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOrganizationPolicyListPolicyEl {}
impl BuildOrganizationPolicyListPolicyEl {
    pub fn build(self) -> OrganizationPolicyListPolicyEl {
        OrganizationPolicyListPolicyEl {
            inherit_from_parent: core::default::Default::default(),
            suggested_value: core::default::Default::default(),
            allow: core::default::Default::default(),
            deny: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OrganizationPolicyListPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrganizationPolicyListPolicyElRef {
    fn new(shared: StackShared, base: String) -> OrganizationPolicyListPolicyElRef {
        OrganizationPolicyListPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OrganizationPolicyListPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `inherit_from_parent` after provisioning.\nIf set to true, the values from the effective Policy of the parent resource are inherited, meaning the values set in this Policy are added to the values inherited up the hierarchy."]
    pub fn inherit_from_parent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inherit_from_parent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `suggested_value` after provisioning.\nThe Google Cloud Console will try to default to a configuration that matches the value specified in this field."]
    pub fn suggested_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.suggested_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow` after provisioning.\n"]
    pub fn allow(&self) -> ListRef<OrganizationPolicyListPolicyElAllowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.allow", self.base))
    }
    #[doc = "Get a reference to the value of field `deny` after provisioning.\n"]
    pub fn deny(&self) -> ListRef<OrganizationPolicyListPolicyElDenyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deny", self.base))
    }
}
#[derive(Serialize)]
pub struct OrganizationPolicyRestorePolicyEl {
    default: PrimField<bool>,
}
impl OrganizationPolicyRestorePolicyEl {}
impl ToListMappable for OrganizationPolicyRestorePolicyEl {
    type O = BlockAssignable<OrganizationPolicyRestorePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOrganizationPolicyRestorePolicyEl {
    #[doc = "May only be set to true. If set, then the default Policy is restored."]
    pub default: PrimField<bool>,
}
impl BuildOrganizationPolicyRestorePolicyEl {
    pub fn build(self) -> OrganizationPolicyRestorePolicyEl {
        OrganizationPolicyRestorePolicyEl {
            default: self.default,
        }
    }
}
pub struct OrganizationPolicyRestorePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrganizationPolicyRestorePolicyElRef {
    fn new(shared: StackShared, base: String) -> OrganizationPolicyRestorePolicyElRef {
        OrganizationPolicyRestorePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OrganizationPolicyRestorePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default` after provisioning.\nMay only be set to true. If set, then the default Policy is restored."]
    pub fn default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }
}
#[derive(Serialize)]
pub struct OrganizationPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl OrganizationPolicyTimeoutsEl {
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
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for OrganizationPolicyTimeoutsEl {
    type O = BlockAssignable<OrganizationPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOrganizationPolicyTimeoutsEl {}
impl BuildOrganizationPolicyTimeoutsEl {
    pub fn build(self) -> OrganizationPolicyTimeoutsEl {
        OrganizationPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            read: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct OrganizationPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrganizationPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OrganizationPolicyTimeoutsElRef {
        OrganizationPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OrganizationPolicyTimeoutsElRef {
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
    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct OrganizationPolicyDynamic {
    boolean_policy: Option<DynamicBlock<OrganizationPolicyBooleanPolicyEl>>,
    list_policy: Option<DynamicBlock<OrganizationPolicyListPolicyEl>>,
    restore_policy: Option<DynamicBlock<OrganizationPolicyRestorePolicyEl>>,
}
