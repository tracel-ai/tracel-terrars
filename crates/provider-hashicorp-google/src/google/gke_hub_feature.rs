use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GkeHubFeatureData {
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
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet_default_member_config: Option<Vec<GkeHubFeatureFleetDefaultMemberConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spec: Option<Vec<GkeHubFeatureSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GkeHubFeatureTimeoutsEl>,
    dynamic: GkeHubFeatureDynamic,
}
struct GkeHubFeature_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GkeHubFeatureData>,
}
#[derive(Clone)]
pub struct GkeHubFeature(Rc<GkeHubFeature_>);
impl GkeHubFeature {
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
    #[doc = "Set the field `labels`.\nGCP labels for this Feature.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nThe full, unique name of this Feature resource"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `fleet_default_member_config`.\n"]
    pub fn set_fleet_default_member_config(
        self,
        v: impl Into<BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fleet_default_member_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fleet_default_member_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl Into<BlockAssignable<GkeHubFeatureSpecEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GkeHubFeatureTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. When the Feature resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. When the Feature resource was deleted."]
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
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nGCP labels for this Feature.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full, unique name of this Feature resource"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_state` after provisioning.\nState of the Feature resource itself."]
    pub fn resource_state(&self) -> ListRef<GkeHubFeatureResourceStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The Hub-wide Feature state"]
    pub fn state(&self) -> ListRef<GkeHubFeatureStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. When the Feature resource was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_default_member_config` after provisioning.\n"]
    pub fn fleet_default_member_config(
        &self,
    ) -> ListRef<GkeHubFeatureFleetDefaultMemberConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet_default_member_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<GkeHubFeatureSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubFeatureTimeoutsElRef {
        GkeHubFeatureTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for GkeHubFeature {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GkeHubFeature {}
impl ToListMappable for GkeHubFeature {
    type O = ListRef<GkeHubFeatureRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GkeHubFeature_ {
    fn extract_resource_type(&self) -> String {
        "google_gke_hub_feature".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGkeHubFeature {
    pub tf_id: String,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
}
impl BuildGkeHubFeature {
    pub fn build(self, stack: &mut Stack) -> GkeHubFeature {
        let out = GkeHubFeature(Rc::new(GkeHubFeature_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GkeHubFeatureData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                fleet_default_member_config: core::default::Default::default(),
                spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GkeHubFeatureRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GkeHubFeatureRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. When the Feature resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. When the Feature resource was deleted."]
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
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nGCP labels for this Feature.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full, unique name of this Feature resource"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_state` after provisioning.\nState of the Feature resource itself."]
    pub fn resource_state(&self) -> ListRef<GkeHubFeatureResourceStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The Hub-wide Feature state"]
    pub fn state(&self) -> ListRef<GkeHubFeatureStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. When the Feature resource was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_default_member_config` after provisioning.\n"]
    pub fn fleet_default_member_config(
        &self,
    ) -> ListRef<GkeHubFeatureFleetDefaultMemberConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet_default_member_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<GkeHubFeatureSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubFeatureTimeoutsElRef {
        GkeHubFeatureTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GkeHubFeatureResourceStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    has_resources: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl GkeHubFeatureResourceStateEl {
    #[doc = "Set the field `has_resources`.\n"]
    pub fn set_has_resources(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.has_resources = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for GkeHubFeatureResourceStateEl {
    type O = BlockAssignable<GkeHubFeatureResourceStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureResourceStateEl {}
impl BuildGkeHubFeatureResourceStateEl {
    pub fn build(self) -> GkeHubFeatureResourceStateEl {
        GkeHubFeatureResourceStateEl {
            has_resources: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct GkeHubFeatureResourceStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureResourceStateElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureResourceStateElRef {
        GkeHubFeatureResourceStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureResourceStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `has_resources` after provisioning.\n"]
    pub fn has_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.has_resources", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeHubFeatureStateElStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl GkeHubFeatureStateElStateEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for GkeHubFeatureStateElStateEl {
    type O = BlockAssignable<GkeHubFeatureStateElStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureStateElStateEl {}
impl BuildGkeHubFeatureStateElStateEl {
    pub fn build(self) -> GkeHubFeatureStateElStateEl {
        GkeHubFeatureStateElStateEl {
            code: core::default::Default::default(),
            description: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct GkeHubFeatureStateElStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureStateElStateElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureStateElStateElRef {
        GkeHubFeatureStateElStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureStateElStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeHubFeatureStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<ListField<GkeHubFeatureStateElStateEl>>,
}
impl GkeHubFeatureStateEl {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<ListField<GkeHubFeatureStateElStateEl>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for GkeHubFeatureStateEl {
    type O = BlockAssignable<GkeHubFeatureStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureStateEl {}
impl BuildGkeHubFeatureStateEl {
    pub fn build(self) -> GkeHubFeatureStateEl {
        GkeHubFeatureStateEl {
            state: core::default::Default::default(),
        }
    }
}
pub struct GkeHubFeatureStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureStateElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureStateElRef {
        GkeHubFeatureStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> ListRef<GkeHubFeatureStateElStateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_proxy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_dir: Option<PrimField<String>>,
    secret_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_repo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_rev: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_wait_secs: Option<PrimField<String>>,
}
impl GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
    #[doc = "Set the field `gcp_service_account_email`.\nThe Google Cloud Service Account Email used for auth when secretType is gcpServiceAccount"]
    pub fn set_gcp_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `https_proxy`.\nURL for the HTTPS Proxy to be used when communicating with the Git repo"]
    pub fn set_https_proxy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.https_proxy = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_dir`.\nThe path within the Git repository that represents the top level of the repo to sync"]
    pub fn set_policy_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy_dir = Some(v.into());
        self
    }
    #[doc = "Set the field `sync_branch`.\nThe branch of the repository to sync from. Default: master"]
    pub fn set_sync_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sync_branch = Some(v.into());
        self
    }
    #[doc = "Set the field `sync_repo`.\nThe URL of the Git repository to use as the source of truth"]
    pub fn set_sync_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sync_repo = Some(v.into());
        self
    }
    #[doc = "Set the field `sync_rev`.\nGit revision (tag or hash) to check out. Default HEAD"]
    pub fn set_sync_rev(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sync_rev = Some(v.into());
        self
    }
    #[doc = "Set the field `sync_wait_secs`.\nPeriod in seconds between consecutive syncs. Default: 15"]
    pub fn set_sync_wait_secs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sync_wait_secs = Some(v.into());
        self
    }
}
impl ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
    type O =
        BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
    #[doc = "Type of secret configured for access to the Git repo"]
    pub secret_type: PrimField<String>,
}
impl BuildGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
    pub fn build(
        self,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
        GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
            gcp_service_account_email: core::default::Default::default(),
            https_proxy: core::default::Default::default(),
            policy_dir: core::default::Default::default(),
            secret_type: self.secret_type,
            sync_branch: core::default::Default::default(),
            sync_repo: core::default::Default::default(),
            sync_rev: core::default::Default::default(),
            sync_wait_secs: core::default::Default::default(),
        }
    }
}
pub struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitElRef {
        GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcp_service_account_email` after provisioning.\nThe Google Cloud Service Account Email used for auth when secretType is gcpServiceAccount"]
    pub fn gcp_service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `https_proxy` after provisioning.\nURL for the HTTPS Proxy to be used when communicating with the Git repo"]
    pub fn https_proxy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.https_proxy", self.base))
    }
    #[doc = "Get a reference to the value of field `policy_dir` after provisioning.\nThe path within the Git repository that represents the top level of the repo to sync"]
    pub fn policy_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_dir", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_type` after provisioning.\nType of secret configured for access to the Git repo"]
    pub fn secret_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_type", self.base))
    }
    #[doc = "Get a reference to the value of field `sync_branch` after provisioning.\nThe branch of the repository to sync from. Default: master"]
    pub fn sync_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sync_branch", self.base))
    }
    #[doc = "Get a reference to the value of field `sync_repo` after provisioning.\nThe URL of the Git repository to use as the source of truth"]
    pub fn sync_repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sync_repo", self.base))
    }
    #[doc = "Get a reference to the value of field `sync_rev` after provisioning.\nGit revision (tag or hash) to check out. Default HEAD"]
    pub fn sync_rev(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sync_rev", self.base))
    }
    #[doc = "Get a reference to the value of field `sync_wait_secs` after provisioning.\nPeriod in seconds between consecutive syncs. Default: 15"]
    pub fn sync_wait_secs(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sync_wait_secs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_dir: Option<PrimField<String>>,
    secret_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_repo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_wait_secs: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
    #[doc = "Set the field `gcp_service_account_email`.\nThe Google Cloud Service Account Email used for auth when secretType is gcpServiceAccount"]
    pub fn set_gcp_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_dir`.\nThe absolute path of the directory that contains the local resources. Default: the root directory of the image"]
    pub fn set_policy_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy_dir = Some(v.into());
        self
    }
    #[doc = "Set the field `sync_repo`.\nThe OCI image repository URL for the package to sync from"]
    pub fn set_sync_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sync_repo = Some(v.into());
        self
    }
    #[doc = "Set the field `sync_wait_secs`.\nPeriod in seconds between consecutive syncs. Default: 15"]
    pub fn set_sync_wait_secs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sync_wait_secs = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nVersion of Config Sync installed"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
    type O =
        BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
    #[doc = "Type of secret configured for access to the Git repo"]
    pub secret_type: PrimField<String>,
}
impl BuildGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
    pub fn build(
        self,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
        GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
            gcp_service_account_email: core::default::Default::default(),
            policy_dir: core::default::Default::default(),
            secret_type: self.secret_type,
            sync_repo: core::default::Default::default(),
            sync_wait_secs: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciElRef {
        GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcp_service_account_email` after provisioning.\nThe Google Cloud Service Account Email used for auth when secretType is gcpServiceAccount"]
    pub fn gcp_service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy_dir` after provisioning.\nThe absolute path of the directory that contains the local resources. Default: the root directory of the image"]
    pub fn policy_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_dir", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_type` after provisioning.\nType of secret configured for access to the Git repo"]
    pub fn secret_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_type", self.base))
    }
    #[doc = "Get a reference to the value of field `sync_repo` after provisioning.\nThe OCI image repository URL for the package to sync from"]
    pub fn sync_repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sync_repo", self.base))
    }
    #[doc = "Get a reference to the value of field `sync_wait_secs` after provisioning.\nPeriod in seconds between consecutive syncs. Default: 15"]
    pub fn sync_wait_secs(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sync_wait_secs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of Config Sync installed"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElDynamic {
    git: Option<
        DynamicBlock<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl>,
    >,
    oci: Option<
        DynamicBlock<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl>,
    >,
}
#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics_gcp_service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prevent_drift: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git: Option<Vec<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oci: Option<Vec<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl>>,
    dynamic: GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElDynamic,
}
impl GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {
    #[doc = "Set the field `enabled`.\nEnables the installation of ConfigSync. If set to true, ConfigSync resources will be created and the other ConfigSync fields will be applied if exist. If set to false, all other ConfigSync fields will be ignored, ConfigSync resources will be deleted. If omitted, ConfigSync resources will be managed depends on the presence of the git or oci field."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `metrics_gcp_service_account_email`.\nThe Email of the Google Cloud Service Account (GSA) used for exporting Config Sync metrics to Cloud Monitoring. The GSA should have the Monitoring Metric Writer(roles/monitoring.metricWriter) IAM role. The Kubernetes ServiceAccount 'default' in the namespace 'config-management-monitoring' should be bound to the GSA."]
    pub fn set_metrics_gcp_service_account_email(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.metrics_gcp_service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `prevent_drift`.\nSet to true to enable the Config Sync admission webhook to prevent drifts. If set to 'false', disables the Config Sync admission webhook and does not prevent drifts."]
    pub fn set_prevent_drift(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.prevent_drift = Some(v.into());
        self
    }
    #[doc = "Set the field `source_format`.\nSpecifies whether the Config Sync Repo is in hierarchical or unstructured mode"]
    pub fn set_source_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_format = Some(v.into());
        self
    }
    #[doc = "Set the field `git`.\n"]
    pub fn set_git(
        mut self,
        v: impl Into<
            BlockAssignable<
                GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.git = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.git = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oci`.\n"]
    pub fn set_oci(
        mut self,
        v: impl Into<
            BlockAssignable<
                GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oci = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oci = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {
    type O = BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {}
impl BuildGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {
    pub fn build(self) -> GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {
        GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {
            enabled: core::default::Default::default(),
            metrics_gcp_service_account_email: core::default::Default::default(),
            prevent_drift: core::default::Default::default(),
            source_format: core::default::Default::default(),
            git: core::default::Default::default(),
            oci: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElRef {
        GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nEnables the installation of ConfigSync. If set to true, ConfigSync resources will be created and the other ConfigSync fields will be applied if exist. If set to false, all other ConfigSync fields will be ignored, ConfigSync resources will be deleted. If omitted, ConfigSync resources will be managed depends on the presence of the git or oci field."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `metrics_gcp_service_account_email` after provisioning.\nThe Email of the Google Cloud Service Account (GSA) used for exporting Config Sync metrics to Cloud Monitoring. The GSA should have the Monitoring Metric Writer(roles/monitoring.metricWriter) IAM role. The Kubernetes ServiceAccount 'default' in the namespace 'config-management-monitoring' should be bound to the GSA."]
    pub fn metrics_gcp_service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metrics_gcp_service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prevent_drift` after provisioning.\nSet to true to enable the Config Sync admission webhook to prevent drifts. If set to 'false', disables the Config Sync admission webhook and does not prevent drifts."]
    pub fn prevent_drift(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prevent_drift", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_format` after provisioning.\nSpecifies whether the Config Sync Repo is in hierarchical or unstructured mode"]
    pub fn source_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(
        &self,
    ) -> ListRef<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }
    #[doc = "Get a reference to the value of field `oci` after provisioning.\n"]
    pub fn oci(
        &self,
    ) -> ListRef<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.oci", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElDynamic {
    config_sync:
        Option<DynamicBlock<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl>>,
}
#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    management: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_sync: Option<Vec<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl>>,
    dynamic: GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElDynamic,
}
impl GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {
    #[doc = "Set the field `management`.\nSet this field to MANAGEMENT_AUTOMATIC to enable Config Sync auto-upgrades, and set this field to MANAGEMENT_MANUAL or MANAGEMENT_UNSPECIFIED to disable Config Sync auto-upgrades. Possible values: [\"MANAGEMENT_UNSPECIFIED\", \"MANAGEMENT_AUTOMATIC\", \"MANAGEMENT_MANUAL\"]"]
    pub fn set_management(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.management = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nVersion of Config Sync installed"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
    #[doc = "Set the field `config_sync`.\n"]
    pub fn set_config_sync(
        mut self,
        v: impl Into<
            BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.config_sync = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.config_sync = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {
    type O = BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {}
impl BuildGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {
    pub fn build(self) -> GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {
        GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {
            management: core::default::Default::default(),
            version: core::default::Default::default(),
            config_sync: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElRef {
        GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `management` after provisioning.\nSet this field to MANAGEMENT_AUTOMATIC to enable Config Sync auto-upgrades, and set this field to MANAGEMENT_MANUAL or MANAGEMENT_UNSPECIFIED to disable Config Sync auto-upgrades. Possible values: [\"MANAGEMENT_UNSPECIFIED\", \"MANAGEMENT_AUTOMATIC\", \"MANAGEMENT_MANUAL\"]"]
    pub fn management(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.management", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of Config Sync installed"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
    #[doc = "Get a reference to the value of field `config_sync` after provisioning.\n"]
    pub fn config_sync(
        &self,
    ) -> ListRef<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config_sync", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElMeshEl {
    management: PrimField<String>,
}
impl GkeHubFeatureFleetDefaultMemberConfigElMeshEl {}
impl ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElMeshEl {
    type O = BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElMeshEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElMeshEl {
    #[doc = "Whether to automatically manage Service Mesh Possible values: [\"MANAGEMENT_UNSPECIFIED\", \"MANAGEMENT_AUTOMATIC\", \"MANAGEMENT_MANUAL\"]"]
    pub management: PrimField<String>,
}
impl BuildGkeHubFeatureFleetDefaultMemberConfigElMeshEl {
    pub fn build(self) -> GkeHubFeatureFleetDefaultMemberConfigElMeshEl {
        GkeHubFeatureFleetDefaultMemberConfigElMeshEl {
            management: self.management,
        }
    }
}
pub struct GkeHubFeatureFleetDefaultMemberConfigElMeshElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureFleetDefaultMemberConfigElMeshElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureFleetDefaultMemberConfigElMeshElRef {
        GkeHubFeatureFleetDefaultMemberConfigElMeshElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureFleetDefaultMemberConfigElMeshElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `management` after provisioning.\nWhether to automatically manage Service Mesh Possible values: [\"MANAGEMENT_UNSPECIFIED\", \"MANAGEMENT_AUTOMATIC\", \"MANAGEMENT_MANUAL\"]"]
    pub fn management(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.management", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<String>>,
}
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl { # [doc = "Set the field `cpu`.\nCPU requirement expressed in Kubernetes resource units."] pub fn set_cpu (mut self , v : impl Into < PrimField < String > >) -> Self { self . cpu = Some (v . into ()) ; self } # [doc = "Set the field `memory`.\nMemory requirement expressed in Kubernetes resource units."] pub fn set_memory (mut self , v : impl Into < PrimField < String > >) -> Self { self . memory = Some (v . into ()) ; self } }
impl ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl { type O = BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl
{}
impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl { pub fn build (self) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl { GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl { cpu : core :: default :: Default :: default () , memory : core :: default :: Default :: default () , } } }
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsElRef { fn new (shared : StackShared , base : String) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsElRef { GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsElRef { shared : shared , base : base . to_string () , } } }
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `cpu` after provisioning.\nCPU requirement expressed in Kubernetes resource units."] pub fn cpu (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.cpu" , self . base)) } # [doc = "Get a reference to the value of field `memory` after provisioning.\nMemory requirement expressed in Kubernetes resource units."] pub fn memory (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.memory" , self . base)) } }
#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<String>>,
}
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl { # [doc = "Set the field `cpu`.\nCPU requirement expressed in Kubernetes resource units."] pub fn set_cpu (mut self , v : impl Into < PrimField < String > >) -> Self { self . cpu = Some (v . into ()) ; self } # [doc = "Set the field `memory`.\nMemory requirement expressed in Kubernetes resource units."] pub fn set_memory (mut self , v : impl Into < PrimField < String > >) -> Self { self . memory = Some (v . into ()) ; self } }
impl ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl { type O = BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl
{}
impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl { pub fn build (self) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl { GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl { cpu : core :: default :: Default :: default () , memory : core :: default :: Default :: default () , } } }
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsElRef { fn new (shared : StackShared , base : String) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsElRef { GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsElRef { shared : shared , base : base . to_string () , } } }
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `cpu` after provisioning.\nCPU requirement expressed in Kubernetes resource units."] pub fn cpu (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.cpu" , self . base)) } # [doc = "Get a reference to the value of field `memory` after provisioning.\nMemory requirement expressed in Kubernetes resource units."] pub fn memory (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.memory" , self . base)) } }
#[derive(Serialize, Default)]
struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElDynamic { limits : Option < DynamicBlock < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl >> , requests : Option < DynamicBlock < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl >> , }
#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl { # [serde (skip_serializing_if = "Option::is_none")] limits : Option < Vec < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl > > , # [serde (skip_serializing_if = "Option::is_none")] requests : Option < Vec < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl > > , dynamic : GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElDynamic , }
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl { # [doc = "Set the field `limits`.\n"] pub fn set_limits (mut self , v : impl Into < BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . limits = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . limits = Some (d) ; } } self } # [doc = "Set the field `requests`.\n"] pub fn set_requests (mut self , v : impl Into < BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . requests = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . requests = Some (d) ; } } self } }
impl ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl { type O = BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl
{}
impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl { pub fn build (self) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl { GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl { limits : core :: default :: Default :: default () , requests : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRef { fn new (shared : StackShared , base : String) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRef { GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRef { shared : shared , base : base . to_string () , } } }
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `limits` after provisioning.\n"] pub fn limits (& self) -> ListRef < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.limits" , self . base)) } # [doc = "Get a reference to the value of field `requests` after provisioning.\n"] pub fn requests (& self) -> ListRef < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.requests" , self . base)) } }
#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl { # [doc = "Set the field `effect`.\nMatches a taint effect."] pub fn set_effect (mut self , v : impl Into < PrimField < String > >) -> Self { self . effect = Some (v . into ()) ; self } # [doc = "Set the field `key`.\nMatches a taint key (not necessarily unique)."] pub fn set_key (mut self , v : impl Into < PrimField < String > >) -> Self { self . key = Some (v . into ()) ; self } # [doc = "Set the field `operator`.\nMatches a taint operator."] pub fn set_operator (mut self , v : impl Into < PrimField < String > >) -> Self { self . operator = Some (v . into ()) ; self } # [doc = "Set the field `value`.\nMatches a taint value."] pub fn set_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . value = Some (v . into ()) ; self } }
impl ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl { type O = BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl
{}
impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl { pub fn build (self) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl { GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl { effect : core :: default :: Default :: default () , key : core :: default :: Default :: default () , operator : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationElRef { fn new (shared : StackShared , base : String) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationElRef { GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationElRef { shared : shared , base : base . to_string () , } } }
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `effect` after provisioning.\nMatches a taint effect."] pub fn effect (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.effect" , self . base)) } # [doc = "Get a reference to the value of field `key` after provisioning.\nMatches a taint key (not necessarily unique)."] pub fn key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.key" , self . base)) } # [doc = "Get a reference to the value of field `operator` after provisioning.\nMatches a taint operator."] pub fn operator (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.operator" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\nMatches a taint value."] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElDynamic { container_resources : Option < DynamicBlock < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl >> , pod_toleration : Option < DynamicBlock < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl >> , }
#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl { component : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] pod_affinity : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] replica_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] container_resources : Option < Vec < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl > > , # [serde (skip_serializing_if = "Option::is_none")] pod_toleration : Option < Vec < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl > > , dynamic : GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElDynamic , }
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl { # [doc = "Set the field `pod_affinity`.\nPod affinity configuration. Possible values: [\"AFFINITY_UNSPECIFIED\", \"NO_AFFINITY\", \"ANTI_AFFINITY\"]"] pub fn set_pod_affinity (mut self , v : impl Into < PrimField < String > >) -> Self { self . pod_affinity = Some (v . into ()) ; self } # [doc = "Set the field `replica_count`.\nPod replica count."] pub fn set_replica_count (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . replica_count = Some (v . into ()) ; self } # [doc = "Set the field `container_resources`.\n"] pub fn set_container_resources (mut self , v : impl Into < BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . container_resources = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . container_resources = Some (d) ; } } self } # [doc = "Set the field `pod_toleration`.\n"] pub fn set_pod_toleration (mut self , v : impl Into < BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . pod_toleration = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . pod_toleration = Some (d) ; } } self } }
impl ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl { type O = BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl
{
    #[doc = ""]
    pub component: PrimField<String>,
}
impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl { pub fn build (self) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl { GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl { component : self . component , pod_affinity : core :: default :: Default :: default () , replica_count : core :: default :: Default :: default () , container_resources : core :: default :: Default :: default () , pod_toleration : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElRef { fn new (shared : StackShared , base : String) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElRef { GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElRef { shared : shared , base : base . to_string () , } } }
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `component` after provisioning.\n"] pub fn component (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.component" , self . base)) } # [doc = "Get a reference to the value of field `pod_affinity` after provisioning.\nPod affinity configuration. Possible values: [\"AFFINITY_UNSPECIFIED\", \"NO_AFFINITY\", \"ANTI_AFFINITY\"]"] pub fn pod_affinity (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pod_affinity" , self . base)) } # [doc = "Get a reference to the value of field `replica_count` after provisioning.\nPod replica count."] pub fn replica_count (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.replica_count" , self . base)) } # [doc = "Get a reference to the value of field `container_resources` after provisioning.\n"] pub fn container_resources (& self) -> ListRef < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.container_resources" , self . base)) } # [doc = "Get a reference to the value of field `pod_toleration` after provisioning.\n"] pub fn pod_toleration (& self) -> ListRef < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.pod_toleration" , self . base)) } }
#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    backends: Option<ListField<PrimField<String>>>,
}
impl
    GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl
{
    #[doc = "Set the field `backends`.\nSpecifies the list of backends Policy Controller will export to. An empty list would effectively disable metrics export. Possible values: [\"MONITORING_BACKEND_UNSPECIFIED\", \"PROMETHEUS\", \"CLOUD_MONITORING\"]"]
    pub fn set_backends(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.backends = Some(v.into());
        self
    }
}
impl ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl { type O = BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl
{}
impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl { pub fn build (self) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl { GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl { backends : core :: default :: Default :: default () , } } }
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringElRef { fn new (shared : StackShared , base : String) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringElRef { GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringElRef { shared : shared , base : base . to_string () , } } }
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `backends` after provisioning.\nSpecifies the list of backends Policy Controller will export to. An empty list would effectively disable metrics export. Possible values: [\"MONITORING_BACKEND_UNSPECIFIED\", \"PROMETHEUS\", \"CLOUD_MONITORING\"]"] pub fn backends (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.backends" , self . base)) } }
#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl
{
    bundle: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exempted_namespaces: Option<ListField<PrimField<String>>>,
}
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl { # [doc = "Set the field `exempted_namespaces`.\nThe set of namespaces to be exempted from the bundle."] pub fn set_exempted_namespaces (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . exempted_namespaces = Some (v . into ()) ; self } }
impl ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl { type O = BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl
{
    #[doc = ""]
    pub bundle: PrimField<String>,
}
impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl { pub fn build (self) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl { GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl { bundle : self . bundle , exempted_namespaces : core :: default :: Default :: default () , } } }
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesElRef { fn new (shared : StackShared , base : String) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesElRef { GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesElRef { shared : shared , base : base . to_string () , } } }
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bundle` after provisioning.\n"] pub fn bundle (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bundle" , self . base)) } # [doc = "Get a reference to the value of field `exempted_namespaces` after provisioning.\nThe set of namespaces to be exempted from the bundle."] pub fn exempted_namespaces (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.exempted_namespaces" , self . base)) } }
#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    installation: Option<PrimField<String>>,
}
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl { # [doc = "Set the field `installation`.\nConfigures the manner in which the template library is installed on the cluster. Possible values: [\"INSTALLATION_UNSPECIFIED\", \"NOT_INSTALLED\", \"ALL\"]"] pub fn set_installation (mut self , v : impl Into < PrimField < String > >) -> Self { self . installation = Some (v . into ()) ; self } }
impl ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl { type O = BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl
{}
impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl { pub fn build (self) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl { GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl { installation : core :: default :: Default :: default () , } } }
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryElRef { fn new (shared : StackShared , base : String) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryElRef { GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryElRef { shared : shared , base : base . to_string () , } } }
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `installation` after provisioning.\nConfigures the manner in which the template library is installed on the cluster. Possible values: [\"INSTALLATION_UNSPECIFIED\", \"NOT_INSTALLED\", \"ALL\"]"] pub fn installation (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.installation" , self . base)) } }
#[derive(Serialize, Default)]
struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElDynamic { bundles : Option < DynamicBlock < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl >> , template_library : Option < DynamicBlock < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl >> , }
#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl { # [serde (skip_serializing_if = "Option::is_none")] bundles : Option < Vec < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl > > , # [serde (skip_serializing_if = "Option::is_none")] template_library : Option < Vec < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl > > , dynamic : GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElDynamic , }
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl { # [doc = "Set the field `bundles`.\n"] pub fn set_bundles (mut self , v : impl Into < BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . bundles = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . bundles = Some (d) ; } } self } # [doc = "Set the field `template_library`.\n"] pub fn set_template_library (mut self , v : impl Into < BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . template_library = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . template_library = Some (d) ; } } self } }
impl ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl { type O = BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl
{}
impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl { pub fn build (self) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl { GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl { bundles : core :: default :: Default :: default () , template_library : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElRef { fn new (shared : StackShared , base : String) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElRef { GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElRef { shared : shared , base : base . to_string () , } } }
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `template_library` after provisioning.\n"] pub fn template_library (& self) -> ListRef < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.template_library" , self . base)) } }
#[derive(Serialize, Default)]
struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDynamic { deployment_configs : Option < DynamicBlock < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl >> , monitoring : Option < DynamicBlock < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl >> , policy_content : Option < DynamicBlock < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl >> , }
#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl { # [serde (skip_serializing_if = "Option::is_none")] audit_interval_seconds : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] constraint_violation_limit : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] exemptable_namespaces : Option < ListField < PrimField < String > > > , install_spec : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] log_denies_enabled : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] mutation_enabled : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] referential_rules_enabled : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] deployment_configs : Option < Vec < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl > > , # [serde (skip_serializing_if = "Option::is_none")] monitoring : Option < Vec < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl > > , # [serde (skip_serializing_if = "Option::is_none")] policy_content : Option < Vec < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl > > , dynamic : GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDynamic , }
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl {
    #[doc = "Set the field `audit_interval_seconds`.\nInterval for Policy Controller Audit scans (in seconds). When set to 0, this disables audit functionality altogether."]
    pub fn set_audit_interval_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.audit_interval_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `constraint_violation_limit`.\nThe maximum number of audit violations to be stored in a constraint. If not set, the internal default of 20 will be used."]
    pub fn set_constraint_violation_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.constraint_violation_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `exemptable_namespaces`.\nThe set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster."]
    pub fn set_exemptable_namespaces(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exemptable_namespaces = Some(v.into());
        self
    }
    #[doc = "Set the field `log_denies_enabled`.\nLogs all denies and dry run failures."]
    pub fn set_log_denies_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.log_denies_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `mutation_enabled`.\nEnables the ability to mutate resources using Policy Controller."]
    pub fn set_mutation_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.mutation_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `referential_rules_enabled`.\nEnables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated."]
    pub fn set_referential_rules_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.referential_rules_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `deployment_configs`.\n"]
    pub fn set_deployment_configs(
        mut self,
        v : impl Into < BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.deployment_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.deployment_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `monitoring`.\n"]
    pub fn set_monitoring(
        mut self,
        v : impl Into < BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.monitoring = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.monitoring = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `policy_content`.\n"]
    pub fn set_policy_content(
        mut self,
        v : impl Into < BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.policy_content = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_content = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl
{
    type O = BlockAssignable<
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl
{
    #[doc = "Configures the mode of the Policy Controller installation Possible values: [\"INSTALL_SPEC_UNSPECIFIED\", \"INSTALL_SPEC_NOT_INSTALLED\", \"INSTALL_SPEC_ENABLED\", \"INSTALL_SPEC_SUSPENDED\", \"INSTALL_SPEC_DETACHED\"]"]
    pub install_spec: PrimField<String>,
}
impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl {
    pub fn build(
        self,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl {
            audit_interval_seconds: core::default::Default::default(),
            constraint_violation_limit: core::default::Default::default(),
            exemptable_namespaces: core::default::Default::default(),
            install_spec: self.install_spec,
            log_denies_enabled: core::default::Default::default(),
            mutation_enabled: core::default::Default::default(),
            referential_rules_enabled: core::default::Default::default(),
            deployment_configs: core::default::Default::default(),
            monitoring: core::default::Default::default(),
            policy_content: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElRef
    {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audit_interval_seconds` after provisioning.\nInterval for Policy Controller Audit scans (in seconds). When set to 0, this disables audit functionality altogether."]
    pub fn audit_interval_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.audit_interval_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `constraint_violation_limit` after provisioning.\nThe maximum number of audit violations to be stored in a constraint. If not set, the internal default of 20 will be used."]
    pub fn constraint_violation_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.constraint_violation_limit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exemptable_namespaces` after provisioning.\nThe set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster."]
    pub fn exemptable_namespaces(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exemptable_namespaces", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `install_spec` after provisioning.\nConfigures the mode of the Policy Controller installation Possible values: [\"INSTALL_SPEC_UNSPECIFIED\", \"INSTALL_SPEC_NOT_INSTALLED\", \"INSTALL_SPEC_ENABLED\", \"INSTALL_SPEC_SUSPENDED\", \"INSTALL_SPEC_DETACHED\"]"]
    pub fn install_spec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.install_spec", self.base))
    }
    #[doc = "Get a reference to the value of field `log_denies_enabled` after provisioning.\nLogs all denies and dry run failures."]
    pub fn log_denies_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_denies_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mutation_enabled` after provisioning.\nEnables the ability to mutate resources using Policy Controller."]
    pub fn mutation_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mutation_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `referential_rules_enabled` after provisioning.\nEnables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated."]
    pub fn referential_rules_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.referential_rules_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]    pub fn monitoring (& self) -> ListRef < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringElRef >{
        ListRef::new(self.shared().clone(), format!("{}.monitoring", self.base))
    }
    #[doc = "Get a reference to the value of field `policy_content` after provisioning.\n"]    pub fn policy_content (& self) -> ListRef < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.policy_content", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElDynamic {
    policy_controller_hub_config: Option<
        DynamicBlock<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_controller_hub_config: Option<
        Vec<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl>,
    >,
    dynamic: GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElDynamic,
}
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {
    #[doc = "Set the field `version`.\nConfigures the version of Policy Controller"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_controller_hub_config`.\n"]
    pub fn set_policy_controller_hub_config(
        mut self,
        v : impl Into < BlockAssignable < GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.policy_controller_hub_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_controller_hub_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {
    type O = BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {}
impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {
    pub fn build(self) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {
            version: core::default::Default::default(),
            policy_controller_hub_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElRef {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nConfigures the version of Policy Controller"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
    #[doc = "Get a reference to the value of field `policy_controller_hub_config` after provisioning.\n"]
    pub fn policy_controller_hub_config(
        &self,
    ) -> ListRef<
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.policy_controller_hub_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GkeHubFeatureFleetDefaultMemberConfigElDynamic {
    configmanagement:
        Option<DynamicBlock<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl>>,
    mesh: Option<DynamicBlock<GkeHubFeatureFleetDefaultMemberConfigElMeshEl>>,
    policycontroller:
        Option<DynamicBlock<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl>>,
}
#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    configmanagement: Option<Vec<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mesh: Option<Vec<GkeHubFeatureFleetDefaultMemberConfigElMeshEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policycontroller: Option<Vec<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl>>,
    dynamic: GkeHubFeatureFleetDefaultMemberConfigElDynamic,
}
impl GkeHubFeatureFleetDefaultMemberConfigEl {
    #[doc = "Set the field `configmanagement`.\n"]
    pub fn set_configmanagement(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.configmanagement = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.configmanagement = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mesh`.\n"]
    pub fn set_mesh(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElMeshEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mesh = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mesh = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `policycontroller`.\n"]
    pub fn set_policycontroller(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.policycontroller = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.policycontroller = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeHubFeatureFleetDefaultMemberConfigEl {
    type O = BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureFleetDefaultMemberConfigEl {}
impl BuildGkeHubFeatureFleetDefaultMemberConfigEl {
    pub fn build(self) -> GkeHubFeatureFleetDefaultMemberConfigEl {
        GkeHubFeatureFleetDefaultMemberConfigEl {
            configmanagement: core::default::Default::default(),
            mesh: core::default::Default::default(),
            policycontroller: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeHubFeatureFleetDefaultMemberConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureFleetDefaultMemberConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureFleetDefaultMemberConfigElRef {
        GkeHubFeatureFleetDefaultMemberConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureFleetDefaultMemberConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `configmanagement` after provisioning.\n"]
    pub fn configmanagement(
        &self,
    ) -> ListRef<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configmanagement", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mesh` after provisioning.\n"]
    pub fn mesh(&self) -> ListRef<GkeHubFeatureFleetDefaultMemberConfigElMeshElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mesh", self.base))
    }
    #[doc = "Get a reference to the value of field `policycontroller` after provisioning.\n"]
    pub fn policycontroller(
        &self,
    ) -> ListRef<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.policycontroller", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl {
    soaking: PrimField<String>,
}
impl GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl {}
impl ToListMappable for GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl {
    type O =
        BlockAssignable<GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl {
    #[doc = "Amount of time to \"soak\" after a rollout has been finished before marking it COMPLETE. Cannot exceed 30 days."]
    pub soaking: PrimField<String>,
}
impl BuildGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl {
    pub fn build(self) -> GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl {
        GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl {
            soaking: self.soaking,
        }
    }
}
pub struct GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsElRef {
        GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `soaking` after provisioning.\nAmount of time to \"soak\" after a rollout has been finished before marking it COMPLETE. Cannot exceed 30 days."]
    pub fn soaking(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.soaking", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl {
    name: PrimField<String>,
    version: PrimField<String>,
}
impl GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl {}
impl ToListMappable for GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl {
    type O = BlockAssignable<GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl {
    #[doc = "Name of the upgrade, e.g., \"k8s_control_plane\". It should be a valid upgrade name. It must not exceet 99 characters."]
    pub name: PrimField<String>,
    #[doc = "Version of the upgrade, e.g., \"1.22.1-gke.100\". It should be a valid version. It must not exceet 99 characters."]
    pub version: PrimField<String>,
}
impl BuildGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl {
    pub fn build(self) -> GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl {
        GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl {
            name: self.name,
            version: self.version,
        }
    }
}
pub struct GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeElRef {
        GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the upgrade, e.g., \"k8s_control_plane\". It should be a valid upgrade name. It must not exceet 99 characters."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the upgrade, e.g., \"1.22.1-gke.100\". It should be a valid version. It must not exceet 99 characters."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElDynamic {
    post_conditions: Option<
        DynamicBlock<GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl>,
    >,
    upgrade:
        Option<DynamicBlock<GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl>>,
}
#[derive(Serialize)]
pub struct GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    post_conditions:
        Option<Vec<GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upgrade: Option<Vec<GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl>>,
    dynamic: GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElDynamic,
}
impl GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl {
    #[doc = "Set the field `post_conditions`.\n"]
    pub fn set_post_conditions(
        mut self,
        v: impl Into<
            BlockAssignable<
                GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.post_conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.post_conditions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `upgrade`.\n"]
    pub fn set_upgrade(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.upgrade = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.upgrade = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl {
    type O = BlockAssignable<GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl {}
impl BuildGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl {
    pub fn build(self) -> GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl {
        GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl {
            post_conditions: core::default::Default::default(),
            upgrade: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElRef {
        GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `post_conditions` after provisioning.\n"]
    pub fn post_conditions(
        &self,
    ) -> ListRef<GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.post_conditions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `upgrade` after provisioning.\n"]
    pub fn upgrade(
        &self,
    ) -> ListRef<GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.upgrade", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeHubFeatureSpecElClusterupgradeElPostConditionsEl {
    soaking: PrimField<String>,
}
impl GkeHubFeatureSpecElClusterupgradeElPostConditionsEl {}
impl ToListMappable for GkeHubFeatureSpecElClusterupgradeElPostConditionsEl {
    type O = BlockAssignable<GkeHubFeatureSpecElClusterupgradeElPostConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureSpecElClusterupgradeElPostConditionsEl {
    #[doc = "Amount of time to \"soak\" after a rollout has been finished before marking it COMPLETE. Cannot exceed 30 days."]
    pub soaking: PrimField<String>,
}
impl BuildGkeHubFeatureSpecElClusterupgradeElPostConditionsEl {
    pub fn build(self) -> GkeHubFeatureSpecElClusterupgradeElPostConditionsEl {
        GkeHubFeatureSpecElClusterupgradeElPostConditionsEl {
            soaking: self.soaking,
        }
    }
}
pub struct GkeHubFeatureSpecElClusterupgradeElPostConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureSpecElClusterupgradeElPostConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeHubFeatureSpecElClusterupgradeElPostConditionsElRef {
        GkeHubFeatureSpecElClusterupgradeElPostConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureSpecElClusterupgradeElPostConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `soaking` after provisioning.\nAmount of time to \"soak\" after a rollout has been finished before marking it COMPLETE. Cannot exceed 30 days."]
    pub fn soaking(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.soaking", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeHubFeatureSpecElClusterupgradeElDynamic {
    gke_upgrade_overrides:
        Option<DynamicBlock<GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl>>,
    post_conditions: Option<DynamicBlock<GkeHubFeatureSpecElClusterupgradeElPostConditionsEl>>,
}
#[derive(Serialize)]
pub struct GkeHubFeatureSpecElClusterupgradeEl {
    upstream_fleets: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_upgrade_overrides: Option<Vec<GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_conditions: Option<Vec<GkeHubFeatureSpecElClusterupgradeElPostConditionsEl>>,
    dynamic: GkeHubFeatureSpecElClusterupgradeElDynamic,
}
impl GkeHubFeatureSpecElClusterupgradeEl {
    #[doc = "Set the field `gke_upgrade_overrides`.\n"]
    pub fn set_gke_upgrade_overrides(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gke_upgrade_overrides = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gke_upgrade_overrides = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `post_conditions`.\n"]
    pub fn set_post_conditions(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureSpecElClusterupgradeElPostConditionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.post_conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.post_conditions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeHubFeatureSpecElClusterupgradeEl {
    type O = BlockAssignable<GkeHubFeatureSpecElClusterupgradeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureSpecElClusterupgradeEl {
    #[doc = "Specified if other fleet should be considered as a source of upgrades. Currently, at most one upstream fleet is allowed. The fleet name should be either fleet project number or id."]
    pub upstream_fleets: ListField<PrimField<String>>,
}
impl BuildGkeHubFeatureSpecElClusterupgradeEl {
    pub fn build(self) -> GkeHubFeatureSpecElClusterupgradeEl {
        GkeHubFeatureSpecElClusterupgradeEl {
            upstream_fleets: self.upstream_fleets,
            gke_upgrade_overrides: core::default::Default::default(),
            post_conditions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeHubFeatureSpecElClusterupgradeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureSpecElClusterupgradeElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureSpecElClusterupgradeElRef {
        GkeHubFeatureSpecElClusterupgradeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureSpecElClusterupgradeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `upstream_fleets` after provisioning.\nSpecified if other fleet should be considered as a source of upgrades. Currently, at most one upstream fleet is allowed. The fleet name should be either fleet project number or id."]
    pub fn upstream_fleets(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upstream_fleets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gke_upgrade_overrides` after provisioning.\n"]
    pub fn gke_upgrade_overrides(
        &self,
    ) -> ListRef<GkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gke_upgrade_overrides", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `post_conditions` after provisioning.\n"]
    pub fn post_conditions(
        &self,
    ) -> ListRef<GkeHubFeatureSpecElClusterupgradeElPostConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.post_conditions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {
    #[doc = "Set the field `mode`.\nSpecified if fleet logging feature is enabled. Possible values: [\"MODE_UNSPECIFIED\", \"COPY\", \"MOVE\"]"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable for GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {
    type O = BlockAssignable<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {}
impl BuildGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {
    pub fn build(self) -> GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {
        GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {
            mode: core::default::Default::default(),
        }
    }
}
pub struct GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigElRef {
        GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nSpecified if fleet logging feature is enabled. Possible values: [\"MODE_UNSPECIFIED\", \"COPY\", \"MOVE\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {
    #[doc = "Set the field `mode`.\nSpecified if fleet logging feature is enabled. Possible values: [\"MODE_UNSPECIFIED\", \"COPY\", \"MOVE\"]"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable
    for GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl
{
    type O = BlockAssignable<
        GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {}
impl BuildGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {
    pub fn build(
        self,
    ) -> GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {
        GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {
            mode: core::default::Default::default(),
        }
    }
}
pub struct GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigElRef {
        GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nSpecified if fleet logging feature is enabled. Possible values: [\"MODE_UNSPECIFIED\", \"COPY\", \"MOVE\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDynamic {
    default_config:
        Option<DynamicBlock<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl>>,
    fleet_scope_logs_config: Option<
        DynamicBlock<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct GkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_config:
        Option<Vec<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet_scope_logs_config:
        Option<Vec<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl>>,
    dynamic: GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDynamic,
}
impl GkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {
    #[doc = "Set the field `default_config`.\n"]
    pub fn set_default_config(
        mut self,
        v: impl Into<
            BlockAssignable<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `fleet_scope_logs_config`.\n"]
    pub fn set_fleet_scope_logs_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fleet_scope_logs_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fleet_scope_logs_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {
    type O = BlockAssignable<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {}
impl BuildGkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {
    pub fn build(self) -> GkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {
        GkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {
            default_config: core::default::Default::default(),
            fleet_scope_logs_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElRef {
        GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_config` after provisioning.\n"]
    pub fn default_config(
        &self,
    ) -> ListRef<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_scope_logs_config` after provisioning.\n"]
    pub fn fleet_scope_logs_config(
        &self,
    ) -> ListRef<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet_scope_logs_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GkeHubFeatureSpecElFleetobservabilityElDynamic {
    logging_config: Option<DynamicBlock<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl>>,
}
#[derive(Serialize)]
pub struct GkeHubFeatureSpecElFleetobservabilityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_config: Option<Vec<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl>>,
    dynamic: GkeHubFeatureSpecElFleetobservabilityElDynamic,
}
impl GkeHubFeatureSpecElFleetobservabilityEl {
    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.logging_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeHubFeatureSpecElFleetobservabilityEl {
    type O = BlockAssignable<GkeHubFeatureSpecElFleetobservabilityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureSpecElFleetobservabilityEl {}
impl BuildGkeHubFeatureSpecElFleetobservabilityEl {
    pub fn build(self) -> GkeHubFeatureSpecElFleetobservabilityEl {
        GkeHubFeatureSpecElFleetobservabilityEl {
            logging_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeHubFeatureSpecElFleetobservabilityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureSpecElFleetobservabilityElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureSpecElFleetobservabilityElRef {
        GkeHubFeatureSpecElFleetobservabilityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureSpecElFleetobservabilityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(
        &self,
    ) -> ListRef<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeHubFeatureSpecElMulticlusteringressEl {
    config_membership: PrimField<String>,
}
impl GkeHubFeatureSpecElMulticlusteringressEl {}
impl ToListMappable for GkeHubFeatureSpecElMulticlusteringressEl {
    type O = BlockAssignable<GkeHubFeatureSpecElMulticlusteringressEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureSpecElMulticlusteringressEl {
    #[doc = "Fully-qualified Membership name which hosts the MultiClusterIngress CRD. Example: 'projects/foo-proj/locations/global/memberships/bar'"]
    pub config_membership: PrimField<String>,
}
impl BuildGkeHubFeatureSpecElMulticlusteringressEl {
    pub fn build(self) -> GkeHubFeatureSpecElMulticlusteringressEl {
        GkeHubFeatureSpecElMulticlusteringressEl {
            config_membership: self.config_membership,
        }
    }
}
pub struct GkeHubFeatureSpecElMulticlusteringressElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureSpecElMulticlusteringressElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureSpecElMulticlusteringressElRef {
        GkeHubFeatureSpecElMulticlusteringressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureSpecElMulticlusteringressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `config_membership` after provisioning.\nFully-qualified Membership name which hosts the MultiClusterIngress CRD. Example: 'projects/foo-proj/locations/global/memberships/bar'"]
    pub fn config_membership(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_membership", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeHubFeatureSpecElRbacrolebindingactuationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_custom_roles: Option<ListField<PrimField<String>>>,
}
impl GkeHubFeatureSpecElRbacrolebindingactuationEl {
    #[doc = "Set the field `allowed_custom_roles`.\nThe list of allowed custom roles (ClusterRoles). If a custom role is not part of this list, it cannot be used in a fleet scope RBACRoleBinding. If a custom role in this list is in use, it cannot be removed from the list until the scope RBACRolebindings using it are deleted."]
    pub fn set_allowed_custom_roles(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_custom_roles = Some(v.into());
        self
    }
}
impl ToListMappable for GkeHubFeatureSpecElRbacrolebindingactuationEl {
    type O = BlockAssignable<GkeHubFeatureSpecElRbacrolebindingactuationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureSpecElRbacrolebindingactuationEl {}
impl BuildGkeHubFeatureSpecElRbacrolebindingactuationEl {
    pub fn build(self) -> GkeHubFeatureSpecElRbacrolebindingactuationEl {
        GkeHubFeatureSpecElRbacrolebindingactuationEl {
            allowed_custom_roles: core::default::Default::default(),
        }
    }
}
pub struct GkeHubFeatureSpecElRbacrolebindingactuationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureSpecElRbacrolebindingactuationElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureSpecElRbacrolebindingactuationElRef {
        GkeHubFeatureSpecElRbacrolebindingactuationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureSpecElRbacrolebindingactuationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_custom_roles` after provisioning.\nThe list of allowed custom roles (ClusterRoles). If a custom role is not part of this list, it cannot be used in a fleet scope RBACRoleBinding. If a custom role in this list is in use, it cannot be removed from the list until the scope RBACRolebindings using it are deleted."]
    pub fn allowed_custom_roles(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_custom_roles", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeHubFeatureSpecElWorkloadidentityEl {
    scope_tenancy_pool: PrimField<String>,
}
impl GkeHubFeatureSpecElWorkloadidentityEl {}
impl ToListMappable for GkeHubFeatureSpecElWorkloadidentityEl {
    type O = BlockAssignable<GkeHubFeatureSpecElWorkloadidentityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureSpecElWorkloadidentityEl {
    #[doc = "Pool to be used for Workload Identity. This pool in trust-domain mode is used with Fleet Tenancy, so that sameness can be enforced. ex: projects/example/locations/global/workloadidentitypools/custompool"]
    pub scope_tenancy_pool: PrimField<String>,
}
impl BuildGkeHubFeatureSpecElWorkloadidentityEl {
    pub fn build(self) -> GkeHubFeatureSpecElWorkloadidentityEl {
        GkeHubFeatureSpecElWorkloadidentityEl {
            scope_tenancy_pool: self.scope_tenancy_pool,
        }
    }
}
pub struct GkeHubFeatureSpecElWorkloadidentityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureSpecElWorkloadidentityElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureSpecElWorkloadidentityElRef {
        GkeHubFeatureSpecElWorkloadidentityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureSpecElWorkloadidentityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scope_tenancy_pool` after provisioning.\nPool to be used for Workload Identity. This pool in trust-domain mode is used with Fleet Tenancy, so that sameness can be enforced. ex: projects/example/locations/global/workloadidentitypools/custompool"]
    pub fn scope_tenancy_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scope_tenancy_pool", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GkeHubFeatureSpecElDynamic {
    clusterupgrade: Option<DynamicBlock<GkeHubFeatureSpecElClusterupgradeEl>>,
    fleetobservability: Option<DynamicBlock<GkeHubFeatureSpecElFleetobservabilityEl>>,
    multiclusteringress: Option<DynamicBlock<GkeHubFeatureSpecElMulticlusteringressEl>>,
    rbacrolebindingactuation: Option<DynamicBlock<GkeHubFeatureSpecElRbacrolebindingactuationEl>>,
    workloadidentity: Option<DynamicBlock<GkeHubFeatureSpecElWorkloadidentityEl>>,
}
#[derive(Serialize)]
pub struct GkeHubFeatureSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    clusterupgrade: Option<Vec<GkeHubFeatureSpecElClusterupgradeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleetobservability: Option<Vec<GkeHubFeatureSpecElFleetobservabilityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiclusteringress: Option<Vec<GkeHubFeatureSpecElMulticlusteringressEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rbacrolebindingactuation: Option<Vec<GkeHubFeatureSpecElRbacrolebindingactuationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workloadidentity: Option<Vec<GkeHubFeatureSpecElWorkloadidentityEl>>,
    dynamic: GkeHubFeatureSpecElDynamic,
}
impl GkeHubFeatureSpecEl {
    #[doc = "Set the field `clusterupgrade`.\n"]
    pub fn set_clusterupgrade(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureSpecElClusterupgradeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.clusterupgrade = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.clusterupgrade = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `fleetobservability`.\n"]
    pub fn set_fleetobservability(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureSpecElFleetobservabilityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fleetobservability = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fleetobservability = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `multiclusteringress`.\n"]
    pub fn set_multiclusteringress(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureSpecElMulticlusteringressEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.multiclusteringress = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.multiclusteringress = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rbacrolebindingactuation`.\n"]
    pub fn set_rbacrolebindingactuation(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureSpecElRbacrolebindingactuationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rbacrolebindingactuation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rbacrolebindingactuation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `workloadidentity`.\n"]
    pub fn set_workloadidentity(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureSpecElWorkloadidentityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.workloadidentity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.workloadidentity = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeHubFeatureSpecEl {
    type O = BlockAssignable<GkeHubFeatureSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureSpecEl {}
impl BuildGkeHubFeatureSpecEl {
    pub fn build(self) -> GkeHubFeatureSpecEl {
        GkeHubFeatureSpecEl {
            clusterupgrade: core::default::Default::default(),
            fleetobservability: core::default::Default::default(),
            multiclusteringress: core::default::Default::default(),
            rbacrolebindingactuation: core::default::Default::default(),
            workloadidentity: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeHubFeatureSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureSpecElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureSpecElRef {
        GkeHubFeatureSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `clusterupgrade` after provisioning.\n"]
    pub fn clusterupgrade(&self) -> ListRef<GkeHubFeatureSpecElClusterupgradeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.clusterupgrade", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fleetobservability` after provisioning.\n"]
    pub fn fleetobservability(&self) -> ListRef<GkeHubFeatureSpecElFleetobservabilityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleetobservability", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `multiclusteringress` after provisioning.\n"]
    pub fn multiclusteringress(&self) -> ListRef<GkeHubFeatureSpecElMulticlusteringressElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multiclusteringress", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rbacrolebindingactuation` after provisioning.\n"]
    pub fn rbacrolebindingactuation(
        &self,
    ) -> ListRef<GkeHubFeatureSpecElRbacrolebindingactuationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rbacrolebindingactuation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `workloadidentity` after provisioning.\n"]
    pub fn workloadidentity(&self) -> ListRef<GkeHubFeatureSpecElWorkloadidentityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workloadidentity", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeHubFeatureTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl GkeHubFeatureTimeoutsEl {
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
impl ToListMappable for GkeHubFeatureTimeoutsEl {
    type O = BlockAssignable<GkeHubFeatureTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubFeatureTimeoutsEl {}
impl BuildGkeHubFeatureTimeoutsEl {
    pub fn build(self) -> GkeHubFeatureTimeoutsEl {
        GkeHubFeatureTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct GkeHubFeatureTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubFeatureTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureTimeoutsElRef {
        GkeHubFeatureTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubFeatureTimeoutsElRef {
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
struct GkeHubFeatureDynamic {
    fleet_default_member_config: Option<DynamicBlock<GkeHubFeatureFleetDefaultMemberConfigEl>>,
    spec: Option<DynamicBlock<GkeHubFeatureSpecEl>>,
}
