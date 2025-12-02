use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct EcsCapacityProviderData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_scaling_group_provider: Option<Vec<EcsCapacityProviderAutoScalingGroupProviderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_instances_provider: Option<Vec<EcsCapacityProviderManagedInstancesProviderEl>>,
    dynamic: EcsCapacityProviderDynamic,
}
struct EcsCapacityProvider_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EcsCapacityProviderData>,
}
#[derive(Clone)]
pub struct EcsCapacityProvider(Rc<EcsCapacityProvider_>);
impl EcsCapacityProvider {
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
    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
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
    #[doc = "Set the field `auto_scaling_group_provider`.\n"]
    pub fn set_auto_scaling_group_provider(
        self,
        v: impl Into<BlockAssignable<EcsCapacityProviderAutoScalingGroupProviderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_scaling_group_provider = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_scaling_group_provider = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `managed_instances_provider`.\n"]
    pub fn set_managed_instances_provider(
        self,
        v: impl Into<BlockAssignable<EcsCapacityProviderManagedInstancesProviderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().managed_instances_provider = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.managed_instances_provider = Some(d);
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `auto_scaling_group_provider` after provisioning.\n"]
    pub fn auto_scaling_group_provider(
        &self,
    ) -> ListRef<EcsCapacityProviderAutoScalingGroupProviderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_scaling_group_provider", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_instances_provider` after provisioning.\n"]
    pub fn managed_instances_provider(
        &self,
    ) -> ListRef<EcsCapacityProviderManagedInstancesProviderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_instances_provider", self.extract_ref()),
        )
    }
}
impl Referable for EcsCapacityProvider {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for EcsCapacityProvider {}
impl ToListMappable for EcsCapacityProvider {
    type O = ListRef<EcsCapacityProviderRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for EcsCapacityProvider_ {
    fn extract_resource_type(&self) -> String {
        "aws_ecs_capacity_provider".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildEcsCapacityProvider {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildEcsCapacityProvider {
    pub fn build(self, stack: &mut Stack) -> EcsCapacityProvider {
        let out = EcsCapacityProvider(Rc::new(EcsCapacityProvider_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EcsCapacityProviderData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cluster: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                auto_scaling_group_provider: core::default::Default::default(),
                managed_instances_provider: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct EcsCapacityProviderRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcsCapacityProviderRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl EcsCapacityProviderRef {
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `auto_scaling_group_provider` after provisioning.\n"]
    pub fn auto_scaling_group_provider(
        &self,
    ) -> ListRef<EcsCapacityProviderAutoScalingGroupProviderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_scaling_group_provider", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_instances_provider` after provisioning.\n"]
    pub fn managed_instances_provider(
        &self,
    ) -> ListRef<EcsCapacityProviderManagedInstancesProviderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_instances_provider", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_warmup_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_scaling_step_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_scaling_step_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_capacity: Option<PrimField<f64>>,
}
impl EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl {
    #[doc = "Set the field `instance_warmup_period`.\n"]
    pub fn set_instance_warmup_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_warmup_period = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_scaling_step_size`.\n"]
    pub fn set_maximum_scaling_step_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_scaling_step_size = Some(v.into());
        self
    }
    #[doc = "Set the field `minimum_scaling_step_size`.\n"]
    pub fn set_minimum_scaling_step_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minimum_scaling_step_size = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
    #[doc = "Set the field `target_capacity`.\n"]
    pub fn set_target_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_capacity = Some(v.into());
        self
    }
}
impl ToListMappable for EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl {
    type O = BlockAssignable<EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl {}
impl BuildEcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl {
    pub fn build(self) -> EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl {
        EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl {
            instance_warmup_period: core::default::Default::default(),
            maximum_scaling_step_size: core::default::Default::default(),
            minimum_scaling_step_size: core::default::Default::default(),
            status: core::default::Default::default(),
            target_capacity: core::default::Default::default(),
        }
    }
}
pub struct EcsCapacityProviderAutoScalingGroupProviderElManagedScalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcsCapacityProviderAutoScalingGroupProviderElManagedScalingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EcsCapacityProviderAutoScalingGroupProviderElManagedScalingElRef {
        EcsCapacityProviderAutoScalingGroupProviderElManagedScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EcsCapacityProviderAutoScalingGroupProviderElManagedScalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_warmup_period` after provisioning.\n"]
    pub fn instance_warmup_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_warmup_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maximum_scaling_step_size` after provisioning.\n"]
    pub fn maximum_scaling_step_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_scaling_step_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `minimum_scaling_step_size` after provisioning.\n"]
    pub fn minimum_scaling_step_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimum_scaling_step_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
    #[doc = "Get a reference to the value of field `target_capacity` after provisioning.\n"]
    pub fn target_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_capacity", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct EcsCapacityProviderAutoScalingGroupProviderElDynamic {
    managed_scaling:
        Option<DynamicBlock<EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl>>,
}
#[derive(Serialize)]
pub struct EcsCapacityProviderAutoScalingGroupProviderEl {
    auto_scaling_group_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_draining: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_termination_protection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_scaling: Option<Vec<EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl>>,
    dynamic: EcsCapacityProviderAutoScalingGroupProviderElDynamic,
}
impl EcsCapacityProviderAutoScalingGroupProviderEl {
    #[doc = "Set the field `managed_draining`.\n"]
    pub fn set_managed_draining(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.managed_draining = Some(v.into());
        self
    }
    #[doc = "Set the field `managed_termination_protection`.\n"]
    pub fn set_managed_termination_protection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.managed_termination_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `managed_scaling`.\n"]
    pub fn set_managed_scaling(
        mut self,
        v: impl Into<BlockAssignable<EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.managed_scaling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_scaling = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for EcsCapacityProviderAutoScalingGroupProviderEl {
    type O = BlockAssignable<EcsCapacityProviderAutoScalingGroupProviderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEcsCapacityProviderAutoScalingGroupProviderEl {
    #[doc = ""]
    pub auto_scaling_group_arn: PrimField<String>,
}
impl BuildEcsCapacityProviderAutoScalingGroupProviderEl {
    pub fn build(self) -> EcsCapacityProviderAutoScalingGroupProviderEl {
        EcsCapacityProviderAutoScalingGroupProviderEl {
            auto_scaling_group_arn: self.auto_scaling_group_arn,
            managed_draining: core::default::Default::default(),
            managed_termination_protection: core::default::Default::default(),
            managed_scaling: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EcsCapacityProviderAutoScalingGroupProviderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcsCapacityProviderAutoScalingGroupProviderElRef {
    fn new(shared: StackShared, base: String) -> EcsCapacityProviderAutoScalingGroupProviderElRef {
        EcsCapacityProviderAutoScalingGroupProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EcsCapacityProviderAutoScalingGroupProviderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_scaling_group_arn` after provisioning.\n"]
    pub fn auto_scaling_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_scaling_group_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `managed_draining` after provisioning.\n"]
    pub fn managed_draining(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.managed_draining", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `managed_termination_protection` after provisioning.\n"]
    pub fn managed_termination_protection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.managed_termination_protection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `managed_scaling` after provisioning.\n"]
    pub fn managed_scaling(
        &self,
    ) -> ListRef<EcsCapacityProviderAutoScalingGroupProviderElManagedScalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_scaling", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorCountEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorCountEl { # [doc = "Set the field `max`.\n"] pub fn set_max (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . max = Some (v . into ()) ; self } # [doc = "Set the field `min`.\n"] pub fn set_min (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . min = Some (v . into ()) ; self } }
impl ToListMappable for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorCountEl { type O = BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorCountEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorCountEl
{}
impl BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorCountEl { pub fn build (self) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorCountEl { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorCountEl { max : core :: default :: Default :: default () , min : core :: default :: Default :: default () , } } }
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorCountElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorCountElRef { fn new (shared : StackShared , base : String) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorCountElRef { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorCountElRef { shared : shared , base : base . to_string () , } } }
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorCountElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `max` after provisioning.\n"] pub fn max (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max" , self . base)) } # [doc = "Get a reference to the value of field `min` after provisioning.\n"] pub fn min (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min" , self . base)) } }
#[derive(Serialize)]
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorTotalMemoryMibEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorTotalMemoryMibEl { # [doc = "Set the field `max`.\n"] pub fn set_max (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . max = Some (v . into ()) ; self } # [doc = "Set the field `min`.\n"] pub fn set_min (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . min = Some (v . into ()) ; self } }
impl ToListMappable for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorTotalMemoryMibEl { type O = BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorTotalMemoryMibEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorTotalMemoryMibEl
{}
impl BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorTotalMemoryMibEl { pub fn build (self) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorTotalMemoryMibEl { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorTotalMemoryMibEl { max : core :: default :: Default :: default () , min : core :: default :: Default :: default () , } } }
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorTotalMemoryMibElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorTotalMemoryMibElRef { fn new (shared : StackShared , base : String) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorTotalMemoryMibElRef { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorTotalMemoryMibElRef { shared : shared , base : base . to_string () , } } }
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorTotalMemoryMibElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `max` after provisioning.\n"] pub fn max (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max" , self . base)) } # [doc = "Get a reference to the value of field `min` after provisioning.\n"] pub fn min (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min" , self . base)) } }
#[derive(Serialize)]
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElBaselineEbsBandwidthMbpsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElBaselineEbsBandwidthMbpsEl { # [doc = "Set the field `max`.\n"] pub fn set_max (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . max = Some (v . into ()) ; self } # [doc = "Set the field `min`.\n"] pub fn set_min (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . min = Some (v . into ()) ; self } }
impl ToListMappable for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElBaselineEbsBandwidthMbpsEl { type O = BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElBaselineEbsBandwidthMbpsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElBaselineEbsBandwidthMbpsEl
{}
impl BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElBaselineEbsBandwidthMbpsEl { pub fn build (self) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElBaselineEbsBandwidthMbpsEl { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElBaselineEbsBandwidthMbpsEl { max : core :: default :: Default :: default () , min : core :: default :: Default :: default () , } } }
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef { fn new (shared : StackShared , base : String) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef { shared : shared , base : base . to_string () , } } }
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `max` after provisioning.\n"] pub fn max (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max" , self . base)) } # [doc = "Get a reference to the value of field `min` after provisioning.\n"] pub fn min (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min" , self . base)) } }
#[derive(Serialize)]
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryGibPerVcpuEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryGibPerVcpuEl { # [doc = "Set the field `max`.\n"] pub fn set_max (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . max = Some (v . into ()) ; self } # [doc = "Set the field `min`.\n"] pub fn set_min (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . min = Some (v . into ()) ; self } }
impl ToListMappable for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryGibPerVcpuEl { type O = BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryGibPerVcpuEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryGibPerVcpuEl
{}
impl BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryGibPerVcpuEl { pub fn build (self) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryGibPerVcpuEl { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryGibPerVcpuEl { max : core :: default :: Default :: default () , min : core :: default :: Default :: default () , } } }
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryGibPerVcpuElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryGibPerVcpuElRef { fn new (shared : StackShared , base : String) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryGibPerVcpuElRef { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryGibPerVcpuElRef { shared : shared , base : base . to_string () , } } }
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryGibPerVcpuElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `max` after provisioning.\n"] pub fn max (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max" , self . base)) } # [doc = "Get a reference to the value of field `min` after provisioning.\n"] pub fn min (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min" , self . base)) } }
#[derive(Serialize)]
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryMibEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    min: PrimField<f64>,
}
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryMibEl { # [doc = "Set the field `max`.\n"] pub fn set_max (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . max = Some (v . into ()) ; self } }
impl ToListMappable for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryMibEl { type O = BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryMibEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryMibEl
{
    #[doc = ""]
    pub min: PrimField<f64>,
}
impl BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryMibEl { pub fn build (self) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryMibEl { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryMibEl { max : core :: default :: Default :: default () , min : self . min , } } }
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryMibElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryMibElRef { fn new (shared : StackShared , base : String) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryMibElRef { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryMibElRef { shared : shared , base : base . to_string () , } } }
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryMibElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `max` after provisioning.\n"] pub fn max (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max" , self . base)) } # [doc = "Get a reference to the value of field `min` after provisioning.\n"] pub fn min (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min" , self . base)) } }
#[derive(Serialize)]
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkBandwidthGbpsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkBandwidthGbpsEl { # [doc = "Set the field `max`.\n"] pub fn set_max (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . max = Some (v . into ()) ; self } # [doc = "Set the field `min`.\n"] pub fn set_min (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . min = Some (v . into ()) ; self } }
impl ToListMappable for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkBandwidthGbpsEl { type O = BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkBandwidthGbpsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkBandwidthGbpsEl
{}
impl BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkBandwidthGbpsEl { pub fn build (self) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkBandwidthGbpsEl { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkBandwidthGbpsEl { max : core :: default :: Default :: default () , min : core :: default :: Default :: default () , } } }
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkBandwidthGbpsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkBandwidthGbpsElRef { fn new (shared : StackShared , base : String) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkBandwidthGbpsElRef { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkBandwidthGbpsElRef { shared : shared , base : base . to_string () , } } }
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkBandwidthGbpsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `max` after provisioning.\n"] pub fn max (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max" , self . base)) } # [doc = "Get a reference to the value of field `min` after provisioning.\n"] pub fn min (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min" , self . base)) } }
#[derive(Serialize)]
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkInterfaceCountEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkInterfaceCountEl { # [doc = "Set the field `max`.\n"] pub fn set_max (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . max = Some (v . into ()) ; self } # [doc = "Set the field `min`.\n"] pub fn set_min (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . min = Some (v . into ()) ; self } }
impl ToListMappable for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkInterfaceCountEl { type O = BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkInterfaceCountEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkInterfaceCountEl
{}
impl BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkInterfaceCountEl { pub fn build (self) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkInterfaceCountEl { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkInterfaceCountEl { max : core :: default :: Default :: default () , min : core :: default :: Default :: default () , } } }
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkInterfaceCountElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkInterfaceCountElRef { fn new (shared : StackShared , base : String) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkInterfaceCountElRef { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkInterfaceCountElRef { shared : shared , base : base . to_string () , } } }
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkInterfaceCountElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `max` after provisioning.\n"] pub fn max (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max" , self . base)) } # [doc = "Get a reference to the value of field `min` after provisioning.\n"] pub fn min (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min" , self . base)) } }
#[derive(Serialize)]
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElTotalLocalStorageGbEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElTotalLocalStorageGbEl { # [doc = "Set the field `max`.\n"] pub fn set_max (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . max = Some (v . into ()) ; self } # [doc = "Set the field `min`.\n"] pub fn set_min (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . min = Some (v . into ()) ; self } }
impl ToListMappable for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElTotalLocalStorageGbEl { type O = BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElTotalLocalStorageGbEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElTotalLocalStorageGbEl
{}
impl BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElTotalLocalStorageGbEl { pub fn build (self) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElTotalLocalStorageGbEl { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElTotalLocalStorageGbEl { max : core :: default :: Default :: default () , min : core :: default :: Default :: default () , } } }
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElTotalLocalStorageGbElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElTotalLocalStorageGbElRef { fn new (shared : StackShared , base : String) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElTotalLocalStorageGbElRef { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElTotalLocalStorageGbElRef { shared : shared , base : base . to_string () , } } }
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElTotalLocalStorageGbElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `max` after provisioning.\n"] pub fn max (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max" , self . base)) } # [doc = "Get a reference to the value of field `min` after provisioning.\n"] pub fn min (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min" , self . base)) } }
#[derive(Serialize)]
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElVcpuCountEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    min: PrimField<f64>,
}
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElVcpuCountEl { # [doc = "Set the field `max`.\n"] pub fn set_max (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . max = Some (v . into ()) ; self } }
impl ToListMappable for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElVcpuCountEl { type O = BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElVcpuCountEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElVcpuCountEl
{
    #[doc = ""]
    pub min: PrimField<f64>,
}
impl BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElVcpuCountEl { pub fn build (self) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElVcpuCountEl { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElVcpuCountEl { max : core :: default :: Default :: default () , min : self . min , } } }
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElVcpuCountElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElVcpuCountElRef { fn new (shared : StackShared , base : String) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElVcpuCountElRef { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElVcpuCountElRef { shared : shared , base : base . to_string () , } } }
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElVcpuCountElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `max` after provisioning.\n"] pub fn max (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max" , self . base)) } # [doc = "Get a reference to the value of field `min` after provisioning.\n"] pub fn min (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min" , self . base)) } }
#[derive(Serialize, Default)]
struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElDynamic { accelerator_count : Option < DynamicBlock < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorCountEl >> , accelerator_total_memory_mib : Option < DynamicBlock < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorTotalMemoryMibEl >> , baseline_ebs_bandwidth_mbps : Option < DynamicBlock < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElBaselineEbsBandwidthMbpsEl >> , memory_gib_per_vcpu : Option < DynamicBlock < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryGibPerVcpuEl >> , memory_mib : Option < DynamicBlock < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryMibEl >> , network_bandwidth_gbps : Option < DynamicBlock < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkBandwidthGbpsEl >> , network_interface_count : Option < DynamicBlock < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkInterfaceCountEl >> , total_local_storage_gb : Option < DynamicBlock < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElTotalLocalStorageGbEl >> , vcpu_count : Option < DynamicBlock < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElVcpuCountEl >> , }
#[derive(Serialize)]
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsEl { # [serde (skip_serializing_if = "Option::is_none")] accelerator_manufacturers : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] accelerator_names : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] accelerator_types : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] allowed_instance_types : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] bare_metal : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] burstable_performance : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] cpu_manufacturers : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] excluded_instance_types : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] instance_generations : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] local_storage : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] local_storage_types : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] max_spot_price_as_percentage_of_optimal_on_demand_price : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] on_demand_max_price_percentage_over_lowest_price : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] require_hibernate_support : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] spot_max_price_percentage_over_lowest_price : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] accelerator_count : Option < Vec < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorCountEl > > , # [serde (skip_serializing_if = "Option::is_none")] accelerator_total_memory_mib : Option < Vec < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorTotalMemoryMibEl > > , # [serde (skip_serializing_if = "Option::is_none")] baseline_ebs_bandwidth_mbps : Option < Vec < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElBaselineEbsBandwidthMbpsEl > > , # [serde (skip_serializing_if = "Option::is_none")] memory_gib_per_vcpu : Option < Vec < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryGibPerVcpuEl > > , # [serde (skip_serializing_if = "Option::is_none")] memory_mib : Option < Vec < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryMibEl > > , # [serde (skip_serializing_if = "Option::is_none")] network_bandwidth_gbps : Option < Vec < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkBandwidthGbpsEl > > , # [serde (skip_serializing_if = "Option::is_none")] network_interface_count : Option < Vec < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkInterfaceCountEl > > , # [serde (skip_serializing_if = "Option::is_none")] total_local_storage_gb : Option < Vec < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElTotalLocalStorageGbEl > > , # [serde (skip_serializing_if = "Option::is_none")] vcpu_count : Option < Vec < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElVcpuCountEl > > , dynamic : EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElDynamic , }
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsEl {
    #[doc = "Set the field `accelerator_manufacturers`.\n"]
    pub fn set_accelerator_manufacturers(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.accelerator_manufacturers = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerator_names`.\n"]
    pub fn set_accelerator_names(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.accelerator_names = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerator_types`.\n"]
    pub fn set_accelerator_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.accelerator_types = Some(v.into());
        self
    }
    #[doc = "Set the field `allowed_instance_types`.\n"]
    pub fn set_allowed_instance_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_instance_types = Some(v.into());
        self
    }
    #[doc = "Set the field `bare_metal`.\n"]
    pub fn set_bare_metal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bare_metal = Some(v.into());
        self
    }
    #[doc = "Set the field `burstable_performance`.\n"]
    pub fn set_burstable_performance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.burstable_performance = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_manufacturers`.\n"]
    pub fn set_cpu_manufacturers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.cpu_manufacturers = Some(v.into());
        self
    }
    #[doc = "Set the field `excluded_instance_types`.\n"]
    pub fn set_excluded_instance_types(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.excluded_instance_types = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_generations`.\n"]
    pub fn set_instance_generations(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.instance_generations = Some(v.into());
        self
    }
    #[doc = "Set the field `local_storage`.\n"]
    pub fn set_local_storage(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_storage = Some(v.into());
        self
    }
    #[doc = "Set the field `local_storage_types`.\n"]
    pub fn set_local_storage_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.local_storage_types = Some(v.into());
        self
    }
    #[doc = "Set the field `max_spot_price_as_percentage_of_optimal_on_demand_price`.\n"]
    pub fn set_max_spot_price_as_percentage_of_optimal_on_demand_price(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.max_spot_price_as_percentage_of_optimal_on_demand_price = Some(v.into());
        self
    }
    #[doc = "Set the field `on_demand_max_price_percentage_over_lowest_price`.\n"]
    pub fn set_on_demand_max_price_percentage_over_lowest_price(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.on_demand_max_price_percentage_over_lowest_price = Some(v.into());
        self
    }
    #[doc = "Set the field `require_hibernate_support`.\n"]
    pub fn set_require_hibernate_support(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_hibernate_support = Some(v.into());
        self
    }
    #[doc = "Set the field `spot_max_price_percentage_over_lowest_price`.\n"]
    pub fn set_spot_max_price_percentage_over_lowest_price(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.spot_max_price_percentage_over_lowest_price = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerator_count`.\n"]
    pub fn set_accelerator_count(
        mut self,
        v : impl Into < BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorCountEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.accelerator_count = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerator_count = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `accelerator_total_memory_mib`.\n"]
    pub fn set_accelerator_total_memory_mib(
        mut self,
        v : impl Into < BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorTotalMemoryMibEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.accelerator_total_memory_mib = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerator_total_memory_mib = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `baseline_ebs_bandwidth_mbps`.\n"]
    pub fn set_baseline_ebs_bandwidth_mbps(
        mut self,
        v : impl Into < BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElBaselineEbsBandwidthMbpsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.baseline_ebs_bandwidth_mbps = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.baseline_ebs_bandwidth_mbps = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `memory_gib_per_vcpu`.\n"]
    pub fn set_memory_gib_per_vcpu(
        mut self,
        v : impl Into < BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryGibPerVcpuEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.memory_gib_per_vcpu = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.memory_gib_per_vcpu = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `memory_mib`.\n"]
    pub fn set_memory_mib(
        mut self,
        v : impl Into < BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryMibEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.memory_mib = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.memory_mib = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_bandwidth_gbps`.\n"]
    pub fn set_network_bandwidth_gbps(
        mut self,
        v : impl Into < BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkBandwidthGbpsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_bandwidth_gbps = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_bandwidth_gbps = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_interface_count`.\n"]
    pub fn set_network_interface_count(
        mut self,
        v : impl Into < BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkInterfaceCountEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_interface_count = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_interface_count = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `total_local_storage_gb`.\n"]
    pub fn set_total_local_storage_gb(
        mut self,
        v : impl Into < BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElTotalLocalStorageGbEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.total_local_storage_gb = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.total_local_storage_gb = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vcpu_count`.\n"]
    pub fn set_vcpu_count(
        mut self,
        v : impl Into < BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElVcpuCountEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vcpu_count = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vcpu_count = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsEl
{
    type O = BlockAssignable<
        EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsEl
{}
impl
    BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsEl
{
    pub fn build(
        self,
    ) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsEl
    {
        EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsEl { accelerator_manufacturers : core :: default :: Default :: default () , accelerator_names : core :: default :: Default :: default () , accelerator_types : core :: default :: Default :: default () , allowed_instance_types : core :: default :: Default :: default () , bare_metal : core :: default :: Default :: default () , burstable_performance : core :: default :: Default :: default () , cpu_manufacturers : core :: default :: Default :: default () , excluded_instance_types : core :: default :: Default :: default () , instance_generations : core :: default :: Default :: default () , local_storage : core :: default :: Default :: default () , local_storage_types : core :: default :: Default :: default () , max_spot_price_as_percentage_of_optimal_on_demand_price : core :: default :: Default :: default () , on_demand_max_price_percentage_over_lowest_price : core :: default :: Default :: default () , require_hibernate_support : core :: default :: Default :: default () , spot_max_price_percentage_over_lowest_price : core :: default :: Default :: default () , accelerator_count : core :: default :: Default :: default () , accelerator_total_memory_mib : core :: default :: Default :: default () , baseline_ebs_bandwidth_mbps : core :: default :: Default :: default () , memory_gib_per_vcpu : core :: default :: Default :: default () , memory_mib : core :: default :: Default :: default () , network_bandwidth_gbps : core :: default :: Default :: default () , network_interface_count : core :: default :: Default :: default () , total_local_storage_gb : core :: default :: Default :: default () , vcpu_count : core :: default :: Default :: default () , dynamic : Default :: default () , }
    }
}
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElRef { fn new (shared : StackShared , base : String) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElRef { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElRef { shared : shared , base : base . to_string () , } } }
impl
    EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_manufacturers` after provisioning.\n"]
    pub fn accelerator_manufacturers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.accelerator_manufacturers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `accelerator_names` after provisioning.\n"]
    pub fn accelerator_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.accelerator_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `accelerator_types` after provisioning.\n"]
    pub fn accelerator_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.accelerator_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_instance_types` after provisioning.\n"]
    pub fn allowed_instance_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.allowed_instance_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bare_metal` after provisioning.\n"]
    pub fn bare_metal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bare_metal", self.base))
    }
    #[doc = "Get a reference to the value of field `burstable_performance` after provisioning.\n"]
    pub fn burstable_performance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.burstable_performance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_manufacturers` after provisioning.\n"]
    pub fn cpu_manufacturers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.cpu_manufacturers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `excluded_instance_types` after provisioning.\n"]
    pub fn excluded_instance_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.excluded_instance_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_generations` after provisioning.\n"]
    pub fn instance_generations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.instance_generations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_storage` after provisioning.\n"]
    pub fn local_storage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_storage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_storage_types` after provisioning.\n"]
    pub fn local_storage_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.local_storage_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_spot_price_as_percentage_of_optimal_on_demand_price` after provisioning.\n"]
    pub fn max_spot_price_as_percentage_of_optimal_on_demand_price(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.max_spot_price_as_percentage_of_optimal_on_demand_price",
                self.base
            ),
        )
    }
    #[doc = "Get a reference to the value of field `on_demand_max_price_percentage_over_lowest_price` after provisioning.\n"]
    pub fn on_demand_max_price_percentage_over_lowest_price(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.on_demand_max_price_percentage_over_lowest_price",
                self.base
            ),
        )
    }
    #[doc = "Get a reference to the value of field `require_hibernate_support` after provisioning.\n"]
    pub fn require_hibernate_support(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_hibernate_support", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `spot_max_price_percentage_over_lowest_price` after provisioning.\n"]
    pub fn spot_max_price_percentage_over_lowest_price(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_max_price_percentage_over_lowest_price", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\n"]    pub fn accelerator_count (& self) -> ListRef < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorCountElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerator_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `accelerator_total_memory_mib` after provisioning.\n"]    pub fn accelerator_total_memory_mib (& self) -> ListRef < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElAcceleratorTotalMemoryMibElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerator_total_memory_mib", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `baseline_ebs_bandwidth_mbps` after provisioning.\n"]    pub fn baseline_ebs_bandwidth_mbps (& self) -> ListRef < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.baseline_ebs_bandwidth_mbps", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_gib_per_vcpu` after provisioning.\n"]    pub fn memory_gib_per_vcpu (& self) -> ListRef < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryGibPerVcpuElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.memory_gib_per_vcpu", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_mib` after provisioning.\n"]    pub fn memory_mib (& self) -> ListRef < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElMemoryMibElRef >{
        ListRef::new(self.shared().clone(), format!("{}.memory_mib", self.base))
    }
    #[doc = "Get a reference to the value of field `network_bandwidth_gbps` after provisioning.\n"]    pub fn network_bandwidth_gbps (& self) -> ListRef < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkBandwidthGbpsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_bandwidth_gbps", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_interface_count` after provisioning.\n"]    pub fn network_interface_count (& self) -> ListRef < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElNetworkInterfaceCountElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interface_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_local_storage_gb` after provisioning.\n"]    pub fn total_local_storage_gb (& self) -> ListRef < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElTotalLocalStorageGbElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.total_local_storage_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vcpu_count` after provisioning.\n"]    pub fn vcpu_count (& self) -> ListRef < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElVcpuCountElRef >{
        ListRef::new(self.shared().clone(), format!("{}.vcpu_count", self.base))
    }
}
#[derive(Serialize)]
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElNetworkConfigurationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    subnets: SetField<PrimField<String>>,
}
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElNetworkConfigurationEl {
    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_groups = Some(v.into());
        self
    }
}
impl ToListMappable
    for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElNetworkConfigurationEl
{
    type O = BlockAssignable<
        EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElNetworkConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElNetworkConfigurationEl
{
    #[doc = ""]
    pub subnets: SetField<PrimField<String>>,
}
impl
    BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElNetworkConfigurationEl
{
    pub fn build(
        self,
    ) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElNetworkConfigurationEl
    {
        EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElNetworkConfigurationEl { security_groups : core :: default :: Default :: default () , subnets : self . subnets , }
    }
}
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElNetworkConfigurationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElNetworkConfigurationElRef { fn new (shared : StackShared , base : String) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElNetworkConfigurationElRef { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElNetworkConfigurationElRef { shared : shared , base : base . to_string () , } } }
impl
    EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElNetworkConfigurationElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
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
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElStorageConfigurationEl
{
    storage_size_gib: PrimField<f64>,
}
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElStorageConfigurationEl {}
impl ToListMappable
    for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElStorageConfigurationEl
{
    type O = BlockAssignable<
        EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElStorageConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElStorageConfigurationEl
{
    #[doc = ""]
    pub storage_size_gib: PrimField<f64>,
}
impl
    BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElStorageConfigurationEl
{
    pub fn build(
        self,
    ) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElStorageConfigurationEl
    {
        EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElStorageConfigurationEl { storage_size_gib : self . storage_size_gib , }
    }
}
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElStorageConfigurationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElStorageConfigurationElRef { fn new (shared : StackShared , base : String) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElStorageConfigurationElRef { EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElStorageConfigurationElRef { shared : shared , base : base . to_string () , } } }
impl
    EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElStorageConfigurationElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `storage_size_gib` after provisioning.\n"]
    pub fn storage_size_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_size_gib", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElDynamic { instance_requirements : Option < DynamicBlock < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsEl >> , network_configuration : Option < DynamicBlock < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElNetworkConfigurationEl >> , storage_configuration : Option < DynamicBlock < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElStorageConfigurationEl >> , }
#[derive(Serialize)]
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateEl { ec2_instance_profile_arn : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] monitoring : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] instance_requirements : Option < Vec < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsEl > > , # [serde (skip_serializing_if = "Option::is_none")] network_configuration : Option < Vec < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElNetworkConfigurationEl > > , # [serde (skip_serializing_if = "Option::is_none")] storage_configuration : Option < Vec < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElStorageConfigurationEl > > , dynamic : EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElDynamic , }
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateEl {
    #[doc = "Set the field `monitoring`.\n"]
    pub fn set_monitoring(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.monitoring = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_requirements`.\n"]
    pub fn set_instance_requirements(
        mut self,
        v : impl Into < BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.instance_requirements = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_requirements = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        mut self,
        v : impl Into < BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElNetworkConfigurationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `storage_configuration`.\n"]
    pub fn set_storage_configuration(
        mut self,
        v : impl Into < BlockAssignable < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElStorageConfigurationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.storage_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateEl {
    type O = BlockAssignable<EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateEl {
    #[doc = ""]
    pub ec2_instance_profile_arn: PrimField<String>,
}
impl BuildEcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateEl {
    pub fn build(self) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateEl {
        EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateEl {
            ec2_instance_profile_arn: self.ec2_instance_profile_arn,
            monitoring: core::default::Default::default(),
            instance_requirements: core::default::Default::default(),
            network_configuration: core::default::Default::default(),
            storage_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElRef {
        EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ec2_instance_profile_arn` after provisioning.\n"]
    pub fn ec2_instance_profile_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ec2_instance_profile_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.base))
    }
    #[doc = "Get a reference to the value of field `instance_requirements` after provisioning.\n"]    pub fn instance_requirements (& self) -> ListRef < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElInstanceRequirementsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_requirements", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]    pub fn network_configuration (& self) -> ListRef < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElNetworkConfigurationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `storage_configuration` after provisioning.\n"]    pub fn storage_configuration (& self) -> ListRef < EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElStorageConfigurationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_configuration", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct EcsCapacityProviderManagedInstancesProviderElDynamic {
    instance_launch_template:
        Option<DynamicBlock<EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateEl>>,
}
#[derive(Serialize)]
pub struct EcsCapacityProviderManagedInstancesProviderEl {
    infrastructure_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    propagate_tags: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_launch_template:
        Option<Vec<EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateEl>>,
    dynamic: EcsCapacityProviderManagedInstancesProviderElDynamic,
}
impl EcsCapacityProviderManagedInstancesProviderEl {
    #[doc = "Set the field `propagate_tags`.\n"]
    pub fn set_propagate_tags(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.propagate_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_launch_template`.\n"]
    pub fn set_instance_launch_template(
        mut self,
        v: impl Into<
            BlockAssignable<EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.instance_launch_template = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_launch_template = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for EcsCapacityProviderManagedInstancesProviderEl {
    type O = BlockAssignable<EcsCapacityProviderManagedInstancesProviderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEcsCapacityProviderManagedInstancesProviderEl {
    #[doc = ""]
    pub infrastructure_role_arn: PrimField<String>,
}
impl BuildEcsCapacityProviderManagedInstancesProviderEl {
    pub fn build(self) -> EcsCapacityProviderManagedInstancesProviderEl {
        EcsCapacityProviderManagedInstancesProviderEl {
            infrastructure_role_arn: self.infrastructure_role_arn,
            propagate_tags: core::default::Default::default(),
            instance_launch_template: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EcsCapacityProviderManagedInstancesProviderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EcsCapacityProviderManagedInstancesProviderElRef {
    fn new(shared: StackShared, base: String) -> EcsCapacityProviderManagedInstancesProviderElRef {
        EcsCapacityProviderManagedInstancesProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EcsCapacityProviderManagedInstancesProviderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `infrastructure_role_arn` after provisioning.\n"]
    pub fn infrastructure_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.infrastructure_role_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.propagate_tags", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_launch_template` after provisioning.\n"]
    pub fn instance_launch_template(
        &self,
    ) -> ListRef<EcsCapacityProviderManagedInstancesProviderElInstanceLaunchTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_launch_template", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct EcsCapacityProviderDynamic {
    auto_scaling_group_provider:
        Option<DynamicBlock<EcsCapacityProviderAutoScalingGroupProviderEl>>,
    managed_instances_provider: Option<DynamicBlock<EcsCapacityProviderManagedInstancesProviderEl>>,
}
