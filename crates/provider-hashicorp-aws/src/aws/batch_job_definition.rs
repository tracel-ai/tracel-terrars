use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct BatchJobDefinitionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_properties: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deregister_on_new_revision: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecs_properties: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_properties: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_capabilities: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    propagate_tags: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduling_priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eks_properties: Option<Vec<BatchJobDefinitionEksPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_strategy: Option<Vec<BatchJobDefinitionRetryStrategyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Vec<BatchJobDefinitionTimeoutEl>>,
    dynamic: BatchJobDefinitionDynamic,
}

struct BatchJobDefinition_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BatchJobDefinitionData>,
}

#[derive(Clone)]
pub struct BatchJobDefinition(Rc<BatchJobDefinition_>);

impl BatchJobDefinition {
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

    #[doc = "Set the field `container_properties`.\n"]
    pub fn set_container_properties(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().container_properties = Some(v.into());
        self
    }

    #[doc = "Set the field `deregister_on_new_revision`.\n"]
    pub fn set_deregister_on_new_revision(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deregister_on_new_revision = Some(v.into());
        self
    }

    #[doc = "Set the field `ecs_properties`.\n"]
    pub fn set_ecs_properties(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ecs_properties = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `node_properties`.\n"]
    pub fn set_node_properties(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().node_properties = Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().parameters = Some(v.into());
        self
    }

    #[doc = "Set the field `platform_capabilities`.\n"]
    pub fn set_platform_capabilities(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().platform_capabilities = Some(v.into());
        self
    }

    #[doc = "Set the field `propagate_tags`.\n"]
    pub fn set_propagate_tags(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().propagate_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `scheduling_priority`.\n"]
    pub fn set_scheduling_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().scheduling_priority = Some(v.into());
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

    #[doc = "Set the field `eks_properties`.\n"]
    pub fn set_eks_properties(
        self,
        v: impl Into<BlockAssignable<BatchJobDefinitionEksPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().eks_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.eks_properties = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `retry_strategy`.\n"]
    pub fn set_retry_strategy(
        self,
        v: impl Into<BlockAssignable<BatchJobDefinitionRetryStrategyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retry_strategy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retry_strategy = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(self, v: impl Into<BlockAssignable<BatchJobDefinitionTimeoutEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.timeout = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn_prefix` after provisioning.\n"]
    pub fn arn_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.arn_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `container_properties` after provisioning.\n"]
    pub fn container_properties(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deregister_on_new_revision` after provisioning.\n"]
    pub fn deregister_on_new_revision(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deregister_on_new_revision", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ecs_properties` after provisioning.\n"]
    pub fn ecs_properties(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ecs_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `node_properties` after provisioning.\n"]
    pub fn node_properties(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `platform_capabilities` after provisioning.\n"]
    pub fn platform_capabilities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.platform_capabilities", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.propagate_tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scheduling_priority` after provisioning.\n"]
    pub fn scheduling_priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scheduling_priority", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `eks_properties` after provisioning.\n"]
    pub fn eks_properties(&self) -> ListRef<BatchJobDefinitionEksPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eks_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `retry_strategy` after provisioning.\n"]
    pub fn retry_strategy(&self) -> ListRef<BatchJobDefinitionRetryStrategyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_strategy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<BatchJobDefinitionTimeoutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }
}

impl Referable for BatchJobDefinition {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for BatchJobDefinition {}

impl ToListMappable for BatchJobDefinition {
    type O = ListRef<BatchJobDefinitionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BatchJobDefinition_ {
    fn extract_resource_type(&self) -> String {
        "aws_batch_job_definition".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBatchJobDefinition {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBatchJobDefinition {
    pub fn build(self, stack: &mut Stack) -> BatchJobDefinition {
        let out = BatchJobDefinition(Rc::new(BatchJobDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BatchJobDefinitionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                container_properties: core::default::Default::default(),
                deregister_on_new_revision: core::default::Default::default(),
                ecs_properties: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                node_properties: core::default::Default::default(),
                parameters: core::default::Default::default(),
                platform_capabilities: core::default::Default::default(),
                propagate_tags: core::default::Default::default(),
                region: core::default::Default::default(),
                scheduling_priority: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                eks_properties: core::default::Default::default(),
                retry_strategy: core::default::Default::default(),
                timeout: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BatchJobDefinitionRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl BatchJobDefinitionRef {
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

    #[doc = "Get a reference to the value of field `arn_prefix` after provisioning.\n"]
    pub fn arn_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.arn_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `container_properties` after provisioning.\n"]
    pub fn container_properties(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deregister_on_new_revision` after provisioning.\n"]
    pub fn deregister_on_new_revision(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deregister_on_new_revision", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ecs_properties` after provisioning.\n"]
    pub fn ecs_properties(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ecs_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `node_properties` after provisioning.\n"]
    pub fn node_properties(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `platform_capabilities` after provisioning.\n"]
    pub fn platform_capabilities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.platform_capabilities", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.propagate_tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scheduling_priority` after provisioning.\n"]
    pub fn scheduling_priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scheduling_priority", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `eks_properties` after provisioning.\n"]
    pub fn eks_properties(&self) -> ListRef<BatchJobDefinitionEksPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eks_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `retry_strategy` after provisioning.\n"]
    pub fn retry_strategy(&self) -> ListRef<BatchJobDefinitionRetryStrategyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_strategy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<BatchJobDefinitionTimeoutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl {
    name: PrimField<String>,
    value: PrimField<String>,
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl {}

impl ToListMappable for BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl {
    type O = BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl {
    pub fn build(self) -> BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl {
        BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvElRef {
        BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    limits: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requests: Option<RecField<PrimField<String>>>,
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl {
    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.limits = Some(v.into());
        self
    }

    #[doc = "Set the field `requests`.\n"]
    pub fn set_requests(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.requests = Some(v.into());
        self
    }
}

impl ToListMappable for BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl {
    type O =
        BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl {}

impl BuildBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl {
    pub fn build(self) -> BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl {
        BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl {
            limits: core::default::Default::default(),
            requests: core::default::Default::default(),
        }
    }
}

pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesElRef {
        BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc = "Get a reference to the value of field `requests` after provisioning.\n"]
    pub fn requests(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.requests", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    privileged: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only_root_file_system: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_as_group: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_as_non_root: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_as_user: Option<PrimField<f64>>,
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl {
    #[doc = "Set the field `privileged`.\n"]
    pub fn set_privileged(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.privileged = Some(v.into());
        self
    }

    #[doc = "Set the field `read_only_root_file_system`.\n"]
    pub fn set_read_only_root_file_system(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only_root_file_system = Some(v.into());
        self
    }

    #[doc = "Set the field `run_as_group`.\n"]
    pub fn set_run_as_group(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.run_as_group = Some(v.into());
        self
    }

    #[doc = "Set the field `run_as_non_root`.\n"]
    pub fn set_run_as_non_root(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.run_as_non_root = Some(v.into());
        self
    }

    #[doc = "Set the field `run_as_user`.\n"]
    pub fn set_run_as_user(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.run_as_user = Some(v.into());
        self
    }
}

impl ToListMappable
    for BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl
{
    type O = BlockAssignable<
        BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl {}

impl BuildBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl {
    pub fn build(
        self,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl {
        BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl {
            privileged: core::default::Default::default(),
            read_only_root_file_system: core::default::Default::default(),
            run_as_group: core::default::Default::default(),
            run_as_non_root: core::default::Default::default(),
            run_as_user: core::default::Default::default(),
        }
    }
}

pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextElRef {
        BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `privileged` after provisioning.\n"]
    pub fn privileged(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.privileged", self.base))
    }

    #[doc = "Get a reference to the value of field `read_only_root_file_system` after provisioning.\n"]
    pub fn read_only_root_file_system(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_only_root_file_system", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `run_as_group` after provisioning.\n"]
    pub fn run_as_group(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_as_group", self.base))
    }

    #[doc = "Get a reference to the value of field `run_as_non_root` after provisioning.\n"]
    pub fn run_as_non_root(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.run_as_non_root", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `run_as_user` after provisioning.\n"]
    pub fn run_as_user(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_as_user", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl {
    mount_path: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl {
    #[doc = "Set the field `read_only`.\n"]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }
}

impl ToListMappable for BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl {
    type O =
        BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl {
    #[doc = ""]
    pub mount_path: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl {
    pub fn build(
        self,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl {
        BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl {
            mount_path: self.mount_path,
            name: self.name,
            read_only: core::default::Default::default(),
        }
    }
}

pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsElRef {
        BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElDynamic {
    env: Option<DynamicBlock<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl>>,
    resources: Option<
        DynamicBlock<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl>,
    >,
    security_context: Option<
        DynamicBlock<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl>,
    >,
    volume_mounts: Option<
        DynamicBlock<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl>,
    >,
}

#[derive(Serialize)]
pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    image: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_pull_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Vec<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<Vec<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_context:
        Option<Vec<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_mounts:
        Option<Vec<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl>>,
    dynamic: BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElDynamic,
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl {
    #[doc = "Set the field `args`.\n"]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }

    #[doc = "Set the field `command`.\n"]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }

    #[doc = "Set the field `image_pull_policy`.\n"]
    pub fn set_image_pull_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_pull_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v: impl Into<BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.env = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.env = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl Into<
            BlockAssignable<
                BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `security_context`.\n"]
    pub fn set_security_context(
        mut self,
        v: impl Into<
            BlockAssignable<
                BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.security_context = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.security_context = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `volume_mounts`.\n"]
    pub fn set_volume_mounts(
        mut self,
        v: impl Into<
            BlockAssignable<
                BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.volume_mounts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.volume_mounts = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl {
    type O = BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl {
    #[doc = ""]
    pub image: PrimField<String>,
}

impl BuildBatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl {
    pub fn build(self) -> BatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl {
        BatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            image: self.image,
            image_pull_policy: core::default::Default::default(),
            name: core::default::Default::default(),
            env: core::default::Default::default(),
            resources: core::default::Default::default(),
            security_context: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElRef {
        BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `image_pull_policy` after provisioning.\n"]
    pub fn image_pull_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_pull_policy", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(
        &self,
    ) -> ListRef<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `security_context` after provisioning.\n"]
    pub fn security_context(
        &self,
    ) -> ListRef<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(
        &self,
    ) -> ListRef<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.volume_mounts", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretEl {
    name: PrimField<String>,
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretEl {}

impl ToListMappable for BatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretEl {
    type O = BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretEl {
    pub fn build(self) -> BatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretEl {
        BatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretEl { name: self.name }
    }
}

pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretElRef {
        BatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl {
    name: PrimField<String>,
    value: PrimField<String>,
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl {}

impl ToListMappable for BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl {
    type O = BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl {
    pub fn build(self) -> BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl {
        BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvElRef {
        BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    limits: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requests: Option<RecField<PrimField<String>>>,
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl {
    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.limits = Some(v.into());
        self
    }

    #[doc = "Set the field `requests`.\n"]
    pub fn set_requests(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.requests = Some(v.into());
        self
    }
}

impl ToListMappable
    for BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl
{
    type O = BlockAssignable<
        BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl {}

impl BuildBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl {
    pub fn build(
        self,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl {
        BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl {
            limits: core::default::Default::default(),
            requests: core::default::Default::default(),
        }
    }
}

pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesElRef {
        BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc = "Get a reference to the value of field `requests` after provisioning.\n"]
    pub fn requests(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.requests", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    privileged: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only_root_file_system: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_as_group: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_as_non_root: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_as_user: Option<PrimField<f64>>,
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl {
    #[doc = "Set the field `privileged`.\n"]
    pub fn set_privileged(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.privileged = Some(v.into());
        self
    }

    #[doc = "Set the field `read_only_root_file_system`.\n"]
    pub fn set_read_only_root_file_system(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only_root_file_system = Some(v.into());
        self
    }

    #[doc = "Set the field `run_as_group`.\n"]
    pub fn set_run_as_group(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.run_as_group = Some(v.into());
        self
    }

    #[doc = "Set the field `run_as_non_root`.\n"]
    pub fn set_run_as_non_root(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.run_as_non_root = Some(v.into());
        self
    }

    #[doc = "Set the field `run_as_user`.\n"]
    pub fn set_run_as_user(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.run_as_user = Some(v.into());
        self
    }
}

impl ToListMappable
    for BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl
{
    type O = BlockAssignable<
        BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl {
}

impl BuildBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl {
    pub fn build(
        self,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl {
        BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl {
            privileged: core::default::Default::default(),
            read_only_root_file_system: core::default::Default::default(),
            run_as_group: core::default::Default::default(),
            run_as_non_root: core::default::Default::default(),
            run_as_user: core::default::Default::default(),
        }
    }
}

pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextElRef {
        BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `privileged` after provisioning.\n"]
    pub fn privileged(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.privileged", self.base))
    }

    #[doc = "Get a reference to the value of field `read_only_root_file_system` after provisioning.\n"]
    pub fn read_only_root_file_system(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_only_root_file_system", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `run_as_group` after provisioning.\n"]
    pub fn run_as_group(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_as_group", self.base))
    }

    #[doc = "Get a reference to the value of field `run_as_non_root` after provisioning.\n"]
    pub fn run_as_non_root(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.run_as_non_root", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `run_as_user` after provisioning.\n"]
    pub fn run_as_user(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_as_user", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl {
    mount_path: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl {
    #[doc = "Set the field `read_only`.\n"]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }
}

impl ToListMappable
    for BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl
{
    type O = BlockAssignable<
        BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl {
    #[doc = ""]
    pub mount_path: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl {
    pub fn build(
        self,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl {
        BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl {
            mount_path: self.mount_path,
            name: self.name,
            read_only: core::default::Default::default(),
        }
    }
}

pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsElRef {
        BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElDynamic {
    env:
        Option<DynamicBlock<BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl>>,
    resources: Option<
        DynamicBlock<BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl>,
    >,
    security_context: Option<
        DynamicBlock<
            BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl,
        >,
    >,
    volume_mounts: Option<
        DynamicBlock<
            BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    image: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_pull_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Vec<BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources:
        Option<Vec<BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_context: Option<
        Vec<BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_mounts:
        Option<Vec<BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl>>,
    dynamic: BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElDynamic,
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl {
    #[doc = "Set the field `args`.\n"]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }

    #[doc = "Set the field `command`.\n"]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }

    #[doc = "Set the field `image_pull_policy`.\n"]
    pub fn set_image_pull_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_pull_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v: impl Into<
            BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.env = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.env = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl Into<
            BlockAssignable<
                BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `security_context`.\n"]
    pub fn set_security_context(
        mut self,
        v: impl Into<
            BlockAssignable<
                BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.security_context = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.security_context = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `volume_mounts`.\n"]
    pub fn set_volume_mounts(
        mut self,
        v: impl Into<
            BlockAssignable<
                BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.volume_mounts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.volume_mounts = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl {
    type O = BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl {
    #[doc = ""]
    pub image: PrimField<String>,
}

impl BuildBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl {
    pub fn build(self) -> BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl {
        BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            image: self.image,
            image_pull_policy: core::default::Default::default(),
            name: core::default::Default::default(),
            env: core::default::Default::default(),
            resources: core::default::Default::default(),
            security_context: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElRef {
        BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `image_pull_policy` after provisioning.\n"]
    pub fn image_pull_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_pull_policy", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(
        &self,
    ) -> ListRef<BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `security_context` after provisioning.\n"]
    pub fn security_context(
        &self,
    ) -> ListRef<BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(
        &self,
    ) -> ListRef<BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.volume_mounts", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl {
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
}

impl ToListMappable for BatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl {
    type O = BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl {}

impl BuildBatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl {
    pub fn build(self) -> BatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl {
        BatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl {
            labels: core::default::Default::default(),
        }
    }
}

pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElMetadataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionEksPropertiesElPodPropertiesElMetadataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElMetadataElRef {
        BatchJobDefinitionEksPropertiesElPodPropertiesElMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    medium: Option<PrimField<String>>,
    size_limit: PrimField<String>,
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl {
    #[doc = "Set the field `medium`.\n"]
    pub fn set_medium(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.medium = Some(v.into());
        self
    }
}

impl ToListMappable for BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl {
    type O = BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl {
    #[doc = ""]
    pub size_limit: PrimField<String>,
}

impl BuildBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl {
    pub fn build(self) -> BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl {
        BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl {
            medium: core::default::Default::default(),
            size_limit: self.size_limit,
        }
    }
}

pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirElRef {
        BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `medium` after provisioning.\n"]
    pub fn medium(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.medium", self.base))
    }

    #[doc = "Get a reference to the value of field `size_limit` after provisioning.\n"]
    pub fn size_limit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_limit", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl {
    path: PrimField<String>,
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl {}

impl ToListMappable for BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl {
    type O = BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl {
    #[doc = ""]
    pub path: PrimField<String>,
}

impl BuildBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl {
    pub fn build(self) -> BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl {
        BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl { path: self.path }
    }
}

pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathElRef {
        BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    optional: Option<PrimField<bool>>,
    secret_name: PrimField<String>,
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl {
    #[doc = "Set the field `optional`.\n"]
    pub fn set_optional(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.optional = Some(v.into());
        self
    }
}

impl ToListMappable for BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl {
    type O = BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl {
    #[doc = ""]
    pub secret_name: PrimField<String>,
}

impl BuildBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl {
    pub fn build(self) -> BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl {
        BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl {
            optional: core::default::Default::default(),
            secret_name: self.secret_name,
        }
    }
}

pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretElRef {
        BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `optional` after provisioning.\n"]
    pub fn optional(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.optional", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElDynamic {
    empty_dir:
        Option<DynamicBlock<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl>>,
    host_path:
        Option<DynamicBlock<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl>>,
    secret: Option<DynamicBlock<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl>>,
}

#[derive(Serialize)]
pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    empty_dir: Option<Vec<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_path: Option<Vec<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<Vec<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl>>,
    dynamic: BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElDynamic,
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `empty_dir`.\n"]
    pub fn set_empty_dir(
        mut self,
        v: impl Into<
            BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.empty_dir = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.empty_dir = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `host_path`.\n"]
    pub fn set_host_path(
        mut self,
        v: impl Into<
            BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.host_path = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.host_path = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v: impl Into<BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secret = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secret = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl {
    type O = BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl {}

impl BuildBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl {
    pub fn build(self) -> BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl {
        BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl {
            name: core::default::Default::default(),
            empty_dir: core::default::Default::default(),
            host_path: core::default::Default::default(),
            secret: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElRef {
        BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `empty_dir` after provisioning.\n"]
    pub fn empty_dir(
        &self,
    ) -> ListRef<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirElRef> {
        ListRef::new(self.shared().clone(), format!("{}.empty_dir", self.base))
    }

    #[doc = "Get a reference to the value of field `host_path` after provisioning.\n"]
    pub fn host_path(
        &self,
    ) -> ListRef<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathElRef> {
        ListRef::new(self.shared().clone(), format!("{}.host_path", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(
        &self,
    ) -> ListRef<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchJobDefinitionEksPropertiesElPodPropertiesElDynamic {
    containers: Option<DynamicBlock<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl>>,
    image_pull_secret:
        Option<DynamicBlock<BatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretEl>>,
    init_containers:
        Option<DynamicBlock<BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl>>,
    metadata: Option<DynamicBlock<BatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl>>,
    volumes: Option<DynamicBlock<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl>>,
}

#[derive(Serialize)]
pub struct BatchJobDefinitionEksPropertiesElPodPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_network: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_process_namespace: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    containers: Option<Vec<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_pull_secret:
        Option<Vec<BatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    init_containers: Option<Vec<BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Vec<BatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes: Option<Vec<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl>>,
    dynamic: BatchJobDefinitionEksPropertiesElPodPropertiesElDynamic,
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesEl {
    #[doc = "Set the field `dns_policy`.\n"]
    pub fn set_dns_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `host_network`.\n"]
    pub fn set_host_network(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.host_network = Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_name`.\n"]
    pub fn set_service_account_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_name = Some(v.into());
        self
    }

    #[doc = "Set the field `share_process_namespace`.\n"]
    pub fn set_share_process_namespace(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.share_process_namespace = Some(v.into());
        self
    }

    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        mut self,
        v: impl Into<BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.containers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.containers = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `image_pull_secret`.\n"]
    pub fn set_image_pull_secret(
        mut self,
        v: impl Into<BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.image_pull_secret = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.image_pull_secret = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `init_containers`.\n"]
    pub fn set_init_containers(
        mut self,
        v: impl Into<BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.init_containers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.init_containers = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl Into<BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metadata = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl Into<BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.volumes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.volumes = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BatchJobDefinitionEksPropertiesElPodPropertiesEl {
    type O = BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionEksPropertiesElPodPropertiesEl {}

impl BuildBatchJobDefinitionEksPropertiesElPodPropertiesEl {
    pub fn build(self) -> BatchJobDefinitionEksPropertiesElPodPropertiesEl {
        BatchJobDefinitionEksPropertiesElPodPropertiesEl {
            dns_policy: core::default::Default::default(),
            host_network: core::default::Default::default(),
            service_account_name: core::default::Default::default(),
            share_process_namespace: core::default::Default::default(),
            containers: core::default::Default::default(),
            image_pull_secret: core::default::Default::default(),
            init_containers: core::default::Default::default(),
            metadata: core::default::Default::default(),
            volumes: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BatchJobDefinitionEksPropertiesElPodPropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionEksPropertiesElPodPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchJobDefinitionEksPropertiesElPodPropertiesElRef {
        BatchJobDefinitionEksPropertiesElPodPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionEksPropertiesElPodPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_policy` after provisioning.\n"]
    pub fn dns_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `host_network` after provisioning.\n"]
    pub fn host_network(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_network", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_name` after provisioning.\n"]
    pub fn service_account_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `share_process_namespace` after provisioning.\n"]
    pub fn share_process_namespace(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.share_process_namespace", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(
        &self,
    ) -> ListRef<BatchJobDefinitionEksPropertiesElPodPropertiesElContainersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.containers", self.base))
    }

    #[doc = "Get a reference to the value of field `image_pull_secret` after provisioning.\n"]
    pub fn image_pull_secret(
        &self,
    ) -> ListRef<BatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_pull_secret", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `init_containers` after provisioning.\n"]
    pub fn init_containers(
        &self,
    ) -> ListRef<BatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.init_containers", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(
        &self,
    ) -> ListRef<BatchJobDefinitionEksPropertiesElPodPropertiesElMetadataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> ListRef<BatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchJobDefinitionEksPropertiesElDynamic {
    pod_properties: Option<DynamicBlock<BatchJobDefinitionEksPropertiesElPodPropertiesEl>>,
}

#[derive(Serialize)]
pub struct BatchJobDefinitionEksPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_properties: Option<Vec<BatchJobDefinitionEksPropertiesElPodPropertiesEl>>,
    dynamic: BatchJobDefinitionEksPropertiesElDynamic,
}

impl BatchJobDefinitionEksPropertiesEl {
    #[doc = "Set the field `pod_properties`.\n"]
    pub fn set_pod_properties(
        mut self,
        v: impl Into<BlockAssignable<BatchJobDefinitionEksPropertiesElPodPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pod_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pod_properties = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BatchJobDefinitionEksPropertiesEl {
    type O = BlockAssignable<BatchJobDefinitionEksPropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionEksPropertiesEl {}

impl BuildBatchJobDefinitionEksPropertiesEl {
    pub fn build(self) -> BatchJobDefinitionEksPropertiesEl {
        BatchJobDefinitionEksPropertiesEl {
            pod_properties: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BatchJobDefinitionEksPropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionEksPropertiesElRef {
    fn new(shared: StackShared, base: String) -> BatchJobDefinitionEksPropertiesElRef {
        BatchJobDefinitionEksPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionEksPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pod_properties` after provisioning.\n"]
    pub fn pod_properties(&self) -> ListRef<BatchJobDefinitionEksPropertiesElPodPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pod_properties", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_exit_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_status_reason: Option<PrimField<String>>,
}

impl BatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
    #[doc = "Set the field `on_exit_code`.\n"]
    pub fn set_on_exit_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_exit_code = Some(v.into());
        self
    }

    #[doc = "Set the field `on_reason`.\n"]
    pub fn set_on_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_reason = Some(v.into());
        self
    }

    #[doc = "Set the field `on_status_reason`.\n"]
    pub fn set_on_status_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_status_reason = Some(v.into());
        self
    }
}

impl ToListMappable for BatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
    type O = BlockAssignable<BatchJobDefinitionRetryStrategyElEvaluateOnExitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
    #[doc = ""]
    pub action: PrimField<String>,
}

impl BuildBatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
    pub fn build(self) -> BatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
        BatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
            action: self.action,
            on_exit_code: core::default::Default::default(),
            on_reason: core::default::Default::default(),
            on_status_reason: core::default::Default::default(),
        }
    }
}

pub struct BatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
        BatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `on_exit_code` after provisioning.\n"]
    pub fn on_exit_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_exit_code", self.base))
    }

    #[doc = "Get a reference to the value of field `on_reason` after provisioning.\n"]
    pub fn on_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `on_status_reason` after provisioning.\n"]
    pub fn on_status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_status_reason", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct BatchJobDefinitionRetryStrategyElDynamic {
    evaluate_on_exit: Option<DynamicBlock<BatchJobDefinitionRetryStrategyElEvaluateOnExitEl>>,
}

#[derive(Serialize)]
pub struct BatchJobDefinitionRetryStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attempts: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluate_on_exit: Option<Vec<BatchJobDefinitionRetryStrategyElEvaluateOnExitEl>>,
    dynamic: BatchJobDefinitionRetryStrategyElDynamic,
}

impl BatchJobDefinitionRetryStrategyEl {
    #[doc = "Set the field `attempts`.\n"]
    pub fn set_attempts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.attempts = Some(v.into());
        self
    }

    #[doc = "Set the field `evaluate_on_exit`.\n"]
    pub fn set_evaluate_on_exit(
        mut self,
        v: impl Into<BlockAssignable<BatchJobDefinitionRetryStrategyElEvaluateOnExitEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.evaluate_on_exit = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.evaluate_on_exit = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BatchJobDefinitionRetryStrategyEl {
    type O = BlockAssignable<BatchJobDefinitionRetryStrategyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionRetryStrategyEl {}

impl BuildBatchJobDefinitionRetryStrategyEl {
    pub fn build(self) -> BatchJobDefinitionRetryStrategyEl {
        BatchJobDefinitionRetryStrategyEl {
            attempts: core::default::Default::default(),
            evaluate_on_exit: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BatchJobDefinitionRetryStrategyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionRetryStrategyElRef {
    fn new(shared: StackShared, base: String) -> BatchJobDefinitionRetryStrategyElRef {
        BatchJobDefinitionRetryStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionRetryStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attempts` after provisioning.\n"]
    pub fn attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.attempts", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate_on_exit` after provisioning.\n"]
    pub fn evaluate_on_exit(
        &self,
    ) -> ListRef<BatchJobDefinitionRetryStrategyElEvaluateOnExitElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.evaluate_on_exit", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attempt_duration_seconds: Option<PrimField<f64>>,
}

impl BatchJobDefinitionTimeoutEl {
    #[doc = "Set the field `attempt_duration_seconds`.\n"]
    pub fn set_attempt_duration_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.attempt_duration_seconds = Some(v.into());
        self
    }
}

impl ToListMappable for BatchJobDefinitionTimeoutEl {
    type O = BlockAssignable<BatchJobDefinitionTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionTimeoutEl {}

impl BuildBatchJobDefinitionTimeoutEl {
    pub fn build(self) -> BatchJobDefinitionTimeoutEl {
        BatchJobDefinitionTimeoutEl {
            attempt_duration_seconds: core::default::Default::default(),
        }
    }
}

pub struct BatchJobDefinitionTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionTimeoutElRef {
    fn new(shared: StackShared, base: String) -> BatchJobDefinitionTimeoutElRef {
        BatchJobDefinitionTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attempt_duration_seconds` after provisioning.\n"]
    pub fn attempt_duration_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.attempt_duration_seconds", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct BatchJobDefinitionDynamic {
    eks_properties: Option<DynamicBlock<BatchJobDefinitionEksPropertiesEl>>,
    retry_strategy: Option<DynamicBlock<BatchJobDefinitionRetryStrategyEl>>,
    timeout: Option<DynamicBlock<BatchJobDefinitionTimeoutEl>>,
}
