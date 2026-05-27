use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GkeHubFleetData {
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
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_cluster_config: Option<Vec<GkeHubFleetDefaultClusterConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GkeHubFleetTimeoutsEl>,
    dynamic: GkeHubFleetDynamic,
}
struct GkeHubFleet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GkeHubFleetData>,
}
#[derive(Clone)]
pub struct GkeHubFleet(Rc<GkeHubFleet_>);
impl GkeHubFleet {
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
    #[doc = "Set the field `display_name`.\nA user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters.\nAllowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
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
    #[doc = "Set the field `default_cluster_config`.\n"]
    pub fn set_default_cluster_config(
        self,
        v: impl Into<BlockAssignable<GkeHubFleetDefaultClusterConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_cluster_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_cluster_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GkeHubFleetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time the fleet was created, in RFC3339 text format."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nThe time the fleet was deleted, in RFC3339 text format."]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters.\nAllowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point."]
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
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the fleet resource."]
    pub fn state(&self) -> ListRef<GkeHubFleetStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nGoogle-generated UUID for this resource. This is unique across all\nFleet resources. If a Fleet resource is deleted and another\nresource with the same name is created, it gets a different uid."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time the fleet was last updated, in RFC3339 text format."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_cluster_config` after provisioning.\n"]
    pub fn default_cluster_config(&self) -> ListRef<GkeHubFleetDefaultClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_cluster_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubFleetTimeoutsElRef {
        GkeHubFleetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for GkeHubFleet {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GkeHubFleet {}
impl ToListMappable for GkeHubFleet {
    type O = ListRef<GkeHubFleetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GkeHubFleet_ {
    fn extract_resource_type(&self) -> String {
        "google_gke_hub_fleet".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGkeHubFleet {
    pub tf_id: String,
}
impl BuildGkeHubFleet {
    pub fn build(self, stack: &mut Stack) -> GkeHubFleet {
        let out = GkeHubFleet(Rc::new(GkeHubFleet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GkeHubFleetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                default_cluster_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GkeHubFleetRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFleetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GkeHubFleetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time the fleet was created, in RFC3339 text format."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nThe time the fleet was deleted, in RFC3339 text format."]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters.\nAllowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point."]
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
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the fleet resource."]
    pub fn state(&self) -> ListRef<GkeHubFleetStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nGoogle-generated UUID for this resource. This is unique across all\nFleet resources. If a Fleet resource is deleted and another\nresource with the same name is created, it gets a different uid."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time the fleet was last updated, in RFC3339 text format."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_cluster_config` after provisioning.\n"]
    pub fn default_cluster_config(&self) -> ListRef<GkeHubFleetDefaultClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_cluster_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubFleetTimeoutsElRef {
        GkeHubFleetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GkeHubFleetStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
}
impl GkeHubFleetStateEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.code = Some(v.into());
        self
    }
}
impl ToListMappable for GkeHubFleetStateEl {
    type O = BlockAssignable<GkeHubFleetStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFleetStateEl {}
impl BuildGkeHubFleetStateEl {
    pub fn build(self) -> GkeHubFleetStateEl {
        GkeHubFleetStateEl {
            code: core::default::Default::default(),
        }
    }
}
pub struct GkeHubFleetStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFleetStateElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFleetStateElRef {
        GkeHubFleetStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFleetStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElPolicyBindingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElPolicyBindingsEl {
    #[doc = "Set the field `name`.\nThe relative resource name of the binauthz platform policy to audit. GKE\nplatform policies have the following format:\n'projects/{project_number}/platforms/gke/policies/{policy_id}'."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable
    for GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElPolicyBindingsEl
{
    type O = BlockAssignable<
        GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElPolicyBindingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElPolicyBindingsEl {}
impl BuildGkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElPolicyBindingsEl {
    pub fn build(
        self,
    ) -> GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElPolicyBindingsEl {
        GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElPolicyBindingsEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElPolicyBindingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElPolicyBindingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElPolicyBindingsElRef {
        GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElPolicyBindingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElPolicyBindingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the binauthz platform policy to audit. GKE\nplatform policies have the following format:\n'projects/{project_number}/platforms/gke/policies/{policy_id}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElDynamic {
    policy_bindings: Option<
        DynamicBlock<GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElPolicyBindingsEl>,
    >,
}
#[derive(Serialize)]
pub struct GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluation_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_bindings:
        Option<Vec<GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElPolicyBindingsEl>>,
    dynamic: GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElDynamic,
}
impl GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigEl {
    #[doc = "Set the field `evaluation_mode`.\nMode of operation for binauthz policy evaluation. Possible values: [\"DISABLED\", \"POLICY_BINDINGS\"]"]
    pub fn set_evaluation_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.evaluation_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_bindings`.\n"]
    pub fn set_policy_bindings(
        mut self,
        v: impl Into<
            BlockAssignable<
                GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElPolicyBindingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.policy_bindings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_bindings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigEl {
    type O = BlockAssignable<GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigEl {}
impl BuildGkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigEl {
    pub fn build(self) -> GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigEl {
        GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigEl {
            evaluation_mode: core::default::Default::default(),
            policy_bindings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElRef {
        GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `evaluation_mode` after provisioning.\nMode of operation for binauthz policy evaluation. Possible values: [\"DISABLED\", \"POLICY_BINDINGS\"]"]
    pub fn evaluation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluation_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy_bindings` after provisioning.\n"]
    pub fn policy_bindings(
        &self,
    ) -> ListRef<GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElPolicyBindingsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.policy_bindings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeHubFleetDefaultClusterConfigElSecurityPostureConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vulnerability_mode: Option<PrimField<String>>,
}
impl GkeHubFleetDefaultClusterConfigElSecurityPostureConfigEl {
    #[doc = "Set the field `mode`.\nSets which mode to use for Security Posture features. Possible values: [\"DISABLED\", \"BASIC\", \"ENTERPRISE\"]"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `vulnerability_mode`.\nSets which mode to use for vulnerability scanning. Possible values: [\"VULNERABILITY_DISABLED\", \"VULNERABILITY_BASIC\", \"VULNERABILITY_ENTERPRISE\"]"]
    pub fn set_vulnerability_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vulnerability_mode = Some(v.into());
        self
    }
}
impl ToListMappable for GkeHubFleetDefaultClusterConfigElSecurityPostureConfigEl {
    type O = BlockAssignable<GkeHubFleetDefaultClusterConfigElSecurityPostureConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFleetDefaultClusterConfigElSecurityPostureConfigEl {}
impl BuildGkeHubFleetDefaultClusterConfigElSecurityPostureConfigEl {
    pub fn build(self) -> GkeHubFleetDefaultClusterConfigElSecurityPostureConfigEl {
        GkeHubFleetDefaultClusterConfigElSecurityPostureConfigEl {
            mode: core::default::Default::default(),
            vulnerability_mode: core::default::Default::default(),
        }
    }
}
pub struct GkeHubFleetDefaultClusterConfigElSecurityPostureConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFleetDefaultClusterConfigElSecurityPostureConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeHubFleetDefaultClusterConfigElSecurityPostureConfigElRef {
        GkeHubFleetDefaultClusterConfigElSecurityPostureConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFleetDefaultClusterConfigElSecurityPostureConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nSets which mode to use for Security Posture features. Possible values: [\"DISABLED\", \"BASIC\", \"ENTERPRISE\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `vulnerability_mode` after provisioning.\nSets which mode to use for vulnerability scanning. Possible values: [\"VULNERABILITY_DISABLED\", \"VULNERABILITY_BASIC\", \"VULNERABILITY_ENTERPRISE\"]"]
    pub fn vulnerability_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vulnerability_mode", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GkeHubFleetDefaultClusterConfigElDynamic {
    binary_authorization_config:
        Option<DynamicBlock<GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigEl>>,
    security_posture_config:
        Option<DynamicBlock<GkeHubFleetDefaultClusterConfigElSecurityPostureConfigEl>>,
}
#[derive(Serialize)]
pub struct GkeHubFleetDefaultClusterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    binary_authorization_config:
        Option<Vec<GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_posture_config: Option<Vec<GkeHubFleetDefaultClusterConfigElSecurityPostureConfigEl>>,
    dynamic: GkeHubFleetDefaultClusterConfigElDynamic,
}
impl GkeHubFleetDefaultClusterConfigEl {
    #[doc = "Set the field `binary_authorization_config`.\n"]
    pub fn set_binary_authorization_config(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.binary_authorization_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.binary_authorization_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `security_posture_config`.\n"]
    pub fn set_security_posture_config(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFleetDefaultClusterConfigElSecurityPostureConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.security_posture_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.security_posture_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeHubFleetDefaultClusterConfigEl {
    type O = BlockAssignable<GkeHubFleetDefaultClusterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFleetDefaultClusterConfigEl {}
impl BuildGkeHubFleetDefaultClusterConfigEl {
    pub fn build(self) -> GkeHubFleetDefaultClusterConfigEl {
        GkeHubFleetDefaultClusterConfigEl {
            binary_authorization_config: core::default::Default::default(),
            security_posture_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeHubFleetDefaultClusterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFleetDefaultClusterConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFleetDefaultClusterConfigElRef {
        GkeHubFleetDefaultClusterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFleetDefaultClusterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `binary_authorization_config` after provisioning.\n"]
    pub fn binary_authorization_config(
        &self,
    ) -> ListRef<GkeHubFleetDefaultClusterConfigElBinaryAuthorizationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.binary_authorization_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `security_posture_config` after provisioning.\n"]
    pub fn security_posture_config(
        &self,
    ) -> ListRef<GkeHubFleetDefaultClusterConfigElSecurityPostureConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_posture_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeHubFleetTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl GkeHubFleetTimeoutsEl {
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
impl ToListMappable for GkeHubFleetTimeoutsEl {
    type O = BlockAssignable<GkeHubFleetTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFleetTimeoutsEl {}
impl BuildGkeHubFleetTimeoutsEl {
    pub fn build(self) -> GkeHubFleetTimeoutsEl {
        GkeHubFleetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct GkeHubFleetTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFleetTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFleetTimeoutsElRef {
        GkeHubFleetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFleetTimeoutsElRef {
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
struct GkeHubFleetDynamic {
    default_cluster_config: Option<DynamicBlock<GkeHubFleetDefaultClusterConfigEl>>,
}
