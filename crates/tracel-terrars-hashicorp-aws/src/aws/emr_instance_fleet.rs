use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EmrInstanceFleetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_on_demand_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_spot_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type_configs: Option<Vec<EmrInstanceFleetInstanceTypeConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_specifications: Option<Vec<EmrInstanceFleetLaunchSpecificationsEl>>,
    dynamic: EmrInstanceFleetDynamic,
}

struct EmrInstanceFleet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EmrInstanceFleetData>,
}

#[derive(Clone)]
pub struct EmrInstanceFleet(Rc<EmrInstanceFleet_>);

impl EmrInstanceFleet {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `target_on_demand_capacity`.\n"]
    pub fn set_target_on_demand_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().target_on_demand_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `target_spot_capacity`.\n"]
    pub fn set_target_spot_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().target_spot_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type_configs`.\n"]
    pub fn set_instance_type_configs(
        self,
        v: impl Into<BlockAssignable<EmrInstanceFleetInstanceTypeConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_type_configs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_type_configs = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `launch_specifications`.\n"]
    pub fn set_launch_specifications(
        self,
        v: impl Into<BlockAssignable<EmrInstanceFleetLaunchSpecificationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().launch_specifications = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.launch_specifications = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioned_on_demand_capacity` after provisioning.\n"]
    pub fn provisioned_on_demand_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_on_demand_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioned_spot_capacity` after provisioning.\n"]
    pub fn provisioned_spot_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_spot_capacity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_on_demand_capacity` after provisioning.\n"]
    pub fn target_on_demand_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_on_demand_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_spot_capacity` after provisioning.\n"]
    pub fn target_spot_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_spot_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_specifications` after provisioning.\n"]
    pub fn launch_specifications(&self) -> ListRef<EmrInstanceFleetLaunchSpecificationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.launch_specifications", self.extract_ref()))
    }
}

impl Referable for EmrInstanceFleet {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EmrInstanceFleet { }

impl ToListMappable for EmrInstanceFleet {
    type O = ListRef<EmrInstanceFleetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EmrInstanceFleet_ {
    fn extract_resource_type(&self) -> String {
        "aws_emr_instance_fleet".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmrInstanceFleet {
    pub tf_id: String,
    #[doc = ""]
    pub cluster_id: PrimField<String>,
}

impl BuildEmrInstanceFleet {
    pub fn build(self, stack: &mut Stack) -> EmrInstanceFleet {
        let out = EmrInstanceFleet(Rc::new(EmrInstanceFleet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EmrInstanceFleetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cluster_id: self.cluster_id,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                target_on_demand_capacity: core::default::Default::default(),
                target_spot_capacity: core::default::Default::default(),
                instance_type_configs: core::default::Default::default(),
                launch_specifications: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmrInstanceFleetRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrInstanceFleetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl EmrInstanceFleetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioned_on_demand_capacity` after provisioning.\n"]
    pub fn provisioned_on_demand_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_on_demand_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioned_spot_capacity` after provisioning.\n"]
    pub fn provisioned_spot_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_spot_capacity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_on_demand_capacity` after provisioning.\n"]
    pub fn target_on_demand_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_on_demand_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_spot_capacity` after provisioning.\n"]
    pub fn target_spot_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_spot_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_specifications` after provisioning.\n"]
    pub fn launch_specifications(&self) -> ListRef<EmrInstanceFleetLaunchSpecificationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.launch_specifications", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EmrInstanceFleetInstanceTypeConfigsElConfigurationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    classification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
}

impl EmrInstanceFleetInstanceTypeConfigsElConfigurationsEl {
    #[doc = "Set the field `classification`.\n"]
    pub fn set_classification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.classification = Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
}

impl ToListMappable for EmrInstanceFleetInstanceTypeConfigsElConfigurationsEl {
    type O = BlockAssignable<EmrInstanceFleetInstanceTypeConfigsElConfigurationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrInstanceFleetInstanceTypeConfigsElConfigurationsEl {}

impl BuildEmrInstanceFleetInstanceTypeConfigsElConfigurationsEl {
    pub fn build(self) -> EmrInstanceFleetInstanceTypeConfigsElConfigurationsEl {
        EmrInstanceFleetInstanceTypeConfigsElConfigurationsEl {
            classification: core::default::Default::default(),
            properties: core::default::Default::default(),
        }
    }
}

pub struct EmrInstanceFleetInstanceTypeConfigsElConfigurationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrInstanceFleetInstanceTypeConfigsElConfigurationsElRef {
    fn new(shared: StackShared, base: String) -> EmrInstanceFleetInstanceTypeConfigsElConfigurationsElRef {
        EmrInstanceFleetInstanceTypeConfigsElConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrInstanceFleetInstanceTypeConfigsElConfigurationsElRef {
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
pub struct EmrInstanceFleetInstanceTypeConfigsElEbsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    size: PrimField<f64>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes_per_instance: Option<PrimField<f64>>,
}

impl EmrInstanceFleetInstanceTypeConfigsElEbsConfigEl {
    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }

    #[doc = "Set the field `volumes_per_instance`.\n"]
    pub fn set_volumes_per_instance(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volumes_per_instance = Some(v.into());
        self
    }
}

impl ToListMappable for EmrInstanceFleetInstanceTypeConfigsElEbsConfigEl {
    type O = BlockAssignable<EmrInstanceFleetInstanceTypeConfigsElEbsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrInstanceFleetInstanceTypeConfigsElEbsConfigEl {
    #[doc = ""]
    pub size: PrimField<f64>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildEmrInstanceFleetInstanceTypeConfigsElEbsConfigEl {
    pub fn build(self) -> EmrInstanceFleetInstanceTypeConfigsElEbsConfigEl {
        EmrInstanceFleetInstanceTypeConfigsElEbsConfigEl {
            iops: core::default::Default::default(),
            size: self.size,
            type_: self.type_,
            volumes_per_instance: core::default::Default::default(),
        }
    }
}

pub struct EmrInstanceFleetInstanceTypeConfigsElEbsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrInstanceFleetInstanceTypeConfigsElEbsConfigElRef {
    fn new(shared: StackShared, base: String) -> EmrInstanceFleetInstanceTypeConfigsElEbsConfigElRef {
        EmrInstanceFleetInstanceTypeConfigsElEbsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrInstanceFleetInstanceTypeConfigsElEbsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes_per_instance` after provisioning.\n"]
    pub fn volumes_per_instance(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volumes_per_instance", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrInstanceFleetInstanceTypeConfigsElDynamic {
    configurations: Option<DynamicBlock<EmrInstanceFleetInstanceTypeConfigsElConfigurationsEl>>,
    ebs_config: Option<DynamicBlock<EmrInstanceFleetInstanceTypeConfigsElEbsConfigEl>>,
}

#[derive(Serialize)]
pub struct EmrInstanceFleetInstanceTypeConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bid_price: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bid_price_as_percentage_of_on_demand_price: Option<PrimField<f64>>,
    instance_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configurations: Option<Vec<EmrInstanceFleetInstanceTypeConfigsElConfigurationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_config: Option<Vec<EmrInstanceFleetInstanceTypeConfigsElEbsConfigEl>>,
    dynamic: EmrInstanceFleetInstanceTypeConfigsElDynamic,
}

impl EmrInstanceFleetInstanceTypeConfigsEl {
    #[doc = "Set the field `bid_price`.\n"]
    pub fn set_bid_price(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bid_price = Some(v.into());
        self
    }

    #[doc = "Set the field `bid_price_as_percentage_of_on_demand_price`.\n"]
    pub fn set_bid_price_as_percentage_of_on_demand_price(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bid_price_as_percentage_of_on_demand_price = Some(v.into());
        self
    }

    #[doc = "Set the field `weighted_capacity`.\n"]
    pub fn set_weighted_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weighted_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `configurations`.\n"]
    pub fn set_configurations(
        mut self,
        v: impl Into<BlockAssignable<EmrInstanceFleetInstanceTypeConfigsElConfigurationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.configurations = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.configurations = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ebs_config`.\n"]
    pub fn set_ebs_config(
        mut self,
        v: impl Into<BlockAssignable<EmrInstanceFleetInstanceTypeConfigsElEbsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ebs_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrInstanceFleetInstanceTypeConfigsEl {
    type O = BlockAssignable<EmrInstanceFleetInstanceTypeConfigsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrInstanceFleetInstanceTypeConfigsEl {
    #[doc = ""]
    pub instance_type: PrimField<String>,
}

impl BuildEmrInstanceFleetInstanceTypeConfigsEl {
    pub fn build(self) -> EmrInstanceFleetInstanceTypeConfigsEl {
        EmrInstanceFleetInstanceTypeConfigsEl {
            bid_price: core::default::Default::default(),
            bid_price_as_percentage_of_on_demand_price: core::default::Default::default(),
            instance_type: self.instance_type,
            weighted_capacity: core::default::Default::default(),
            configurations: core::default::Default::default(),
            ebs_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrInstanceFleetInstanceTypeConfigsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrInstanceFleetInstanceTypeConfigsElRef {
    fn new(shared: StackShared, base: String) -> EmrInstanceFleetInstanceTypeConfigsElRef {
        EmrInstanceFleetInstanceTypeConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrInstanceFleetInstanceTypeConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bid_price` after provisioning.\n"]
    pub fn bid_price(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bid_price", self.base))
    }

    #[doc = "Get a reference to the value of field `bid_price_as_percentage_of_on_demand_price` after provisioning.\n"]
    pub fn bid_price_as_percentage_of_on_demand_price(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bid_price_as_percentage_of_on_demand_price", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_capacity` after provisioning.\n"]
    pub fn weighted_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weighted_capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrInstanceFleetLaunchSpecificationsElOnDemandSpecificationEl {
    allocation_strategy: PrimField<String>,
}

impl EmrInstanceFleetLaunchSpecificationsElOnDemandSpecificationEl { }

impl ToListMappable for EmrInstanceFleetLaunchSpecificationsElOnDemandSpecificationEl {
    type O = BlockAssignable<EmrInstanceFleetLaunchSpecificationsElOnDemandSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrInstanceFleetLaunchSpecificationsElOnDemandSpecificationEl {
    #[doc = ""]
    pub allocation_strategy: PrimField<String>,
}

impl BuildEmrInstanceFleetLaunchSpecificationsElOnDemandSpecificationEl {
    pub fn build(self) -> EmrInstanceFleetLaunchSpecificationsElOnDemandSpecificationEl {
        EmrInstanceFleetLaunchSpecificationsElOnDemandSpecificationEl {
            allocation_strategy: self.allocation_strategy,
        }
    }
}

pub struct EmrInstanceFleetLaunchSpecificationsElOnDemandSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrInstanceFleetLaunchSpecificationsElOnDemandSpecificationElRef {
    fn new(shared: StackShared, base: String) -> EmrInstanceFleetLaunchSpecificationsElOnDemandSpecificationElRef {
        EmrInstanceFleetLaunchSpecificationsElOnDemandSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrInstanceFleetLaunchSpecificationsElOnDemandSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocation_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrInstanceFleetLaunchSpecificationsElSpotSpecificationEl {
    allocation_strategy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_duration_minutes: Option<PrimField<f64>>,
    timeout_action: PrimField<String>,
    timeout_duration_minutes: PrimField<f64>,
}

impl EmrInstanceFleetLaunchSpecificationsElSpotSpecificationEl {
    #[doc = "Set the field `block_duration_minutes`.\n"]
    pub fn set_block_duration_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.block_duration_minutes = Some(v.into());
        self
    }
}

impl ToListMappable for EmrInstanceFleetLaunchSpecificationsElSpotSpecificationEl {
    type O = BlockAssignable<EmrInstanceFleetLaunchSpecificationsElSpotSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrInstanceFleetLaunchSpecificationsElSpotSpecificationEl {
    #[doc = ""]
    pub allocation_strategy: PrimField<String>,
    #[doc = ""]
    pub timeout_action: PrimField<String>,
    #[doc = ""]
    pub timeout_duration_minutes: PrimField<f64>,
}

impl BuildEmrInstanceFleetLaunchSpecificationsElSpotSpecificationEl {
    pub fn build(self) -> EmrInstanceFleetLaunchSpecificationsElSpotSpecificationEl {
        EmrInstanceFleetLaunchSpecificationsElSpotSpecificationEl {
            allocation_strategy: self.allocation_strategy,
            block_duration_minutes: core::default::Default::default(),
            timeout_action: self.timeout_action,
            timeout_duration_minutes: self.timeout_duration_minutes,
        }
    }
}

pub struct EmrInstanceFleetLaunchSpecificationsElSpotSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrInstanceFleetLaunchSpecificationsElSpotSpecificationElRef {
    fn new(shared: StackShared, base: String) -> EmrInstanceFleetLaunchSpecificationsElSpotSpecificationElRef {
        EmrInstanceFleetLaunchSpecificationsElSpotSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrInstanceFleetLaunchSpecificationsElSpotSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocation_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `block_duration_minutes` after provisioning.\n"]
    pub fn block_duration_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.block_duration_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_action` after provisioning.\n"]
    pub fn timeout_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout_action", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_duration_minutes` after provisioning.\n"]
    pub fn timeout_duration_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout_duration_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrInstanceFleetLaunchSpecificationsElDynamic {
    on_demand_specification: Option<DynamicBlock<EmrInstanceFleetLaunchSpecificationsElOnDemandSpecificationEl>>,
    spot_specification: Option<DynamicBlock<EmrInstanceFleetLaunchSpecificationsElSpotSpecificationEl>>,
}

#[derive(Serialize)]
pub struct EmrInstanceFleetLaunchSpecificationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    on_demand_specification: Option<Vec<EmrInstanceFleetLaunchSpecificationsElOnDemandSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_specification: Option<Vec<EmrInstanceFleetLaunchSpecificationsElSpotSpecificationEl>>,
    dynamic: EmrInstanceFleetLaunchSpecificationsElDynamic,
}

impl EmrInstanceFleetLaunchSpecificationsEl {
    #[doc = "Set the field `on_demand_specification`.\n"]
    pub fn set_on_demand_specification(
        mut self,
        v: impl Into<BlockAssignable<EmrInstanceFleetLaunchSpecificationsElOnDemandSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.on_demand_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.on_demand_specification = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spot_specification`.\n"]
    pub fn set_spot_specification(
        mut self,
        v: impl Into<BlockAssignable<EmrInstanceFleetLaunchSpecificationsElSpotSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.spot_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.spot_specification = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrInstanceFleetLaunchSpecificationsEl {
    type O = BlockAssignable<EmrInstanceFleetLaunchSpecificationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrInstanceFleetLaunchSpecificationsEl {}

impl BuildEmrInstanceFleetLaunchSpecificationsEl {
    pub fn build(self) -> EmrInstanceFleetLaunchSpecificationsEl {
        EmrInstanceFleetLaunchSpecificationsEl {
            on_demand_specification: core::default::Default::default(),
            spot_specification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrInstanceFleetLaunchSpecificationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrInstanceFleetLaunchSpecificationsElRef {
    fn new(shared: StackShared, base: String) -> EmrInstanceFleetLaunchSpecificationsElRef {
        EmrInstanceFleetLaunchSpecificationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrInstanceFleetLaunchSpecificationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `on_demand_specification` after provisioning.\n"]
    pub fn on_demand_specification(&self) -> ListRef<EmrInstanceFleetLaunchSpecificationsElOnDemandSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.on_demand_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_specification` after provisioning.\n"]
    pub fn spot_specification(&self) -> ListRef<EmrInstanceFleetLaunchSpecificationsElSpotSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spot_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrInstanceFleetDynamic {
    instance_type_configs: Option<DynamicBlock<EmrInstanceFleetInstanceTypeConfigsEl>>,
    launch_specifications: Option<DynamicBlock<EmrInstanceFleetLaunchSpecificationsEl>>,
}
