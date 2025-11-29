use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Sesv2ConfigurationSetEventDestinationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    configuration_set_name: PrimField<String>,
    event_destination_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_destination: Option<Vec<Sesv2ConfigurationSetEventDestinationEventDestinationEl>>,
    dynamic: Sesv2ConfigurationSetEventDestinationDynamic,
}

struct Sesv2ConfigurationSetEventDestination_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Sesv2ConfigurationSetEventDestinationData>,
}

#[derive(Clone)]
pub struct Sesv2ConfigurationSetEventDestination(Rc<Sesv2ConfigurationSetEventDestination_>);

impl Sesv2ConfigurationSetEventDestination {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `event_destination`.\n"]
    pub fn set_event_destination(
        self,
        v: impl Into<BlockAssignable<Sesv2ConfigurationSetEventDestinationEventDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_destination = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_destination_name` after provisioning.\n"]
    pub fn event_destination_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_destination_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_destination` after provisioning.\n"]
    pub fn event_destination(&self) -> ListRef<Sesv2ConfigurationSetEventDestinationEventDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.event_destination", self.extract_ref()))
    }
}

impl Referable for Sesv2ConfigurationSetEventDestination {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Sesv2ConfigurationSetEventDestination { }

impl ToListMappable for Sesv2ConfigurationSetEventDestination {
    type O = ListRef<Sesv2ConfigurationSetEventDestinationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Sesv2ConfigurationSetEventDestination_ {
    fn extract_resource_type(&self) -> String {
        "aws_sesv2_configuration_set_event_destination".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSesv2ConfigurationSetEventDestination {
    pub tf_id: String,
    #[doc = ""]
    pub configuration_set_name: PrimField<String>,
    #[doc = ""]
    pub event_destination_name: PrimField<String>,
}

impl BuildSesv2ConfigurationSetEventDestination {
    pub fn build(self, stack: &mut Stack) -> Sesv2ConfigurationSetEventDestination {
        let out = Sesv2ConfigurationSetEventDestination(Rc::new(Sesv2ConfigurationSetEventDestination_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Sesv2ConfigurationSetEventDestinationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                configuration_set_name: self.configuration_set_name,
                event_destination_name: self.event_destination_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                event_destination: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Sesv2ConfigurationSetEventDestinationRef {
    shared: StackShared,
    base: String,
}

impl Ref for Sesv2ConfigurationSetEventDestinationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl Sesv2ConfigurationSetEventDestinationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_destination_name` after provisioning.\n"]
    pub fn event_destination_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_destination_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_destination` after provisioning.\n"]
    pub fn event_destination(&self) -> ListRef<Sesv2ConfigurationSetEventDestinationEventDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.event_destination", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl {
    default_dimension_value: PrimField<String>,
    dimension_name: PrimField<String>,
    dimension_value_source: PrimField<String>,
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl { }

impl ToListMappable for Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl {
    type O =
        BlockAssignable<
            Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl {
    #[doc = ""]
    pub default_dimension_value: PrimField<String>,
    #[doc = ""]
    pub dimension_name: PrimField<String>,
    #[doc = ""]
    pub dimension_value_source: PrimField<String>,
}

impl BuildSesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl {
    pub fn build(
        self,
    ) -> Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl {
        Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl {
            default_dimension_value: self.default_dimension_value,
            dimension_name: self.dimension_name,
            dimension_value_source: self.dimension_value_source,
        }
    }
}

pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationElRef {
        Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_dimension_value` after provisioning.\n"]
    pub fn default_dimension_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_dimension_value", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension_name` after provisioning.\n"]
    pub fn dimension_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dimension_name", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension_value_source` after provisioning.\n"]
    pub fn dimension_value_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dimension_value_source", self.base))
    }
}

#[derive(Serialize, Default)]
struct Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDynamic {
    dimension_configuration: Option<
        DynamicBlock<
            Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension_configuration: Option<
        Vec<Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl>,
    >,
    dynamic: Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDynamic,
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl {
    #[doc = "Set the field `dimension_configuration`.\n"]
    pub fn set_dimension_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl {
    type O = BlockAssignable<Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl {}

impl BuildSesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl {
    pub fn build(self) -> Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl {
        Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl {
            dimension_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElRef {
        Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dimension_configuration` after provisioning.\n"]
    pub fn dimension_configuration(
        &self,
    ) -> ListRef<
        Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.dimension_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElEventBridgeDestinationEl {
    event_bus_arn: PrimField<String>,
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElEventBridgeDestinationEl { }

impl ToListMappable for Sesv2ConfigurationSetEventDestinationEventDestinationElEventBridgeDestinationEl {
    type O = BlockAssignable<Sesv2ConfigurationSetEventDestinationEventDestinationElEventBridgeDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2ConfigurationSetEventDestinationEventDestinationElEventBridgeDestinationEl {
    #[doc = ""]
    pub event_bus_arn: PrimField<String>,
}

impl BuildSesv2ConfigurationSetEventDestinationEventDestinationElEventBridgeDestinationEl {
    pub fn build(self) -> Sesv2ConfigurationSetEventDestinationEventDestinationElEventBridgeDestinationEl {
        Sesv2ConfigurationSetEventDestinationEventDestinationElEventBridgeDestinationEl {
            event_bus_arn: self.event_bus_arn,
        }
    }
}

pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElEventBridgeDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Sesv2ConfigurationSetEventDestinationEventDestinationElEventBridgeDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Sesv2ConfigurationSetEventDestinationEventDestinationElEventBridgeDestinationElRef {
        Sesv2ConfigurationSetEventDestinationEventDestinationElEventBridgeDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElEventBridgeDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_bus_arn` after provisioning.\n"]
    pub fn event_bus_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_bus_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl {
    delivery_stream_arn: PrimField<String>,
    iam_role_arn: PrimField<String>,
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl { }

impl ToListMappable for Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl {
    type O = BlockAssignable<Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl {
    #[doc = ""]
    pub delivery_stream_arn: PrimField<String>,
    #[doc = ""]
    pub iam_role_arn: PrimField<String>,
}

impl BuildSesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl {
    pub fn build(self) -> Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl {
        Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl {
            delivery_stream_arn: self.delivery_stream_arn,
            iam_role_arn: self.iam_role_arn,
        }
    }
}

pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationElRef {
        Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delivery_stream_arn` after provisioning.\n"]
    pub fn delivery_stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delivery_stream_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl {
    application_arn: PrimField<String>,
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl { }

impl ToListMappable for Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl {
    type O = BlockAssignable<Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl {
    #[doc = ""]
    pub application_arn: PrimField<String>,
}

impl BuildSesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl {
    pub fn build(self) -> Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl {
        Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl {
            application_arn: self.application_arn,
        }
    }
}

pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationElRef {
        Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl {
    topic_arn: PrimField<String>,
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl { }

impl ToListMappable for Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl {
    type O = BlockAssignable<Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl {
    #[doc = ""]
    pub topic_arn: PrimField<String>,
}

impl BuildSesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl {
    pub fn build(self) -> Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl {
        Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl { topic_arn: self.topic_arn }
    }
}

pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationElRef {
        Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct Sesv2ConfigurationSetEventDestinationEventDestinationElDynamic {
    cloud_watch_destination: Option<
        DynamicBlock<Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl>,
    >,
    event_bridge_destination: Option<
        DynamicBlock<Sesv2ConfigurationSetEventDestinationEventDestinationElEventBridgeDestinationEl>,
    >,
    kinesis_firehose_destination: Option<
        DynamicBlock<Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl>,
    >,
    pinpoint_destination: Option<
        DynamicBlock<Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl>,
    >,
    sns_destination: Option<DynamicBlock<Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl>>,
}

#[derive(Serialize)]
pub struct Sesv2ConfigurationSetEventDestinationEventDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    matching_event_types: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_watch_destination: Option<Vec<Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_bridge_destination: Option<
        Vec<Sesv2ConfigurationSetEventDestinationEventDestinationElEventBridgeDestinationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_firehose_destination: Option<
        Vec<Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pinpoint_destination: Option<Vec<Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sns_destination: Option<Vec<Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl>>,
    dynamic: Sesv2ConfigurationSetEventDestinationEventDestinationElDynamic,
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_watch_destination`.\n"]
    pub fn set_cloud_watch_destination(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_watch_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_watch_destination = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `event_bridge_destination`.\n"]
    pub fn set_event_bridge_destination(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Sesv2ConfigurationSetEventDestinationEventDestinationElEventBridgeDestinationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.event_bridge_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.event_bridge_destination = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_firehose_destination`.\n"]
    pub fn set_kinesis_firehose_destination(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_firehose_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_firehose_destination = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pinpoint_destination`.\n"]
    pub fn set_pinpoint_destination(
        mut self,
        v: impl Into<BlockAssignable<Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pinpoint_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pinpoint_destination = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sns_destination`.\n"]
    pub fn set_sns_destination(
        mut self,
        v: impl Into<BlockAssignable<Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sns_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sns_destination = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Sesv2ConfigurationSetEventDestinationEventDestinationEl {
    type O = BlockAssignable<Sesv2ConfigurationSetEventDestinationEventDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2ConfigurationSetEventDestinationEventDestinationEl {
    #[doc = ""]
    pub matching_event_types: SetField<PrimField<String>>,
}

impl BuildSesv2ConfigurationSetEventDestinationEventDestinationEl {
    pub fn build(self) -> Sesv2ConfigurationSetEventDestinationEventDestinationEl {
        Sesv2ConfigurationSetEventDestinationEventDestinationEl {
            enabled: core::default::Default::default(),
            matching_event_types: self.matching_event_types,
            cloud_watch_destination: core::default::Default::default(),
            event_bridge_destination: core::default::Default::default(),
            kinesis_firehose_destination: core::default::Default::default(),
            pinpoint_destination: core::default::Default::default(),
            sns_destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Sesv2ConfigurationSetEventDestinationEventDestinationElRef {
    fn new(shared: StackShared, base: String) -> Sesv2ConfigurationSetEventDestinationEventDestinationElRef {
        Sesv2ConfigurationSetEventDestinationEventDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `matching_event_types` after provisioning.\n"]
    pub fn matching_event_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.matching_event_types", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_watch_destination` after provisioning.\n"]
    pub fn cloud_watch_destination(
        &self,
    ) -> ListRef<Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloud_watch_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `event_bridge_destination` after provisioning.\n"]
    pub fn event_bridge_destination(
        &self,
    ) -> ListRef<Sesv2ConfigurationSetEventDestinationEventDestinationElEventBridgeDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.event_bridge_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_firehose_destination` after provisioning.\n"]
    pub fn kinesis_firehose_destination(
        &self,
    ) -> ListRef<Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kinesis_firehose_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `pinpoint_destination` after provisioning.\n"]
    pub fn pinpoint_destination(
        &self,
    ) -> ListRef<Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pinpoint_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `sns_destination` after provisioning.\n"]
    pub fn sns_destination(&self) -> ListRef<Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sns_destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct Sesv2ConfigurationSetEventDestinationDynamic {
    event_destination: Option<DynamicBlock<Sesv2ConfigurationSetEventDestinationEventDestinationEl>>,
}
