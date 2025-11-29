use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEcsTaskDefinitionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    task_definition: PrimField<String>,
}

struct DataEcsTaskDefinition_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEcsTaskDefinitionData>,
}

#[derive(Clone)]
pub struct DataEcsTaskDefinition(Rc<DataEcsTaskDefinition_>);

impl DataEcsTaskDefinition {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
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

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> ListRef<DataEcsTaskDefinitionEphemeralStorageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `placement_constraints` after provisioning.\n"]
    pub fn placement_constraints(&self) -> SetRef<DataEcsTaskDefinitionPlacementConstraintsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.placement_constraints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_configuration` after provisioning.\n"]
    pub fn proxy_configuration(&self) -> ListRef<DataEcsTaskDefinitionProxyConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.proxy_configuration", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `runtime_platform` after provisioning.\n"]
    pub fn runtime_platform(&self) -> ListRef<DataEcsTaskDefinitionRuntimePlatformElRef> {
        ListRef::new(self.shared().clone(), format!("{}.runtime_platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_role_arn` after provisioning.\n"]
    pub fn task_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume` after provisioning.\n"]
    pub fn volume(&self) -> SetRef<DataEcsTaskDefinitionVolumeElRef> {
        SetRef::new(self.shared().clone(), format!("{}.volume", self.extract_ref()))
    }
}

impl Referable for DataEcsTaskDefinition {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataEcsTaskDefinition { }

impl ToListMappable for DataEcsTaskDefinition {
    type O = ListRef<DataEcsTaskDefinitionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEcsTaskDefinition_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ecs_task_definition".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEcsTaskDefinition {
    pub tf_id: String,
    #[doc = ""]
    pub task_definition: PrimField<String>,
}

impl BuildDataEcsTaskDefinition {
    pub fn build(self, stack: &mut Stack) -> DataEcsTaskDefinition {
        let out = DataEcsTaskDefinition(Rc::new(DataEcsTaskDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEcsTaskDefinitionData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                task_definition: self.task_definition,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEcsTaskDefinitionRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskDefinitionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataEcsTaskDefinitionRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> ListRef<DataEcsTaskDefinitionEphemeralStorageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `placement_constraints` after provisioning.\n"]
    pub fn placement_constraints(&self) -> SetRef<DataEcsTaskDefinitionPlacementConstraintsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.placement_constraints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_configuration` after provisioning.\n"]
    pub fn proxy_configuration(&self) -> ListRef<DataEcsTaskDefinitionProxyConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.proxy_configuration", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `runtime_platform` after provisioning.\n"]
    pub fn runtime_platform(&self) -> ListRef<DataEcsTaskDefinitionRuntimePlatformElRef> {
        ListRef::new(self.shared().clone(), format!("{}.runtime_platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_role_arn` after provisioning.\n"]
    pub fn task_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume` after provisioning.\n"]
    pub fn volume(&self) -> SetRef<DataEcsTaskDefinitionVolumeElRef> {
        SetRef::new(self.shared().clone(), format!("{}.volume", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskDefinitionEphemeralStorageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    size_in_gib: Option<PrimField<f64>>,
}

impl DataEcsTaskDefinitionEphemeralStorageEl {
    #[doc = "Set the field `size_in_gib`.\n"]
    pub fn set_size_in_gib(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size_in_gib = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsTaskDefinitionEphemeralStorageEl {
    type O = BlockAssignable<DataEcsTaskDefinitionEphemeralStorageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinitionEphemeralStorageEl {}

impl BuildDataEcsTaskDefinitionEphemeralStorageEl {
    pub fn build(self) -> DataEcsTaskDefinitionEphemeralStorageEl {
        DataEcsTaskDefinitionEphemeralStorageEl { size_in_gib: core::default::Default::default() }
    }
}

pub struct DataEcsTaskDefinitionEphemeralStorageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskDefinitionEphemeralStorageElRef {
    fn new(shared: StackShared, base: String) -> DataEcsTaskDefinitionEphemeralStorageElRef {
        DataEcsTaskDefinitionEphemeralStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinitionEphemeralStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size_in_gib` after provisioning.\n"]
    pub fn size_in_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_in_gib", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskDefinitionPlacementConstraintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataEcsTaskDefinitionPlacementConstraintsEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsTaskDefinitionPlacementConstraintsEl {
    type O = BlockAssignable<DataEcsTaskDefinitionPlacementConstraintsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinitionPlacementConstraintsEl {}

impl BuildDataEcsTaskDefinitionPlacementConstraintsEl {
    pub fn build(self) -> DataEcsTaskDefinitionPlacementConstraintsEl {
        DataEcsTaskDefinitionPlacementConstraintsEl {
            expression: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataEcsTaskDefinitionPlacementConstraintsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskDefinitionPlacementConstraintsElRef {
    fn new(shared: StackShared, base: String) -> DataEcsTaskDefinitionPlacementConstraintsElRef {
        DataEcsTaskDefinitionPlacementConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinitionPlacementConstraintsElRef {
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
pub struct DataEcsTaskDefinitionProxyConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataEcsTaskDefinitionProxyConfigurationEl {
    #[doc = "Set the field `container_name`.\n"]
    pub fn set_container_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_name = Some(v.into());
        self
    }

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

impl ToListMappable for DataEcsTaskDefinitionProxyConfigurationEl {
    type O = BlockAssignable<DataEcsTaskDefinitionProxyConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinitionProxyConfigurationEl {}

impl BuildDataEcsTaskDefinitionProxyConfigurationEl {
    pub fn build(self) -> DataEcsTaskDefinitionProxyConfigurationEl {
        DataEcsTaskDefinitionProxyConfigurationEl {
            container_name: core::default::Default::default(),
            properties: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataEcsTaskDefinitionProxyConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskDefinitionProxyConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataEcsTaskDefinitionProxyConfigurationElRef {
        DataEcsTaskDefinitionProxyConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinitionProxyConfigurationElRef {
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
pub struct DataEcsTaskDefinitionRuntimePlatformEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_architecture: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operating_system_family: Option<PrimField<String>>,
}

impl DataEcsTaskDefinitionRuntimePlatformEl {
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

impl ToListMappable for DataEcsTaskDefinitionRuntimePlatformEl {
    type O = BlockAssignable<DataEcsTaskDefinitionRuntimePlatformEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinitionRuntimePlatformEl {}

impl BuildDataEcsTaskDefinitionRuntimePlatformEl {
    pub fn build(self) -> DataEcsTaskDefinitionRuntimePlatformEl {
        DataEcsTaskDefinitionRuntimePlatformEl {
            cpu_architecture: core::default::Default::default(),
            operating_system_family: core::default::Default::default(),
        }
    }
}

pub struct DataEcsTaskDefinitionRuntimePlatformElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskDefinitionRuntimePlatformElRef {
    fn new(shared: StackShared, base: String) -> DataEcsTaskDefinitionRuntimePlatformElRef {
        DataEcsTaskDefinitionRuntimePlatformElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinitionRuntimePlatformElRef {
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
pub struct DataEcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {
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

impl DataEcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {
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

impl ToListMappable for DataEcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {
    type O = BlockAssignable<DataEcsTaskDefinitionVolumeElDockerVolumeConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {}

impl BuildDataEcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {
    pub fn build(self) -> DataEcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {
        DataEcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {
            autoprovision: core::default::Default::default(),
            driver: core::default::Default::default(),
            driver_opts: core::default::Default::default(),
            labels: core::default::Default::default(),
            scope: core::default::Default::default(),
        }
    }
}

pub struct DataEcsTaskDefinitionVolumeElDockerVolumeConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskDefinitionVolumeElDockerVolumeConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataEcsTaskDefinitionVolumeElDockerVolumeConfigurationElRef {
        DataEcsTaskDefinitionVolumeElDockerVolumeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinitionVolumeElDockerVolumeConfigurationElRef {
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
pub struct DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_point_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam: Option<PrimField<String>>,
}

impl DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {
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

impl ToListMappable for DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {
    type O = BlockAssignable<DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {}

impl BuildDataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {
    pub fn build(self) -> DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {
        DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {
            access_point_id: core::default::Default::default(),
            iam: core::default::Default::default(),
        }
    }
}

pub struct DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigElRef {
        DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigElRef {
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

#[derive(Serialize)]
pub struct DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_config: Option<ListField<DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_directory: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_encryption: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_encryption_port: Option<PrimField<f64>>,
}

impl DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
    #[doc = "Set the field `authorization_config`.\n"]
    pub fn set_authorization_config(
        mut self,
        v: impl Into<ListField<DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl>>,
    ) -> Self {
        self.authorization_config = Some(v.into());
        self
    }

    #[doc = "Set the field `file_system_id`.\n"]
    pub fn set_file_system_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_system_id = Some(v.into());
        self
    }

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
}

impl ToListMappable for DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
    type O = BlockAssignable<DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {}

impl BuildDataEcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
    pub fn build(self) -> DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
        DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
            authorization_config: core::default::Default::default(),
            file_system_id: core::default::Default::default(),
            root_directory: core::default::Default::default(),
            transit_encryption: core::default::Default::default(),
            transit_encryption_port: core::default::Default::default(),
        }
    }
}

pub struct DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElRef {
        DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_config` after provisioning.\n"]
    pub fn authorization_config(
        &self,
    ) -> ListRef<DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authorization_config", self.base))
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
}

#[derive(Serialize)]
pub struct DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials_parameter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
}

impl DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {
    #[doc = "Set the field `credentials_parameter`.\n"]
    pub fn set_credentials_parameter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.credentials_parameter = Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {
    type O =
        BlockAssignable<DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {}

impl BuildDataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {
    pub fn build(
        self,
    ) -> DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {
        DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {
            credentials_parameter: core::default::Default::default(),
            domain: core::default::Default::default(),
        }
    }
}

pub struct DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigElRef {
        DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigElRef {
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

#[derive(Serialize)]
pub struct DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_config: Option<
        ListField<DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_directory: Option<PrimField<String>>,
}

impl DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
    #[doc = "Set the field `authorization_config`.\n"]
    pub fn set_authorization_config(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl,
                        >,
                    >,
    ) -> Self {
        self.authorization_config = Some(v.into());
        self
    }

    #[doc = "Set the field `file_system_id`.\n"]
    pub fn set_file_system_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_system_id = Some(v.into());
        self
    }

    #[doc = "Set the field `root_directory`.\n"]
    pub fn set_root_directory(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.root_directory = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
    type O = BlockAssignable<DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {}

impl BuildDataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
    pub fn build(self) -> DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
        DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
            authorization_config: core::default::Default::default(),
            file_system_id: core::default::Default::default(),
            root_directory: core::default::Default::default(),
        }
    }
}

pub struct DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElRef {
        DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_config` after provisioning.\n"]
    pub fn authorization_config(
        &self,
    ) -> ListRef<DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authorization_config", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_directory", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskDefinitionVolumeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    configure_at_launch: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    docker_volume_configuration: Option<ListField<DataEcsTaskDefinitionVolumeElDockerVolumeConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    efs_volume_configuration: Option<ListField<DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fsx_windows_file_server_volume_configuration: Option<
        ListField<DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataEcsTaskDefinitionVolumeEl {
    #[doc = "Set the field `configure_at_launch`.\n"]
    pub fn set_configure_at_launch(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.configure_at_launch = Some(v.into());
        self
    }

    #[doc = "Set the field `docker_volume_configuration`.\n"]
    pub fn set_docker_volume_configuration(
        mut self,
        v: impl Into<ListField<DataEcsTaskDefinitionVolumeElDockerVolumeConfigurationEl>>,
    ) -> Self {
        self.docker_volume_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `efs_volume_configuration`.\n"]
    pub fn set_efs_volume_configuration(
        mut self,
        v: impl Into<ListField<DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationEl>>,
    ) -> Self {
        self.efs_volume_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `fsx_windows_file_server_volume_configuration`.\n"]
    pub fn set_fsx_windows_file_server_volume_configuration(
        mut self,
        v: impl Into<ListField<DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl>>,
    ) -> Self {
        self.fsx_windows_file_server_volume_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `host_path`.\n"]
    pub fn set_host_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_path = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsTaskDefinitionVolumeEl {
    type O = BlockAssignable<DataEcsTaskDefinitionVolumeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinitionVolumeEl {}

impl BuildDataEcsTaskDefinitionVolumeEl {
    pub fn build(self) -> DataEcsTaskDefinitionVolumeEl {
        DataEcsTaskDefinitionVolumeEl {
            configure_at_launch: core::default::Default::default(),
            docker_volume_configuration: core::default::Default::default(),
            efs_volume_configuration: core::default::Default::default(),
            fsx_windows_file_server_volume_configuration: core::default::Default::default(),
            host_path: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEcsTaskDefinitionVolumeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskDefinitionVolumeElRef {
    fn new(shared: StackShared, base: String) -> DataEcsTaskDefinitionVolumeElRef {
        DataEcsTaskDefinitionVolumeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinitionVolumeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configure_at_launch` after provisioning.\n"]
    pub fn configure_at_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.configure_at_launch", self.base))
    }

    #[doc = "Get a reference to the value of field `docker_volume_configuration` after provisioning.\n"]
    pub fn docker_volume_configuration(&self) -> ListRef<DataEcsTaskDefinitionVolumeElDockerVolumeConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.docker_volume_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `efs_volume_configuration` after provisioning.\n"]
    pub fn efs_volume_configuration(&self) -> ListRef<DataEcsTaskDefinitionVolumeElEfsVolumeConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.efs_volume_configuration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fsx_windows_file_server_volume_configuration` after provisioning.\n"]
    pub fn fsx_windows_file_server_volume_configuration(
        &self,
    ) -> ListRef<DataEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fsx_windows_file_server_volume_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `host_path` after provisioning.\n"]
    pub fn host_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_path", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
