use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataImagebuilderInfrastructureConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
struct DataImagebuilderInfrastructureConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataImagebuilderInfrastructureConfigurationData>,
}
#[derive(Clone)]
pub struct DataImagebuilderInfrastructureConfiguration(
    Rc<DataImagebuilderInfrastructureConfiguration_>,
);
impl DataImagebuilderInfrastructureConfiguration {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
    #[doc = "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resource_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.date_created", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `date_updated` after provisioning.\n"]
    pub fn date_updated(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.date_updated", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_metadata_options` after provisioning.\n"]
    pub fn instance_metadata_options(
        &self,
    ) -> ListRef<DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_metadata_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_profile_name` after provisioning.\n"]
    pub fn instance_profile_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_profile_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_types` after provisioning.\n"]
    pub fn instance_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.instance_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `key_pair` after provisioning.\n"]
    pub fn key_pair(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_pair", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> ListRef<DataImagebuilderInfrastructureConfigurationLoggingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> ListRef<DataImagebuilderInfrastructureConfigurationPlacementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.placement", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\n"]
    pub fn resource_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sns_topic_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `terminate_instance_on_failure` after provisioning.\n"]
    pub fn terminate_instance_on_failure(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instance_on_failure", self.extract_ref()),
        )
    }
}
impl Referable for DataImagebuilderInfrastructureConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataImagebuilderInfrastructureConfiguration {}
impl ToListMappable for DataImagebuilderInfrastructureConfiguration {
    type O = ListRef<DataImagebuilderInfrastructureConfigurationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataImagebuilderInfrastructureConfiguration_ {
    fn extract_datasource_type(&self) -> String {
        "aws_imagebuilder_infrastructure_configuration".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataImagebuilderInfrastructureConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub arn: PrimField<String>,
}
impl BuildDataImagebuilderInfrastructureConfiguration {
    pub fn build(self, stack: &mut Stack) -> DataImagebuilderInfrastructureConfiguration {
        let out = DataImagebuilderInfrastructureConfiguration(Rc::new(
            DataImagebuilderInfrastructureConfiguration_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataImagebuilderInfrastructureConfigurationData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    arn: self.arn,
                    id: core::default::Default::default(),
                    region: core::default::Default::default(),
                    resource_tags: core::default::Default::default(),
                    tags: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataImagebuilderInfrastructureConfigurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataImagebuilderInfrastructureConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataImagebuilderInfrastructureConfigurationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.date_created", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `date_updated` after provisioning.\n"]
    pub fn date_updated(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.date_updated", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_metadata_options` after provisioning.\n"]
    pub fn instance_metadata_options(
        &self,
    ) -> ListRef<DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_metadata_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_profile_name` after provisioning.\n"]
    pub fn instance_profile_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_profile_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_types` after provisioning.\n"]
    pub fn instance_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.instance_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `key_pair` after provisioning.\n"]
    pub fn key_pair(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_pair", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> ListRef<DataImagebuilderInfrastructureConfigurationLoggingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> ListRef<DataImagebuilderInfrastructureConfigurationPlacementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.placement", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\n"]
    pub fn resource_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sns_topic_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `terminate_instance_on_failure` after provisioning.\n"]
    pub fn terminate_instance_on_failure(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instance_on_failure", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_put_response_hop_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_tokens: Option<PrimField<String>>,
}
impl DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {
    #[doc = "Set the field `http_put_response_hop_limit`.\n"]
    pub fn set_http_put_response_hop_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.http_put_response_hop_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `http_tokens`.\n"]
    pub fn set_http_tokens(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_tokens = Some(v.into());
        self
    }
}
impl ToListMappable for DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {
    type O = BlockAssignable<DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {}
impl BuildDataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {
    pub fn build(self) -> DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {
        DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {
            http_put_response_hop_limit: core::default::Default::default(),
            http_tokens: core::default::Default::default(),
        }
    }
}
pub struct DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef {
        DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_put_response_hop_limit` after provisioning.\n"]
    pub fn http_put_response_hop_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_put_response_hop_limit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `http_tokens` after provisioning.\n"]
    pub fn http_tokens(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_tokens", self.base))
    }
}
#[derive(Serialize)]
pub struct DataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_key_prefix: Option<PrimField<String>>,
}
impl DataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
    #[doc = "Set the field `s3_bucket_name`.\n"]
    pub fn set_s3_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_bucket_name = Some(v.into());
        self
    }
    #[doc = "Set the field `s3_key_prefix`.\n"]
    pub fn set_s3_key_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_key_prefix = Some(v.into());
        self
    }
}
impl ToListMappable for DataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
    type O = BlockAssignable<DataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {}
impl BuildDataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
    pub fn build(self) -> DataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
        DataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
            s3_bucket_name: core::default::Default::default(),
            s3_key_prefix: core::default::Default::default(),
        }
    }
}
pub struct DataImagebuilderInfrastructureConfigurationLoggingElS3LogsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataImagebuilderInfrastructureConfigurationLoggingElS3LogsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataImagebuilderInfrastructureConfigurationLoggingElS3LogsElRef {
        DataImagebuilderInfrastructureConfigurationLoggingElS3LogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataImagebuilderInfrastructureConfigurationLoggingElS3LogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_key_prefix", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataImagebuilderInfrastructureConfigurationLoggingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_logs: Option<ListField<DataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl>>,
}
impl DataImagebuilderInfrastructureConfigurationLoggingEl {
    #[doc = "Set the field `s3_logs`.\n"]
    pub fn set_s3_logs(
        mut self,
        v: impl Into<ListField<DataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl>>,
    ) -> Self {
        self.s3_logs = Some(v.into());
        self
    }
}
impl ToListMappable for DataImagebuilderInfrastructureConfigurationLoggingEl {
    type O = BlockAssignable<DataImagebuilderInfrastructureConfigurationLoggingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataImagebuilderInfrastructureConfigurationLoggingEl {}
impl BuildDataImagebuilderInfrastructureConfigurationLoggingEl {
    pub fn build(self) -> DataImagebuilderInfrastructureConfigurationLoggingEl {
        DataImagebuilderInfrastructureConfigurationLoggingEl {
            s3_logs: core::default::Default::default(),
        }
    }
}
pub struct DataImagebuilderInfrastructureConfigurationLoggingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataImagebuilderInfrastructureConfigurationLoggingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataImagebuilderInfrastructureConfigurationLoggingElRef {
        DataImagebuilderInfrastructureConfigurationLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataImagebuilderInfrastructureConfigurationLoggingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_logs` after provisioning.\n"]
    pub fn s3_logs(
        &self,
    ) -> ListRef<DataImagebuilderInfrastructureConfigurationLoggingElS3LogsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_logs", self.base))
    }
}
#[derive(Serialize)]
pub struct DataImagebuilderInfrastructureConfigurationPlacementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_resource_group_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tenancy: Option<PrimField<String>>,
}
impl DataImagebuilderInfrastructureConfigurationPlacementEl {
    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `host_id`.\n"]
    pub fn set_host_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_id = Some(v.into());
        self
    }
    #[doc = "Set the field `host_resource_group_arn`.\n"]
    pub fn set_host_resource_group_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_resource_group_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `tenancy`.\n"]
    pub fn set_tenancy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tenancy = Some(v.into());
        self
    }
}
impl ToListMappable for DataImagebuilderInfrastructureConfigurationPlacementEl {
    type O = BlockAssignable<DataImagebuilderInfrastructureConfigurationPlacementEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataImagebuilderInfrastructureConfigurationPlacementEl {}
impl BuildDataImagebuilderInfrastructureConfigurationPlacementEl {
    pub fn build(self) -> DataImagebuilderInfrastructureConfigurationPlacementEl {
        DataImagebuilderInfrastructureConfigurationPlacementEl {
            availability_zone: core::default::Default::default(),
            host_id: core::default::Default::default(),
            host_resource_group_arn: core::default::Default::default(),
            tenancy: core::default::Default::default(),
        }
    }
}
pub struct DataImagebuilderInfrastructureConfigurationPlacementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataImagebuilderInfrastructureConfigurationPlacementElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataImagebuilderInfrastructureConfigurationPlacementElRef {
        DataImagebuilderInfrastructureConfigurationPlacementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataImagebuilderInfrastructureConfigurationPlacementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `host_id` after provisioning.\n"]
    pub fn host_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_id", self.base))
    }
    #[doc = "Get a reference to the value of field `host_resource_group_arn` after provisioning.\n"]
    pub fn host_resource_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_resource_group_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.base))
    }
}
