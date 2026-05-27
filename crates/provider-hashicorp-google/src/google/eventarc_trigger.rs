use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct EventarcTriggerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_data_content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<Vec<EventarcTriggerDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matching_criteria: Option<Vec<EventarcTriggerMatchingCriteriaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<Vec<EventarcTriggerRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EventarcTriggerTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transport: Option<Vec<EventarcTriggerTransportEl>>,
    dynamic: EventarcTriggerDynamic,
}
struct EventarcTrigger_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EventarcTriggerData>,
}
#[derive(Clone)]
pub struct EventarcTrigger(Rc<EventarcTrigger_>);
impl EventarcTrigger {
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
    #[doc = "Set the field `channel`.\nOptional. The name of the channel associated with the trigger in 'projects/{project}/locations/{location}/channels/{channel}' format. You must provide a channel to receive events from Eventarc SaaS partners."]
    pub fn set_channel(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().channel = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `event_data_content_type`.\nOptional. EventDataContentType specifies the type of payload in MIME format that is expected from the CloudEvent data field. This is set to 'application/json' if the value is not defined."]
    pub fn set_event_data_content_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().event_data_content_type = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nOptional. User labels attached to the triggers that can be used to group resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nOptional. The IAM service account email associated with the trigger. The service account represents the identity of the trigger. The principal who calls this API must have 'iam.serviceAccounts.actAs' permission in the service account. See https://cloud.google.com/iam/docs/understanding-service-accounts#sa_common for more information. For Cloud Run destinations, this service account is used to generate identity tokens when invoking the service. See https://cloud.google.com/run/docs/triggering/pubsub-push#create-service-account for information on how to invoke authenticated Cloud Run services. In order to create Audit Log triggers, the service account should also have 'roles/eventarc.eventReceiver' IAM role."]
    pub fn set_service_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        self,
        v: impl Into<BlockAssignable<EventarcTriggerDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `matching_criteria`.\n"]
    pub fn set_matching_criteria(
        self,
        v: impl Into<BlockAssignable<EventarcTriggerMatchingCriteriaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().matching_criteria = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.matching_criteria = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        self,
        v: impl Into<BlockAssignable<EventarcTriggerRetryPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retry_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retry_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<EventarcTriggerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `transport`.\n"]
    pub fn set_transport(self, v: impl Into<BlockAssignable<EventarcTriggerTransportEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().transport = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.transport = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `channel` after provisioning.\nOptional. The name of the channel associated with the trigger in 'projects/{project}/locations/{location}/channels/{channel}' format. You must provide a channel to receive events from Eventarc SaaS partners."]
    pub fn channel(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.channel", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\nOutput only. The reason(s) why a trigger is in FAILED state."]
    pub fn conditions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation time."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nOutput only. This checksum is computed by the server based on the value of other fields, and may be sent only on create requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_data_content_type` after provisioning.\nOptional. EventDataContentType specifies the type of payload in MIME format that is expected from the CloudEvent data field. This is set to 'application/json' if the value is not defined."]
    pub fn event_data_content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_data_content_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. User labels attached to the triggers that can be used to group resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nRequired. The resource name of the trigger. Must be unique within the location on the project."]
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
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nOptional. The IAM service account email associated with the trigger. The service account represents the identity of the trigger. The principal who calls this API must have 'iam.serviceAccounts.actAs' permission in the service account. See https://cloud.google.com/iam/docs/understanding-service-accounts#sa_common for more information. For Cloud Run destinations, this service account is used to generate identity tokens when invoking the service. See https://cloud.google.com/run/docs/triggering/pubsub-push#create-service-account for information on how to invoke authenticated Cloud Run services. In order to create Audit Log triggers, the service account should also have 'roles/eventarc.eventReceiver' IAM role."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Server assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last-modified time."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> ListRef<EventarcTriggerDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<EventarcTriggerRetryPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EventarcTriggerTimeoutsElRef {
        EventarcTriggerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transport` after provisioning.\n"]
    pub fn transport(&self) -> ListRef<EventarcTriggerTransportElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transport", self.extract_ref()),
        )
    }
}
impl Referable for EventarcTrigger {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for EventarcTrigger {}
impl ToListMappable for EventarcTrigger {
    type O = ListRef<EventarcTriggerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for EventarcTrigger_ {
    fn extract_resource_type(&self) -> String {
        "google_eventarc_trigger".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildEventarcTrigger {
    pub tf_id: String,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
    #[doc = "Required. The resource name of the trigger. Must be unique within the location on the project."]
    pub name: PrimField<String>,
}
impl BuildEventarcTrigger {
    pub fn build(self, stack: &mut Stack) -> EventarcTrigger {
        let out = EventarcTrigger(Rc::new(EventarcTrigger_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EventarcTriggerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                channel: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                event_data_content_type: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                service_account: core::default::Default::default(),
                destination: core::default::Default::default(),
                matching_criteria: core::default::Default::default(),
                retry_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                transport: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct EventarcTriggerRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcTriggerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl EventarcTriggerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channel` after provisioning.\nOptional. The name of the channel associated with the trigger in 'projects/{project}/locations/{location}/channels/{channel}' format. You must provide a channel to receive events from Eventarc SaaS partners."]
    pub fn channel(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.channel", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\nOutput only. The reason(s) why a trigger is in FAILED state."]
    pub fn conditions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation time."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nOutput only. This checksum is computed by the server based on the value of other fields, and may be sent only on create requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_data_content_type` after provisioning.\nOptional. EventDataContentType specifies the type of payload in MIME format that is expected from the CloudEvent data field. This is set to 'application/json' if the value is not defined."]
    pub fn event_data_content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_data_content_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. User labels attached to the triggers that can be used to group resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nRequired. The resource name of the trigger. Must be unique within the location on the project."]
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
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nOptional. The IAM service account email associated with the trigger. The service account represents the identity of the trigger. The principal who calls this API must have 'iam.serviceAccounts.actAs' permission in the service account. See https://cloud.google.com/iam/docs/understanding-service-accounts#sa_common for more information. For Cloud Run destinations, this service account is used to generate identity tokens when invoking the service. See https://cloud.google.com/run/docs/triggering/pubsub-push#create-service-account for information on how to invoke authenticated Cloud Run services. In order to create Audit Log triggers, the service account should also have 'roles/eventarc.eventReceiver' IAM role."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Server assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last-modified time."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> ListRef<EventarcTriggerDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<EventarcTriggerRetryPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EventarcTriggerTimeoutsElRef {
        EventarcTriggerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transport` after provisioning.\n"]
    pub fn transport(&self) -> ListRef<EventarcTriggerTransportElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transport", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct EventarcTriggerDestinationElCloudRunServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    service: PrimField<String>,
}
impl EventarcTriggerDestinationElCloudRunServiceEl {
    #[doc = "Set the field `path`.\nOptional. The relative path on the Cloud Run service the events should be sent to. The value must conform to the definition of URI path segment (section 3.3 of RFC2396). Examples: \"/route\", \"route\", \"route/subroute\"."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRequired. The region the Cloud Run service is deployed in."]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}
impl ToListMappable for EventarcTriggerDestinationElCloudRunServiceEl {
    type O = BlockAssignable<EventarcTriggerDestinationElCloudRunServiceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcTriggerDestinationElCloudRunServiceEl {
    #[doc = "Required. The name of the Cloud Run service being addressed. See https://cloud.google.com/run/docs/reference/rest/v1/namespaces.services. Only services located in the same project of the trigger object can be addressed."]
    pub service: PrimField<String>,
}
impl BuildEventarcTriggerDestinationElCloudRunServiceEl {
    pub fn build(self) -> EventarcTriggerDestinationElCloudRunServiceEl {
        EventarcTriggerDestinationElCloudRunServiceEl {
            path: core::default::Default::default(),
            region: core::default::Default::default(),
            service: self.service,
        }
    }
}
pub struct EventarcTriggerDestinationElCloudRunServiceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcTriggerDestinationElCloudRunServiceElRef {
    fn new(shared: StackShared, base: String) -> EventarcTriggerDestinationElCloudRunServiceElRef {
        EventarcTriggerDestinationElCloudRunServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcTriggerDestinationElCloudRunServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nOptional. The relative path on the Cloud Run service the events should be sent to. The value must conform to the definition of URI path segment (section 3.3 of RFC2396). Examples: \"/route\", \"route\", \"route/subroute\"."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRequired. The region the Cloud Run service is deployed in."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nRequired. The name of the Cloud Run service being addressed. See https://cloud.google.com/run/docs/reference/rest/v1/namespaces.services. Only services located in the same project of the trigger object can be addressed."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct EventarcTriggerDestinationElGkeEl {
    cluster: PrimField<String>,
    location: PrimField<String>,
    namespace: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    service: PrimField<String>,
}
impl EventarcTriggerDestinationElGkeEl {
    #[doc = "Set the field `path`.\nOptional. The relative path on the GKE service the events should be sent to. The value must conform to the definition of a URI path segment (section 3.3 of RFC2396). Examples: \"/route\", \"route\", \"route/subroute\"."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}
impl ToListMappable for EventarcTriggerDestinationElGkeEl {
    type O = BlockAssignable<EventarcTriggerDestinationElGkeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcTriggerDestinationElGkeEl {
    #[doc = "Required. The name of the cluster the GKE service is running in. The cluster must be running in the same project as the trigger being created."]
    pub cluster: PrimField<String>,
    #[doc = "Required. The name of the Google Compute Engine in which the cluster resides, which can either be compute zone (for example, us-central1-a) for the zonal clusters or region (for example, us-central1) for regional clusters."]
    pub location: PrimField<String>,
    #[doc = "Required. The namespace the GKE service is running in."]
    pub namespace: PrimField<String>,
    #[doc = "Required. Name of the GKE service."]
    pub service: PrimField<String>,
}
impl BuildEventarcTriggerDestinationElGkeEl {
    pub fn build(self) -> EventarcTriggerDestinationElGkeEl {
        EventarcTriggerDestinationElGkeEl {
            cluster: self.cluster,
            location: self.location,
            namespace: self.namespace,
            path: core::default::Default::default(),
            service: self.service,
        }
    }
}
pub struct EventarcTriggerDestinationElGkeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcTriggerDestinationElGkeElRef {
    fn new(shared: StackShared, base: String) -> EventarcTriggerDestinationElGkeElRef {
        EventarcTriggerDestinationElGkeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcTriggerDestinationElGkeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nRequired. The name of the cluster the GKE service is running in. The cluster must be running in the same project as the trigger being created."]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nRequired. The name of the Google Compute Engine in which the cluster resides, which can either be compute zone (for example, us-central1-a) for the zonal clusters or region (for example, us-central1) for regional clusters."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nRequired. The namespace the GKE service is running in."]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nOptional. The relative path on the GKE service the events should be sent to. The value must conform to the definition of a URI path segment (section 3.3 of RFC2396). Examples: \"/route\", \"route\", \"route/subroute\"."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nRequired. Name of the GKE service."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct EventarcTriggerDestinationElHttpEndpointEl {
    uri: PrimField<String>,
}
impl EventarcTriggerDestinationElHttpEndpointEl {}
impl ToListMappable for EventarcTriggerDestinationElHttpEndpointEl {
    type O = BlockAssignable<EventarcTriggerDestinationElHttpEndpointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcTriggerDestinationElHttpEndpointEl {
    #[doc = "Required. The URI of the HTTP enpdoint. The value must be a RFC2396 URI string. Examples: 'http://10.10.10.8:80/route', 'http://svc.us-central1.p.local:8080/'. Only HTTP and HTTPS protocols are supported. The host can be either a static IP addressable from the VPC specified by the network config, or an internal DNS hostname of the service resolvable via Cloud DNS."]
    pub uri: PrimField<String>,
}
impl BuildEventarcTriggerDestinationElHttpEndpointEl {
    pub fn build(self) -> EventarcTriggerDestinationElHttpEndpointEl {
        EventarcTriggerDestinationElHttpEndpointEl { uri: self.uri }
    }
}
pub struct EventarcTriggerDestinationElHttpEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcTriggerDestinationElHttpEndpointElRef {
    fn new(shared: StackShared, base: String) -> EventarcTriggerDestinationElHttpEndpointElRef {
        EventarcTriggerDestinationElHttpEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcTriggerDestinationElHttpEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nRequired. The URI of the HTTP enpdoint. The value must be a RFC2396 URI string. Examples: 'http://10.10.10.8:80/route', 'http://svc.us-central1.p.local:8080/'. Only HTTP and HTTPS protocols are supported. The host can be either a static IP addressable from the VPC specified by the network config, or an internal DNS hostname of the service resolvable via Cloud DNS."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct EventarcTriggerDestinationElNetworkConfigEl {
    network_attachment: PrimField<String>,
}
impl EventarcTriggerDestinationElNetworkConfigEl {}
impl ToListMappable for EventarcTriggerDestinationElNetworkConfigEl {
    type O = BlockAssignable<EventarcTriggerDestinationElNetworkConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcTriggerDestinationElNetworkConfigEl {
    #[doc = "Required. Name of the NetworkAttachment that allows access to the destination VPC. Format: 'projects/{PROJECT_ID}/regions/{REGION}/networkAttachments/{NETWORK_ATTACHMENT_NAME}'"]
    pub network_attachment: PrimField<String>,
}
impl BuildEventarcTriggerDestinationElNetworkConfigEl {
    pub fn build(self) -> EventarcTriggerDestinationElNetworkConfigEl {
        EventarcTriggerDestinationElNetworkConfigEl {
            network_attachment: self.network_attachment,
        }
    }
}
pub struct EventarcTriggerDestinationElNetworkConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcTriggerDestinationElNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> EventarcTriggerDestinationElNetworkConfigElRef {
        EventarcTriggerDestinationElNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcTriggerDestinationElNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network_attachment` after provisioning.\nRequired. Name of the NetworkAttachment that allows access to the destination VPC. Format: 'projects/{PROJECT_ID}/regions/{REGION}/networkAttachments/{NETWORK_ATTACHMENT_NAME}'"]
    pub fn network_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_attachment", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct EventarcTriggerDestinationElDynamic {
    cloud_run_service: Option<DynamicBlock<EventarcTriggerDestinationElCloudRunServiceEl>>,
    gke: Option<DynamicBlock<EventarcTriggerDestinationElGkeEl>>,
    http_endpoint: Option<DynamicBlock<EventarcTriggerDestinationElHttpEndpointEl>>,
    network_config: Option<DynamicBlock<EventarcTriggerDestinationElNetworkConfigEl>>,
}
#[derive(Serialize)]
pub struct EventarcTriggerDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    workflow: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_run_service: Option<Vec<EventarcTriggerDestinationElCloudRunServiceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gke: Option<Vec<EventarcTriggerDestinationElGkeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_endpoint: Option<Vec<EventarcTriggerDestinationElHttpEndpointEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_config: Option<Vec<EventarcTriggerDestinationElNetworkConfigEl>>,
    dynamic: EventarcTriggerDestinationElDynamic,
}
impl EventarcTriggerDestinationEl {
    #[doc = "Set the field `workflow`.\nThe resource name of the Workflow whose Executions are triggered by the events. The Workflow resource should be deployed in the same project as the trigger. Format: 'projects/{project}/locations/{location}/workflows/{workflow}'"]
    pub fn set_workflow(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.workflow = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_run_service`.\n"]
    pub fn set_cloud_run_service(
        mut self,
        v: impl Into<BlockAssignable<EventarcTriggerDestinationElCloudRunServiceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_run_service = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_run_service = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gke`.\n"]
    pub fn set_gke(
        mut self,
        v: impl Into<BlockAssignable<EventarcTriggerDestinationElGkeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gke = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gke = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_endpoint`.\n"]
    pub fn set_http_endpoint(
        mut self,
        v: impl Into<BlockAssignable<EventarcTriggerDestinationElHttpEndpointEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_endpoint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_endpoint = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        mut self,
        v: impl Into<BlockAssignable<EventarcTriggerDestinationElNetworkConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for EventarcTriggerDestinationEl {
    type O = BlockAssignable<EventarcTriggerDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcTriggerDestinationEl {}
impl BuildEventarcTriggerDestinationEl {
    pub fn build(self) -> EventarcTriggerDestinationEl {
        EventarcTriggerDestinationEl {
            workflow: core::default::Default::default(),
            cloud_run_service: core::default::Default::default(),
            gke: core::default::Default::default(),
            http_endpoint: core::default::Default::default(),
            network_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EventarcTriggerDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcTriggerDestinationElRef {
    fn new(shared: StackShared, base: String) -> EventarcTriggerDestinationElRef {
        EventarcTriggerDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcTriggerDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_function` after provisioning.\nThe Cloud Function resource name. Only Cloud Functions V2 is supported. Format projects/{project}/locations/{location}/functions/{function} This is a read-only field. [WARNING] Creating Cloud Functions V2 triggers is only supported via the Cloud Functions product. An error will be returned if the user sets this value."]
    pub fn cloud_function(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_function", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `workflow` after provisioning.\nThe resource name of the Workflow whose Executions are triggered by the events. The Workflow resource should be deployed in the same project as the trigger. Format: 'projects/{project}/locations/{location}/workflows/{workflow}'"]
    pub fn workflow(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workflow", self.base))
    }
    #[doc = "Get a reference to the value of field `cloud_run_service` after provisioning.\n"]
    pub fn cloud_run_service(&self) -> ListRef<EventarcTriggerDestinationElCloudRunServiceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_run_service", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gke` after provisioning.\n"]
    pub fn gke(&self) -> ListRef<EventarcTriggerDestinationElGkeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gke", self.base))
    }
    #[doc = "Get a reference to the value of field `http_endpoint` after provisioning.\n"]
    pub fn http_endpoint(&self) -> ListRef<EventarcTriggerDestinationElHttpEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<EventarcTriggerDestinationElNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct EventarcTriggerMatchingCriteriaEl {
    attribute: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    value: PrimField<String>,
}
impl EventarcTriggerMatchingCriteriaEl {
    #[doc = "Set the field `operator`.\nOptional. The operator used for matching the events with the value of the filter. If not specified, only events that have an exact key-value pair specified in the filter are matched. The only allowed value is 'match-path-pattern'."]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }
}
impl ToListMappable for EventarcTriggerMatchingCriteriaEl {
    type O = BlockAssignable<EventarcTriggerMatchingCriteriaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcTriggerMatchingCriteriaEl {
    #[doc = "Required. The name of a CloudEvents attribute. Currently, only a subset of attributes are supported for filtering. All triggers MUST provide a filter for the 'type' attribute."]
    pub attribute: PrimField<String>,
    #[doc = "Required. The value for the attribute. See https://cloud.google.com/eventarc/docs/creating-triggers#trigger-gcloud for available values."]
    pub value: PrimField<String>,
}
impl BuildEventarcTriggerMatchingCriteriaEl {
    pub fn build(self) -> EventarcTriggerMatchingCriteriaEl {
        EventarcTriggerMatchingCriteriaEl {
            attribute: self.attribute,
            operator: core::default::Default::default(),
            value: self.value,
        }
    }
}
pub struct EventarcTriggerMatchingCriteriaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcTriggerMatchingCriteriaElRef {
    fn new(shared: StackShared, base: String) -> EventarcTriggerMatchingCriteriaElRef {
        EventarcTriggerMatchingCriteriaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcTriggerMatchingCriteriaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attribute` after provisioning.\nRequired. The name of a CloudEvents attribute. Currently, only a subset of attributes are supported for filtering. All triggers MUST provide a filter for the 'type' attribute."]
    pub fn attribute(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\nOptional. The operator used for matching the events with the value of the filter. If not specified, only events that have an exact key-value pair specified in the filter are matched. The only allowed value is 'match-path-pattern'."]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nRequired. The value for the attribute. See https://cloud.google.com/eventarc/docs/creating-triggers#trigger-gcloud for available values."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct EventarcTriggerRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_attempts: Option<PrimField<f64>>,
}
impl EventarcTriggerRetryPolicyEl {
    #[doc = "Set the field `max_attempts`.\nThe maximum number of delivery attempts for any message. The only valid\nvalue is 1."]
    pub fn set_max_attempts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_attempts = Some(v.into());
        self
    }
}
impl ToListMappable for EventarcTriggerRetryPolicyEl {
    type O = BlockAssignable<EventarcTriggerRetryPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcTriggerRetryPolicyEl {}
impl BuildEventarcTriggerRetryPolicyEl {
    pub fn build(self) -> EventarcTriggerRetryPolicyEl {
        EventarcTriggerRetryPolicyEl {
            max_attempts: core::default::Default::default(),
        }
    }
}
pub struct EventarcTriggerRetryPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcTriggerRetryPolicyElRef {
    fn new(shared: StackShared, base: String) -> EventarcTriggerRetryPolicyElRef {
        EventarcTriggerRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcTriggerRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_attempts` after provisioning.\nThe maximum number of delivery attempts for any message. The only valid\nvalue is 1."]
    pub fn max_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_attempts", self.base))
    }
}
#[derive(Serialize)]
pub struct EventarcTriggerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl EventarcTriggerTimeoutsEl {
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
impl ToListMappable for EventarcTriggerTimeoutsEl {
    type O = BlockAssignable<EventarcTriggerTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcTriggerTimeoutsEl {}
impl BuildEventarcTriggerTimeoutsEl {
    pub fn build(self) -> EventarcTriggerTimeoutsEl {
        EventarcTriggerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct EventarcTriggerTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcTriggerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EventarcTriggerTimeoutsElRef {
        EventarcTriggerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcTriggerTimeoutsElRef {
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
#[derive(Serialize)]
pub struct EventarcTriggerTransportElPubsubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<PrimField<String>>,
}
impl EventarcTriggerTransportElPubsubEl {
    #[doc = "Set the field `topic`.\nOptional. The name of the Pub/Sub topic created and managed by Eventarc system as a transport for the event delivery. Format: 'projects/{PROJECT_ID}/topics/{TOPIC_NAME}. You may set an existing topic for triggers of the type google.cloud.pubsub.topic.v1.messagePublished' only. The topic you provide here will not be deleted by Eventarc at trigger deletion."]
    pub fn set_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic = Some(v.into());
        self
    }
}
impl ToListMappable for EventarcTriggerTransportElPubsubEl {
    type O = BlockAssignable<EventarcTriggerTransportElPubsubEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcTriggerTransportElPubsubEl {}
impl BuildEventarcTriggerTransportElPubsubEl {
    pub fn build(self) -> EventarcTriggerTransportElPubsubEl {
        EventarcTriggerTransportElPubsubEl {
            topic: core::default::Default::default(),
        }
    }
}
pub struct EventarcTriggerTransportElPubsubElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcTriggerTransportElPubsubElRef {
    fn new(shared: StackShared, base: String) -> EventarcTriggerTransportElPubsubElRef {
        EventarcTriggerTransportElPubsubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcTriggerTransportElPubsubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `subscription` after provisioning.\nOutput only. The name of the Pub/Sub subscription created and managed by Eventarc system as a transport for the event delivery. Format: 'projects/{PROJECT_ID}/subscriptions/{SUBSCRIPTION_NAME}'."]
    pub fn subscription(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subscription", self.base))
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nOptional. The name of the Pub/Sub topic created and managed by Eventarc system as a transport for the event delivery. Format: 'projects/{PROJECT_ID}/topics/{TOPIC_NAME}. You may set an existing topic for triggers of the type google.cloud.pubsub.topic.v1.messagePublished' only. The topic you provide here will not be deleted by Eventarc at trigger deletion."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize, Default)]
struct EventarcTriggerTransportElDynamic {
    pubsub: Option<DynamicBlock<EventarcTriggerTransportElPubsubEl>>,
}
#[derive(Serialize)]
pub struct EventarcTriggerTransportEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pubsub: Option<Vec<EventarcTriggerTransportElPubsubEl>>,
    dynamic: EventarcTriggerTransportElDynamic,
}
impl EventarcTriggerTransportEl {
    #[doc = "Set the field `pubsub`.\n"]
    pub fn set_pubsub(
        mut self,
        v: impl Into<BlockAssignable<EventarcTriggerTransportElPubsubEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pubsub = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pubsub = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for EventarcTriggerTransportEl {
    type O = BlockAssignable<EventarcTriggerTransportEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcTriggerTransportEl {}
impl BuildEventarcTriggerTransportEl {
    pub fn build(self) -> EventarcTriggerTransportEl {
        EventarcTriggerTransportEl {
            pubsub: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EventarcTriggerTransportElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcTriggerTransportElRef {
    fn new(shared: StackShared, base: String) -> EventarcTriggerTransportElRef {
        EventarcTriggerTransportElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcTriggerTransportElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pubsub` after provisioning.\n"]
    pub fn pubsub(&self) -> ListRef<EventarcTriggerTransportElPubsubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pubsub", self.base))
    }
}
#[derive(Serialize, Default)]
struct EventarcTriggerDynamic {
    destination: Option<DynamicBlock<EventarcTriggerDestinationEl>>,
    matching_criteria: Option<DynamicBlock<EventarcTriggerMatchingCriteriaEl>>,
    retry_policy: Option<DynamicBlock<EventarcTriggerRetryPolicyEl>>,
    transport: Option<DynamicBlock<EventarcTriggerTransportEl>>,
}
