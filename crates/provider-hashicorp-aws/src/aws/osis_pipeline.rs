use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct OsisPipelineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    max_units: PrimField<f64>,
    min_units: PrimField<f64>,
    pipeline_configuration_body: PrimField<String>,
    pipeline_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffer_options: Option<Vec<OsisPipelineBufferOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_at_rest_options: Option<Vec<OsisPipelineEncryptionAtRestOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_publishing_options: Option<Vec<OsisPipelineLogPublishingOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OsisPipelineTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_options: Option<Vec<OsisPipelineVpcOptionsEl>>,
    dynamic: OsisPipelineDynamic,
}
struct OsisPipeline_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OsisPipelineData>,
}
#[derive(Clone)]
pub struct OsisPipeline(Rc<OsisPipeline_>);
impl OsisPipeline {
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
    #[doc = "Set the field `buffer_options`.\n"]
    pub fn set_buffer_options(
        self,
        v: impl Into<BlockAssignable<OsisPipelineBufferOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().buffer_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.buffer_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `encryption_at_rest_options`.\n"]
    pub fn set_encryption_at_rest_options(
        self,
        v: impl Into<BlockAssignable<OsisPipelineEncryptionAtRestOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_at_rest_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_at_rest_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `log_publishing_options`.\n"]
    pub fn set_log_publishing_options(
        self,
        v: impl Into<BlockAssignable<OsisPipelineLogPublishingOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_publishing_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_publishing_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<OsisPipelineTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_options`.\n"]
    pub fn set_vpc_options(self, v: impl Into<BlockAssignable<OsisPipelineVpcOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_options = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ingest_endpoint_urls` after provisioning.\n"]
    pub fn ingest_endpoint_urls(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ingest_endpoint_urls", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_units` after provisioning.\n"]
    pub fn max_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_units", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_units` after provisioning.\n"]
    pub fn min_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_units", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pipeline_arn` after provisioning.\n"]
    pub fn pipeline_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pipeline_configuration_body` after provisioning.\n"]
    pub fn pipeline_configuration_body(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_configuration_body", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pipeline_name` after provisioning.\n"]
    pub fn pipeline_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `buffer_options` after provisioning.\n"]
    pub fn buffer_options(&self) -> ListRef<OsisPipelineBufferOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.buffer_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_at_rest_options` after provisioning.\n"]
    pub fn encryption_at_rest_options(&self) -> ListRef<OsisPipelineEncryptionAtRestOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_at_rest_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_publishing_options` after provisioning.\n"]
    pub fn log_publishing_options(&self) -> ListRef<OsisPipelineLogPublishingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_publishing_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OsisPipelineTimeoutsElRef {
        OsisPipelineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> ListRef<OsisPipelineVpcOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_options", self.extract_ref()),
        )
    }
}
impl Referable for OsisPipeline {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for OsisPipeline {}
impl ToListMappable for OsisPipeline {
    type O = ListRef<OsisPipelineRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for OsisPipeline_ {
    fn extract_resource_type(&self) -> String {
        "aws_osis_pipeline".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildOsisPipeline {
    pub tf_id: String,
    #[doc = ""]
    pub max_units: PrimField<f64>,
    #[doc = ""]
    pub min_units: PrimField<f64>,
    #[doc = ""]
    pub pipeline_configuration_body: PrimField<String>,
    #[doc = ""]
    pub pipeline_name: PrimField<String>,
}
impl BuildOsisPipeline {
    pub fn build(self, stack: &mut Stack) -> OsisPipeline {
        let out = OsisPipeline(Rc::new(OsisPipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OsisPipelineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                max_units: self.max_units,
                min_units: self.min_units,
                pipeline_configuration_body: self.pipeline_configuration_body,
                pipeline_name: self.pipeline_name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                buffer_options: core::default::Default::default(),
                encryption_at_rest_options: core::default::Default::default(),
                log_publishing_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_options: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct OsisPipelineRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsisPipelineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl OsisPipelineRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ingest_endpoint_urls` after provisioning.\n"]
    pub fn ingest_endpoint_urls(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ingest_endpoint_urls", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_units` after provisioning.\n"]
    pub fn max_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_units", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_units` after provisioning.\n"]
    pub fn min_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_units", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pipeline_arn` after provisioning.\n"]
    pub fn pipeline_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pipeline_configuration_body` after provisioning.\n"]
    pub fn pipeline_configuration_body(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_configuration_body", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pipeline_name` after provisioning.\n"]
    pub fn pipeline_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `buffer_options` after provisioning.\n"]
    pub fn buffer_options(&self) -> ListRef<OsisPipelineBufferOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.buffer_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_at_rest_options` after provisioning.\n"]
    pub fn encryption_at_rest_options(&self) -> ListRef<OsisPipelineEncryptionAtRestOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_at_rest_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_publishing_options` after provisioning.\n"]
    pub fn log_publishing_options(&self) -> ListRef<OsisPipelineLogPublishingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_publishing_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OsisPipelineTimeoutsElRef {
        OsisPipelineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> ListRef<OsisPipelineVpcOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_options", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct OsisPipelineBufferOptionsEl {
    persistent_buffer_enabled: PrimField<bool>,
}
impl OsisPipelineBufferOptionsEl {}
impl ToListMappable for OsisPipelineBufferOptionsEl {
    type O = BlockAssignable<OsisPipelineBufferOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsisPipelineBufferOptionsEl {
    #[doc = ""]
    pub persistent_buffer_enabled: PrimField<bool>,
}
impl BuildOsisPipelineBufferOptionsEl {
    pub fn build(self) -> OsisPipelineBufferOptionsEl {
        OsisPipelineBufferOptionsEl {
            persistent_buffer_enabled: self.persistent_buffer_enabled,
        }
    }
}
pub struct OsisPipelineBufferOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsisPipelineBufferOptionsElRef {
    fn new(shared: StackShared, base: String) -> OsisPipelineBufferOptionsElRef {
        OsisPipelineBufferOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsisPipelineBufferOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `persistent_buffer_enabled` after provisioning.\n"]
    pub fn persistent_buffer_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.persistent_buffer_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OsisPipelineEncryptionAtRestOptionsEl {
    kms_key_arn: PrimField<String>,
}
impl OsisPipelineEncryptionAtRestOptionsEl {}
impl ToListMappable for OsisPipelineEncryptionAtRestOptionsEl {
    type O = BlockAssignable<OsisPipelineEncryptionAtRestOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsisPipelineEncryptionAtRestOptionsEl {
    #[doc = ""]
    pub kms_key_arn: PrimField<String>,
}
impl BuildOsisPipelineEncryptionAtRestOptionsEl {
    pub fn build(self) -> OsisPipelineEncryptionAtRestOptionsEl {
        OsisPipelineEncryptionAtRestOptionsEl {
            kms_key_arn: self.kms_key_arn,
        }
    }
}
pub struct OsisPipelineEncryptionAtRestOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsisPipelineEncryptionAtRestOptionsElRef {
    fn new(shared: StackShared, base: String) -> OsisPipelineEncryptionAtRestOptionsElRef {
        OsisPipelineEncryptionAtRestOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsisPipelineEncryptionAtRestOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct OsisPipelineLogPublishingOptionsElCloudwatchLogDestinationEl {
    log_group: PrimField<String>,
}
impl OsisPipelineLogPublishingOptionsElCloudwatchLogDestinationEl {}
impl ToListMappable for OsisPipelineLogPublishingOptionsElCloudwatchLogDestinationEl {
    type O = BlockAssignable<OsisPipelineLogPublishingOptionsElCloudwatchLogDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsisPipelineLogPublishingOptionsElCloudwatchLogDestinationEl {
    #[doc = ""]
    pub log_group: PrimField<String>,
}
impl BuildOsisPipelineLogPublishingOptionsElCloudwatchLogDestinationEl {
    pub fn build(self) -> OsisPipelineLogPublishingOptionsElCloudwatchLogDestinationEl {
        OsisPipelineLogPublishingOptionsElCloudwatchLogDestinationEl {
            log_group: self.log_group,
        }
    }
}
pub struct OsisPipelineLogPublishingOptionsElCloudwatchLogDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsisPipelineLogPublishingOptionsElCloudwatchLogDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsisPipelineLogPublishingOptionsElCloudwatchLogDestinationElRef {
        OsisPipelineLogPublishingOptionsElCloudwatchLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsisPipelineLogPublishingOptionsElCloudwatchLogDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsisPipelineLogPublishingOptionsElDynamic {
    cloudwatch_log_destination:
        Option<DynamicBlock<OsisPipelineLogPublishingOptionsElCloudwatchLogDestinationEl>>,
}
#[derive(Serialize)]
pub struct OsisPipelineLogPublishingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_logging_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_log_destination:
        Option<Vec<OsisPipelineLogPublishingOptionsElCloudwatchLogDestinationEl>>,
    dynamic: OsisPipelineLogPublishingOptionsElDynamic,
}
impl OsisPipelineLogPublishingOptionsEl {
    #[doc = "Set the field `is_logging_enabled`.\n"]
    pub fn set_is_logging_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_logging_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `cloudwatch_log_destination`.\n"]
    pub fn set_cloudwatch_log_destination(
        mut self,
        v: impl Into<BlockAssignable<OsisPipelineLogPublishingOptionsElCloudwatchLogDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_log_destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_log_destination = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsisPipelineLogPublishingOptionsEl {
    type O = BlockAssignable<OsisPipelineLogPublishingOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsisPipelineLogPublishingOptionsEl {}
impl BuildOsisPipelineLogPublishingOptionsEl {
    pub fn build(self) -> OsisPipelineLogPublishingOptionsEl {
        OsisPipelineLogPublishingOptionsEl {
            is_logging_enabled: core::default::Default::default(),
            cloudwatch_log_destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsisPipelineLogPublishingOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsisPipelineLogPublishingOptionsElRef {
    fn new(shared: StackShared, base: String) -> OsisPipelineLogPublishingOptionsElRef {
        OsisPipelineLogPublishingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsisPipelineLogPublishingOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_logging_enabled` after provisioning.\n"]
    pub fn is_logging_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_logging_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloudwatch_log_destination` after provisioning.\n"]
    pub fn cloudwatch_log_destination(
        &self,
    ) -> ListRef<OsisPipelineLogPublishingOptionsElCloudwatchLogDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_log_destination", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OsisPipelineTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl OsisPipelineTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for OsisPipelineTimeoutsEl {
    type O = BlockAssignable<OsisPipelineTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsisPipelineTimeoutsEl {}
impl BuildOsisPipelineTimeoutsEl {
    pub fn build(self) -> OsisPipelineTimeoutsEl {
        OsisPipelineTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct OsisPipelineTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsisPipelineTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OsisPipelineTimeoutsElRef {
        OsisPipelineTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsisPipelineTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize)]
pub struct OsisPipelineVpcOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    subnet_ids: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_endpoint_management: Option<PrimField<String>>,
}
impl OsisPipelineVpcOptionsEl {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_endpoint_management`.\n"]
    pub fn set_vpc_endpoint_management(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_endpoint_management = Some(v.into());
        self
    }
}
impl ToListMappable for OsisPipelineVpcOptionsEl {
    type O = BlockAssignable<OsisPipelineVpcOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsisPipelineVpcOptionsEl {
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
}
impl BuildOsisPipelineVpcOptionsEl {
    pub fn build(self) -> OsisPipelineVpcOptionsEl {
        OsisPipelineVpcOptionsEl {
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
            vpc_endpoint_management: core::default::Default::default(),
        }
    }
}
pub struct OsisPipelineVpcOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsisPipelineVpcOptionsElRef {
    fn new(shared: StackShared, base: String) -> OsisPipelineVpcOptionsElRef {
        OsisPipelineVpcOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsisPipelineVpcOptionsElRef {
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
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_endpoint_management` after provisioning.\n"]
    pub fn vpc_endpoint_management(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_endpoint_management", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct OsisPipelineDynamic {
    buffer_options: Option<DynamicBlock<OsisPipelineBufferOptionsEl>>,
    encryption_at_rest_options: Option<DynamicBlock<OsisPipelineEncryptionAtRestOptionsEl>>,
    log_publishing_options: Option<DynamicBlock<OsisPipelineLogPublishingOptionsEl>>,
    vpc_options: Option<DynamicBlock<OsisPipelineVpcOptionsEl>>,
}
