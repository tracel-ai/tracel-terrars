use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CloudwatchLogDeliveryDestinationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_destination_type: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_destination_configuration:
        Option<Vec<CloudwatchLogDeliveryDestinationDeliveryDestinationConfigurationEl>>,
    dynamic: CloudwatchLogDeliveryDestinationDynamic,
}

struct CloudwatchLogDeliveryDestination_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudwatchLogDeliveryDestinationData>,
}

#[derive(Clone)]
pub struct CloudwatchLogDeliveryDestination(Rc<CloudwatchLogDeliveryDestination_>);

impl CloudwatchLogDeliveryDestination {
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

    #[doc = "Set the field `delivery_destination_type`.\n"]
    pub fn set_delivery_destination_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().delivery_destination_type = Some(v.into());
        self
    }

    #[doc = "Set the field `output_format`.\n"]
    pub fn set_output_format(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().output_format = Some(v.into());
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

    #[doc = "Set the field `delivery_destination_configuration`.\n"]
    pub fn set_delivery_destination_configuration(
        self,
        v: impl Into<
            BlockAssignable<CloudwatchLogDeliveryDestinationDeliveryDestinationConfigurationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().delivery_destination_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .delivery_destination_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delivery_destination_type` after provisioning.\n"]
    pub fn delivery_destination_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_destination_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `output_format` after provisioning.\n"]
    pub fn output_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_format", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `delivery_destination_configuration` after provisioning.\n"]
    pub fn delivery_destination_configuration(
        &self,
    ) -> ListRef<CloudwatchLogDeliveryDestinationDeliveryDestinationConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delivery_destination_configuration", self.extract_ref()),
        )
    }
}

impl Referable for CloudwatchLogDeliveryDestination {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for CloudwatchLogDeliveryDestination {}

impl ToListMappable for CloudwatchLogDeliveryDestination {
    type O = ListRef<CloudwatchLogDeliveryDestinationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudwatchLogDeliveryDestination_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudwatch_log_delivery_destination".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudwatchLogDeliveryDestination {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCloudwatchLogDeliveryDestination {
    pub fn build(self, stack: &mut Stack) -> CloudwatchLogDeliveryDestination {
        let out = CloudwatchLogDeliveryDestination(Rc::new(CloudwatchLogDeliveryDestination_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudwatchLogDeliveryDestinationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                delivery_destination_type: core::default::Default::default(),
                name: self.name,
                output_format: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                delivery_destination_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudwatchLogDeliveryDestinationRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchLogDeliveryDestinationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CloudwatchLogDeliveryDestinationRef {
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

    #[doc = "Get a reference to the value of field `delivery_destination_type` after provisioning.\n"]
    pub fn delivery_destination_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_destination_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `output_format` after provisioning.\n"]
    pub fn output_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_format", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `delivery_destination_configuration` after provisioning.\n"]
    pub fn delivery_destination_configuration(
        &self,
    ) -> ListRef<CloudwatchLogDeliveryDestinationDeliveryDestinationConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delivery_destination_configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogDeliveryDestinationDeliveryDestinationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_resource_arn: Option<PrimField<String>>,
}

impl CloudwatchLogDeliveryDestinationDeliveryDestinationConfigurationEl {
    #[doc = "Set the field `destination_resource_arn`.\n"]
    pub fn set_destination_resource_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_resource_arn = Some(v.into());
        self
    }
}

impl ToListMappable for CloudwatchLogDeliveryDestinationDeliveryDestinationConfigurationEl {
    type O = BlockAssignable<CloudwatchLogDeliveryDestinationDeliveryDestinationConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogDeliveryDestinationDeliveryDestinationConfigurationEl {}

impl BuildCloudwatchLogDeliveryDestinationDeliveryDestinationConfigurationEl {
    pub fn build(self) -> CloudwatchLogDeliveryDestinationDeliveryDestinationConfigurationEl {
        CloudwatchLogDeliveryDestinationDeliveryDestinationConfigurationEl {
            destination_resource_arn: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchLogDeliveryDestinationDeliveryDestinationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchLogDeliveryDestinationDeliveryDestinationConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchLogDeliveryDestinationDeliveryDestinationConfigurationElRef {
        CloudwatchLogDeliveryDestinationDeliveryDestinationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogDeliveryDestinationDeliveryDestinationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_resource_arn` after provisioning.\n"]
    pub fn destination_resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_resource_arn", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudwatchLogDeliveryDestinationDynamic {
    delivery_destination_configuration:
        Option<DynamicBlock<CloudwatchLogDeliveryDestinationDeliveryDestinationConfigurationEl>>,
}
