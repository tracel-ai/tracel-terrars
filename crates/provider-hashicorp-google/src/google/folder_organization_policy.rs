use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FolderOrganizationPolicyData {
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
    folder: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boolean_policy: Option<Vec<FolderOrganizationPolicyBooleanPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    list_policy: Option<Vec<FolderOrganizationPolicyListPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_policy: Option<Vec<FolderOrganizationPolicyRestorePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FolderOrganizationPolicyTimeoutsEl>,
    dynamic: FolderOrganizationPolicyDynamic,
}
struct FolderOrganizationPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FolderOrganizationPolicyData>,
}
#[derive(Clone)]
pub struct FolderOrganizationPolicy(Rc<FolderOrganizationPolicy_>);
impl FolderOrganizationPolicy {
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
        v: impl Into<BlockAssignable<FolderOrganizationPolicyBooleanPolicyEl>>,
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
        v: impl Into<BlockAssignable<FolderOrganizationPolicyListPolicyEl>>,
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
        v: impl Into<BlockAssignable<FolderOrganizationPolicyRestorePolicyEl>>,
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
    pub fn set_timeouts(self, v: impl Into<FolderOrganizationPolicyTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `folder` after provisioning.\nThe resource name of the folder to set the policy for. Its format is folders/{folder_id}."]
    pub fn folder(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.folder", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    pub fn boolean_policy(&self) -> ListRef<FolderOrganizationPolicyBooleanPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.boolean_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `list_policy` after provisioning.\n"]
    pub fn list_policy(&self) -> ListRef<FolderOrganizationPolicyListPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.list_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_policy` after provisioning.\n"]
    pub fn restore_policy(&self) -> ListRef<FolderOrganizationPolicyRestorePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FolderOrganizationPolicyTimeoutsElRef {
        FolderOrganizationPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for FolderOrganizationPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FolderOrganizationPolicy {}
impl ToListMappable for FolderOrganizationPolicy {
    type O = ListRef<FolderOrganizationPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FolderOrganizationPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_folder_organization_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFolderOrganizationPolicy {
    pub tf_id: String,
    #[doc = "The name of the Constraint the Policy is configuring, for example, serviceuser.services."]
    pub constraint: PrimField<String>,
    #[doc = "The resource name of the folder to set the policy for. Its format is folders/{folder_id}."]
    pub folder: PrimField<String>,
}
impl BuildFolderOrganizationPolicy {
    pub fn build(self, stack: &mut Stack) -> FolderOrganizationPolicy {
        let out = FolderOrganizationPolicy(Rc::new(FolderOrganizationPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FolderOrganizationPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                constraint: self.constraint,
                deletion_policy: core::default::Default::default(),
                folder: self.folder,
                id: core::default::Default::default(),
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
pub struct FolderOrganizationPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for FolderOrganizationPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FolderOrganizationPolicyRef {
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
    #[doc = "Get a reference to the value of field `folder` after provisioning.\nThe resource name of the folder to set the policy for. Its format is folders/{folder_id}."]
    pub fn folder(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.folder", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    pub fn boolean_policy(&self) -> ListRef<FolderOrganizationPolicyBooleanPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.boolean_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `list_policy` after provisioning.\n"]
    pub fn list_policy(&self) -> ListRef<FolderOrganizationPolicyListPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.list_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_policy` after provisioning.\n"]
    pub fn restore_policy(&self) -> ListRef<FolderOrganizationPolicyRestorePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FolderOrganizationPolicyTimeoutsElRef {
        FolderOrganizationPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FolderOrganizationPolicyBooleanPolicyEl {
    enforced: PrimField<bool>,
}
impl FolderOrganizationPolicyBooleanPolicyEl {}
impl ToListMappable for FolderOrganizationPolicyBooleanPolicyEl {
    type O = BlockAssignable<FolderOrganizationPolicyBooleanPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFolderOrganizationPolicyBooleanPolicyEl {
    #[doc = "If true, then the Policy is enforced. If false, then any configuration is acceptable."]
    pub enforced: PrimField<bool>,
}
impl BuildFolderOrganizationPolicyBooleanPolicyEl {
    pub fn build(self) -> FolderOrganizationPolicyBooleanPolicyEl {
        FolderOrganizationPolicyBooleanPolicyEl {
            enforced: self.enforced,
        }
    }
}
pub struct FolderOrganizationPolicyBooleanPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FolderOrganizationPolicyBooleanPolicyElRef {
    fn new(shared: StackShared, base: String) -> FolderOrganizationPolicyBooleanPolicyElRef {
        FolderOrganizationPolicyBooleanPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FolderOrganizationPolicyBooleanPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enforced` after provisioning.\nIf true, then the Policy is enforced. If false, then any configuration is acceptable."]
    pub fn enforced(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforced", self.base))
    }
}
#[derive(Serialize)]
pub struct FolderOrganizationPolicyListPolicyElAllowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}
impl FolderOrganizationPolicyListPolicyElAllowEl {
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
impl ToListMappable for FolderOrganizationPolicyListPolicyElAllowEl {
    type O = BlockAssignable<FolderOrganizationPolicyListPolicyElAllowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFolderOrganizationPolicyListPolicyElAllowEl {}
impl BuildFolderOrganizationPolicyListPolicyElAllowEl {
    pub fn build(self) -> FolderOrganizationPolicyListPolicyElAllowEl {
        FolderOrganizationPolicyListPolicyElAllowEl {
            all: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct FolderOrganizationPolicyListPolicyElAllowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FolderOrganizationPolicyListPolicyElAllowElRef {
    fn new(shared: StackShared, base: String) -> FolderOrganizationPolicyListPolicyElAllowElRef {
        FolderOrganizationPolicyListPolicyElAllowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FolderOrganizationPolicyListPolicyElAllowElRef {
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
pub struct FolderOrganizationPolicyListPolicyElDenyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}
impl FolderOrganizationPolicyListPolicyElDenyEl {
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
impl ToListMappable for FolderOrganizationPolicyListPolicyElDenyEl {
    type O = BlockAssignable<FolderOrganizationPolicyListPolicyElDenyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFolderOrganizationPolicyListPolicyElDenyEl {}
impl BuildFolderOrganizationPolicyListPolicyElDenyEl {
    pub fn build(self) -> FolderOrganizationPolicyListPolicyElDenyEl {
        FolderOrganizationPolicyListPolicyElDenyEl {
            all: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct FolderOrganizationPolicyListPolicyElDenyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FolderOrganizationPolicyListPolicyElDenyElRef {
    fn new(shared: StackShared, base: String) -> FolderOrganizationPolicyListPolicyElDenyElRef {
        FolderOrganizationPolicyListPolicyElDenyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FolderOrganizationPolicyListPolicyElDenyElRef {
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
struct FolderOrganizationPolicyListPolicyElDynamic {
    allow: Option<DynamicBlock<FolderOrganizationPolicyListPolicyElAllowEl>>,
    deny: Option<DynamicBlock<FolderOrganizationPolicyListPolicyElDenyEl>>,
}
#[derive(Serialize)]
pub struct FolderOrganizationPolicyListPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    inherit_from_parent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow: Option<Vec<FolderOrganizationPolicyListPolicyElAllowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deny: Option<Vec<FolderOrganizationPolicyListPolicyElDenyEl>>,
    dynamic: FolderOrganizationPolicyListPolicyElDynamic,
}
impl FolderOrganizationPolicyListPolicyEl {
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
        v: impl Into<BlockAssignable<FolderOrganizationPolicyListPolicyElAllowEl>>,
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
        v: impl Into<BlockAssignable<FolderOrganizationPolicyListPolicyElDenyEl>>,
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
impl ToListMappable for FolderOrganizationPolicyListPolicyEl {
    type O = BlockAssignable<FolderOrganizationPolicyListPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFolderOrganizationPolicyListPolicyEl {}
impl BuildFolderOrganizationPolicyListPolicyEl {
    pub fn build(self) -> FolderOrganizationPolicyListPolicyEl {
        FolderOrganizationPolicyListPolicyEl {
            inherit_from_parent: core::default::Default::default(),
            suggested_value: core::default::Default::default(),
            allow: core::default::Default::default(),
            deny: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FolderOrganizationPolicyListPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FolderOrganizationPolicyListPolicyElRef {
    fn new(shared: StackShared, base: String) -> FolderOrganizationPolicyListPolicyElRef {
        FolderOrganizationPolicyListPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FolderOrganizationPolicyListPolicyElRef {
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
    pub fn allow(&self) -> ListRef<FolderOrganizationPolicyListPolicyElAllowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.allow", self.base))
    }
    #[doc = "Get a reference to the value of field `deny` after provisioning.\n"]
    pub fn deny(&self) -> ListRef<FolderOrganizationPolicyListPolicyElDenyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deny", self.base))
    }
}
#[derive(Serialize)]
pub struct FolderOrganizationPolicyRestorePolicyEl {
    default: PrimField<bool>,
}
impl FolderOrganizationPolicyRestorePolicyEl {}
impl ToListMappable for FolderOrganizationPolicyRestorePolicyEl {
    type O = BlockAssignable<FolderOrganizationPolicyRestorePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFolderOrganizationPolicyRestorePolicyEl {
    #[doc = "May only be set to true. If set, then the default Policy is restored."]
    pub default: PrimField<bool>,
}
impl BuildFolderOrganizationPolicyRestorePolicyEl {
    pub fn build(self) -> FolderOrganizationPolicyRestorePolicyEl {
        FolderOrganizationPolicyRestorePolicyEl {
            default: self.default,
        }
    }
}
pub struct FolderOrganizationPolicyRestorePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FolderOrganizationPolicyRestorePolicyElRef {
    fn new(shared: StackShared, base: String) -> FolderOrganizationPolicyRestorePolicyElRef {
        FolderOrganizationPolicyRestorePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FolderOrganizationPolicyRestorePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default` after provisioning.\nMay only be set to true. If set, then the default Policy is restored."]
    pub fn default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }
}
#[derive(Serialize)]
pub struct FolderOrganizationPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl FolderOrganizationPolicyTimeoutsEl {
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
impl ToListMappable for FolderOrganizationPolicyTimeoutsEl {
    type O = BlockAssignable<FolderOrganizationPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFolderOrganizationPolicyTimeoutsEl {}
impl BuildFolderOrganizationPolicyTimeoutsEl {
    pub fn build(self) -> FolderOrganizationPolicyTimeoutsEl {
        FolderOrganizationPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            read: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct FolderOrganizationPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FolderOrganizationPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FolderOrganizationPolicyTimeoutsElRef {
        FolderOrganizationPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FolderOrganizationPolicyTimeoutsElRef {
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
struct FolderOrganizationPolicyDynamic {
    boolean_policy: Option<DynamicBlock<FolderOrganizationPolicyBooleanPolicyEl>>,
    list_policy: Option<DynamicBlock<FolderOrganizationPolicyListPolicyEl>>,
    restore_policy: Option<DynamicBlock<FolderOrganizationPolicyRestorePolicyEl>>,
}
