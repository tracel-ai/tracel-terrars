use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VerifiedaccessInstanceLoggingConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    verifiedaccess_instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_logs: Option<Vec<VerifiedaccessInstanceLoggingConfigurationAccessLogsEl>>,
    dynamic: VerifiedaccessInstanceLoggingConfigurationDynamic,
}
struct VerifiedaccessInstanceLoggingConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VerifiedaccessInstanceLoggingConfigurationData>,
}
#[derive(Clone)]
pub struct VerifiedaccessInstanceLoggingConfiguration(
    Rc<VerifiedaccessInstanceLoggingConfiguration_>,
);
impl VerifiedaccessInstanceLoggingConfiguration {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `access_logs`.\n"]
    pub fn set_access_logs(
        self,
        v: impl Into<BlockAssignable<VerifiedaccessInstanceLoggingConfigurationAccessLogsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_logs = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `verifiedaccess_instance_id` after provisioning.\n"]
    pub fn verifiedaccess_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verifiedaccess_instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_logs` after provisioning.\n"]
    pub fn access_logs(
        &self,
    ) -> ListRef<VerifiedaccessInstanceLoggingConfigurationAccessLogsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_logs", self.extract_ref()),
        )
    }
}
impl Referable for VerifiedaccessInstanceLoggingConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VerifiedaccessInstanceLoggingConfiguration {}
impl ToListMappable for VerifiedaccessInstanceLoggingConfiguration {
    type O = ListRef<VerifiedaccessInstanceLoggingConfigurationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VerifiedaccessInstanceLoggingConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_verifiedaccess_instance_logging_configuration".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVerifiedaccessInstanceLoggingConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub verifiedaccess_instance_id: PrimField<String>,
}
impl BuildVerifiedaccessInstanceLoggingConfiguration {
    pub fn build(self, stack: &mut Stack) -> VerifiedaccessInstanceLoggingConfiguration {
        let out = VerifiedaccessInstanceLoggingConfiguration(Rc::new(
            VerifiedaccessInstanceLoggingConfiguration_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(VerifiedaccessInstanceLoggingConfigurationData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    id: core::default::Default::default(),
                    region: core::default::Default::default(),
                    verifiedaccess_instance_id: self.verifiedaccess_instance_id,
                    access_logs: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VerifiedaccessInstanceLoggingConfigurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for VerifiedaccessInstanceLoggingConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VerifiedaccessInstanceLoggingConfigurationRef {
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `verifiedaccess_instance_id` after provisioning.\n"]
    pub fn verifiedaccess_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verifiedaccess_instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_logs` after provisioning.\n"]
    pub fn access_logs(
        &self,
    ) -> ListRef<VerifiedaccessInstanceLoggingConfigurationAccessLogsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_logs", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VerifiedaccessInstanceLoggingConfigurationAccessLogsElCloudwatchLogsEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group: Option<PrimField<String>>,
}
impl VerifiedaccessInstanceLoggingConfigurationAccessLogsElCloudwatchLogsEl {
    #[doc = "Set the field `log_group`.\n"]
    pub fn set_log_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group = Some(v.into());
        self
    }
}
impl ToListMappable for VerifiedaccessInstanceLoggingConfigurationAccessLogsElCloudwatchLogsEl {
    type O =
        BlockAssignable<VerifiedaccessInstanceLoggingConfigurationAccessLogsElCloudwatchLogsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVerifiedaccessInstanceLoggingConfigurationAccessLogsElCloudwatchLogsEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}
impl BuildVerifiedaccessInstanceLoggingConfigurationAccessLogsElCloudwatchLogsEl {
    pub fn build(self) -> VerifiedaccessInstanceLoggingConfigurationAccessLogsElCloudwatchLogsEl {
        VerifiedaccessInstanceLoggingConfigurationAccessLogsElCloudwatchLogsEl {
            enabled: self.enabled,
            log_group: core::default::Default::default(),
        }
    }
}
pub struct VerifiedaccessInstanceLoggingConfigurationAccessLogsElCloudwatchLogsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VerifiedaccessInstanceLoggingConfigurationAccessLogsElCloudwatchLogsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VerifiedaccessInstanceLoggingConfigurationAccessLogsElCloudwatchLogsElRef {
        VerifiedaccessInstanceLoggingConfigurationAccessLogsElCloudwatchLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VerifiedaccessInstanceLoggingConfigurationAccessLogsElCloudwatchLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }
}
#[derive(Serialize)]
pub struct VerifiedaccessInstanceLoggingConfigurationAccessLogsElKinesisDataFirehoseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_stream: Option<PrimField<String>>,
    enabled: PrimField<bool>,
}
impl VerifiedaccessInstanceLoggingConfigurationAccessLogsElKinesisDataFirehoseEl {
    #[doc = "Set the field `delivery_stream`.\n"]
    pub fn set_delivery_stream(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delivery_stream = Some(v.into());
        self
    }
}
impl ToListMappable
    for VerifiedaccessInstanceLoggingConfigurationAccessLogsElKinesisDataFirehoseEl
{
    type O = BlockAssignable<
        VerifiedaccessInstanceLoggingConfigurationAccessLogsElKinesisDataFirehoseEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVerifiedaccessInstanceLoggingConfigurationAccessLogsElKinesisDataFirehoseEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}
impl BuildVerifiedaccessInstanceLoggingConfigurationAccessLogsElKinesisDataFirehoseEl {
    pub fn build(
        self,
    ) -> VerifiedaccessInstanceLoggingConfigurationAccessLogsElKinesisDataFirehoseEl {
        VerifiedaccessInstanceLoggingConfigurationAccessLogsElKinesisDataFirehoseEl {
            delivery_stream: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}
pub struct VerifiedaccessInstanceLoggingConfigurationAccessLogsElKinesisDataFirehoseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VerifiedaccessInstanceLoggingConfigurationAccessLogsElKinesisDataFirehoseElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VerifiedaccessInstanceLoggingConfigurationAccessLogsElKinesisDataFirehoseElRef {
        VerifiedaccessInstanceLoggingConfigurationAccessLogsElKinesisDataFirehoseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VerifiedaccessInstanceLoggingConfigurationAccessLogsElKinesisDataFirehoseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delivery_stream` after provisioning.\n"]
    pub fn delivery_stream(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_stream", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct VerifiedaccessInstanceLoggingConfigurationAccessLogsElS3El {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_owner: Option<PrimField<String>>,
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}
impl VerifiedaccessInstanceLoggingConfigurationAccessLogsElS3El {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }
    #[doc = "Set the field `bucket_owner`.\n"]
    pub fn set_bucket_owner(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_owner = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
}
impl ToListMappable for VerifiedaccessInstanceLoggingConfigurationAccessLogsElS3El {
    type O = BlockAssignable<VerifiedaccessInstanceLoggingConfigurationAccessLogsElS3El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVerifiedaccessInstanceLoggingConfigurationAccessLogsElS3El {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}
impl BuildVerifiedaccessInstanceLoggingConfigurationAccessLogsElS3El {
    pub fn build(self) -> VerifiedaccessInstanceLoggingConfigurationAccessLogsElS3El {
        VerifiedaccessInstanceLoggingConfigurationAccessLogsElS3El {
            bucket_name: core::default::Default::default(),
            bucket_owner: core::default::Default::default(),
            enabled: self.enabled,
            prefix: core::default::Default::default(),
        }
    }
}
pub struct VerifiedaccessInstanceLoggingConfigurationAccessLogsElS3ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VerifiedaccessInstanceLoggingConfigurationAccessLogsElS3ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VerifiedaccessInstanceLoggingConfigurationAccessLogsElS3ElRef {
        VerifiedaccessInstanceLoggingConfigurationAccessLogsElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VerifiedaccessInstanceLoggingConfigurationAccessLogsElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `bucket_owner` after provisioning.\n"]
    pub fn bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_owner", self.base))
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}
#[derive(Serialize, Default)]
struct VerifiedaccessInstanceLoggingConfigurationAccessLogsElDynamic {
    cloudwatch_logs: Option<
        DynamicBlock<VerifiedaccessInstanceLoggingConfigurationAccessLogsElCloudwatchLogsEl>,
    >,
    kinesis_data_firehose: Option<
        DynamicBlock<VerifiedaccessInstanceLoggingConfigurationAccessLogsElKinesisDataFirehoseEl>,
    >,
    s3: Option<DynamicBlock<VerifiedaccessInstanceLoggingConfigurationAccessLogsElS3El>>,
}
#[derive(Serialize)]
pub struct VerifiedaccessInstanceLoggingConfigurationAccessLogsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_trust_context: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logs:
        Option<Vec<VerifiedaccessInstanceLoggingConfigurationAccessLogsElCloudwatchLogsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_data_firehose:
        Option<Vec<VerifiedaccessInstanceLoggingConfigurationAccessLogsElKinesisDataFirehoseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<VerifiedaccessInstanceLoggingConfigurationAccessLogsElS3El>>,
    dynamic: VerifiedaccessInstanceLoggingConfigurationAccessLogsElDynamic,
}
impl VerifiedaccessInstanceLoggingConfigurationAccessLogsEl {
    #[doc = "Set the field `include_trust_context`.\n"]
    pub fn set_include_trust_context(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_trust_context = Some(v.into());
        self
    }
    #[doc = "Set the field `log_version`.\n"]
    pub fn set_log_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_version = Some(v.into());
        self
    }
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v: impl Into<
            BlockAssignable<VerifiedaccessInstanceLoggingConfigurationAccessLogsElCloudwatchLogsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kinesis_data_firehose`.\n"]
    pub fn set_kinesis_data_firehose(
        mut self,
        v: impl Into<
            BlockAssignable<
                VerifiedaccessInstanceLoggingConfigurationAccessLogsElKinesisDataFirehoseEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_data_firehose = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_data_firehose = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl Into<BlockAssignable<VerifiedaccessInstanceLoggingConfigurationAccessLogsElS3El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VerifiedaccessInstanceLoggingConfigurationAccessLogsEl {
    type O = BlockAssignable<VerifiedaccessInstanceLoggingConfigurationAccessLogsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVerifiedaccessInstanceLoggingConfigurationAccessLogsEl {}
impl BuildVerifiedaccessInstanceLoggingConfigurationAccessLogsEl {
    pub fn build(self) -> VerifiedaccessInstanceLoggingConfigurationAccessLogsEl {
        VerifiedaccessInstanceLoggingConfigurationAccessLogsEl {
            include_trust_context: core::default::Default::default(),
            log_version: core::default::Default::default(),
            cloudwatch_logs: core::default::Default::default(),
            kinesis_data_firehose: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VerifiedaccessInstanceLoggingConfigurationAccessLogsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VerifiedaccessInstanceLoggingConfigurationAccessLogsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VerifiedaccessInstanceLoggingConfigurationAccessLogsElRef {
        VerifiedaccessInstanceLoggingConfigurationAccessLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VerifiedaccessInstanceLoggingConfigurationAccessLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `include_trust_context` after provisioning.\n"]
    pub fn include_trust_context(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_trust_context", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `log_version` after provisioning.\n"]
    pub fn log_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_version", self.base))
    }
    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(
        &self,
    ) -> ListRef<VerifiedaccessInstanceLoggingConfigurationAccessLogsElCloudwatchLogsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_logs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kinesis_data_firehose` after provisioning.\n"]
    pub fn kinesis_data_firehose(
        &self,
    ) -> ListRef<VerifiedaccessInstanceLoggingConfigurationAccessLogsElKinesisDataFirehoseElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_data_firehose", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<VerifiedaccessInstanceLoggingConfigurationAccessLogsElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}
#[derive(Serialize, Default)]
struct VerifiedaccessInstanceLoggingConfigurationDynamic {
    access_logs: Option<DynamicBlock<VerifiedaccessInstanceLoggingConfigurationAccessLogsEl>>,
}
