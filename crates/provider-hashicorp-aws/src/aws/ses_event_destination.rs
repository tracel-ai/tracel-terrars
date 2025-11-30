use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SesEventDestinationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    configuration_set_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    matching_types: SetField<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_destination: Option<Vec<SesEventDestinationCloudwatchDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_destination: Option<Vec<SesEventDestinationKinesisDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sns_destination: Option<Vec<SesEventDestinationSnsDestinationEl>>,
    dynamic: SesEventDestinationDynamic,
}

struct SesEventDestination_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SesEventDestinationData>,
}

#[derive(Clone)]
pub struct SesEventDestination(Rc<SesEventDestination_>);

impl SesEventDestination {
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

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
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

    #[doc = "Set the field `cloudwatch_destination`.\n"]
    pub fn set_cloudwatch_destination(
        self,
        v: impl Into<BlockAssignable<SesEventDestinationCloudwatchDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudwatch_destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudwatch_destination = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `kinesis_destination`.\n"]
    pub fn set_kinesis_destination(
        self,
        v: impl Into<BlockAssignable<SesEventDestinationKinesisDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kinesis_destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kinesis_destination = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `sns_destination`.\n"]
    pub fn set_sns_destination(
        self,
        v: impl Into<BlockAssignable<SesEventDestinationSnsDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sns_destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sns_destination = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_set_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `matching_types` after provisioning.\n"]
    pub fn matching_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.matching_types", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `kinesis_destination` after provisioning.\n"]
    pub fn kinesis_destination(&self) -> ListRef<SesEventDestinationKinesisDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_destination", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sns_destination` after provisioning.\n"]
    pub fn sns_destination(&self) -> ListRef<SesEventDestinationSnsDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sns_destination", self.extract_ref()),
        )
    }
}

impl Referable for SesEventDestination {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SesEventDestination {}

impl ToListMappable for SesEventDestination {
    type O = ListRef<SesEventDestinationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SesEventDestination_ {
    fn extract_resource_type(&self) -> String {
        "aws_ses_event_destination".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSesEventDestination {
    pub tf_id: String,
    #[doc = ""]
    pub configuration_set_name: PrimField<String>,
    #[doc = ""]
    pub matching_types: SetField<PrimField<String>>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildSesEventDestination {
    pub fn build(self, stack: &mut Stack) -> SesEventDestination {
        let out = SesEventDestination(Rc::new(SesEventDestination_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SesEventDestinationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                configuration_set_name: self.configuration_set_name,
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                matching_types: self.matching_types,
                name: self.name,
                region: core::default::Default::default(),
                cloudwatch_destination: core::default::Default::default(),
                kinesis_destination: core::default::Default::default(),
                sns_destination: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SesEventDestinationRef {
    shared: StackShared,
    base: String,
}

impl Ref for SesEventDestinationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SesEventDestinationRef {
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

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_set_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `matching_types` after provisioning.\n"]
    pub fn matching_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.matching_types", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `kinesis_destination` after provisioning.\n"]
    pub fn kinesis_destination(&self) -> ListRef<SesEventDestinationKinesisDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_destination", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sns_destination` after provisioning.\n"]
    pub fn sns_destination(&self) -> ListRef<SesEventDestinationSnsDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sns_destination", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SesEventDestinationCloudwatchDestinationEl {
    default_value: PrimField<String>,
    dimension_name: PrimField<String>,
    value_source: PrimField<String>,
}

impl SesEventDestinationCloudwatchDestinationEl {}

impl ToListMappable for SesEventDestinationCloudwatchDestinationEl {
    type O = BlockAssignable<SesEventDestinationCloudwatchDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesEventDestinationCloudwatchDestinationEl {
    #[doc = ""]
    pub default_value: PrimField<String>,
    #[doc = ""]
    pub dimension_name: PrimField<String>,
    #[doc = ""]
    pub value_source: PrimField<String>,
}

impl BuildSesEventDestinationCloudwatchDestinationEl {
    pub fn build(self) -> SesEventDestinationCloudwatchDestinationEl {
        SesEventDestinationCloudwatchDestinationEl {
            default_value: self.default_value,
            dimension_name: self.dimension_name,
            value_source: self.value_source,
        }
    }
}

pub struct SesEventDestinationCloudwatchDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SesEventDestinationCloudwatchDestinationElRef {
    fn new(shared: StackShared, base: String) -> SesEventDestinationCloudwatchDestinationElRef {
        SesEventDestinationCloudwatchDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SesEventDestinationCloudwatchDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_value", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension_name` after provisioning.\n"]
    pub fn dimension_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dimension_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `value_source` after provisioning.\n"]
    pub fn value_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value_source", self.base))
    }
}

#[derive(Serialize)]
pub struct SesEventDestinationKinesisDestinationEl {
    role_arn: PrimField<String>,
    stream_arn: PrimField<String>,
}

impl SesEventDestinationKinesisDestinationEl {}

impl ToListMappable for SesEventDestinationKinesisDestinationEl {
    type O = BlockAssignable<SesEventDestinationKinesisDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesEventDestinationKinesisDestinationEl {
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub stream_arn: PrimField<String>,
}

impl BuildSesEventDestinationKinesisDestinationEl {
    pub fn build(self) -> SesEventDestinationKinesisDestinationEl {
        SesEventDestinationKinesisDestinationEl {
            role_arn: self.role_arn,
            stream_arn: self.stream_arn,
        }
    }
}

pub struct SesEventDestinationKinesisDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SesEventDestinationKinesisDestinationElRef {
    fn new(shared: StackShared, base: String) -> SesEventDestinationKinesisDestinationElRef {
        SesEventDestinationKinesisDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SesEventDestinationKinesisDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SesEventDestinationSnsDestinationEl {
    topic_arn: PrimField<String>,
}

impl SesEventDestinationSnsDestinationEl {}

impl ToListMappable for SesEventDestinationSnsDestinationEl {
    type O = BlockAssignable<SesEventDestinationSnsDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesEventDestinationSnsDestinationEl {
    #[doc = ""]
    pub topic_arn: PrimField<String>,
}

impl BuildSesEventDestinationSnsDestinationEl {
    pub fn build(self) -> SesEventDestinationSnsDestinationEl {
        SesEventDestinationSnsDestinationEl {
            topic_arn: self.topic_arn,
        }
    }
}

pub struct SesEventDestinationSnsDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SesEventDestinationSnsDestinationElRef {
    fn new(shared: StackShared, base: String) -> SesEventDestinationSnsDestinationElRef {
        SesEventDestinationSnsDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SesEventDestinationSnsDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SesEventDestinationDynamic {
    cloudwatch_destination: Option<DynamicBlock<SesEventDestinationCloudwatchDestinationEl>>,
    kinesis_destination: Option<DynamicBlock<SesEventDestinationKinesisDestinationEl>>,
    sns_destination: Option<DynamicBlock<SesEventDestinationSnsDestinationEl>>,
}
