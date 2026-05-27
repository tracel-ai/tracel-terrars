use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct PubsubTopicData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_retention_duration: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingestion_data_source_settings: Option<Vec<PubsubTopicIngestionDataSourceSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_storage_policy: Option<Vec<PubsubTopicMessageStoragePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_transforms: Option<Vec<PubsubTopicMessageTransformsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_settings: Option<Vec<PubsubTopicSchemaSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<PubsubTopicTimeoutsEl>,
    dynamic: PubsubTopicDynamic,
}
struct PubsubTopic_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PubsubTopicData>,
}
#[derive(Clone)]
pub struct PubsubTopic(Rc<PubsubTopic_>);
impl PubsubTopic {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_name`.\nThe resource name of the Cloud KMS CryptoKey to be used to protect access\nto messages published on this topic. Your project's PubSub service account\n('service-{{PROJECT_NUMBER}}@gcp-sa-pubsub.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nThe expected format is 'projects/*/locations/*/keyRings/*/cryptoKeys/*'"]
    pub fn set_kms_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nA set of key/value label pairs to assign to this Topic.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `message_retention_duration`.\nIndicates the minimum duration to retain a message after it is published\nto the topic. If this field is set, messages published to the topic in\nthe last messageRetentionDuration are always available to subscribers.\nFor instance, it allows any attached subscription to seek to a timestamp\nthat is up to messageRetentionDuration in the past. If this field is not\nset, message retention is controlled by settings on individual subscriptions.\nThe rotation period has the format of a decimal number, followed by the\nletter 's' (seconds). Cannot be more than 31 days or less than 10 minutes."]
    pub fn set_message_retention_duration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().message_retention_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nInput only. Resource manager tags to be bound to the topic. Tag keys and\nvalues have the same definition as resource manager tags. Keys must be in\nthe format tagKeys/{tag_key_id}, and values are in the format\ntagValues/456. The field is ignored when empty. The field is immutable and\ncauses resource replacement when mutated. This field is only set at create\ntime and modifying this field after creation will trigger recreation. To\napply tags to an existing resource, see the 'google_tags_tag_value'\nresource."]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `ingestion_data_source_settings`.\n"]
    pub fn set_ingestion_data_source_settings(
        self,
        v: impl Into<BlockAssignable<PubsubTopicIngestionDataSourceSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ingestion_data_source_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .ingestion_data_source_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `message_storage_policy`.\n"]
    pub fn set_message_storage_policy(
        self,
        v: impl Into<BlockAssignable<PubsubTopicMessageStoragePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().message_storage_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.message_storage_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `message_transforms`.\n"]
    pub fn set_message_transforms(
        self,
        v: impl Into<BlockAssignable<PubsubTopicMessageTransformsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().message_transforms = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.message_transforms = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `schema_settings`.\n"]
    pub fn set_schema_settings(
        self,
        v: impl Into<BlockAssignable<PubsubTopicSchemaSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schema_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schema_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<PubsubTopicTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
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
    #[doc = "Get a reference to the value of field `ingestion_data_source_settings` after provisioning.\n"]
    pub fn ingestion_data_source_settings(
        &self,
    ) -> ListRef<PubsubTopicIngestionDataSourceSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingestion_data_source_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `message_storage_policy` after provisioning.\n"]
    pub fn message_storage_policy(&self) -> ListRef<PubsubTopicMessageStoragePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.message_storage_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `message_transforms` after provisioning.\n"]
    pub fn message_transforms(&self) -> ListRef<PubsubTopicMessageTransformsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.message_transforms", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema_settings` after provisioning.\n"]
    pub fn schema_settings(&self) -> ListRef<PubsubTopicSchemaSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PubsubTopicTimeoutsElRef {
        PubsubTopicTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for PubsubTopic {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for PubsubTopic {}
impl ToListMappable for PubsubTopic {
    type O = ListRef<PubsubTopicRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for PubsubTopic_ {
    fn extract_resource_type(&self) -> String {
        "google_pubsub_topic".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildPubsubTopic {
    pub tf_id: String,
    #[doc = "Name of the topic."]
    pub name: PrimField<String>,
}
impl BuildPubsubTopic {
    pub fn build(self, stack: &mut Stack) -> PubsubTopic {
        let out = PubsubTopic(Rc::new(PubsubTopic_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PubsubTopicData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                labels: core::default::Default::default(),
                message_retention_duration: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                tags: core::default::Default::default(),
                ingestion_data_source_settings: core::default::Default::default(),
                message_storage_policy: core::default::Default::default(),
                message_transforms: core::default::Default::default(),
                schema_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct PubsubTopicRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubTopicRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl PubsubTopicRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `ingestion_data_source_settings` after provisioning.\n"]
    pub fn ingestion_data_source_settings(
        &self,
    ) -> ListRef<PubsubTopicIngestionDataSourceSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingestion_data_source_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `message_storage_policy` after provisioning.\n"]
    pub fn message_storage_policy(&self) -> ListRef<PubsubTopicMessageStoragePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.message_storage_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `message_transforms` after provisioning.\n"]
    pub fn message_transforms(&self) -> ListRef<PubsubTopicMessageTransformsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.message_transforms", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema_settings` after provisioning.\n"]
    pub fn schema_settings(&self) -> ListRef<PubsubTopicSchemaSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PubsubTopicTimeoutsElRef {
        PubsubTopicTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct PubsubTopicIngestionDataSourceSettingsElAwsKinesisEl {
    aws_role_arn: PrimField<String>,
    consumer_arn: PrimField<String>,
    gcp_service_account: PrimField<String>,
    stream_arn: PrimField<String>,
}
impl PubsubTopicIngestionDataSourceSettingsElAwsKinesisEl {}
impl ToListMappable for PubsubTopicIngestionDataSourceSettingsElAwsKinesisEl {
    type O = BlockAssignable<PubsubTopicIngestionDataSourceSettingsElAwsKinesisEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubTopicIngestionDataSourceSettingsElAwsKinesisEl {
    #[doc = "AWS role ARN to be used for Federated Identity authentication with\nKinesis. Check the Pub/Sub docs for how to set up this role and the\nrequired permissions that need to be attached to it."]
    pub aws_role_arn: PrimField<String>,
    #[doc = "The Kinesis consumer ARN to used for ingestion in\nEnhanced Fan-Out mode. The consumer must be already\ncreated and ready to be used."]
    pub consumer_arn: PrimField<String>,
    #[doc = "The GCP service account to be used for Federated Identity authentication\nwith Kinesis (via a 'AssumeRoleWithWebIdentity' call for the provided\nrole). The 'awsRoleArn' must be set up with 'accounts.google.com:sub'\nequals to this service account number."]
    pub gcp_service_account: PrimField<String>,
    #[doc = "The Kinesis stream ARN to ingest data from."]
    pub stream_arn: PrimField<String>,
}
impl BuildPubsubTopicIngestionDataSourceSettingsElAwsKinesisEl {
    pub fn build(self) -> PubsubTopicIngestionDataSourceSettingsElAwsKinesisEl {
        PubsubTopicIngestionDataSourceSettingsElAwsKinesisEl {
            aws_role_arn: self.aws_role_arn,
            consumer_arn: self.consumer_arn,
            gcp_service_account: self.gcp_service_account,
            stream_arn: self.stream_arn,
        }
    }
}
pub struct PubsubTopicIngestionDataSourceSettingsElAwsKinesisElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubTopicIngestionDataSourceSettingsElAwsKinesisElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PubsubTopicIngestionDataSourceSettingsElAwsKinesisElRef {
        PubsubTopicIngestionDataSourceSettingsElAwsKinesisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubTopicIngestionDataSourceSettingsElAwsKinesisElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aws_role_arn` after provisioning.\nAWS role ARN to be used for Federated Identity authentication with\nKinesis. Check the Pub/Sub docs for how to set up this role and the\nrequired permissions that need to be attached to it."]
    pub fn aws_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `consumer_arn` after provisioning.\nThe Kinesis consumer ARN to used for ingestion in\nEnhanced Fan-Out mode. The consumer must be already\ncreated and ready to be used."]
    pub fn consumer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.consumer_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `gcp_service_account` after provisioning.\nThe GCP service account to be used for Federated Identity authentication\nwith Kinesis (via a 'AssumeRoleWithWebIdentity' call for the provided\nrole). The 'awsRoleArn' must be set up with 'accounts.google.com:sub'\nequals to this service account number."]
    pub fn gcp_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\nThe Kinesis stream ARN to ingest data from."]
    pub fn stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct PubsubTopicIngestionDataSourceSettingsElAwsMskEl {
    aws_role_arn: PrimField<String>,
    cluster_arn: PrimField<String>,
    gcp_service_account: PrimField<String>,
    topic: PrimField<String>,
}
impl PubsubTopicIngestionDataSourceSettingsElAwsMskEl {}
impl ToListMappable for PubsubTopicIngestionDataSourceSettingsElAwsMskEl {
    type O = BlockAssignable<PubsubTopicIngestionDataSourceSettingsElAwsMskEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubTopicIngestionDataSourceSettingsElAwsMskEl {
    #[doc = "AWS role ARN to be used for Federated Identity authentication with\nMSK. Check the Pub/Sub docs for how to set up this role and the\nrequired permissions that need to be attached to it."]
    pub aws_role_arn: PrimField<String>,
    #[doc = "ARN that uniquely identifies the MSK cluster."]
    pub cluster_arn: PrimField<String>,
    #[doc = "The GCP service account to be used for Federated Identity authentication\nwith MSK (via a 'AssumeRoleWithWebIdentity' call for the provided\nrole). The 'awsRoleArn' must be set up with 'accounts.google.com:sub'\nequals to this service account number."]
    pub gcp_service_account: PrimField<String>,
    #[doc = "The name of the MSK topic that Pub/Sub will import from."]
    pub topic: PrimField<String>,
}
impl BuildPubsubTopicIngestionDataSourceSettingsElAwsMskEl {
    pub fn build(self) -> PubsubTopicIngestionDataSourceSettingsElAwsMskEl {
        PubsubTopicIngestionDataSourceSettingsElAwsMskEl {
            aws_role_arn: self.aws_role_arn,
            cluster_arn: self.cluster_arn,
            gcp_service_account: self.gcp_service_account,
            topic: self.topic,
        }
    }
}
pub struct PubsubTopicIngestionDataSourceSettingsElAwsMskElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubTopicIngestionDataSourceSettingsElAwsMskElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PubsubTopicIngestionDataSourceSettingsElAwsMskElRef {
        PubsubTopicIngestionDataSourceSettingsElAwsMskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubTopicIngestionDataSourceSettingsElAwsMskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aws_role_arn` after provisioning.\nAWS role ARN to be used for Federated Identity authentication with\nMSK. Check the Pub/Sub docs for how to set up this role and the\nrequired permissions that need to be attached to it."]
    pub fn aws_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\nARN that uniquely identifies the MSK cluster."]
    pub fn cluster_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `gcp_service_account` after provisioning.\nThe GCP service account to be used for Federated Identity authentication\nwith MSK (via a 'AssumeRoleWithWebIdentity' call for the provided\nrole). The 'awsRoleArn' must be set up with 'accounts.google.com:sub'\nequals to this service account number."]
    pub fn gcp_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nThe name of the MSK topic that Pub/Sub will import from."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize)]
pub struct PubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl {
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
impl PubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl {
    #[doc = "Set the field `client_id`.\nThe Azure event hub client ID to use for ingestion."]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }
    #[doc = "Set the field `event_hub`.\nThe Azure event hub to ingest data from."]
    pub fn set_event_hub(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event_hub = Some(v.into());
        self
    }
    #[doc = "Set the field `gcp_service_account`.\nThe GCP service account to be used for Federated Identity authentication\nwith Azure (via a 'AssumeRoleWithWebIdentity' call for the provided\nrole)."]
    pub fn set_gcp_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `namespace`.\nThe Azure event hub namespace to ingest data from."]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_group`.\nThe name of the resource group within an Azure subscription."]
    pub fn set_resource_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_group = Some(v.into());
        self
    }
    #[doc = "Set the field `subscription_id`.\nThe Azure event hub subscription ID to use for ingestion."]
    pub fn set_subscription_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subscription_id = Some(v.into());
        self
    }
    #[doc = "Set the field `tenant_id`.\nThe Azure event hub tenant ID to use for ingestion."]
    pub fn set_tenant_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tenant_id = Some(v.into());
        self
    }
}
impl ToListMappable for PubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl {
    type O = BlockAssignable<PubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl {}
impl BuildPubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl {
    pub fn build(self) -> PubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl {
        PubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl {
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
pub struct PubsubTopicIngestionDataSourceSettingsElAzureEventHubsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubTopicIngestionDataSourceSettingsElAzureEventHubsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PubsubTopicIngestionDataSourceSettingsElAzureEventHubsElRef {
        PubsubTopicIngestionDataSourceSettingsElAzureEventHubsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubTopicIngestionDataSourceSettingsElAzureEventHubsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe Azure event hub client ID to use for ingestion."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `event_hub` after provisioning.\nThe Azure event hub to ingest data from."]
    pub fn event_hub(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_hub", self.base))
    }
    #[doc = "Get a reference to the value of field `gcp_service_account` after provisioning.\nThe GCP service account to be used for Federated Identity authentication\nwith Azure (via a 'AssumeRoleWithWebIdentity' call for the provided\nrole)."]
    pub fn gcp_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nThe Azure event hub namespace to ingest data from."]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_group` after provisioning.\nThe name of the resource group within an Azure subscription."]
    pub fn resource_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_group", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subscription_id` after provisioning.\nThe Azure event hub subscription ID to use for ingestion."]
    pub fn subscription_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\nThe Azure event hub tenant ID to use for ingestion."]
    pub fn tenant_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}
#[derive(Serialize)]
pub struct PubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl {}
impl PubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl {}
impl ToListMappable for PubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl {
    type O = BlockAssignable<PubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl {}
impl BuildPubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl {
    pub fn build(self) -> PubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl {
        PubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl {}
    }
}
pub struct PubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatElRef {
        PubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct PubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl {}
impl PubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl {}
impl ToListMappable for PubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl {
    type O =
        BlockAssignable<PubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl {}
impl BuildPubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl {
    pub fn build(self) -> PubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl {
        PubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl {}
    }
}
pub struct PubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatElRef {
        PubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct PubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delimiter: Option<PrimField<String>>,
}
impl PubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl {
    #[doc = "Set the field `delimiter`.\nThe delimiter to use when using the 'text' format. Each line of text as\nspecified by the delimiter will be set to the 'data' field of a Pub/Sub\nmessage. When unset, '\\n' is used."]
    pub fn set_delimiter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delimiter = Some(v.into());
        self
    }
}
impl ToListMappable for PubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl {
    type O = BlockAssignable<PubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl {}
impl BuildPubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl {
    pub fn build(self) -> PubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl {
        PubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl {
            delimiter: core::default::Default::default(),
        }
    }
}
pub struct PubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatElRef {
        PubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delimiter` after provisioning.\nThe delimiter to use when using the 'text' format. Each line of text as\nspecified by the delimiter will be set to the 'data' field of a Pub/Sub\nmessage. When unset, '\\n' is used."]
    pub fn delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }
}
#[derive(Serialize, Default)]
struct PubsubTopicIngestionDataSourceSettingsElCloudStorageElDynamic {
    avro_format:
        Option<DynamicBlock<PubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl>>,
    pubsub_avro_format: Option<
        DynamicBlock<PubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl>,
    >,
    text_format:
        Option<DynamicBlock<PubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl>>,
}
#[derive(Serialize)]
pub struct PubsubTopicIngestionDataSourceSettingsElCloudStorageEl {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_glob: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_object_create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avro_format: Option<Vec<PubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pubsub_avro_format:
        Option<Vec<PubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_format: Option<Vec<PubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl>>,
    dynamic: PubsubTopicIngestionDataSourceSettingsElCloudStorageElDynamic,
}
impl PubsubTopicIngestionDataSourceSettingsElCloudStorageEl {
    #[doc = "Set the field `match_glob`.\nGlob pattern used to match objects that will be ingested. If unset, all\nobjects will be ingested. See the supported patterns:\nhttps://cloud.google.com/storage/docs/json_api/v1/objects/list#list-objects-and-prefixes-using-glob"]
    pub fn set_match_glob(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.match_glob = Some(v.into());
        self
    }
    #[doc = "Set the field `minimum_object_create_time`.\nThe timestamp set in RFC3339 text format. If set, only objects with a\nlarger or equal timestamp will be ingested. Unset by default, meaning\nall objects will be ingested."]
    pub fn set_minimum_object_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.minimum_object_create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `avro_format`.\n"]
    pub fn set_avro_format(
        mut self,
        v: impl Into<
            BlockAssignable<PubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.avro_format = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.avro_format = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `pubsub_avro_format`.\n"]
    pub fn set_pubsub_avro_format(
        mut self,
        v: impl Into<
            BlockAssignable<
                PubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pubsub_avro_format = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pubsub_avro_format = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `text_format`.\n"]
    pub fn set_text_format(
        mut self,
        v: impl Into<
            BlockAssignable<PubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.text_format = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.text_format = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PubsubTopicIngestionDataSourceSettingsElCloudStorageEl {
    type O = BlockAssignable<PubsubTopicIngestionDataSourceSettingsElCloudStorageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubTopicIngestionDataSourceSettingsElCloudStorageEl {
    #[doc = "Cloud Storage bucket. The bucket name must be without any\nprefix like \"gs://\". See the bucket naming requirements:\nhttps://cloud.google.com/storage/docs/buckets#naming."]
    pub bucket: PrimField<String>,
}
impl BuildPubsubTopicIngestionDataSourceSettingsElCloudStorageEl {
    pub fn build(self) -> PubsubTopicIngestionDataSourceSettingsElCloudStorageEl {
        PubsubTopicIngestionDataSourceSettingsElCloudStorageEl {
            bucket: self.bucket,
            match_glob: core::default::Default::default(),
            minimum_object_create_time: core::default::Default::default(),
            avro_format: core::default::Default::default(),
            pubsub_avro_format: core::default::Default::default(),
            text_format: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PubsubTopicIngestionDataSourceSettingsElCloudStorageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubTopicIngestionDataSourceSettingsElCloudStorageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PubsubTopicIngestionDataSourceSettingsElCloudStorageElRef {
        PubsubTopicIngestionDataSourceSettingsElCloudStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubTopicIngestionDataSourceSettingsElCloudStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nCloud Storage bucket. The bucket name must be without any\nprefix like \"gs://\". See the bucket naming requirements:\nhttps://cloud.google.com/storage/docs/buckets#naming."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `match_glob` after provisioning.\nGlob pattern used to match objects that will be ingested. If unset, all\nobjects will be ingested. See the supported patterns:\nhttps://cloud.google.com/storage/docs/json_api/v1/objects/list#list-objects-and-prefixes-using-glob"]
    pub fn match_glob(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.match_glob", self.base))
    }
    #[doc = "Get a reference to the value of field `minimum_object_create_time` after provisioning.\nThe timestamp set in RFC3339 text format. If set, only objects with a\nlarger or equal timestamp will be ingested. Unset by default, meaning\nall objects will be ingested."]
    pub fn minimum_object_create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimum_object_create_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `avro_format` after provisioning.\n"]
    pub fn avro_format(
        &self,
    ) -> ListRef<PubsubTopicIngestionDataSourceSettingsElCloudStorageElAvroFormatElRef> {
        ListRef::new(self.shared().clone(), format!("{}.avro_format", self.base))
    }
    #[doc = "Get a reference to the value of field `pubsub_avro_format` after provisioning.\n"]
    pub fn pubsub_avro_format(
        &self,
    ) -> ListRef<PubsubTopicIngestionDataSourceSettingsElCloudStorageElPubsubAvroFormatElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pubsub_avro_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text_format` after provisioning.\n"]
    pub fn text_format(
        &self,
    ) -> ListRef<PubsubTopicIngestionDataSourceSettingsElCloudStorageElTextFormatElRef> {
        ListRef::new(self.shared().clone(), format!("{}.text_format", self.base))
    }
}
#[derive(Serialize)]
pub struct PubsubTopicIngestionDataSourceSettingsElConfluentCloudEl {
    bootstrap_server: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_id: Option<PrimField<String>>,
    gcp_service_account: PrimField<String>,
    identity_pool_id: PrimField<String>,
    topic: PrimField<String>,
}
impl PubsubTopicIngestionDataSourceSettingsElConfluentCloudEl {
    #[doc = "Set the field `cluster_id`.\nThe Confluent Cloud cluster ID."]
    pub fn set_cluster_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_id = Some(v.into());
        self
    }
}
impl ToListMappable for PubsubTopicIngestionDataSourceSettingsElConfluentCloudEl {
    type O = BlockAssignable<PubsubTopicIngestionDataSourceSettingsElConfluentCloudEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubTopicIngestionDataSourceSettingsElConfluentCloudEl {
    #[doc = "The Confluent Cloud bootstrap server. The format is url:port."]
    pub bootstrap_server: PrimField<String>,
    #[doc = "The GCP service account to be used for Federated Identity authentication\nwith Confluent Cloud."]
    pub gcp_service_account: PrimField<String>,
    #[doc = "Identity pool ID to be used for Federated Identity authentication with Confluent Cloud."]
    pub identity_pool_id: PrimField<String>,
    #[doc = "Name of the Confluent Cloud topic that Pub/Sub will import from."]
    pub topic: PrimField<String>,
}
impl BuildPubsubTopicIngestionDataSourceSettingsElConfluentCloudEl {
    pub fn build(self) -> PubsubTopicIngestionDataSourceSettingsElConfluentCloudEl {
        PubsubTopicIngestionDataSourceSettingsElConfluentCloudEl {
            bootstrap_server: self.bootstrap_server,
            cluster_id: core::default::Default::default(),
            gcp_service_account: self.gcp_service_account,
            identity_pool_id: self.identity_pool_id,
            topic: self.topic,
        }
    }
}
pub struct PubsubTopicIngestionDataSourceSettingsElConfluentCloudElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubTopicIngestionDataSourceSettingsElConfluentCloudElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PubsubTopicIngestionDataSourceSettingsElConfluentCloudElRef {
        PubsubTopicIngestionDataSourceSettingsElConfluentCloudElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubTopicIngestionDataSourceSettingsElConfluentCloudElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bootstrap_server` after provisioning.\nThe Confluent Cloud bootstrap server. The format is url:port."]
    pub fn bootstrap_server(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_server", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\nThe Confluent Cloud cluster ID."]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }
    #[doc = "Get a reference to the value of field `gcp_service_account` after provisioning.\nThe GCP service account to be used for Federated Identity authentication\nwith Confluent Cloud."]
    pub fn gcp_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `identity_pool_id` after provisioning.\nIdentity pool ID to be used for Federated Identity authentication with Confluent Cloud."]
    pub fn identity_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_pool_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nName of the Confluent Cloud topic that Pub/Sub will import from."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize)]
pub struct PubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<PrimField<String>>,
}
impl PubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl {
    #[doc = "Set the field `severity`.\nThe minimum severity level of Platform Logs that will be written. If unspecified,\nno Platform Logs will be written. Default value: \"SEVERITY_UNSPECIFIED\" Possible values: [\"SEVERITY_UNSPECIFIED\", \"DISABLED\", \"DEBUG\", \"INFO\", \"WARNING\", \"ERROR\"]"]
    pub fn set_severity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.severity = Some(v.into());
        self
    }
}
impl ToListMappable for PubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl {
    type O = BlockAssignable<PubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl {}
impl BuildPubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl {
    pub fn build(self) -> PubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl {
        PubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl {
            severity: core::default::Default::default(),
        }
    }
}
pub struct PubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsElRef {
        PubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `severity` after provisioning.\nThe minimum severity level of Platform Logs that will be written. If unspecified,\nno Platform Logs will be written. Default value: \"SEVERITY_UNSPECIFIED\" Possible values: [\"SEVERITY_UNSPECIFIED\", \"DISABLED\", \"DEBUG\", \"INFO\", \"WARNING\", \"ERROR\"]"]
    pub fn severity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }
}
#[derive(Serialize, Default)]
struct PubsubTopicIngestionDataSourceSettingsElDynamic {
    aws_kinesis: Option<DynamicBlock<PubsubTopicIngestionDataSourceSettingsElAwsKinesisEl>>,
    aws_msk: Option<DynamicBlock<PubsubTopicIngestionDataSourceSettingsElAwsMskEl>>,
    azure_event_hubs:
        Option<DynamicBlock<PubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl>>,
    cloud_storage: Option<DynamicBlock<PubsubTopicIngestionDataSourceSettingsElCloudStorageEl>>,
    confluent_cloud: Option<DynamicBlock<PubsubTopicIngestionDataSourceSettingsElConfluentCloudEl>>,
    platform_logs_settings:
        Option<DynamicBlock<PubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl>>,
}
#[derive(Serialize)]
pub struct PubsubTopicIngestionDataSourceSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_kinesis: Option<Vec<PubsubTopicIngestionDataSourceSettingsElAwsKinesisEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_msk: Option<Vec<PubsubTopicIngestionDataSourceSettingsElAwsMskEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    azure_event_hubs: Option<Vec<PubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_storage: Option<Vec<PubsubTopicIngestionDataSourceSettingsElCloudStorageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confluent_cloud: Option<Vec<PubsubTopicIngestionDataSourceSettingsElConfluentCloudEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_logs_settings:
        Option<Vec<PubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl>>,
    dynamic: PubsubTopicIngestionDataSourceSettingsElDynamic,
}
impl PubsubTopicIngestionDataSourceSettingsEl {
    #[doc = "Set the field `aws_kinesis`.\n"]
    pub fn set_aws_kinesis(
        mut self,
        v: impl Into<BlockAssignable<PubsubTopicIngestionDataSourceSettingsElAwsKinesisEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_kinesis = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_kinesis = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `aws_msk`.\n"]
    pub fn set_aws_msk(
        mut self,
        v: impl Into<BlockAssignable<PubsubTopicIngestionDataSourceSettingsElAwsMskEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_msk = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_msk = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `azure_event_hubs`.\n"]
    pub fn set_azure_event_hubs(
        mut self,
        v: impl Into<BlockAssignable<PubsubTopicIngestionDataSourceSettingsElAzureEventHubsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.azure_event_hubs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_event_hubs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cloud_storage`.\n"]
    pub fn set_cloud_storage(
        mut self,
        v: impl Into<BlockAssignable<PubsubTopicIngestionDataSourceSettingsElCloudStorageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_storage = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `confluent_cloud`.\n"]
    pub fn set_confluent_cloud(
        mut self,
        v: impl Into<BlockAssignable<PubsubTopicIngestionDataSourceSettingsElConfluentCloudEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.confluent_cloud = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.confluent_cloud = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `platform_logs_settings`.\n"]
    pub fn set_platform_logs_settings(
        mut self,
        v: impl Into<BlockAssignable<PubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.platform_logs_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.platform_logs_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PubsubTopicIngestionDataSourceSettingsEl {
    type O = BlockAssignable<PubsubTopicIngestionDataSourceSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubTopicIngestionDataSourceSettingsEl {}
impl BuildPubsubTopicIngestionDataSourceSettingsEl {
    pub fn build(self) -> PubsubTopicIngestionDataSourceSettingsEl {
        PubsubTopicIngestionDataSourceSettingsEl {
            aws_kinesis: core::default::Default::default(),
            aws_msk: core::default::Default::default(),
            azure_event_hubs: core::default::Default::default(),
            cloud_storage: core::default::Default::default(),
            confluent_cloud: core::default::Default::default(),
            platform_logs_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PubsubTopicIngestionDataSourceSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubTopicIngestionDataSourceSettingsElRef {
    fn new(shared: StackShared, base: String) -> PubsubTopicIngestionDataSourceSettingsElRef {
        PubsubTopicIngestionDataSourceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubTopicIngestionDataSourceSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aws_kinesis` after provisioning.\n"]
    pub fn aws_kinesis(&self) -> ListRef<PubsubTopicIngestionDataSourceSettingsElAwsKinesisElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aws_kinesis", self.base))
    }
    #[doc = "Get a reference to the value of field `aws_msk` after provisioning.\n"]
    pub fn aws_msk(&self) -> ListRef<PubsubTopicIngestionDataSourceSettingsElAwsMskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aws_msk", self.base))
    }
    #[doc = "Get a reference to the value of field `azure_event_hubs` after provisioning.\n"]
    pub fn azure_event_hubs(
        &self,
    ) -> ListRef<PubsubTopicIngestionDataSourceSettingsElAzureEventHubsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.azure_event_hubs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_storage` after provisioning.\n"]
    pub fn cloud_storage(
        &self,
    ) -> ListRef<PubsubTopicIngestionDataSourceSettingsElCloudStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_storage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `confluent_cloud` after provisioning.\n"]
    pub fn confluent_cloud(
        &self,
    ) -> ListRef<PubsubTopicIngestionDataSourceSettingsElConfluentCloudElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.confluent_cloud", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `platform_logs_settings` after provisioning.\n"]
    pub fn platform_logs_settings(
        &self,
    ) -> ListRef<PubsubTopicIngestionDataSourceSettingsElPlatformLogsSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.platform_logs_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PubsubTopicMessageStoragePolicyEl {
    allowed_persistence_regions: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_in_transit: Option<PrimField<bool>>,
}
impl PubsubTopicMessageStoragePolicyEl {
    #[doc = "Set the field `enforce_in_transit`.\nIf true, 'allowedPersistenceRegions' is also used to enforce in-transit\nguarantees for messages. That is, Pub/Sub will fail topics.publish\noperations on this topic and subscribe operations on any subscription\nattached to this topic in any region that is not in 'allowedPersistenceRegions'."]
    pub fn set_enforce_in_transit(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enforce_in_transit = Some(v.into());
        self
    }
}
impl ToListMappable for PubsubTopicMessageStoragePolicyEl {
    type O = BlockAssignable<PubsubTopicMessageStoragePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubTopicMessageStoragePolicyEl {
    #[doc = "A list of IDs of GCP regions where messages that are published to\nthe topic may be persisted in storage. Messages published by\npublishers running in non-allowed GCP regions (or running outside\nof GCP altogether) will be routed for storage in one of the\nallowed regions. An empty list means that no regions are allowed,\nand is not a valid configuration."]
    pub allowed_persistence_regions: SetField<PrimField<String>>,
}
impl BuildPubsubTopicMessageStoragePolicyEl {
    pub fn build(self) -> PubsubTopicMessageStoragePolicyEl {
        PubsubTopicMessageStoragePolicyEl {
            allowed_persistence_regions: self.allowed_persistence_regions,
            enforce_in_transit: core::default::Default::default(),
        }
    }
}
pub struct PubsubTopicMessageStoragePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubTopicMessageStoragePolicyElRef {
    fn new(shared: StackShared, base: String) -> PubsubTopicMessageStoragePolicyElRef {
        PubsubTopicMessageStoragePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubTopicMessageStoragePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_persistence_regions` after provisioning.\nA list of IDs of GCP regions where messages that are published to\nthe topic may be persisted in storage. Messages published by\npublishers running in non-allowed GCP regions (or running outside\nof GCP altogether) will be routed for storage in one of the\nallowed regions. An empty list means that no regions are allowed,\nand is not a valid configuration."]
    pub fn allowed_persistence_regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.allowed_persistence_regions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_in_transit` after provisioning.\nIf true, 'allowedPersistenceRegions' is also used to enforce in-transit\nguarantees for messages. That is, Pub/Sub will fail topics.publish\noperations on this topic and subscribe operations on any subscription\nattached to this topic in any region that is not in 'allowedPersistenceRegions'."]
    pub fn enforce_in_transit(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_in_transit", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
}
impl PubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl {
    #[doc = "Set the field `parameters`.\nA parameters object to be included in each inference request.\nThe parameters object is combined with the data field of the Pub/Sub\nmessage to form the inference request."]
    pub fn set_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.parameters = Some(v.into());
        self
    }
}
impl ToListMappable for PubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl {
    type O = BlockAssignable<PubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl {}
impl BuildPubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl {
    pub fn build(self) -> PubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl {
        PubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl {
            parameters: core::default::Default::default(),
        }
    }
}
pub struct PubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceElRef {
        PubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\nA parameters object to be included in each inference request.\nThe parameters object is combined with the data field of the Pub/Sub\nmessage to form the inference request."]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}
#[derive(Serialize, Default)]
struct PubsubTopicMessageTransformsElAiInferenceElDynamic {
    unstructured_inference:
        Option<DynamicBlock<PubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl>>,
}
#[derive(Serialize)]
pub struct PubsubTopicMessageTransformsElAiInferenceEl {
    endpoint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unstructured_inference:
        Option<Vec<PubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl>>,
    dynamic: PubsubTopicMessageTransformsElAiInferenceElDynamic,
}
impl PubsubTopicMessageTransformsElAiInferenceEl {
    #[doc = "Set the field `service_account_email`.\nThe service account to use to make prediction requests against\nendpoints."]
    pub fn set_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `unstructured_inference`.\n"]
    pub fn set_unstructured_inference(
        mut self,
        v: impl Into<
            BlockAssignable<PubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.unstructured_inference = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.unstructured_inference = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PubsubTopicMessageTransformsElAiInferenceEl {
    type O = BlockAssignable<PubsubTopicMessageTransformsElAiInferenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubTopicMessageTransformsElAiInferenceEl {
    #[doc = "The endpoint to a Vertex AI model of the form\n'projects/{project}/locations/{location}/endpoints/{endpoint}' or\n'projects/{project}/locations/{location}/publishers/{publisher}/models/{model}'.\nVertex AI API requests will be sent to this endpoint."]
    pub endpoint: PrimField<String>,
}
impl BuildPubsubTopicMessageTransformsElAiInferenceEl {
    pub fn build(self) -> PubsubTopicMessageTransformsElAiInferenceEl {
        PubsubTopicMessageTransformsElAiInferenceEl {
            endpoint: self.endpoint,
            service_account_email: core::default::Default::default(),
            unstructured_inference: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PubsubTopicMessageTransformsElAiInferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubTopicMessageTransformsElAiInferenceElRef {
    fn new(shared: StackShared, base: String) -> PubsubTopicMessageTransformsElAiInferenceElRef {
        PubsubTopicMessageTransformsElAiInferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubTopicMessageTransformsElAiInferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nThe endpoint to a Vertex AI model of the form\n'projects/{project}/locations/{location}/endpoints/{endpoint}' or\n'projects/{project}/locations/{location}/publishers/{publisher}/models/{model}'.\nVertex AI API requests will be sent to this endpoint."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\nThe service account to use to make prediction requests against\nendpoints."]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `unstructured_inference` after provisioning.\n"]
    pub fn unstructured_inference(
        &self,
    ) -> ListRef<PubsubTopicMessageTransformsElAiInferenceElUnstructuredInferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.unstructured_inference", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PubsubTopicMessageTransformsElJavascriptUdfEl {
    code: PrimField<String>,
    function_name: PrimField<String>,
}
impl PubsubTopicMessageTransformsElJavascriptUdfEl {}
impl ToListMappable for PubsubTopicMessageTransformsElJavascriptUdfEl {
    type O = BlockAssignable<PubsubTopicMessageTransformsElJavascriptUdfEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubTopicMessageTransformsElJavascriptUdfEl {
    #[doc = "JavaScript code that contains a function 'function_name' with the\nfollowing signature:\n'''\n  /**\n  * Transforms a Pub/Sub message.\n  *\n  * @return {(Object<string, (string | Object<string, string>)>|null)} - To\n  * filter a message, return 'null'. To transform a message return a map\n  * with the following keys:\n  *   - (required) 'data' : {string}\n  *   - (optional) 'attributes' : {Object<string, string>}\n  * Returning empty 'attributes' will remove all attributes from the\n  * message.\n  *\n  * @param  {(Object<string, (string | Object<string, string>)>} Pub/Sub\n  * message. Keys:\n  *   - (required) 'data' : {string}\n  *   - (required) 'attributes' : {Object<string, string>}\n  *\n  * @param  {Object<string, any>} metadata - Pub/Sub message metadata.\n  * Keys:\n  *   - (required) 'message_id'  : {string}\n  *   - (optional) 'publish_time': {string} YYYY-MM-DDTHH:MM:SSZ format\n  *   - (optional) 'ordering_key': {string}\n  */\n  function <function_name>(message, metadata) {\n  }\n'''"]
    pub code: PrimField<String>,
    #[doc = "Name of the JavaScript function that should be applied to Pub/Sub messages."]
    pub function_name: PrimField<String>,
}
impl BuildPubsubTopicMessageTransformsElJavascriptUdfEl {
    pub fn build(self) -> PubsubTopicMessageTransformsElJavascriptUdfEl {
        PubsubTopicMessageTransformsElJavascriptUdfEl {
            code: self.code,
            function_name: self.function_name,
        }
    }
}
pub struct PubsubTopicMessageTransformsElJavascriptUdfElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubTopicMessageTransformsElJavascriptUdfElRef {
    fn new(shared: StackShared, base: String) -> PubsubTopicMessageTransformsElJavascriptUdfElRef {
        PubsubTopicMessageTransformsElJavascriptUdfElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubTopicMessageTransformsElJavascriptUdfElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\nJavaScript code that contains a function 'function_name' with the\nfollowing signature:\n'''\n  /**\n  * Transforms a Pub/Sub message.\n  *\n  * @return {(Object<string, (string | Object<string, string>)>|null)} - To\n  * filter a message, return 'null'. To transform a message return a map\n  * with the following keys:\n  *   - (required) 'data' : {string}\n  *   - (optional) 'attributes' : {Object<string, string>}\n  * Returning empty 'attributes' will remove all attributes from the\n  * message.\n  *\n  * @param  {(Object<string, (string | Object<string, string>)>} Pub/Sub\n  * message. Keys:\n  *   - (required) 'data' : {string}\n  *   - (required) 'attributes' : {Object<string, string>}\n  *\n  * @param  {Object<string, any>} metadata - Pub/Sub message metadata.\n  * Keys:\n  *   - (required) 'message_id'  : {string}\n  *   - (optional) 'publish_time': {string} YYYY-MM-DDTHH:MM:SSZ format\n  *   - (optional) 'ordering_key': {string}\n  */\n  function <function_name>(message, metadata) {\n  }\n'''"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `function_name` after provisioning.\nName of the JavaScript function that should be applied to Pub/Sub messages."]
    pub fn function_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.function_name", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PubsubTopicMessageTransformsElDynamic {
    ai_inference: Option<DynamicBlock<PubsubTopicMessageTransformsElAiInferenceEl>>,
    javascript_udf: Option<DynamicBlock<PubsubTopicMessageTransformsElJavascriptUdfEl>>,
}
#[derive(Serialize)]
pub struct PubsubTopicMessageTransformsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ai_inference: Option<Vec<PubsubTopicMessageTransformsElAiInferenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    javascript_udf: Option<Vec<PubsubTopicMessageTransformsElJavascriptUdfEl>>,
    dynamic: PubsubTopicMessageTransformsElDynamic,
}
impl PubsubTopicMessageTransformsEl {
    #[doc = "Set the field `disabled`.\nControls whether or not to use this transform. If not set or 'false',\nthe transform will be applied to messages. Default: 'true'."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `ai_inference`.\n"]
    pub fn set_ai_inference(
        mut self,
        v: impl Into<BlockAssignable<PubsubTopicMessageTransformsElAiInferenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ai_inference = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ai_inference = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `javascript_udf`.\n"]
    pub fn set_javascript_udf(
        mut self,
        v: impl Into<BlockAssignable<PubsubTopicMessageTransformsElJavascriptUdfEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.javascript_udf = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.javascript_udf = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PubsubTopicMessageTransformsEl {
    type O = BlockAssignable<PubsubTopicMessageTransformsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubTopicMessageTransformsEl {}
impl BuildPubsubTopicMessageTransformsEl {
    pub fn build(self) -> PubsubTopicMessageTransformsEl {
        PubsubTopicMessageTransformsEl {
            disabled: core::default::Default::default(),
            ai_inference: core::default::Default::default(),
            javascript_udf: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PubsubTopicMessageTransformsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubTopicMessageTransformsElRef {
    fn new(shared: StackShared, base: String) -> PubsubTopicMessageTransformsElRef {
        PubsubTopicMessageTransformsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubTopicMessageTransformsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nControls whether or not to use this transform. If not set or 'false',\nthe transform will be applied to messages. Default: 'true'."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `ai_inference` after provisioning.\n"]
    pub fn ai_inference(&self) -> ListRef<PubsubTopicMessageTransformsElAiInferenceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ai_inference", self.base))
    }
    #[doc = "Get a reference to the value of field `javascript_udf` after provisioning.\n"]
    pub fn javascript_udf(&self) -> ListRef<PubsubTopicMessageTransformsElJavascriptUdfElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.javascript_udf", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PubsubTopicSchemaSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
    schema: PrimField<String>,
}
impl PubsubTopicSchemaSettingsEl {
    #[doc = "Set the field `encoding`.\nThe encoding of messages validated against schema. Default value: \"ENCODING_UNSPECIFIED\" Possible values: [\"ENCODING_UNSPECIFIED\", \"JSON\", \"BINARY\"]"]
    pub fn set_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding = Some(v.into());
        self
    }
}
impl ToListMappable for PubsubTopicSchemaSettingsEl {
    type O = BlockAssignable<PubsubTopicSchemaSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubTopicSchemaSettingsEl {
    #[doc = "The name of the schema that messages published should be\nvalidated against. Format is projects/{project}/schemas/{schema}.\nThe value of this field will be _deleted-schema_\nif the schema has been deleted."]
    pub schema: PrimField<String>,
}
impl BuildPubsubTopicSchemaSettingsEl {
    pub fn build(self) -> PubsubTopicSchemaSettingsEl {
        PubsubTopicSchemaSettingsEl {
            encoding: core::default::Default::default(),
            schema: self.schema,
        }
    }
}
pub struct PubsubTopicSchemaSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubTopicSchemaSettingsElRef {
    fn new(shared: StackShared, base: String) -> PubsubTopicSchemaSettingsElRef {
        PubsubTopicSchemaSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubTopicSchemaSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encoding` after provisioning.\nThe encoding of messages validated against schema. Default value: \"ENCODING_UNSPECIFIED\" Possible values: [\"ENCODING_UNSPECIFIED\", \"JSON\", \"BINARY\"]"]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nThe name of the schema that messages published should be\nvalidated against. Format is projects/{project}/schemas/{schema}.\nThe value of this field will be _deleted-schema_\nif the schema has been deleted."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}
#[derive(Serialize)]
pub struct PubsubTopicTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl PubsubTopicTimeoutsEl {
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
impl ToListMappable for PubsubTopicTimeoutsEl {
    type O = BlockAssignable<PubsubTopicTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPubsubTopicTimeoutsEl {}
impl BuildPubsubTopicTimeoutsEl {
    pub fn build(self) -> PubsubTopicTimeoutsEl {
        PubsubTopicTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct PubsubTopicTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PubsubTopicTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> PubsubTopicTimeoutsElRef {
        PubsubTopicTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PubsubTopicTimeoutsElRef {
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
struct PubsubTopicDynamic {
    ingestion_data_source_settings: Option<DynamicBlock<PubsubTopicIngestionDataSourceSettingsEl>>,
    message_storage_policy: Option<DynamicBlock<PubsubTopicMessageStoragePolicyEl>>,
    message_transforms: Option<DynamicBlock<PubsubTopicMessageTransformsEl>>,
    schema_settings: Option<DynamicBlock<PubsubTopicSchemaSettingsEl>>,
}
