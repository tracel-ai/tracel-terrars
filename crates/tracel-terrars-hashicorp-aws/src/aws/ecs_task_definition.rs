use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcsTaskDefinitionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    container_definitions: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_fault_injection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_role_arn: Option<PrimField<String>>,
    family: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipc_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pid_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requires_compatibilities: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    track_latest: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_storage: Option<Vec<EcsTaskDefinitionEphemeralStorageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_constraints: Option<Vec<EcsTaskDefinitionPlacementConstraintsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_configuration: Option<Vec<EcsTaskDefinitionProxyConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_platform: Option<Vec<EcsTaskDefinitionRuntimePlatformEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume: Option<Vec<EcsTaskDefinitionVolumeEl>>,
    dynamic: EcsTaskDefinitionDynamic,
}

struct EcsTaskDefinition_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EcsTaskDefinitionData>,
}

#[derive(Clone)]
pub struct EcsTaskDefinition(Rc<EcsTaskDefinition_>);

impl EcsTaskDefinition {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cpu = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_fault_injection`.\n"]
    pub fn set_enable_fault_injection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_fault_injection = Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role_arn`.\n"]
    pub fn set_execution_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().execution_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `ipc_mode`.\n"]
    pub fn set_ipc_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ipc_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().memory = Some(v.into());
        self
    }

    #[doc = "Set the field `network_mode`.\n"]
    pub fn set_network_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `pid_mode`.\n"]
    pub fn set_pid_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().pid_mode = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `requires_compatibilities`.\n"]
    pub fn set_requires_compatibilities(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().requires_compatibilities = Some(v.into());
        self
    }

    #[doc = "Set the field `skip_destroy`.\n"]
    pub fn set_skip_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_destroy = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `task_role_arn`.\n"]
    pub fn set_task_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().task_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `track_latest`.\n"]
    pub fn set_track_latest(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().track_latest = Some(v.into());
        self
    }

    #[doc = "Set the field `ephemeral_storage`.\n"]
    pub fn set_ephemeral_storage(self, v: impl Into<BlockAssignable<EcsTaskDefinitionEphemeralStorageEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ephemeral_storage = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ephemeral_storage = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement_constraints`.\n"]
    pub fn set_placement_constraints(
        self,
        v: impl Into<BlockAssignable<EcsTaskDefinitionPlacementConstraintsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placement_constraints = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placement_constraints = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proxy_configuration`.\n"]
    pub fn set_proxy_configuration(self, v: impl Into<BlockAssignable<EcsTaskDefinitionProxyConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().proxy_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.proxy_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `runtime_platform`.\n"]
    pub fn set_runtime_platform(self, v: impl Into<BlockAssignable<EcsTaskDefinitionRuntimePlatformEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().runtime_platform = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.runtime_platform = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `volume`.\n"]
    pub fn set_volume(self, v: impl Into<BlockAssignable<EcsTaskDefinitionVolumeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().volume = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.volume = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn_without_revision` after provisioning.\n"]
    pub fn arn_without_revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn_without_revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_definitions` after provisioning.\n"]
    pub fn container_definitions(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_definitions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_fault_injection` after provisioning.\n"]
    pub fn enable_fault_injection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_fault_injection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `family` after provisioning.\n"]
    pub fn family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipc_mode` after provisioning.\n"]
    pub fn ipc_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipc_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_mode` after provisioning.\n"]
    pub fn network_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pid_mode` after provisioning.\n"]
    pub fn pid_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pid_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requires_compatibilities` after provisioning.\n"]
    pub fn requires_compatibilities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.requires_compatibilities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_destroy` after provisioning.\n"]
    pub fn skip_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_role_arn` after provisioning.\n"]
    pub fn task_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `track_latest` after provisioning.\n"]
    pub fn track_latest(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.track_latest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> ListRef<EcsTaskDefinitionEphemeralStorageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_configuration` after provisioning.\n"]
    pub fn proxy_configuration(&self) -> ListRef<EcsTaskDefinitionProxyConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.proxy_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_platform` after provisioning.\n"]
    pub fn runtime_platform(&self) -> ListRef<EcsTaskDefinitionRuntimePlatformElRef> {
        ListRef::new(self.shared().clone(), format!("{}.runtime_platform", self.extract_ref()))
    }
}

impl Referable for EcsTaskDefinition {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EcsTaskDefinition { }

impl ToListMappable for EcsTaskDefinition {
    type O = ListRef<EcsTaskDefinitionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EcsTaskDefinition_ {
    fn extract_resource_type(&self) -> String {
        "aws_ecs_task_definition".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcsTaskDefinition {
    pub tf_id: String,
    #[doc = ""]
    pub container_definitions: PrimField<String>,
    #[doc = ""]
    pub family: PrimField<String>,
}

impl BuildEcsTaskDefinition {
    pub fn build(self, stack: &mut Stack) -> EcsTaskDefinition {
        let out = EcsTaskDefinition(Rc::new(EcsTaskDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EcsTaskDefinitionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                container_definitions: self.container_definitions,
                cpu: core::default::Default::default(),
                enable_fault_injection: core::default::Default::default(),
                execution_role_arn: core::default::Default::default(),
                family: self.family,
                id: core::default::Default::default(),
                ipc_mode: core::default::Default::default(),
                memory: core::default::Default::default(),
                network_mode: core::default::Default::default(),
                pid_mode: core::default::Default::default(),
                region: core::default::Default::default(),
                requires_compatibilities: core::default::Default::default(),
                skip_destroy: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                task_role_arn: core::default::Default::default(),
                track_latest: core::default::Default::default(),
                ephemeral_storage: core::default::Default::default(),
                placement_constraints: core::default::Default::default(),
                proxy_configuration: core::default::Default::default(),
                runtime_platform: core::default::Default::default(),
                volume: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcsTaskDefinitionRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsTaskDefinitionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl EcsTaskDefinitionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn_without_revision` after provisioning.\n"]
    pub fn arn_without_revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn_without_revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_definitions` after provisioning.\n"]
    pub fn container_definitions(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_definitions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_fault_injection` after provisioning.\n"]
    pub fn enable_fault_injection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_fault_injection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `family` after provisioning.\n"]
    pub fn family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipc_mode` after provisioning.\n"]
    pub fn ipc_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipc_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_mode` after provisioning.\n"]
    pub fn network_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pid_mode` after provisioning.\n"]
    pub fn pid_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pid_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requires_compatibilities` after provisioning.\n"]
    pub fn requires_compatibilities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.requires_compatibilities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_destroy` after provisioning.\n"]
    pub fn skip_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_role_arn` after provisioning.\n"]
    pub fn task_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `track_latest` after provisioning.\n"]
    pub fn track_latest(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.track_latest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> ListRef<EcsTaskDefinitionEphemeralStorageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_configuration` after provisioning.\n"]
    pub fn proxy_configuration(&self) -> ListRef<EcsTaskDefinitionProxyConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.proxy_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_platform` after provisioning.\n"]
    pub fn runtime_platform(&self) -> ListRef<EcsTaskDefinitionRuntimePlatformElRef> {
        ListRef::new(self.shared().clone(), format!("{}.runtime_platform", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionEphemeralStorageEl {
    size_in_gib: PrimField<f64>,
}

impl EcsTaskDefinitionEphemeralStorageEl { }

impl ToListMappable for EcsTaskDefinitionEphemeralStorageEl {
    type O = BlockAssignable<EcsTaskDefinitionEphemeralStorageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionEphemeralStorageEl {
    #[doc = ""]
    pub size_in_gib: PrimField<f64>,
}

impl BuildEcsTaskDefinitionEphemeralStorageEl {
    pub fn build(self) -> EcsTaskDefinitionEphemeralStorageEl {
        EcsTaskDefinitionEphemeralStorageEl { size_in_gib: self.size_in_gib }
    }
}

pub struct EcsTaskDefinitionEphemeralStorageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsTaskDefinitionEphemeralStorageElRef {
    fn new(shared: StackShared, base: String) -> EcsTaskDefinitionEphemeralStorageElRef {
        EcsTaskDefinitionEphemeralStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionEphemeralStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size_in_gib` after provisioning.\n"]
    pub fn size_in_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_in_gib", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionPlacementConstraintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl EcsTaskDefinitionPlacementConstraintsEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }
}

impl ToListMappable for EcsTaskDefinitionPlacementConstraintsEl {
    type O = BlockAssignable<EcsTaskDefinitionPlacementConstraintsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionPlacementConstraintsEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildEcsTaskDefinitionPlacementConstraintsEl {
    pub fn build(self) -> EcsTaskDefinitionPlacementConstraintsEl {
        EcsTaskDefinitionPlacementConstraintsEl {
            expression: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct EcsTaskDefinitionPlacementConstraintsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsTaskDefinitionPlacementConstraintsElRef {
    fn new(shared: StackShared, base: String) -> EcsTaskDefinitionPlacementConstraintsElRef {
        EcsTaskDefinitionPlacementConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionPlacementConstraintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionProxyConfigurationEl {
    container_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl EcsTaskDefinitionProxyConfigurationEl {
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for EcsTaskDefinitionProxyConfigurationEl {
    type O = BlockAssignable<EcsTaskDefinitionProxyConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionProxyConfigurationEl {
    #[doc = ""]
    pub container_name: PrimField<String>,
}

impl BuildEcsTaskDefinitionProxyConfigurationEl {
    pub fn build(self) -> EcsTaskDefinitionProxyConfigurationEl {
        EcsTaskDefinitionProxyConfigurationEl {
            container_name: self.container_name,
            properties: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct EcsTaskDefinitionProxyConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsTaskDefinitionProxyConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EcsTaskDefinitionProxyConfigurationElRef {
        EcsTaskDefinitionProxyConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionProxyConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_name", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionRuntimePlatformEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_architecture: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operating_system_family: Option<PrimField<String>>,
}

impl EcsTaskDefinitionRuntimePlatformEl {
    #[doc = "Set the field `cpu_architecture`.\n"]
    pub fn set_cpu_architecture(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_architecture = Some(v.into());
        self
    }

    #[doc = "Set the field `operating_system_family`.\n"]
    pub fn set_operating_system_family(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operating_system_family = Some(v.into());
        self
    }
}

impl ToListMappable for EcsTaskDefinitionRuntimePlatformEl {
    type O = BlockAssignable<EcsTaskDefinitionRuntimePlatformEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionRuntimePlatformEl {}

impl BuildEcsTaskDefinitionRuntimePlatformEl {
    pub fn build(self) -> EcsTaskDefinitionRuntimePlatformEl {
        EcsTaskDefinitionRuntimePlatformEl {
            cpu_architecture: core::default::Default::default(),
            operating_system_family: core::default::Default::default(),
        }
    }
}

pub struct EcsTaskDefinitionRuntimePlatformElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsTaskDefinitionRuntimePlatformElRef {
    fn new(shared: StackShared, base: String) -> EcsTaskDefinitionRuntimePlatformElRef {
        EcsTaskDefinitionRuntimePlatformElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionRuntimePlatformElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_architecture` after provisioning.\n"]
    pub fn cpu_architecture(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_architecture", self.base))
    }

    #[doc = "Get a reference to the value of field `operating_system_family` after provisioning.\n"]
    pub fn operating_system_family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operating_system_family", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    autoprovision: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    driver: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    driver_opts: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
}

impl EcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {
    #[doc = "Set the field `autoprovision`.\n"]
    pub fn set_autoprovision(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.autoprovision = Some(v.into());
        self
    }

    #[doc = "Set the field `driver`.\n"]
    pub fn set_driver(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.driver = Some(v.into());
        self
    }

    #[doc = "Set the field `driver_opts`.\n"]
    pub fn set_driver_opts(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.driver_opts = Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }
}

impl ToListMappable for EcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {
    type O = BlockAssignable<EcsTaskDefinitionVolumeElDockerVolumeConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {}

impl BuildEcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {
    pub fn build(self) -> EcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {
        EcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {
            autoprovision: core::default::Default::default(),
            driver: core::default::Default::default(),
            driver_opts: core::default::Default::default(),
            labels: core::default::Default::default(),
            scope: core::default::Default::default(),
        }
    }
}

pub struct EcsTaskDefinitionVolumeElDockerVolumeConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsTaskDefinitionVolumeElDockerVolumeConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EcsTaskDefinitionVolumeElDockerVolumeConfigurationElRef {
        EcsTaskDefinitionVolumeElDockerVolumeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionVolumeElDockerVolumeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoprovision` after provisioning.\n"]
    pub fn autoprovision(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.autoprovision", self.base))
    }

    #[doc = "Get a reference to the value of field `driver` after provisioning.\n"]
    pub fn driver(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.driver", self.base))
    }

    #[doc = "Get a reference to the value of field `driver_opts` after provisioning.\n"]
    pub fn driver_opts(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.driver_opts", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_point_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam: Option<PrimField<String>>,
}

impl EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {
    #[doc = "Set the field `access_point_id`.\n"]
    pub fn set_access_point_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_point_id = Some(v.into());
        self
    }

    #[doc = "Set the field `iam`.\n"]
    pub fn set_iam(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.iam = Some(v.into());
        self
    }
}

impl ToListMappable for EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {
    type O = BlockAssignable<EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {}

impl BuildEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {
    pub fn build(self) -> EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {
        EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {
            access_point_id: core::default::Default::default(),
            iam: core::default::Default::default(),
        }
    }
}

pub struct EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigElRef {
        EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_point_id` after provisioning.\n"]
    pub fn access_point_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_point_id", self.base))
    }

    #[doc = "Get a reference to the value of field `iam` after provisioning.\n"]
    pub fn iam(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsTaskDefinitionVolumeElEfsVolumeConfigurationElDynamic {
    authorization_config: Option<
        DynamicBlock<EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
    file_system_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_directory: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_encryption: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_encryption_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_config: Option<Vec<EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl>>,
    dynamic: EcsTaskDefinitionVolumeElEfsVolumeConfigurationElDynamic,
}

impl EcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
    #[doc = "Set the field `root_directory`.\n"]
    pub fn set_root_directory(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.root_directory = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_encryption`.\n"]
    pub fn set_transit_encryption(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transit_encryption = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_encryption_port`.\n"]
    pub fn set_transit_encryption_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.transit_encryption_port = Some(v.into());
        self
    }

    #[doc = "Set the field `authorization_config`.\n"]
    pub fn set_authorization_config(
        mut self,
        v: impl Into<BlockAssignable<EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authorization_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authorization_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
    type O = BlockAssignable<EcsTaskDefinitionVolumeElEfsVolumeConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
    #[doc = ""]
    pub file_system_id: PrimField<String>,
}

impl BuildEcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
    pub fn build(self) -> EcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
        EcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
            file_system_id: self.file_system_id,
            root_directory: core::default::Default::default(),
            transit_encryption: core::default::Default::default(),
            transit_encryption_port: core::default::Default::default(),
            authorization_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EcsTaskDefinitionVolumeElEfsVolumeConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsTaskDefinitionVolumeElEfsVolumeConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EcsTaskDefinitionVolumeElEfsVolumeConfigurationElRef {
        EcsTaskDefinitionVolumeElEfsVolumeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionVolumeElEfsVolumeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_directory", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_encryption` after provisioning.\n"]
    pub fn transit_encryption(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_encryption_port` after provisioning.\n"]
    pub fn transit_encryption_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption_port", self.base))
    }

    #[doc = "Get a reference to the value of field `authorization_config` after provisioning.\n"]
    pub fn authorization_config(
        &self,
    ) -> ListRef<EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authorization_config", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {
    credentials_parameter: PrimField<String>,
    domain: PrimField<String>,
}

impl EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl { }

impl ToListMappable for EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {
    type O =
        BlockAssignable<EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {
    #[doc = ""]
    pub credentials_parameter: PrimField<String>,
    #[doc = ""]
    pub domain: PrimField<String>,
}

impl BuildEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {
    pub fn build(self) -> EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {
        EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {
            credentials_parameter: self.credentials_parameter,
            domain: self.domain,
        }
    }
}

pub struct EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigElRef {
        EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `credentials_parameter` after provisioning.\n"]
    pub fn credentials_parameter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.credentials_parameter", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElDynamic {
    authorization_config: Option<
        DynamicBlock<EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
    file_system_id: PrimField<String>,
    root_directory: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_config: Option<
        Vec<EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl>,
    >,
    dynamic: EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElDynamic,
}

impl EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
    #[doc = "Set the field `authorization_config`.\n"]
    pub fn set_authorization_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authorization_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authorization_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
    type O = BlockAssignable<EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
    #[doc = ""]
    pub file_system_id: PrimField<String>,
    #[doc = ""]
    pub root_directory: PrimField<String>,
}

impl BuildEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
    pub fn build(self) -> EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
        EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
            file_system_id: self.file_system_id,
            root_directory: self.root_directory,
            authorization_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElRef {
        EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_directory", self.base))
    }

    #[doc = "Get a reference to the value of field `authorization_config` after provisioning.\n"]
    pub fn authorization_config(
        &self,
    ) -> ListRef<EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authorization_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsTaskDefinitionVolumeElDynamic {
    docker_volume_configuration: Option<DynamicBlock<EcsTaskDefinitionVolumeElDockerVolumeConfigurationEl>>,
    efs_volume_configuration: Option<DynamicBlock<EcsTaskDefinitionVolumeElEfsVolumeConfigurationEl>>,
    fsx_windows_file_server_volume_configuration: Option<
        DynamicBlock<EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionVolumeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    configure_at_launch: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_path: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    docker_volume_configuration: Option<Vec<EcsTaskDefinitionVolumeElDockerVolumeConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    efs_volume_configuration: Option<Vec<EcsTaskDefinitionVolumeElEfsVolumeConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fsx_windows_file_server_volume_configuration: Option<
        Vec<EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl>,
    >,
    dynamic: EcsTaskDefinitionVolumeElDynamic,
}

impl EcsTaskDefinitionVolumeEl {
    #[doc = "Set the field `configure_at_launch`.\n"]
    pub fn set_configure_at_launch(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.configure_at_launch = Some(v.into());
        self
    }

    #[doc = "Set the field `host_path`.\n"]
    pub fn set_host_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_path = Some(v.into());
        self
    }

    #[doc = "Set the field `docker_volume_configuration`.\n"]
    pub fn set_docker_volume_configuration(
        mut self,
        v: impl Into<BlockAssignable<EcsTaskDefinitionVolumeElDockerVolumeConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.docker_volume_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.docker_volume_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `efs_volume_configuration`.\n"]
    pub fn set_efs_volume_configuration(
        mut self,
        v: impl Into<BlockAssignable<EcsTaskDefinitionVolumeElEfsVolumeConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.efs_volume_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.efs_volume_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fsx_windows_file_server_volume_configuration`.\n"]
    pub fn set_fsx_windows_file_server_volume_configuration(
        mut self,
        v: impl Into<BlockAssignable<EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fsx_windows_file_server_volume_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fsx_windows_file_server_volume_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EcsTaskDefinitionVolumeEl {
    type O = BlockAssignable<EcsTaskDefinitionVolumeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionVolumeEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildEcsTaskDefinitionVolumeEl {
    pub fn build(self) -> EcsTaskDefinitionVolumeEl {
        EcsTaskDefinitionVolumeEl {
            configure_at_launch: core::default::Default::default(),
            host_path: core::default::Default::default(),
            name: self.name,
            docker_volume_configuration: core::default::Default::default(),
            efs_volume_configuration: core::default::Default::default(),
            fsx_windows_file_server_volume_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EcsTaskDefinitionVolumeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsTaskDefinitionVolumeElRef {
    fn new(shared: StackShared, base: String) -> EcsTaskDefinitionVolumeElRef {
        EcsTaskDefinitionVolumeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionVolumeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configure_at_launch` after provisioning.\n"]
    pub fn configure_at_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.configure_at_launch", self.base))
    }

    #[doc = "Get a reference to the value of field `host_path` after provisioning.\n"]
    pub fn host_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_path", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `docker_volume_configuration` after provisioning.\n"]
    pub fn docker_volume_configuration(&self) -> ListRef<EcsTaskDefinitionVolumeElDockerVolumeConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.docker_volume_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `efs_volume_configuration` after provisioning.\n"]
    pub fn efs_volume_configuration(&self) -> ListRef<EcsTaskDefinitionVolumeElEfsVolumeConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.efs_volume_configuration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fsx_windows_file_server_volume_configuration` after provisioning.\n"]
    pub fn fsx_windows_file_server_volume_configuration(
        &self,
    ) -> ListRef<EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fsx_windows_file_server_volume_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsTaskDefinitionDynamic {
    ephemeral_storage: Option<DynamicBlock<EcsTaskDefinitionEphemeralStorageEl>>,
    placement_constraints: Option<DynamicBlock<EcsTaskDefinitionPlacementConstraintsEl>>,
    proxy_configuration: Option<DynamicBlock<EcsTaskDefinitionProxyConfigurationEl>>,
    runtime_platform: Option<DynamicBlock<EcsTaskDefinitionRuntimePlatformEl>>,
    volume: Option<DynamicBlock<EcsTaskDefinitionVolumeEl>>,
}
