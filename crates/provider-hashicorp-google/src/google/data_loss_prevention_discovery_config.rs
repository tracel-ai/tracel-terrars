use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataLossPreventionDiscoveryConfigData {
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
    inspect_templates: Option<ListField<PrimField<String>>>,
    location: PrimField<String>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<Vec<DataLossPreventionDiscoveryConfigActionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org_config: Option<Vec<DataLossPreventionDiscoveryConfigOrgConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    other_cloud_starting_location:
        Option<Vec<DataLossPreventionDiscoveryConfigOtherCloudStartingLocationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    targets: Option<Vec<DataLossPreventionDiscoveryConfigTargetsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataLossPreventionDiscoveryConfigTimeoutsEl>,
    dynamic: DataLossPreventionDiscoveryConfigDynamic,
}
struct DataLossPreventionDiscoveryConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataLossPreventionDiscoveryConfigData>,
}
#[derive(Clone)]
pub struct DataLossPreventionDiscoveryConfig(Rc<DataLossPreventionDiscoveryConfig_>);
impl DataLossPreventionDiscoveryConfig {
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
    #[doc = "Set the field `display_name`.\nDisplay Name (max 1000 Chars)"]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `inspect_templates`.\nDetection logic for profile generation"]
    pub fn set_inspect_templates(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().inspect_templates = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\nRequired. A status for this configuration Possible values: [\"RUNNING\", \"PAUSED\"]"]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
        self
    }
    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(
        self,
        v: impl Into<BlockAssignable<DataLossPreventionDiscoveryConfigActionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.actions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `org_config`.\n"]
    pub fn set_org_config(
        self,
        v: impl Into<BlockAssignable<DataLossPreventionDiscoveryConfigOrgConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().org_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.org_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `other_cloud_starting_location`.\n"]
    pub fn set_other_cloud_starting_location(
        self,
        v: impl Into<BlockAssignable<DataLossPreventionDiscoveryConfigOtherCloudStartingLocationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().other_cloud_starting_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .other_cloud_starting_location = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `targets`.\n"]
    pub fn set_targets(
        self,
        v: impl Into<BlockAssignable<DataLossPreventionDiscoveryConfigTargetsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().targets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.targets = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataLossPreventionDiscoveryConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation timestamp of a DiscoveryConfig."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay Name (max 1000 Chars)"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `errors` after provisioning.\nOutput only. A stream of errors encountered when the config was activated. Repeated errors may result in the config automatically being paused. Output only field. Will return the last 100 errors. Whenever the config is modified this list will be cleared."]
    pub fn errors(&self) -> ListRef<DataLossPreventionDiscoveryConfigErrorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.errors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `inspect_templates` after provisioning.\nDetection logic for profile generation"]
    pub fn inspect_templates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inspect_templates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_run_time` after provisioning.\nOutput only. The timestamp of the last time this config was executed"]
    pub fn last_run_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_run_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation to create the discovery config in."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique resource name for the DiscoveryConfig, assigned by the service when the DiscoveryConfig is created."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the discovery config in any of the following formats:\n\n* 'projects/{{project}}/locations/{{location}}'\n* 'organizations/{{organization_id}}/locations/{{location}}'"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nRequired. A status for this configuration Possible values: [\"RUNNING\", \"PAUSED\"]"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last update timestamp of a DiscoveryConfig."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> ListRef<DataLossPreventionDiscoveryConfigActionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_config` after provisioning.\n"]
    pub fn org_config(&self) -> ListRef<DataLossPreventionDiscoveryConfigOrgConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.org_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `other_cloud_starting_location` after provisioning.\n"]
    pub fn other_cloud_starting_location(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.other_cloud_starting_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.targets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionDiscoveryConfigTimeoutsElRef {
        DataLossPreventionDiscoveryConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataLossPreventionDiscoveryConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataLossPreventionDiscoveryConfig {}
impl ToListMappable for DataLossPreventionDiscoveryConfig {
    type O = ListRef<DataLossPreventionDiscoveryConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataLossPreventionDiscoveryConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_data_loss_prevention_discovery_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataLossPreventionDiscoveryConfig {
    pub tf_id: String,
    #[doc = "Location to create the discovery config in."]
    pub location: PrimField<String>,
    #[doc = "The parent of the discovery config in any of the following formats:\n\n* 'projects/{{project}}/locations/{{location}}'\n* 'organizations/{{organization_id}}/locations/{{location}}'"]
    pub parent: PrimField<String>,
}
impl BuildDataLossPreventionDiscoveryConfig {
    pub fn build(self, stack: &mut Stack) -> DataLossPreventionDiscoveryConfig {
        let out = DataLossPreventionDiscoveryConfig(Rc::new(DataLossPreventionDiscoveryConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataLossPreventionDiscoveryConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                inspect_templates: core::default::Default::default(),
                location: self.location,
                parent: self.parent,
                status: core::default::Default::default(),
                actions: core::default::Default::default(),
                org_config: core::default::Default::default(),
                other_cloud_starting_location: core::default::Default::default(),
                targets: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataLossPreventionDiscoveryConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataLossPreventionDiscoveryConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation timestamp of a DiscoveryConfig."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay Name (max 1000 Chars)"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `errors` after provisioning.\nOutput only. A stream of errors encountered when the config was activated. Repeated errors may result in the config automatically being paused. Output only field. Will return the last 100 errors. Whenever the config is modified this list will be cleared."]
    pub fn errors(&self) -> ListRef<DataLossPreventionDiscoveryConfigErrorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.errors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `inspect_templates` after provisioning.\nDetection logic for profile generation"]
    pub fn inspect_templates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inspect_templates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_run_time` after provisioning.\nOutput only. The timestamp of the last time this config was executed"]
    pub fn last_run_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_run_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation to create the discovery config in."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique resource name for the DiscoveryConfig, assigned by the service when the DiscoveryConfig is created."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the discovery config in any of the following formats:\n\n* 'projects/{{project}}/locations/{{location}}'\n* 'organizations/{{organization_id}}/locations/{{location}}'"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nRequired. A status for this configuration Possible values: [\"RUNNING\", \"PAUSED\"]"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last update timestamp of a DiscoveryConfig."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> ListRef<DataLossPreventionDiscoveryConfigActionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_config` after provisioning.\n"]
    pub fn org_config(&self) -> ListRef<DataLossPreventionDiscoveryConfigOrgConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.org_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `other_cloud_starting_location` after provisioning.\n"]
    pub fn other_cloud_starting_location(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.other_cloud_starting_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.targets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionDiscoveryConfigTimeoutsElRef {
        DataLossPreventionDiscoveryConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigErrorsElDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<ListField<RecField<PrimField<String>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigErrorsElDetailsEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl Into<ListField<RecField<PrimField<String>>>>) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigErrorsElDetailsEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigErrorsElDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigErrorsElDetailsEl {}
impl BuildDataLossPreventionDiscoveryConfigErrorsElDetailsEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigErrorsElDetailsEl {
        DataLossPreventionDiscoveryConfigErrorsElDetailsEl {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigErrorsElDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigErrorsElDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigErrorsElDetailsElRef {
        DataLossPreventionDiscoveryConfigErrorsElDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigErrorsElDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> ListRef<RecRef<PrimExpr<String>>> {
        ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigErrorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<ListField<DataLossPreventionDiscoveryConfigErrorsElDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigErrorsEl {
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v: impl Into<ListField<DataLossPreventionDiscoveryConfigErrorsElDetailsEl>>,
    ) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `timestamp`.\n"]
    pub fn set_timestamp(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timestamp = Some(v.into());
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigErrorsEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigErrorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigErrorsEl {}
impl BuildDataLossPreventionDiscoveryConfigErrorsEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigErrorsEl {
        DataLossPreventionDiscoveryConfigErrorsEl {
            details: core::default::Default::default(),
            timestamp: core::default::Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigErrorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigErrorsElRef {
    fn new(shared: StackShared, base: String) -> DataLossPreventionDiscoveryConfigErrorsElRef {
        DataLossPreventionDiscoveryConfigErrorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigErrorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> ListRef<DataLossPreventionDiscoveryConfigErrorsElDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `timestamp` after provisioning.\n"]
    pub fn timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timestamp", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigActionsElExportDataElProfileTableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_id: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigActionsElExportDataElProfileTableEl {
    #[doc = "Set the field `dataset_id`.\nDataset Id of the table"]
    pub fn set_dataset_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataset_id = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\nThe Google Cloud Platform project ID of the project containing the table. If omitted, the project ID is inferred from the API call."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `table_id`.\nName of the table"]
    pub fn set_table_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigActionsElExportDataElProfileTableEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigActionsElExportDataElProfileTableEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigActionsElExportDataElProfileTableEl {}
impl BuildDataLossPreventionDiscoveryConfigActionsElExportDataElProfileTableEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigActionsElExportDataElProfileTableEl {
        DataLossPreventionDiscoveryConfigActionsElExportDataElProfileTableEl {
            dataset_id: core::default::Default::default(),
            project_id: core::default::Default::default(),
            table_id: core::default::Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigActionsElExportDataElProfileTableElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigActionsElExportDataElProfileTableElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigActionsElExportDataElProfileTableElRef {
        DataLossPreventionDiscoveryConfigActionsElExportDataElProfileTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigActionsElExportDataElProfileTableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nDataset Id of the table"]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe Google Cloud Platform project ID of the project containing the table. If omitted, the project ID is inferred from the API call."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nName of the table"]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigActionsElExportDataElSampleFindingsTableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_id: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigActionsElExportDataElSampleFindingsTableEl {
    #[doc = "Set the field `dataset_id`.\nDataset Id of the table"]
    pub fn set_dataset_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataset_id = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\nThe Google Cloud Platform project ID of the project containing the table. If omitted, the project ID is inferred from the API call."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `table_id`.\nName of the table"]
    pub fn set_table_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table_id = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigActionsElExportDataElSampleFindingsTableEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigActionsElExportDataElSampleFindingsTableEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigActionsElExportDataElSampleFindingsTableEl {}
impl BuildDataLossPreventionDiscoveryConfigActionsElExportDataElSampleFindingsTableEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigActionsElExportDataElSampleFindingsTableEl {
        DataLossPreventionDiscoveryConfigActionsElExportDataElSampleFindingsTableEl {
            dataset_id: core::default::Default::default(),
            project_id: core::default::Default::default(),
            table_id: core::default::Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigActionsElExportDataElSampleFindingsTableElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigActionsElExportDataElSampleFindingsTableElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigActionsElExportDataElSampleFindingsTableElRef {
        DataLossPreventionDiscoveryConfigActionsElExportDataElSampleFindingsTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigActionsElExportDataElSampleFindingsTableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nDataset Id of the table"]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe Google Cloud Platform project ID of the project containing the table. If omitted, the project ID is inferred from the API call."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nName of the table"]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigActionsElExportDataElDynamic {
    profile_table:
        Option<DynamicBlock<DataLossPreventionDiscoveryConfigActionsElExportDataElProfileTableEl>>,
    sample_findings_table: Option<
        DynamicBlock<DataLossPreventionDiscoveryConfigActionsElExportDataElSampleFindingsTableEl>,
    >,
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigActionsElExportDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    profile_table:
        Option<Vec<DataLossPreventionDiscoveryConfigActionsElExportDataElProfileTableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_findings_table:
        Option<Vec<DataLossPreventionDiscoveryConfigActionsElExportDataElSampleFindingsTableEl>>,
    dynamic: DataLossPreventionDiscoveryConfigActionsElExportDataElDynamic,
}
impl DataLossPreventionDiscoveryConfigActionsElExportDataEl {
    #[doc = "Set the field `profile_table`.\n"]
    pub fn set_profile_table(
        mut self,
        v: impl Into<
            BlockAssignable<DataLossPreventionDiscoveryConfigActionsElExportDataElProfileTableEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.profile_table = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.profile_table = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sample_findings_table`.\n"]
    pub fn set_sample_findings_table(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigActionsElExportDataElSampleFindingsTableEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sample_findings_table = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sample_findings_table = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigActionsElExportDataEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigActionsElExportDataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigActionsElExportDataEl {}
impl BuildDataLossPreventionDiscoveryConfigActionsElExportDataEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigActionsElExportDataEl {
        DataLossPreventionDiscoveryConfigActionsElExportDataEl {
            profile_table: core::default::Default::default(),
            sample_findings_table: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigActionsElExportDataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigActionsElExportDataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigActionsElExportDataElRef {
        DataLossPreventionDiscoveryConfigActionsElExportDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigActionsElExportDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `profile_table` after provisioning.\n"]
    pub fn profile_table(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigActionsElExportDataElProfileTableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.profile_table", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sample_findings_table` after provisioning.\n"]
    pub fn sample_findings_table(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigActionsElExportDataElSampleFindingsTableElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sample_findings_table", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElConditionsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_risk_score: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_sensitivity_score: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElConditionsEl { # [doc = "Set the field `minimum_risk_score`.\nThe minimum data risk score that triggers the condition. Possible values: [\"HIGH\", \"MEDIUM_OR_HIGH\"]"] pub fn set_minimum_risk_score (mut self , v : impl Into < PrimField < String > >) -> Self { self . minimum_risk_score = Some (v . into ()) ; self } # [doc = "Set the field `minimum_sensitivity_score`.\nThe minimum sensitivity level that triggers the condition. Possible values: [\"HIGH\", \"MEDIUM_OR_HIGH\"]"] pub fn set_minimum_sensitivity_score (mut self , v : impl Into < PrimField < String > >) -> Self { self . minimum_sensitivity_score = Some (v . into ()) ; self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElConditionsEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElConditionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElConditionsEl
{}
impl BuildDataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElConditionsEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElConditionsEl { DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElConditionsEl { minimum_risk_score : core :: default :: Default :: default () , minimum_sensitivity_score : core :: default :: Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElConditionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElConditionsElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElConditionsElRef { DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElConditionsElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElConditionsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `minimum_risk_score` after provisioning.\nThe minimum data risk score that triggers the condition. Possible values: [\"HIGH\", \"MEDIUM_OR_HIGH\"]"] pub fn minimum_risk_score (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.minimum_risk_score" , self . base)) } # [doc = "Get a reference to the value of field `minimum_sensitivity_score` after provisioning.\nThe minimum sensitivity level that triggers the condition. Possible values: [\"HIGH\", \"MEDIUM_OR_HIGH\"]"] pub fn minimum_sensitivity_score (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.minimum_sensitivity_score" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElDynamic { conditions : Option < DynamicBlock < DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElConditionsEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsEl { # [serde (skip_serializing_if = "Option::is_none")] logical_operator : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] conditions : Option < Vec < DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElConditionsEl > > , dynamic : DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElDynamic , }
impl DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsEl {
    #[doc = "Set the field `logical_operator`.\nThe operator to apply to the collection of conditions Possible values: [\"OR\", \"AND\"]"]
    pub fn set_logical_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logical_operator = Some(v.into());
        self
    }
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElConditionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsEl
{
    type O = BlockAssignable < DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsEl
{}
impl BuildDataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsEl { DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsEl { logical_operator : core :: default :: Default :: default () , conditions : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElRef { DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElRef { shared : shared , base : base . to_string () , } } }
impl
    DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `logical_operator` after provisioning.\nThe operator to apply to the collection of conditions Possible values: [\"OR\", \"AND\"]"]
    pub fn logical_operator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logical_operator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]    pub fn conditions (& self) -> ListRef < DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElConditionsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElDynamic { expressions : Option < DynamicBlock < DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionEl { # [serde (skip_serializing_if = "Option::is_none")] expressions : Option < Vec < DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsEl > > , dynamic : DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElDynamic , }
impl DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionEl {
    #[doc = "Set the field `expressions`.\n"]
    pub fn set_expressions(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.expressions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.expressions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionEl {}
impl BuildDataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionEl {
        DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionEl {
            expressions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElRef {
        DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expressions` after provisioning.\n"]    pub fn expressions (& self) -> ListRef < DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElExpressionsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.expressions", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElDynamic {
    pubsub_condition: Option<
        DynamicBlock<
            DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigActionsElPubSubNotificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    detail_of_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pubsub_condition: Option<
        Vec<DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionEl>,
    >,
    dynamic: DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElDynamic,
}
impl DataLossPreventionDiscoveryConfigActionsElPubSubNotificationEl {
    #[doc = "Set the field `detail_of_message`.\nHow much data to include in the pub/sub message. Possible values: [\"TABLE_PROFILE\", \"RESOURCE_NAME\"]"]
    pub fn set_detail_of_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.detail_of_message = Some(v.into());
        self
    }
    #[doc = "Set the field `event`.\nThe type of event that triggers a Pub/Sub. At most one PubSubNotification per EventType is permitted. Possible values: [\"NEW_PROFILE\", \"CHANGED_PROFILE\", \"SCORE_INCREASED\", \"ERROR_CHANGED\"]"]
    pub fn set_event(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event = Some(v.into());
        self
    }
    #[doc = "Set the field `topic`.\nCloud Pub/Sub topic to send notifications to. Format is projects/{project}/topics/{topic}."]
    pub fn set_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic = Some(v.into());
        self
    }
    #[doc = "Set the field `pubsub_condition`.\n"]
    pub fn set_pubsub_condition(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pubsub_condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pubsub_condition = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigActionsElPubSubNotificationEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigActionsElPubSubNotificationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigActionsElPubSubNotificationEl {}
impl BuildDataLossPreventionDiscoveryConfigActionsElPubSubNotificationEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigActionsElPubSubNotificationEl {
        DataLossPreventionDiscoveryConfigActionsElPubSubNotificationEl {
            detail_of_message: core::default::Default::default(),
            event: core::default::Default::default(),
            topic: core::default::Default::default(),
            pubsub_condition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElRef {
        DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `detail_of_message` after provisioning.\nHow much data to include in the pub/sub message. Possible values: [\"TABLE_PROFILE\", \"RESOURCE_NAME\"]"]
    pub fn detail_of_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.detail_of_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `event` after provisioning.\nThe type of event that triggers a Pub/Sub. At most one PubSubNotification per EventType is permitted. Possible values: [\"NEW_PROFILE\", \"CHANGED_PROFILE\", \"SCORE_INCREASED\", \"ERROR_CHANGED\"]"]
    pub fn event(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event", self.base))
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nCloud Pub/Sub topic to send notifications to. Format is projects/{project}/topics/{topic}."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
    #[doc = "Get a reference to the value of field `pubsub_condition` after provisioning.\n"]
    pub fn pubsub_condition(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElPubsubConditionElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pubsub_condition", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigActionsElPublishToChronicleEl {}
impl DataLossPreventionDiscoveryConfigActionsElPublishToChronicleEl {}
impl ToListMappable for DataLossPreventionDiscoveryConfigActionsElPublishToChronicleEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigActionsElPublishToChronicleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigActionsElPublishToChronicleEl {}
impl BuildDataLossPreventionDiscoveryConfigActionsElPublishToChronicleEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigActionsElPublishToChronicleEl {
        DataLossPreventionDiscoveryConfigActionsElPublishToChronicleEl {}
    }
}
pub struct DataLossPreventionDiscoveryConfigActionsElPublishToChronicleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigActionsElPublishToChronicleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigActionsElPublishToChronicleElRef {
        DataLossPreventionDiscoveryConfigActionsElPublishToChronicleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigActionsElPublishToChronicleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigActionsElPublishToDataplexCatalogEl {}
impl DataLossPreventionDiscoveryConfigActionsElPublishToDataplexCatalogEl {}
impl ToListMappable for DataLossPreventionDiscoveryConfigActionsElPublishToDataplexCatalogEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigActionsElPublishToDataplexCatalogEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigActionsElPublishToDataplexCatalogEl {}
impl BuildDataLossPreventionDiscoveryConfigActionsElPublishToDataplexCatalogEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigActionsElPublishToDataplexCatalogEl {
        DataLossPreventionDiscoveryConfigActionsElPublishToDataplexCatalogEl {}
    }
}
pub struct DataLossPreventionDiscoveryConfigActionsElPublishToDataplexCatalogElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigActionsElPublishToDataplexCatalogElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigActionsElPublishToDataplexCatalogElRef {
        DataLossPreventionDiscoveryConfigActionsElPublishToDataplexCatalogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigActionsElPublishToDataplexCatalogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigActionsElPublishToSccEl {}
impl DataLossPreventionDiscoveryConfigActionsElPublishToSccEl {}
impl ToListMappable for DataLossPreventionDiscoveryConfigActionsElPublishToSccEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigActionsElPublishToSccEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigActionsElPublishToSccEl {}
impl BuildDataLossPreventionDiscoveryConfigActionsElPublishToSccEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigActionsElPublishToSccEl {
        DataLossPreventionDiscoveryConfigActionsElPublishToSccEl {}
    }
}
pub struct DataLossPreventionDiscoveryConfigActionsElPublishToSccElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigActionsElPublishToSccElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigActionsElPublishToSccElRef {
        DataLossPreventionDiscoveryConfigActionsElPublishToSccElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigActionsElPublishToSccElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElSensitivityScoreEl
{
    score: PrimField<String>,
}
impl DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElSensitivityScoreEl {}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElSensitivityScoreEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElSensitivityScoreEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElSensitivityScoreEl
{
    #[doc = "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\", \"SENSITIVITY_UNKNOWN\"]"]
    pub score: PrimField<String>,
}
impl
    BuildDataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElSensitivityScoreEl
{
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElSensitivityScoreEl
    {
        DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElSensitivityScoreEl {
            score: self.score,
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElSensitivityScoreElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElSensitivityScoreElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElSensitivityScoreElRef
    {
        DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElSensitivityScoreElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElSensitivityScoreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `score` after provisioning.\nThe sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\", \"SENSITIVITY_UNKNOWN\"]"]
    pub fn score(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.score", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElTagEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    namespaced_value: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElTagEl {
    #[doc = "Set the field `namespaced_value`.\nThe namespaced name for the tag value to attach to resources. Must be in the format '{parent_id}/{tag_key_short_name}/{short_name}', for example, \"123456/environment/prod\"."]
    pub fn set_namespaced_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespaced_value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElTagEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElTagEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElTagEl {}
impl BuildDataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElTagEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElTagEl {
        DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElTagEl {
            namespaced_value: core::default::Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElTagElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElTagElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElTagElRef {
        DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElTagElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `namespaced_value` after provisioning.\nThe namespaced name for the tag value to attach to resources. Must be in the format '{parent_id}/{tag_key_short_name}/{short_name}', for example, \"123456/environment/prod\"."]
    pub fn namespaced_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespaced_value", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElDynamic { sensitivity_score : Option < DynamicBlock < DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElSensitivityScoreEl >> , tag : Option < DynamicBlock < DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElTagEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsEl { # [serde (skip_serializing_if = "Option::is_none")] sensitivity_score : Option < Vec < DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElSensitivityScoreEl > > , # [serde (skip_serializing_if = "Option::is_none")] tag : Option < Vec < DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElTagEl > > , dynamic : DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElDynamic , }
impl DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsEl {
    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElSensitivityScoreEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sensitivity_score = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElTagEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsEl {
    type O =
        BlockAssignable<DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsEl {}
impl BuildDataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsEl {
        DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsEl {
            sensitivity_score: core::default::Default::default(),
            tag: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElRef {
        DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]    pub fn sensitivity_score (& self) -> ListRef < DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElSensitivityScoreElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.sensitivity_score", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElTagElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigActionsElTagResourcesElDynamic {
    tag_conditions: Option<
        DynamicBlock<DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsEl>,
    >,
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigActionsElTagResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lower_data_risk_to_low: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile_generations_to_tag: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_conditions:
        Option<Vec<DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsEl>>,
    dynamic: DataLossPreventionDiscoveryConfigActionsElTagResourcesElDynamic,
}
impl DataLossPreventionDiscoveryConfigActionsElTagResourcesEl {
    #[doc = "Set the field `lower_data_risk_to_low`.\nWhether applying a tag to a resource should lower the risk of the profile for that resource. For example, in conjunction with an [IAM deny policy](https://cloud.google.com/iam/docs/deny-overview), you can deny all principals a permission if a tag value is present, mitigating the risk of the resource. This also lowers the data risk of resources at the lower levels of the resource hierarchy. For example, reducing the data risk of a table data profile also reduces the data risk of the constituent column data profiles."]
    pub fn set_lower_data_risk_to_low(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.lower_data_risk_to_low = Some(v.into());
        self
    }
    #[doc = "Set the field `profile_generations_to_tag`.\nThe profile generations for which the tag should be attached to resources. If you attach a tag to only new profiles, then if the sensitivity score of a profile subsequently changes, its tag doesn't change. By default, this field includes only new profiles. To include both new and updated profiles for tagging, this field should explicitly include both 'PROFILE_GENERATION_NEW' and 'PROFILE_GENERATION_UPDATE'. Possible values: [\"PROFILE_GENERATION_NEW\", \"PROFILE_GENERATION_UPDATE\"]"]
    pub fn set_profile_generations_to_tag(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.profile_generations_to_tag = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_conditions`.\n"]
    pub fn set_tag_conditions(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_conditions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigActionsElTagResourcesEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigActionsElTagResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigActionsElTagResourcesEl {}
impl BuildDataLossPreventionDiscoveryConfigActionsElTagResourcesEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigActionsElTagResourcesEl {
        DataLossPreventionDiscoveryConfigActionsElTagResourcesEl {
            lower_data_risk_to_low: core::default::Default::default(),
            profile_generations_to_tag: core::default::Default::default(),
            tag_conditions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigActionsElTagResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigActionsElTagResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigActionsElTagResourcesElRef {
        DataLossPreventionDiscoveryConfigActionsElTagResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigActionsElTagResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `lower_data_risk_to_low` after provisioning.\nWhether applying a tag to a resource should lower the risk of the profile for that resource. For example, in conjunction with an [IAM deny policy](https://cloud.google.com/iam/docs/deny-overview), you can deny all principals a permission if a tag value is present, mitigating the risk of the resource. This also lowers the data risk of resources at the lower levels of the resource hierarchy. For example, reducing the data risk of a table data profile also reduces the data risk of the constituent column data profiles."]
    pub fn lower_data_risk_to_low(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lower_data_risk_to_low", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `profile_generations_to_tag` after provisioning.\nThe profile generations for which the tag should be attached to resources. If you attach a tag to only new profiles, then if the sensitivity score of a profile subsequently changes, its tag doesn't change. By default, this field includes only new profiles. To include both new and updated profiles for tagging, this field should explicitly include both 'PROFILE_GENERATION_NEW' and 'PROFILE_GENERATION_UPDATE'. Possible values: [\"PROFILE_GENERATION_NEW\", \"PROFILE_GENERATION_UPDATE\"]"]
    pub fn profile_generations_to_tag(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.profile_generations_to_tag", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag_conditions` after provisioning.\n"]
    pub fn tag_conditions(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigActionsElTagResourcesElTagConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tag_conditions", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigActionsElDynamic {
    export_data: Option<DynamicBlock<DataLossPreventionDiscoveryConfigActionsElExportDataEl>>,
    pub_sub_notification:
        Option<DynamicBlock<DataLossPreventionDiscoveryConfigActionsElPubSubNotificationEl>>,
    publish_to_chronicle:
        Option<DynamicBlock<DataLossPreventionDiscoveryConfigActionsElPublishToChronicleEl>>,
    publish_to_dataplex_catalog:
        Option<DynamicBlock<DataLossPreventionDiscoveryConfigActionsElPublishToDataplexCatalogEl>>,
    publish_to_scc: Option<DynamicBlock<DataLossPreventionDiscoveryConfigActionsElPublishToSccEl>>,
    tag_resources: Option<DynamicBlock<DataLossPreventionDiscoveryConfigActionsElTagResourcesEl>>,
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigActionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    export_data: Option<Vec<DataLossPreventionDiscoveryConfigActionsElExportDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub_sub_notification:
        Option<Vec<DataLossPreventionDiscoveryConfigActionsElPubSubNotificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publish_to_chronicle:
        Option<Vec<DataLossPreventionDiscoveryConfigActionsElPublishToChronicleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publish_to_dataplex_catalog:
        Option<Vec<DataLossPreventionDiscoveryConfigActionsElPublishToDataplexCatalogEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publish_to_scc: Option<Vec<DataLossPreventionDiscoveryConfigActionsElPublishToSccEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_resources: Option<Vec<DataLossPreventionDiscoveryConfigActionsElTagResourcesEl>>,
    dynamic: DataLossPreventionDiscoveryConfigActionsElDynamic,
}
impl DataLossPreventionDiscoveryConfigActionsEl {
    #[doc = "Set the field `export_data`.\n"]
    pub fn set_export_data(
        mut self,
        v: impl Into<BlockAssignable<DataLossPreventionDiscoveryConfigActionsElExportDataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.export_data = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.export_data = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `pub_sub_notification`.\n"]
    pub fn set_pub_sub_notification(
        mut self,
        v: impl Into<BlockAssignable<DataLossPreventionDiscoveryConfigActionsElPubSubNotificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pub_sub_notification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pub_sub_notification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `publish_to_chronicle`.\n"]
    pub fn set_publish_to_chronicle(
        mut self,
        v: impl Into<BlockAssignable<DataLossPreventionDiscoveryConfigActionsElPublishToChronicleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.publish_to_chronicle = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.publish_to_chronicle = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `publish_to_dataplex_catalog`.\n"]
    pub fn set_publish_to_dataplex_catalog(
        mut self,
        v: impl Into<
            BlockAssignable<DataLossPreventionDiscoveryConfigActionsElPublishToDataplexCatalogEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.publish_to_dataplex_catalog = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.publish_to_dataplex_catalog = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `publish_to_scc`.\n"]
    pub fn set_publish_to_scc(
        mut self,
        v: impl Into<BlockAssignable<DataLossPreventionDiscoveryConfigActionsElPublishToSccEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.publish_to_scc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.publish_to_scc = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tag_resources`.\n"]
    pub fn set_tag_resources(
        mut self,
        v: impl Into<BlockAssignable<DataLossPreventionDiscoveryConfigActionsElTagResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_resources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigActionsEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigActionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigActionsEl {}
impl BuildDataLossPreventionDiscoveryConfigActionsEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigActionsEl {
        DataLossPreventionDiscoveryConfigActionsEl {
            export_data: core::default::Default::default(),
            pub_sub_notification: core::default::Default::default(),
            publish_to_chronicle: core::default::Default::default(),
            publish_to_dataplex_catalog: core::default::Default::default(),
            publish_to_scc: core::default::Default::default(),
            tag_resources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigActionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigActionsElRef {
    fn new(shared: StackShared, base: String) -> DataLossPreventionDiscoveryConfigActionsElRef {
        DataLossPreventionDiscoveryConfigActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `export_data` after provisioning.\n"]
    pub fn export_data(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigActionsElExportDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.export_data", self.base))
    }
    #[doc = "Get a reference to the value of field `pub_sub_notification` after provisioning.\n"]
    pub fn pub_sub_notification(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigActionsElPubSubNotificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pub_sub_notification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `publish_to_chronicle` after provisioning.\n"]
    pub fn publish_to_chronicle(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigActionsElPublishToChronicleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.publish_to_chronicle", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `publish_to_dataplex_catalog` after provisioning.\n"]
    pub fn publish_to_dataplex_catalog(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigActionsElPublishToDataplexCatalogElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.publish_to_dataplex_catalog", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `publish_to_scc` after provisioning.\n"]
    pub fn publish_to_scc(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigActionsElPublishToSccElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.publish_to_scc", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag_resources` after provisioning.\n"]
    pub fn tag_resources(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigActionsElTagResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tag_resources", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigOrgConfigElLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    folder_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization_id: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigOrgConfigElLocationEl {
    #[doc = "Set the field `folder_id`.\nThe ID for the folder within an organization to scan"]
    pub fn set_folder_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.folder_id = Some(v.into());
        self
    }
    #[doc = "Set the field `organization_id`.\nThe ID of an organization to scan"]
    pub fn set_organization_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organization_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigOrgConfigElLocationEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigOrgConfigElLocationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigOrgConfigElLocationEl {}
impl BuildDataLossPreventionDiscoveryConfigOrgConfigElLocationEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigOrgConfigElLocationEl {
        DataLossPreventionDiscoveryConfigOrgConfigElLocationEl {
            folder_id: core::default::Default::default(),
            organization_id: core::default::Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigOrgConfigElLocationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigOrgConfigElLocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigOrgConfigElLocationElRef {
        DataLossPreventionDiscoveryConfigOrgConfigElLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigOrgConfigElLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `folder_id` after provisioning.\nThe ID for the folder within an organization to scan"]
    pub fn folder_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.folder_id", self.base))
    }
    #[doc = "Get a reference to the value of field `organization_id` after provisioning.\nThe ID of an organization to scan"]
    pub fn organization_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_id", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigOrgConfigElDynamic {
    location: Option<DynamicBlock<DataLossPreventionDiscoveryConfigOrgConfigElLocationEl>>,
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigOrgConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<Vec<DataLossPreventionDiscoveryConfigOrgConfigElLocationEl>>,
    dynamic: DataLossPreventionDiscoveryConfigOrgConfigElDynamic,
}
impl DataLossPreventionDiscoveryConfigOrgConfigEl {
    #[doc = "Set the field `project_id`.\nThe project that will run the scan. The DLP service account that exists within this project must have access to all resources that are profiled, and the cloud DLP API must be enabled."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(
        mut self,
        v: impl Into<BlockAssignable<DataLossPreventionDiscoveryConfigOrgConfigElLocationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.location = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigOrgConfigEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigOrgConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigOrgConfigEl {}
impl BuildDataLossPreventionDiscoveryConfigOrgConfigEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigOrgConfigEl {
        DataLossPreventionDiscoveryConfigOrgConfigEl {
            project_id: core::default::Default::default(),
            location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigOrgConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigOrgConfigElRef {
    fn new(shared: StackShared, base: String) -> DataLossPreventionDiscoveryConfigOrgConfigElRef {
        DataLossPreventionDiscoveryConfigOrgConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigOrgConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe project that will run the scan. The DLP service account that exists within this project must have access to all resources that are profiled, and the cloud DLP API must be enabled."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> ListRef<DataLossPreventionDiscoveryConfigOrgConfigElLocationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.location", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElAwsLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    all_asset_inventory_assets: Option<PrimField<bool>>,
}
impl DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElAwsLocationEl {
    #[doc = "Set the field `account_id`.\nThe AWS account ID that this discovery config applies to. Within an organization, you can find the AWS account ID inside an AWS account ARN. Example: arn:<partition>:organizations::<management-account-id>:account/<organization-id>/<account-id>"]
    pub fn set_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.account_id = Some(v.into());
        self
    }
    #[doc = "Set the field `all_asset_inventory_assets`.\nAll AWS assets stored in Asset Inventory that didn't match other AWS discovery configs."]
    pub fn set_all_asset_inventory_assets(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all_asset_inventory_assets = Some(v.into());
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElAwsLocationEl {
    type O =
        BlockAssignable<DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElAwsLocationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigOtherCloudStartingLocationElAwsLocationEl {}
impl BuildDataLossPreventionDiscoveryConfigOtherCloudStartingLocationElAwsLocationEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElAwsLocationEl {
        DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElAwsLocationEl {
            account_id: core::default::Default::default(),
            all_asset_inventory_assets: core::default::Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElAwsLocationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElAwsLocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElAwsLocationElRef {
        DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElAwsLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElAwsLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe AWS account ID that this discovery config applies to. Within an organization, you can find the AWS account ID inside an AWS account ARN. Example: arn:<partition>:organizations::<management-account-id>:account/<organization-id>/<account-id>"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }
    #[doc = "Get a reference to the value of field `all_asset_inventory_assets` after provisioning.\nAll AWS assets stored in Asset Inventory that didn't match other AWS discovery configs."]
    pub fn all_asset_inventory_assets(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.all_asset_inventory_assets", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElDynamic {
    aws_location: Option<
        DynamicBlock<DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElAwsLocationEl>,
    >,
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigOtherCloudStartingLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_location:
        Option<Vec<DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElAwsLocationEl>>,
    dynamic: DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElDynamic,
}
impl DataLossPreventionDiscoveryConfigOtherCloudStartingLocationEl {
    #[doc = "Set the field `aws_location`.\n"]
    pub fn set_aws_location(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElAwsLocationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_location = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigOtherCloudStartingLocationEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigOtherCloudStartingLocationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigOtherCloudStartingLocationEl {}
impl BuildDataLossPreventionDiscoveryConfigOtherCloudStartingLocationEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigOtherCloudStartingLocationEl {
        DataLossPreventionDiscoveryConfigOtherCloudStartingLocationEl {
            aws_location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElRef {
        DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aws_location` after provisioning.\n"]
    pub fn aws_location(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigOtherCloudStartingLocationElAwsLocationElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.aws_location", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElInspectTemplateModifiedCadenceEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElInspectTemplateModifiedCadenceEl { # [doc = "Set the field `frequency`.\nHow frequently data profiles can be updated when the template is modified. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"] pub fn set_frequency (mut self , v : impl Into < PrimField < String > >) -> Self { self . frequency = Some (v . into ()) ; self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElInspectTemplateModifiedCadenceEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElInspectTemplateModifiedCadenceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElInspectTemplateModifiedCadenceEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElInspectTemplateModifiedCadenceEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElInspectTemplateModifiedCadenceEl { DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElInspectTemplateModifiedCadenceEl { frequency : core :: default :: Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElInspectTemplateModifiedCadenceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElInspectTemplateModifiedCadenceElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElInspectTemplateModifiedCadenceElRef { DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElInspectTemplateModifiedCadenceElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElInspectTemplateModifiedCadenceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `frequency` after provisioning.\nHow frequently data profiles can be updated when the template is modified. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"] pub fn frequency (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.frequency" , self . base)) } }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElSchemaModifiedCadenceEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<ListField<PrimField<String>>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElSchemaModifiedCadenceEl {
    #[doc = "Set the field `frequency`.\nHow frequently profiles may be updated when schemas are modified. Default to monthly Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn set_frequency(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.frequency = Some(v.into());
        self
    }
    #[doc = "Set the field `types`.\nThe type of events to consider when deciding if the table's schema has been modified and should have the profile updated. Defaults to NEW_COLUMN. Possible values: [\"SCHEMA_NEW_COLUMNS\", \"SCHEMA_REMOVED_COLUMNS\"]"]
    pub fn set_types(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.types = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElSchemaModifiedCadenceEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElSchemaModifiedCadenceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElSchemaModifiedCadenceEl
{}
impl
    BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElSchemaModifiedCadenceEl
{
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElSchemaModifiedCadenceEl
    {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElSchemaModifiedCadenceEl {
            frequency: core::default::Default::default(),
            types: core::default::Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElSchemaModifiedCadenceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElSchemaModifiedCadenceElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElSchemaModifiedCadenceElRef { DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElSchemaModifiedCadenceElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElSchemaModifiedCadenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `frequency` after provisioning.\nHow frequently profiles may be updated when schemas are modified. Default to monthly Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.base))
    }
    #[doc = "Get a reference to the value of field `types` after provisioning.\nThe type of events to consider when deciding if the table's schema has been modified and should have the profile updated. Defaults to NEW_COLUMN. Possible values: [\"SCHEMA_NEW_COLUMNS\", \"SCHEMA_REMOVED_COLUMNS\"]"]
    pub fn types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.types", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElTableModifiedCadenceEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<ListField<PrimField<String>>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElTableModifiedCadenceEl {
    #[doc = "Set the field `frequency`.\nHow frequently data profiles can be updated when tables are modified. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn set_frequency(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.frequency = Some(v.into());
        self
    }
    #[doc = "Set the field `types`.\nThe type of events to consider when deciding if the table has been modified and should have the profile updated. Defaults to MODIFIED_TIMESTAMP Possible values: [\"TABLE_MODIFIED_TIMESTAMP\"]"]
    pub fn set_types(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.types = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElTableModifiedCadenceEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElTableModifiedCadenceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElTableModifiedCadenceEl
{}
impl
    BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElTableModifiedCadenceEl
{
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElTableModifiedCadenceEl
    {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElTableModifiedCadenceEl {
            frequency: core::default::Default::default(),
            types: core::default::Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElTableModifiedCadenceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElTableModifiedCadenceElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElTableModifiedCadenceElRef
    {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElTableModifiedCadenceElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElTableModifiedCadenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `frequency` after provisioning.\nHow frequently data profiles can be updated when tables are modified. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.base))
    }
    #[doc = "Get a reference to the value of field `types` after provisioning.\nThe type of events to consider when deciding if the table has been modified and should have the profile updated. Defaults to MODIFIED_TIMESTAMP Possible values: [\"TABLE_MODIFIED_TIMESTAMP\"]"]
    pub fn types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.types", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElDynamic { inspect_template_modified_cadence : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElInspectTemplateModifiedCadenceEl >> , schema_modified_cadence : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElSchemaModifiedCadenceEl >> , table_modified_cadence : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElTableModifiedCadenceEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceEl { # [serde (skip_serializing_if = "Option::is_none")] inspect_template_modified_cadence : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElInspectTemplateModifiedCadenceEl > > , # [serde (skip_serializing_if = "Option::is_none")] schema_modified_cadence : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElSchemaModifiedCadenceEl > > , # [serde (skip_serializing_if = "Option::is_none")] table_modified_cadence : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElTableModifiedCadenceEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceEl {
    #[doc = "Set the field `inspect_template_modified_cadence`.\n"]
    pub fn set_inspect_template_modified_cadence(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElInspectTemplateModifiedCadenceEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inspect_template_modified_cadence = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inspect_template_modified_cadence = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `schema_modified_cadence`.\n"]
    pub fn set_schema_modified_cadence(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElSchemaModifiedCadenceEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schema_modified_cadence = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_modified_cadence = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `table_modified_cadence`.\n"]
    pub fn set_table_modified_cadence(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElTableModifiedCadenceEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.table_modified_cadence = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.table_modified_cadence = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceEl {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceEl {
            inspect_template_modified_cadence: core::default::Default::default(),
            schema_modified_cadence: core::default::Default::default(),
            table_modified_cadence: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElRef {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `inspect_template_modified_cadence` after provisioning.\n"]    pub fn inspect_template_modified_cadence (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElInspectTemplateModifiedCadenceElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.inspect_template_modified_cadence", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schema_modified_cadence` after provisioning.\n"]    pub fn schema_modified_cadence (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElSchemaModifiedCadenceElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_modified_cadence", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `table_modified_cadence` after provisioning.\n"]    pub fn table_modified_cadence (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElTableModifiedCadenceElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.table_modified_cadence", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElOrConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    min_age: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_row_count: Option<PrimField<f64>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElOrConditionsEl {
    #[doc = "Set the field `min_age`.\nDuration format. The minimum age a table must have before Cloud DLP can profile it. Value greater than 1."]
    pub fn set_min_age(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_age = Some(v.into());
        self
    }
    #[doc = "Set the field `min_row_count`.\nMinimum number of rows that should be present before Cloud DLP profiles as a table."]
    pub fn set_min_row_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_row_count = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElOrConditionsEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElOrConditionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElOrConditionsEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElOrConditionsEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElOrConditionsEl {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElOrConditionsEl {
            min_age: core::default::Default::default(),
            min_row_count: core::default::Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElOrConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElOrConditionsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElOrConditionsElRef
    {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElOrConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElOrConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `min_age` after provisioning.\nDuration format. The minimum age a table must have before Cloud DLP can profile it. Value greater than 1."]
    pub fn min_age(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_age", self.base))
    }
    #[doc = "Get a reference to the value of field `min_row_count` after provisioning.\nMinimum number of rows that should be present before Cloud DLP profiles as a table."]
    pub fn min_row_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_row_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElTypesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<ListField<PrimField<String>>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElTypesEl {
    #[doc = "Set the field `types`.\nA set of BiqQuery table types Possible values: [\"BIG_QUERY_TABLE_TYPE_TABLE\", \"BIG_QUERY_TABLE_TYPE_EXTERNAL_BIG_LAKE\"]"]
    pub fn set_types(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.types = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElTypesEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElTypesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElTypesEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElTypesEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElTypesEl {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElTypesEl {
            types: core::default::Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElTypesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElTypesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElTypesElRef {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElTypesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `types` after provisioning.\nA set of BiqQuery table types Possible values: [\"BIG_QUERY_TABLE_TYPE_TABLE\", \"BIG_QUERY_TABLE_TYPE_EXTERNAL_BIG_LAKE\"]"]
    pub fn types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.types", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElDynamic {
    or_conditions: Option<
        DynamicBlock<
            DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElOrConditionsEl,
        >,
    >,
    types: Option<
        DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElTypesEl>,
    >,
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_after: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    type_collection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    or_conditions: Option<
        Vec<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElOrConditionsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    types:
        Option<Vec<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElTypesEl>>,
    dynamic: DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElDynamic,
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsEl {
    #[doc = "Set the field `created_after`.\nA timestamp in RFC3339 UTC \"Zulu\" format with nanosecond resolution and upto nine fractional digits."]
    pub fn set_created_after(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_after = Some(v.into());
        self
    }
    #[doc = "Set the field `type_collection`.\nRestrict discovery to categories of table types. Currently view, materialized view, snapshot and non-biglake external tables are supported. Possible values: [\"BIG_QUERY_COLLECTION_ALL_TYPES\", \"BIG_QUERY_COLLECTION_ONLY_SUPPORTED_TYPES\"]"]
    pub fn set_type_collection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_collection = Some(v.into());
        self
    }
    #[doc = "Set the field `or_conditions`.\n"]
    pub fn set_or_conditions(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElOrConditionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.or_conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.or_conditions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `types`.\n"]
    pub fn set_types(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElTypesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.types = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.types = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsEl {
    type O =
        BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsEl {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsEl {
            created_after: core::default::Default::default(),
            type_collection: core::default::Default::default(),
            or_conditions: core::default::Default::default(),
            types: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElRef {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `created_after` after provisioning.\nA timestamp in RFC3339 UTC \"Zulu\" format with nanosecond resolution and upto nine fractional digits."]
    pub fn created_after(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_after", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_collection` after provisioning.\nRestrict discovery to categories of table types. Currently view, materialized view, snapshot and non-biglake external tables are supported. Possible values: [\"BIG_QUERY_COLLECTION_ALL_TYPES\", \"BIG_QUERY_COLLECTION_ONLY_SUPPORTED_TYPES\"]"]
    pub fn type_collection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type_collection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `or_conditions` after provisioning.\n"]
    pub fn or_conditions(
        &self,
    ) -> ListRef<
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElOrConditionsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.or_conditions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `types` after provisioning.\n"]
    pub fn types(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElTypesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.types", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDisabledEl {}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDisabledEl {}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDisabledEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDisabledEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDisabledEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDisabledEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDisabledEl {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDisabledEl {}
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDisabledElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDisabledElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDisabledElRef {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDisabledElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDisabledElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElOtherTablesEl {}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElOtherTablesEl {}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElOtherTablesEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElOtherTablesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElOtherTablesEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElOtherTablesEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElOtherTablesEl {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElOtherTablesEl {}
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElOtherTablesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElOtherTablesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElOtherTablesElRef {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElOtherTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElOtherTablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTableReferenceEl {
    dataset_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    table_id: PrimField<String>,
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTableReferenceEl {
    #[doc = "Set the field `project_id`.\nThe Google Cloud project ID of the project containing the table.\nIf omitted, the project ID is inferred from the parent project.\nThis field is required if the parent resource is an organization."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTableReferenceEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTableReferenceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTableReferenceEl {
    #[doc = "Dataset ID of the table."]
    pub dataset_id: PrimField<String>,
    #[doc = "Name of the table."]
    pub table_id: PrimField<String>,
}
impl BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTableReferenceEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTableReferenceEl {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTableReferenceEl {
            dataset_id: self.dataset_id,
            project_id: core::default::Default::default(),
            table_id: self.table_id,
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTableReferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTableReferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTableReferenceElRef {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTableReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTableReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nDataset ID of the table."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe Google Cloud project ID of the project containing the table.\nIf omitted, the project ID is inferred from the parent project.\nThis field is required if the parent resource is an organization."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nName of the table."]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElPatternsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_id_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_id_regex: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElPatternsEl { # [doc = "Set the field `dataset_id_regex`.\nif unset, this property matches all datasets"] pub fn set_dataset_id_regex (mut self , v : impl Into < PrimField < String > >) -> Self { self . dataset_id_regex = Some (v . into ()) ; self } # [doc = "Set the field `project_id_regex`.\nFor organizations, if unset, will match all projects. Has no effect for data profile configurations created within a project."] pub fn set_project_id_regex (mut self , v : impl Into < PrimField < String > >) -> Self { self . project_id_regex = Some (v . into ()) ; self } # [doc = "Set the field `table_id_regex`.\nif unset, this property matches all tables"] pub fn set_table_id_regex (mut self , v : impl Into < PrimField < String > >) -> Self { self . table_id_regex = Some (v . into ()) ; self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElPatternsEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElPatternsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElPatternsEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElPatternsEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElPatternsEl { DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElPatternsEl { dataset_id_regex : core :: default :: Default :: default () , project_id_regex : core :: default :: Default :: default () , table_id_regex : core :: default :: Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElPatternsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElPatternsElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElPatternsElRef { DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElPatternsElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElPatternsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `dataset_id_regex` after provisioning.\nif unset, this property matches all datasets"] pub fn dataset_id_regex (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.dataset_id_regex" , self . base)) } # [doc = "Get a reference to the value of field `project_id_regex` after provisioning.\nFor organizations, if unset, will match all projects. Has no effect for data profile configurations created within a project."] pub fn project_id_regex (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.project_id_regex" , self . base)) } # [doc = "Get a reference to the value of field `table_id_regex` after provisioning.\nif unset, this property matches all tables"] pub fn table_id_regex (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.table_id_regex" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElDynamic { patterns : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElPatternsEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesEl { # [serde (skip_serializing_if = "Option::is_none")] patterns : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElPatternsEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesEl {
    #[doc = "Set the field `patterns`.\n"]
    pub fn set_patterns(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElPatternsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.patterns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.patterns = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesEl
{}
impl
    BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesEl
{
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesEl
    {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesEl {
            patterns: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElRef { DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `patterns` after provisioning.\n"]    pub fn patterns (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElPatternsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.patterns", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElDynamic { include_regexes : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesEl { # [serde (skip_serializing_if = "Option::is_none")] include_regexes : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesEl {
    #[doc = "Set the field `include_regexes`.\n"]
    pub fn set_include_regexes(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.include_regexes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.include_regexes = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesEl {
    type O =
        BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesEl {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesEl {
            include_regexes: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElRef {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `include_regexes` after provisioning.\n"]    pub fn include_regexes (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElIncludeRegexesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_regexes", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElDynamic {
    other_tables: Option<
        DynamicBlock<
            DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElOtherTablesEl,
        >,
    >,
    table_reference: Option<
        DynamicBlock<
            DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTableReferenceEl,
        >,
    >,
    tables: Option<
        DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesEl>,
    >,
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    other_tables: Option<
        Vec<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElOtherTablesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_reference: Option<
        Vec<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTableReferenceEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    tables: Option<Vec<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesEl>>,
    dynamic: DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElDynamic,
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterEl {
    #[doc = "Set the field `other_tables`.\n"]
    pub fn set_other_tables(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElOtherTablesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.other_tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.other_tables = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `table_reference`.\n"]
    pub fn set_table_reference(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTableReferenceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.table_reference = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.table_reference = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tables`.\n"]
    pub fn set_tables(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tables = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterEl {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterEl {
            other_tables: core::default::Default::default(),
            table_reference: core::default::Default::default(),
            tables: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElRef {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `other_tables` after provisioning.\n"]
    pub fn other_tables(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElOtherTablesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.other_tables", self.base))
    }
    #[doc = "Get a reference to the value of field `table_reference` after provisioning.\n"]
    pub fn table_reference(
        &self,
    ) -> ListRef<
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTableReferenceElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table_reference", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElTablesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.tables", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDynamic {
    cadence:
        Option<DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceEl>>,
    conditions: Option<
        DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsEl>,
    >,
    disabled:
        Option<DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDisabledEl>>,
    filter:
        Option<DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterEl>>,
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cadence: Option<Vec<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<Vec<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<Vec<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDisabledEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterEl>>,
    dynamic: DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDynamic,
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetEl {
    #[doc = "Set the field `cadence`.\n"]
    pub fn set_cadence(
        mut self,
        v: impl Into<
            BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cadence = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cadence = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl Into<
            BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(
        mut self,
        v: impl Into<
            BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDisabledEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.disabled = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.disabled = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl Into<
            BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElBigQueryTargetEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetEl {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetEl {
            cadence: core::default::Default::default(),
            conditions: core::default::Default::default(),
            disabled: core::default::Default::default(),
            filter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElRef {
        DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cadence` after provisioning.\n"]
    pub fn cadence(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElCadenceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cadence", self.base))
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElConditionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElDisabledElRef> {
        ListRef::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    database_engines: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<ListField<PrimField<String>>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElConditionsEl {
    #[doc = "Set the field `database_engines`.\nDatabase engines that should be profiled. Optional. Defaults to ALL_SUPPORTED_DATABASE_ENGINES if unspecified. Possible values: [\"ALL_SUPPORTED_DATABASE_ENGINES\", \"MYSQL\", \"POSTGRES\"]"]
    pub fn set_database_engines(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.database_engines = Some(v.into());
        self
    }
    #[doc = "Set the field `types`.\nData profiles will only be generated for the database resource types specified in this field. If not specified, defaults to [DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES]. Possible values: [\"DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES\", \"DATABASE_RESOURCE_TYPE_TABLE\"]"]
    pub fn set_types(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.types = Some(v.into());
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElConditionsEl {
    type O =
        BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElConditionsEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElConditionsEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElConditionsEl {
        DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElConditionsEl {
            database_engines: core::default::Default::default(),
            types: core::default::Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElConditionsElRef {
        DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database_engines` after provisioning.\nDatabase engines that should be profiled. Optional. Defaults to ALL_SUPPORTED_DATABASE_ENGINES if unspecified. Possible values: [\"ALL_SUPPORTED_DATABASE_ENGINES\", \"MYSQL\", \"POSTGRES\"]"]
    pub fn database_engines(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.database_engines", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `types` after provisioning.\nData profiles will only be generated for the database resource types specified in this field. If not specified, defaults to [DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES]. Possible values: [\"DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES\", \"DATABASE_RESOURCE_TYPE_TABLE\"]"]
    pub fn types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.types", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDisabledEl {}
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDisabledEl {}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDisabledEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDisabledEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDisabledEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDisabledEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDisabledEl {
        DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDisabledEl {}
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDisabledElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDisabledElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDisabledElRef {
        DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDisabledElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDisabledElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElPatternsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    database_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_resource_name_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id_regex: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElPatternsEl { # [doc = "Set the field `database_regex`.\nRegex to test the database name against. If empty, all databases match."] pub fn set_database_regex (mut self , v : impl Into < PrimField < String > >) -> Self { self . database_regex = Some (v . into ()) ; self } # [doc = "Set the field `database_resource_name_regex`.\nRegex to test the database resource's name against. An example of a database resource name is a table's name. Other database resource names like view names could be included in the future. If empty, all database resources match.'"] pub fn set_database_resource_name_regex (mut self , v : impl Into < PrimField < String > >) -> Self { self . database_resource_name_regex = Some (v . into ()) ; self } # [doc = "Set the field `instance_regex`.\nRegex to test the instance name against. If empty, all instances match."] pub fn set_instance_regex (mut self , v : impl Into < PrimField < String > >) -> Self { self . instance_regex = Some (v . into ()) ; self } # [doc = "Set the field `project_id_regex`.\nFor organizations, if unset, will match all projects. Has no effect for data profile configurations created within a project."] pub fn set_project_id_regex (mut self , v : impl Into < PrimField < String > >) -> Self { self . project_id_regex = Some (v . into ()) ; self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElPatternsEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElPatternsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElPatternsEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElPatternsEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElPatternsEl { DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElPatternsEl { database_regex : core :: default :: Default :: default () , database_resource_name_regex : core :: default :: Default :: default () , instance_regex : core :: default :: Default :: default () , project_id_regex : core :: default :: Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElPatternsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElPatternsElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElPatternsElRef { DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElPatternsElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElPatternsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `database_regex` after provisioning.\nRegex to test the database name against. If empty, all databases match."] pub fn database_regex (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.database_regex" , self . base)) } # [doc = "Get a reference to the value of field `database_resource_name_regex` after provisioning.\nRegex to test the database resource's name against. An example of a database resource name is a table's name. Other database resource names like view names could be included in the future. If empty, all database resources match.'"] pub fn database_resource_name_regex (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.database_resource_name_regex" , self . base)) } # [doc = "Get a reference to the value of field `instance_regex` after provisioning.\nRegex to test the instance name against. If empty, all instances match."] pub fn instance_regex (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.instance_regex" , self . base)) } # [doc = "Get a reference to the value of field `project_id_regex` after provisioning.\nFor organizations, if unset, will match all projects. Has no effect for data profile configurations created within a project."] pub fn project_id_regex (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.project_id_regex" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElDynamic { patterns : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElPatternsEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesEl { # [serde (skip_serializing_if = "Option::is_none")] patterns : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElPatternsEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElDynamic , }
impl
    DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesEl
{
    #[doc = "Set the field `patterns`.\n"]
    pub fn set_patterns(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElPatternsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.patterns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.patterns = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesEl { DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesEl { patterns : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElRef { DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `patterns` after provisioning.\n"] pub fn patterns (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElPatternsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.patterns" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElDynamic { include_regexes : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionEl { # [serde (skip_serializing_if = "Option::is_none")] include_regexes : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionEl {
    #[doc = "Set the field `include_regexes`.\n"]
    pub fn set_include_regexes(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.include_regexes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.include_regexes = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionEl {
        DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionEl {
            include_regexes: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElRef {
        DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `include_regexes` after provisioning.\n"]    pub fn include_regexes (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElIncludeRegexesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_regexes", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDatabaseResourceReferenceEl
{
    database: PrimField<String>,
    database_resource: PrimField<String>,
    instance: PrimField<String>,
    project_id: PrimField<String>,
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDatabaseResourceReferenceEl {}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDatabaseResourceReferenceEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDatabaseResourceReferenceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDatabaseResourceReferenceEl
{
    #[doc = "Required. Name of a database within the instance."]
    pub database: PrimField<String>,
    #[doc = "Required. Name of a database resource, for example, a table within the database."]
    pub database_resource: PrimField<String>,
    #[doc = "Required. The instance where this resource is located. For example: Cloud SQL instance ID."]
    pub instance: PrimField<String>,
    #[doc = "Required. If within a project-level config, then this must match the config's project ID."]
    pub project_id: PrimField<String>,
}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDatabaseResourceReferenceEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDatabaseResourceReferenceEl { DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDatabaseResourceReferenceEl { database : self . database , database_resource : self . database_resource , instance : self . instance , project_id : self . project_id , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDatabaseResourceReferenceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDatabaseResourceReferenceElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDatabaseResourceReferenceElRef { DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDatabaseResourceReferenceElRef { shared : shared , base : base . to_string () , } } }
impl
    DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDatabaseResourceReferenceElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nRequired. Name of a database within the instance."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `database_resource` after provisioning.\nRequired. Name of a database resource, for example, a table within the database."]
    pub fn database_resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_resource", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nRequired. The instance where this resource is located. For example: Cloud SQL instance ID."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nRequired. If within a project-level config, then this must match the config's project ID."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElOthersEl {}
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElOthersEl {}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElOthersEl {
    type O =
        BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElOthersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElOthersEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElOthersEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElOthersEl {
        DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElOthersEl {}
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElOthersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElOthersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElOthersElRef {
        DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElOthersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElOthersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDynamic { collection : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionEl >> , database_resource_reference : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDatabaseResourceReferenceEl >> , others : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElOthersEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterEl { # [serde (skip_serializing_if = "Option::is_none")] collection : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionEl > > , # [serde (skip_serializing_if = "Option::is_none")] database_resource_reference : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDatabaseResourceReferenceEl > > , # [serde (skip_serializing_if = "Option::is_none")] others : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElOthersEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterEl {
    #[doc = "Set the field `collection`.\n"]
    pub fn set_collection(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.collection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.collection = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `database_resource_reference`.\n"]
    pub fn set_database_resource_reference(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDatabaseResourceReferenceEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.database_resource_reference = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.database_resource_reference = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `others`.\n"]
    pub fn set_others(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElOthersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.others = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.others = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterEl {
        DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterEl {
            collection: core::default::Default::default(),
            database_resource_reference: core::default::Default::default(),
            others: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElRef {
        DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `collection` after provisioning.\n"]
    pub fn collection(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElCollectionElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.collection", self.base))
    }
    #[doc = "Get a reference to the value of field `database_resource_reference` after provisioning.\n"]    pub fn database_resource_reference (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElDatabaseResourceReferenceElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.database_resource_reference", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `others` after provisioning.\n"]
    pub fn others(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElOthersElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.others", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl
{
    frequency: PrimField<String>,
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl { }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl
{
    #[doc = "How frequently data profiles can be updated when the template is modified. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub frequency: PrimField<String>,
}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl { DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl { frequency : self . frequency , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElInspectTemplateModifiedCadenceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElInspectTemplateModifiedCadenceElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElInspectTemplateModifiedCadenceElRef { DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElInspectTemplateModifiedCadenceElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElInspectTemplateModifiedCadenceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `frequency` after provisioning.\nHow frequently data profiles can be updated when the template is modified. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"] pub fn frequency (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.frequency" , self . base)) } }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElSchemaModifiedCadenceEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<ListField<PrimField<String>>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElSchemaModifiedCadenceEl { # [doc = "Set the field `frequency`.\nFrequency to regenerate data profiles when the schema is modified. Defaults to monthly. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"] pub fn set_frequency (mut self , v : impl Into < PrimField < String > >) -> Self { self . frequency = Some (v . into ()) ; self } # [doc = "Set the field `types`.\nThe types of schema modifications to consider. Defaults to NEW_COLUMNS. Possible values: [\"NEW_COLUMNS\", \"REMOVED_COLUMNS\"]"] pub fn set_types (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . types = Some (v . into ()) ; self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElSchemaModifiedCadenceEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElSchemaModifiedCadenceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElSchemaModifiedCadenceEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElSchemaModifiedCadenceEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElSchemaModifiedCadenceEl { DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElSchemaModifiedCadenceEl { frequency : core :: default :: Default :: default () , types : core :: default :: Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElSchemaModifiedCadenceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElSchemaModifiedCadenceElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElSchemaModifiedCadenceElRef { DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElSchemaModifiedCadenceElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElSchemaModifiedCadenceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `frequency` after provisioning.\nFrequency to regenerate data profiles when the schema is modified. Defaults to monthly. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"] pub fn frequency (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.frequency" , self . base)) } # [doc = "Get a reference to the value of field `types` after provisioning.\nThe types of schema modifications to consider. Defaults to NEW_COLUMNS. Possible values: [\"NEW_COLUMNS\", \"REMOVED_COLUMNS\"]"] pub fn types (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.types" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElDynamic { inspect_template_modified_cadence : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl >> , schema_modified_cadence : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElSchemaModifiedCadenceEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceEl { # [serde (skip_serializing_if = "Option::is_none")] refresh_frequency : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] inspect_template_modified_cadence : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl > > , # [serde (skip_serializing_if = "Option::is_none")] schema_modified_cadence : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElSchemaModifiedCadenceEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceEl {
    #[doc = "Set the field `refresh_frequency`.\nData changes (non-schema changes) in Cloud SQL tables can't trigger reprofiling. If you set this field, profiles are refreshed at this frequency regardless of whether the underlying tables have changes. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn set_refresh_frequency(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.refresh_frequency = Some(v.into());
        self
    }
    #[doc = "Set the field `inspect_template_modified_cadence`.\n"]
    pub fn set_inspect_template_modified_cadence(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inspect_template_modified_cadence = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inspect_template_modified_cadence = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `schema_modified_cadence`.\n"]
    pub fn set_schema_modified_cadence(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElSchemaModifiedCadenceEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schema_modified_cadence = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_modified_cadence = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceEl {
        DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceEl {
            refresh_frequency: core::default::Default::default(),
            inspect_template_modified_cadence: core::default::Default::default(),
            schema_modified_cadence: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElRef {
        DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `refresh_frequency` after provisioning.\nData changes (non-schema changes) in Cloud SQL tables can't trigger reprofiling. If you set this field, profiles are refreshed at this frequency regardless of whether the underlying tables have changes. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn refresh_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.refresh_frequency", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inspect_template_modified_cadence` after provisioning.\n"]    pub fn inspect_template_modified_cadence (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElInspectTemplateModifiedCadenceElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.inspect_template_modified_cadence", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schema_modified_cadence` after provisioning.\n"]    pub fn schema_modified_cadence (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElSchemaModifiedCadenceElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_modified_cadence", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDynamic {
    conditions: Option<
        DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElConditionsEl>,
    >,
    disabled:
        Option<DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDisabledEl>>,
    filter:
        Option<DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterEl>>,
    generation_cadence: Option<
        DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceEl>,
    >,
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<Vec<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<Vec<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDisabledEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_cadence:
        Option<Vec<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceEl>>,
    dynamic: DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDynamic,
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetEl {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl Into<
            BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElConditionsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(
        mut self,
        v: impl Into<
            BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDisabledEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.disabled = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.disabled = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl Into<
            BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `generation_cadence`.\n"]
    pub fn set_generation_cadence(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.generation_cadence = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.generation_cadence = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetEl {
        DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetEl {
            conditions: core::default::Default::default(),
            disabled: core::default::Default::default(),
            filter: core::default::Default::default(),
            generation_cadence: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElRef {
        DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElConditionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElDisabledElRef> {
        ListRef::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `generation_cadence` after provisioning.\n"]
    pub fn generation_cadence(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElGenerationCadenceElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.generation_cadence", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElCloudStorageConditionsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    included_bucket_attributes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_object_attributes: Option<ListField<PrimField<String>>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElCloudStorageConditionsEl { # [doc = "Set the field `included_bucket_attributes`.\nOnly objects with the specified attributes will be scanned. Defaults to [ALL_SUPPORTED_BUCKETS] if unset. Possible values: [\"ALL_SUPPORTED_BUCKETS\", \"AUTOCLASS_DISABLED\", \"AUTOCLASS_ENABLED\"]"] pub fn set_included_bucket_attributes (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . included_bucket_attributes = Some (v . into ()) ; self } # [doc = "Set the field `included_object_attributes`.\nOnly objects with the specified attributes will be scanned. If an object has one of the specified attributes but is inside an excluded bucket, it will not be scanned. Defaults to [ALL_SUPPORTED_OBJECTS]. A profile will be created even if no objects match the included_object_attributes. Possible values: [\"ALL_SUPPORTED_OBJECTS\", \"STANDARD\", \"NEARLINE\", \"COLDLINE\", \"ARCHIVE\", \"REGIONAL\", \"MULTI_REGIONAL\", \"DURABLE_REDUCED_AVAILABILITY\"]"] pub fn set_included_object_attributes (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . included_object_attributes = Some (v . into ()) ; self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElCloudStorageConditionsEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElCloudStorageConditionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElCloudStorageConditionsEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElCloudStorageConditionsEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElCloudStorageConditionsEl { DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElCloudStorageConditionsEl { included_bucket_attributes : core :: default :: Default :: default () , included_object_attributes : core :: default :: Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElCloudStorageConditionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElCloudStorageConditionsElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElCloudStorageConditionsElRef { DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElCloudStorageConditionsElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElCloudStorageConditionsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `included_bucket_attributes` after provisioning.\nOnly objects with the specified attributes will be scanned. Defaults to [ALL_SUPPORTED_BUCKETS] if unset. Possible values: [\"ALL_SUPPORTED_BUCKETS\", \"AUTOCLASS_DISABLED\", \"AUTOCLASS_ENABLED\"]"] pub fn included_bucket_attributes (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.included_bucket_attributes" , self . base)) } # [doc = "Get a reference to the value of field `included_object_attributes` after provisioning.\nOnly objects with the specified attributes will be scanned. If an object has one of the specified attributes but is inside an excluded bucket, it will not be scanned. Defaults to [ALL_SUPPORTED_OBJECTS]. A profile will be created even if no objects match the included_object_attributes. Possible values: [\"ALL_SUPPORTED_OBJECTS\", \"STANDARD\", \"NEARLINE\", \"COLDLINE\", \"ARCHIVE\", \"REGIONAL\", \"MULTI_REGIONAL\", \"DURABLE_REDUCED_AVAILABILITY\"]"] pub fn included_object_attributes (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.included_object_attributes" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElDynamic { cloud_storage_conditions : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElCloudStorageConditionsEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsEl { # [serde (skip_serializing_if = "Option::is_none")] created_after : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] min_age : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] cloud_storage_conditions : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElCloudStorageConditionsEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsEl {
    #[doc = "Set the field `created_after`.\nFile store must have been created after this date. Used to avoid backfilling. A timestamp in RFC3339 UTC \"Zulu\" format with nanosecond resolution and upto nine fractional digits."]
    pub fn set_created_after(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_after = Some(v.into());
        self
    }
    #[doc = "Set the field `min_age`.\nDuration format. Minimum age a file store must have. If set, the value must be 1 hour or greater."]
    pub fn set_min_age(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_age = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_storage_conditions`.\n"]
    pub fn set_cloud_storage_conditions(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElCloudStorageConditionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_storage_conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_conditions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsEl {
    type O =
        BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsEl {
        DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsEl {
            created_after: core::default::Default::default(),
            min_age: core::default::Default::default(),
            cloud_storage_conditions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElRef {
        DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `created_after` after provisioning.\nFile store must have been created after this date. Used to avoid backfilling. A timestamp in RFC3339 UTC \"Zulu\" format with nanosecond resolution and upto nine fractional digits."]
    pub fn created_after(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_after", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_age` after provisioning.\nDuration format. Minimum age a file store must have. If set, the value must be 1 hour or greater."]
    pub fn min_age(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_age", self.base))
    }
    #[doc = "Get a reference to the value of field `cloud_storage_conditions` after provisioning.\n"]    pub fn cloud_storage_conditions (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElCloudStorageConditionsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_storage_conditions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDisabledEl {}
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDisabledEl {}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDisabledEl {
    type O =
        BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDisabledEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDisabledEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDisabledEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDisabledEl {
        DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDisabledEl {}
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDisabledElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDisabledElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDisabledElRef {
        DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDisabledElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDisabledElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCloudStorageResourceReferenceEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCloudStorageResourceReferenceEl { # [doc = "Set the field `bucket_name`.\nThe bucket to scan."] pub fn set_bucket_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . bucket_name = Some (v . into ()) ; self } # [doc = "Set the field `project_id`.\nIf within a project-level config, then this must match the config's project id."] pub fn set_project_id (mut self , v : impl Into < PrimField < String > >) -> Self { self . project_id = Some (v . into ()) ; self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCloudStorageResourceReferenceEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCloudStorageResourceReferenceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCloudStorageResourceReferenceEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCloudStorageResourceReferenceEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCloudStorageResourceReferenceEl { DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCloudStorageResourceReferenceEl { bucket_name : core :: default :: Default :: default () , project_id : core :: default :: Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCloudStorageResourceReferenceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCloudStorageResourceReferenceElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCloudStorageResourceReferenceElRef { DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCloudStorageResourceReferenceElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCloudStorageResourceReferenceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket_name` after provisioning.\nThe bucket to scan."] pub fn bucket_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bucket_name" , self . base)) } # [doc = "Get a reference to the value of field `project_id` after provisioning.\nIf within a project-level config, then this must match the config's project id."] pub fn project_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.project_id" , self . base)) } }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElCloudStorageRegexEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id_regex: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElCloudStorageRegexEl { # [doc = "Set the field `bucket_name_regex`.\nRegex to test the bucket name against. If empty, all buckets match. Example: \"marketing2021\" or \"(marketing)\\d{4}\" will both match the bucket gs://marketing2021"] pub fn set_bucket_name_regex (mut self , v : impl Into < PrimField < String > >) -> Self { self . bucket_name_regex = Some (v . into ()) ; self } # [doc = "Set the field `project_id_regex`.\nFor organizations, if unset, will match all projects."] pub fn set_project_id_regex (mut self , v : impl Into < PrimField < String > >) -> Self { self . project_id_regex = Some (v . into ()) ; self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElCloudStorageRegexEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElCloudStorageRegexEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElCloudStorageRegexEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElCloudStorageRegexEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElCloudStorageRegexEl { DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElCloudStorageRegexEl { bucket_name_regex : core :: default :: Default :: default () , project_id_regex : core :: default :: Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElCloudStorageRegexElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElCloudStorageRegexElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElCloudStorageRegexElRef { DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElCloudStorageRegexElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElCloudStorageRegexElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket_name_regex` after provisioning.\nRegex to test the bucket name against. If empty, all buckets match. Example: \"marketing2021\" or \"(marketing)\\d{4}\" will both match the bucket gs://marketing2021"] pub fn bucket_name_regex (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bucket_name_regex" , self . base)) } # [doc = "Get a reference to the value of field `project_id_regex` after provisioning.\nFor organizations, if unset, will match all projects."] pub fn project_id_regex (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.project_id_regex" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElDynamic { cloud_storage_regex : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElCloudStorageRegexEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsEl { # [serde (skip_serializing_if = "Option::is_none")] cloud_storage_regex : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElCloudStorageRegexEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsEl { # [doc = "Set the field `cloud_storage_regex`.\n"] pub fn set_cloud_storage_regex (mut self , v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElCloudStorageRegexEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . cloud_storage_regex = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . cloud_storage_regex = Some (d) ; } } self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsEl { DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsEl { cloud_storage_regex : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElRef { DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `cloud_storage_regex` after provisioning.\n"] pub fn cloud_storage_regex (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElCloudStorageRegexElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.cloud_storage_regex" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElDynamic { patterns : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesEl { # [serde (skip_serializing_if = "Option::is_none")] patterns : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesEl { # [doc = "Set the field `patterns`.\n"] pub fn set_patterns (mut self , v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . patterns = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . patterns = Some (d) ; } } self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesEl { DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesEl { patterns : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElRef { DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `patterns` after provisioning.\n"] pub fn patterns (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElPatternsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.patterns" , self . base)) } }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElTagFiltersEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    namespaced_tag_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespaced_tag_value: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElTagFiltersEl { # [doc = "Set the field `namespaced_tag_key`.\nThe namespaced name for the tag key. Must be in the format\n'{parent_id}/{tag_key_short_name}', for example, \"123456/sensitive\" for\nan organization parent, or \"my-project/sensitive\" for a project parent."] pub fn set_namespaced_tag_key (mut self , v : impl Into < PrimField < String > >) -> Self { self . namespaced_tag_key = Some (v . into ()) ; self } # [doc = "Set the field `namespaced_tag_value`.\nThe namespaced name for the tag value. Must be in the format\n'{parent_id}/{tag_key_short_name}/{short_name}', for example,\n\"123456/environment/prod\" for an organization parent, or\n\"my-project/environment/prod\" for a project parent."] pub fn set_namespaced_tag_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . namespaced_tag_value = Some (v . into ()) ; self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElTagFiltersEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElTagFiltersEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElTagFiltersEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElTagFiltersEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElTagFiltersEl { DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElTagFiltersEl { namespaced_tag_key : core :: default :: Default :: default () , namespaced_tag_value : core :: default :: Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElTagFiltersElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElTagFiltersElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElTagFiltersElRef { DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElTagFiltersElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElTagFiltersElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `namespaced_tag_key` after provisioning.\nThe namespaced name for the tag key. Must be in the format\n'{parent_id}/{tag_key_short_name}', for example, \"123456/sensitive\" for\nan organization parent, or \"my-project/sensitive\" for a project parent."] pub fn namespaced_tag_key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.namespaced_tag_key" , self . base)) } # [doc = "Get a reference to the value of field `namespaced_tag_value` after provisioning.\nThe namespaced name for the tag value. Must be in the format\n'{parent_id}/{tag_key_short_name}/{short_name}', for example,\n\"123456/environment/prod\" for an organization parent, or\n\"my-project/environment/prod\" for a project parent."] pub fn namespaced_tag_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.namespaced_tag_value" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElDynamic { tag_filters : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElTagFiltersEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsEl { # [serde (skip_serializing_if = "Option::is_none")] tag_filters : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElTagFiltersEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElDynamic , }
impl
    DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsEl
{
    #[doc = "Set the field `tag_filters`.\n"]
    pub fn set_tag_filters(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElTagFiltersEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_filters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_filters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsEl { DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsEl { tag_filters : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElRef { DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `tag_filters` after provisioning.\n"] pub fn tag_filters (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElTagFiltersElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.tag_filters" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElDynamic { include_regexes : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesEl >> , include_tags : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionEl { # [serde (skip_serializing_if = "Option::is_none")] include_regexes : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesEl > > , # [serde (skip_serializing_if = "Option::is_none")] include_tags : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionEl {
    #[doc = "Set the field `include_regexes`.\n"]
    pub fn set_include_regexes(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.include_regexes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.include_regexes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `include_tags`.\n"]
    pub fn set_include_tags(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.include_tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.include_tags = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionEl {
        DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionEl {
            include_regexes: core::default::Default::default(),
            include_tags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElRef {
        DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `include_regexes` after provisioning.\n"]    pub fn include_regexes (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeRegexesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_regexes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_tags` after provisioning.\n"]    pub fn include_tags (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElIncludeTagsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.include_tags", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElOthersEl {}
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElOthersEl {}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElOthersEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElOthersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElOthersEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElOthersEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElOthersEl {
        DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElOthersEl {}
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElOthersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElOthersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElOthersElRef {
        DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElOthersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElOthersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElDynamic { cloud_storage_resource_reference : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCloudStorageResourceReferenceEl >> , collection : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionEl >> , others : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElOthersEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterEl { # [serde (skip_serializing_if = "Option::is_none")] cloud_storage_resource_reference : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCloudStorageResourceReferenceEl > > , # [serde (skip_serializing_if = "Option::is_none")] collection : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionEl > > , # [serde (skip_serializing_if = "Option::is_none")] others : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElOthersEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterEl {
    #[doc = "Set the field `cloud_storage_resource_reference`.\n"]
    pub fn set_cloud_storage_resource_reference(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCloudStorageResourceReferenceEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_storage_resource_reference = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_resource_reference = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `collection`.\n"]
    pub fn set_collection(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.collection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.collection = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `others`.\n"]
    pub fn set_others(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElOthersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.others = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.others = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterEl {
    type O =
        BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterEl {
        DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterEl {
            cloud_storage_resource_reference: core::default::Default::default(),
            collection: core::default::Default::default(),
            others: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElRef {
        DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_storage_resource_reference` after provisioning.\n"]    pub fn cloud_storage_resource_reference (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCloudStorageResourceReferenceElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_storage_resource_reference", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `collection` after provisioning.\n"]
    pub fn collection(
        &self,
    ) -> ListRef<
        DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElCollectionElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.collection", self.base))
    }
    #[doc = "Get a reference to the value of field `others` after provisioning.\n"]
    pub fn others(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElOthersElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.others", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl { # [doc = "Set the field `frequency`.\nHow frequently data profiles can be updated when the template is modified. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"] pub fn set_frequency (mut self , v : impl Into < PrimField < String > >) -> Self { self . frequency = Some (v . into ()) ; self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl { DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl { frequency : core :: default :: Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElInspectTemplateModifiedCadenceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElInspectTemplateModifiedCadenceElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElInspectTemplateModifiedCadenceElRef { DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElInspectTemplateModifiedCadenceElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElInspectTemplateModifiedCadenceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `frequency` after provisioning.\nHow frequently data profiles can be updated when the template is modified. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"] pub fn frequency (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.frequency" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElDynamic { inspect_template_modified_cadence : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceEl { # [serde (skip_serializing_if = "Option::is_none")] refresh_frequency : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] inspect_template_modified_cadence : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceEl {
    #[doc = "Set the field `refresh_frequency`.\nData changes in Cloud Storage can't trigger reprofiling. If you set this field, profiles are refreshed at this frequency regardless of whether the underlying buckets have changes. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn set_refresh_frequency(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.refresh_frequency = Some(v.into());
        self
    }
    #[doc = "Set the field `inspect_template_modified_cadence`.\n"]
    pub fn set_inspect_template_modified_cadence(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inspect_template_modified_cadence = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inspect_template_modified_cadence = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceEl {
}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceEl {
        DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceEl {
            refresh_frequency: core::default::Default::default(),
            inspect_template_modified_cadence: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElRef {
        DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `refresh_frequency` after provisioning.\nData changes in Cloud Storage can't trigger reprofiling. If you set this field, profiles are refreshed at this frequency regardless of whether the underlying buckets have changes. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn refresh_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.refresh_frequency", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inspect_template_modified_cadence` after provisioning.\n"]    pub fn inspect_template_modified_cadence (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElInspectTemplateModifiedCadenceElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.inspect_template_modified_cadence", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDynamic {
    conditions: Option<
        DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsEl>,
    >,
    disabled: Option<
        DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDisabledEl>,
    >,
    filter: Option<
        DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterEl>,
    >,
    generation_cadence: Option<
        DynamicBlock<
            DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions:
        Option<Vec<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<Vec<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDisabledEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_cadence: Option<
        Vec<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceEl>,
    >,
    dynamic: DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDynamic,
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetEl {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDisabledEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.disabled = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.disabled = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl Into<
            BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `generation_cadence`.\n"]
    pub fn set_generation_cadence(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.generation_cadence = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.generation_cadence = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetEl {
        DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetEl {
            conditions: core::default::Default::default(),
            disabled: core::default::Default::default(),
            filter: core::default::Default::default(),
            generation_cadence: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElRef {
        DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElConditionsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElDisabledElRef> {
        ListRef::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `generation_cadence` after provisioning.\n"]
    pub fn generation_cadence(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElGenerationCadenceElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.generation_cadence", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElAmazonS3BucketConditionsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_storage_classes: Option<ListField<PrimField<String>>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElAmazonS3BucketConditionsEl { # [doc = "Set the field `bucket_types`.\nBucket types that should be profiled. Optional. Defaults to TYPE_ALL_SUPPORTED if unspecified. Possible values: [\"TYPE_ALL_SUPPORTED\", \"TYPE_GENERAL_PURPOSE\"]"] pub fn set_bucket_types (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . bucket_types = Some (v . into ()) ; self } # [doc = "Set the field `object_storage_classes`.\nObject classes that should be profiled. Optional. Defaults to ALL_SUPPORTED_CLASSES if unspecified. Possible values: [\"ALL_SUPPORTED_CLASSES\", \"STANDARD\", \"STANDARD_INFREQUENT_ACCESS\", \"GLACIER_INSTANT_RETRIEVAL\", \"INTELLIGENT_TIERING\"]"] pub fn set_object_storage_classes (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . object_storage_classes = Some (v . into ()) ; self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElAmazonS3BucketConditionsEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElAmazonS3BucketConditionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElAmazonS3BucketConditionsEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElAmazonS3BucketConditionsEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElAmazonS3BucketConditionsEl { DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElAmazonS3BucketConditionsEl { bucket_types : core :: default :: Default :: default () , object_storage_classes : core :: default :: Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElAmazonS3BucketConditionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElAmazonS3BucketConditionsElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElAmazonS3BucketConditionsElRef { DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElAmazonS3BucketConditionsElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElAmazonS3BucketConditionsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket_types` after provisioning.\nBucket types that should be profiled. Optional. Defaults to TYPE_ALL_SUPPORTED if unspecified. Possible values: [\"TYPE_ALL_SUPPORTED\", \"TYPE_GENERAL_PURPOSE\"]"] pub fn bucket_types (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.bucket_types" , self . base)) } # [doc = "Get a reference to the value of field `object_storage_classes` after provisioning.\nObject classes that should be profiled. Optional. Defaults to ALL_SUPPORTED_CLASSES if unspecified. Possible values: [\"ALL_SUPPORTED_CLASSES\", \"STANDARD\", \"STANDARD_INFREQUENT_ACCESS\", \"GLACIER_INSTANT_RETRIEVAL\", \"INTELLIGENT_TIERING\"]"] pub fn object_storage_classes (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.object_storage_classes" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElDynamic { amazon_s3_bucket_conditions : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElAmazonS3BucketConditionsEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsEl { # [serde (skip_serializing_if = "Option::is_none")] min_age : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] amazon_s3_bucket_conditions : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElAmazonS3BucketConditionsEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsEl {
    #[doc = "Set the field `min_age`.\nDuration format.  Minimum age a resource must be before a profile can be generated. Value must be 1 hour or greater. Minimum age is not supported for Azure Blob Storage containers."]
    pub fn set_min_age(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_age = Some(v.into());
        self
    }
    #[doc = "Set the field `amazon_s3_bucket_conditions`.\n"]
    pub fn set_amazon_s3_bucket_conditions(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElAmazonS3BucketConditionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.amazon_s3_bucket_conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_s3_bucket_conditions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsEl {
    type O =
        BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsEl {
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsEl {
            min_age: core::default::Default::default(),
            amazon_s3_bucket_conditions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElRef {
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `min_age` after provisioning.\nDuration format.  Minimum age a resource must be before a profile can be generated. Value must be 1 hour or greater. Minimum age is not supported for Azure Blob Storage containers."]
    pub fn min_age(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_age", self.base))
    }
    #[doc = "Get a reference to the value of field `amazon_s3_bucket_conditions` after provisioning.\n"]    pub fn amazon_s3_bucket_conditions (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElAmazonS3BucketConditionsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.amazon_s3_bucket_conditions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDataSourceTypeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDataSourceTypeEl {
    #[doc = "Set the field `data_source`.\n"]
    pub fn set_data_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_source = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDataSourceTypeEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDataSourceTypeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDataSourceTypeEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDataSourceTypeEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDataSourceTypeEl {
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDataSourceTypeEl {
            data_source: core::default::Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDataSourceTypeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDataSourceTypeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDataSourceTypeElRef {
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDataSourceTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDataSourceTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDisabledEl {}
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDisabledEl {}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDisabledEl {
    type O =
        BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDisabledEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDisabledEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDisabledEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDisabledEl {
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDisabledEl {}
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDisabledElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDisabledElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDisabledElRef {
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDisabledElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDisabledElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElAwsAccountRegexEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id_regex: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElAwsAccountRegexEl { # [doc = "Set the field `account_id_regex`.\nRegex to test the AWS account ID against. If empty, all accounts match. Example: arn:aws:organizations::123:account/o-b2c3d4/345"] pub fn set_account_id_regex (mut self , v : impl Into < PrimField < String > >) -> Self { self . account_id_regex = Some (v . into ()) ; self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElAwsAccountRegexEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElAwsAccountRegexEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElAwsAccountRegexEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElAwsAccountRegexEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElAwsAccountRegexEl { DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElAwsAccountRegexEl { account_id_regex : core :: default :: Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElAwsAccountRegexElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElAwsAccountRegexElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElAwsAccountRegexElRef { DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElAwsAccountRegexElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElAwsAccountRegexElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `account_id_regex` after provisioning.\nRegex to test the AWS account ID against. If empty, all accounts match. Example: arn:aws:organizations::123:account/o-b2c3d4/345"] pub fn account_id_regex (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.account_id_regex" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElDynamic { aws_account_regex : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElAwsAccountRegexEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexEl { # [serde (skip_serializing_if = "Option::is_none")] bucket_name_regex : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] aws_account_regex : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElAwsAccountRegexEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexEl { # [doc = "Set the field `bucket_name_regex`.\nRegex to test the bucket name against. If empty, all buckets match."] pub fn set_bucket_name_regex (mut self , v : impl Into < PrimField < String > >) -> Self { self . bucket_name_regex = Some (v . into ()) ; self } # [doc = "Set the field `aws_account_regex`.\n"] pub fn set_aws_account_regex (mut self , v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElAwsAccountRegexEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . aws_account_regex = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . aws_account_regex = Some (d) ; } } self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexEl { DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexEl { bucket_name_regex : core :: default :: Default :: default () , aws_account_regex : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElRef { DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket_name_regex` after provisioning.\nRegex to test the bucket name against. If empty, all buckets match."] pub fn bucket_name_regex (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bucket_name_regex" , self . base)) } # [doc = "Get a reference to the value of field `aws_account_regex` after provisioning.\n"] pub fn aws_account_regex (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElAwsAccountRegexElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.aws_account_regex" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElDynamic { amazon_s3_bucket_regex : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsEl { # [serde (skip_serializing_if = "Option::is_none")] amazon_s3_bucket_regex : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsEl { # [doc = "Set the field `amazon_s3_bucket_regex`.\n"] pub fn set_amazon_s3_bucket_regex (mut self , v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . amazon_s3_bucket_regex = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . amazon_s3_bucket_regex = Some (d) ; } } self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsEl { DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsEl { amazon_s3_bucket_regex : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElRef { DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `amazon_s3_bucket_regex` after provisioning.\n"] pub fn amazon_s3_bucket_regex (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElAmazonS3BucketRegexElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.amazon_s3_bucket_regex" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElDynamic { patterns : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesEl { # [serde (skip_serializing_if = "Option::is_none")] patterns : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElDynamic , }
impl
    DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesEl
{
    #[doc = "Set the field `patterns`.\n"]
    pub fn set_patterns(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.patterns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.patterns = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesEl { DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesEl { patterns : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElRef { DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `patterns` after provisioning.\n"] pub fn patterns (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElPatternsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.patterns" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElDynamic { include_regexes : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionEl { # [serde (skip_serializing_if = "Option::is_none")] include_regexes : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionEl {
    #[doc = "Set the field `include_regexes`.\n"]
    pub fn set_include_regexes(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.include_regexes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.include_regexes = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionEl {
}
impl BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionEl {
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionEl {
            include_regexes: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElRef {
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `include_regexes` after provisioning.\n"]    pub fn include_regexes (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElIncludeRegexesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_regexes", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElOthersEl {}
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElOthersEl {}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElOthersEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElOthersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElOthersEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElOthersEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElOthersEl {
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElOthersEl {}
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElOthersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElOthersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElOthersElRef {
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElOthersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElOthersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElAwsAccountEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElAwsAccountEl { # [doc = "Set the field `account_id`.\nAWS account ID."] pub fn set_account_id (mut self , v : impl Into < PrimField < String > >) -> Self { self . account_id = Some (v . into ()) ; self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElAwsAccountEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElAwsAccountEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElAwsAccountEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElAwsAccountEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElAwsAccountEl { DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElAwsAccountEl { account_id : core :: default :: Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElAwsAccountElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElAwsAccountElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElAwsAccountElRef { DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElAwsAccountElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElAwsAccountElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `account_id` after provisioning.\nAWS account ID."] pub fn account_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.account_id" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElDynamic { aws_account : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElAwsAccountEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketEl { # [serde (skip_serializing_if = "Option::is_none")] bucket_name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] aws_account : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElAwsAccountEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketEl { # [doc = "Set the field `bucket_name`.\nThe bucket name."] pub fn set_bucket_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . bucket_name = Some (v . into ()) ; self } # [doc = "Set the field `aws_account`.\n"] pub fn set_aws_account (mut self , v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElAwsAccountEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . aws_account = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . aws_account = Some (d) ; } } self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketEl { DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketEl { bucket_name : core :: default :: Default :: default () , aws_account : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElRef { DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket_name` after provisioning.\nThe bucket name."] pub fn bucket_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bucket_name" , self . base)) } # [doc = "Get a reference to the value of field `aws_account` after provisioning.\n"] pub fn aws_account (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElAwsAccountElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.aws_account" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElDynamic { amazon_s3_bucket : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceEl { # [serde (skip_serializing_if = "Option::is_none")] amazon_s3_bucket : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceEl {
    #[doc = "Set the field `amazon_s3_bucket`.\n"]
    pub fn set_amazon_s3_bucket(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.amazon_s3_bucket = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_s3_bucket = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceEl {
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceEl {
            amazon_s3_bucket: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElRef
    {
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `amazon_s3_bucket` after provisioning.\n"]    pub fn amazon_s3_bucket (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElAmazonS3BucketElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.amazon_s3_bucket", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElDynamic {
    collection: Option<
        DynamicBlock<
            DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionEl,
        >,
    >,
    others: Option<
        DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElOthersEl>,
    >,
    single_resource: Option<
        DynamicBlock<
            DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    collection: Option<
        Vec<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    others:
        Option<Vec<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElOthersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_resource: Option<
        Vec<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceEl>,
    >,
    dynamic: DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElDynamic,
}
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterEl {
    #[doc = "Set the field `collection`.\n"]
    pub fn set_collection(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.collection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.collection = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `others`.\n"]
    pub fn set_others(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElOthersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.others = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.others = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `single_resource`.\n"]
    pub fn set_single_resource(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.single_resource = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.single_resource = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterEl {
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterEl {
            collection: core::default::Default::default(),
            others: core::default::Default::default(),
            single_resource: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElRef {
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `collection` after provisioning.\n"]
    pub fn collection(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElCollectionElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.collection", self.base))
    }
    #[doc = "Get a reference to the value of field `others` after provisioning.\n"]
    pub fn others(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElOthersElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.others", self.base))
    }
    #[doc = "Get a reference to the value of field `single_resource` after provisioning.\n"]
    pub fn single_resource(
        &self,
    ) -> ListRef<
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElSingleResourceElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.single_resource", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl { # [doc = "Set the field `frequency`.\nHow frequently data profiles can be updated when the template is modified. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"] pub fn set_frequency (mut self , v : impl Into < PrimField < String > >) -> Self { self . frequency = Some (v . into ()) ; self } }
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl { type O = BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl
{}
impl BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl { pub fn build (self) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl { DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl { frequency : core :: default :: Default :: default () , } } }
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElInspectTemplateModifiedCadenceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElInspectTemplateModifiedCadenceElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElInspectTemplateModifiedCadenceElRef { DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElInspectTemplateModifiedCadenceElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElInspectTemplateModifiedCadenceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `frequency` after provisioning.\nHow frequently data profiles can be updated when the template is modified. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"] pub fn frequency (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.frequency" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElDynamic { inspect_template_modified_cadence : Option < DynamicBlock < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceEl { # [serde (skip_serializing_if = "Option::is_none")] refresh_frequency : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] inspect_template_modified_cadence : Option < Vec < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl > > , dynamic : DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElDynamic , }
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceEl {
    #[doc = "Set the field `refresh_frequency`.\nFrequency to update profiles regardless of whether the underlying resource has changes. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn set_refresh_frequency(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.refresh_frequency = Some(v.into());
        self
    }
    #[doc = "Set the field `inspect_template_modified_cadence`.\n"]
    pub fn set_inspect_template_modified_cadence(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElInspectTemplateModifiedCadenceEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inspect_template_modified_cadence = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inspect_template_modified_cadence = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceEl
{
    type O = BlockAssignable<
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceEl {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceEl {
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceEl {
            refresh_frequency: core::default::Default::default(),
            inspect_template_modified_cadence: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElRef {
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `refresh_frequency` after provisioning.\nFrequency to update profiles regardless of whether the underlying resource has changes. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn refresh_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.refresh_frequency", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inspect_template_modified_cadence` after provisioning.\n"]    pub fn inspect_template_modified_cadence (& self) -> ListRef < DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElInspectTemplateModifiedCadenceElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.inspect_template_modified_cadence", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDynamic {
    conditions: Option<
        DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsEl>,
    >,
    data_source_type: Option<
        DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDataSourceTypeEl>,
    >,
    disabled: Option<
        DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDisabledEl>,
    >,
    filter:
        Option<DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterEl>>,
    generation_cadence: Option<
        DynamicBlock<
            DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions:
        Option<Vec<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source_type:
        Option<Vec<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDataSourceTypeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<Vec<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDisabledEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_cadence: Option<
        Vec<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceEl>,
    >,
    dynamic: DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDynamic,
}
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetEl {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `data_source_type`.\n"]
    pub fn set_data_source_type(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDataSourceTypeEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_source_type = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_source_type = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(
        mut self,
        v: impl Into<
            BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDisabledEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.disabled = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.disabled = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl Into<
            BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `generation_cadence`.\n"]
    pub fn set_generation_cadence(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.generation_cadence = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.generation_cadence = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetEl {
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetEl {
            conditions: core::default::Default::default(),
            data_source_type: core::default::Default::default(),
            disabled: core::default::Default::default(),
            filter: core::default::Default::default(),
            generation_cadence: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElRef {
        DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElConditionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
    #[doc = "Get a reference to the value of field `data_source_type` after provisioning.\n"]
    pub fn data_source_type(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDataSourceTypeElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_source_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElDisabledElRef> {
        ListRef::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `generation_cadence` after provisioning.\n"]
    pub fn generation_cadence(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElGenerationCadenceElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.generation_cadence", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsElSecretsTargetEl {}
impl DataLossPreventionDiscoveryConfigTargetsElSecretsTargetEl {}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsElSecretsTargetEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElSecretsTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsElSecretsTargetEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsElSecretsTargetEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTargetsElSecretsTargetEl {
        DataLossPreventionDiscoveryConfigTargetsElSecretsTargetEl {}
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElSecretsTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElSecretsTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionDiscoveryConfigTargetsElSecretsTargetElRef {
        DataLossPreventionDiscoveryConfigTargetsElSecretsTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElSecretsTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigTargetsElDynamic {
    big_query_target:
        Option<DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetEl>>,
    cloud_sql_target:
        Option<DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetEl>>,
    cloud_storage_target:
        Option<DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetEl>>,
    other_cloud_target:
        Option<DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetEl>>,
    secrets_target: Option<DynamicBlock<DataLossPreventionDiscoveryConfigTargetsElSecretsTargetEl>>,
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTargetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    big_query_target: Option<Vec<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_sql_target: Option<Vec<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_storage_target:
        Option<Vec<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    other_cloud_target: Option<Vec<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets_target: Option<Vec<DataLossPreventionDiscoveryConfigTargetsElSecretsTargetEl>>,
    dynamic: DataLossPreventionDiscoveryConfigTargetsElDynamic,
}
impl DataLossPreventionDiscoveryConfigTargetsEl {
    #[doc = "Set the field `big_query_target`.\n"]
    pub fn set_big_query_target(
        mut self,
        v: impl Into<BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.big_query_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.big_query_target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cloud_sql_target`.\n"]
    pub fn set_cloud_sql_target(
        mut self,
        v: impl Into<BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_sql_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_sql_target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cloud_storage_target`.\n"]
    pub fn set_cloud_storage_target(
        mut self,
        v: impl Into<BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_storage_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `other_cloud_target`.\n"]
    pub fn set_other_cloud_target(
        mut self,
        v: impl Into<BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.other_cloud_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.other_cloud_target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secrets_target`.\n"]
    pub fn set_secrets_target(
        mut self,
        v: impl Into<BlockAssignable<DataLossPreventionDiscoveryConfigTargetsElSecretsTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secrets_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secrets_target = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionDiscoveryConfigTargetsEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigTargetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTargetsEl {}
impl BuildDataLossPreventionDiscoveryConfigTargetsEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTargetsEl {
        DataLossPreventionDiscoveryConfigTargetsEl {
            big_query_target: core::default::Default::default(),
            cloud_sql_target: core::default::Default::default(),
            cloud_storage_target: core::default::Default::default(),
            other_cloud_target: core::default::Default::default(),
            secrets_target: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTargetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTargetsElRef {
    fn new(shared: StackShared, base: String) -> DataLossPreventionDiscoveryConfigTargetsElRef {
        DataLossPreventionDiscoveryConfigTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTargetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `big_query_target` after provisioning.\n"]
    pub fn big_query_target(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElBigQueryTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.big_query_target", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_sql_target` after provisioning.\n"]
    pub fn cloud_sql_target(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElCloudSqlTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_sql_target", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_storage_target` after provisioning.\n"]
    pub fn cloud_storage_target(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElCloudStorageTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_storage_target", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `other_cloud_target` after provisioning.\n"]
    pub fn other_cloud_target(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElOtherCloudTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.other_cloud_target", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secrets_target` after provisioning.\n"]
    pub fn secrets_target(
        &self,
    ) -> ListRef<DataLossPreventionDiscoveryConfigTargetsElSecretsTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secrets_target", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataLossPreventionDiscoveryConfigTimeoutsEl {
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
impl ToListMappable for DataLossPreventionDiscoveryConfigTimeoutsEl {
    type O = BlockAssignable<DataLossPreventionDiscoveryConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionDiscoveryConfigTimeoutsEl {}
impl BuildDataLossPreventionDiscoveryConfigTimeoutsEl {
    pub fn build(self) -> DataLossPreventionDiscoveryConfigTimeoutsEl {
        DataLossPreventionDiscoveryConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataLossPreventionDiscoveryConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionDiscoveryConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataLossPreventionDiscoveryConfigTimeoutsElRef {
        DataLossPreventionDiscoveryConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionDiscoveryConfigTimeoutsElRef {
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
struct DataLossPreventionDiscoveryConfigDynamic {
    actions: Option<DynamicBlock<DataLossPreventionDiscoveryConfigActionsEl>>,
    org_config: Option<DynamicBlock<DataLossPreventionDiscoveryConfigOrgConfigEl>>,
    other_cloud_starting_location:
        Option<DynamicBlock<DataLossPreventionDiscoveryConfigOtherCloudStartingLocationEl>>,
    targets: Option<DynamicBlock<DataLossPreventionDiscoveryConfigTargetsEl>>,
}
