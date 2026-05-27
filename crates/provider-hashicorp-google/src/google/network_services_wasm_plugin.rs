use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkServicesWasmPluginData {
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
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    main_version_id: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_config: Option<Vec<NetworkServicesWasmPluginLogConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkServicesWasmPluginTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    versions: Option<Vec<NetworkServicesWasmPluginVersionsEl>>,
    dynamic: NetworkServicesWasmPluginDynamic,
}
struct NetworkServicesWasmPlugin_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkServicesWasmPluginData>,
}
#[derive(Clone)]
pub struct NetworkServicesWasmPlugin(Rc<NetworkServicesWasmPlugin_>);
impl NetworkServicesWasmPlugin {
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
    #[doc = "Set the field `description`.\nOptional. A human-readable description of the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nOptional. Set of labels associated with the WasmPlugin resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe location of the traffic extension"]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `log_config`.\n"]
    pub fn set_log_config(
        self,
        v: impl Into<BlockAssignable<NetworkServicesWasmPluginLogConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkServicesWasmPluginTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `versions`.\n"]
    pub fn set_versions(
        self,
        v: impl Into<BlockAssignable<NetworkServicesWasmPluginVersionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().versions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.versions = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp when the resource was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. A human-readable description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Set of labels associated with the WasmPlugin resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the traffic extension"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `main_version_id` after provisioning.\nThe ID of the WasmPluginVersion resource that is the currently serving one. The version referred to must be a child of this WasmPlugin resource and should be listed in the \"versions\" field."]
    pub fn main_version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.main_version_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Name of the WasmPlugin resource."]
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The timestamp when the resource was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `used_by` after provisioning.\nOutput only. List of all extensions that use this WasmPlugin resource."]
    pub fn used_by(&self) -> ListRef<NetworkServicesWasmPluginUsedByElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.used_by", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<NetworkServicesWasmPluginLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesWasmPluginTimeoutsElRef {
        NetworkServicesWasmPluginTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkServicesWasmPlugin {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkServicesWasmPlugin {}
impl ToListMappable for NetworkServicesWasmPlugin {
    type O = ListRef<NetworkServicesWasmPluginRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkServicesWasmPlugin_ {
    fn extract_resource_type(&self) -> String {
        "google_network_services_wasm_plugin".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkServicesWasmPlugin {
    pub tf_id: String,
    #[doc = "The ID of the WasmPluginVersion resource that is the currently serving one. The version referred to must be a child of this WasmPlugin resource and should be listed in the \"versions\" field."]
    pub main_version_id: PrimField<String>,
    #[doc = "Identifier. Name of the WasmPlugin resource."]
    pub name: PrimField<String>,
}
impl BuildNetworkServicesWasmPlugin {
    pub fn build(self, stack: &mut Stack) -> NetworkServicesWasmPlugin {
        let out = NetworkServicesWasmPlugin(Rc::new(NetworkServicesWasmPlugin_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkServicesWasmPluginData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                main_version_id: self.main_version_id,
                name: self.name,
                project: core::default::Default::default(),
                log_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                versions: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkServicesWasmPluginRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesWasmPluginRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkServicesWasmPluginRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp when the resource was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. A human-readable description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Set of labels associated with the WasmPlugin resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the traffic extension"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `main_version_id` after provisioning.\nThe ID of the WasmPluginVersion resource that is the currently serving one. The version referred to must be a child of this WasmPlugin resource and should be listed in the \"versions\" field."]
    pub fn main_version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.main_version_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Name of the WasmPlugin resource."]
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The timestamp when the resource was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `used_by` after provisioning.\nOutput only. List of all extensions that use this WasmPlugin resource."]
    pub fn used_by(&self) -> ListRef<NetworkServicesWasmPluginUsedByElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.used_by", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<NetworkServicesWasmPluginLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesWasmPluginTimeoutsElRef {
        NetworkServicesWasmPluginTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesWasmPluginUsedByEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl NetworkServicesWasmPluginUsedByEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesWasmPluginUsedByEl {
    type O = BlockAssignable<NetworkServicesWasmPluginUsedByEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesWasmPluginUsedByEl {}
impl BuildNetworkServicesWasmPluginUsedByEl {
    pub fn build(self) -> NetworkServicesWasmPluginUsedByEl {
        NetworkServicesWasmPluginUsedByEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesWasmPluginUsedByElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesWasmPluginUsedByElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesWasmPluginUsedByElRef {
        NetworkServicesWasmPluginUsedByElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesWasmPluginUsedByElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesWasmPluginLogConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_log_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate: Option<PrimField<f64>>,
}
impl NetworkServicesWasmPluginLogConfigEl {
    #[doc = "Set the field `enable`.\nOptional. Specifies whether to enable logging for activity by this plugin."]
    pub fn set_enable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable = Some(v.into());
        self
    }
    #[doc = "Set the field `min_log_level`.\nNon-empty default. Specificies the lowest level of the plugin logs that are exported to Cloud Logging. This setting relates to the logs generated by using logging statements in your Wasm code.\nThis field is can be set only if logging is enabled for the plugin.\nIf the field is not provided when logging is enabled, it is set to INFO by default. Possible values: [\"LOG_LEVEL_UNSPECIFIED\", \"TRACE\", \"DEBUG\", \"INFO\", \"WARN\", \"ERROR\", \"CRITICAL\"]"]
    pub fn set_min_log_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_log_level = Some(v.into());
        self
    }
    #[doc = "Set the field `sample_rate`.\nNon-empty default. Configures the sampling rate of activity logs, where 1.0 means all logged activity is reported and 0.0 means no activity is reported.\nA floating point value between 0.0 and 1.0 indicates that a percentage of log messages is stored.\nThe default value when logging is enabled is 1.0. The value of the field must be between 0 and 1 (inclusive).\nThis field can be specified only if logging is enabled for this plugin."]
    pub fn set_sample_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sample_rate = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesWasmPluginLogConfigEl {
    type O = BlockAssignable<NetworkServicesWasmPluginLogConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesWasmPluginLogConfigEl {}
impl BuildNetworkServicesWasmPluginLogConfigEl {
    pub fn build(self) -> NetworkServicesWasmPluginLogConfigEl {
        NetworkServicesWasmPluginLogConfigEl {
            enable: core::default::Default::default(),
            min_log_level: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesWasmPluginLogConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesWasmPluginLogConfigElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesWasmPluginLogConfigElRef {
        NetworkServicesWasmPluginLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesWasmPluginLogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\nOptional. Specifies whether to enable logging for activity by this plugin."]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
    #[doc = "Get a reference to the value of field `min_log_level` after provisioning.\nNon-empty default. Specificies the lowest level of the plugin logs that are exported to Cloud Logging. This setting relates to the logs generated by using logging statements in your Wasm code.\nThis field is can be set only if logging is enabled for the plugin.\nIf the field is not provided when logging is enabled, it is set to INFO by default. Possible values: [\"LOG_LEVEL_UNSPECIFIED\", \"TRACE\", \"DEBUG\", \"INFO\", \"WARN\", \"ERROR\", \"CRITICAL\"]"]
    pub fn min_log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_log_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sample_rate` after provisioning.\nNon-empty default. Configures the sampling rate of activity logs, where 1.0 means all logged activity is reported and 0.0 means no activity is reported.\nA floating point value between 0.0 and 1.0 indicates that a percentage of log messages is stored.\nThe default value when logging is enabled is 1.0. The value of the field must be between 0 and 1 (inclusive).\nThis field can be specified only if logging is enabled for this plugin."]
    pub fn sample_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sample_rate", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesWasmPluginTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkServicesWasmPluginTimeoutsEl {
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
impl ToListMappable for NetworkServicesWasmPluginTimeoutsEl {
    type O = BlockAssignable<NetworkServicesWasmPluginTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesWasmPluginTimeoutsEl {}
impl BuildNetworkServicesWasmPluginTimeoutsEl {
    pub fn build(self) -> NetworkServicesWasmPluginTimeoutsEl {
        NetworkServicesWasmPluginTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesWasmPluginTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesWasmPluginTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesWasmPluginTimeoutsElRef {
        NetworkServicesWasmPluginTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesWasmPluginTimeoutsElRef {
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
#[derive(Serialize)]
pub struct NetworkServicesWasmPluginVersionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plugin_config_data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plugin_config_uri: Option<PrimField<String>>,
    version_name: PrimField<String>,
}
impl NetworkServicesWasmPluginVersionsEl {
    #[doc = "Set the field `description`.\nOptional. A human-readable description of the resource."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `image_uri`.\nOptional. URI of the container image containing the plugin, stored in the Artifact Registry. When a new WasmPluginVersion resource is created, the digest of the container image is saved in the imageDigest field.\nWhen downloading an image, the digest value is used instead of an image tag."]
    pub fn set_image_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nOptional. Set of labels associated with the WasmPlugin resource."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `plugin_config_data`.\nA base64-encoded string containing the configuration for the plugin. The configuration is provided to the plugin at runtime through the ON_CONFIGURE callback.\nWhen a new WasmPluginVersion resource is created, the digest of the contents is saved in the pluginConfigDigest field.\nConflics with pluginConfigUri."]
    pub fn set_plugin_config_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.plugin_config_data = Some(v.into());
        self
    }
    #[doc = "Set the field `plugin_config_uri`.\nURI of the plugin configuration stored in the Artifact Registry. The configuration is provided to the plugin at runtime through the ON_CONFIGURE callback.\nThe container image must contain only a single file with the name plugin.config.\nWhen a new WasmPluginVersion resource is created, the digest of the container image is saved in the pluginConfigDigest field.\nConflics with pluginConfigData."]
    pub fn set_plugin_config_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.plugin_config_uri = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesWasmPluginVersionsEl {
    type O = BlockAssignable<NetworkServicesWasmPluginVersionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesWasmPluginVersionsEl {
    #[doc = ""]
    pub version_name: PrimField<String>,
}
impl BuildNetworkServicesWasmPluginVersionsEl {
    pub fn build(self) -> NetworkServicesWasmPluginVersionsEl {
        NetworkServicesWasmPluginVersionsEl {
            description: core::default::Default::default(),
            image_uri: core::default::Default::default(),
            labels: core::default::Default::default(),
            plugin_config_data: core::default::Default::default(),
            plugin_config_uri: core::default::Default::default(),
            version_name: self.version_name,
        }
    }
}
pub struct NetworkServicesWasmPluginVersionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesWasmPluginVersionsElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesWasmPluginVersionsElRef {
        NetworkServicesWasmPluginVersionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesWasmPluginVersionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp when the resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. A human-readable description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `image_digest` after provisioning.\nOutput only. The resolved digest for the image specified in the image field. The digest is resolved during the creation of WasmPluginVersion resource.\nThis field holds the digest value, regardless of whether a tag or digest was originally specified in the image field."]
    pub fn image_digest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_digest", self.base))
    }
    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\nOptional. URI of the container image containing the plugin, stored in the Artifact Registry. When a new WasmPluginVersion resource is created, the digest of the container image is saved in the imageDigest field.\nWhen downloading an image, the digest value is used instead of an image tag."]
    pub fn image_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Set of labels associated with the WasmPlugin resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `plugin_config_data` after provisioning.\nA base64-encoded string containing the configuration for the plugin. The configuration is provided to the plugin at runtime through the ON_CONFIGURE callback.\nWhen a new WasmPluginVersion resource is created, the digest of the contents is saved in the pluginConfigDigest field.\nConflics with pluginConfigUri."]
    pub fn plugin_config_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plugin_config_data", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `plugin_config_digest` after provisioning.\nOutput only. This field holds the digest (usually checksum) value for the plugin configuration.\nThe value is calculated based on the contents of pluginConfigData or the container image defined by the pluginConfigUri field."]
    pub fn plugin_config_digest(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plugin_config_digest", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `plugin_config_uri` after provisioning.\nURI of the plugin configuration stored in the Artifact Registry. The configuration is provided to the plugin at runtime through the ON_CONFIGURE callback.\nThe container image must contain only a single file with the name plugin.config.\nWhen a new WasmPluginVersion resource is created, the digest of the container image is saved in the pluginConfigDigest field.\nConflics with pluginConfigData."]
    pub fn plugin_config_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plugin_config_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The timestamp when the resource was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `version_name` after provisioning.\n"]
    pub fn version_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesWasmPluginDynamic {
    log_config: Option<DynamicBlock<NetworkServicesWasmPluginLogConfigEl>>,
    versions: Option<DynamicBlock<NetworkServicesWasmPluginVersionsEl>>,
}
