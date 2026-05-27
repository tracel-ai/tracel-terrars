use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataPubsubSubscriptionData {
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
struct DataPubsubSubscription_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataPubsubSubscriptionData>,
}
#[derive(Clone)]
pub struct DataPubsubSubscription(Rc<DataPubsubSubscription_>);
impl DataPubsubSubscription {
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
    #[doc = "Get a reference to the value of field `ack_deadline_seconds` after provisioning.\nThis value is the maximum time after a subscriber receives a message\nbefore the subscriber should acknowledge the message. After message\ndelivery but before the ack deadline expires and before the message is\nacknowledged, it is an outstanding message and will not be delivered\nagain during that time (on a best-effort basis).\n\nFor pull subscriptions, this value is used as the initial value for\nthe ack deadline. To override this value for a given message, call\nsubscriptions.modifyAckDeadline with the corresponding ackId if using\npull. The minimum custom deadline you can specify is 10 seconds. The\nmaximum custom deadline you can specify is 600 seconds (10 minutes).\nIf this parameter is 0, a default value of 10 seconds is used.\n\nFor push delivery, this value is also used to set the request timeout\nfor the call to the push endpoint.\n\nIf the subscriber never acknowledges the message, the Pub/Sub system\nwill eventually redeliver the message."]
    pub fn ack_deadline_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ack_deadline_seconds", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bigquery_config` after provisioning.\nIf delivery to BigQuery is used with this subscription, this field is used to configure it.\nEither pushConfig, bigQueryConfig or cloudStorageConfig can be set, but not combined.\nIf all three are empty, then the subscriber will pull and ack messages using API methods."]
    pub fn bigquery_config(&self) -> ListRef<DataPubsubSubscriptionBigqueryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_storage_config` after provisioning.\nIf delivery to Cloud Storage is used with this subscription, this field is used to configure it.\nEither pushConfig, bigQueryConfig or cloudStorageConfig can be set, but not combined.\nIf all three are empty, then the subscriber will pull and ack messages using API methods."]
    pub fn cloud_storage_config(&self) -> ListRef<DataPubsubSubscriptionCloudStorageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_storage_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dead_letter_policy` after provisioning.\nA policy that specifies the conditions for dead lettering messages in\nthis subscription. If dead_letter_policy is not set, dead lettering\nis disabled.\n\nThe Cloud Pub/Sub service account associated with this subscription's\nparent project (i.e.,\nservice-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com) must have\npermission to Acknowledge() messages on this subscription."]
    pub fn dead_letter_policy(&self) -> ListRef<DataPubsubSubscriptionDeadLetterPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dead_letter_policy", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `enable_exactly_once_delivery` after provisioning.\nIf 'true', Pub/Sub provides the following guarantees for the delivery\nof a message with a given value of messageId on this Subscriptions':\n\n- The message sent to a subscriber is guaranteed not to be resent before the message's acknowledgement deadline expires.\n\n- An acknowledged message will not be resent to a subscriber.\n\nNote that subscribers may still receive multiple copies of a message when 'enable_exactly_once_delivery'\nis true if the message was published multiple times by a publisher client. These copies are considered distinct by Pub/Sub and have distinct messageId values"]
    pub fn enable_exactly_once_delivery(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_exactly_once_delivery", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_message_ordering` after provisioning.\nIf 'true', messages published with the same orderingKey in PubsubMessage will be delivered to\nthe subscribers in the order in which they are received by the Pub/Sub system. Otherwise, they\nmay be delivered in any order."]
    pub fn enable_message_ordering(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_message_ordering", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expiration_policy` after provisioning.\nA policy that specifies the conditions for this subscription's expiration.\nA subscription is considered active as long as any connected subscriber\nis successfully consuming messages from the subscription or is issuing\noperations on the subscription. If expirationPolicy is not set, a default\npolicy with ttl of 31 days will be used.  If it is set but ttl is \"\", the\nresource never expires.  The minimum allowed value for expirationPolicy.ttl\nis 1 day."]
    pub fn expiration_policy(&self) -> ListRef<DataPubsubSubscriptionExpirationPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.expiration_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nThe subscription only delivers the messages that match the filter.\nPub/Sub automatically acknowledges the messages that don't match the filter. You can filter messages\nby their attributes. The maximum length of a filter is 256 bytes. After creating the subscription,\nyou can't modify the filter."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this Subscription.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `message_retention_duration` after provisioning.\nHow long to retain unacknowledged messages in the subscription's\nbacklog, from the moment a message is published. If\nretain_acked_messages is true, then this also configures the retention\nof acknowledged messages, and thus configures how far back in time a\nsubscriptions.seek can be done. Defaults to 7 days. Cannot be more\nthan 31 days ('\"2678400s\"') or less than 10 minutes ('\"600s\"').\n\nA duration in seconds with up to nine fractional digits, terminated\nby 's'. Example: '\"600.5s\"'."]
    pub fn message_retention_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_retention_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `message_transforms` after provisioning.\nTransforms to be applied to messages published to the topic. Transforms are applied in the\norder specified."]
    pub fn message_transforms(&self) -> ListRef<DataPubsubSubscriptionMessageTransformsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.message_transforms", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the subscription."]
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
    #[doc = "Get a reference to the value of field `push_config` after provisioning.\nIf push delivery is used with this subscription, this field is used to\nconfigure it. An empty pushConfig signifies that the subscriber will\npull and ack messages using API methods."]
    pub fn push_config(&self) -> ListRef<DataPubsubSubscriptionPushConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.push_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retain_acked_messages` after provisioning.\nIndicates whether to retain acknowledged messages. If 'true', then\nmessages are not expunged from the subscription's backlog, even if\nthey are acknowledged, until they fall out of the\nmessageRetentionDuration window."]
    pub fn retain_acked_messages(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retain_acked_messages", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\nA policy that specifies how Pub/Sub retries message delivery for this subscription.\n\nIf not set, the default retry policy is applied. This generally implies that messages will be retried as soon as possible for healthy subscribers.\nRetryPolicy will be triggered on NACKs or acknowledgement deadline exceeded events for a given message"]
    pub fn retry_policy(&self) -> ListRef<DataPubsubSubscriptionRetryPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nInput only. Resource manager tags to be bound to the subscription. Tag\nkeys and values have the same definition as resource manager tags. Keys\nmust be in the format tagKeys/{tag_key_id}, and values are in the format\ntagValues/456. The field is ignored when empty. The field is immutable and\ncauses resource replacement when mutated. This field is only set at create\ntime and modifying this field after creation will trigger recreation. To\napply tags to an existing resource, see the 'google_tags_tag_value'\nresource."]
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
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nA reference to a Topic resource, of the form projects/{project}/topics/{{name}}\n(as in the id property of a google_pubsub_topic), or just a topic name if\nthe topic is in the same project as the subscription."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.topic", self.extract_ref()),
        )
    }
}
impl Referable for DataPubsubSubscription {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataPubsubSubscription {}
impl ToListMappable for DataPubsubSubscription {
    type O = ListRef<DataPubsubSubscriptionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataPubsubSubscription_ {
    fn extract_datasource_type(&self) -> String {
        "google_pubsub_subscription".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataPubsubSubscription {
    pub tf_id: String,
    #[doc = "Name of the subscription."]
    pub name: PrimField<String>,
}
impl BuildDataPubsubSubscription {
    pub fn build(self, stack: &mut Stack) -> DataPubsubSubscription {
        let out = DataPubsubSubscription(Rc::new(DataPubsubSubscription_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataPubsubSubscriptionData {
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
pub struct DataPubsubSubscriptionRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubSubscriptionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataPubsubSubscriptionRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `ack_deadline_seconds` after provisioning.\nThis value is the maximum time after a subscriber receives a message\nbefore the subscriber should acknowledge the message. After message\ndelivery but before the ack deadline expires and before the message is\nacknowledged, it is an outstanding message and will not be delivered\nagain during that time (on a best-effort basis).\n\nFor pull subscriptions, this value is used as the initial value for\nthe ack deadline. To override this value for a given message, call\nsubscriptions.modifyAckDeadline with the corresponding ackId if using\npull. The minimum custom deadline you can specify is 10 seconds. The\nmaximum custom deadline you can specify is 600 seconds (10 minutes).\nIf this parameter is 0, a default value of 10 seconds is used.\n\nFor push delivery, this value is also used to set the request timeout\nfor the call to the push endpoint.\n\nIf the subscriber never acknowledges the message, the Pub/Sub system\nwill eventually redeliver the message."]
    pub fn ack_deadline_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ack_deadline_seconds", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bigquery_config` after provisioning.\nIf delivery to BigQuery is used with this subscription, this field is used to configure it.\nEither pushConfig, bigQueryConfig or cloudStorageConfig can be set, but not combined.\nIf all three are empty, then the subscriber will pull and ack messages using API methods."]
    pub fn bigquery_config(&self) -> ListRef<DataPubsubSubscriptionBigqueryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_storage_config` after provisioning.\nIf delivery to Cloud Storage is used with this subscription, this field is used to configure it.\nEither pushConfig, bigQueryConfig or cloudStorageConfig can be set, but not combined.\nIf all three are empty, then the subscriber will pull and ack messages using API methods."]
    pub fn cloud_storage_config(&self) -> ListRef<DataPubsubSubscriptionCloudStorageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_storage_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dead_letter_policy` after provisioning.\nA policy that specifies the conditions for dead lettering messages in\nthis subscription. If dead_letter_policy is not set, dead lettering\nis disabled.\n\nThe Cloud Pub/Sub service account associated with this subscription's\nparent project (i.e.,\nservice-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com) must have\npermission to Acknowledge() messages on this subscription."]
    pub fn dead_letter_policy(&self) -> ListRef<DataPubsubSubscriptionDeadLetterPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dead_letter_policy", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `enable_exactly_once_delivery` after provisioning.\nIf 'true', Pub/Sub provides the following guarantees for the delivery\nof a message with a given value of messageId on this Subscriptions':\n\n- The message sent to a subscriber is guaranteed not to be resent before the message's acknowledgement deadline expires.\n\n- An acknowledged message will not be resent to a subscriber.\n\nNote that subscribers may still receive multiple copies of a message when 'enable_exactly_once_delivery'\nis true if the message was published multiple times by a publisher client. These copies are considered distinct by Pub/Sub and have distinct messageId values"]
    pub fn enable_exactly_once_delivery(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_exactly_once_delivery", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_message_ordering` after provisioning.\nIf 'true', messages published with the same orderingKey in PubsubMessage will be delivered to\nthe subscribers in the order in which they are received by the Pub/Sub system. Otherwise, they\nmay be delivered in any order."]
    pub fn enable_message_ordering(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_message_ordering", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expiration_policy` after provisioning.\nA policy that specifies the conditions for this subscription's expiration.\nA subscription is considered active as long as any connected subscriber\nis successfully consuming messages from the subscription or is issuing\noperations on the subscription. If expirationPolicy is not set, a default\npolicy with ttl of 31 days will be used.  If it is set but ttl is \"\", the\nresource never expires.  The minimum allowed value for expirationPolicy.ttl\nis 1 day."]
    pub fn expiration_policy(&self) -> ListRef<DataPubsubSubscriptionExpirationPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.expiration_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nThe subscription only delivers the messages that match the filter.\nPub/Sub automatically acknowledges the messages that don't match the filter. You can filter messages\nby their attributes. The maximum length of a filter is 256 bytes. After creating the subscription,\nyou can't modify the filter."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this Subscription.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `message_retention_duration` after provisioning.\nHow long to retain unacknowledged messages in the subscription's\nbacklog, from the moment a message is published. If\nretain_acked_messages is true, then this also configures the retention\nof acknowledged messages, and thus configures how far back in time a\nsubscriptions.seek can be done. Defaults to 7 days. Cannot be more\nthan 31 days ('\"2678400s\"') or less than 10 minutes ('\"600s\"').\n\nA duration in seconds with up to nine fractional digits, terminated\nby 's'. Example: '\"600.5s\"'."]
    pub fn message_retention_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_retention_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `message_transforms` after provisioning.\nTransforms to be applied to messages published to the topic. Transforms are applied in the\norder specified."]
    pub fn message_transforms(&self) -> ListRef<DataPubsubSubscriptionMessageTransformsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.message_transforms", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the subscription."]
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
    #[doc = "Get a reference to the value of field `push_config` after provisioning.\nIf push delivery is used with this subscription, this field is used to\nconfigure it. An empty pushConfig signifies that the subscriber will\npull and ack messages using API methods."]
    pub fn push_config(&self) -> ListRef<DataPubsubSubscriptionPushConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.push_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retain_acked_messages` after provisioning.\nIndicates whether to retain acknowledged messages. If 'true', then\nmessages are not expunged from the subscription's backlog, even if\nthey are acknowledged, until they fall out of the\nmessageRetentionDuration window."]
    pub fn retain_acked_messages(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retain_acked_messages", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\nA policy that specifies how Pub/Sub retries message delivery for this subscription.\n\nIf not set, the default retry policy is applied. This generally implies that messages will be retried as soon as possible for healthy subscribers.\nRetryPolicy will be triggered on NACKs or acknowledgement deadline exceeded events for a given message"]
    pub fn retry_policy(&self) -> ListRef<DataPubsubSubscriptionRetryPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nInput only. Resource manager tags to be bound to the subscription. Tag\nkeys and values have the same definition as resource manager tags. Keys\nmust be in the format tagKeys/{tag_key_id}, and values are in the format\ntagValues/456. The field is ignored when empty. The field is immutable and\ncauses resource replacement when mutated. This field is only set at create\ntime and modifying this field after creation will trigger recreation. To\napply tags to an existing resource, see the 'google_tags_tag_value'\nresource."]
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
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nA reference to a Topic resource, of the form projects/{project}/topics/{{name}}\n(as in the id property of a google_pubsub_topic), or just a topic name if\nthe topic is in the same project as the subscription."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.topic", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataPubsubSubscriptionBigqueryConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_unknown_fields: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_table_schema: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_topic_schema: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_metadata: Option<PrimField<bool>>,
}
impl DataPubsubSubscriptionBigqueryConfigEl {
    #[doc = "Set the field `drop_unknown_fields`.\n"]
    pub fn set_drop_unknown_fields(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.drop_unknown_fields = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_email`.\n"]
    pub fn set_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `table`.\n"]
    pub fn set_table(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table = Some(v.into());
        self
    }
    #[doc = "Set the field `use_table_schema`.\n"]
    pub fn set_use_table_schema(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_table_schema = Some(v.into());
        self
    }
    #[doc = "Set the field `use_topic_schema`.\n"]
    pub fn set_use_topic_schema(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_topic_schema = Some(v.into());
        self
    }
    #[doc = "Set the field `write_metadata`.\n"]
    pub fn set_write_metadata(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.write_metadata = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubSubscriptionBigqueryConfigEl {
    type O = BlockAssignable<DataPubsubSubscriptionBigqueryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubSubscriptionBigqueryConfigEl {}
impl BuildDataPubsubSubscriptionBigqueryConfigEl {
    pub fn build(self) -> DataPubsubSubscriptionBigqueryConfigEl {
        DataPubsubSubscriptionBigqueryConfigEl {
            drop_unknown_fields: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            table: core::default::Default::default(),
            use_table_schema: core::default::Default::default(),
            use_topic_schema: core::default::Default::default(),
            write_metadata: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubSubscriptionBigqueryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubSubscriptionBigqueryConfigElRef {
    fn new(shared: StackShared, base: String) -> DataPubsubSubscriptionBigqueryConfigElRef {
        DataPubsubSubscriptionBigqueryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubSubscriptionBigqueryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `drop_unknown_fields` after provisioning.\n"]
    pub fn drop_unknown_fields(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.drop_unknown_fields", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\n"]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
    #[doc = "Get a reference to the value of field `use_table_schema` after provisioning.\n"]
    pub fn use_table_schema(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_table_schema", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `use_topic_schema` after provisioning.\n"]
    pub fn use_topic_schema(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_topic_schema", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `write_metadata` after provisioning.\n"]
    pub fn write_metadata(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.write_metadata", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataPubsubSubscriptionCloudStorageConfigElAvroConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    use_topic_schema: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_metadata: Option<PrimField<bool>>,
}
impl DataPubsubSubscriptionCloudStorageConfigElAvroConfigEl {
    #[doc = "Set the field `use_topic_schema`.\n"]
    pub fn set_use_topic_schema(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_topic_schema = Some(v.into());
        self
    }
    #[doc = "Set the field `write_metadata`.\n"]
    pub fn set_write_metadata(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.write_metadata = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubSubscriptionCloudStorageConfigElAvroConfigEl {
    type O = BlockAssignable<DataPubsubSubscriptionCloudStorageConfigElAvroConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubSubscriptionCloudStorageConfigElAvroConfigEl {}
impl BuildDataPubsubSubscriptionCloudStorageConfigElAvroConfigEl {
    pub fn build(self) -> DataPubsubSubscriptionCloudStorageConfigElAvroConfigEl {
        DataPubsubSubscriptionCloudStorageConfigElAvroConfigEl {
            use_topic_schema: core::default::Default::default(),
            write_metadata: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubSubscriptionCloudStorageConfigElAvroConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubSubscriptionCloudStorageConfigElAvroConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPubsubSubscriptionCloudStorageConfigElAvroConfigElRef {
        DataPubsubSubscriptionCloudStorageConfigElAvroConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubSubscriptionCloudStorageConfigElAvroConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `use_topic_schema` after provisioning.\n"]
    pub fn use_topic_schema(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_topic_schema", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `write_metadata` after provisioning.\n"]
    pub fn write_metadata(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.write_metadata", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataPubsubSubscriptionCloudStorageConfigElTextConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl DataPubsubSubscriptionCloudStorageConfigElTextConfigEl {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubSubscriptionCloudStorageConfigElTextConfigEl {
    type O = BlockAssignable<DataPubsubSubscriptionCloudStorageConfigElTextConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubSubscriptionCloudStorageConfigElTextConfigEl {}
impl BuildDataPubsubSubscriptionCloudStorageConfigElTextConfigEl {
    pub fn build(self) -> DataPubsubSubscriptionCloudStorageConfigElTextConfigEl {
        DataPubsubSubscriptionCloudStorageConfigElTextConfigEl {
            state: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubSubscriptionCloudStorageConfigElTextConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubSubscriptionCloudStorageConfigElTextConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPubsubSubscriptionCloudStorageConfigElTextConfigElRef {
        DataPubsubSubscriptionCloudStorageConfigElTextConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubSubscriptionCloudStorageConfigElTextConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct DataPubsubSubscriptionCloudStorageConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    avro_config: Option<ListField<DataPubsubSubscriptionCloudStorageConfigElAvroConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filename_datetime_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filename_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filename_suffix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_messages: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_config: Option<ListField<DataPubsubSubscriptionCloudStorageConfigElTextConfigEl>>,
}
impl DataPubsubSubscriptionCloudStorageConfigEl {
    #[doc = "Set the field `avro_config`.\n"]
    pub fn set_avro_config(
        mut self,
        v: impl Into<ListField<DataPubsubSubscriptionCloudStorageConfigElAvroConfigEl>>,
    ) -> Self {
        self.avro_config = Some(v.into());
        self
    }
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `filename_datetime_format`.\n"]
    pub fn set_filename_datetime_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filename_datetime_format = Some(v.into());
        self
    }
    #[doc = "Set the field `filename_prefix`.\n"]
    pub fn set_filename_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filename_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `filename_suffix`.\n"]
    pub fn set_filename_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filename_suffix = Some(v.into());
        self
    }
    #[doc = "Set the field `max_bytes`.\n"]
    pub fn set_max_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_bytes = Some(v.into());
        self
    }
    #[doc = "Set the field `max_duration`.\n"]
    pub fn set_max_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `max_messages`.\n"]
    pub fn set_max_messages(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_messages = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_email`.\n"]
    pub fn set_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `text_config`.\n"]
    pub fn set_text_config(
        mut self,
        v: impl Into<ListField<DataPubsubSubscriptionCloudStorageConfigElTextConfigEl>>,
    ) -> Self {
        self.text_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubSubscriptionCloudStorageConfigEl {
    type O = BlockAssignable<DataPubsubSubscriptionCloudStorageConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubSubscriptionCloudStorageConfigEl {}
impl BuildDataPubsubSubscriptionCloudStorageConfigEl {
    pub fn build(self) -> DataPubsubSubscriptionCloudStorageConfigEl {
        DataPubsubSubscriptionCloudStorageConfigEl {
            avro_config: core::default::Default::default(),
            bucket: core::default::Default::default(),
            filename_datetime_format: core::default::Default::default(),
            filename_prefix: core::default::Default::default(),
            filename_suffix: core::default::Default::default(),
            max_bytes: core::default::Default::default(),
            max_duration: core::default::Default::default(),
            max_messages: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            state: core::default::Default::default(),
            text_config: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubSubscriptionCloudStorageConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubSubscriptionCloudStorageConfigElRef {
    fn new(shared: StackShared, base: String) -> DataPubsubSubscriptionCloudStorageConfigElRef {
        DataPubsubSubscriptionCloudStorageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubSubscriptionCloudStorageConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `avro_config` after provisioning.\n"]
    pub fn avro_config(
        &self,
    ) -> ListRef<DataPubsubSubscriptionCloudStorageConfigElAvroConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.avro_config", self.base))
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `filename_datetime_format` after provisioning.\n"]
    pub fn filename_datetime_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filename_datetime_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filename_prefix` after provisioning.\n"]
    pub fn filename_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filename_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filename_suffix` after provisioning.\n"]
    pub fn filename_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filename_suffix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_bytes` after provisioning.\n"]
    pub fn max_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_bytes", self.base))
    }
    #[doc = "Get a reference to the value of field `max_duration` after provisioning.\n"]
    pub fn max_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_duration", self.base))
    }
    #[doc = "Get a reference to the value of field `max_messages` after provisioning.\n"]
    pub fn max_messages(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_messages", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\n"]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `text_config` after provisioning.\n"]
    pub fn text_config(
        &self,
    ) -> ListRef<DataPubsubSubscriptionCloudStorageConfigElTextConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.text_config", self.base))
    }
}
#[derive(Serialize)]
pub struct DataPubsubSubscriptionDeadLetterPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_letter_topic: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_delivery_attempts: Option<PrimField<f64>>,
}
impl DataPubsubSubscriptionDeadLetterPolicyEl {
    #[doc = "Set the field `dead_letter_topic`.\n"]
    pub fn set_dead_letter_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dead_letter_topic = Some(v.into());
        self
    }
    #[doc = "Set the field `max_delivery_attempts`.\n"]
    pub fn set_max_delivery_attempts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_delivery_attempts = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubSubscriptionDeadLetterPolicyEl {
    type O = BlockAssignable<DataPubsubSubscriptionDeadLetterPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubSubscriptionDeadLetterPolicyEl {}
impl BuildDataPubsubSubscriptionDeadLetterPolicyEl {
    pub fn build(self) -> DataPubsubSubscriptionDeadLetterPolicyEl {
        DataPubsubSubscriptionDeadLetterPolicyEl {
            dead_letter_topic: core::default::Default::default(),
            max_delivery_attempts: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubSubscriptionDeadLetterPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubSubscriptionDeadLetterPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataPubsubSubscriptionDeadLetterPolicyElRef {
        DataPubsubSubscriptionDeadLetterPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubSubscriptionDeadLetterPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dead_letter_topic` after provisioning.\n"]
    pub fn dead_letter_topic(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dead_letter_topic", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_delivery_attempts` after provisioning.\n"]
    pub fn max_delivery_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_delivery_attempts", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataPubsubSubscriptionExpirationPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<String>>,
}
impl DataPubsubSubscriptionExpirationPolicyEl {
    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ttl = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubSubscriptionExpirationPolicyEl {
    type O = BlockAssignable<DataPubsubSubscriptionExpirationPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubSubscriptionExpirationPolicyEl {}
impl BuildDataPubsubSubscriptionExpirationPolicyEl {
    pub fn build(self) -> DataPubsubSubscriptionExpirationPolicyEl {
        DataPubsubSubscriptionExpirationPolicyEl {
            ttl: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubSubscriptionExpirationPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubSubscriptionExpirationPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataPubsubSubscriptionExpirationPolicyElRef {
        DataPubsubSubscriptionExpirationPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubSubscriptionExpirationPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}
#[derive(Serialize)]
pub struct DataPubsubSubscriptionMessageTransformsElAiInferenceElUnstructuredInferenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
}
impl DataPubsubSubscriptionMessageTransformsElAiInferenceElUnstructuredInferenceEl {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.parameters = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataPubsubSubscriptionMessageTransformsElAiInferenceElUnstructuredInferenceEl
{
    type O = BlockAssignable<
        DataPubsubSubscriptionMessageTransformsElAiInferenceElUnstructuredInferenceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubSubscriptionMessageTransformsElAiInferenceElUnstructuredInferenceEl {}
impl BuildDataPubsubSubscriptionMessageTransformsElAiInferenceElUnstructuredInferenceEl {
    pub fn build(
        self,
    ) -> DataPubsubSubscriptionMessageTransformsElAiInferenceElUnstructuredInferenceEl {
        DataPubsubSubscriptionMessageTransformsElAiInferenceElUnstructuredInferenceEl {
            parameters: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubSubscriptionMessageTransformsElAiInferenceElUnstructuredInferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubSubscriptionMessageTransformsElAiInferenceElUnstructuredInferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPubsubSubscriptionMessageTransformsElAiInferenceElUnstructuredInferenceElRef {
        DataPubsubSubscriptionMessageTransformsElAiInferenceElUnstructuredInferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubSubscriptionMessageTransformsElAiInferenceElUnstructuredInferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}
#[derive(Serialize)]
pub struct DataPubsubSubscriptionMessageTransformsElAiInferenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unstructured_inference: Option<
        ListField<DataPubsubSubscriptionMessageTransformsElAiInferenceElUnstructuredInferenceEl>,
    >,
}
impl DataPubsubSubscriptionMessageTransformsElAiInferenceEl {
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
        v: impl Into<
            ListField<
                DataPubsubSubscriptionMessageTransformsElAiInferenceElUnstructuredInferenceEl,
            >,
        >,
    ) -> Self {
        self.unstructured_inference = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubSubscriptionMessageTransformsElAiInferenceEl {
    type O = BlockAssignable<DataPubsubSubscriptionMessageTransformsElAiInferenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubSubscriptionMessageTransformsElAiInferenceEl {}
impl BuildDataPubsubSubscriptionMessageTransformsElAiInferenceEl {
    pub fn build(self) -> DataPubsubSubscriptionMessageTransformsElAiInferenceEl {
        DataPubsubSubscriptionMessageTransformsElAiInferenceEl {
            endpoint: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            unstructured_inference: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubSubscriptionMessageTransformsElAiInferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubSubscriptionMessageTransformsElAiInferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPubsubSubscriptionMessageTransformsElAiInferenceElRef {
        DataPubsubSubscriptionMessageTransformsElAiInferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubSubscriptionMessageTransformsElAiInferenceElRef {
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
    ) -> ListRef<DataPubsubSubscriptionMessageTransformsElAiInferenceElUnstructuredInferenceElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.unstructured_inference", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataPubsubSubscriptionMessageTransformsElJavascriptUdfEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_name: Option<PrimField<String>>,
}
impl DataPubsubSubscriptionMessageTransformsElJavascriptUdfEl {
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
impl ToListMappable for DataPubsubSubscriptionMessageTransformsElJavascriptUdfEl {
    type O = BlockAssignable<DataPubsubSubscriptionMessageTransformsElJavascriptUdfEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubSubscriptionMessageTransformsElJavascriptUdfEl {}
impl BuildDataPubsubSubscriptionMessageTransformsElJavascriptUdfEl {
    pub fn build(self) -> DataPubsubSubscriptionMessageTransformsElJavascriptUdfEl {
        DataPubsubSubscriptionMessageTransformsElJavascriptUdfEl {
            code: core::default::Default::default(),
            function_name: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubSubscriptionMessageTransformsElJavascriptUdfElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubSubscriptionMessageTransformsElJavascriptUdfElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPubsubSubscriptionMessageTransformsElJavascriptUdfElRef {
        DataPubsubSubscriptionMessageTransformsElJavascriptUdfElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubSubscriptionMessageTransformsElJavascriptUdfElRef {
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
pub struct DataPubsubSubscriptionMessageTransformsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ai_inference: Option<ListField<DataPubsubSubscriptionMessageTransformsElAiInferenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    javascript_udf: Option<ListField<DataPubsubSubscriptionMessageTransformsElJavascriptUdfEl>>,
}
impl DataPubsubSubscriptionMessageTransformsEl {
    #[doc = "Set the field `ai_inference`.\n"]
    pub fn set_ai_inference(
        mut self,
        v: impl Into<ListField<DataPubsubSubscriptionMessageTransformsElAiInferenceEl>>,
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
        v: impl Into<ListField<DataPubsubSubscriptionMessageTransformsElJavascriptUdfEl>>,
    ) -> Self {
        self.javascript_udf = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubSubscriptionMessageTransformsEl {
    type O = BlockAssignable<DataPubsubSubscriptionMessageTransformsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubSubscriptionMessageTransformsEl {}
impl BuildDataPubsubSubscriptionMessageTransformsEl {
    pub fn build(self) -> DataPubsubSubscriptionMessageTransformsEl {
        DataPubsubSubscriptionMessageTransformsEl {
            ai_inference: core::default::Default::default(),
            disabled: core::default::Default::default(),
            javascript_udf: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubSubscriptionMessageTransformsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubSubscriptionMessageTransformsElRef {
    fn new(shared: StackShared, base: String) -> DataPubsubSubscriptionMessageTransformsElRef {
        DataPubsubSubscriptionMessageTransformsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubSubscriptionMessageTransformsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ai_inference` after provisioning.\n"]
    pub fn ai_inference(
        &self,
    ) -> ListRef<DataPubsubSubscriptionMessageTransformsElAiInferenceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ai_inference", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `javascript_udf` after provisioning.\n"]
    pub fn javascript_udf(
        &self,
    ) -> ListRef<DataPubsubSubscriptionMessageTransformsElJavascriptUdfElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.javascript_udf", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataPubsubSubscriptionPushConfigElNoWrapperEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    write_metadata: Option<PrimField<bool>>,
}
impl DataPubsubSubscriptionPushConfigElNoWrapperEl {
    #[doc = "Set the field `write_metadata`.\n"]
    pub fn set_write_metadata(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.write_metadata = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubSubscriptionPushConfigElNoWrapperEl {
    type O = BlockAssignable<DataPubsubSubscriptionPushConfigElNoWrapperEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubSubscriptionPushConfigElNoWrapperEl {}
impl BuildDataPubsubSubscriptionPushConfigElNoWrapperEl {
    pub fn build(self) -> DataPubsubSubscriptionPushConfigElNoWrapperEl {
        DataPubsubSubscriptionPushConfigElNoWrapperEl {
            write_metadata: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubSubscriptionPushConfigElNoWrapperElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubSubscriptionPushConfigElNoWrapperElRef {
    fn new(shared: StackShared, base: String) -> DataPubsubSubscriptionPushConfigElNoWrapperElRef {
        DataPubsubSubscriptionPushConfigElNoWrapperElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubSubscriptionPushConfigElNoWrapperElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `write_metadata` after provisioning.\n"]
    pub fn write_metadata(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.write_metadata", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataPubsubSubscriptionPushConfigElOidcTokenEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audience: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
}
impl DataPubsubSubscriptionPushConfigElOidcTokenEl {
    #[doc = "Set the field `audience`.\n"]
    pub fn set_audience(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.audience = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_email`.\n"]
    pub fn set_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_email = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubSubscriptionPushConfigElOidcTokenEl {
    type O = BlockAssignable<DataPubsubSubscriptionPushConfigElOidcTokenEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubSubscriptionPushConfigElOidcTokenEl {}
impl BuildDataPubsubSubscriptionPushConfigElOidcTokenEl {
    pub fn build(self) -> DataPubsubSubscriptionPushConfigElOidcTokenEl {
        DataPubsubSubscriptionPushConfigElOidcTokenEl {
            audience: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubSubscriptionPushConfigElOidcTokenElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubSubscriptionPushConfigElOidcTokenElRef {
    fn new(shared: StackShared, base: String) -> DataPubsubSubscriptionPushConfigElOidcTokenElRef {
        DataPubsubSubscriptionPushConfigElOidcTokenElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubSubscriptionPushConfigElOidcTokenElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audience` after provisioning.\n"]
    pub fn audience(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.audience", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\n"]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataPubsubSubscriptionPushConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_wrapper: Option<ListField<DataPubsubSubscriptionPushConfigElNoWrapperEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oidc_token: Option<ListField<DataPubsubSubscriptionPushConfigElOidcTokenEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_endpoint: Option<PrimField<String>>,
}
impl DataPubsubSubscriptionPushConfigEl {
    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.attributes = Some(v.into());
        self
    }
    #[doc = "Set the field `no_wrapper`.\n"]
    pub fn set_no_wrapper(
        mut self,
        v: impl Into<ListField<DataPubsubSubscriptionPushConfigElNoWrapperEl>>,
    ) -> Self {
        self.no_wrapper = Some(v.into());
        self
    }
    #[doc = "Set the field `oidc_token`.\n"]
    pub fn set_oidc_token(
        mut self,
        v: impl Into<ListField<DataPubsubSubscriptionPushConfigElOidcTokenEl>>,
    ) -> Self {
        self.oidc_token = Some(v.into());
        self
    }
    #[doc = "Set the field `push_endpoint`.\n"]
    pub fn set_push_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.push_endpoint = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubSubscriptionPushConfigEl {
    type O = BlockAssignable<DataPubsubSubscriptionPushConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubSubscriptionPushConfigEl {}
impl BuildDataPubsubSubscriptionPushConfigEl {
    pub fn build(self) -> DataPubsubSubscriptionPushConfigEl {
        DataPubsubSubscriptionPushConfigEl {
            attributes: core::default::Default::default(),
            no_wrapper: core::default::Default::default(),
            oidc_token: core::default::Default::default(),
            push_endpoint: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubSubscriptionPushConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubSubscriptionPushConfigElRef {
    fn new(shared: StackShared, base: String) -> DataPubsubSubscriptionPushConfigElRef {
        DataPubsubSubscriptionPushConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubSubscriptionPushConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.attributes", self.base))
    }
    #[doc = "Get a reference to the value of field `no_wrapper` after provisioning.\n"]
    pub fn no_wrapper(&self) -> ListRef<DataPubsubSubscriptionPushConfigElNoWrapperElRef> {
        ListRef::new(self.shared().clone(), format!("{}.no_wrapper", self.base))
    }
    #[doc = "Get a reference to the value of field `oidc_token` after provisioning.\n"]
    pub fn oidc_token(&self) -> ListRef<DataPubsubSubscriptionPushConfigElOidcTokenElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oidc_token", self.base))
    }
    #[doc = "Get a reference to the value of field `push_endpoint` after provisioning.\n"]
    pub fn push_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.push_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataPubsubSubscriptionRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_backoff: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_backoff: Option<PrimField<String>>,
}
impl DataPubsubSubscriptionRetryPolicyEl {
    #[doc = "Set the field `maximum_backoff`.\n"]
    pub fn set_maximum_backoff(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.maximum_backoff = Some(v.into());
        self
    }
    #[doc = "Set the field `minimum_backoff`.\n"]
    pub fn set_minimum_backoff(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.minimum_backoff = Some(v.into());
        self
    }
}
impl ToListMappable for DataPubsubSubscriptionRetryPolicyEl {
    type O = BlockAssignable<DataPubsubSubscriptionRetryPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPubsubSubscriptionRetryPolicyEl {}
impl BuildDataPubsubSubscriptionRetryPolicyEl {
    pub fn build(self) -> DataPubsubSubscriptionRetryPolicyEl {
        DataPubsubSubscriptionRetryPolicyEl {
            maximum_backoff: core::default::Default::default(),
            minimum_backoff: core::default::Default::default(),
        }
    }
}
pub struct DataPubsubSubscriptionRetryPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPubsubSubscriptionRetryPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataPubsubSubscriptionRetryPolicyElRef {
        DataPubsubSubscriptionRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPubsubSubscriptionRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `maximum_backoff` after provisioning.\n"]
    pub fn maximum_backoff(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_backoff", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `minimum_backoff` after provisioning.\n"]
    pub fn minimum_backoff(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimum_backoff", self.base),
        )
    }
}
