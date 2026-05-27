use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct EventarcPipelineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crypto_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    pipeline_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destinations: Option<Vec<EventarcPipelineDestinationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_payload_format: Option<Vec<EventarcPipelineInputPayloadFormatEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_config: Option<Vec<EventarcPipelineLoggingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mediations: Option<Vec<EventarcPipelineMediationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<Vec<EventarcPipelineRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EventarcPipelineTimeoutsEl>,
    dynamic: EventarcPipelineDynamic,
}
struct EventarcPipeline_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EventarcPipelineData>,
}
#[derive(Clone)]
pub struct EventarcPipeline(Rc<EventarcPipeline_>);
impl EventarcPipeline {
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
    #[doc = "Set the field `annotations`.\nUser-defined annotations. See https://google.aip.dev/128#annotations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `crypto_key_name`.\nResource name of a KMS crypto key (managed by the user) used to\nencrypt/decrypt the event data. If not set, an internal Google-owned key\nwill be used to encrypt messages. It must match the pattern\n\"projects/{project}/locations/{location}/keyRings/{keyring}/cryptoKeys/{key}\"."]
    pub fn set_crypto_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().crypto_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nDisplay name of resource."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUser labels attached to the Pipeline that can be used to group\nresources. An object containing a list of \"key\": value pairs. Example: {\n\"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        self,
        v: impl Into<BlockAssignable<EventarcPipelineDestinationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destinations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destinations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `input_payload_format`.\n"]
    pub fn set_input_payload_format(
        self,
        v: impl Into<BlockAssignable<EventarcPipelineInputPayloadFormatEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_payload_format = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_payload_format = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        self,
        v: impl Into<BlockAssignable<EventarcPipelineLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mediations`.\n"]
    pub fn set_mediations(
        self,
        v: impl Into<BlockAssignable<EventarcPipelineMediationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mediations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mediations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        self,
        v: impl Into<BlockAssignable<EventarcPipelineRetryPolicyEl>>,
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
    pub fn set_timeouts(self, v: impl Into<EventarcPipelineTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUser-defined annotations. See https://google.aip.dev/128#annotations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe creation time.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up\nto nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crypto_key_name` after provisioning.\nResource name of a KMS crypto key (managed by the user) used to\nencrypt/decrypt the event data. If not set, an internal Google-owned key\nwill be used to encrypt messages. It must match the pattern\n\"projects/{project}/locations/{location}/keyRings/{keyring}/cryptoKeys/{key}\"."]
    pub fn crypto_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crypto_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of resource."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of\nother fields, and might be sent only on create requests to ensure that the\nclient has an up-to-date value before proceeding."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser labels attached to the Pipeline that can be used to group\nresources. An object containing a list of \"key\": value pairs. Example: {\n\"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Pipeline. Must be unique within the\nlocation of the project and must be in\n'projects/{project}/locations/{location}/pipelines/{pipeline}' format."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pipeline_id` after provisioning.\nThe user-provided ID to be assigned to the Pipeline. It should match the\nformat '^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$'."]
    pub fn pipeline_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nServer-assigned unique identifier for the Pipeline. The value\nis a UUID4 string and guaranteed to remain unchanged until the resource is\ndeleted."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe last-modified time.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up\nto nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> ListRef<EventarcPipelineDestinationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destinations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `input_payload_format` after provisioning.\n"]
    pub fn input_payload_format(&self) -> ListRef<EventarcPipelineInputPayloadFormatElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_payload_format", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<EventarcPipelineLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mediations` after provisioning.\n"]
    pub fn mediations(&self) -> ListRef<EventarcPipelineMediationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mediations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<EventarcPipelineRetryPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EventarcPipelineTimeoutsElRef {
        EventarcPipelineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for EventarcPipeline {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for EventarcPipeline {}
impl ToListMappable for EventarcPipeline {
    type O = ListRef<EventarcPipelineRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for EventarcPipeline_ {
    fn extract_resource_type(&self) -> String {
        "google_eventarc_pipeline".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildEventarcPipeline {
    pub tf_id: String,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "The user-provided ID to be assigned to the Pipeline. It should match the\nformat '^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$'."]
    pub pipeline_id: PrimField<String>,
}
impl BuildEventarcPipeline {
    pub fn build(self, stack: &mut Stack) -> EventarcPipeline {
        let out = EventarcPipeline(Rc::new(EventarcPipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EventarcPipelineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                crypto_key_name: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                pipeline_id: self.pipeline_id,
                project: core::default::Default::default(),
                destinations: core::default::Default::default(),
                input_payload_format: core::default::Default::default(),
                logging_config: core::default::Default::default(),
                mediations: core::default::Default::default(),
                retry_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct EventarcPipelineRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl EventarcPipelineRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUser-defined annotations. See https://google.aip.dev/128#annotations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe creation time.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up\nto nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crypto_key_name` after provisioning.\nResource name of a KMS crypto key (managed by the user) used to\nencrypt/decrypt the event data. If not set, an internal Google-owned key\nwill be used to encrypt messages. It must match the pattern\n\"projects/{project}/locations/{location}/keyRings/{keyring}/cryptoKeys/{key}\"."]
    pub fn crypto_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crypto_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of resource."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of\nother fields, and might be sent only on create requests to ensure that the\nclient has an up-to-date value before proceeding."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser labels attached to the Pipeline that can be used to group\nresources. An object containing a list of \"key\": value pairs. Example: {\n\"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Pipeline. Must be unique within the\nlocation of the project and must be in\n'projects/{project}/locations/{location}/pipelines/{pipeline}' format."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pipeline_id` after provisioning.\nThe user-provided ID to be assigned to the Pipeline. It should match the\nformat '^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$'."]
    pub fn pipeline_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nServer-assigned unique identifier for the Pipeline. The value\nis a UUID4 string and guaranteed to remain unchanged until the resource is\ndeleted."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe last-modified time.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up\nto nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> ListRef<EventarcPipelineDestinationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destinations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `input_payload_format` after provisioning.\n"]
    pub fn input_payload_format(&self) -> ListRef<EventarcPipelineInputPayloadFormatElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_payload_format", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<EventarcPipelineLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mediations` after provisioning.\n"]
    pub fn mediations(&self) -> ListRef<EventarcPipelineMediationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mediations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<EventarcPipelineRetryPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EventarcPipelineTimeoutsElRef {
        EventarcPipelineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct EventarcPipelineDestinationsElAuthenticationConfigElGoogleOidcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audience: Option<PrimField<String>>,
    service_account: PrimField<String>,
}
impl EventarcPipelineDestinationsElAuthenticationConfigElGoogleOidcEl {
    #[doc = "Set the field `audience`.\nAudience to be used to generate the OIDC Token. The audience claim\nidentifies the recipient that the JWT is intended for. If\nunspecified, the destination URI will be used."]
    pub fn set_audience(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.audience = Some(v.into());
        self
    }
}
impl ToListMappable for EventarcPipelineDestinationsElAuthenticationConfigElGoogleOidcEl {
    type O = BlockAssignable<EventarcPipelineDestinationsElAuthenticationConfigElGoogleOidcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineDestinationsElAuthenticationConfigElGoogleOidcEl {
    #[doc = "Service account email used to generate the OIDC Token.\nThe principal who calls this API must have\niam.serviceAccounts.actAs permission in the service account. See\nhttps://cloud.google.com/iam/docs/understanding-service-accounts\nfor more information. Eventarc service agents must have\nroles/roles/iam.serviceAccountTokenCreator role to allow the\nPipeline to create OpenID tokens for authenticated requests."]
    pub service_account: PrimField<String>,
}
impl BuildEventarcPipelineDestinationsElAuthenticationConfigElGoogleOidcEl {
    pub fn build(self) -> EventarcPipelineDestinationsElAuthenticationConfigElGoogleOidcEl {
        EventarcPipelineDestinationsElAuthenticationConfigElGoogleOidcEl {
            audience: core::default::Default::default(),
            service_account: self.service_account,
        }
    }
}
pub struct EventarcPipelineDestinationsElAuthenticationConfigElGoogleOidcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineDestinationsElAuthenticationConfigElGoogleOidcElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EventarcPipelineDestinationsElAuthenticationConfigElGoogleOidcElRef {
        EventarcPipelineDestinationsElAuthenticationConfigElGoogleOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineDestinationsElAuthenticationConfigElGoogleOidcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audience` after provisioning.\nAudience to be used to generate the OIDC Token. The audience claim\nidentifies the recipient that the JWT is intended for. If\nunspecified, the destination URI will be used."]
    pub fn audience(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.audience", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nService account email used to generate the OIDC Token.\nThe principal who calls this API must have\niam.serviceAccounts.actAs permission in the service account. See\nhttps://cloud.google.com/iam/docs/understanding-service-accounts\nfor more information. Eventarc service agents must have\nroles/roles/iam.serviceAccountTokenCreator role to allow the\nPipeline to create OpenID tokens for authenticated requests."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct EventarcPipelineDestinationsElAuthenticationConfigElOauthTokenEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    service_account: PrimField<String>,
}
impl EventarcPipelineDestinationsElAuthenticationConfigElOauthTokenEl {
    #[doc = "Set the field `scope`.\nOAuth scope to be used for generating OAuth access token. If not\nspecified, \"https://www.googleapis.com/auth/cloud-platform\" will be\nused."]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }
}
impl ToListMappable for EventarcPipelineDestinationsElAuthenticationConfigElOauthTokenEl {
    type O = BlockAssignable<EventarcPipelineDestinationsElAuthenticationConfigElOauthTokenEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineDestinationsElAuthenticationConfigElOauthTokenEl {
    #[doc = "Service account email used to generate the [OAuth\ntoken](https://developers.google.com/identity/protocols/OAuth2).\nThe principal who calls this API must have\niam.serviceAccounts.actAs permission in the service account. See\nhttps://cloud.google.com/iam/docs/understanding-service-accounts\nfor more information. Eventarc service agents must have\nroles/roles/iam.serviceAccountTokenCreator role to allow Pipeline\nto create OAuth2 tokens for authenticated requests."]
    pub service_account: PrimField<String>,
}
impl BuildEventarcPipelineDestinationsElAuthenticationConfigElOauthTokenEl {
    pub fn build(self) -> EventarcPipelineDestinationsElAuthenticationConfigElOauthTokenEl {
        EventarcPipelineDestinationsElAuthenticationConfigElOauthTokenEl {
            scope: core::default::Default::default(),
            service_account: self.service_account,
        }
    }
}
pub struct EventarcPipelineDestinationsElAuthenticationConfigElOauthTokenElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineDestinationsElAuthenticationConfigElOauthTokenElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EventarcPipelineDestinationsElAuthenticationConfigElOauthTokenElRef {
        EventarcPipelineDestinationsElAuthenticationConfigElOauthTokenElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineDestinationsElAuthenticationConfigElOauthTokenElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\nOAuth scope to be used for generating OAuth access token. If not\nspecified, \"https://www.googleapis.com/auth/cloud-platform\" will be\nused."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nService account email used to generate the [OAuth\ntoken](https://developers.google.com/identity/protocols/OAuth2).\nThe principal who calls this API must have\niam.serviceAccounts.actAs permission in the service account. See\nhttps://cloud.google.com/iam/docs/understanding-service-accounts\nfor more information. Eventarc service agents must have\nroles/roles/iam.serviceAccountTokenCreator role to allow Pipeline\nto create OAuth2 tokens for authenticated requests."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct EventarcPipelineDestinationsElAuthenticationConfigElDynamic {
    google_oidc:
        Option<DynamicBlock<EventarcPipelineDestinationsElAuthenticationConfigElGoogleOidcEl>>,
    oauth_token:
        Option<DynamicBlock<EventarcPipelineDestinationsElAuthenticationConfigElOauthTokenEl>>,
}
#[derive(Serialize)]
pub struct EventarcPipelineDestinationsElAuthenticationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    google_oidc: Option<Vec<EventarcPipelineDestinationsElAuthenticationConfigElGoogleOidcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_token: Option<Vec<EventarcPipelineDestinationsElAuthenticationConfigElOauthTokenEl>>,
    dynamic: EventarcPipelineDestinationsElAuthenticationConfigElDynamic,
}
impl EventarcPipelineDestinationsElAuthenticationConfigEl {
    #[doc = "Set the field `google_oidc`.\n"]
    pub fn set_google_oidc(
        mut self,
        v: impl Into<BlockAssignable<EventarcPipelineDestinationsElAuthenticationConfigElGoogleOidcEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.google_oidc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.google_oidc = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oauth_token`.\n"]
    pub fn set_oauth_token(
        mut self,
        v: impl Into<BlockAssignable<EventarcPipelineDestinationsElAuthenticationConfigElOauthTokenEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth_token = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_token = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for EventarcPipelineDestinationsElAuthenticationConfigEl {
    type O = BlockAssignable<EventarcPipelineDestinationsElAuthenticationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineDestinationsElAuthenticationConfigEl {}
impl BuildEventarcPipelineDestinationsElAuthenticationConfigEl {
    pub fn build(self) -> EventarcPipelineDestinationsElAuthenticationConfigEl {
        EventarcPipelineDestinationsElAuthenticationConfigEl {
            google_oidc: core::default::Default::default(),
            oauth_token: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EventarcPipelineDestinationsElAuthenticationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineDestinationsElAuthenticationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EventarcPipelineDestinationsElAuthenticationConfigElRef {
        EventarcPipelineDestinationsElAuthenticationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineDestinationsElAuthenticationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `google_oidc` after provisioning.\n"]
    pub fn google_oidc(
        &self,
    ) -> ListRef<EventarcPipelineDestinationsElAuthenticationConfigElGoogleOidcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.google_oidc", self.base))
    }
    #[doc = "Get a reference to the value of field `oauth_token` after provisioning.\n"]
    pub fn oauth_token(
        &self,
    ) -> ListRef<EventarcPipelineDestinationsElAuthenticationConfigElOauthTokenElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oauth_token", self.base))
    }
}
#[derive(Serialize)]
pub struct EventarcPipelineDestinationsElHttpEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_binding_template: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl EventarcPipelineDestinationsElHttpEndpointEl {
    #[doc = "Set the field `message_binding_template`.\nThe CEL expression used to modify how the destination-bound HTTP\nrequest is constructed.\n\nIf a binding expression is not specified here, the message\nis treated as a CloudEvent and is mapped to the HTTP request according\nto the CloudEvent HTTP Protocol Binding Binary Content Mode\n(https://github.com/cloudevents/spec/blob/main/cloudevents/bindings/http-protocol-binding.md#31-binary-content-mode).\nIn this representation, all fields except the 'data' and\n'datacontenttype' field on the message are mapped to HTTP request\nheaders with a prefix of 'ce-'.\n\nTo construct the HTTP request payload and the value of the content-type\nHTTP header, the payload format is defined as follows:\n1) Use the output_payload_format_type on the Pipeline.Destination if it\nis set, else:\n2) Use the input_payload_format_type on the Pipeline if it is set,\nelse:\n3) Treat the payload as opaque binary data.\n\nThe 'data' field of the message is converted to the payload format or\nleft as-is for case 3) and then attached as the payload of the HTTP\nrequest. The 'content-type' header on the HTTP request is set to the\npayload format type or left empty for case 3). However, if a mediation\nhas updated the 'datacontenttype' field on the message so that it is\nnot the same as the payload format type but it is still a prefix of the\npayload format type, then the 'content-type' header on the HTTP request\nis set to this 'datacontenttype' value. For example, if the\n'datacontenttype' is \"application/json\" and the payload format type is\n\"application/json; charset=utf-8\", then the 'content-type' header on\nthe HTTP request is set to \"application/json; charset=utf-8\".\n\nIf a non-empty binding expression is specified then this expression is\nused to modify the default CloudEvent HTTP Protocol Binding Binary\nContent representation.\nThe result of the CEL expression must be a map of key/value pairs\nwhich is used as follows:\n- If a map named 'headers' exists on the result of the expression,\nthen its key/value pairs are directly mapped to the HTTP request\nheaders. The headers values are constructed from the corresponding\nvalue type's canonical representation. If the 'headers' field doesn't\nexist then the resulting HTTP request will be the headers of the\nCloudEvent HTTP Binding Binary Content Mode representation of the final\nmessage. Note: If the specified binding expression, has updated the\n'datacontenttype' field on the message so that it is not the same as\nthe payload format type but it is still a prefix of the payload format\ntype, then the 'content-type' header in the 'headers' map is set to\nthis 'datacontenttype' value.\n- If a field named 'body' exists on the result of the expression then\nits value is directly mapped to the body of the request. If the value\nof the 'body' field is of type bytes or string then it is used for\nthe HTTP request body as-is, with no conversion. If the body field is\nof any other type then it is converted to a JSON string. If the body\nfield does not exist then the resulting payload of the HTTP request\nwill be data value of the CloudEvent HTTP Binding Binary Content Mode\nrepresentation of the final message as described earlier.\n- Any other fields in the resulting expression will be ignored.\n\nThe CEL expression may access the incoming CloudEvent message in its\ndefinition, as follows:\n- The 'data' field of the incoming CloudEvent message can be accessed\nusing the 'message.data' value. Subfields of 'message.data' may also be\naccessed if an input_payload_format has been specified on the Pipeline.\n- Each attribute of the incoming CloudEvent message can be accessed\nusing the 'message.' value, where  is replaced with the\nname of the attribute.\n- Existing headers can be accessed in the CEL expression using the\n'headers' variable. The 'headers' variable defines a map of key/value\npairs corresponding to the HTTP headers of the CloudEvent HTTP Binding\nBinary Content Mode representation of the final message as described\nearlier. For example, the following CEL expression can be used to\nconstruct an HTTP request by adding an additional header to the HTTP\nheaders of the CloudEvent HTTP Binding Binary Content Mode\nrepresentation of the final message and by overwriting the body of the\nrequest:\n\n'''\n{\n\"headers\": headers.merge({\"new-header-key\": \"new-header-value\"}),\n\"body\": \"new-body\"\n}\n'''\n- The default binding for the message payload can be accessed using the\n'body' variable. It conatins a string representation of the message\npayload in the format specified by the 'output_payload_format' field.\nIf the 'input_payload_format' field is not set, the 'body'\nvariable contains the same message payload bytes that were published.\n\nAdditionally, the following CEL extension functions are provided for\nuse in this CEL expression:\n- toBase64Url:\nmap.toBase64Url() -> string\n- Converts a CelValue to a base64url encoded string\n- toJsonString: map.toJsonString() -> string\n- Converts a CelValue to a JSON string\n- merge:\nmap1.merge(map2) -> map3\n- Merges the passed CEL map with the existing CEL map the\nfunction is applied to.\n- If the same key exists in both maps, if the key's value is type\nmap both maps are merged else the value from the passed map is\nused.\n- denormalize:\nmap.denormalize() -> map\n- Denormalizes a CEL map such that every value of type map or key\nin the map is expanded to return a single level map.\n- The resulting keys are \".\" separated indices of the map keys.\n- For example:\n{\n\"a\": 1,\n\"b\": {\n\"c\": 2,\n\"d\": 3\n}\n\"e\": [4, 5]\n}\n.denormalize()\n-> {\n\"a\": 1,\n\"b.c\": 2,\n\"b.d\": 3,\n\"e.0\": 4,\n\"e.1\": 5\n}\n- setField:\nmap.setField(key, value) -> message\n- Sets the field of the message with the given key to the\ngiven value.\n- If the field is not present it will be added.\n- If the field is present it will be overwritten.\n- The key can be a dot separated path to set a field in a nested\nmessage.\n- Key must be of type string.\n- Value may be any valid type.\n- removeFields:\nmap.removeFields([key1, key2, ...]) -> message\n- Removes the fields of the map with the given keys.\n- The keys can be a dot separated path to remove a field in a\nnested message.\n- If a key is not found it will be ignored.\n- Keys must be of type string.\n- toMap:\n[map1, map2, ...].toMap() -> map\n- Converts a CEL list of CEL maps to a single CEL map\n- toCloudEventJsonWithPayloadFormat:\nmessage.toCloudEventJsonWithPayloadFormat() -> map\n- Converts a message to the corresponding structure of JSON\nformat for CloudEvents.\n- It converts 'data' to destination payload format\nspecified in 'output_payload_format'. If 'output_payload_format' is\nnot set, the data will remain unchanged.\n- It also sets the corresponding datacontenttype of\nthe CloudEvent, as indicated by\n'output_payload_format'. If no\n'output_payload_format' is set it will use the value of the\n\"datacontenttype\" attribute on the CloudEvent if present, else\nremove \"datacontenttype\" attribute.\n- This function expects that the content of the message will\nadhere to the standard CloudEvent format. If it doesn't then this\nfunction will fail.\n- The result is a CEL map that corresponds to the JSON\nrepresentation of the CloudEvent. To convert that data to a JSON\nstring it can be chained with the toJsonString function.\n\nThe Pipeline expects that the message it receives adheres to the\nstandard CloudEvent format. If it doesn't then the outgoing message\nrequest may fail with a persistent error."]
    pub fn set_message_binding_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_binding_template = Some(v.into());
        self
    }
}
impl ToListMappable for EventarcPipelineDestinationsElHttpEndpointEl {
    type O = BlockAssignable<EventarcPipelineDestinationsElHttpEndpointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineDestinationsElHttpEndpointEl {
    #[doc = "The URI of the HTTP enpdoint.\n\nThe value must be a RFC2396 URI string.\nExamples: 'https://svc.us-central1.p.local:8080/route'.\nOnly the HTTPS protocol is supported."]
    pub uri: PrimField<String>,
}
impl BuildEventarcPipelineDestinationsElHttpEndpointEl {
    pub fn build(self) -> EventarcPipelineDestinationsElHttpEndpointEl {
        EventarcPipelineDestinationsElHttpEndpointEl {
            message_binding_template: core::default::Default::default(),
            uri: self.uri,
        }
    }
}
pub struct EventarcPipelineDestinationsElHttpEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineDestinationsElHttpEndpointElRef {
    fn new(shared: StackShared, base: String) -> EventarcPipelineDestinationsElHttpEndpointElRef {
        EventarcPipelineDestinationsElHttpEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineDestinationsElHttpEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message_binding_template` after provisioning.\nThe CEL expression used to modify how the destination-bound HTTP\nrequest is constructed.\n\nIf a binding expression is not specified here, the message\nis treated as a CloudEvent and is mapped to the HTTP request according\nto the CloudEvent HTTP Protocol Binding Binary Content Mode\n(https://github.com/cloudevents/spec/blob/main/cloudevents/bindings/http-protocol-binding.md#31-binary-content-mode).\nIn this representation, all fields except the 'data' and\n'datacontenttype' field on the message are mapped to HTTP request\nheaders with a prefix of 'ce-'.\n\nTo construct the HTTP request payload and the value of the content-type\nHTTP header, the payload format is defined as follows:\n1) Use the output_payload_format_type on the Pipeline.Destination if it\nis set, else:\n2) Use the input_payload_format_type on the Pipeline if it is set,\nelse:\n3) Treat the payload as opaque binary data.\n\nThe 'data' field of the message is converted to the payload format or\nleft as-is for case 3) and then attached as the payload of the HTTP\nrequest. The 'content-type' header on the HTTP request is set to the\npayload format type or left empty for case 3). However, if a mediation\nhas updated the 'datacontenttype' field on the message so that it is\nnot the same as the payload format type but it is still a prefix of the\npayload format type, then the 'content-type' header on the HTTP request\nis set to this 'datacontenttype' value. For example, if the\n'datacontenttype' is \"application/json\" and the payload format type is\n\"application/json; charset=utf-8\", then the 'content-type' header on\nthe HTTP request is set to \"application/json; charset=utf-8\".\n\nIf a non-empty binding expression is specified then this expression is\nused to modify the default CloudEvent HTTP Protocol Binding Binary\nContent representation.\nThe result of the CEL expression must be a map of key/value pairs\nwhich is used as follows:\n- If a map named 'headers' exists on the result of the expression,\nthen its key/value pairs are directly mapped to the HTTP request\nheaders. The headers values are constructed from the corresponding\nvalue type's canonical representation. If the 'headers' field doesn't\nexist then the resulting HTTP request will be the headers of the\nCloudEvent HTTP Binding Binary Content Mode representation of the final\nmessage. Note: If the specified binding expression, has updated the\n'datacontenttype' field on the message so that it is not the same as\nthe payload format type but it is still a prefix of the payload format\ntype, then the 'content-type' header in the 'headers' map is set to\nthis 'datacontenttype' value.\n- If a field named 'body' exists on the result of the expression then\nits value is directly mapped to the body of the request. If the value\nof the 'body' field is of type bytes or string then it is used for\nthe HTTP request body as-is, with no conversion. If the body field is\nof any other type then it is converted to a JSON string. If the body\nfield does not exist then the resulting payload of the HTTP request\nwill be data value of the CloudEvent HTTP Binding Binary Content Mode\nrepresentation of the final message as described earlier.\n- Any other fields in the resulting expression will be ignored.\n\nThe CEL expression may access the incoming CloudEvent message in its\ndefinition, as follows:\n- The 'data' field of the incoming CloudEvent message can be accessed\nusing the 'message.data' value. Subfields of 'message.data' may also be\naccessed if an input_payload_format has been specified on the Pipeline.\n- Each attribute of the incoming CloudEvent message can be accessed\nusing the 'message.' value, where  is replaced with the\nname of the attribute.\n- Existing headers can be accessed in the CEL expression using the\n'headers' variable. The 'headers' variable defines a map of key/value\npairs corresponding to the HTTP headers of the CloudEvent HTTP Binding\nBinary Content Mode representation of the final message as described\nearlier. For example, the following CEL expression can be used to\nconstruct an HTTP request by adding an additional header to the HTTP\nheaders of the CloudEvent HTTP Binding Binary Content Mode\nrepresentation of the final message and by overwriting the body of the\nrequest:\n\n'''\n{\n\"headers\": headers.merge({\"new-header-key\": \"new-header-value\"}),\n\"body\": \"new-body\"\n}\n'''\n- The default binding for the message payload can be accessed using the\n'body' variable. It conatins a string representation of the message\npayload in the format specified by the 'output_payload_format' field.\nIf the 'input_payload_format' field is not set, the 'body'\nvariable contains the same message payload bytes that were published.\n\nAdditionally, the following CEL extension functions are provided for\nuse in this CEL expression:\n- toBase64Url:\nmap.toBase64Url() -> string\n- Converts a CelValue to a base64url encoded string\n- toJsonString: map.toJsonString() -> string\n- Converts a CelValue to a JSON string\n- merge:\nmap1.merge(map2) -> map3\n- Merges the passed CEL map with the existing CEL map the\nfunction is applied to.\n- If the same key exists in both maps, if the key's value is type\nmap both maps are merged else the value from the passed map is\nused.\n- denormalize:\nmap.denormalize() -> map\n- Denormalizes a CEL map such that every value of type map or key\nin the map is expanded to return a single level map.\n- The resulting keys are \".\" separated indices of the map keys.\n- For example:\n{\n\"a\": 1,\n\"b\": {\n\"c\": 2,\n\"d\": 3\n}\n\"e\": [4, 5]\n}\n.denormalize()\n-> {\n\"a\": 1,\n\"b.c\": 2,\n\"b.d\": 3,\n\"e.0\": 4,\n\"e.1\": 5\n}\n- setField:\nmap.setField(key, value) -> message\n- Sets the field of the message with the given key to the\ngiven value.\n- If the field is not present it will be added.\n- If the field is present it will be overwritten.\n- The key can be a dot separated path to set a field in a nested\nmessage.\n- Key must be of type string.\n- Value may be any valid type.\n- removeFields:\nmap.removeFields([key1, key2, ...]) -> message\n- Removes the fields of the map with the given keys.\n- The keys can be a dot separated path to remove a field in a\nnested message.\n- If a key is not found it will be ignored.\n- Keys must be of type string.\n- toMap:\n[map1, map2, ...].toMap() -> map\n- Converts a CEL list of CEL maps to a single CEL map\n- toCloudEventJsonWithPayloadFormat:\nmessage.toCloudEventJsonWithPayloadFormat() -> map\n- Converts a message to the corresponding structure of JSON\nformat for CloudEvents.\n- It converts 'data' to destination payload format\nspecified in 'output_payload_format'. If 'output_payload_format' is\nnot set, the data will remain unchanged.\n- It also sets the corresponding datacontenttype of\nthe CloudEvent, as indicated by\n'output_payload_format'. If no\n'output_payload_format' is set it will use the value of the\n\"datacontenttype\" attribute on the CloudEvent if present, else\nremove \"datacontenttype\" attribute.\n- This function expects that the content of the message will\nadhere to the standard CloudEvent format. If it doesn't then this\nfunction will fail.\n- The result is a CEL map that corresponds to the JSON\nrepresentation of the CloudEvent. To convert that data to a JSON\nstring it can be chained with the toJsonString function.\n\nThe Pipeline expects that the message it receives adheres to the\nstandard CloudEvent format. If it doesn't then the outgoing message\nrequest may fail with a persistent error."]
    pub fn message_binding_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_binding_template", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe URI of the HTTP enpdoint.\n\nThe value must be a RFC2396 URI string.\nExamples: 'https://svc.us-central1.p.local:8080/route'.\nOnly the HTTPS protocol is supported."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct EventarcPipelineDestinationsElNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network_attachment: Option<PrimField<String>>,
}
impl EventarcPipelineDestinationsElNetworkConfigEl {
    #[doc = "Set the field `network_attachment`.\nName of the NetworkAttachment that allows access to the consumer VPC.\n\nFormat:\n'projects/{PROJECT_ID}/regions/{REGION}/networkAttachments/{NETWORK_ATTACHMENT_NAME}'\n\nRequired for HTTP endpoint destinations. Must not be specified for\nWorkflows, MessageBus, or Topic destinations."]
    pub fn set_network_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_attachment = Some(v.into());
        self
    }
}
impl ToListMappable for EventarcPipelineDestinationsElNetworkConfigEl {
    type O = BlockAssignable<EventarcPipelineDestinationsElNetworkConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineDestinationsElNetworkConfigEl {}
impl BuildEventarcPipelineDestinationsElNetworkConfigEl {
    pub fn build(self) -> EventarcPipelineDestinationsElNetworkConfigEl {
        EventarcPipelineDestinationsElNetworkConfigEl {
            network_attachment: core::default::Default::default(),
        }
    }
}
pub struct EventarcPipelineDestinationsElNetworkConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineDestinationsElNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> EventarcPipelineDestinationsElNetworkConfigElRef {
        EventarcPipelineDestinationsElNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineDestinationsElNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network_attachment` after provisioning.\nName of the NetworkAttachment that allows access to the consumer VPC.\n\nFormat:\n'projects/{PROJECT_ID}/regions/{REGION}/networkAttachments/{NETWORK_ATTACHMENT_NAME}'\n\nRequired for HTTP endpoint destinations. Must not be specified for\nWorkflows, MessageBus, or Topic destinations."]
    pub fn network_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_attachment", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct EventarcPipelineDestinationsElOutputPayloadFormatElAvroEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_definition: Option<PrimField<String>>,
}
impl EventarcPipelineDestinationsElOutputPayloadFormatElAvroEl {
    #[doc = "Set the field `schema_definition`.\nThe entire schema definition is stored in this field."]
    pub fn set_schema_definition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema_definition = Some(v.into());
        self
    }
}
impl ToListMappable for EventarcPipelineDestinationsElOutputPayloadFormatElAvroEl {
    type O = BlockAssignable<EventarcPipelineDestinationsElOutputPayloadFormatElAvroEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineDestinationsElOutputPayloadFormatElAvroEl {}
impl BuildEventarcPipelineDestinationsElOutputPayloadFormatElAvroEl {
    pub fn build(self) -> EventarcPipelineDestinationsElOutputPayloadFormatElAvroEl {
        EventarcPipelineDestinationsElOutputPayloadFormatElAvroEl {
            schema_definition: core::default::Default::default(),
        }
    }
}
pub struct EventarcPipelineDestinationsElOutputPayloadFormatElAvroElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineDestinationsElOutputPayloadFormatElAvroElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EventarcPipelineDestinationsElOutputPayloadFormatElAvroElRef {
        EventarcPipelineDestinationsElOutputPayloadFormatElAvroElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineDestinationsElOutputPayloadFormatElAvroElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema_definition` after provisioning.\nThe entire schema definition is stored in this field."]
    pub fn schema_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schema_definition", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct EventarcPipelineDestinationsElOutputPayloadFormatElJsonEl {}
impl EventarcPipelineDestinationsElOutputPayloadFormatElJsonEl {}
impl ToListMappable for EventarcPipelineDestinationsElOutputPayloadFormatElJsonEl {
    type O = BlockAssignable<EventarcPipelineDestinationsElOutputPayloadFormatElJsonEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineDestinationsElOutputPayloadFormatElJsonEl {}
impl BuildEventarcPipelineDestinationsElOutputPayloadFormatElJsonEl {
    pub fn build(self) -> EventarcPipelineDestinationsElOutputPayloadFormatElJsonEl {
        EventarcPipelineDestinationsElOutputPayloadFormatElJsonEl {}
    }
}
pub struct EventarcPipelineDestinationsElOutputPayloadFormatElJsonElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineDestinationsElOutputPayloadFormatElJsonElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EventarcPipelineDestinationsElOutputPayloadFormatElJsonElRef {
        EventarcPipelineDestinationsElOutputPayloadFormatElJsonElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineDestinationsElOutputPayloadFormatElJsonElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct EventarcPipelineDestinationsElOutputPayloadFormatElProtobufEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_definition: Option<PrimField<String>>,
}
impl EventarcPipelineDestinationsElOutputPayloadFormatElProtobufEl {
    #[doc = "Set the field `schema_definition`.\nThe entire schema definition is stored in this field."]
    pub fn set_schema_definition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema_definition = Some(v.into());
        self
    }
}
impl ToListMappable for EventarcPipelineDestinationsElOutputPayloadFormatElProtobufEl {
    type O = BlockAssignable<EventarcPipelineDestinationsElOutputPayloadFormatElProtobufEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineDestinationsElOutputPayloadFormatElProtobufEl {}
impl BuildEventarcPipelineDestinationsElOutputPayloadFormatElProtobufEl {
    pub fn build(self) -> EventarcPipelineDestinationsElOutputPayloadFormatElProtobufEl {
        EventarcPipelineDestinationsElOutputPayloadFormatElProtobufEl {
            schema_definition: core::default::Default::default(),
        }
    }
}
pub struct EventarcPipelineDestinationsElOutputPayloadFormatElProtobufElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineDestinationsElOutputPayloadFormatElProtobufElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EventarcPipelineDestinationsElOutputPayloadFormatElProtobufElRef {
        EventarcPipelineDestinationsElOutputPayloadFormatElProtobufElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineDestinationsElOutputPayloadFormatElProtobufElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema_definition` after provisioning.\nThe entire schema definition is stored in this field."]
    pub fn schema_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schema_definition", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct EventarcPipelineDestinationsElOutputPayloadFormatElDynamic {
    avro: Option<DynamicBlock<EventarcPipelineDestinationsElOutputPayloadFormatElAvroEl>>,
    json: Option<DynamicBlock<EventarcPipelineDestinationsElOutputPayloadFormatElJsonEl>>,
    protobuf: Option<DynamicBlock<EventarcPipelineDestinationsElOutputPayloadFormatElProtobufEl>>,
}
#[derive(Serialize)]
pub struct EventarcPipelineDestinationsElOutputPayloadFormatEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    avro: Option<Vec<EventarcPipelineDestinationsElOutputPayloadFormatElAvroEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json: Option<Vec<EventarcPipelineDestinationsElOutputPayloadFormatElJsonEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protobuf: Option<Vec<EventarcPipelineDestinationsElOutputPayloadFormatElProtobufEl>>,
    dynamic: EventarcPipelineDestinationsElOutputPayloadFormatElDynamic,
}
impl EventarcPipelineDestinationsElOutputPayloadFormatEl {
    #[doc = "Set the field `avro`.\n"]
    pub fn set_avro(
        mut self,
        v: impl Into<BlockAssignable<EventarcPipelineDestinationsElOutputPayloadFormatElAvroEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.avro = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.avro = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `json`.\n"]
    pub fn set_json(
        mut self,
        v: impl Into<BlockAssignable<EventarcPipelineDestinationsElOutputPayloadFormatElJsonEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.json = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.json = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `protobuf`.\n"]
    pub fn set_protobuf(
        mut self,
        v: impl Into<BlockAssignable<EventarcPipelineDestinationsElOutputPayloadFormatElProtobufEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.protobuf = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.protobuf = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for EventarcPipelineDestinationsElOutputPayloadFormatEl {
    type O = BlockAssignable<EventarcPipelineDestinationsElOutputPayloadFormatEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineDestinationsElOutputPayloadFormatEl {}
impl BuildEventarcPipelineDestinationsElOutputPayloadFormatEl {
    pub fn build(self) -> EventarcPipelineDestinationsElOutputPayloadFormatEl {
        EventarcPipelineDestinationsElOutputPayloadFormatEl {
            avro: core::default::Default::default(),
            json: core::default::Default::default(),
            protobuf: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EventarcPipelineDestinationsElOutputPayloadFormatElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineDestinationsElOutputPayloadFormatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EventarcPipelineDestinationsElOutputPayloadFormatElRef {
        EventarcPipelineDestinationsElOutputPayloadFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineDestinationsElOutputPayloadFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `avro` after provisioning.\n"]
    pub fn avro(&self) -> ListRef<EventarcPipelineDestinationsElOutputPayloadFormatElAvroElRef> {
        ListRef::new(self.shared().clone(), format!("{}.avro", self.base))
    }
    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> ListRef<EventarcPipelineDestinationsElOutputPayloadFormatElJsonElRef> {
        ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }
    #[doc = "Get a reference to the value of field `protobuf` after provisioning.\n"]
    pub fn protobuf(
        &self,
    ) -> ListRef<EventarcPipelineDestinationsElOutputPayloadFormatElProtobufElRef> {
        ListRef::new(self.shared().clone(), format!("{}.protobuf", self.base))
    }
}
#[derive(Serialize, Default)]
struct EventarcPipelineDestinationsElDynamic {
    authentication_config:
        Option<DynamicBlock<EventarcPipelineDestinationsElAuthenticationConfigEl>>,
    http_endpoint: Option<DynamicBlock<EventarcPipelineDestinationsElHttpEndpointEl>>,
    network_config: Option<DynamicBlock<EventarcPipelineDestinationsElNetworkConfigEl>>,
    output_payload_format:
        Option<DynamicBlock<EventarcPipelineDestinationsElOutputPayloadFormatEl>>,
}
#[derive(Serialize)]
pub struct EventarcPipelineDestinationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_bus: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workflow: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_config: Option<Vec<EventarcPipelineDestinationsElAuthenticationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_endpoint: Option<Vec<EventarcPipelineDestinationsElHttpEndpointEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_config: Option<Vec<EventarcPipelineDestinationsElNetworkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_payload_format: Option<Vec<EventarcPipelineDestinationsElOutputPayloadFormatEl>>,
    dynamic: EventarcPipelineDestinationsElDynamic,
}
impl EventarcPipelineDestinationsEl {
    #[doc = "Set the field `message_bus`.\nThe resource name of the Message Bus to which events should be\npublished. The Message Bus resource should exist in the same project as\nthe Pipeline. Format:\n'projects/{project}/locations/{location}/messageBuses/{message_bus}'"]
    pub fn set_message_bus(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_bus = Some(v.into());
        self
    }
    #[doc = "Set the field `topic`.\nThe resource name of the Pub/Sub topic to which events should be\npublished. Format:\n'projects/{project}/locations/{location}/topics/{topic}'"]
    pub fn set_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic = Some(v.into());
        self
    }
    #[doc = "Set the field `workflow`.\nThe resource name of the Workflow whose Executions are triggered by\nthe events. The Workflow resource should be deployed in the same\nproject as the Pipeline. Format:\n'projects/{project}/locations/{location}/workflows/{workflow}'"]
    pub fn set_workflow(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.workflow = Some(v.into());
        self
    }
    #[doc = "Set the field `authentication_config`.\n"]
    pub fn set_authentication_config(
        mut self,
        v: impl Into<BlockAssignable<EventarcPipelineDestinationsElAuthenticationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authentication_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_endpoint`.\n"]
    pub fn set_http_endpoint(
        mut self,
        v: impl Into<BlockAssignable<EventarcPipelineDestinationsElHttpEndpointEl>>,
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
        v: impl Into<BlockAssignable<EventarcPipelineDestinationsElNetworkConfigEl>>,
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
    #[doc = "Set the field `output_payload_format`.\n"]
    pub fn set_output_payload_format(
        mut self,
        v: impl Into<BlockAssignable<EventarcPipelineDestinationsElOutputPayloadFormatEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output_payload_format = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output_payload_format = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for EventarcPipelineDestinationsEl {
    type O = BlockAssignable<EventarcPipelineDestinationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineDestinationsEl {}
impl BuildEventarcPipelineDestinationsEl {
    pub fn build(self) -> EventarcPipelineDestinationsEl {
        EventarcPipelineDestinationsEl {
            message_bus: core::default::Default::default(),
            topic: core::default::Default::default(),
            workflow: core::default::Default::default(),
            authentication_config: core::default::Default::default(),
            http_endpoint: core::default::Default::default(),
            network_config: core::default::Default::default(),
            output_payload_format: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EventarcPipelineDestinationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineDestinationsElRef {
    fn new(shared: StackShared, base: String) -> EventarcPipelineDestinationsElRef {
        EventarcPipelineDestinationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineDestinationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message_bus` after provisioning.\nThe resource name of the Message Bus to which events should be\npublished. The Message Bus resource should exist in the same project as\nthe Pipeline. Format:\n'projects/{project}/locations/{location}/messageBuses/{message_bus}'"]
    pub fn message_bus(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_bus", self.base))
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nThe resource name of the Pub/Sub topic to which events should be\npublished. Format:\n'projects/{project}/locations/{location}/topics/{topic}'"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
    #[doc = "Get a reference to the value of field `workflow` after provisioning.\nThe resource name of the Workflow whose Executions are triggered by\nthe events. The Workflow resource should be deployed in the same\nproject as the Pipeline. Format:\n'projects/{project}/locations/{location}/workflows/{workflow}'"]
    pub fn workflow(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workflow", self.base))
    }
    #[doc = "Get a reference to the value of field `authentication_config` after provisioning.\n"]
    pub fn authentication_config(
        &self,
    ) -> ListRef<EventarcPipelineDestinationsElAuthenticationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authentication_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `http_endpoint` after provisioning.\n"]
    pub fn http_endpoint(&self) -> ListRef<EventarcPipelineDestinationsElHttpEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<EventarcPipelineDestinationsElNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_payload_format` after provisioning.\n"]
    pub fn output_payload_format(
        &self,
    ) -> ListRef<EventarcPipelineDestinationsElOutputPayloadFormatElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_payload_format", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct EventarcPipelineInputPayloadFormatElAvroEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_definition: Option<PrimField<String>>,
}
impl EventarcPipelineInputPayloadFormatElAvroEl {
    #[doc = "Set the field `schema_definition`.\nThe entire schema definition is stored in this field."]
    pub fn set_schema_definition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema_definition = Some(v.into());
        self
    }
}
impl ToListMappable for EventarcPipelineInputPayloadFormatElAvroEl {
    type O = BlockAssignable<EventarcPipelineInputPayloadFormatElAvroEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineInputPayloadFormatElAvroEl {}
impl BuildEventarcPipelineInputPayloadFormatElAvroEl {
    pub fn build(self) -> EventarcPipelineInputPayloadFormatElAvroEl {
        EventarcPipelineInputPayloadFormatElAvroEl {
            schema_definition: core::default::Default::default(),
        }
    }
}
pub struct EventarcPipelineInputPayloadFormatElAvroElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineInputPayloadFormatElAvroElRef {
    fn new(shared: StackShared, base: String) -> EventarcPipelineInputPayloadFormatElAvroElRef {
        EventarcPipelineInputPayloadFormatElAvroElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineInputPayloadFormatElAvroElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema_definition` after provisioning.\nThe entire schema definition is stored in this field."]
    pub fn schema_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schema_definition", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct EventarcPipelineInputPayloadFormatElJsonEl {}
impl EventarcPipelineInputPayloadFormatElJsonEl {}
impl ToListMappable for EventarcPipelineInputPayloadFormatElJsonEl {
    type O = BlockAssignable<EventarcPipelineInputPayloadFormatElJsonEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineInputPayloadFormatElJsonEl {}
impl BuildEventarcPipelineInputPayloadFormatElJsonEl {
    pub fn build(self) -> EventarcPipelineInputPayloadFormatElJsonEl {
        EventarcPipelineInputPayloadFormatElJsonEl {}
    }
}
pub struct EventarcPipelineInputPayloadFormatElJsonElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineInputPayloadFormatElJsonElRef {
    fn new(shared: StackShared, base: String) -> EventarcPipelineInputPayloadFormatElJsonElRef {
        EventarcPipelineInputPayloadFormatElJsonElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineInputPayloadFormatElJsonElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct EventarcPipelineInputPayloadFormatElProtobufEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_definition: Option<PrimField<String>>,
}
impl EventarcPipelineInputPayloadFormatElProtobufEl {
    #[doc = "Set the field `schema_definition`.\nThe entire schema definition is stored in this field."]
    pub fn set_schema_definition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema_definition = Some(v.into());
        self
    }
}
impl ToListMappable for EventarcPipelineInputPayloadFormatElProtobufEl {
    type O = BlockAssignable<EventarcPipelineInputPayloadFormatElProtobufEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineInputPayloadFormatElProtobufEl {}
impl BuildEventarcPipelineInputPayloadFormatElProtobufEl {
    pub fn build(self) -> EventarcPipelineInputPayloadFormatElProtobufEl {
        EventarcPipelineInputPayloadFormatElProtobufEl {
            schema_definition: core::default::Default::default(),
        }
    }
}
pub struct EventarcPipelineInputPayloadFormatElProtobufElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineInputPayloadFormatElProtobufElRef {
    fn new(shared: StackShared, base: String) -> EventarcPipelineInputPayloadFormatElProtobufElRef {
        EventarcPipelineInputPayloadFormatElProtobufElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineInputPayloadFormatElProtobufElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema_definition` after provisioning.\nThe entire schema definition is stored in this field."]
    pub fn schema_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schema_definition", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct EventarcPipelineInputPayloadFormatElDynamic {
    avro: Option<DynamicBlock<EventarcPipelineInputPayloadFormatElAvroEl>>,
    json: Option<DynamicBlock<EventarcPipelineInputPayloadFormatElJsonEl>>,
    protobuf: Option<DynamicBlock<EventarcPipelineInputPayloadFormatElProtobufEl>>,
}
#[derive(Serialize)]
pub struct EventarcPipelineInputPayloadFormatEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    avro: Option<Vec<EventarcPipelineInputPayloadFormatElAvroEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json: Option<Vec<EventarcPipelineInputPayloadFormatElJsonEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protobuf: Option<Vec<EventarcPipelineInputPayloadFormatElProtobufEl>>,
    dynamic: EventarcPipelineInputPayloadFormatElDynamic,
}
impl EventarcPipelineInputPayloadFormatEl {
    #[doc = "Set the field `avro`.\n"]
    pub fn set_avro(
        mut self,
        v: impl Into<BlockAssignable<EventarcPipelineInputPayloadFormatElAvroEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.avro = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.avro = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `json`.\n"]
    pub fn set_json(
        mut self,
        v: impl Into<BlockAssignable<EventarcPipelineInputPayloadFormatElJsonEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.json = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.json = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `protobuf`.\n"]
    pub fn set_protobuf(
        mut self,
        v: impl Into<BlockAssignable<EventarcPipelineInputPayloadFormatElProtobufEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.protobuf = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.protobuf = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for EventarcPipelineInputPayloadFormatEl {
    type O = BlockAssignable<EventarcPipelineInputPayloadFormatEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineInputPayloadFormatEl {}
impl BuildEventarcPipelineInputPayloadFormatEl {
    pub fn build(self) -> EventarcPipelineInputPayloadFormatEl {
        EventarcPipelineInputPayloadFormatEl {
            avro: core::default::Default::default(),
            json: core::default::Default::default(),
            protobuf: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EventarcPipelineInputPayloadFormatElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineInputPayloadFormatElRef {
    fn new(shared: StackShared, base: String) -> EventarcPipelineInputPayloadFormatElRef {
        EventarcPipelineInputPayloadFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineInputPayloadFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `avro` after provisioning.\n"]
    pub fn avro(&self) -> ListRef<EventarcPipelineInputPayloadFormatElAvroElRef> {
        ListRef::new(self.shared().clone(), format!("{}.avro", self.base))
    }
    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> ListRef<EventarcPipelineInputPayloadFormatElJsonElRef> {
        ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }
    #[doc = "Get a reference to the value of field `protobuf` after provisioning.\n"]
    pub fn protobuf(&self) -> ListRef<EventarcPipelineInputPayloadFormatElProtobufElRef> {
        ListRef::new(self.shared().clone(), format!("{}.protobuf", self.base))
    }
}
#[derive(Serialize)]
pub struct EventarcPipelineLoggingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    log_severity: Option<PrimField<String>>,
}
impl EventarcPipelineLoggingConfigEl {
    #[doc = "Set the field `log_severity`.\nThe minimum severity of logs that will be sent to Stackdriver/Platform\nTelemetry. Logs at severitiy ≥ this value will be sent, unless it is NONE. Possible values: [\"NONE\", \"DEBUG\", \"INFO\", \"NOTICE\", \"WARNING\", \"ERROR\", \"CRITICAL\", \"ALERT\", \"EMERGENCY\"]"]
    pub fn set_log_severity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_severity = Some(v.into());
        self
    }
}
impl ToListMappable for EventarcPipelineLoggingConfigEl {
    type O = BlockAssignable<EventarcPipelineLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineLoggingConfigEl {}
impl BuildEventarcPipelineLoggingConfigEl {
    pub fn build(self) -> EventarcPipelineLoggingConfigEl {
        EventarcPipelineLoggingConfigEl {
            log_severity: core::default::Default::default(),
        }
    }
}
pub struct EventarcPipelineLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> EventarcPipelineLoggingConfigElRef {
        EventarcPipelineLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `log_severity` after provisioning.\nThe minimum severity of logs that will be sent to Stackdriver/Platform\nTelemetry. Logs at severitiy ≥ this value will be sent, unless it is NONE. Possible values: [\"NONE\", \"DEBUG\", \"INFO\", \"NOTICE\", \"WARNING\", \"ERROR\", \"CRITICAL\", \"ALERT\", \"EMERGENCY\"]"]
    pub fn log_severity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_severity", self.base))
    }
}
#[derive(Serialize)]
pub struct EventarcPipelineMediationsElTransformationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    transformation_template: Option<PrimField<String>>,
}
impl EventarcPipelineMediationsElTransformationEl {
    #[doc = "Set the field `transformation_template`.\nThe CEL expression template to apply to transform messages.\nThe following CEL extension functions are provided for\nuse in this CEL expression:\n- merge:\nmap1.merge(map2) -> map3\n- Merges the passed CEL map with the existing CEL map the\nfunction is applied to.\n- If the same key exists in both maps, if the key's value is type\nmap both maps are merged else the value from the passed map is\nused.\n- denormalize:\nmap.denormalize() -> map\n- Denormalizes a CEL map such that every value of type map or key\nin the map is expanded to return a single level map.\n- The resulting keys are \".\" separated indices of the map keys.\n- For example:\n{\n\"a\": 1,\n\"b\": {\n\"c\": 2,\n\"d\": 3\n}\n\"e\": [4, 5]\n}\n.denormalize()\n-> {\n\"a\": 1,\n\"b.c\": 2,\n\"b.d\": 3,\n\"e.0\": 4,\n\"e.1\": 5\n}\n- setField:\nmap.setField(key, value) -> message\n- Sets the field of the message with the given key to the\ngiven value.\n- If the field is not present it will be added.\n- If the field is present it will be overwritten.\n- The key can be a dot separated path to set a field in a nested\nmessage.\n- Key must be of type string.\n- Value may be any valid type.\n- removeFields:\nmap.removeFields([key1, key2, ...]) -> message\n- Removes the fields of the map with the given keys.\n- The keys can be a dot separated path to remove a field in a\nnested message.\n- If a key is not found it will be ignored.\n- Keys must be of type string.\n- toMap:\n[map1, map2, ...].toMap() -> map\n- Converts a CEL list of CEL maps to a single CEL map\n- toDestinationPayloadFormat():\nmessage.data.toDestinationPayloadFormat() -> string or bytes\n- Converts the message data to the destination payload format\nspecified in Pipeline.Destination.output_payload_format\n- This function is meant to be applied to the message.data field.\n- If the destination payload format is not set, the function will\nreturn the message data unchanged.\n- toCloudEventJsonWithPayloadFormat:\nmessage.toCloudEventJsonWithPayloadFormat() -> map\n- Converts a message to the corresponding structure of JSON\nformat for CloudEvents\n- This function applies toDestinationPayloadFormat() to the\nmessage data. It also sets the corresponding datacontenttype of\nthe CloudEvent, as indicated by\nPipeline.Destination.output_payload_format. If no\noutput_payload_format is set it will use the existing\ndatacontenttype on the CloudEvent if present, else leave\ndatacontenttype absent.\n- This function expects that the content of the message will\nadhere to the standard CloudEvent format. If it doesn't then this\nfunction will fail.\n- The result is a CEL map that corresponds to the JSON\nrepresentation of the CloudEvent. To convert that data to a JSON\nstring it can be chained with the toJsonString function."]
    pub fn set_transformation_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transformation_template = Some(v.into());
        self
    }
}
impl ToListMappable for EventarcPipelineMediationsElTransformationEl {
    type O = BlockAssignable<EventarcPipelineMediationsElTransformationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineMediationsElTransformationEl {}
impl BuildEventarcPipelineMediationsElTransformationEl {
    pub fn build(self) -> EventarcPipelineMediationsElTransformationEl {
        EventarcPipelineMediationsElTransformationEl {
            transformation_template: core::default::Default::default(),
        }
    }
}
pub struct EventarcPipelineMediationsElTransformationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineMediationsElTransformationElRef {
    fn new(shared: StackShared, base: String) -> EventarcPipelineMediationsElTransformationElRef {
        EventarcPipelineMediationsElTransformationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineMediationsElTransformationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `transformation_template` after provisioning.\nThe CEL expression template to apply to transform messages.\nThe following CEL extension functions are provided for\nuse in this CEL expression:\n- merge:\nmap1.merge(map2) -> map3\n- Merges the passed CEL map with the existing CEL map the\nfunction is applied to.\n- If the same key exists in both maps, if the key's value is type\nmap both maps are merged else the value from the passed map is\nused.\n- denormalize:\nmap.denormalize() -> map\n- Denormalizes a CEL map such that every value of type map or key\nin the map is expanded to return a single level map.\n- The resulting keys are \".\" separated indices of the map keys.\n- For example:\n{\n\"a\": 1,\n\"b\": {\n\"c\": 2,\n\"d\": 3\n}\n\"e\": [4, 5]\n}\n.denormalize()\n-> {\n\"a\": 1,\n\"b.c\": 2,\n\"b.d\": 3,\n\"e.0\": 4,\n\"e.1\": 5\n}\n- setField:\nmap.setField(key, value) -> message\n- Sets the field of the message with the given key to the\ngiven value.\n- If the field is not present it will be added.\n- If the field is present it will be overwritten.\n- The key can be a dot separated path to set a field in a nested\nmessage.\n- Key must be of type string.\n- Value may be any valid type.\n- removeFields:\nmap.removeFields([key1, key2, ...]) -> message\n- Removes the fields of the map with the given keys.\n- The keys can be a dot separated path to remove a field in a\nnested message.\n- If a key is not found it will be ignored.\n- Keys must be of type string.\n- toMap:\n[map1, map2, ...].toMap() -> map\n- Converts a CEL list of CEL maps to a single CEL map\n- toDestinationPayloadFormat():\nmessage.data.toDestinationPayloadFormat() -> string or bytes\n- Converts the message data to the destination payload format\nspecified in Pipeline.Destination.output_payload_format\n- This function is meant to be applied to the message.data field.\n- If the destination payload format is not set, the function will\nreturn the message data unchanged.\n- toCloudEventJsonWithPayloadFormat:\nmessage.toCloudEventJsonWithPayloadFormat() -> map\n- Converts a message to the corresponding structure of JSON\nformat for CloudEvents\n- This function applies toDestinationPayloadFormat() to the\nmessage data. It also sets the corresponding datacontenttype of\nthe CloudEvent, as indicated by\nPipeline.Destination.output_payload_format. If no\noutput_payload_format is set it will use the existing\ndatacontenttype on the CloudEvent if present, else leave\ndatacontenttype absent.\n- This function expects that the content of the message will\nadhere to the standard CloudEvent format. If it doesn't then this\nfunction will fail.\n- The result is a CEL map that corresponds to the JSON\nrepresentation of the CloudEvent. To convert that data to a JSON\nstring it can be chained with the toJsonString function."]
    pub fn transformation_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transformation_template", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct EventarcPipelineMediationsElDynamic {
    transformation: Option<DynamicBlock<EventarcPipelineMediationsElTransformationEl>>,
}
#[derive(Serialize)]
pub struct EventarcPipelineMediationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    transformation: Option<Vec<EventarcPipelineMediationsElTransformationEl>>,
    dynamic: EventarcPipelineMediationsElDynamic,
}
impl EventarcPipelineMediationsEl {
    #[doc = "Set the field `transformation`.\n"]
    pub fn set_transformation(
        mut self,
        v: impl Into<BlockAssignable<EventarcPipelineMediationsElTransformationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.transformation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.transformation = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for EventarcPipelineMediationsEl {
    type O = BlockAssignable<EventarcPipelineMediationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineMediationsEl {}
impl BuildEventarcPipelineMediationsEl {
    pub fn build(self) -> EventarcPipelineMediationsEl {
        EventarcPipelineMediationsEl {
            transformation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EventarcPipelineMediationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineMediationsElRef {
    fn new(shared: StackShared, base: String) -> EventarcPipelineMediationsElRef {
        EventarcPipelineMediationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineMediationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `transformation` after provisioning.\n"]
    pub fn transformation(&self) -> ListRef<EventarcPipelineMediationsElTransformationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transformation", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct EventarcPipelineRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_attempts: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_retry_delay: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_retry_delay: Option<PrimField<String>>,
}
impl EventarcPipelineRetryPolicyEl {
    #[doc = "Set the field `max_attempts`.\nThe maximum number of delivery attempts for any message. The value must\nbe between 1 and 100.\nThe default value for this field is 5."]
    pub fn set_max_attempts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_attempts = Some(v.into());
        self
    }
    #[doc = "Set the field `max_retry_delay`.\nThe maximum amount of seconds to wait between retry attempts. The value\nmust be between 1 and 600.\nThe default value for this field is 60."]
    pub fn set_max_retry_delay(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_retry_delay = Some(v.into());
        self
    }
    #[doc = "Set the field `min_retry_delay`.\nThe minimum amount of seconds to wait between retry attempts. The value\nmust be between 1 and 600.\nThe default value for this field is 5."]
    pub fn set_min_retry_delay(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_retry_delay = Some(v.into());
        self
    }
}
impl ToListMappable for EventarcPipelineRetryPolicyEl {
    type O = BlockAssignable<EventarcPipelineRetryPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineRetryPolicyEl {}
impl BuildEventarcPipelineRetryPolicyEl {
    pub fn build(self) -> EventarcPipelineRetryPolicyEl {
        EventarcPipelineRetryPolicyEl {
            max_attempts: core::default::Default::default(),
            max_retry_delay: core::default::Default::default(),
            min_retry_delay: core::default::Default::default(),
        }
    }
}
pub struct EventarcPipelineRetryPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineRetryPolicyElRef {
    fn new(shared: StackShared, base: String) -> EventarcPipelineRetryPolicyElRef {
        EventarcPipelineRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_attempts` after provisioning.\nThe maximum number of delivery attempts for any message. The value must\nbe between 1 and 100.\nThe default value for this field is 5."]
    pub fn max_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_attempts", self.base))
    }
    #[doc = "Get a reference to the value of field `max_retry_delay` after provisioning.\nThe maximum amount of seconds to wait between retry attempts. The value\nmust be between 1 and 600.\nThe default value for this field is 60."]
    pub fn max_retry_delay(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_retry_delay", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_retry_delay` after provisioning.\nThe minimum amount of seconds to wait between retry attempts. The value\nmust be between 1 and 600.\nThe default value for this field is 5."]
    pub fn min_retry_delay(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_retry_delay", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct EventarcPipelineTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl EventarcPipelineTimeoutsEl {
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
impl ToListMappable for EventarcPipelineTimeoutsEl {
    type O = BlockAssignable<EventarcPipelineTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEventarcPipelineTimeoutsEl {}
impl BuildEventarcPipelineTimeoutsEl {
    pub fn build(self) -> EventarcPipelineTimeoutsEl {
        EventarcPipelineTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct EventarcPipelineTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EventarcPipelineTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EventarcPipelineTimeoutsElRef {
        EventarcPipelineTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EventarcPipelineTimeoutsElRef {
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
struct EventarcPipelineDynamic {
    destinations: Option<DynamicBlock<EventarcPipelineDestinationsEl>>,
    input_payload_format: Option<DynamicBlock<EventarcPipelineInputPayloadFormatEl>>,
    logging_config: Option<DynamicBlock<EventarcPipelineLoggingConfigEl>>,
    mediations: Option<DynamicBlock<EventarcPipelineMediationsEl>>,
    retry_policy: Option<DynamicBlock<EventarcPipelineRetryPolicyEl>>,
}
