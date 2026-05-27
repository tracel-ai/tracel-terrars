use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ClouddeployDeliveryPipelineData {
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
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suspended: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serial_pipeline: Option<Vec<ClouddeployDeliveryPipelineSerialPipelineEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ClouddeployDeliveryPipelineTimeoutsEl>,
    dynamic: ClouddeployDeliveryPipelineDynamic,
}
struct ClouddeployDeliveryPipeline_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ClouddeployDeliveryPipelineData>,
}
#[derive(Clone)]
pub struct ClouddeployDeliveryPipeline(Rc<ClouddeployDeliveryPipeline_>);
impl ClouddeployDeliveryPipeline {
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
    #[doc = "Set the field `annotations`.\nUser annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nDescription of the `DeliveryPipeline`. Max length is 255 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `suspended`.\nWhen suspended, no new releases or rollouts can be created, but in-progress ones will complete."]
    pub fn set_suspended(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().suspended = Some(v.into());
        self
    }
    #[doc = "Set the field `serial_pipeline`.\n"]
    pub fn set_serial_pipeline(
        self,
        v: impl Into<BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().serial_pipeline = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.serial_pipeline = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ClouddeployDeliveryPipelineTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUser annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\nOutput only. Information around the state of the Delivery Pipeline."]
    pub fn condition(&self) -> ListRef<ClouddeployDeliveryPipelineConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the pipeline was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the `DeliveryPipeline`. Max length is 255 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the `DeliveryPipeline`. Format is `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `suspended` after provisioning.\nWhen suspended, no new releases or rollouts can be created, but in-progress ones will complete."]
    pub fn suspended(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.suspended", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique identifier of the `DeliveryPipeline`."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Most recent time at which the pipeline was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `serial_pipeline` after provisioning.\n"]
    pub fn serial_pipeline(&self) -> ListRef<ClouddeployDeliveryPipelineSerialPipelineElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.serial_pipeline", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployDeliveryPipelineTimeoutsElRef {
        ClouddeployDeliveryPipelineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ClouddeployDeliveryPipeline {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ClouddeployDeliveryPipeline {}
impl ToListMappable for ClouddeployDeliveryPipeline {
    type O = ListRef<ClouddeployDeliveryPipelineRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ClouddeployDeliveryPipeline_ {
    fn extract_resource_type(&self) -> String {
        "google_clouddeploy_delivery_pipeline".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildClouddeployDeliveryPipeline {
    pub tf_id: String,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
    #[doc = "Name of the `DeliveryPipeline`. Format is `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`."]
    pub name: PrimField<String>,
}
impl BuildClouddeployDeliveryPipeline {
    pub fn build(self, stack: &mut Stack) -> ClouddeployDeliveryPipeline {
        let out = ClouddeployDeliveryPipeline(Rc::new(ClouddeployDeliveryPipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ClouddeployDeliveryPipelineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                suspended: core::default::Default::default(),
                serial_pipeline: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ClouddeployDeliveryPipelineRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ClouddeployDeliveryPipelineRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUser annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\nOutput only. Information around the state of the Delivery Pipeline."]
    pub fn condition(&self) -> ListRef<ClouddeployDeliveryPipelineConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the pipeline was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the `DeliveryPipeline`. Max length is 255 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the `DeliveryPipeline`. Format is `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `suspended` after provisioning.\nWhen suspended, no new releases or rollouts can be created, but in-progress ones will complete."]
    pub fn suspended(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.suspended", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique identifier of the `DeliveryPipeline`."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Most recent time at which the pipeline was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `serial_pipeline` after provisioning.\n"]
    pub fn serial_pipeline(&self) -> ListRef<ClouddeployDeliveryPipelineSerialPipelineElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.serial_pipeline", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployDeliveryPipelineTimeoutsElRef {
        ClouddeployDeliveryPipelineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl ClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.status = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl {
    type O = BlockAssignable<ClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl {}
impl BuildClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl {
        ClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl {
            status: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployDeliveryPipelineConditionElPipelineReadyConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineConditionElPipelineReadyConditionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeliveryPipelineConditionElPipelineReadyConditionElRef {
        ClouddeployDeliveryPipelineConditionElPipelineReadyConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeliveryPipelineConditionElPipelineReadyConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    missing_targets: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl ClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl {
    #[doc = "Set the field `missing_targets`.\n"]
    pub fn set_missing_targets(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.missing_targets = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.status = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl {
    type O = BlockAssignable<ClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl {}
impl BuildClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl {
        ClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl {
            missing_targets: core::default::Default::default(),
            status: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployDeliveryPipelineConditionElTargetsPresentConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineConditionElTargetsPresentConditionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeliveryPipelineConditionElTargetsPresentConditionElRef {
        ClouddeployDeliveryPipelineConditionElTargetsPresentConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeliveryPipelineConditionElTargetsPresentConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `missing_targets` after provisioning.\n"]
    pub fn missing_targets(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.missing_targets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    error_details: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<bool>>,
}
impl ClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl {
    #[doc = "Set the field `error_details`.\n"]
    pub fn set_error_details(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.error_details = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl {
    type O = BlockAssignable<ClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl {}
impl BuildClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl {
        ClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl {
            error_details: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployDeliveryPipelineConditionElTargetsTypeConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineConditionElTargetsTypeConditionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeliveryPipelineConditionElTargetsTypeConditionElRef {
        ClouddeployDeliveryPipelineConditionElTargetsTypeConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeliveryPipelineConditionElTargetsTypeConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `error_details` after provisioning.\n"]
    pub fn error_details(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.error_details", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_ready_condition:
        Option<ListField<ClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    targets_present_condition:
        Option<ListField<ClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    targets_type_condition:
        Option<ListField<ClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl>>,
}
impl ClouddeployDeliveryPipelineConditionEl {
    #[doc = "Set the field `pipeline_ready_condition`.\n"]
    pub fn set_pipeline_ready_condition(
        mut self,
        v: impl Into<ListField<ClouddeployDeliveryPipelineConditionElPipelineReadyConditionEl>>,
    ) -> Self {
        self.pipeline_ready_condition = Some(v.into());
        self
    }
    #[doc = "Set the field `targets_present_condition`.\n"]
    pub fn set_targets_present_condition(
        mut self,
        v: impl Into<ListField<ClouddeployDeliveryPipelineConditionElTargetsPresentConditionEl>>,
    ) -> Self {
        self.targets_present_condition = Some(v.into());
        self
    }
    #[doc = "Set the field `targets_type_condition`.\n"]
    pub fn set_targets_type_condition(
        mut self,
        v: impl Into<ListField<ClouddeployDeliveryPipelineConditionElTargetsTypeConditionEl>>,
    ) -> Self {
        self.targets_type_condition = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployDeliveryPipelineConditionEl {
    type O = BlockAssignable<ClouddeployDeliveryPipelineConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineConditionEl {}
impl BuildClouddeployDeliveryPipelineConditionEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineConditionEl {
        ClouddeployDeliveryPipelineConditionEl {
            pipeline_ready_condition: core::default::Default::default(),
            targets_present_condition: core::default::Default::default(),
            targets_type_condition: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployDeliveryPipelineConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineConditionElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployDeliveryPipelineConditionElRef {
        ClouddeployDeliveryPipelineConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeliveryPipelineConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pipeline_ready_condition` after provisioning.\n"]
    pub fn pipeline_ready_condition(
        &self,
    ) -> ListRef<ClouddeployDeliveryPipelineConditionElPipelineReadyConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pipeline_ready_condition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `targets_present_condition` after provisioning.\n"]
    pub fn targets_present_condition(
        &self,
    ) -> ListRef<ClouddeployDeliveryPipelineConditionElTargetsPresentConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.targets_present_condition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `targets_type_condition` after provisioning.\n"]
    pub fn targets_type_condition(
        &self,
    ) -> ListRef<ClouddeployDeliveryPipelineConditionElTargetsTypeConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.targets_type_condition", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    match_target_labels: Option<RecField<PrimField<String>>>,
    values: RecField<PrimField<String>>,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl {
    #[doc = "Set the field `match_target_labels`.\nOptional. Deploy parameters are applied to targets with match labels. If unspecified, deploy parameters are applied to all targets (including child targets of a multi-target)."]
    pub fn set_match_target_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.match_target_labels = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl {
    type O = BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl {
    #[doc = "Required. Values are deploy parameters in key-value pairs."]
    pub values: RecField<PrimField<String>>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl {
            match_target_labels: core::default::Default::default(),
            values: self.values,
        }
    }
}
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `match_target_labels` after provisioning.\nOptional. Deploy parameters are applied to targets with match labels. If unspecified, deploy parameters are applied to all targets (including child targets of a multi-target)."]
    pub fn match_target_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.match_target_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\nRequired. Values are deploy parameters in key-value pairs."]
    pub fn values(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElContainerEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<RecField<PrimField<String>>>,
    image: PrimField<String>,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElContainerEl { # [doc = "Set the field `args`.\nOptional. Args is the container arguments to use. This overrides the default arguments defined in the container image."] pub fn set_args (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . args = Some (v . into ()) ; self } # [doc = "Set the field `command`.\nOptional. Command is the container entrypoint to use. This overrides the default entrypoint defined in the container image."] pub fn set_command (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . command = Some (v . into ()) ; self } # [doc = "Set the field `env`.\nOptional. Environment variables that are set in the container."] pub fn set_env (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . env = Some (v . into ()) ; self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElContainerEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElContainerEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElContainerEl
{
    #[doc = "Required. Image is the container image to use."]
    pub image: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElContainerEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElContainerEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElContainerEl { args : core :: default :: Default :: default () , command : core :: default :: Default :: default () , env : core :: default :: Default :: default () , image : self . image , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElContainerElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElContainerElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElContainerElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElContainerElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElContainerElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `args` after provisioning.\nOptional. Args is the container arguments to use. This overrides the default arguments defined in the container image."] pub fn args (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.args" , self . base)) } # [doc = "Get a reference to the value of field `command` after provisioning.\nOptional. Command is the container entrypoint to use. This overrides the default entrypoint defined in the container image."] pub fn command (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.command" , self . base)) } # [doc = "Get a reference to the value of field `env` after provisioning.\nOptional. Environment variables that are set in the container."] pub fn env (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.env" , self . base)) } # [doc = "Get a reference to the value of field `image` after provisioning.\nRequired. Image is the container image to use."] pub fn image (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElDynamic { container : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElContainerEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskEl { # [serde (skip_serializing_if = "Option::is_none")] container : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElContainerEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskEl { # [doc = "Set the field `container`.\n"] pub fn set_container (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElContainerEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . container = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . container = Some (d) ; } } self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskEl { container : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `container` after provisioning.\n"] pub fn container (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElContainerElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.container" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElDynamic { task : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksEl { # [serde (skip_serializing_if = "Option::is_none")] frequency : Option < PrimField < String > > , id : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] task : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksEl { # [doc = "Set the field `frequency`.\nOptional. Frequency of the custom check."] pub fn set_frequency (mut self , v : impl Into < PrimField < String > >) -> Self { self . frequency = Some (v . into ()) ; self } # [doc = "Set the field `task`.\n"] pub fn set_task (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . task = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . task = Some (d) ; } } self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksEl
{
    #[doc = "Required. Unique identifier for the custom check."]
    pub id: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksEl { frequency : core :: default :: Default :: default () , id : self . id , task : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `frequency` after provisioning.\nOptional. Frequency of the custom check."] pub fn frequency (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.frequency" , self . base)) } # [doc = "Get a reference to the value of field `id` after provisioning.\nRequired. Unique identifier for the custom check."] pub fn id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.id" , self . base)) } # [doc = "Get a reference to the value of field `task` after provisioning.\n"] pub fn task (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElTaskElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.task" , self . base)) } }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElAlertPolicyChecksEl
{
    alert_policies: ListField<PrimField<String>>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElAlertPolicyChecksEl { # [doc = "Set the field `labels`.\nOptional. Labels to filter the alert policies."] pub fn set_labels (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . labels = Some (v . into ()) ; self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElAlertPolicyChecksEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElAlertPolicyChecksEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElAlertPolicyChecksEl
{
    #[doc = "Required. The list of alert policy names to check. Format: `projects/{project}/alertPolicies/{alert_policy}`."]
    pub alert_policies: ListField<PrimField<String>>,
    #[doc = "Required. Unique identifier for the alert policy check."]
    pub id: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElAlertPolicyChecksEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElAlertPolicyChecksEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElAlertPolicyChecksEl { alert_policies : self . alert_policies , id : self . id , labels : core :: default :: Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElAlertPolicyChecksElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElAlertPolicyChecksElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElAlertPolicyChecksElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElAlertPolicyChecksElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElAlertPolicyChecksElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `alert_policies` after provisioning.\nRequired. The list of alert policy names to check. Format: `projects/{project}/alertPolicies/{alert_policy}`."] pub fn alert_policies (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.alert_policies" , self . base)) } # [doc = "Get a reference to the value of field `id` after provisioning.\nRequired. Unique identifier for the alert policy check."] pub fn id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.id" , self . base)) } # [doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels to filter the alert policies."] pub fn labels (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.labels" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElDynamic { alert_policy_checks : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElAlertPolicyChecksEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudEl { # [serde (skip_serializing_if = "Option::is_none")] alert_policy_checks : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElAlertPolicyChecksEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudEl { # [doc = "Set the field `alert_policy_checks`.\n"] pub fn set_alert_policy_checks (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElAlertPolicyChecksEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . alert_policy_checks = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . alert_policy_checks = Some (d) ; } } self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudEl { alert_policy_checks : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `alert_policy_checks` after provisioning.\n"] pub fn alert_policy_checks (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElAlertPolicyChecksElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.alert_policy_checks" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElDynamic { custom_checks : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksEl >> , google_cloud : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisEl { duration : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] custom_checks : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksEl > > , # [serde (skip_serializing_if = "Option::is_none")] google_cloud : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisEl { # [doc = "Set the field `custom_checks`.\n"] pub fn set_custom_checks (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . custom_checks = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . custom_checks = Some (d) ; } } self } # [doc = "Set the field `google_cloud`.\n"] pub fn set_google_cloud (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . google_cloud = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . google_cloud = Some (d) ; } } self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisEl
{
    #[doc = "Required. Duration of the analysis."]
    pub duration: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisEl { duration : self . duration , custom_checks : core :: default :: Default :: default () , google_cloud : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `duration` after provisioning.\nRequired. Duration of the analysis."] pub fn duration (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.duration" , self . base)) } # [doc = "Get a reference to the value of field `custom_checks` after provisioning.\n"] pub fn custom_checks (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElCustomChecksElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.custom_checks" , self . base)) } # [doc = "Get a reference to the value of field `google_cloud` after provisioning.\n"] pub fn google_cloud (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElGoogleCloudElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.google_cloud" , self . base)) } }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<ListField<PrimField<String>>>,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl { # [doc = "Set the field `actions`.\nOptional. A sequence of skaffold custom actions to invoke during execution of the postdeploy job."] pub fn set_actions (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . actions = Some (v . into ()) ; self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl { actions : core :: default :: Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `actions` after provisioning.\nOptional. A sequence of skaffold custom actions to invoke during execution of the postdeploy job."] pub fn actions (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.actions" , self . base)) } }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<ListField<PrimField<String>>>,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl { # [doc = "Set the field `actions`.\nOptional. A sequence of skaffold custom actions to invoke during execution of the predeploy job."] pub fn set_actions (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . actions = Some (v . into ()) ; self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl { actions : core :: default :: Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `actions` after provisioning.\nOptional. A sequence of skaffold custom actions to invoke during execution of the predeploy job."] pub fn actions (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.actions" , self . base)) } }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElContainerEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<RecField<PrimField<String>>>,
    image: PrimField<String>,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElContainerEl { # [doc = "Set the field `args`.\nOptional. Args is the container arguments to use. This overrides the default arguments defined in the container image."] pub fn set_args (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . args = Some (v . into ()) ; self } # [doc = "Set the field `command`.\nOptional. Command is the container entrypoint to use. This overrides the default entrypoint defined in the container image."] pub fn set_command (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . command = Some (v . into ()) ; self } # [doc = "Set the field `env`.\nOptional. Environment variables that are set in the container."] pub fn set_env (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . env = Some (v . into ()) ; self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElContainerEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElContainerEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElContainerEl
{
    #[doc = "Required. Image is the container image to use."]
    pub image: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElContainerEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElContainerEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElContainerEl { args : core :: default :: Default :: default () , command : core :: default :: Default :: default () , env : core :: default :: Default :: default () , image : self . image , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElContainerElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElContainerElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElContainerElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElContainerElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElContainerElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `args` after provisioning.\nOptional. Args is the container arguments to use. This overrides the default arguments defined in the container image."] pub fn args (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.args" , self . base)) } # [doc = "Get a reference to the value of field `command` after provisioning.\nOptional. Command is the container entrypoint to use. This overrides the default entrypoint defined in the container image."] pub fn command (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.command" , self . base)) } # [doc = "Get a reference to the value of field `env` after provisioning.\nOptional. Environment variables that are set in the container."] pub fn env (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.env" , self . base)) } # [doc = "Get a reference to the value of field `image` after provisioning.\nRequired. Image is the container image to use."] pub fn image (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElDynamic { container : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElContainerEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksEl { # [serde (skip_serializing_if = "Option::is_none")] container : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElContainerEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksEl { # [doc = "Set the field `container`.\n"] pub fn set_container (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElContainerEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . container = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . container = Some (d) ; } } self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksEl { container : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `container` after provisioning.\n"] pub fn container (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElContainerElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.container" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElDynamic { tasks : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigEl { # [serde (skip_serializing_if = "Option::is_none")] tasks : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigEl { # [doc = "Set the field `tasks`.\n"] pub fn set_tasks (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . tasks = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . tasks = Some (d) ; } } self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigEl { tasks : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `tasks` after provisioning.\n"] pub fn tasks (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElTasksElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.tasks" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElDynamic { analysis : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisEl >> , postdeploy : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl >> , predeploy : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl >> , verify_config : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl { percentages : ListField < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] verify : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] analysis : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisEl > > , # [serde (skip_serializing_if = "Option::is_none")] postdeploy : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl > > , # [serde (skip_serializing_if = "Option::is_none")] predeploy : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl > > , # [serde (skip_serializing_if = "Option::is_none")] verify_config : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl {
    #[doc = "Set the field `verify`.\nWhether to run verify tests after each percentage deployment."]
    pub fn set_verify(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.verify = Some(v.into());
        self
    }
    #[doc = "Set the field `analysis`.\n"]
    pub fn set_analysis(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.analysis = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.analysis = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `postdeploy`.\n"]
    pub fn set_postdeploy(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.postdeploy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.postdeploy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `predeploy`.\n"]
    pub fn set_predeploy(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.predeploy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.predeploy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `verify_config`.\n"]
    pub fn set_verify_config(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.verify_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.verify_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl
{
    type O = BlockAssignable<
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl
{
    #[doc = "Required. The percentage based deployments that will occur as a part of a `Rollout`. List is expected in ascending order and each integer n is 0 <= n < 100."]
    pub percentages: ListField<PrimField<f64>>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl
    {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl {
            percentages: self.percentages,
            verify: core::default::Default::default(),
            analysis: core::default::Default::default(),
            postdeploy: core::default::Default::default(),
            predeploy: core::default::Default::default(),
            verify_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElRef
    {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `percentages` after provisioning.\nRequired. The percentage based deployments that will occur as a part of a `Rollout`. List is expected in ascending order and each integer n is 0 <= n < 100."]
    pub fn percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.percentages", self.base))
    }
    #[doc = "Get a reference to the value of field `verify` after provisioning.\nWhether to run verify tests after each percentage deployment."]
    pub fn verify(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.verify", self.base))
    }
    #[doc = "Get a reference to the value of field `analysis` after provisioning.\n"]    pub fn analysis (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElAnalysisElRef >{
        ListRef::new(self.shared().clone(), format!("{}.analysis", self.base))
    }
    #[doc = "Get a reference to the value of field `postdeploy` after provisioning.\n"]    pub fn postdeploy (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPostdeployElRef >{
        ListRef::new(self.shared().clone(), format!("{}.postdeploy", self.base))
    }
    #[doc = "Get a reference to the value of field `predeploy` after provisioning.\n"]    pub fn predeploy (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElPredeployElRef >{
        ListRef::new(self.shared().clone(), format!("{}.predeploy", self.base))
    }
    #[doc = "Get a reference to the value of field `verify_config` after provisioning.\n"]    pub fn verify_config (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElVerifyConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.verify_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElContainerEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<RecField<PrimField<String>>>,
    image: PrimField<String>,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElContainerEl { # [doc = "Set the field `args`.\nOptional. Args is the container arguments to use. This overrides the default arguments defined in the container image."] pub fn set_args (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . args = Some (v . into ()) ; self } # [doc = "Set the field `command`.\nOptional. Command is the container entrypoint to use. This overrides the default entrypoint defined in the container image."] pub fn set_command (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . command = Some (v . into ()) ; self } # [doc = "Set the field `env`.\nOptional. Environment variables that are set in the container."] pub fn set_env (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . env = Some (v . into ()) ; self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElContainerEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElContainerEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElContainerEl
{
    #[doc = "Required. Image is the container image to use."]
    pub image: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElContainerEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElContainerEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElContainerEl { args : core :: default :: Default :: default () , command : core :: default :: Default :: default () , env : core :: default :: Default :: default () , image : self . image , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElContainerElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElContainerElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElContainerElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElContainerElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElContainerElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `args` after provisioning.\nOptional. Args is the container arguments to use. This overrides the default arguments defined in the container image."] pub fn args (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.args" , self . base)) } # [doc = "Get a reference to the value of field `command` after provisioning.\nOptional. Command is the container entrypoint to use. This overrides the default entrypoint defined in the container image."] pub fn command (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.command" , self . base)) } # [doc = "Get a reference to the value of field `env` after provisioning.\nOptional. Environment variables that are set in the container."] pub fn env (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.env" , self . base)) } # [doc = "Get a reference to the value of field `image` after provisioning.\nRequired. Image is the container image to use."] pub fn image (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElDynamic { container : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElContainerEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskEl { # [serde (skip_serializing_if = "Option::is_none")] container : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElContainerEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskEl { # [doc = "Set the field `container`.\n"] pub fn set_container (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElContainerEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . container = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . container = Some (d) ; } } self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskEl { container : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `container` after provisioning.\n"] pub fn container (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElContainerElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.container" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElDynamic { task : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksEl { # [serde (skip_serializing_if = "Option::is_none")] frequency : Option < PrimField < String > > , id : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] task : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksEl { # [doc = "Set the field `frequency`.\nOptional. Frequency of the custom check."] pub fn set_frequency (mut self , v : impl Into < PrimField < String > >) -> Self { self . frequency = Some (v . into ()) ; self } # [doc = "Set the field `task`.\n"] pub fn set_task (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . task = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . task = Some (d) ; } } self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksEl
{
    #[doc = "Required. Unique identifier for the custom check."]
    pub id: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksEl { frequency : core :: default :: Default :: default () , id : self . id , task : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `frequency` after provisioning.\nOptional. Frequency of the custom check."] pub fn frequency (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.frequency" , self . base)) } # [doc = "Get a reference to the value of field `id` after provisioning.\nRequired. Unique identifier for the custom check."] pub fn id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.id" , self . base)) } # [doc = "Get a reference to the value of field `task` after provisioning.\n"] pub fn task (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElTaskElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.task" , self . base)) } }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElAlertPolicyChecksEl
{
    alert_policies: ListField<PrimField<String>>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElAlertPolicyChecksEl { # [doc = "Set the field `labels`.\nOptional. Labels to filter the alert policies."] pub fn set_labels (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . labels = Some (v . into ()) ; self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElAlertPolicyChecksEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElAlertPolicyChecksEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElAlertPolicyChecksEl
{
    #[doc = "Required. The list of alert policy names to check. Format: `projects/{project}/alertPolicies/{alert_policy}`."]
    pub alert_policies: ListField<PrimField<String>>,
    #[doc = "Required. Unique identifier for the alert policy check."]
    pub id: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElAlertPolicyChecksEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElAlertPolicyChecksEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElAlertPolicyChecksEl { alert_policies : self . alert_policies , id : self . id , labels : core :: default :: Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElAlertPolicyChecksElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElAlertPolicyChecksElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElAlertPolicyChecksElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElAlertPolicyChecksElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElAlertPolicyChecksElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `alert_policies` after provisioning.\nRequired. The list of alert policy names to check. Format: `projects/{project}/alertPolicies/{alert_policy}`."] pub fn alert_policies (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.alert_policies" , self . base)) } # [doc = "Get a reference to the value of field `id` after provisioning.\nRequired. Unique identifier for the alert policy check."] pub fn id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.id" , self . base)) } # [doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels to filter the alert policies."] pub fn labels (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.labels" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElDynamic { alert_policy_checks : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElAlertPolicyChecksEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudEl { # [serde (skip_serializing_if = "Option::is_none")] alert_policy_checks : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElAlertPolicyChecksEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudEl { # [doc = "Set the field `alert_policy_checks`.\n"] pub fn set_alert_policy_checks (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElAlertPolicyChecksEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . alert_policy_checks = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . alert_policy_checks = Some (d) ; } } self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudEl { alert_policy_checks : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `alert_policy_checks` after provisioning.\n"] pub fn alert_policy_checks (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElAlertPolicyChecksElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.alert_policy_checks" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElDynamic { custom_checks : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksEl >> , google_cloud : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisEl { duration : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] custom_checks : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksEl > > , # [serde (skip_serializing_if = "Option::is_none")] google_cloud : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisEl { # [doc = "Set the field `custom_checks`.\n"] pub fn set_custom_checks (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . custom_checks = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . custom_checks = Some (d) ; } } self } # [doc = "Set the field `google_cloud`.\n"] pub fn set_google_cloud (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . google_cloud = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . google_cloud = Some (d) ; } } self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisEl
{
    #[doc = "Required. Duration of the analysis."]
    pub duration: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisEl { duration : self . duration , custom_checks : core :: default :: Default :: default () , google_cloud : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `duration` after provisioning.\nRequired. Duration of the analysis."] pub fn duration (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.duration" , self . base)) } # [doc = "Get a reference to the value of field `custom_checks` after provisioning.\n"] pub fn custom_checks (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElCustomChecksElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.custom_checks" , self . base)) } # [doc = "Get a reference to the value of field `google_cloud` after provisioning.\n"] pub fn google_cloud (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElGoogleCloudElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.google_cloud" , self . base)) } }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<ListField<PrimField<String>>>,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl { # [doc = "Set the field `actions`.\nOptional. A sequence of skaffold custom actions to invoke during execution of the postdeploy job."] pub fn set_actions (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . actions = Some (v . into ()) ; self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl { actions : core :: default :: Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `actions` after provisioning.\nOptional. A sequence of skaffold custom actions to invoke during execution of the postdeploy job."] pub fn actions (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.actions" , self . base)) } }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<ListField<PrimField<String>>>,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl { # [doc = "Set the field `actions`.\nOptional. A sequence of skaffold custom actions to invoke during execution of the predeploy job."] pub fn set_actions (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . actions = Some (v . into ()) ; self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl { actions : core :: default :: Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `actions` after provisioning.\nOptional. A sequence of skaffold custom actions to invoke during execution of the predeploy job."] pub fn actions (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.actions" , self . base)) } }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElContainerEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<RecField<PrimField<String>>>,
    image: PrimField<String>,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElContainerEl { # [doc = "Set the field `args`.\nOptional. Args is the container arguments to use. This overrides the default arguments defined in the container image."] pub fn set_args (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . args = Some (v . into ()) ; self } # [doc = "Set the field `command`.\nOptional. Command is the container entrypoint to use. This overrides the default entrypoint defined in the container image."] pub fn set_command (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . command = Some (v . into ()) ; self } # [doc = "Set the field `env`.\nOptional. Environment variables that are set in the container."] pub fn set_env (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . env = Some (v . into ()) ; self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElContainerEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElContainerEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElContainerEl
{
    #[doc = "Required. Image is the container image to use."]
    pub image: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElContainerEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElContainerEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElContainerEl { args : core :: default :: Default :: default () , command : core :: default :: Default :: default () , env : core :: default :: Default :: default () , image : self . image , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElContainerElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElContainerElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElContainerElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElContainerElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElContainerElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `args` after provisioning.\nOptional. Args is the container arguments to use. This overrides the default arguments defined in the container image."] pub fn args (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.args" , self . base)) } # [doc = "Get a reference to the value of field `command` after provisioning.\nOptional. Command is the container entrypoint to use. This overrides the default entrypoint defined in the container image."] pub fn command (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.command" , self . base)) } # [doc = "Get a reference to the value of field `env` after provisioning.\nOptional. Environment variables that are set in the container."] pub fn env (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.env" , self . base)) } # [doc = "Get a reference to the value of field `image` after provisioning.\nRequired. Image is the container image to use."] pub fn image (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElDynamic { container : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElContainerEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksEl { # [serde (skip_serializing_if = "Option::is_none")] container : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElContainerEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksEl { # [doc = "Set the field `container`.\n"] pub fn set_container (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElContainerEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . container = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . container = Some (d) ; } } self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksEl { container : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `container` after provisioning.\n"] pub fn container (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElContainerElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.container" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElDynamic { tasks : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigEl { # [serde (skip_serializing_if = "Option::is_none")] tasks : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigEl { # [doc = "Set the field `tasks`.\n"] pub fn set_tasks (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . tasks = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . tasks = Some (d) ; } } self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigEl { tasks : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `tasks` after provisioning.\n"] pub fn tasks (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElTasksElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.tasks" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElDynamic { analysis : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisEl >> , postdeploy : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl >> , predeploy : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl >> , verify_config : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl { percentage : PrimField < f64 > , phase_id : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] profiles : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] verify : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] analysis : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisEl > > , # [serde (skip_serializing_if = "Option::is_none")] postdeploy : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl > > , # [serde (skip_serializing_if = "Option::is_none")] predeploy : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl > > , # [serde (skip_serializing_if = "Option::is_none")] verify_config : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl { # [doc = "Set the field `profiles`.\nSkaffold profiles to use when rendering the manifest for this phase. These are in addition to the profiles list specified in the `DeliveryPipeline` stage."] pub fn set_profiles (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . profiles = Some (v . into ()) ; self } # [doc = "Set the field `verify`.\nWhether to run verify tests after the deployment."] pub fn set_verify (mut self , v : impl Into < PrimField < bool > >) -> Self { self . verify = Some (v . into ()) ; self } # [doc = "Set the field `analysis`.\n"] pub fn set_analysis (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . analysis = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . analysis = Some (d) ; } } self } # [doc = "Set the field `postdeploy`.\n"] pub fn set_postdeploy (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . postdeploy = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . postdeploy = Some (d) ; } } self } # [doc = "Set the field `predeploy`.\n"] pub fn set_predeploy (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . predeploy = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . predeploy = Some (d) ; } } self } # [doc = "Set the field `verify_config`.\n"] pub fn set_verify_config (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . verify_config = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . verify_config = Some (d) ; } } self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl
{
    #[doc = "Required. Percentage deployment for the phase."]
    pub percentage: PrimField<f64>,
    #[doc = "Required. The ID to assign to the `Rollout` phase. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: `^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$`."]
    pub phase_id: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl { percentage : self . percentage , phase_id : self . phase_id , profiles : core :: default :: Default :: default () , verify : core :: default :: Default :: default () , analysis : core :: default :: Default :: default () , postdeploy : core :: default :: Default :: default () , predeploy : core :: default :: Default :: default () , verify_config : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `percentage` after provisioning.\nRequired. Percentage deployment for the phase."] pub fn percentage (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.percentage" , self . base)) } # [doc = "Get a reference to the value of field `phase_id` after provisioning.\nRequired. The ID to assign to the `Rollout` phase. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: `^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$`."] pub fn phase_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.phase_id" , self . base)) } # [doc = "Get a reference to the value of field `profiles` after provisioning.\nSkaffold profiles to use when rendering the manifest for this phase. These are in addition to the profiles list specified in the `DeliveryPipeline` stage."] pub fn profiles (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.profiles" , self . base)) } # [doc = "Get a reference to the value of field `verify` after provisioning.\nWhether to run verify tests after the deployment."] pub fn verify (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.verify" , self . base)) } # [doc = "Get a reference to the value of field `analysis` after provisioning.\n"] pub fn analysis (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElAnalysisElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.analysis" , self . base)) } # [doc = "Get a reference to the value of field `postdeploy` after provisioning.\n"] pub fn postdeploy (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPostdeployElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.postdeploy" , self . base)) } # [doc = "Get a reference to the value of field `predeploy` after provisioning.\n"] pub fn predeploy (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElPredeployElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.predeploy" , self . base)) } # [doc = "Get a reference to the value of field `verify_config` after provisioning.\n"] pub fn verify_config (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElVerifyConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.verify_config" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElDynamic { phase_configs : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl { # [serde (skip_serializing_if = "Option::is_none")] phase_configs : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl {
    #[doc = "Set the field `phase_configs`.\n"]
    pub fn set_phase_configs(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.phase_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.phase_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl { phase_configs : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElRef { shared : shared , base : base . to_string () , } } }
impl
    ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `phase_configs` after provisioning.\n"]    pub fn phase_configs (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElPhaseConfigsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.phase_configs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_traffic_control: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    canary_revision_tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prior_revision_tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stable_revision_tags: Option<ListField<PrimField<String>>>,
}
impl
    ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl
{
    #[doc = "Set the field `automatic_traffic_control`.\nWhether Cloud Deploy should update the traffic stanza in a Cloud Run Service on the user's behalf to facilitate traffic splitting. This is required to be true for CanaryDeployments, but optional for CustomCanaryDeployments."]
    pub fn set_automatic_traffic_control(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.automatic_traffic_control = Some(v.into());
        self
    }
    #[doc = "Set the field `canary_revision_tags`.\nOptional. A list of tags that are added to the canary revision while the canary phase is in progress."]
    pub fn set_canary_revision_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.canary_revision_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `prior_revision_tags`.\nOptional. A list of tags that are added to the prior revision while the canary phase is in progress."]
    pub fn set_prior_revision_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.prior_revision_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `stable_revision_tags`.\nOptional. A list of tags that are added to the final stable revision when the stable phase is applied."]
    pub fn set_stable_revision_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.stable_revision_tags = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl { automatic_traffic_control : core :: default :: Default :: default () , canary_revision_tags : core :: default :: Default :: default () , prior_revision_tags : core :: default :: Default :: default () , stable_revision_tags : core :: default :: Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `automatic_traffic_control` after provisioning.\nWhether Cloud Deploy should update the traffic stanza in a Cloud Run Service on the user's behalf to facilitate traffic splitting. This is required to be true for CanaryDeployments, but optional for CustomCanaryDeployments."] pub fn automatic_traffic_control (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.automatic_traffic_control" , self . base)) } # [doc = "Get a reference to the value of field `canary_revision_tags` after provisioning.\nOptional. A list of tags that are added to the canary revision while the canary phase is in progress."] pub fn canary_revision_tags (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.canary_revision_tags" , self . base)) } # [doc = "Get a reference to the value of field `prior_revision_tags` after provisioning.\nOptional. A list of tags that are added to the prior revision while the canary phase is in progress."] pub fn prior_revision_tags (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.prior_revision_tags" , self . base)) } # [doc = "Get a reference to the value of field `stable_revision_tags` after provisioning.\nOptional. A list of tags that are added to the final stable revision when the stable phase is applied."] pub fn stable_revision_tags (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.stable_revision_tags" , self . base)) } }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRouteDestinationsEl
{
    destination_ids: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    propagate_service: Option<PrimField<bool>>,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRouteDestinationsEl { # [doc = "Set the field `propagate_service`.\nOptional. Whether to propagate the Kubernetes Service to the route destination clusters. The Service will always be deployed to the Target cluster even if the HTTPRoute is not. This option may be used to facilitiate successful DNS lookup in the route destination clusters. Can only be set to true if destinations are specified."] pub fn set_propagate_service (mut self , v : impl Into < PrimField < bool > >) -> Self { self . propagate_service = Some (v . into ()) ; self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRouteDestinationsEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRouteDestinationsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRouteDestinationsEl
{
    #[doc = "Required. The clusters where the Gateway API HTTPRoute resource will be deployed to. Valid entries include the associated entities IDs configured in the Target resource and \"@self\" to include the Target cluster."]
    pub destination_ids: ListField<PrimField<String>>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRouteDestinationsEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRouteDestinationsEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRouteDestinationsEl { destination_ids : self . destination_ids , propagate_service : core :: default :: Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRouteDestinationsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRouteDestinationsElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRouteDestinationsElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRouteDestinationsElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRouteDestinationsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `destination_ids` after provisioning.\nRequired. The clusters where the Gateway API HTTPRoute resource will be deployed to. Valid entries include the associated entities IDs configured in the Target resource and \"@self\" to include the Target cluster."] pub fn destination_ids (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.destination_ids" , self . base)) } # [doc = "Get a reference to the value of field `propagate_service` after provisioning.\nOptional. Whether to propagate the Kubernetes Service to the route destination clusters. The Service will always be deployed to the Target cluster even if the HTTPRoute is not. This option may be used to facilitiate successful DNS lookup in the route destination clusters. Can only be set to true if destinations are specified."] pub fn propagate_service (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.propagate_service" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElDynamic { route_destinations : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRouteDestinationsEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl { deployment : PrimField < String > , http_route : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] pod_selector_label : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] route_update_wait_time : Option < PrimField < String > > , service : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] stable_cutback_duration : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] route_destinations : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRouteDestinationsEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl { # [doc = "Set the field `pod_selector_label`.\nOptional. The label to use when selecting Pods for the Deployment and Service resources. This label must already be present in both resources."] pub fn set_pod_selector_label (mut self , v : impl Into < PrimField < String > >) -> Self { self . pod_selector_label = Some (v . into ()) ; self } # [doc = "Set the field `route_update_wait_time`.\nOptional. The time to wait for route updates to propagate. The maximum configurable time is 3 hours, in seconds format. If unspecified, there is no wait time."] pub fn set_route_update_wait_time (mut self , v : impl Into < PrimField < String > >) -> Self { self . route_update_wait_time = Some (v . into ()) ; self } # [doc = "Set the field `stable_cutback_duration`.\nOptional. The amount of time to migrate traffic back from the canary Service to the original Service during the stable phase deployment. If specified, must be between 15s and 3600s. If unspecified, there is no cutback time."] pub fn set_stable_cutback_duration (mut self , v : impl Into < PrimField < String > >) -> Self { self . stable_cutback_duration = Some (v . into ()) ; self } # [doc = "Set the field `route_destinations`.\n"] pub fn set_route_destinations (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRouteDestinationsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . route_destinations = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . route_destinations = Some (d) ; } } self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl
{
    #[doc = "Required. Name of the Kubernetes Deployment whose traffic is managed by the specified HTTPRoute and Service."]
    pub deployment: PrimField<String>,
    #[doc = "Required. Name of the Gateway API HTTPRoute."]
    pub http_route: PrimField<String>,
    #[doc = "Required. Name of the Kubernetes Service."]
    pub service: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl { deployment : self . deployment , http_route : self . http_route , pod_selector_label : core :: default :: Default :: default () , route_update_wait_time : core :: default :: Default :: default () , service : self . service , stable_cutback_duration : core :: default :: Default :: default () , route_destinations : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `deployment` after provisioning.\nRequired. Name of the Kubernetes Deployment whose traffic is managed by the specified HTTPRoute and Service."] pub fn deployment (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.deployment" , self . base)) } # [doc = "Get a reference to the value of field `http_route` after provisioning.\nRequired. Name of the Gateway API HTTPRoute."] pub fn http_route (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.http_route" , self . base)) } # [doc = "Get a reference to the value of field `pod_selector_label` after provisioning.\nOptional. The label to use when selecting Pods for the Deployment and Service resources. This label must already be present in both resources."] pub fn pod_selector_label (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pod_selector_label" , self . base)) } # [doc = "Get a reference to the value of field `route_update_wait_time` after provisioning.\nOptional. The time to wait for route updates to propagate. The maximum configurable time is 3 hours, in seconds format. If unspecified, there is no wait time."] pub fn route_update_wait_time (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.route_update_wait_time" , self . base)) } # [doc = "Get a reference to the value of field `service` after provisioning.\nRequired. Name of the Kubernetes Service."] pub fn service (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.service" , self . base)) } # [doc = "Get a reference to the value of field `stable_cutback_duration` after provisioning.\nOptional. The amount of time to migrate traffic back from the canary Service to the original Service during the stable phase deployment. If specified, must be between 15s and 3600s. If unspecified, there is no cutback time."] pub fn stable_cutback_duration (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.stable_cutback_duration" , self . base)) } # [doc = "Get a reference to the value of field `route_destinations` after provisioning.\n"] pub fn route_destinations (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRouteDestinationsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.route_destinations" , self . base)) } }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl
{
    deployment: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_pod_overprovisioning: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_selector_label: Option<PrimField<String>>,
    service: PrimField<String>,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl { # [doc = "Set the field `disable_pod_overprovisioning`.\nOptional. Whether to disable Pod overprovisioning. If Pod overprovisioning is disabled then Cloud Deploy will limit the number of total Pods used for the deployment strategy to the number of Pods the Deployment has on the cluster."] pub fn set_disable_pod_overprovisioning (mut self , v : impl Into < PrimField < bool > >) -> Self { self . disable_pod_overprovisioning = Some (v . into ()) ; self } # [doc = "Set the field `pod_selector_label`.\nOptional. The label to use when selecting Pods for the Deployment resource. This label must already be present in the Deployment."] pub fn set_pod_selector_label (mut self , v : impl Into < PrimField < String > >) -> Self { self . pod_selector_label = Some (v . into ()) ; self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl
{
    #[doc = "Required. Name of the Kubernetes Deployment whose traffic is managed by the specified Service."]
    pub deployment: PrimField<String>,
    #[doc = "Required. Name of the Kubernetes Service."]
    pub service: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl { deployment : self . deployment , disable_pod_overprovisioning : core :: default :: Default :: default () , pod_selector_label : core :: default :: Default :: default () , service : self . service , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `deployment` after provisioning.\nRequired. Name of the Kubernetes Deployment whose traffic is managed by the specified Service."] pub fn deployment (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.deployment" , self . base)) } # [doc = "Get a reference to the value of field `disable_pod_overprovisioning` after provisioning.\nOptional. Whether to disable Pod overprovisioning. If Pod overprovisioning is disabled then Cloud Deploy will limit the number of total Pods used for the deployment strategy to the number of Pods the Deployment has on the cluster."] pub fn disable_pod_overprovisioning (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.disable_pod_overprovisioning" , self . base)) } # [doc = "Get a reference to the value of field `pod_selector_label` after provisioning.\nOptional. The label to use when selecting Pods for the Deployment resource. This label must already be present in the Deployment."] pub fn pod_selector_label (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pod_selector_label" , self . base)) } # [doc = "Get a reference to the value of field `service` after provisioning.\nRequired. Name of the Kubernetes Service."] pub fn service (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.service" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElDynamic { gateway_service_mesh : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl >> , service_networking : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl { # [serde (skip_serializing_if = "Option::is_none")] gateway_service_mesh : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl > > , # [serde (skip_serializing_if = "Option::is_none")] service_networking : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElDynamic , }
impl
    ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl
{
    #[doc = "Set the field `gateway_service_mesh`.\n"]
    pub fn set_gateway_service_mesh(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gateway_service_mesh = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gateway_service_mesh = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_networking`.\n"]
    pub fn set_service_networking(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_networking = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_networking = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl { gateway_service_mesh : core :: default :: Default :: default () , service_networking : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `gateway_service_mesh` after provisioning.\n"] pub fn gateway_service_mesh (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElGatewayServiceMeshElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.gateway_service_mesh" , self . base)) } # [doc = "Get a reference to the value of field `service_networking` after provisioning.\n"] pub fn service_networking (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElServiceNetworkingElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.service_networking" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElDynamic { cloud_run : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl >> , kubernetes : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl { # [serde (skip_serializing_if = "Option::is_none")] cloud_run : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl > > , # [serde (skip_serializing_if = "Option::is_none")] kubernetes : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl {
    #[doc = "Set the field `cloud_run`.\n"]
    pub fn set_cloud_run(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_run = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_run = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kubernetes`.\n"]
    pub fn set_kubernetes(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kubernetes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kubernetes = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl
{
    type O = BlockAssignable<
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl {
            cloud_run: core::default::Default::default(),
            kubernetes: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElRef
    {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_run` after provisioning.\n"]    pub fn cloud_run (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElCloudRunElRef >{
        ListRef::new(self.shared().clone(), format!("{}.cloud_run", self.base))
    }
    #[doc = "Get a reference to the value of field `kubernetes` after provisioning.\n"]    pub fn kubernetes (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElKubernetesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.kubernetes", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElDynamic { canary_deployment : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl >> , custom_canary_deployment : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl >> , runtime_config : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl { # [serde (skip_serializing_if = "Option::is_none")] canary_deployment : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl > > , # [serde (skip_serializing_if = "Option::is_none")] custom_canary_deployment : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl > > , # [serde (skip_serializing_if = "Option::is_none")] runtime_config : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl {
    #[doc = "Set the field `canary_deployment`.\n"]
    pub fn set_canary_deployment(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.canary_deployment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.canary_deployment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `custom_canary_deployment`.\n"]
    pub fn set_custom_canary_deployment(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_canary_deployment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_canary_deployment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `runtime_config`.\n"]
    pub fn set_runtime_config(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.runtime_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.runtime_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl {
    type O = BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl {}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl {
            canary_deployment: core::default::Default::default(),
            custom_canary_deployment: core::default::Default::default(),
            runtime_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `canary_deployment` after provisioning.\n"]
    pub fn canary_deployment(
        &self,
    ) -> ListRef<
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCanaryDeploymentElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.canary_deployment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_canary_deployment` after provisioning.\n"]    pub fn custom_canary_deployment (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElCustomCanaryDeploymentElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_canary_deployment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_config` after provisioning.\n"]
    pub fn runtime_config(
        &self,
    ) -> ListRef<
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRuntimeConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.runtime_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElContainerEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<RecField<PrimField<String>>>,
    image: PrimField<String>,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElContainerEl { # [doc = "Set the field `args`.\nOptional. Args is the container arguments to use. This overrides the default arguments defined in the container image."] pub fn set_args (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . args = Some (v . into ()) ; self } # [doc = "Set the field `command`.\nOptional. Command is the container entrypoint to use. This overrides the default entrypoint defined in the container image."] pub fn set_command (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . command = Some (v . into ()) ; self } # [doc = "Set the field `env`.\nOptional. Environment variables that are set in the container."] pub fn set_env (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . env = Some (v . into ()) ; self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElContainerEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElContainerEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElContainerEl
{
    #[doc = "Required. Image is the container image to use."]
    pub image: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElContainerEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElContainerEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElContainerEl { args : core :: default :: Default :: default () , command : core :: default :: Default :: default () , env : core :: default :: Default :: default () , image : self . image , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElContainerElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElContainerElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElContainerElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElContainerElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElContainerElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `args` after provisioning.\nOptional. Args is the container arguments to use. This overrides the default arguments defined in the container image."] pub fn args (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.args" , self . base)) } # [doc = "Get a reference to the value of field `command` after provisioning.\nOptional. Command is the container entrypoint to use. This overrides the default entrypoint defined in the container image."] pub fn command (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.command" , self . base)) } # [doc = "Get a reference to the value of field `env` after provisioning.\nOptional. Environment variables that are set in the container."] pub fn env (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.env" , self . base)) } # [doc = "Get a reference to the value of field `image` after provisioning.\nRequired. Image is the container image to use."] pub fn image (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElDynamic { container : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElContainerEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskEl { # [serde (skip_serializing_if = "Option::is_none")] container : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElContainerEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskEl { # [doc = "Set the field `container`.\n"] pub fn set_container (mut self , v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElContainerEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . container = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . container = Some (d) ; } } self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskEl { container : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `container` after provisioning.\n"] pub fn container (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElContainerElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.container" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElDynamic { task : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksEl { # [serde (skip_serializing_if = "Option::is_none")] frequency : Option < PrimField < String > > , id : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] task : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElDynamic , }
impl
    ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksEl
{
    #[doc = "Set the field `frequency`.\nOptional. Frequency of the custom check."]
    pub fn set_frequency(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.frequency = Some(v.into());
        self
    }
    #[doc = "Set the field `task`.\n"]
    pub fn set_task(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.task = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.task = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksEl
{
    #[doc = "Required. Unique identifier for the custom check."]
    pub id: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksEl { frequency : core :: default :: Default :: default () , id : self . id , task : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `frequency` after provisioning.\nOptional. Frequency of the custom check."] pub fn frequency (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.frequency" , self . base)) } # [doc = "Get a reference to the value of field `id` after provisioning.\nRequired. Unique identifier for the custom check."] pub fn id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.id" , self . base)) } # [doc = "Get a reference to the value of field `task` after provisioning.\n"] pub fn task (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElTaskElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.task" , self . base)) } }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElAlertPolicyChecksEl
{
    alert_policies: ListField<PrimField<String>>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElAlertPolicyChecksEl { # [doc = "Set the field `labels`.\nOptional. Labels to filter the alert policies."] pub fn set_labels (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . labels = Some (v . into ()) ; self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElAlertPolicyChecksEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElAlertPolicyChecksEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElAlertPolicyChecksEl
{
    #[doc = "Required. The list of alert policy names to check. Format: `projects/{project}/alertPolicies/{alert_policy}`."]
    pub alert_policies: ListField<PrimField<String>>,
    #[doc = "Required. Unique identifier for the alert policy check."]
    pub id: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElAlertPolicyChecksEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElAlertPolicyChecksEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElAlertPolicyChecksEl { alert_policies : self . alert_policies , id : self . id , labels : core :: default :: Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElAlertPolicyChecksElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElAlertPolicyChecksElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElAlertPolicyChecksElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElAlertPolicyChecksElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElAlertPolicyChecksElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `alert_policies` after provisioning.\nRequired. The list of alert policy names to check. Format: `projects/{project}/alertPolicies/{alert_policy}`."] pub fn alert_policies (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.alert_policies" , self . base)) } # [doc = "Get a reference to the value of field `id` after provisioning.\nRequired. Unique identifier for the alert policy check."] pub fn id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.id" , self . base)) } # [doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels to filter the alert policies."] pub fn labels (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.labels" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElDynamic { alert_policy_checks : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElAlertPolicyChecksEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudEl { # [serde (skip_serializing_if = "Option::is_none")] alert_policy_checks : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElAlertPolicyChecksEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElDynamic , }
impl
    ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudEl
{
    #[doc = "Set the field `alert_policy_checks`.\n"]
    pub fn set_alert_policy_checks(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElAlertPolicyChecksEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.alert_policy_checks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.alert_policy_checks = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudEl { alert_policy_checks : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `alert_policy_checks` after provisioning.\n"] pub fn alert_policy_checks (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElAlertPolicyChecksElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.alert_policy_checks" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElDynamic { custom_checks : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksEl >> , google_cloud : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisEl { duration : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] custom_checks : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksEl > > , # [serde (skip_serializing_if = "Option::is_none")] google_cloud : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisEl {
    #[doc = "Set the field `custom_checks`.\n"]
    pub fn set_custom_checks(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_checks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_checks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `google_cloud`.\n"]
    pub fn set_google_cloud(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.google_cloud = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.google_cloud = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisEl
{
    type O = BlockAssignable<
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisEl {
    #[doc = "Required. Duration of the analysis."]
    pub duration: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisEl {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisEl {
            duration: self.duration,
            custom_checks: core::default::Default::default(),
            google_cloud: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\nRequired. Duration of the analysis."]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }
    #[doc = "Get a reference to the value of field `custom_checks` after provisioning.\n"]    pub fn custom_checks (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElCustomChecksElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_checks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `google_cloud` after provisioning.\n"]    pub fn google_cloud (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElGoogleCloudElRef >{
        ListRef::new(self.shared().clone(), format!("{}.google_cloud", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElContainerEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<RecField<PrimField<String>>>,
    image: PrimField<String>,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElContainerEl { # [doc = "Set the field `args`.\nOptional. Args is the container arguments to use. This overrides the default arguments defined in the container image."] pub fn set_args (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . args = Some (v . into ()) ; self } # [doc = "Set the field `command`.\nOptional. Command is the container entrypoint to use. This overrides the default entrypoint defined in the container image."] pub fn set_command (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . command = Some (v . into ()) ; self } # [doc = "Set the field `env`.\nOptional. Environment variables that are set in the container."] pub fn set_env (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . env = Some (v . into ()) ; self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElContainerEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElContainerEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElContainerEl
{
    #[doc = "Required. Image is the container image to use."]
    pub image: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElContainerEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElContainerEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElContainerEl { args : core :: default :: Default :: default () , command : core :: default :: Default :: default () , env : core :: default :: Default :: default () , image : self . image , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElContainerElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElContainerElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElContainerElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElContainerElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElContainerElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `args` after provisioning.\nOptional. Args is the container arguments to use. This overrides the default arguments defined in the container image."] pub fn args (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.args" , self . base)) } # [doc = "Get a reference to the value of field `command` after provisioning.\nOptional. Command is the container entrypoint to use. This overrides the default entrypoint defined in the container image."] pub fn command (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.command" , self . base)) } # [doc = "Get a reference to the value of field `env` after provisioning.\nOptional. Environment variables that are set in the container."] pub fn env (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.env" , self . base)) } # [doc = "Get a reference to the value of field `image` after provisioning.\nRequired. Image is the container image to use."] pub fn image (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElDynamic { container : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElContainerEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksEl { # [serde (skip_serializing_if = "Option::is_none")] container : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElContainerEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksEl {
    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElContainerEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.container = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.container = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksEl
{
    type O = BlockAssignable<
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksEl
{}
impl
    BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksEl
{
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksEl
    {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksEl {
            container: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]    pub fn container (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElContainerElRef >{
        ListRef::new(self.shared().clone(), format!("{}.container", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElDynamic { tasks : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl { # [serde (skip_serializing_if = "Option::is_none")] actions : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] tasks : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl {
    #[doc = "Set the field `actions`.\nOptional. A sequence of skaffold custom actions to invoke during execution of the postdeploy job."]
    pub fn set_actions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.actions = Some(v.into());
        self
    }
    #[doc = "Set the field `tasks`.\n"]
    pub fn set_tasks(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tasks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tasks = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl
{
    type O = BlockAssignable<
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl {
            actions: core::default::Default::default(),
            tasks: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElRef
    {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `actions` after provisioning.\nOptional. A sequence of skaffold custom actions to invoke during execution of the postdeploy job."]
    pub fn actions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }
    #[doc = "Get a reference to the value of field `tasks` after provisioning.\n"]    pub fn tasks (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElTasksElRef >{
        ListRef::new(self.shared().clone(), format!("{}.tasks", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElContainerEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<RecField<PrimField<String>>>,
    image: PrimField<String>,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElContainerEl { # [doc = "Set the field `args`.\nOptional. Args is the container arguments to use. This overrides the default arguments defined in the container image."] pub fn set_args (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . args = Some (v . into ()) ; self } # [doc = "Set the field `command`.\nOptional. Command is the container entrypoint to use. This overrides the default entrypoint defined in the container image."] pub fn set_command (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . command = Some (v . into ()) ; self } # [doc = "Set the field `env`.\nOptional. Environment variables that are set in the container."] pub fn set_env (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . env = Some (v . into ()) ; self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElContainerEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElContainerEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElContainerEl
{
    #[doc = "Required. Image is the container image to use."]
    pub image: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElContainerEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElContainerEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElContainerEl { args : core :: default :: Default :: default () , command : core :: default :: Default :: default () , env : core :: default :: Default :: default () , image : self . image , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElContainerElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElContainerElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElContainerElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElContainerElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElContainerElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `args` after provisioning.\nOptional. Args is the container arguments to use. This overrides the default arguments defined in the container image."] pub fn args (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.args" , self . base)) } # [doc = "Get a reference to the value of field `command` after provisioning.\nOptional. Command is the container entrypoint to use. This overrides the default entrypoint defined in the container image."] pub fn command (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.command" , self . base)) } # [doc = "Get a reference to the value of field `env` after provisioning.\nOptional. Environment variables that are set in the container."] pub fn env (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.env" , self . base)) } # [doc = "Get a reference to the value of field `image` after provisioning.\nRequired. Image is the container image to use."] pub fn image (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElDynamic { container : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElContainerEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksEl { # [serde (skip_serializing_if = "Option::is_none")] container : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElContainerEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksEl {
    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElContainerEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.container = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.container = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksEl
{
    type O = BlockAssignable<
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksEl
{}
impl
    BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksEl
{
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksEl
    {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksEl {
            container: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElRef
    {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElRef { shared : shared , base : base . to_string () , }
    }
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]    pub fn container (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElContainerElRef >{
        ListRef::new(self.shared().clone(), format!("{}.container", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElDynamic { tasks : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl { # [serde (skip_serializing_if = "Option::is_none")] actions : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] tasks : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl {
    #[doc = "Set the field `actions`.\nOptional. A sequence of skaffold custom actions to invoke during execution of the predeploy job."]
    pub fn set_actions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.actions = Some(v.into());
        self
    }
    #[doc = "Set the field `tasks`.\n"]
    pub fn set_tasks(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tasks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tasks = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl
{
    type O = BlockAssignable<
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl {
            actions: core::default::Default::default(),
            tasks: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `actions` after provisioning.\nOptional. A sequence of skaffold custom actions to invoke during execution of the predeploy job."]
    pub fn actions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }
    #[doc = "Get a reference to the value of field `tasks` after provisioning.\n"]    pub fn tasks (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElTasksElRef >{
        ListRef::new(self.shared().clone(), format!("{}.tasks", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElContainerEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<RecField<PrimField<String>>>,
    image: PrimField<String>,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElContainerEl { # [doc = "Set the field `args`.\nOptional. Args is the container arguments to use. This overrides the default arguments defined in the container image."] pub fn set_args (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . args = Some (v . into ()) ; self } # [doc = "Set the field `command`.\nOptional. Command is the container entrypoint to use. This overrides the default entrypoint defined in the container image."] pub fn set_command (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . command = Some (v . into ()) ; self } # [doc = "Set the field `env`.\nOptional. Environment variables that are set in the container."] pub fn set_env (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . env = Some (v . into ()) ; self } }
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElContainerEl { type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElContainerEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElContainerEl
{
    #[doc = "Required. Image is the container image to use."]
    pub image: PrimField<String>,
}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElContainerEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElContainerEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElContainerEl { args : core :: default :: Default :: default () , command : core :: default :: Default :: default () , env : core :: default :: Default :: default () , image : self . image , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElContainerElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElContainerElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElContainerElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElContainerElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElContainerElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `args` after provisioning.\nOptional. Args is the container arguments to use. This overrides the default arguments defined in the container image."] pub fn args (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.args" , self . base)) } # [doc = "Get a reference to the value of field `command` after provisioning.\nOptional. Command is the container entrypoint to use. This overrides the default entrypoint defined in the container image."] pub fn command (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.command" , self . base)) } # [doc = "Get a reference to the value of field `env` after provisioning.\nOptional. Environment variables that are set in the container."] pub fn env (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.env" , self . base)) } # [doc = "Get a reference to the value of field `image` after provisioning.\nRequired. Image is the container image to use."] pub fn image (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image" , self . base)) } }
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElDynamic { container : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElContainerEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksEl { # [serde (skip_serializing_if = "Option::is_none")] container : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElContainerEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksEl {
    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElContainerEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.container = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.container = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksEl
{
    type O = BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksEl { pub fn build (self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksEl { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksEl { container : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElRef { ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElRef { shared : shared , base : base . to_string () , } } }
impl
    ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]    pub fn container (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElContainerElRef >{
        ListRef::new(self.shared().clone(), format!("{}.container", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElDynamic { tasks : Option < DynamicBlock < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigEl { # [serde (skip_serializing_if = "Option::is_none")] tasks : Option < Vec < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksEl > > , dynamic : ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElDynamic , }
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigEl {
    #[doc = "Set the field `tasks`.\n"]
    pub fn set_tasks(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tasks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tasks = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigEl
{
    type O = BlockAssignable<
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigEl
{}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigEl {
    pub fn build(
        self,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigEl {
            tasks: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElRef
    {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tasks` after provisioning.\n"]    pub fn tasks (& self) -> ListRef < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElTasksElRef >{
        ListRef::new(self.shared().clone(), format!("{}.tasks", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElDynamic {
    analysis: Option<
        DynamicBlock<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisEl,
        >,
    >,
    postdeploy: Option<
        DynamicBlock<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl,
        >,
    >,
    predeploy: Option<
        DynamicBlock<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl,
        >,
    >,
    verify_config: Option<
        DynamicBlock<
            ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    verify: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    analysis: Option<
        Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    postdeploy: Option<
        Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    predeploy: Option<
        Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    verify_config: Option<
        Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigEl>,
    >,
    dynamic: ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElDynamic,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl {
    #[doc = "Set the field `verify`.\nWhether to verify a deployment."]
    pub fn set_verify(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.verify = Some(v.into());
        self
    }
    #[doc = "Set the field `analysis`.\n"]
    pub fn set_analysis(
        mut self,
        v: impl Into<
            BlockAssignable<
                ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.analysis = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.analysis = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `postdeploy`.\n"]
    pub fn set_postdeploy(
        mut self,
        v: impl Into<
            BlockAssignable<
                ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.postdeploy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.postdeploy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `predeploy`.\n"]
    pub fn set_predeploy(
        mut self,
        v: impl Into<
            BlockAssignable<
                ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.predeploy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.predeploy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `verify_config`.\n"]
    pub fn set_verify_config(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.verify_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.verify_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl {
    type O =
        BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl {}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl {
            verify: core::default::Default::default(),
            analysis: core::default::Default::default(),
            postdeploy: core::default::Default::default(),
            predeploy: core::default::Default::default(),
            verify_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `verify` after provisioning.\nWhether to verify a deployment."]
    pub fn verify(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.verify", self.base))
    }
    #[doc = "Get a reference to the value of field `analysis` after provisioning.\n"]
    pub fn analysis(
        &self,
    ) -> ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElAnalysisElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.analysis", self.base))
    }
    #[doc = "Get a reference to the value of field `postdeploy` after provisioning.\n"]
    pub fn postdeploy(
        &self,
    ) -> ListRef<
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPostdeployElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.postdeploy", self.base))
    }
    #[doc = "Get a reference to the value of field `predeploy` after provisioning.\n"]
    pub fn predeploy(
        &self,
    ) -> ListRef<
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElPredeployElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.predeploy", self.base))
    }
    #[doc = "Get a reference to the value of field `verify_config` after provisioning.\n"]
    pub fn verify_config(
        &self,
    ) -> ListRef<
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElVerifyConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.verify_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElDynamic {
    canary:
        Option<DynamicBlock<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl>>,
    standard: Option<
        DynamicBlock<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl>,
    >,
}
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    canary: Option<Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    standard: Option<Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl>>,
    dynamic: ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElDynamic,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl {
    #[doc = "Set the field `canary`.\n"]
    pub fn set_canary(
        mut self,
        v: impl Into<
            BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.canary = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.canary = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `standard`.\n"]
    pub fn set_standard(
        mut self,
        v: impl Into<
            BlockAssignable<
                ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.standard = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.standard = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl {
    type O = BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl {}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl {
            canary: core::default::Default::default(),
            standard: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `canary` after provisioning.\n"]
    pub fn canary(
        &self,
    ) -> ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElCanaryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.canary", self.base))
    }
    #[doc = "Get a reference to the value of field `standard` after provisioning.\n"]
    pub fn standard(
        &self,
    ) -> ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElStandardElRef> {
        ListRef::new(self.shared().clone(), format!("{}.standard", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElStagesElDynamic {
    deploy_parameters:
        Option<DynamicBlock<ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl>>,
    strategy: Option<DynamicBlock<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl>>,
}
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    profiles: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_parameters:
        Option<Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strategy: Option<Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl>>,
    dynamic: ClouddeployDeliveryPipelineSerialPipelineElStagesElDynamic,
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesEl {
    #[doc = "Set the field `profiles`.\nSkaffold profiles to use when rendering the manifest for this stage's `Target`."]
    pub fn set_profiles(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.profiles = Some(v.into());
        self
    }
    #[doc = "Set the field `target_id`.\nThe target_id to which this stage points. This field refers exclusively to the last segment of a target name. For example, this field would just be `my-target` (rather than `projects/project/locations/location/targets/my-target`). The location of the `Target` is inferred to be the same as the location of the `DeliveryPipeline` that contains this `Stage`."]
    pub fn set_target_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_id = Some(v.into());
        self
    }
    #[doc = "Set the field `deploy_parameters`.\n"]
    pub fn set_deploy_parameters(
        mut self,
        v: impl Into<
            BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.deploy_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.deploy_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `strategy`.\n"]
    pub fn set_strategy(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.strategy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.strategy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineElStagesEl {
    type O = BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineSerialPipelineElStagesEl {}
impl BuildClouddeployDeliveryPipelineSerialPipelineElStagesEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineSerialPipelineElStagesEl {
        ClouddeployDeliveryPipelineSerialPipelineElStagesEl {
            profiles: core::default::Default::default(),
            target_id: core::default::Default::default(),
            deploy_parameters: core::default::Default::default(),
            strategy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeliveryPipelineSerialPipelineElStagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElStagesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeliveryPipelineSerialPipelineElStagesElRef {
        ClouddeployDeliveryPipelineSerialPipelineElStagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeliveryPipelineSerialPipelineElStagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `profiles` after provisioning.\nSkaffold profiles to use when rendering the manifest for this stage's `Target`."]
    pub fn profiles(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.profiles", self.base))
    }
    #[doc = "Get a reference to the value of field `target_id` after provisioning.\nThe target_id to which this stage points. This field refers exclusively to the last segment of a target name. For example, this field would just be `my-target` (rather than `projects/project/locations/location/targets/my-target`). The location of the `Target` is inferred to be the same as the location of the `DeliveryPipeline` that contains this `Stage`."]
    pub fn target_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_id", self.base))
    }
    #[doc = "Get a reference to the value of field `deploy_parameters` after provisioning.\n"]
    pub fn deploy_parameters(
        &self,
    ) -> ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElDeployParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deploy_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `strategy` after provisioning.\n"]
    pub fn strategy(
        &self,
    ) -> ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElStrategyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.strategy", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployDeliveryPipelineSerialPipelineElDynamic {
    stages: Option<DynamicBlock<ClouddeployDeliveryPipelineSerialPipelineElStagesEl>>,
}
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineSerialPipelineEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    stages: Option<Vec<ClouddeployDeliveryPipelineSerialPipelineElStagesEl>>,
    dynamic: ClouddeployDeliveryPipelineSerialPipelineElDynamic,
}
impl ClouddeployDeliveryPipelineSerialPipelineEl {
    #[doc = "Set the field `stages`.\n"]
    pub fn set_stages(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineElStagesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.stages = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.stages = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployDeliveryPipelineSerialPipelineEl {
    type O = BlockAssignable<ClouddeployDeliveryPipelineSerialPipelineEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineSerialPipelineEl {}
impl BuildClouddeployDeliveryPipelineSerialPipelineEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineSerialPipelineEl {
        ClouddeployDeliveryPipelineSerialPipelineEl {
            stages: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeliveryPipelineSerialPipelineElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineSerialPipelineElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployDeliveryPipelineSerialPipelineElRef {
        ClouddeployDeliveryPipelineSerialPipelineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeliveryPipelineSerialPipelineElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `stages` after provisioning.\n"]
    pub fn stages(&self) -> ListRef<ClouddeployDeliveryPipelineSerialPipelineElStagesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stages", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeliveryPipelineTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ClouddeployDeliveryPipelineTimeoutsEl {
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
impl ToListMappable for ClouddeployDeliveryPipelineTimeoutsEl {
    type O = BlockAssignable<ClouddeployDeliveryPipelineTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeliveryPipelineTimeoutsEl {}
impl BuildClouddeployDeliveryPipelineTimeoutsEl {
    pub fn build(self) -> ClouddeployDeliveryPipelineTimeoutsEl {
        ClouddeployDeliveryPipelineTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployDeliveryPipelineTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeliveryPipelineTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployDeliveryPipelineTimeoutsElRef {
        ClouddeployDeliveryPipelineTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeliveryPipelineTimeoutsElRef {
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
struct ClouddeployDeliveryPipelineDynamic {
    serial_pipeline: Option<DynamicBlock<ClouddeployDeliveryPipelineSerialPipelineEl>>,
}
