use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CognitoLogDeliveryConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    user_pool_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_configurations: Option<Vec<CognitoLogDeliveryConfigurationLogConfigurationsEl>>,
    dynamic: CognitoLogDeliveryConfigurationDynamic,
}

struct CognitoLogDeliveryConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CognitoLogDeliveryConfigurationData>,
}

#[derive(Clone)]
pub struct CognitoLogDeliveryConfiguration(Rc<CognitoLogDeliveryConfiguration_>);

impl CognitoLogDeliveryConfiguration {
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

    #[doc = "Set the field `log_configurations`.\n"]
    pub fn set_log_configurations(
        self,
        v: impl Into<BlockAssignable<CognitoLogDeliveryConfigurationLogConfigurationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_configurations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_configurations = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_pool_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_configurations` after provisioning.\n"]
    pub fn log_configurations(
        &self,
    ) -> ListRef<CognitoLogDeliveryConfigurationLogConfigurationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_configurations", self.extract_ref()),
        )
    }
}

impl Referable for CognitoLogDeliveryConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for CognitoLogDeliveryConfiguration {}

impl ToListMappable for CognitoLogDeliveryConfiguration {
    type O = ListRef<CognitoLogDeliveryConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CognitoLogDeliveryConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_cognito_log_delivery_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCognitoLogDeliveryConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub user_pool_id: PrimField<String>,
}

impl BuildCognitoLogDeliveryConfiguration {
    pub fn build(self, stack: &mut Stack) -> CognitoLogDeliveryConfiguration {
        let out = CognitoLogDeliveryConfiguration(Rc::new(CognitoLogDeliveryConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CognitoLogDeliveryConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                region: core::default::Default::default(),
                user_pool_id: self.user_pool_id,
                log_configurations: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CognitoLogDeliveryConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoLogDeliveryConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CognitoLogDeliveryConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_pool_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_configurations` after provisioning.\n"]
    pub fn log_configurations(
        &self,
    ) -> ListRef<CognitoLogDeliveryConfigurationLogConfigurationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_configurations", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CognitoLogDeliveryConfigurationLogConfigurationsElCloudWatchLogsConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group_arn: Option<PrimField<String>>,
}

impl CognitoLogDeliveryConfigurationLogConfigurationsElCloudWatchLogsConfigurationEl {
    #[doc = "Set the field `log_group_arn`.\n"]
    pub fn set_log_group_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group_arn = Some(v.into());
        self
    }
}

impl ToListMappable
    for CognitoLogDeliveryConfigurationLogConfigurationsElCloudWatchLogsConfigurationEl
{
    type O = BlockAssignable<
        CognitoLogDeliveryConfigurationLogConfigurationsElCloudWatchLogsConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoLogDeliveryConfigurationLogConfigurationsElCloudWatchLogsConfigurationEl {}

impl BuildCognitoLogDeliveryConfigurationLogConfigurationsElCloudWatchLogsConfigurationEl {
    pub fn build(
        self,
    ) -> CognitoLogDeliveryConfigurationLogConfigurationsElCloudWatchLogsConfigurationEl {
        CognitoLogDeliveryConfigurationLogConfigurationsElCloudWatchLogsConfigurationEl {
            log_group_arn: core::default::Default::default(),
        }
    }
}

pub struct CognitoLogDeliveryConfigurationLogConfigurationsElCloudWatchLogsConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoLogDeliveryConfigurationLogConfigurationsElCloudWatchLogsConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CognitoLogDeliveryConfigurationLogConfigurationsElCloudWatchLogsConfigurationElRef {
        CognitoLogDeliveryConfigurationLogConfigurationsElCloudWatchLogsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoLogDeliveryConfigurationLogConfigurationsElCloudWatchLogsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_arn` after provisioning.\n"]
    pub fn log_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_group_arn", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CognitoLogDeliveryConfigurationLogConfigurationsElFirehoseConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_arn: Option<PrimField<String>>,
}

impl CognitoLogDeliveryConfigurationLogConfigurationsElFirehoseConfigurationEl {
    #[doc = "Set the field `stream_arn`.\n"]
    pub fn set_stream_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stream_arn = Some(v.into());
        self
    }
}

impl ToListMappable for CognitoLogDeliveryConfigurationLogConfigurationsElFirehoseConfigurationEl {
    type O =
        BlockAssignable<CognitoLogDeliveryConfigurationLogConfigurationsElFirehoseConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoLogDeliveryConfigurationLogConfigurationsElFirehoseConfigurationEl {}

impl BuildCognitoLogDeliveryConfigurationLogConfigurationsElFirehoseConfigurationEl {
    pub fn build(
        self,
    ) -> CognitoLogDeliveryConfigurationLogConfigurationsElFirehoseConfigurationEl {
        CognitoLogDeliveryConfigurationLogConfigurationsElFirehoseConfigurationEl {
            stream_arn: core::default::Default::default(),
        }
    }
}

pub struct CognitoLogDeliveryConfigurationLogConfigurationsElFirehoseConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoLogDeliveryConfigurationLogConfigurationsElFirehoseConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CognitoLogDeliveryConfigurationLogConfigurationsElFirehoseConfigurationElRef {
        CognitoLogDeliveryConfigurationLogConfigurationsElFirehoseConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoLogDeliveryConfigurationLogConfigurationsElFirehoseConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoLogDeliveryConfigurationLogConfigurationsElS3ConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_arn: Option<PrimField<String>>,
}

impl CognitoLogDeliveryConfigurationLogConfigurationsElS3ConfigurationEl {
    #[doc = "Set the field `bucket_arn`.\n"]
    pub fn set_bucket_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_arn = Some(v.into());
        self
    }
}

impl ToListMappable for CognitoLogDeliveryConfigurationLogConfigurationsElS3ConfigurationEl {
    type O = BlockAssignable<CognitoLogDeliveryConfigurationLogConfigurationsElS3ConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoLogDeliveryConfigurationLogConfigurationsElS3ConfigurationEl {}

impl BuildCognitoLogDeliveryConfigurationLogConfigurationsElS3ConfigurationEl {
    pub fn build(self) -> CognitoLogDeliveryConfigurationLogConfigurationsElS3ConfigurationEl {
        CognitoLogDeliveryConfigurationLogConfigurationsElS3ConfigurationEl {
            bucket_arn: core::default::Default::default(),
        }
    }
}

pub struct CognitoLogDeliveryConfigurationLogConfigurationsElS3ConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoLogDeliveryConfigurationLogConfigurationsElS3ConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CognitoLogDeliveryConfigurationLogConfigurationsElS3ConfigurationElRef {
        CognitoLogDeliveryConfigurationLogConfigurationsElS3ConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoLogDeliveryConfigurationLogConfigurationsElS3ConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoLogDeliveryConfigurationLogConfigurationsElDynamic {
    cloud_watch_logs_configuration: Option<
        DynamicBlock<
            CognitoLogDeliveryConfigurationLogConfigurationsElCloudWatchLogsConfigurationEl,
        >,
    >,
    firehose_configuration: Option<
        DynamicBlock<CognitoLogDeliveryConfigurationLogConfigurationsElFirehoseConfigurationEl>,
    >,
    s3_configuration:
        Option<DynamicBlock<CognitoLogDeliveryConfigurationLogConfigurationsElS3ConfigurationEl>>,
}

#[derive(Serialize)]
pub struct CognitoLogDeliveryConfigurationLogConfigurationsEl {
    event_source: PrimField<String>,
    log_level: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_watch_logs_configuration: Option<
        Vec<CognitoLogDeliveryConfigurationLogConfigurationsElCloudWatchLogsConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    firehose_configuration:
        Option<Vec<CognitoLogDeliveryConfigurationLogConfigurationsElFirehoseConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_configuration:
        Option<Vec<CognitoLogDeliveryConfigurationLogConfigurationsElS3ConfigurationEl>>,
    dynamic: CognitoLogDeliveryConfigurationLogConfigurationsElDynamic,
}

impl CognitoLogDeliveryConfigurationLogConfigurationsEl {
    #[doc = "Set the field `cloud_watch_logs_configuration`.\n"]
    pub fn set_cloud_watch_logs_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                CognitoLogDeliveryConfigurationLogConfigurationsElCloudWatchLogsConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_watch_logs_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_watch_logs_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `firehose_configuration`.\n"]
    pub fn set_firehose_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                CognitoLogDeliveryConfigurationLogConfigurationsElFirehoseConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.firehose_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.firehose_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<CognitoLogDeliveryConfigurationLogConfigurationsElS3ConfigurationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CognitoLogDeliveryConfigurationLogConfigurationsEl {
    type O = BlockAssignable<CognitoLogDeliveryConfigurationLogConfigurationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoLogDeliveryConfigurationLogConfigurationsEl {
    #[doc = ""]
    pub event_source: PrimField<String>,
    #[doc = ""]
    pub log_level: PrimField<String>,
}

impl BuildCognitoLogDeliveryConfigurationLogConfigurationsEl {
    pub fn build(self) -> CognitoLogDeliveryConfigurationLogConfigurationsEl {
        CognitoLogDeliveryConfigurationLogConfigurationsEl {
            event_source: self.event_source,
            log_level: self.log_level,
            cloud_watch_logs_configuration: core::default::Default::default(),
            firehose_configuration: core::default::Default::default(),
            s3_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CognitoLogDeliveryConfigurationLogConfigurationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoLogDeliveryConfigurationLogConfigurationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CognitoLogDeliveryConfigurationLogConfigurationsElRef {
        CognitoLogDeliveryConfigurationLogConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoLogDeliveryConfigurationLogConfigurationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_source` after provisioning.\n"]
    pub fn event_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_source", self.base))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_watch_logs_configuration` after provisioning.\n"]
    pub fn cloud_watch_logs_configuration(
        &self,
    ) -> ListRef<CognitoLogDeliveryConfigurationLogConfigurationsElCloudWatchLogsConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_watch_logs_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `firehose_configuration` after provisioning.\n"]
    pub fn firehose_configuration(
        &self,
    ) -> ListRef<CognitoLogDeliveryConfigurationLogConfigurationsElFirehoseConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.firehose_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> ListRef<CognitoLogDeliveryConfigurationLogConfigurationsElS3ConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_configuration", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CognitoLogDeliveryConfigurationDynamic {
    log_configurations: Option<DynamicBlock<CognitoLogDeliveryConfigurationLogConfigurationsEl>>,
}
