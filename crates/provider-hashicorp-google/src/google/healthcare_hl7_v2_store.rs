use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct HealthcareHl7V2StoreData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    dataset: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reject_duplicate_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_config: Option<Vec<HealthcareHl7V2StoreNotificationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_configs: Option<Vec<HealthcareHl7V2StoreNotificationConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parser_config: Option<Vec<HealthcareHl7V2StoreParserConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<HealthcareHl7V2StoreTimeoutsEl>,
    dynamic: HealthcareHl7V2StoreDynamic,
}
struct HealthcareHl7V2Store_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<HealthcareHl7V2StoreData>,
}
#[derive(Clone)]
pub struct HealthcareHl7V2Store(Rc<HealthcareHl7V2Store_>);
impl HealthcareHl7V2Store {
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
    #[doc = "Set the field `labels`.\nUser-supplied key-value pairs used to organize HL7v2 stores.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must\nconform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128\nbytes, and must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be associated with a given store.\n\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `reject_duplicate_message`.\nDetermines whether duplicate messages are allowed."]
    pub fn set_reject_duplicate_message(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().reject_duplicate_message = Some(v.into());
        self
    }
    #[doc = "Set the field `notification_config`.\n"]
    pub fn set_notification_config(
        self,
        v: impl Into<BlockAssignable<HealthcareHl7V2StoreNotificationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `notification_configs`.\n"]
    pub fn set_notification_configs(
        self,
        v: impl Into<BlockAssignable<HealthcareHl7V2StoreNotificationConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `parser_config`.\n"]
    pub fn set_parser_config(
        self,
        v: impl Into<BlockAssignable<HealthcareHl7V2StoreParserConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parser_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parser_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<HealthcareHl7V2StoreTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `dataset` after provisioning.\nIdentifies the dataset addressed by this request. Must be in the format\n'projects/{project}/locations/{location}/datasets/{dataset}'"]
    pub fn dataset(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-supplied key-value pairs used to organize HL7v2 stores.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must\nconform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128\nbytes, and must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be associated with a given store.\n\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the Hl7V2Store.\n\n** Changing this property may recreate the Hl7v2 store (removing all data) **"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reject_duplicate_message` after provisioning.\nDetermines whether duplicate messages are allowed."]
    pub fn reject_duplicate_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reject_duplicate_message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe fully qualified name of this dataset"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> ListRef<HealthcareHl7V2StoreNotificationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_configs` after provisioning.\n"]
    pub fn notification_configs(&self) -> ListRef<HealthcareHl7V2StoreNotificationConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parser_config` after provisioning.\n"]
    pub fn parser_config(&self) -> ListRef<HealthcareHl7V2StoreParserConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parser_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> HealthcareHl7V2StoreTimeoutsElRef {
        HealthcareHl7V2StoreTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for HealthcareHl7V2Store {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for HealthcareHl7V2Store {}
impl ToListMappable for HealthcareHl7V2Store {
    type O = ListRef<HealthcareHl7V2StoreRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for HealthcareHl7V2Store_ {
    fn extract_resource_type(&self) -> String {
        "google_healthcare_hl7_v2_store".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildHealthcareHl7V2Store {
    pub tf_id: String,
    #[doc = "Identifies the dataset addressed by this request. Must be in the format\n'projects/{project}/locations/{location}/datasets/{dataset}'"]
    pub dataset: PrimField<String>,
    #[doc = "The resource name for the Hl7V2Store.\n\n** Changing this property may recreate the Hl7v2 store (removing all data) **"]
    pub name: PrimField<String>,
}
impl BuildHealthcareHl7V2Store {
    pub fn build(self, stack: &mut Stack) -> HealthcareHl7V2Store {
        let out = HealthcareHl7V2Store(Rc::new(HealthcareHl7V2Store_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(HealthcareHl7V2StoreData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                dataset: self.dataset,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                reject_duplicate_message: core::default::Default::default(),
                notification_config: core::default::Default::default(),
                notification_configs: core::default::Default::default(),
                parser_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct HealthcareHl7V2StoreRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcareHl7V2StoreRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl HealthcareHl7V2StoreRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset` after provisioning.\nIdentifies the dataset addressed by this request. Must be in the format\n'projects/{project}/locations/{location}/datasets/{dataset}'"]
    pub fn dataset(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-supplied key-value pairs used to organize HL7v2 stores.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must\nconform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128\nbytes, and must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be associated with a given store.\n\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the Hl7V2Store.\n\n** Changing this property may recreate the Hl7v2 store (removing all data) **"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reject_duplicate_message` after provisioning.\nDetermines whether duplicate messages are allowed."]
    pub fn reject_duplicate_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reject_duplicate_message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe fully qualified name of this dataset"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> ListRef<HealthcareHl7V2StoreNotificationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_configs` after provisioning.\n"]
    pub fn notification_configs(&self) -> ListRef<HealthcareHl7V2StoreNotificationConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parser_config` after provisioning.\n"]
    pub fn parser_config(&self) -> ListRef<HealthcareHl7V2StoreParserConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parser_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> HealthcareHl7V2StoreTimeoutsElRef {
        HealthcareHl7V2StoreTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct HealthcareHl7V2StoreNotificationConfigEl {
    pubsub_topic: PrimField<String>,
}
impl HealthcareHl7V2StoreNotificationConfigEl {}
impl ToListMappable for HealthcareHl7V2StoreNotificationConfigEl {
    type O = BlockAssignable<HealthcareHl7V2StoreNotificationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcareHl7V2StoreNotificationConfigEl {
    #[doc = "The Cloud Pub/Sub topic that notifications of changes are published on. Supplied by the client.\nPubsubMessage.Data will contain the resource name. PubsubMessage.MessageId is the ID of this message.\nIt is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message\nwas published. Notifications are only sent if the topic is non-empty. Topic names must be scoped to a\nproject. service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com must have publisher permissions on the given\nCloud Pub/Sub topic. Not having adequate permissions will cause the calls that send notifications to fail."]
    pub pubsub_topic: PrimField<String>,
}
impl BuildHealthcareHl7V2StoreNotificationConfigEl {
    pub fn build(self) -> HealthcareHl7V2StoreNotificationConfigEl {
        HealthcareHl7V2StoreNotificationConfigEl {
            pubsub_topic: self.pubsub_topic,
        }
    }
}
pub struct HealthcareHl7V2StoreNotificationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcareHl7V2StoreNotificationConfigElRef {
    fn new(shared: StackShared, base: String) -> HealthcareHl7V2StoreNotificationConfigElRef {
        HealthcareHl7V2StoreNotificationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcareHl7V2StoreNotificationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pubsub_topic` after provisioning.\nThe Cloud Pub/Sub topic that notifications of changes are published on. Supplied by the client.\nPubsubMessage.Data will contain the resource name. PubsubMessage.MessageId is the ID of this message.\nIt is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message\nwas published. Notifications are only sent if the topic is non-empty. Topic names must be scoped to a\nproject. service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com must have publisher permissions on the given\nCloud Pub/Sub topic. Not having adequate permissions will cause the calls that send notifications to fail."]
    pub fn pubsub_topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pubsub_topic", self.base))
    }
}
#[derive(Serialize)]
pub struct HealthcareHl7V2StoreNotificationConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    pubsub_topic: PrimField<String>,
}
impl HealthcareHl7V2StoreNotificationConfigsEl {
    #[doc = "Set the field `filter`.\nRestricts notifications sent for messages matching a filter. If this is empty, all messages\nare matched. Syntax: https://cloud.google.com/appengine/docs/standard/python/search/query_strings\n\nFields/functions available for filtering are:\n\n* messageType, from the MSH-9.1 field. For example, NOT messageType = \"ADT\".\n* send_date or sendDate, the YYYY-MM-DD date the message was sent in the dataset's timeZone, from the MSH-7 segment. For example, send_date < \"2017-01-02\".\n* sendTime, the timestamp when the message was sent, using the RFC3339 time format for comparisons, from the MSH-7 segment. For example, sendTime < \"2017-01-02T00:00:00-05:00\".\n* sendFacility, the care center that the message came from, from the MSH-4 segment. For example, sendFacility = \"ABC\".\n* PatientId(value, type), which matches if the message lists a patient having an ID of the given value and type in the PID-2, PID-3, or PID-4 segments. For example, PatientId(\"123456\", \"MRN\").\n* labels.x, a string value of the label with key x as set using the Message.labels map. For example, labels.\"priority\"=\"high\". The operator :* can be used to assert the existence of a label. For example, labels.\"priority\":*."]
    pub fn set_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter = Some(v.into());
        self
    }
}
impl ToListMappable for HealthcareHl7V2StoreNotificationConfigsEl {
    type O = BlockAssignable<HealthcareHl7V2StoreNotificationConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcareHl7V2StoreNotificationConfigsEl {
    #[doc = "The Cloud Pub/Sub topic that notifications of changes are published on. Supplied by the client.\nPubsubMessage.Data will contain the resource name. PubsubMessage.MessageId is the ID of this message.\nIt is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message\nwas published. Notifications are only sent if the topic is non-empty. Topic names must be scoped to a\nproject. service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com must have publisher permissions on the given\nCloud Pub/Sub topic. Not having adequate permissions will cause the calls that send notifications to fail.\n\nIf a notification cannot be published to Cloud Pub/Sub, errors will be logged to Stackdriver"]
    pub pubsub_topic: PrimField<String>,
}
impl BuildHealthcareHl7V2StoreNotificationConfigsEl {
    pub fn build(self) -> HealthcareHl7V2StoreNotificationConfigsEl {
        HealthcareHl7V2StoreNotificationConfigsEl {
            filter: core::default::Default::default(),
            pubsub_topic: self.pubsub_topic,
        }
    }
}
pub struct HealthcareHl7V2StoreNotificationConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcareHl7V2StoreNotificationConfigsElRef {
    fn new(shared: StackShared, base: String) -> HealthcareHl7V2StoreNotificationConfigsElRef {
        HealthcareHl7V2StoreNotificationConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcareHl7V2StoreNotificationConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nRestricts notifications sent for messages matching a filter. If this is empty, all messages\nare matched. Syntax: https://cloud.google.com/appengine/docs/standard/python/search/query_strings\n\nFields/functions available for filtering are:\n\n* messageType, from the MSH-9.1 field. For example, NOT messageType = \"ADT\".\n* send_date or sendDate, the YYYY-MM-DD date the message was sent in the dataset's timeZone, from the MSH-7 segment. For example, send_date < \"2017-01-02\".\n* sendTime, the timestamp when the message was sent, using the RFC3339 time format for comparisons, from the MSH-7 segment. For example, sendTime < \"2017-01-02T00:00:00-05:00\".\n* sendFacility, the care center that the message came from, from the MSH-4 segment. For example, sendFacility = \"ABC\".\n* PatientId(value, type), which matches if the message lists a patient having an ID of the given value and type in the PID-2, PID-3, or PID-4 segments. For example, PatientId(\"123456\", \"MRN\").\n* labels.x, a string value of the label with key x as set using the Message.labels map. For example, labels.\"priority\"=\"high\". The operator :* can be used to assert the existence of a label. For example, labels.\"priority\":*."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `pubsub_topic` after provisioning.\nThe Cloud Pub/Sub topic that notifications of changes are published on. Supplied by the client.\nPubsubMessage.Data will contain the resource name. PubsubMessage.MessageId is the ID of this message.\nIt is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message\nwas published. Notifications are only sent if the topic is non-empty. Topic names must be scoped to a\nproject. service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com must have publisher permissions on the given\nCloud Pub/Sub topic. Not having adequate permissions will cause the calls that send notifications to fail.\n\nIf a notification cannot be published to Cloud Pub/Sub, errors will be logged to Stackdriver"]
    pub fn pubsub_topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pubsub_topic", self.base))
    }
}
#[derive(Serialize)]
pub struct HealthcareHl7V2StoreParserConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_null_header: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    segment_terminator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl HealthcareHl7V2StoreParserConfigEl {
    #[doc = "Set the field `allow_null_header`.\nDetermines whether messages with no header are allowed."]
    pub fn set_allow_null_header(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_null_header = Some(v.into());
        self
    }
    #[doc = "Set the field `schema`.\nJSON encoded string for schemas used to parse messages in this\nstore if schematized parsing is desired."]
    pub fn set_schema(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema = Some(v.into());
        self
    }
    #[doc = "Set the field `segment_terminator`.\nByte(s) to be used as the segment terminator. If this is unset, '\\r' will be used as segment terminator.\n\nA base64-encoded string."]
    pub fn set_segment_terminator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.segment_terminator = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nThe version of the unschematized parser to be used when a custom 'schema' is not set. Default value: \"V1\" Possible values: [\"V1\", \"V2\", \"V3\"]"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for HealthcareHl7V2StoreParserConfigEl {
    type O = BlockAssignable<HealthcareHl7V2StoreParserConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcareHl7V2StoreParserConfigEl {}
impl BuildHealthcareHl7V2StoreParserConfigEl {
    pub fn build(self) -> HealthcareHl7V2StoreParserConfigEl {
        HealthcareHl7V2StoreParserConfigEl {
            allow_null_header: core::default::Default::default(),
            schema: core::default::Default::default(),
            segment_terminator: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct HealthcareHl7V2StoreParserConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcareHl7V2StoreParserConfigElRef {
    fn new(shared: StackShared, base: String) -> HealthcareHl7V2StoreParserConfigElRef {
        HealthcareHl7V2StoreParserConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcareHl7V2StoreParserConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_null_header` after provisioning.\nDetermines whether messages with no header are allowed."]
    pub fn allow_null_header(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_null_header", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nJSON encoded string for schemas used to parse messages in this\nstore if schematized parsing is desired."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `segment_terminator` after provisioning.\nByte(s) to be used as the segment terminator. If this is unset, '\\r' will be used as segment terminator.\n\nA base64-encoded string."]
    pub fn segment_terminator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.segment_terminator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe version of the unschematized parser to be used when a custom 'schema' is not set. Default value: \"V1\" Possible values: [\"V1\", \"V2\", \"V3\"]"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct HealthcareHl7V2StoreTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl HealthcareHl7V2StoreTimeoutsEl {
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
impl ToListMappable for HealthcareHl7V2StoreTimeoutsEl {
    type O = BlockAssignable<HealthcareHl7V2StoreTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcareHl7V2StoreTimeoutsEl {}
impl BuildHealthcareHl7V2StoreTimeoutsEl {
    pub fn build(self) -> HealthcareHl7V2StoreTimeoutsEl {
        HealthcareHl7V2StoreTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct HealthcareHl7V2StoreTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcareHl7V2StoreTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> HealthcareHl7V2StoreTimeoutsElRef {
        HealthcareHl7V2StoreTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcareHl7V2StoreTimeoutsElRef {
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
struct HealthcareHl7V2StoreDynamic {
    notification_config: Option<DynamicBlock<HealthcareHl7V2StoreNotificationConfigEl>>,
    notification_configs: Option<DynamicBlock<HealthcareHl7V2StoreNotificationConfigsEl>>,
    parser_config: Option<DynamicBlock<HealthcareHl7V2StoreParserConfigEl>>,
}
