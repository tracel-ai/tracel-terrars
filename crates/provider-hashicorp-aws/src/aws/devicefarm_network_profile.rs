use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DevicefarmNetworkProfileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    downlink_bandwidth_bits: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    downlink_delay_ms: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    downlink_jitter_ms: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    downlink_loss_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    project_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uplink_bandwidth_bits: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uplink_delay_ms: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uplink_jitter_ms: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uplink_loss_percent: Option<PrimField<f64>>,
}

struct DevicefarmNetworkProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DevicefarmNetworkProfileData>,
}

#[derive(Clone)]
pub struct DevicefarmNetworkProfile(Rc<DevicefarmNetworkProfile_>);

impl DevicefarmNetworkProfile {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `downlink_bandwidth_bits`.\n"]
    pub fn set_downlink_bandwidth_bits(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().downlink_bandwidth_bits = Some(v.into());
        self
    }

    #[doc = "Set the field `downlink_delay_ms`.\n"]
    pub fn set_downlink_delay_ms(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().downlink_delay_ms = Some(v.into());
        self
    }

    #[doc = "Set the field `downlink_jitter_ms`.\n"]
    pub fn set_downlink_jitter_ms(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().downlink_jitter_ms = Some(v.into());
        self
    }

    #[doc = "Set the field `downlink_loss_percent`.\n"]
    pub fn set_downlink_loss_percent(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().downlink_loss_percent = Some(v.into());
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

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `uplink_bandwidth_bits`.\n"]
    pub fn set_uplink_bandwidth_bits(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().uplink_bandwidth_bits = Some(v.into());
        self
    }

    #[doc = "Set the field `uplink_delay_ms`.\n"]
    pub fn set_uplink_delay_ms(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().uplink_delay_ms = Some(v.into());
        self
    }

    #[doc = "Set the field `uplink_jitter_ms`.\n"]
    pub fn set_uplink_jitter_ms(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().uplink_jitter_ms = Some(v.into());
        self
    }

    #[doc = "Set the field `uplink_loss_percent`.\n"]
    pub fn set_uplink_loss_percent(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().uplink_loss_percent = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `downlink_bandwidth_bits` after provisioning.\n"]
    pub fn downlink_bandwidth_bits(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.downlink_bandwidth_bits", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `downlink_delay_ms` after provisioning.\n"]
    pub fn downlink_delay_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.downlink_delay_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `downlink_jitter_ms` after provisioning.\n"]
    pub fn downlink_jitter_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.downlink_jitter_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `downlink_loss_percent` after provisioning.\n"]
    pub fn downlink_loss_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.downlink_loss_percent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_arn` after provisioning.\n"]
    pub fn project_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `uplink_bandwidth_bits` after provisioning.\n"]
    pub fn uplink_bandwidth_bits(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.uplink_bandwidth_bits", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uplink_delay_ms` after provisioning.\n"]
    pub fn uplink_delay_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.uplink_delay_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uplink_jitter_ms` after provisioning.\n"]
    pub fn uplink_jitter_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.uplink_jitter_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uplink_loss_percent` after provisioning.\n"]
    pub fn uplink_loss_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.uplink_loss_percent", self.extract_ref()))
    }
}

impl Referable for DevicefarmNetworkProfile {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DevicefarmNetworkProfile { }

impl ToListMappable for DevicefarmNetworkProfile {
    type O = ListRef<DevicefarmNetworkProfileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DevicefarmNetworkProfile_ {
    fn extract_resource_type(&self) -> String {
        "aws_devicefarm_network_profile".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDevicefarmNetworkProfile {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub project_arn: PrimField<String>,
}

impl BuildDevicefarmNetworkProfile {
    pub fn build(self, stack: &mut Stack) -> DevicefarmNetworkProfile {
        let out = DevicefarmNetworkProfile(Rc::new(DevicefarmNetworkProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DevicefarmNetworkProfileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                downlink_bandwidth_bits: core::default::Default::default(),
                downlink_delay_ms: core::default::Default::default(),
                downlink_jitter_ms: core::default::Default::default(),
                downlink_loss_percent: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project_arn: self.project_arn,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: core::default::Default::default(),
                uplink_bandwidth_bits: core::default::Default::default(),
                uplink_delay_ms: core::default::Default::default(),
                uplink_jitter_ms: core::default::Default::default(),
                uplink_loss_percent: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DevicefarmNetworkProfileRef {
    shared: StackShared,
    base: String,
}

impl Ref for DevicefarmNetworkProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DevicefarmNetworkProfileRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `downlink_bandwidth_bits` after provisioning.\n"]
    pub fn downlink_bandwidth_bits(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.downlink_bandwidth_bits", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `downlink_delay_ms` after provisioning.\n"]
    pub fn downlink_delay_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.downlink_delay_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `downlink_jitter_ms` after provisioning.\n"]
    pub fn downlink_jitter_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.downlink_jitter_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `downlink_loss_percent` after provisioning.\n"]
    pub fn downlink_loss_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.downlink_loss_percent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_arn` after provisioning.\n"]
    pub fn project_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `uplink_bandwidth_bits` after provisioning.\n"]
    pub fn uplink_bandwidth_bits(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.uplink_bandwidth_bits", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uplink_delay_ms` after provisioning.\n"]
    pub fn uplink_delay_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.uplink_delay_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uplink_jitter_ms` after provisioning.\n"]
    pub fn uplink_jitter_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.uplink_jitter_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uplink_loss_percent` after provisioning.\n"]
    pub fn uplink_loss_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.uplink_loss_percent", self.extract_ref()))
    }
}
