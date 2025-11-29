use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EmrserverlessApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    architecture: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    release_label: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_start_configuration: Option<Vec<EmrserverlessApplicationAutoStartConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_stop_configuration: Option<Vec<EmrserverlessApplicationAutoStopConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_configuration: Option<Vec<EmrserverlessApplicationImageConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_capacity: Option<Vec<EmrserverlessApplicationInitialCapacityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interactive_configuration: Option<Vec<EmrserverlessApplicationInteractiveConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_capacity: Option<Vec<EmrserverlessApplicationMaximumCapacityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring_configuration: Option<Vec<EmrserverlessApplicationMonitoringConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_configuration: Option<Vec<EmrserverlessApplicationNetworkConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_configuration: Option<Vec<EmrserverlessApplicationRuntimeConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduler_configuration: Option<Vec<EmrserverlessApplicationSchedulerConfigurationEl>>,
    dynamic: EmrserverlessApplicationDynamic,
}

struct EmrserverlessApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EmrserverlessApplicationData>,
}

#[derive(Clone)]
pub struct EmrserverlessApplication(Rc<EmrserverlessApplication_>);

impl EmrserverlessApplication {
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

    #[doc = "Set the field `architecture`.\n"]
    pub fn set_architecture(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().architecture = Some(v.into());
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

    #[doc = "Set the field `auto_start_configuration`.\n"]
    pub fn set_auto_start_configuration(
        self,
        v: impl Into<BlockAssignable<EmrserverlessApplicationAutoStartConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_start_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_start_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `auto_stop_configuration`.\n"]
    pub fn set_auto_stop_configuration(
        self,
        v: impl Into<BlockAssignable<EmrserverlessApplicationAutoStopConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_stop_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_stop_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image_configuration`.\n"]
    pub fn set_image_configuration(
        self,
        v: impl Into<BlockAssignable<EmrserverlessApplicationImageConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.image_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `initial_capacity`.\n"]
    pub fn set_initial_capacity(self, v: impl Into<BlockAssignable<EmrserverlessApplicationInitialCapacityEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().initial_capacity = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.initial_capacity = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `interactive_configuration`.\n"]
    pub fn set_interactive_configuration(
        self,
        v: impl Into<BlockAssignable<EmrserverlessApplicationInteractiveConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().interactive_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.interactive_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maximum_capacity`.\n"]
    pub fn set_maximum_capacity(self, v: impl Into<BlockAssignable<EmrserverlessApplicationMaximumCapacityEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maximum_capacity = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maximum_capacity = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monitoring_configuration`.\n"]
    pub fn set_monitoring_configuration(
        self,
        v: impl Into<BlockAssignable<EmrserverlessApplicationMonitoringConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().monitoring_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.monitoring_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl Into<BlockAssignable<EmrserverlessApplicationNetworkConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `runtime_configuration`.\n"]
    pub fn set_runtime_configuration(
        self,
        v: impl Into<BlockAssignable<EmrserverlessApplicationRuntimeConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().runtime_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.runtime_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scheduler_configuration`.\n"]
    pub fn set_scheduler_configuration(
        self,
        v: impl Into<BlockAssignable<EmrserverlessApplicationSchedulerConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scheduler_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scheduler_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_start_configuration` after provisioning.\n"]
    pub fn auto_start_configuration(&self) -> ListRef<EmrserverlessApplicationAutoStartConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_start_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_stop_configuration` after provisioning.\n"]
    pub fn auto_stop_configuration(&self) -> ListRef<EmrserverlessApplicationAutoStopConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_stop_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_configuration` after provisioning.\n"]
    pub fn image_configuration(&self) -> ListRef<EmrserverlessApplicationImageConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interactive_configuration` after provisioning.\n"]
    pub fn interactive_configuration(&self) -> ListRef<EmrserverlessApplicationInteractiveConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.interactive_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_capacity` after provisioning.\n"]
    pub fn maximum_capacity(&self) -> ListRef<EmrserverlessApplicationMaximumCapacityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maximum_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_configuration` after provisioning.\n"]
    pub fn monitoring_configuration(&self) -> ListRef<EmrserverlessApplicationMonitoringConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.monitoring_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<EmrserverlessApplicationNetworkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_configuration` after provisioning.\n"]
    pub fn runtime_configuration(&self) -> ListRef<EmrserverlessApplicationRuntimeConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.runtime_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduler_configuration` after provisioning.\n"]
    pub fn scheduler_configuration(&self) -> ListRef<EmrserverlessApplicationSchedulerConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scheduler_configuration", self.extract_ref()))
    }
}

impl Referable for EmrserverlessApplication {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EmrserverlessApplication { }

impl ToListMappable for EmrserverlessApplication {
    type O = ListRef<EmrserverlessApplicationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EmrserverlessApplication_ {
    fn extract_resource_type(&self) -> String {
        "aws_emrserverless_application".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmrserverlessApplication {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub release_label: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildEmrserverlessApplication {
    pub fn build(self, stack: &mut Stack) -> EmrserverlessApplication {
        let out = EmrserverlessApplication(Rc::new(EmrserverlessApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EmrserverlessApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                architecture: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                release_label: self.release_label,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                auto_start_configuration: core::default::Default::default(),
                auto_stop_configuration: core::default::Default::default(),
                image_configuration: core::default::Default::default(),
                initial_capacity: core::default::Default::default(),
                interactive_configuration: core::default::Default::default(),
                maximum_capacity: core::default::Default::default(),
                monitoring_configuration: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                runtime_configuration: core::default::Default::default(),
                scheduler_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmrserverlessApplicationRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl EmrserverlessApplicationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_start_configuration` after provisioning.\n"]
    pub fn auto_start_configuration(&self) -> ListRef<EmrserverlessApplicationAutoStartConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_start_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_stop_configuration` after provisioning.\n"]
    pub fn auto_stop_configuration(&self) -> ListRef<EmrserverlessApplicationAutoStopConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_stop_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_configuration` after provisioning.\n"]
    pub fn image_configuration(&self) -> ListRef<EmrserverlessApplicationImageConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interactive_configuration` after provisioning.\n"]
    pub fn interactive_configuration(&self) -> ListRef<EmrserverlessApplicationInteractiveConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.interactive_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_capacity` after provisioning.\n"]
    pub fn maximum_capacity(&self) -> ListRef<EmrserverlessApplicationMaximumCapacityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maximum_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_configuration` after provisioning.\n"]
    pub fn monitoring_configuration(&self) -> ListRef<EmrserverlessApplicationMonitoringConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.monitoring_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<EmrserverlessApplicationNetworkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_configuration` after provisioning.\n"]
    pub fn runtime_configuration(&self) -> ListRef<EmrserverlessApplicationRuntimeConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.runtime_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduler_configuration` after provisioning.\n"]
    pub fn scheduler_configuration(&self) -> ListRef<EmrserverlessApplicationSchedulerConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scheduler_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationAutoStartConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl EmrserverlessApplicationAutoStartConfigurationEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for EmrserverlessApplicationAutoStartConfigurationEl {
    type O = BlockAssignable<EmrserverlessApplicationAutoStartConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationAutoStartConfigurationEl {}

impl BuildEmrserverlessApplicationAutoStartConfigurationEl {
    pub fn build(self) -> EmrserverlessApplicationAutoStartConfigurationEl {
        EmrserverlessApplicationAutoStartConfigurationEl { enabled: core::default::Default::default() }
    }
}

pub struct EmrserverlessApplicationAutoStartConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationAutoStartConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EmrserverlessApplicationAutoStartConfigurationElRef {
        EmrserverlessApplicationAutoStartConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationAutoStartConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationAutoStopConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout_minutes: Option<PrimField<f64>>,
}

impl EmrserverlessApplicationAutoStopConfigurationEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `idle_timeout_minutes`.\n"]
    pub fn set_idle_timeout_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.idle_timeout_minutes = Some(v.into());
        self
    }
}

impl ToListMappable for EmrserverlessApplicationAutoStopConfigurationEl {
    type O = BlockAssignable<EmrserverlessApplicationAutoStopConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationAutoStopConfigurationEl {}

impl BuildEmrserverlessApplicationAutoStopConfigurationEl {
    pub fn build(self) -> EmrserverlessApplicationAutoStopConfigurationEl {
        EmrserverlessApplicationAutoStopConfigurationEl {
            enabled: core::default::Default::default(),
            idle_timeout_minutes: core::default::Default::default(),
        }
    }
}

pub struct EmrserverlessApplicationAutoStopConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationAutoStopConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EmrserverlessApplicationAutoStopConfigurationElRef {
        EmrserverlessApplicationAutoStopConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationAutoStopConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `idle_timeout_minutes` after provisioning.\n"]
    pub fn idle_timeout_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationImageConfigurationEl {
    image_uri: PrimField<String>,
}

impl EmrserverlessApplicationImageConfigurationEl { }

impl ToListMappable for EmrserverlessApplicationImageConfigurationEl {
    type O = BlockAssignable<EmrserverlessApplicationImageConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationImageConfigurationEl {
    #[doc = ""]
    pub image_uri: PrimField<String>,
}

impl BuildEmrserverlessApplicationImageConfigurationEl {
    pub fn build(self) -> EmrserverlessApplicationImageConfigurationEl {
        EmrserverlessApplicationImageConfigurationEl { image_uri: self.image_uri }
    }
}

pub struct EmrserverlessApplicationImageConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationImageConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EmrserverlessApplicationImageConfigurationElRef {
        EmrserverlessApplicationImageConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationImageConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\n"]
    pub fn image_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl {
    cpu: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk: Option<PrimField<String>>,
    memory: PrimField<String>,
}

impl EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl {
    #[doc = "Set the field `disk`.\n"]
    pub fn set_disk(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk = Some(v.into());
        self
    }
}

impl ToListMappable for EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl {
    type O = BlockAssignable<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl {
    #[doc = ""]
    pub cpu: PrimField<String>,
    #[doc = ""]
    pub memory: PrimField<String>,
}

impl BuildEmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl {
    pub fn build(self) -> EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl {
        EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl {
            cpu: self.cpu,
            disk: core::default::Default::default(),
            memory: self.memory,
        }
    }
}

pub struct EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationElRef {
        EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElDynamic {
    worker_configuration: Option<
        DynamicBlock<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
    worker_count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_configuration: Option<
        Vec<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl>,
    >,
    dynamic: EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElDynamic,
}

impl EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
    #[doc = "Set the field `worker_configuration`.\n"]
    pub fn set_worker_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.worker_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.worker_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
    type O = BlockAssignable<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
    #[doc = ""]
    pub worker_count: PrimField<f64>,
}

impl BuildEmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
    pub fn build(self) -> EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
        EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
            worker_count: self.worker_count,
            worker_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElRef {
    fn new(shared: StackShared, base: String) -> EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElRef {
        EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `worker_count` after provisioning.\n"]
    pub fn worker_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_count", self.base))
    }

    #[doc = "Get a reference to the value of field `worker_configuration` after provisioning.\n"]
    pub fn worker_configuration(
        &self,
    ) -> ListRef<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.worker_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrserverlessApplicationInitialCapacityElDynamic {
    initial_capacity_config: Option<DynamicBlock<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl>>,
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationInitialCapacityEl {
    initial_capacity_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_capacity_config: Option<Vec<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl>>,
    dynamic: EmrserverlessApplicationInitialCapacityElDynamic,
}

impl EmrserverlessApplicationInitialCapacityEl {
    #[doc = "Set the field `initial_capacity_config`.\n"]
    pub fn set_initial_capacity_config(
        mut self,
        v: impl Into<BlockAssignable<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.initial_capacity_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.initial_capacity_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrserverlessApplicationInitialCapacityEl {
    type O = BlockAssignable<EmrserverlessApplicationInitialCapacityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationInitialCapacityEl {
    #[doc = ""]
    pub initial_capacity_type: PrimField<String>,
}

impl BuildEmrserverlessApplicationInitialCapacityEl {
    pub fn build(self) -> EmrserverlessApplicationInitialCapacityEl {
        EmrserverlessApplicationInitialCapacityEl {
            initial_capacity_type: self.initial_capacity_type,
            initial_capacity_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrserverlessApplicationInitialCapacityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationInitialCapacityElRef {
    fn new(shared: StackShared, base: String) -> EmrserverlessApplicationInitialCapacityElRef {
        EmrserverlessApplicationInitialCapacityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationInitialCapacityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `initial_capacity_type` after provisioning.\n"]
    pub fn initial_capacity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.initial_capacity_type", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_capacity_config` after provisioning.\n"]
    pub fn initial_capacity_config(
        &self,
    ) -> ListRef<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.initial_capacity_config", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationInteractiveConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    livy_endpoint_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    studio_enabled: Option<PrimField<bool>>,
}

impl EmrserverlessApplicationInteractiveConfigurationEl {
    #[doc = "Set the field `livy_endpoint_enabled`.\n"]
    pub fn set_livy_endpoint_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.livy_endpoint_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `studio_enabled`.\n"]
    pub fn set_studio_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.studio_enabled = Some(v.into());
        self
    }
}

impl ToListMappable for EmrserverlessApplicationInteractiveConfigurationEl {
    type O = BlockAssignable<EmrserverlessApplicationInteractiveConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationInteractiveConfigurationEl {}

impl BuildEmrserverlessApplicationInteractiveConfigurationEl {
    pub fn build(self) -> EmrserverlessApplicationInteractiveConfigurationEl {
        EmrserverlessApplicationInteractiveConfigurationEl {
            livy_endpoint_enabled: core::default::Default::default(),
            studio_enabled: core::default::Default::default(),
        }
    }
}

pub struct EmrserverlessApplicationInteractiveConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationInteractiveConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EmrserverlessApplicationInteractiveConfigurationElRef {
        EmrserverlessApplicationInteractiveConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationInteractiveConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `livy_endpoint_enabled` after provisioning.\n"]
    pub fn livy_endpoint_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livy_endpoint_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `studio_enabled` after provisioning.\n"]
    pub fn studio_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.studio_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationMaximumCapacityEl {
    cpu: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk: Option<PrimField<String>>,
    memory: PrimField<String>,
}

impl EmrserverlessApplicationMaximumCapacityEl {
    #[doc = "Set the field `disk`.\n"]
    pub fn set_disk(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk = Some(v.into());
        self
    }
}

impl ToListMappable for EmrserverlessApplicationMaximumCapacityEl {
    type O = BlockAssignable<EmrserverlessApplicationMaximumCapacityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationMaximumCapacityEl {
    #[doc = ""]
    pub cpu: PrimField<String>,
    #[doc = ""]
    pub memory: PrimField<String>,
}

impl BuildEmrserverlessApplicationMaximumCapacityEl {
    pub fn build(self) -> EmrserverlessApplicationMaximumCapacityEl {
        EmrserverlessApplicationMaximumCapacityEl {
            cpu: self.cpu,
            disk: core::default::Default::default(),
            memory: self.memory,
        }
    }
}

pub struct EmrserverlessApplicationMaximumCapacityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationMaximumCapacityElRef {
    fn new(shared: StackShared, base: String) -> EmrserverlessApplicationMaximumCapacityElRef {
        EmrserverlessApplicationMaximumCapacityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationMaximumCapacityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElLogTypesEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElLogTypesEl { }

impl ToListMappable for EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElLogTypesEl {
    type O =
        BlockAssignable<EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElLogTypesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElLogTypesEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildEmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElLogTypesEl {
    pub fn build(
        self,
    ) -> EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElLogTypesEl {
        EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElLogTypesEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElLogTypesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElLogTypesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElLogTypesElRef {
        EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElLogTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElLogTypesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElDynamic {
    log_types: Option<
        DynamicBlock<EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElLogTypesEl>,
    >,
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_stream_name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_types: Option<Vec<EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElLogTypesEl>>,
    dynamic: EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElDynamic,
}

impl EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationEl {
    #[doc = "Set the field `encryption_key_arn`.\n"]
    pub fn set_encryption_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_key_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `log_group_name`.\n"]
    pub fn set_log_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group_name = Some(v.into());
        self
    }

    #[doc = "Set the field `log_stream_name_prefix`.\n"]
    pub fn set_log_stream_name_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_stream_name_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `log_types`.\n"]
    pub fn set_log_types(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElLogTypesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.log_types = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.log_types = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationEl {
    type O = BlockAssignable<EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildEmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationEl {
    pub fn build(self) -> EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationEl {
        EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationEl {
            enabled: self.enabled,
            encryption_key_arn: core::default::Default::default(),
            log_group_name: core::default::Default::default(),
            log_stream_name_prefix: core::default::Default::default(),
            log_types: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElRef {
        EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_key_arn` after provisioning.\n"]
    pub fn encryption_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `log_stream_name_prefix` after provisioning.\n"]
    pub fn log_stream_name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_stream_name_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationMonitoringConfigurationElManagedPersistenceMonitoringConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key_arn: Option<PrimField<String>>,
}

impl EmrserverlessApplicationMonitoringConfigurationElManagedPersistenceMonitoringConfigurationEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_key_arn`.\n"]
    pub fn set_encryption_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_key_arn = Some(v.into());
        self
    }
}

impl ToListMappable for EmrserverlessApplicationMonitoringConfigurationElManagedPersistenceMonitoringConfigurationEl {
    type O =
        BlockAssignable<
            EmrserverlessApplicationMonitoringConfigurationElManagedPersistenceMonitoringConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationMonitoringConfigurationElManagedPersistenceMonitoringConfigurationEl {}

impl BuildEmrserverlessApplicationMonitoringConfigurationElManagedPersistenceMonitoringConfigurationEl {
    pub fn build(
        self,
    ) -> EmrserverlessApplicationMonitoringConfigurationElManagedPersistenceMonitoringConfigurationEl {
        EmrserverlessApplicationMonitoringConfigurationElManagedPersistenceMonitoringConfigurationEl {
            enabled: core::default::Default::default(),
            encryption_key_arn: core::default::Default::default(),
        }
    }
}

pub struct EmrserverlessApplicationMonitoringConfigurationElManagedPersistenceMonitoringConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationMonitoringConfigurationElManagedPersistenceMonitoringConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrserverlessApplicationMonitoringConfigurationElManagedPersistenceMonitoringConfigurationElRef {
        EmrserverlessApplicationMonitoringConfigurationElManagedPersistenceMonitoringConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationMonitoringConfigurationElManagedPersistenceMonitoringConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_key_arn` after provisioning.\n"]
    pub fn encryption_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationMonitoringConfigurationElPrometheusMonitoringConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_write_url: Option<PrimField<String>>,
}

impl EmrserverlessApplicationMonitoringConfigurationElPrometheusMonitoringConfigurationEl {
    #[doc = "Set the field `remote_write_url`.\n"]
    pub fn set_remote_write_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.remote_write_url = Some(v.into());
        self
    }
}

impl ToListMappable for EmrserverlessApplicationMonitoringConfigurationElPrometheusMonitoringConfigurationEl {
    type O = BlockAssignable<EmrserverlessApplicationMonitoringConfigurationElPrometheusMonitoringConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationMonitoringConfigurationElPrometheusMonitoringConfigurationEl {}

impl BuildEmrserverlessApplicationMonitoringConfigurationElPrometheusMonitoringConfigurationEl {
    pub fn build(self) -> EmrserverlessApplicationMonitoringConfigurationElPrometheusMonitoringConfigurationEl {
        EmrserverlessApplicationMonitoringConfigurationElPrometheusMonitoringConfigurationEl {
            remote_write_url: core::default::Default::default(),
        }
    }
}

pub struct EmrserverlessApplicationMonitoringConfigurationElPrometheusMonitoringConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationMonitoringConfigurationElPrometheusMonitoringConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrserverlessApplicationMonitoringConfigurationElPrometheusMonitoringConfigurationElRef {
        EmrserverlessApplicationMonitoringConfigurationElPrometheusMonitoringConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationMonitoringConfigurationElPrometheusMonitoringConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `remote_write_url` after provisioning.\n"]
    pub fn remote_write_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.remote_write_url", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationMonitoringConfigurationElS3MonitoringConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_uri: Option<PrimField<String>>,
}

impl EmrserverlessApplicationMonitoringConfigurationElS3MonitoringConfigurationEl {
    #[doc = "Set the field `encryption_key_arn`.\n"]
    pub fn set_encryption_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_key_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `log_uri`.\n"]
    pub fn set_log_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_uri = Some(v.into());
        self
    }
}

impl ToListMappable for EmrserverlessApplicationMonitoringConfigurationElS3MonitoringConfigurationEl {
    type O = BlockAssignable<EmrserverlessApplicationMonitoringConfigurationElS3MonitoringConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationMonitoringConfigurationElS3MonitoringConfigurationEl {}

impl BuildEmrserverlessApplicationMonitoringConfigurationElS3MonitoringConfigurationEl {
    pub fn build(self) -> EmrserverlessApplicationMonitoringConfigurationElS3MonitoringConfigurationEl {
        EmrserverlessApplicationMonitoringConfigurationElS3MonitoringConfigurationEl {
            encryption_key_arn: core::default::Default::default(),
            log_uri: core::default::Default::default(),
        }
    }
}

pub struct EmrserverlessApplicationMonitoringConfigurationElS3MonitoringConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationMonitoringConfigurationElS3MonitoringConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrserverlessApplicationMonitoringConfigurationElS3MonitoringConfigurationElRef {
        EmrserverlessApplicationMonitoringConfigurationElS3MonitoringConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationMonitoringConfigurationElS3MonitoringConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_key_arn` after provisioning.\n"]
    pub fn encryption_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `log_uri` after provisioning.\n"]
    pub fn log_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrserverlessApplicationMonitoringConfigurationElDynamic {
    cloudwatch_logging_configuration: Option<
        DynamicBlock<EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationEl>,
    >,
    managed_persistence_monitoring_configuration: Option<
        DynamicBlock<EmrserverlessApplicationMonitoringConfigurationElManagedPersistenceMonitoringConfigurationEl>,
    >,
    prometheus_monitoring_configuration: Option<
        DynamicBlock<EmrserverlessApplicationMonitoringConfigurationElPrometheusMonitoringConfigurationEl>,
    >,
    s3_monitoring_configuration: Option<
        DynamicBlock<EmrserverlessApplicationMonitoringConfigurationElS3MonitoringConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationMonitoringConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logging_configuration: Option<
        Vec<EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_persistence_monitoring_configuration: Option<
        Vec<EmrserverlessApplicationMonitoringConfigurationElManagedPersistenceMonitoringConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    prometheus_monitoring_configuration: Option<
        Vec<EmrserverlessApplicationMonitoringConfigurationElPrometheusMonitoringConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_monitoring_configuration: Option<
        Vec<EmrserverlessApplicationMonitoringConfigurationElS3MonitoringConfigurationEl>,
    >,
    dynamic: EmrserverlessApplicationMonitoringConfigurationElDynamic,
}

impl EmrserverlessApplicationMonitoringConfigurationEl {
    #[doc = "Set the field `cloudwatch_logging_configuration`.\n"]
    pub fn set_cloudwatch_logging_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_persistence_monitoring_configuration`.\n"]
    pub fn set_managed_persistence_monitoring_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            EmrserverlessApplicationMonitoringConfigurationElManagedPersistenceMonitoringConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.managed_persistence_monitoring_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_persistence_monitoring_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `prometheus_monitoring_configuration`.\n"]
    pub fn set_prometheus_monitoring_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            EmrserverlessApplicationMonitoringConfigurationElPrometheusMonitoringConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.prometheus_monitoring_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.prometheus_monitoring_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_monitoring_configuration`.\n"]
    pub fn set_s3_monitoring_configuration(
        mut self,
        v: impl Into<BlockAssignable<EmrserverlessApplicationMonitoringConfigurationElS3MonitoringConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_monitoring_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_monitoring_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrserverlessApplicationMonitoringConfigurationEl {
    type O = BlockAssignable<EmrserverlessApplicationMonitoringConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationMonitoringConfigurationEl {}

impl BuildEmrserverlessApplicationMonitoringConfigurationEl {
    pub fn build(self) -> EmrserverlessApplicationMonitoringConfigurationEl {
        EmrserverlessApplicationMonitoringConfigurationEl {
            cloudwatch_logging_configuration: core::default::Default::default(),
            managed_persistence_monitoring_configuration: core::default::Default::default(),
            prometheus_monitoring_configuration: core::default::Default::default(),
            s3_monitoring_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrserverlessApplicationMonitoringConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationMonitoringConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EmrserverlessApplicationMonitoringConfigurationElRef {
        EmrserverlessApplicationMonitoringConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationMonitoringConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_configuration` after provisioning.\n"]
    pub fn cloudwatch_logging_configuration(
        &self,
    ) -> ListRef<EmrserverlessApplicationMonitoringConfigurationElCloudwatchLoggingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_configuration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `managed_persistence_monitoring_configuration` after provisioning.\n"]
    pub fn managed_persistence_monitoring_configuration(
        &self,
    ) -> ListRef<EmrserverlessApplicationMonitoringConfigurationElManagedPersistenceMonitoringConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.managed_persistence_monitoring_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `prometheus_monitoring_configuration` after provisioning.\n"]
    pub fn prometheus_monitoring_configuration(
        &self,
    ) -> ListRef<EmrserverlessApplicationMonitoringConfigurationElPrometheusMonitoringConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.prometheus_monitoring_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_monitoring_configuration` after provisioning.\n"]
    pub fn s3_monitoring_configuration(
        &self,
    ) -> ListRef<EmrserverlessApplicationMonitoringConfigurationElS3MonitoringConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_monitoring_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationNetworkConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
}

impl EmrserverlessApplicationNetworkConfigurationEl {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnet_ids = Some(v.into());
        self
    }
}

impl ToListMappable for EmrserverlessApplicationNetworkConfigurationEl {
    type O = BlockAssignable<EmrserverlessApplicationNetworkConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationNetworkConfigurationEl {}

impl BuildEmrserverlessApplicationNetworkConfigurationEl {
    pub fn build(self) -> EmrserverlessApplicationNetworkConfigurationEl {
        EmrserverlessApplicationNetworkConfigurationEl {
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
        }
    }
}

pub struct EmrserverlessApplicationNetworkConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationNetworkConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EmrserverlessApplicationNetworkConfigurationElRef {
        EmrserverlessApplicationNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationNetworkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationRuntimeConfigurationEl {
    classification: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
}

impl EmrserverlessApplicationRuntimeConfigurationEl {
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
}

impl ToListMappable for EmrserverlessApplicationRuntimeConfigurationEl {
    type O = BlockAssignable<EmrserverlessApplicationRuntimeConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationRuntimeConfigurationEl {
    #[doc = ""]
    pub classification: PrimField<String>,
}

impl BuildEmrserverlessApplicationRuntimeConfigurationEl {
    pub fn build(self) -> EmrserverlessApplicationRuntimeConfigurationEl {
        EmrserverlessApplicationRuntimeConfigurationEl {
            classification: self.classification,
            properties: core::default::Default::default(),
        }
    }
}

pub struct EmrserverlessApplicationRuntimeConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationRuntimeConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EmrserverlessApplicationRuntimeConfigurationElRef {
        EmrserverlessApplicationRuntimeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationRuntimeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `classification` after provisioning.\n"]
    pub fn classification(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.classification", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationSchedulerConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent_runs: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue_timeout_minutes: Option<PrimField<f64>>,
}

impl EmrserverlessApplicationSchedulerConfigurationEl {
    #[doc = "Set the field `max_concurrent_runs`.\n"]
    pub fn set_max_concurrent_runs(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrent_runs = Some(v.into());
        self
    }

    #[doc = "Set the field `queue_timeout_minutes`.\n"]
    pub fn set_queue_timeout_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.queue_timeout_minutes = Some(v.into());
        self
    }
}

impl ToListMappable for EmrserverlessApplicationSchedulerConfigurationEl {
    type O = BlockAssignable<EmrserverlessApplicationSchedulerConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationSchedulerConfigurationEl {}

impl BuildEmrserverlessApplicationSchedulerConfigurationEl {
    pub fn build(self) -> EmrserverlessApplicationSchedulerConfigurationEl {
        EmrserverlessApplicationSchedulerConfigurationEl {
            max_concurrent_runs: core::default::Default::default(),
            queue_timeout_minutes: core::default::Default::default(),
        }
    }
}

pub struct EmrserverlessApplicationSchedulerConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationSchedulerConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EmrserverlessApplicationSchedulerConfigurationElRef {
        EmrserverlessApplicationSchedulerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationSchedulerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_concurrent_runs` after provisioning.\n"]
    pub fn max_concurrent_runs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_runs", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_timeout_minutes` after provisioning.\n"]
    pub fn queue_timeout_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_timeout_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrserverlessApplicationDynamic {
    auto_start_configuration: Option<DynamicBlock<EmrserverlessApplicationAutoStartConfigurationEl>>,
    auto_stop_configuration: Option<DynamicBlock<EmrserverlessApplicationAutoStopConfigurationEl>>,
    image_configuration: Option<DynamicBlock<EmrserverlessApplicationImageConfigurationEl>>,
    initial_capacity: Option<DynamicBlock<EmrserverlessApplicationInitialCapacityEl>>,
    interactive_configuration: Option<DynamicBlock<EmrserverlessApplicationInteractiveConfigurationEl>>,
    maximum_capacity: Option<DynamicBlock<EmrserverlessApplicationMaximumCapacityEl>>,
    monitoring_configuration: Option<DynamicBlock<EmrserverlessApplicationMonitoringConfigurationEl>>,
    network_configuration: Option<DynamicBlock<EmrserverlessApplicationNetworkConfigurationEl>>,
    runtime_configuration: Option<DynamicBlock<EmrserverlessApplicationRuntimeConfigurationEl>>,
    scheduler_configuration: Option<DynamicBlock<EmrserverlessApplicationSchedulerConfigurationEl>>,
}
