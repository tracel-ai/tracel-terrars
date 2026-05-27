use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct StorageNotificationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_attributes: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_name_prefix: Option<PrimField<String>>,
    payload_format: PrimField<String>,
    topic: PrimField<String>,
}
struct StorageNotification_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StorageNotificationData>,
}
#[derive(Clone)]
pub struct StorageNotification(Rc<StorageNotification_>);
impl StorageNotification {
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
    #[doc = "Set the field `custom_attributes`.\nA set of key/value attribute pairs to attach to each Cloud Pub/Sub message published for this notification subscription."]
    pub fn set_custom_attributes(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().custom_attributes = Some(v.into());
        self
    }
    #[doc = "Set the field `event_types`.\nList of event type filters for this notification config. If not specified, Cloud Storage will send notifications for all event types. The valid types are: \"OBJECT_FINALIZE\", \"OBJECT_METADATA_UPDATE\", \"OBJECT_DELETE\", \"OBJECT_ARCHIVE\""]
    pub fn set_event_types(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().event_types = Some(v.into());
        self
    }
    #[doc = "Set the field `object_name_prefix`.\nSpecifies a prefix path filter for this notification config. Cloud Storage will only send notifications for objects in this bucket whose names begin with the specified prefix."]
    pub fn set_object_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().object_name_prefix = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe name of the bucket."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_attributes` after provisioning.\nA set of key/value attribute pairs to attach to each Cloud Pub/Sub message published for this notification subscription."]
    pub fn custom_attributes(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.custom_attributes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_types` after provisioning.\nList of event type filters for this notification config. If not specified, Cloud Storage will send notifications for all event types. The valid types are: \"OBJECT_FINALIZE\", \"OBJECT_METADATA_UPDATE\", \"OBJECT_DELETE\", \"OBJECT_ARCHIVE\""]
    pub fn event_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.event_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `notification_id` after provisioning.\nThe ID of the created notification."]
    pub fn notification_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notification_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `object_name_prefix` after provisioning.\nSpecifies a prefix path filter for this notification config. Cloud Storage will only send notifications for objects in this bucket whose names begin with the specified prefix."]
    pub fn object_name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object_name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `payload_format` after provisioning.\nThe desired content of the Payload. One of \"JSON_API_V1\" or \"NONE\"."]
    pub fn payload_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.payload_format", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nThe Cloud Pub/Sub topic to which this subscription publishes."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.topic", self.extract_ref()),
        )
    }
}
impl Referable for StorageNotification {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for StorageNotification {}
impl ToListMappable for StorageNotification {
    type O = ListRef<StorageNotificationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for StorageNotification_ {
    fn extract_resource_type(&self) -> String {
        "google_storage_notification".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildStorageNotification {
    pub tf_id: String,
    #[doc = "The name of the bucket."]
    pub bucket: PrimField<String>,
    #[doc = "The desired content of the Payload. One of \"JSON_API_V1\" or \"NONE\"."]
    pub payload_format: PrimField<String>,
    #[doc = "The Cloud Pub/Sub topic to which this subscription publishes."]
    pub topic: PrimField<String>,
}
impl BuildStorageNotification {
    pub fn build(self, stack: &mut Stack) -> StorageNotification {
        let out = StorageNotification(Rc::new(StorageNotification_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(StorageNotificationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket: self.bucket,
                custom_attributes: core::default::Default::default(),
                event_types: core::default::Default::default(),
                object_name_prefix: core::default::Default::default(),
                payload_format: self.payload_format,
                topic: self.topic,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct StorageNotificationRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageNotificationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl StorageNotificationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe name of the bucket."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_attributes` after provisioning.\nA set of key/value attribute pairs to attach to each Cloud Pub/Sub message published for this notification subscription."]
    pub fn custom_attributes(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.custom_attributes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_types` after provisioning.\nList of event type filters for this notification config. If not specified, Cloud Storage will send notifications for all event types. The valid types are: \"OBJECT_FINALIZE\", \"OBJECT_METADATA_UPDATE\", \"OBJECT_DELETE\", \"OBJECT_ARCHIVE\""]
    pub fn event_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.event_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `notification_id` after provisioning.\nThe ID of the created notification."]
    pub fn notification_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notification_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `object_name_prefix` after provisioning.\nSpecifies a prefix path filter for this notification config. Cloud Storage will only send notifications for objects in this bucket whose names begin with the specified prefix."]
    pub fn object_name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object_name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `payload_format` after provisioning.\nThe desired content of the Payload. One of \"JSON_API_V1\" or \"NONE\"."]
    pub fn payload_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.payload_format", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nThe Cloud Pub/Sub topic to which this subscription publishes."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.topic", self.extract_ref()),
        )
    }
}
