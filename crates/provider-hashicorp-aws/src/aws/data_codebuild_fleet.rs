use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCodebuildFleetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
struct DataCodebuildFleet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCodebuildFleetData>,
}
#[derive(Clone)]
pub struct DataCodebuildFleet(Rc<DataCodebuildFleet_>);
impl DataCodebuildFleet {
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
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `base_capacity` after provisioning.\n"]
    pub fn base_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compute_configuration` after provisioning.\n"]
    pub fn compute_configuration(&self) -> ListRef<DataCodebuildFleetComputeConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compute_type` after provisioning.\n"]
    pub fn compute_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment_type` after provisioning.\n"]
    pub fn environment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_service_role` after provisioning.\n"]
    pub fn fleet_service_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_service_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `image_id` after provisioning.\n"]
    pub fn image_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `overflow_behavior` after provisioning.\n"]
    pub fn overflow_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.overflow_behavior", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scaling_configuration` after provisioning.\n"]
    pub fn scaling_configuration(&self) -> ListRef<DataCodebuildFleetScalingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> SetRef<DataCodebuildFleetStatusElRef> {
        SetRef::new(
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
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<DataCodebuildFleetVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}
impl Referable for DataCodebuildFleet {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCodebuildFleet {}
impl ToListMappable for DataCodebuildFleet {
    type O = ListRef<DataCodebuildFleetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCodebuildFleet_ {
    fn extract_datasource_type(&self) -> String {
        "aws_codebuild_fleet".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCodebuildFleet {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildDataCodebuildFleet {
    pub fn build(self, stack: &mut Stack) -> DataCodebuildFleet {
        let out = DataCodebuildFleet(Rc::new(DataCodebuildFleet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCodebuildFleetData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataCodebuildFleetRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCodebuildFleetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCodebuildFleetRef {
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
    #[doc = "Get a reference to the value of field `base_capacity` after provisioning.\n"]
    pub fn base_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compute_configuration` after provisioning.\n"]
    pub fn compute_configuration(&self) -> ListRef<DataCodebuildFleetComputeConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compute_type` after provisioning.\n"]
    pub fn compute_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment_type` after provisioning.\n"]
    pub fn environment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_service_role` after provisioning.\n"]
    pub fn fleet_service_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_service_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `image_id` after provisioning.\n"]
    pub fn image_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `overflow_behavior` after provisioning.\n"]
    pub fn overflow_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.overflow_behavior", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scaling_configuration` after provisioning.\n"]
    pub fn scaling_configuration(&self) -> ListRef<DataCodebuildFleetScalingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> SetRef<DataCodebuildFleetStatusElRef> {
        SetRef::new(
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
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<DataCodebuildFleetVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCodebuildFleetComputeConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcpu: Option<PrimField<f64>>,
}
impl DataCodebuildFleetComputeConfigurationEl {
    #[doc = "Set the field `disk`.\n"]
    pub fn set_disk(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory = Some(v.into());
        self
    }
    #[doc = "Set the field `vcpu`.\n"]
    pub fn set_vcpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.vcpu = Some(v.into());
        self
    }
}
impl ToListMappable for DataCodebuildFleetComputeConfigurationEl {
    type O = BlockAssignable<DataCodebuildFleetComputeConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCodebuildFleetComputeConfigurationEl {}
impl BuildDataCodebuildFleetComputeConfigurationEl {
    pub fn build(self) -> DataCodebuildFleetComputeConfigurationEl {
        DataCodebuildFleetComputeConfigurationEl {
            disk: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            memory: core::default::Default::default(),
            vcpu: core::default::Default::default(),
        }
    }
}
pub struct DataCodebuildFleetComputeConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCodebuildFleetComputeConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataCodebuildFleetComputeConfigurationElRef {
        DataCodebuildFleetComputeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCodebuildFleetComputeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
    #[doc = "Get a reference to the value of field `vcpu` after provisioning.\n"]
    pub fn vcpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vcpu", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_value: Option<PrimField<f64>>,
}
impl DataCodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl {
    #[doc = "Set the field `metric_type`.\n"]
    pub fn set_metric_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_type = Some(v.into());
        self
    }
    #[doc = "Set the field `target_value`.\n"]
    pub fn set_target_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_value = Some(v.into());
        self
    }
}
impl ToListMappable for DataCodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl {
    type O =
        BlockAssignable<DataCodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl {}
impl BuildDataCodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl {
    pub fn build(self) -> DataCodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl {
        DataCodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl {
            metric_type: core::default::Default::default(),
            target_value: core::default::Default::default(),
        }
    }
}
pub struct DataCodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsElRef {
        DataCodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metric_type` after provisioning.\n"]
    pub fn metric_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_type", self.base))
    }
    #[doc = "Get a reference to the value of field `target_value` after provisioning.\n"]
    pub fn target_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCodebuildFleetScalingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_tracking_scaling_configs:
        Option<ListField<DataCodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl>>,
}
impl DataCodebuildFleetScalingConfigurationEl {
    #[doc = "Set the field `desired_capacity`.\n"]
    pub fn set_desired_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.desired_capacity = Some(v.into());
        self
    }
    #[doc = "Set the field `max_capacity`.\n"]
    pub fn set_max_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_capacity = Some(v.into());
        self
    }
    #[doc = "Set the field `scaling_type`.\n"]
    pub fn set_scaling_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scaling_type = Some(v.into());
        self
    }
    #[doc = "Set the field `target_tracking_scaling_configs`.\n"]
    pub fn set_target_tracking_scaling_configs(
        mut self,
        v: impl Into<ListField<DataCodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl>>,
    ) -> Self {
        self.target_tracking_scaling_configs = Some(v.into());
        self
    }
}
impl ToListMappable for DataCodebuildFleetScalingConfigurationEl {
    type O = BlockAssignable<DataCodebuildFleetScalingConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCodebuildFleetScalingConfigurationEl {}
impl BuildDataCodebuildFleetScalingConfigurationEl {
    pub fn build(self) -> DataCodebuildFleetScalingConfigurationEl {
        DataCodebuildFleetScalingConfigurationEl {
            desired_capacity: core::default::Default::default(),
            max_capacity: core::default::Default::default(),
            scaling_type: core::default::Default::default(),
            target_tracking_scaling_configs: core::default::Default::default(),
        }
    }
}
pub struct DataCodebuildFleetScalingConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCodebuildFleetScalingConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataCodebuildFleetScalingConfigurationElRef {
        DataCodebuildFleetScalingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCodebuildFleetScalingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `desired_capacity` after provisioning.\n"]
    pub fn desired_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_capacity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_capacity", self.base))
    }
    #[doc = "Get a reference to the value of field `scaling_type` after provisioning.\n"]
    pub fn scaling_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scaling_type", self.base))
    }
    #[doc = "Get a reference to the value of field `target_tracking_scaling_configs` after provisioning.\n"]
    pub fn target_tracking_scaling_configs(
        &self,
    ) -> ListRef<DataCodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_tracking_scaling_configs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCodebuildFleetStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<String>>,
}
impl DataCodebuildFleetStatusEl {
    #[doc = "Set the field `context`.\n"]
    pub fn set_context(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.context = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status_code = Some(v.into());
        self
    }
}
impl ToListMappable for DataCodebuildFleetStatusEl {
    type O = BlockAssignable<DataCodebuildFleetStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCodebuildFleetStatusEl {}
impl BuildDataCodebuildFleetStatusEl {
    pub fn build(self) -> DataCodebuildFleetStatusEl {
        DataCodebuildFleetStatusEl {
            context: core::default::Default::default(),
            message: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}
pub struct DataCodebuildFleetStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCodebuildFleetStatusElRef {
    fn new(shared: StackShared, base: String) -> DataCodebuildFleetStatusElRef {
        DataCodebuildFleetStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCodebuildFleetStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.context", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCodebuildFleetVpcConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnets: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
}
impl DataCodebuildFleetVpcConfigEl {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `subnets`.\n"]
    pub fn set_subnets(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnets = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataCodebuildFleetVpcConfigEl {
    type O = BlockAssignable<DataCodebuildFleetVpcConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCodebuildFleetVpcConfigEl {}
impl BuildDataCodebuildFleetVpcConfigEl {
    pub fn build(self) -> DataCodebuildFleetVpcConfigEl {
        DataCodebuildFleetVpcConfigEl {
            security_group_ids: core::default::Default::default(),
            subnets: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}
pub struct DataCodebuildFleetVpcConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCodebuildFleetVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> DataCodebuildFleetVpcConfigElRef {
        DataCodebuildFleetVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCodebuildFleetVpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}
