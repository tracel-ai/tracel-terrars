use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct FlowLogData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deliver_cross_account_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eni_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_destination: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_destination_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_aggregation_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traffic_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_attachment_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_options: Option<Vec<FlowLogDestinationOptionsEl>>,
    dynamic: FlowLogDynamic,
}

struct FlowLog_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FlowLogData>,
}

#[derive(Clone)]
pub struct FlowLog(Rc<FlowLog_>);

impl FlowLog {
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

    #[doc = "Set the field `deliver_cross_account_role`.\n"]
    pub fn set_deliver_cross_account_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deliver_cross_account_role = Some(v.into());
        self
    }

    #[doc = "Set the field `eni_id`.\n"]
    pub fn set_eni_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().eni_id = Some(v.into());
        self
    }

    #[doc = "Set the field `iam_role_arn`.\n"]
    pub fn set_iam_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().iam_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `log_destination`.\n"]
    pub fn set_log_destination(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().log_destination = Some(v.into());
        self
    }

    #[doc = "Set the field `log_destination_type`.\n"]
    pub fn set_log_destination_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().log_destination_type = Some(v.into());
        self
    }

    #[doc = "Set the field `log_format`.\n"]
    pub fn set_log_format(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().log_format = Some(v.into());
        self
    }

    #[doc = "Set the field `max_aggregation_interval`.\n"]
    pub fn set_max_aggregation_interval(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_aggregation_interval = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subnet_id = Some(v.into());
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

    #[doc = "Set the field `traffic_type`.\n"]
    pub fn set_traffic_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().traffic_type = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_attachment_id`.\n"]
    pub fn set_transit_gateway_attachment_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().transit_gateway_attachment_id = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_id`.\n"]
    pub fn set_transit_gateway_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().transit_gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpc_id = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_options`.\n"]
    pub fn set_destination_options(
        self,
        v: impl Into<BlockAssignable<FlowLogDestinationOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_options = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deliver_cross_account_role` after provisioning.\n"]
    pub fn deliver_cross_account_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deliver_cross_account_role", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `eni_id` after provisioning.\n"]
    pub fn eni_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.eni_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_destination", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_destination_type` after provisioning.\n"]
    pub fn log_destination_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_destination_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_format` after provisioning.\n"]
    pub fn log_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_format", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_aggregation_interval` after provisioning.\n"]
    pub fn max_aggregation_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_aggregation_interval", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet_id", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `traffic_type` after provisioning.\n"]
    pub fn traffic_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traffic_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_attachment_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination_options` after provisioning.\n"]
    pub fn destination_options(&self) -> ListRef<FlowLogDestinationOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_options", self.extract_ref()),
        )
    }
}

impl Referable for FlowLog {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for FlowLog {}

impl ToListMappable for FlowLog {
    type O = ListRef<FlowLogRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for FlowLog_ {
    fn extract_resource_type(&self) -> String {
        "aws_flow_log".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFlowLog {
    pub tf_id: String,
}

impl BuildFlowLog {
    pub fn build(self, stack: &mut Stack) -> FlowLog {
        let out = FlowLog(Rc::new(FlowLog_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FlowLogData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deliver_cross_account_role: core::default::Default::default(),
                eni_id: core::default::Default::default(),
                iam_role_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                log_destination: core::default::Default::default(),
                log_destination_type: core::default::Default::default(),
                log_format: core::default::Default::default(),
                max_aggregation_interval: core::default::Default::default(),
                region: core::default::Default::default(),
                subnet_id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                traffic_type: core::default::Default::default(),
                transit_gateway_attachment_id: core::default::Default::default(),
                transit_gateway_id: core::default::Default::default(),
                vpc_id: core::default::Default::default(),
                destination_options: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FlowLogRef {
    shared: StackShared,
    base: String,
}

impl Ref for FlowLogRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl FlowLogRef {
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

    #[doc = "Get a reference to the value of field `deliver_cross_account_role` after provisioning.\n"]
    pub fn deliver_cross_account_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deliver_cross_account_role", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `eni_id` after provisioning.\n"]
    pub fn eni_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.eni_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_destination", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_destination_type` after provisioning.\n"]
    pub fn log_destination_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_destination_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_format` after provisioning.\n"]
    pub fn log_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_format", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_aggregation_interval` after provisioning.\n"]
    pub fn max_aggregation_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_aggregation_interval", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet_id", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `traffic_type` after provisioning.\n"]
    pub fn traffic_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traffic_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_attachment_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination_options` after provisioning.\n"]
    pub fn destination_options(&self) -> ListRef<FlowLogDestinationOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_options", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct FlowLogDestinationOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hive_compatible_partitions: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_hour_partition: Option<PrimField<bool>>,
}

impl FlowLogDestinationOptionsEl {
    #[doc = "Set the field `file_format`.\n"]
    pub fn set_file_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_format = Some(v.into());
        self
    }

    #[doc = "Set the field `hive_compatible_partitions`.\n"]
    pub fn set_hive_compatible_partitions(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.hive_compatible_partitions = Some(v.into());
        self
    }

    #[doc = "Set the field `per_hour_partition`.\n"]
    pub fn set_per_hour_partition(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.per_hour_partition = Some(v.into());
        self
    }
}

impl ToListMappable for FlowLogDestinationOptionsEl {
    type O = BlockAssignable<FlowLogDestinationOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFlowLogDestinationOptionsEl {}

impl BuildFlowLogDestinationOptionsEl {
    pub fn build(self) -> FlowLogDestinationOptionsEl {
        FlowLogDestinationOptionsEl {
            file_format: core::default::Default::default(),
            hive_compatible_partitions: core::default::Default::default(),
            per_hour_partition: core::default::Default::default(),
        }
    }
}

pub struct FlowLogDestinationOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FlowLogDestinationOptionsElRef {
    fn new(shared: StackShared, base: String) -> FlowLogDestinationOptionsElRef {
        FlowLogDestinationOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FlowLogDestinationOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_format` after provisioning.\n"]
    pub fn file_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_format", self.base))
    }

    #[doc = "Get a reference to the value of field `hive_compatible_partitions` after provisioning.\n"]
    pub fn hive_compatible_partitions(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hive_compatible_partitions", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `per_hour_partition` after provisioning.\n"]
    pub fn per_hour_partition(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.per_hour_partition", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct FlowLogDynamic {
    destination_options: Option<DynamicBlock<FlowLogDestinationOptionsEl>>,
}
