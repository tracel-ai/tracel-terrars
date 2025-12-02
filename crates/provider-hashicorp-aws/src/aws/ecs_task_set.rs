use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct EcsTaskSetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_delete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    service: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    task_definition: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_until_stable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_until_stable_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_provider_strategy: Option<Vec<EcsTaskSetCapacityProviderStrategyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer: Option<Vec<EcsTaskSetLoadBalancerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_configuration: Option<Vec<EcsTaskSetNetworkConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<Vec<EcsTaskSetScaleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_registries: Option<Vec<EcsTaskSetServiceRegistriesEl>>,
    dynamic: EcsTaskSetDynamic,
}
struct EcsTaskSet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EcsTaskSetData>,
}
#[derive(Clone)]
pub struct EcsTaskSet(Rc<EcsTaskSet_>);
impl EcsTaskSet {
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
    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().external_id = Some(v.into());
        self
    }
    #[doc = "Set the field `force_delete`.\n"]
    pub fn set_force_delete(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_delete = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `launch_type`.\n"]
    pub fn set_launch_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().launch_type = Some(v.into());
        self
    }
    #[doc = "Set the field `platform_version`.\n"]
    pub fn set_platform_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().platform_version = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
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
    #[doc = "Set the field `wait_until_stable`.\n"]
    pub fn set_wait_until_stable(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_until_stable = Some(v.into());
        self
    }
    #[doc = "Set the field `wait_until_stable_timeout`.\n"]
    pub fn set_wait_until_stable_timeout(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().wait_until_stable_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `capacity_provider_strategy`.\n"]
    pub fn set_capacity_provider_strategy(
        self,
        v: impl Into<BlockAssignable<EcsTaskSetCapacityProviderStrategyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_provider_strategy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity_provider_strategy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `load_balancer`.\n"]
    pub fn set_load_balancer(
        self,
        v: impl Into<BlockAssignable<EcsTaskSetLoadBalancerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().load_balancer = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.load_balancer = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl Into<BlockAssignable<EcsTaskSetNetworkConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scale`.\n"]
    pub fn set_scale(self, v: impl Into<BlockAssignable<EcsTaskSetScaleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scale = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scale = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_registries`.\n"]
    pub fn set_service_registries(
        self,
        v: impl Into<BlockAssignable<EcsTaskSetServiceRegistriesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_registries = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_registries = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\n"]
    pub fn force_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_delete", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `launch_type` after provisioning.\n"]
    pub fn launch_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stability_status` after provisioning.\n"]
    pub fn stability_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stability_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.task_definition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `task_set_id` after provisioning.\n"]
    pub fn task_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.task_set_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wait_until_stable` after provisioning.\n"]
    pub fn wait_until_stable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_until_stable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wait_until_stable_timeout` after provisioning.\n"]
    pub fn wait_until_stable_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_until_stable_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<EcsTaskSetNetworkConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scale` after provisioning.\n"]
    pub fn scale(&self) -> ListRef<EcsTaskSetScaleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scale", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_registries` after provisioning.\n"]
    pub fn service_registries(&self) -> ListRef<EcsTaskSetServiceRegistriesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_registries", self.extract_ref()),
        )
    }
}
impl Referable for EcsTaskSet {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for EcsTaskSet {}
impl ToListMappable for EcsTaskSet {
    type O = ListRef<EcsTaskSetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for EcsTaskSet_ {
    fn extract_resource_type(&self) -> String {
        "aws_ecs_task_set".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildEcsTaskSet {
    pub tf_id: String,
    #[doc = ""]
    pub cluster: PrimField<String>,
    #[doc = ""]
    pub service: PrimField<String>,
    #[doc = ""]
    pub task_definition: PrimField<String>,
}
impl BuildEcsTaskSet {
    pub fn build(self, stack: &mut Stack) -> EcsTaskSet {
        let out = EcsTaskSet(Rc::new(EcsTaskSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EcsTaskSetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cluster: self.cluster,
                external_id: core::default::Default::default(),
                force_delete: core::default::Default::default(),
                id: core::default::Default::default(),
                launch_type: core::default::Default::default(),
                platform_version: core::default::Default::default(),
                region: core::default::Default::default(),
                service: self.service,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                task_definition: self.task_definition,
                wait_until_stable: core::default::Default::default(),
                wait_until_stable_timeout: core::default::Default::default(),
                capacity_provider_strategy: core::default::Default::default(),
                load_balancer: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                scale: core::default::Default::default(),
                service_registries: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct EcsTaskSetRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcsTaskSetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl EcsTaskSetRef {
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
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\n"]
    pub fn force_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_delete", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `launch_type` after provisioning.\n"]
    pub fn launch_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stability_status` after provisioning.\n"]
    pub fn stability_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stability_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.task_definition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `task_set_id` after provisioning.\n"]
    pub fn task_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.task_set_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wait_until_stable` after provisioning.\n"]
    pub fn wait_until_stable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_until_stable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wait_until_stable_timeout` after provisioning.\n"]
    pub fn wait_until_stable_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_until_stable_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<EcsTaskSetNetworkConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scale` after provisioning.\n"]
    pub fn scale(&self) -> ListRef<EcsTaskSetScaleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scale", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_registries` after provisioning.\n"]
    pub fn service_registries(&self) -> ListRef<EcsTaskSetServiceRegistriesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_registries", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct EcsTaskSetCapacityProviderStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<PrimField<f64>>,
    capacity_provider: PrimField<String>,
    weight: PrimField<f64>,
}
impl EcsTaskSetCapacityProviderStrategyEl {
    #[doc = "Set the field `base`.\n"]
    pub fn set_base(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.base = Some(v.into());
        self
    }
}
impl ToListMappable for EcsTaskSetCapacityProviderStrategyEl {
    type O = BlockAssignable<EcsTaskSetCapacityProviderStrategyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEcsTaskSetCapacityProviderStrategyEl {
    #[doc = ""]
    pub capacity_provider: PrimField<String>,
    #[doc = ""]
    pub weight: PrimField<f64>,
}
impl BuildEcsTaskSetCapacityProviderStrategyEl {
    pub fn build(self) -> EcsTaskSetCapacityProviderStrategyEl {
        EcsTaskSetCapacityProviderStrategyEl {
            base: core::default::Default::default(),
            capacity_provider: self.capacity_provider,
            weight: self.weight,
        }
    }
}
pub struct EcsTaskSetCapacityProviderStrategyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcsTaskSetCapacityProviderStrategyElRef {
    fn new(shared: StackShared, base: String) -> EcsTaskSetCapacityProviderStrategyElRef {
        EcsTaskSetCapacityProviderStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EcsTaskSetCapacityProviderStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `base` after provisioning.\n"]
    pub fn base(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.base", self.base))
    }
    #[doc = "Get a reference to the value of field `capacity_provider` after provisioning.\n"]
    pub fn capacity_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_provider", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}
#[derive(Serialize)]
pub struct EcsTaskSetLoadBalancerEl {
    container_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group_arn: Option<PrimField<String>>,
}
impl EcsTaskSetLoadBalancerEl {
    #[doc = "Set the field `container_port`.\n"]
    pub fn set_container_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.container_port = Some(v.into());
        self
    }
    #[doc = "Set the field `load_balancer_name`.\n"]
    pub fn set_load_balancer_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.load_balancer_name = Some(v.into());
        self
    }
    #[doc = "Set the field `target_group_arn`.\n"]
    pub fn set_target_group_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_group_arn = Some(v.into());
        self
    }
}
impl ToListMappable for EcsTaskSetLoadBalancerEl {
    type O = BlockAssignable<EcsTaskSetLoadBalancerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEcsTaskSetLoadBalancerEl {
    #[doc = ""]
    pub container_name: PrimField<String>,
}
impl BuildEcsTaskSetLoadBalancerEl {
    pub fn build(self) -> EcsTaskSetLoadBalancerEl {
        EcsTaskSetLoadBalancerEl {
            container_name: self.container_name,
            container_port: core::default::Default::default(),
            load_balancer_name: core::default::Default::default(),
            target_group_arn: core::default::Default::default(),
        }
    }
}
pub struct EcsTaskSetLoadBalancerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcsTaskSetLoadBalancerElRef {
    fn new(shared: StackShared, base: String) -> EcsTaskSetLoadBalancerElRef {
        EcsTaskSetLoadBalancerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EcsTaskSetLoadBalancerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_port` after provisioning.\n"]
    pub fn container_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_port", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancer_name` after provisioning.\n"]
    pub fn load_balancer_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancer_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_group_arn` after provisioning.\n"]
    pub fn target_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_group_arn", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct EcsTaskSetNetworkConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    assign_public_ip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    subnets: SetField<PrimField<String>>,
}
impl EcsTaskSetNetworkConfigurationEl {
    #[doc = "Set the field `assign_public_ip`.\n"]
    pub fn set_assign_public_ip(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.assign_public_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_groups = Some(v.into());
        self
    }
}
impl ToListMappable for EcsTaskSetNetworkConfigurationEl {
    type O = BlockAssignable<EcsTaskSetNetworkConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEcsTaskSetNetworkConfigurationEl {
    #[doc = ""]
    pub subnets: SetField<PrimField<String>>,
}
impl BuildEcsTaskSetNetworkConfigurationEl {
    pub fn build(self) -> EcsTaskSetNetworkConfigurationEl {
        EcsTaskSetNetworkConfigurationEl {
            assign_public_ip: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnets: self.subnets,
        }
    }
}
pub struct EcsTaskSetNetworkConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcsTaskSetNetworkConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EcsTaskSetNetworkConfigurationElRef {
        EcsTaskSetNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EcsTaskSetNetworkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `assign_public_ip` after provisioning.\n"]
    pub fn assign_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assign_public_ip", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}
#[derive(Serialize)]
pub struct EcsTaskSetScaleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}
impl EcsTaskSetScaleEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for EcsTaskSetScaleEl {
    type O = BlockAssignable<EcsTaskSetScaleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEcsTaskSetScaleEl {}
impl BuildEcsTaskSetScaleEl {
    pub fn build(self) -> EcsTaskSetScaleEl {
        EcsTaskSetScaleEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct EcsTaskSetScaleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcsTaskSetScaleElRef {
    fn new(shared: StackShared, base: String) -> EcsTaskSetScaleElRef {
        EcsTaskSetScaleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EcsTaskSetScaleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct EcsTaskSetServiceRegistriesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    registry_arn: PrimField<String>,
}
impl EcsTaskSetServiceRegistriesEl {
    #[doc = "Set the field `container_name`.\n"]
    pub fn set_container_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_name = Some(v.into());
        self
    }
    #[doc = "Set the field `container_port`.\n"]
    pub fn set_container_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.container_port = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for EcsTaskSetServiceRegistriesEl {
    type O = BlockAssignable<EcsTaskSetServiceRegistriesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEcsTaskSetServiceRegistriesEl {
    #[doc = ""]
    pub registry_arn: PrimField<String>,
}
impl BuildEcsTaskSetServiceRegistriesEl {
    pub fn build(self) -> EcsTaskSetServiceRegistriesEl {
        EcsTaskSetServiceRegistriesEl {
            container_name: core::default::Default::default(),
            container_port: core::default::Default::default(),
            port: core::default::Default::default(),
            registry_arn: self.registry_arn,
        }
    }
}
pub struct EcsTaskSetServiceRegistriesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcsTaskSetServiceRegistriesElRef {
    fn new(shared: StackShared, base: String) -> EcsTaskSetServiceRegistriesElRef {
        EcsTaskSetServiceRegistriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EcsTaskSetServiceRegistriesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_port` after provisioning.\n"]
    pub fn container_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_port", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `registry_arn` after provisioning.\n"]
    pub fn registry_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_arn", self.base))
    }
}
#[derive(Serialize, Default)]
struct EcsTaskSetDynamic {
    capacity_provider_strategy: Option<DynamicBlock<EcsTaskSetCapacityProviderStrategyEl>>,
    load_balancer: Option<DynamicBlock<EcsTaskSetLoadBalancerEl>>,
    network_configuration: Option<DynamicBlock<EcsTaskSetNetworkConfigurationEl>>,
    scale: Option<DynamicBlock<EcsTaskSetScaleEl>>,
    service_registries: Option<DynamicBlock<EcsTaskSetServiceRegistriesEl>>,
}
