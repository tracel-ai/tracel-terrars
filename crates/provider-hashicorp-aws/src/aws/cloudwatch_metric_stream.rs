use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudwatchMetricStreamData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    firehose_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_linked_accounts_metrics: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    output_format: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_filter: Option<Vec<CloudwatchMetricStreamExcludeFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_filter: Option<Vec<CloudwatchMetricStreamIncludeFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statistics_configuration: Option<Vec<CloudwatchMetricStreamStatisticsConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudwatchMetricStreamTimeoutsEl>,
    dynamic: CloudwatchMetricStreamDynamic,
}
struct CloudwatchMetricStream_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudwatchMetricStreamData>,
}
#[derive(Clone)]
pub struct CloudwatchMetricStream(Rc<CloudwatchMetricStream_>);
impl CloudwatchMetricStream {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `include_linked_accounts_metrics`.\n"]
    pub fn set_include_linked_accounts_metrics(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_linked_accounts_metrics = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
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
    #[doc = "Set the field `exclude_filter`.\n"]
    pub fn set_exclude_filter(
        self,
        v: impl Into<BlockAssignable<CloudwatchMetricStreamExcludeFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().exclude_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.exclude_filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `include_filter`.\n"]
    pub fn set_include_filter(
        self,
        v: impl Into<BlockAssignable<CloudwatchMetricStreamIncludeFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().include_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.include_filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `statistics_configuration`.\n"]
    pub fn set_statistics_configuration(
        self,
        v: impl Into<BlockAssignable<CloudwatchMetricStreamStatisticsConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().statistics_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.statistics_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudwatchMetricStreamTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `firehose_arn` after provisioning.\n"]
    pub fn firehose_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.firehose_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `include_linked_accounts_metrics` after provisioning.\n"]
    pub fn include_linked_accounts_metrics(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_linked_accounts_metrics", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_update_date` after provisioning.\n"]
    pub fn last_update_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_update_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudwatchMetricStreamTimeoutsElRef {
        CloudwatchMetricStreamTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CloudwatchMetricStream {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudwatchMetricStream {}
impl ToListMappable for CloudwatchMetricStream {
    type O = ListRef<CloudwatchMetricStreamRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudwatchMetricStream_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudwatch_metric_stream".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudwatchMetricStream {
    pub tf_id: String,
    #[doc = ""]
    pub firehose_arn: PrimField<String>,
    #[doc = ""]
    pub output_format: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildCloudwatchMetricStream {
    pub fn build(self, stack: &mut Stack) -> CloudwatchMetricStream {
        let out = CloudwatchMetricStream(Rc::new(CloudwatchMetricStream_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudwatchMetricStreamData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                firehose_arn: self.firehose_arn,
                id: core::default::Default::default(),
                include_linked_accounts_metrics: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                output_format: self.output_format,
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                exclude_filter: core::default::Default::default(),
                include_filter: core::default::Default::default(),
                statistics_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudwatchMetricStreamRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchMetricStreamRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudwatchMetricStreamRef {
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
    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `firehose_arn` after provisioning.\n"]
    pub fn firehose_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.firehose_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `include_linked_accounts_metrics` after provisioning.\n"]
    pub fn include_linked_accounts_metrics(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_linked_accounts_metrics", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_update_date` after provisioning.\n"]
    pub fn last_update_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_update_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudwatchMetricStreamTimeoutsElRef {
        CloudwatchMetricStreamTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudwatchMetricStreamExcludeFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_names: Option<SetField<PrimField<String>>>,
    namespace: PrimField<String>,
}
impl CloudwatchMetricStreamExcludeFilterEl {
    #[doc = "Set the field `metric_names`.\n"]
    pub fn set_metric_names(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.metric_names = Some(v.into());
        self
    }
}
impl ToListMappable for CloudwatchMetricStreamExcludeFilterEl {
    type O = BlockAssignable<CloudwatchMetricStreamExcludeFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchMetricStreamExcludeFilterEl {
    #[doc = ""]
    pub namespace: PrimField<String>,
}
impl BuildCloudwatchMetricStreamExcludeFilterEl {
    pub fn build(self) -> CloudwatchMetricStreamExcludeFilterEl {
        CloudwatchMetricStreamExcludeFilterEl {
            metric_names: core::default::Default::default(),
            namespace: self.namespace,
        }
    }
}
pub struct CloudwatchMetricStreamExcludeFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchMetricStreamExcludeFilterElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchMetricStreamExcludeFilterElRef {
        CloudwatchMetricStreamExcludeFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchMetricStreamExcludeFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metric_names` after provisioning.\n"]
    pub fn metric_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.metric_names", self.base))
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudwatchMetricStreamIncludeFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_names: Option<SetField<PrimField<String>>>,
    namespace: PrimField<String>,
}
impl CloudwatchMetricStreamIncludeFilterEl {
    #[doc = "Set the field `metric_names`.\n"]
    pub fn set_metric_names(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.metric_names = Some(v.into());
        self
    }
}
impl ToListMappable for CloudwatchMetricStreamIncludeFilterEl {
    type O = BlockAssignable<CloudwatchMetricStreamIncludeFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchMetricStreamIncludeFilterEl {
    #[doc = ""]
    pub namespace: PrimField<String>,
}
impl BuildCloudwatchMetricStreamIncludeFilterEl {
    pub fn build(self) -> CloudwatchMetricStreamIncludeFilterEl {
        CloudwatchMetricStreamIncludeFilterEl {
            metric_names: core::default::Default::default(),
            namespace: self.namespace,
        }
    }
}
pub struct CloudwatchMetricStreamIncludeFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchMetricStreamIncludeFilterElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchMetricStreamIncludeFilterElRef {
        CloudwatchMetricStreamIncludeFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchMetricStreamIncludeFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metric_names` after provisioning.\n"]
    pub fn metric_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.metric_names", self.base))
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl {
    metric_name: PrimField<String>,
    namespace: PrimField<String>,
}
impl CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl {}
impl ToListMappable for CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl {
    type O = BlockAssignable<CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl {
    #[doc = ""]
    pub metric_name: PrimField<String>,
    #[doc = ""]
    pub namespace: PrimField<String>,
}
impl BuildCloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl {
    pub fn build(self) -> CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl {
        CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl {
            metric_name: self.metric_name,
            namespace: self.namespace,
        }
    }
}
pub struct CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricElRef {
        CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudwatchMetricStreamStatisticsConfigurationElDynamic {
    include_metric:
        Option<DynamicBlock<CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl>>,
}
#[derive(Serialize)]
pub struct CloudwatchMetricStreamStatisticsConfigurationEl {
    additional_statistics: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_metric: Option<Vec<CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl>>,
    dynamic: CloudwatchMetricStreamStatisticsConfigurationElDynamic,
}
impl CloudwatchMetricStreamStatisticsConfigurationEl {
    #[doc = "Set the field `include_metric`.\n"]
    pub fn set_include_metric(
        mut self,
        v: impl Into<BlockAssignable<CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.include_metric = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.include_metric = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudwatchMetricStreamStatisticsConfigurationEl {
    type O = BlockAssignable<CloudwatchMetricStreamStatisticsConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchMetricStreamStatisticsConfigurationEl {
    #[doc = ""]
    pub additional_statistics: SetField<PrimField<String>>,
}
impl BuildCloudwatchMetricStreamStatisticsConfigurationEl {
    pub fn build(self) -> CloudwatchMetricStreamStatisticsConfigurationEl {
        CloudwatchMetricStreamStatisticsConfigurationEl {
            additional_statistics: self.additional_statistics,
            include_metric: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudwatchMetricStreamStatisticsConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchMetricStreamStatisticsConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchMetricStreamStatisticsConfigurationElRef {
        CloudwatchMetricStreamStatisticsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchMetricStreamStatisticsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_statistics` after provisioning.\n"]
    pub fn additional_statistics(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.additional_statistics", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudwatchMetricStreamTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CloudwatchMetricStreamTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for CloudwatchMetricStreamTimeoutsEl {
    type O = BlockAssignable<CloudwatchMetricStreamTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchMetricStreamTimeoutsEl {}
impl BuildCloudwatchMetricStreamTimeoutsEl {
    pub fn build(self) -> CloudwatchMetricStreamTimeoutsEl {
        CloudwatchMetricStreamTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CloudwatchMetricStreamTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchMetricStreamTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchMetricStreamTimeoutsElRef {
        CloudwatchMetricStreamTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchMetricStreamTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudwatchMetricStreamDynamic {
    exclude_filter: Option<DynamicBlock<CloudwatchMetricStreamExcludeFilterEl>>,
    include_filter: Option<DynamicBlock<CloudwatchMetricStreamIncludeFilterEl>>,
    statistics_configuration: Option<DynamicBlock<CloudwatchMetricStreamStatisticsConfigurationEl>>,
}
