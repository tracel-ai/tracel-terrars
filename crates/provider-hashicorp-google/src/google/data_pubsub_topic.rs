use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataPubsubTopicData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataPubsubTopic_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataPubsubTopicData>,
}
#[derive(Clone)]
pub struct DataPubsubTopic(Rc<DataPubsubTopic_>);
impl DataPubsubTopic {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ingestion_data_source_settings` after provisioning.\nSettings for ingestion from a data source into this topic."]
    pub fn ingestion_data_source_settings(
        &self,
    ) -> ListRef<DataPubsubTopicIngestionDataSourceSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingestion_data_source_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe resource name of the Cloud KMS CryptoKey to be used to protect access\nto messages published on this topic. Your project's PubSub service account\n('service-{{PROJECT_NUMBER}}@gcp-sa-pubsub.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nThe expected format is 'projects/*/locations/*/keyRings/*/cryptoKeys/*'"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this Topic.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `message_retention_duration` after provisioning.\nIndicates the minimum duration to retain a message after it is published\nto the topic. If this field is set, messages published to the topic in\nthe last messageRetentionDuration are always available to subscribers.\nFor instance, it allows any attached subscription to seek to a timestamp\nthat is up to messageRetentionDuration in the past. If this field is not\nset, message retention is controlled by settings on individual subscriptions.\nThe rotation period has the format of a decimal number, followed by the\nletter 's' (seconds). Cannot be more than 31 days or less than 10 minutes."]
    pub fn message_retention_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_retention_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `message_storage_policy` after provisioning.\nPolicy constraining the set of Google Cloud Platform regions where\nmessages published to the topic may be stored. If not present, then no\nconstraints are in effect."]
    pub fn message_storage_policy(&self) -> ListRef<DataPubsubTopicMessageStoragePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.message_storage_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `message_transforms` after provisioning.\nTransforms to be applied to messages published to the topic. Transforms are applied in the\norder specified."]
    pub fn message_transforms(&self) -> ListRef<DataPubsubTopicMessageTransformsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.message_transforms", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the topic."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema_settings` after provisioning.\nSettings for validating messages published against a schema."]
    pub fn schema_settings(&self) -> ListRef<DataPubsubTopicSchemaSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nInput only. Resource manager tags to be bound to the topic. Tag keys and\nvalues have the same definition as resource manager tags. Keys must be in\nthe format tagKeys/{tag_key_id}, and values are in the format\ntagValues/456. The field is ignored when empty. The field is immutable and\ncauses resource replacement when mutated. This field is only set at create\ntime and modifying this field after creation will trigger recreation. To\napply tags to an existing resource, see the 'google_tags_tag_value'\nresource."]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
}
impl Referable for DataPubsubTopic {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataPubsubTopic {}
impl ToListMappable for DataPubsubTopic {
    type O = ListRef<DataPubsubTopicRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataPubsubTopic_ {
    fn extract_datasource_type(&self) -> String {
        "google_pubsub_topic".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataPubsubTopic {
    pub tf_id: String,
    #[doc = "Name of the topic."]
    pub name: PrimField<String>,
}
impl BuildDataPubsubTopic {
    pub fn build(self, stack: &mut Stack) -> DataPubsubTopic {
        let out = DataPubsubTopic(Rc::new(DataPubsubTopic_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataPubsubTopicData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataPubsubTopicRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubTopicRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataPubsubTopicRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ingestion_data_source_settings` after provisioning.\nSettings for ingestion from a data source into this topic."]
    pub fn ingestion_data_source_settings(
        &self,
    ) -> ListRef<DataPubsubTopicIngestionDataSourceSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingestion_data_source_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe resource name of the Cloud KMS CryptoKey to be used to protect access\nto messages published on this topic. Your project's PubSub service account\n('service-{{PROJECT_NUMBER}}@gcp-sa-pubsub.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nThe expected format is 'projects/*/locations/*/keyRings/*/cryptoKeys/*'"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this Topic.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `message_retention_duration` after provisioning.\nIndicates the minimum duration to retain a message after it is published\nto the topic. If this field is set, messages published to the topic in\nthe last messageRetentionDuration are always available to subscribers.\nFor instance, it allows any attached subscription to seek to a timestamp\nthat is up to messageRetentionDuration in the past. If this field is not\nset, message retention is controlled by settings on individual subscriptions.\nThe rotation period has the format of a decimal number, followed by the\nletter 's' (seconds). Cannot be more than 31 days or less than 10 minutes."]
    pub fn message_retention_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_retention_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `message_storage_policy` after provisioning.\nPolicy constraining the set of Google Cloud Platform regions where\nmessages published to the topic may be stored. If not present, then no\nconstraints are in effect."]
    pub fn message_storage_policy(&self) -> ListRef<DataPubsubTopicMessageStoragePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.message_storage_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `message_transforms` after provisioning.\nTransforms to be applied to messages published to the topic. Transforms are applied in the\norder specified."]
    pub fn message_transforms(&self) -> ListRef<DataPubsubTopicMessageTransformsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.message_transforms", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the topic."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema_settings` after provisioning.\nSettings for validating messages published against a schema."]
    pub fn schema_settings(&self) -> ListRef<DataPubsubTopicSchemaSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nInput only. Resource manager tags to be bound to the topic. Tag keys and\nvalues have the same definition as resource manager tags. Keys must be in\nthe format tagKeys/{tag_key_id}, and values are in the format\ntagValues/456. The field is ignored when empty. The field is immutable and\ncauses resource replacement when mutated. This field is only set at create\ntime and modifying this field after creation will trigger recreation. To\napply tags to an existing resource, see the 'google_tags_tag_value'\nresource."]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataPubsubTopicIngestionDataSourceSettingsElAwsKinesisEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_arn: Option<PrimField<String>>,
}
impl DataPubsubTopicIngestionDataSourceSettingsElAwsKinesisEl {
    #[doc = "Set the field `aws_role_arn`.\n"]
    pub fn set_aws_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.aws_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `consumer_arn`.\n"]
    pub fn set_consumer_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consumer_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `gcp_service_account`.\n"]
    pub fn set_gcp_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `stream_arn`.\n"]
    pub fn set_stream_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stream_arn = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubTopicIngestionDataSourceSettingsElAwsKinesisEl {
    type O = BlockAssignable<DataPubsubTopicIngestionDataSourceSettingsElAwsKinesisEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubTopicIngestionDataSourceSettingsElAwsKinesisEl {}
impl BuildDataPubsubTopicIngestionDataSourceSettingsElAwsKinesisEl {
    pub fn build(self) -> DataPubsubTopicIngestionDataSourceSettingsElAwsKinesisEl {
        DataPubsubTopicIngestionDataSourceSettingsElAwsKinesisEl {
            aws_role_arn: core::default::Default::default(),
            consumer_arn: core::default::Default::default(),
            gcp_service_account: core::default::Default::default(),
            stream_arn: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubTopicIngestionDataSourceSettingsElAwsKinesisElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubTopicIngestionDataSourceSettingsElAwsKinesisElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPubsubTopicIngestionDataSourceSettingsElAwsKinesisElRef {
        DataPubsubTopicIngestionDataSourceSettingsElAwsKinesisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubTopicIngestionDataSourceSettingsElAwsKinesisElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aws_role_arn` after provisioning.\n"]
    pub fn aws_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `consumer_arn` after provisioning.\n"]
    pub fn consumer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.consumer_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `gcp_service_account` after provisioning.\n"]
    pub fn gcp_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct DataPubsubTopicIngestionDataSourceSettingsElAwsMskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<PrimField<String>>,
}
impl DataPubsubTopicIngestionDataSourceSettingsElAwsMskEl {
    #[doc = "Set the field `aws_role_arn`.\n"]
    pub fn set_aws_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.aws_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_arn`.\n"]
    pub fn set_cluster_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `gcp_service_account`.\n"]
    pub fn set_gcp_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `topic`.\n"]
    pub fn set_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubTopicIngestionDataSourceSettingsElAwsMskEl {
    type O = BlockAssignable<DataPubsubTopicIngestionDataSourceSettingsElAwsMskEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubTopicIngestionDataSourceSettingsElAwsMskEl {}
impl BuildDataPubsubTopicIngestionDataSourceSettingsElAwsMskEl {
    pub fn build(self) -> DataPubsubTopicIngestionDataSourceSettingsElAwsMskEl {
        DataPubsubTopicIngestionDataSourceSettingsElAwsMskEl {
            aws_role_arn: core::default::Default::default(),
            cluster_arn: core::default::Default::default(),
            gcp_service_account: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubTopicIngestionDataSourceSettingsElAwsMskElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubTopicIngestionDataSourceSettingsElAwsMskElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPubsubTopicIngestionDataSourceSettingsElAwsMskElRef {
        DataPubsubTopicIngestionDataSourceSettingsElAwsMskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubTopicIngestionDataSourceSettingsElAwsMskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aws_role_arn` after provisioning.\n"]
    pub fn aws_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `gcp_service_account` after provisioning.\n"]
    pub fn gcp_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize)]
pub struct DataPubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_hub: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tenant_id: Option<PrimField<String>>,
}
impl DataPubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl {
    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }
    #[doc = "Set the field `event_hub`.\n"]
    pub fn set_event_hub(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event_hub = Some(v.into());
        self
    }
    #[doc = "Set the field `gcp_service_account`.\n"]
    pub fn set_gcp_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_group`.\n"]
    pub fn set_resource_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_group = Some(v.into());
        self
    }
    #[doc = "Set the field `subscription_id`.\n"]
    pub fn set_subscription_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subscription_id = Some(v.into());
        self
    }
    #[doc = "Set the field `tenant_id`.\n"]
    pub fn set_tenant_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tenant_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl {
    type O = BlockAssignable<DataPubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl {}
impl BuildDataPubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl {
    pub fn build(self) -> DataPubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl {
        DataPubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl {
            client_id: core::default::Default::default(),
            event_hub: core::default::Default::default(),
            gcp_service_account: core::default::Default::default(),
            namespace: core::default::Default::default(),
            resource_group: core::default::Default::default(),
            subscription_id: core::default::Default::default(),
            tenant_id: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubTopicIngestionDataSourceSettingsElAzureEventHubsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubTopicIngestionDataSourceSettingsElAzureEventHubsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPubsubTopicIngestionDataSourceSettingsElAzureEventHubsElRef {
        DataPubsubTopicIngestionDataSourceSettingsElAzureEventHubsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubTopicIngestionDataSourceSettingsElAzureEventHubsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `event_hub` after provisioning.\n"]
    pub fn event_hub(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_hub", self.base))
    }
    #[doc = "Get a reference to the value of field `gcp_service_account` after provisioning.\n"]
    pub fn gcp_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_group` after provisioning.\n"]
    pub fn resource_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_group", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subscription_id` after provisioning.\n"]
    pub fn subscription_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\n"]
    pub fn tenant_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl {}
impl DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl {}
impl ToListMappable for DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl {
    type O =
        BlockAssignable<DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl {}
impl BuildDataPubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl {
    pub fn build(self) -> DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl {
        DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl {}
    }
}
pub struct DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatElRef {
        DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl {}
impl DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl {}
impl ToListMappable
    for DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl
{
    type O = BlockAssignable<
        DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl {}
impl BuildDataPubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl {
    pub fn build(
        self,
    ) -> DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl {
        DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl {}
    }
}
pub struct DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatElRef {
        DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delimiter: Option<PrimField<String>>,
}
impl DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl {
    #[doc = "Set the field `delimiter`.\n"]
    pub fn set_delimiter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delimiter = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl {
    type O =
        BlockAssignable<DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl {}
impl BuildDataPubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl {
    pub fn build(self) -> DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl {
        DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl {
            delimiter: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatElRef {
        DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delimiter` after provisioning.\n"]
    pub fn delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }
}
#[derive(Serialize)]
pub struct DataPubsubTopicIngestionDataSourceSettingsElCloudStorageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    avro_format:
        Option<ListField<DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_glob: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_object_create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pubsub_avro_format: Option<
        ListField<DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_format:
        Option<ListField<DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl>>,
}
impl DataPubsubTopicIngestionDataSourceSettingsElCloudStorageEl {
    #[doc = "Set the field `avro_format`.\n"]
    pub fn set_avro_format(
        mut self,
        v: impl Into<ListField<DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl>>,
    ) -> Self {
        self.avro_format = Some(v.into());
        self
    }
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `match_glob`.\n"]
    pub fn set_match_glob(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.match_glob = Some(v.into());
        self
    }
    #[doc = "Set the field `minimum_object_create_time`.\n"]
    pub fn set_minimum_object_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.minimum_object_create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `pubsub_avro_format`.\n"]
    pub fn set_pubsub_avro_format(
        mut self,
        v: impl Into<
            ListField<DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl>,
        >,
    ) -> Self {
        self.pubsub_avro_format = Some(v.into());
        self
    }
    #[doc = "Set the field `text_format`.\n"]
    pub fn set_text_format(
        mut self,
        v: impl Into<ListField<DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl>>,
    ) -> Self {
        self.text_format = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubTopicIngestionDataSourceSettingsElCloudStorageEl {
    type O = BlockAssignable<DataPubsubTopicIngestionDataSourceSettingsElCloudStorageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubTopicIngestionDataSourceSettingsElCloudStorageEl {}
impl BuildDataPubsubTopicIngestionDataSourceSettingsElCloudStorageEl {
    pub fn build(self) -> DataPubsubTopicIngestionDataSourceSettingsElCloudStorageEl {
        DataPubsubTopicIngestionDataSourceSettingsElCloudStorageEl {
            avro_format: core::default::Default::default(),
            bucket: core::default::Default::default(),
            match_glob: core::default::Default::default(),
            minimum_object_create_time: core::default::Default::default(),
            pubsub_avro_format: core::default::Default::default(),
            text_format: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElRef {
        DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `avro_format` after provisioning.\n"]
    pub fn avro_format(
        &self,
    ) -> ListRef<DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatElRef> {
        ListRef::new(self.shared().clone(), format!("{}.avro_format", self.base))
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `match_glob` after provisioning.\n"]
    pub fn match_glob(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.match_glob", self.base))
    }
    #[doc = "Get a reference to the value of field `minimum_object_create_time` after provisioning.\n"]
    pub fn minimum_object_create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimum_object_create_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pubsub_avro_format` after provisioning.\n"]
    pub fn pubsub_avro_format(
        &self,
    ) -> ListRef<DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pubsub_avro_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text_format` after provisioning.\n"]
    pub fn text_format(
        &self,
    ) -> ListRef<DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatElRef> {
        ListRef::new(self.shared().clone(), format!("{}.text_format", self.base))
    }
}
#[derive(Serialize)]
pub struct DataPubsubTopicIngestionDataSourceSettingsElConfluentCloudEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bootstrap_server: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_pool_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<PrimField<String>>,
}
impl DataPubsubTopicIngestionDataSourceSettingsElConfluentCloudEl {
    #[doc = "Set the field `bootstrap_server`.\n"]
    pub fn set_bootstrap_server(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bootstrap_server = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_id`.\n"]
    pub fn set_cluster_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_id = Some(v.into());
        self
    }
    #[doc = "Set the field `gcp_service_account`.\n"]
    pub fn set_gcp_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_pool_id`.\n"]
    pub fn set_identity_pool_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_pool_id = Some(v.into());
        self
    }
    #[doc = "Set the field `topic`.\n"]
    pub fn set_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubTopicIngestionDataSourceSettingsElConfluentCloudEl {
    type O = BlockAssignable<DataPubsubTopicIngestionDataSourceSettingsElConfluentCloudEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubTopicIngestionDataSourceSettingsElConfluentCloudEl {}
impl BuildDataPubsubTopicIngestionDataSourceSettingsElConfluentCloudEl {
    pub fn build(self) -> DataPubsubTopicIngestionDataSourceSettingsElConfluentCloudEl {
        DataPubsubTopicIngestionDataSourceSettingsElConfluentCloudEl {
            bootstrap_server: core::default::Default::default(),
            cluster_id: core::default::Default::default(),
            gcp_service_account: core::default::Default::default(),
            identity_pool_id: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubTopicIngestionDataSourceSettingsElConfluentCloudElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubTopicIngestionDataSourceSettingsElConfluentCloudElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPubsubTopicIngestionDataSourceSettingsElConfluentCloudElRef {
        DataPubsubTopicIngestionDataSourceSettingsElConfluentCloudElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubTopicIngestionDataSourceSettingsElConfluentCloudElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bootstrap_server` after provisioning.\n"]
    pub fn bootstrap_server(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_server", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }
    #[doc = "Get a reference to the value of field `gcp_service_account` after provisioning.\n"]
    pub fn gcp_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `identity_pool_id` after provisioning.\n"]
    pub fn identity_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_pool_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize)]
pub struct DataPubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<PrimField<String>>,
}
impl DataPubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl {
    #[doc = "Set the field `severity`.\n"]
    pub fn set_severity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.severity = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl {
    type O = BlockAssignable<DataPubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl {}
impl BuildDataPubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl {
    pub fn build(self) -> DataPubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl {
        DataPubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl {
            severity: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsElRef {
        DataPubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `severity` after provisioning.\n"]
    pub fn severity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }
}
#[derive(Serialize)]
pub struct DataPubsubTopicIngestionDataSourceSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_kinesis: Option<ListField<DataPubsubTopicIngestionDataSourceSettingsElAwsKinesisEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_msk: Option<ListField<DataPubsubTopicIngestionDataSourceSettingsElAwsMskEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    azure_event_hubs:
        Option<ListField<DataPubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_storage: Option<ListField<DataPubsubTopicIngestionDataSourceSettingsElCloudStorageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confluent_cloud:
        Option<ListField<DataPubsubTopicIngestionDataSourceSettingsElConfluentCloudEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_logs_settings:
        Option<ListField<DataPubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl>>,
}
impl DataPubsubTopicIngestionDataSourceSettingsEl {
    #[doc = "Set the field `aws_kinesis`.\n"]
    pub fn set_aws_kinesis(
        mut self,
        v: impl Into<ListField<DataPubsubTopicIngestionDataSourceSettingsElAwsKinesisEl>>,
    ) -> Self {
        self.aws_kinesis = Some(v.into());
        self
    }
    #[doc = "Set the field `aws_msk`.\n"]
    pub fn set_aws_msk(
        mut self,
        v: impl Into<ListField<DataPubsubTopicIngestionDataSourceSettingsElAwsMskEl>>,
    ) -> Self {
        self.aws_msk = Some(v.into());
        self
    }
    #[doc = "Set the field `azure_event_hubs`.\n"]
    pub fn set_azure_event_hubs(
        mut self,
        v: impl Into<ListField<DataPubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl>>,
    ) -> Self {
        self.azure_event_hubs = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_storage`.\n"]
    pub fn set_cloud_storage(
        mut self,
        v: impl Into<ListField<DataPubsubTopicIngestionDataSourceSettingsElCloudStorageEl>>,
    ) -> Self {
        self.cloud_storage = Some(v.into());
        self
    }
    #[doc = "Set the field `confluent_cloud`.\n"]
    pub fn set_confluent_cloud(
        mut self,
        v: impl Into<ListField<DataPubsubTopicIngestionDataSourceSettingsElConfluentCloudEl>>,
    ) -> Self {
        self.confluent_cloud = Some(v.into());
        self
    }
    #[doc = "Set the field `platform_logs_settings`.\n"]
    pub fn set_platform_logs_settings(
        mut self,
        v: impl Into<ListField<DataPubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl>>,
    ) -> Self {
        self.platform_logs_settings = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubTopicIngestionDataSourceSettingsEl {
    type O = BlockAssignable<DataPubsubTopicIngestionDataSourceSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubTopicIngestionDataSourceSettingsEl {}
impl BuildDataPubsubTopicIngestionDataSourceSettingsEl {
    pub fn build(self) -> DataPubsubTopicIngestionDataSourceSettingsEl {
        DataPubsubTopicIngestionDataSourceSettingsEl {
            aws_kinesis: core::default::Default::default(),
            aws_msk: core::default::Default::default(),
            azure_event_hubs: core::default::Default::default(),
            cloud_storage: core::default::Default::default(),
            confluent_cloud: core::default::Default::default(),
            platform_logs_settings: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubTopicIngestionDataSourceSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubTopicIngestionDataSourceSettingsElRef {
    fn new(shared: StackShared, base: String) -> DataPubsubTopicIngestionDataSourceSettingsElRef {
        DataPubsubTopicIngestionDataSourceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubTopicIngestionDataSourceSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aws_kinesis` after provisioning.\n"]
    pub fn aws_kinesis(
        &self,
    ) -> ListRef<DataPubsubTopicIngestionDataSourceSettingsElAwsKinesisElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aws_kinesis", self.base))
    }
    #[doc = "Get a reference to the value of field `aws_msk` after provisioning.\n"]
    pub fn aws_msk(&self) -> ListRef<DataPubsubTopicIngestionDataSourceSettingsElAwsMskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aws_msk", self.base))
    }
    #[doc = "Get a reference to the value of field `azure_event_hubs` after provisioning.\n"]
    pub fn azure_event_hubs(
        &self,
    ) -> ListRef<DataPubsubTopicIngestionDataSourceSettingsElAzureEventHubsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.azure_event_hubs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_storage` after provisioning.\n"]
    pub fn cloud_storage(
        &self,
    ) -> ListRef<DataPubsubTopicIngestionDataSourceSettingsElCloudStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_storage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `confluent_cloud` after provisioning.\n"]
    pub fn confluent_cloud(
        &self,
    ) -> ListRef<DataPubsubTopicIngestionDataSourceSettingsElConfluentCloudElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.confluent_cloud", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `platform_logs_settings` after provisioning.\n"]
    pub fn platform_logs_settings(
        &self,
    ) -> ListRef<DataPubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.platform_logs_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataPubsubTopicMessageStoragePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_persistence_regions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_in_transit: Option<PrimField<bool>>,
}
impl DataPubsubTopicMessageStoragePolicyEl {
    #[doc = "Set the field `allowed_persistence_regions`.\n"]
    pub fn set_allowed_persistence_regions(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.allowed_persistence_regions = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce_in_transit`.\n"]
    pub fn set_enforce_in_transit(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enforce_in_transit = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubTopicMessageStoragePolicyEl {
    type O = BlockAssignable<DataPubsubTopicMessageStoragePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubTopicMessageStoragePolicyEl {}
impl BuildDataPubsubTopicMessageStoragePolicyEl {
    pub fn build(self) -> DataPubsubTopicMessageStoragePolicyEl {
        DataPubsubTopicMessageStoragePolicyEl {
            allowed_persistence_regions: core::default::Default::default(),
            enforce_in_transit: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubTopicMessageStoragePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubTopicMessageStoragePolicyElRef {
    fn new(shared: StackShared, base: String) -> DataPubsubTopicMessageStoragePolicyElRef {
        DataPubsubTopicMessageStoragePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubTopicMessageStoragePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_persistence_regions` after provisioning.\n"]
    pub fn allowed_persistence_regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.allowed_persistence_regions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_in_transit` after provisioning.\n"]
    pub fn enforce_in_transit(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_in_transit", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataPubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
}
impl DataPubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.parameters = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl {
    type O =
        BlockAssignable<DataPubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl {}
impl BuildDataPubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl {
    pub fn build(self) -> DataPubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl {
        DataPubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl {
            parameters: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceElRef {
        DataPubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}
#[derive(Serialize)]
pub struct DataPubsubTopicMessageTransformsElAiInferenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unstructured_inference:
        Option<ListField<DataPubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl>>,
}
impl DataPubsubTopicMessageTransformsElAiInferenceEl {
    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_email`.\n"]
    pub fn set_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `unstructured_inference`.\n"]
    pub fn set_unstructured_inference(
        mut self,
        v: impl Into<ListField<DataPubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl>>,
    ) -> Self {
        self.unstructured_inference = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubTopicMessageTransformsElAiInferenceEl {
    type O = BlockAssignable<DataPubsubTopicMessageTransformsElAiInferenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubTopicMessageTransformsElAiInferenceEl {}
impl BuildDataPubsubTopicMessageTransformsElAiInferenceEl {
    pub fn build(self) -> DataPubsubTopicMessageTransformsElAiInferenceEl {
        DataPubsubTopicMessageTransformsElAiInferenceEl {
            endpoint: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            unstructured_inference: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubTopicMessageTransformsElAiInferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubTopicMessageTransformsElAiInferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPubsubTopicMessageTransformsElAiInferenceElRef {
        DataPubsubTopicMessageTransformsElAiInferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubTopicMessageTransformsElAiInferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\n"]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `unstructured_inference` after provisioning.\n"]
    pub fn unstructured_inference(
        &self,
    ) -> ListRef<DataPubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.unstructured_inference", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataPubsubTopicMessageTransformsElJavascriptUdfEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_name: Option<PrimField<String>>,
}
impl DataPubsubTopicMessageTransformsElJavascriptUdfEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `function_name`.\n"]
    pub fn set_function_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.function_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubTopicMessageTransformsElJavascriptUdfEl {
    type O = BlockAssignable<DataPubsubTopicMessageTransformsElJavascriptUdfEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubTopicMessageTransformsElJavascriptUdfEl {}
impl BuildDataPubsubTopicMessageTransformsElJavascriptUdfEl {
    pub fn build(self) -> DataPubsubTopicMessageTransformsElJavascriptUdfEl {
        DataPubsubTopicMessageTransformsElJavascriptUdfEl {
            code: core::default::Default::default(),
            function_name: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubTopicMessageTransformsElJavascriptUdfElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubTopicMessageTransformsElJavascriptUdfElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPubsubTopicMessageTransformsElJavascriptUdfElRef {
        DataPubsubTopicMessageTransformsElJavascriptUdfElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubTopicMessageTransformsElJavascriptUdfElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.function_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataPubsubTopicMessageTransformsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ai_inference: Option<ListField<DataPubsubTopicMessageTransformsElAiInferenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    javascript_udf: Option<ListField<DataPubsubTopicMessageTransformsElJavascriptUdfEl>>,
}
impl DataPubsubTopicMessageTransformsEl {
    #[doc = "Set the field `ai_inference`.\n"]
    pub fn set_ai_inference(
        mut self,
        v: impl Into<ListField<DataPubsubTopicMessageTransformsElAiInferenceEl>>,
    ) -> Self {
        self.ai_inference = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `javascript_udf`.\n"]
    pub fn set_javascript_udf(
        mut self,
        v: impl Into<ListField<DataPubsubTopicMessageTransformsElJavascriptUdfEl>>,
    ) -> Self {
        self.javascript_udf = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubTopicMessageTransformsEl {
    type O = BlockAssignable<DataPubsubTopicMessageTransformsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubTopicMessageTransformsEl {}
impl BuildDataPubsubTopicMessageTransformsEl {
    pub fn build(self) -> DataPubsubTopicMessageTransformsEl {
        DataPubsubTopicMessageTransformsEl {
            ai_inference: core::default::Default::default(),
            disabled: core::default::Default::default(),
            javascript_udf: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubTopicMessageTransformsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubTopicMessageTransformsElRef {
    fn new(shared: StackShared, base: String) -> DataPubsubTopicMessageTransformsElRef {
        DataPubsubTopicMessageTransformsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubTopicMessageTransformsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ai_inference` after provisioning.\n"]
    pub fn ai_inference(&self) -> ListRef<DataPubsubTopicMessageTransformsElAiInferenceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ai_inference", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `javascript_udf` after provisioning.\n"]
    pub fn javascript_udf(&self) -> ListRef<DataPubsubTopicMessageTransformsElJavascriptUdfElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.javascript_udf", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataPubsubTopicSchemaSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<PrimField<String>>,
}
impl DataPubsubTopicSchemaSettingsEl {
    #[doc = "Set the field `encoding`.\n"]
    pub fn set_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding = Some(v.into());
        self
    }
    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubTopicSchemaSettingsEl {
    type O = BlockAssignable<DataPubsubTopicSchemaSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubTopicSchemaSettingsEl {}
impl BuildDataPubsubTopicSchemaSettingsEl {
    pub fn build(self) -> DataPubsubTopicSchemaSettingsEl {
        DataPubsubTopicSchemaSettingsEl {
            encoding: core::default::Default::default(),
            schema: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubTopicSchemaSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubTopicSchemaSettingsElRef {
    fn new(shared: StackShared, base: String) -> DataPubsubTopicSchemaSettingsElRef {
        DataPubsubTopicSchemaSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubTopicSchemaSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encoding` after provisioning.\n"]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}
