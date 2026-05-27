use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudRunV2ServiceData {
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
    client: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_audiences: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_uri_disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iap_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoker_iam_disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_stage: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary_authorization: Option<Vec<CloudRunV2ServiceBinaryAuthorizationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_config: Option<Vec<CloudRunV2ServiceBuildConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_region_settings: Option<Vec<CloudRunV2ServiceMultiRegionSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling: Option<Vec<CloudRunV2ServiceScalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template: Option<Vec<CloudRunV2ServiceTemplateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudRunV2ServiceTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traffic: Option<Vec<CloudRunV2ServiceTrafficEl>>,
    dynamic: CloudRunV2ServiceDynamic,
}
struct CloudRunV2Service_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudRunV2ServiceData>,
}
#[derive(Clone)]
pub struct CloudRunV2Service(Rc<CloudRunV2Service_>);
impl CloudRunV2Service {
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
    #[doc = "Set the field `annotations`.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new resources.\nAll system annotations in v1 now have a corresponding field in v2 Service.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `client`.\nArbitrary identifier for the API client."]
    pub fn set_client(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().client = Some(v.into());
        self
    }
    #[doc = "Set the field `client_version`.\nArbitrary version identifier for the API client."]
    pub fn set_client_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().client_version = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_audiences`.\nOne or more custom audiences that you want this service to support. Specify each custom audience as the full URL in a string. The custom audiences are encoded in the token and used to authenticate requests.\nFor more information, see https://cloud.google.com/run/docs/configuring/custom-audiences."]
    pub fn set_custom_audiences(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().custom_audiences = Some(v.into());
        self
    }
    #[doc = "Set the field `default_uri_disabled`.\nDisables public resolution of the default URI of this service."]
    pub fn set_default_uri_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default_uri_disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\nWhether Terraform will be prevented from destroying the service. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the service,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the service will fail.\nWhen the field is set to false, deleting the service is allowed."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nUser-provided description of the Service. This field currently has a 512-character limit."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `iap_enabled`.\nUsed to enable/disable IAP for the cloud-run service."]
    pub fn set_iap_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().iap_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ingress`.\nProvides the ingress settings for this Service. On output, returns the currently observed ingress settings, or INGRESS_TRAFFIC_UNSPECIFIED if no revision is active. Possible values: [\"INGRESS_TRAFFIC_ALL\", \"INGRESS_TRAFFIC_INTERNAL_ONLY\", \"INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER\"]"]
    pub fn set_ingress(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ingress = Some(v.into());
        self
    }
    #[doc = "Set the field `invoker_iam_disabled`.\nDisables IAM permission check for run.routes.invoke for callers of this service. For more information, visit https://cloud.google.com/run/docs/securing/managing-access#invoker_check."]
    pub fn set_invoker_iam_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().invoker_iam_disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with  'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 Service.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `launch_stage`.\nThe launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA.\nIf no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features.\n\nFor example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values: [\"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn set_launch_stage(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().launch_stage = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `binary_authorization`.\n"]
    pub fn set_binary_authorization(
        self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceBinaryAuthorizationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().binary_authorization = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.binary_authorization = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `build_config`.\n"]
    pub fn set_build_config(
        self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceBuildConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().build_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.build_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `multi_region_settings`.\n"]
    pub fn set_multi_region_settings(
        self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceMultiRegionSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().multi_region_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.multi_region_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scaling`.\n"]
    pub fn set_scaling(self, v: impl Into<BlockAssignable<CloudRunV2ServiceScalingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scaling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scaling = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `template`.\n"]
    pub fn set_template(self, v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().template = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.template = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudRunV2ServiceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `traffic`.\n"]
    pub fn set_traffic(self, v: impl Into<BlockAssignable<CloudRunV2ServiceTrafficEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().traffic = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.traffic = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new resources.\nAll system annotations in v1 now have a corresponding field in v2 Service.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client` after provisioning.\nArbitrary identifier for the API client."]
    pub fn client(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_version` after provisioning.\nArbitrary version identifier for the API client."]
    pub fn client_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\nThe Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Service does not reach its Serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn conditions(&self) -> ListRef<CloudRunV2ServiceConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe creation time."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creator` after provisioning.\nEmail address of the authenticated creator."]
    pub fn creator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creator", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_audiences` after provisioning.\nOne or more custom audiences that you want this service to support. Specify each custom audience as the full URL in a string. The custom audiences are encoded in the token and used to authenticate requests.\nFor more information, see https://cloud.google.com/run/docs/configuring/custom-audiences."]
    pub fn custom_audiences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_audiences", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_uri_disabled` after provisioning.\nDisables public resolution of the default URI of this service."]
    pub fn default_uri_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_uri_disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nThe deletion time."]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the service. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the service,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the service will fail.\nWhen the field is set to false, deleting the service is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description of the Service. This field currently has a 512-character limit."]
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nA system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nFor a deleted resource, the time after which it will be permanently deleted."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nA number that monotonically increases every time the user modifies the desired state. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a string instead of an integer."]
    pub fn generation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iap_enabled` after provisioning.\nUsed to enable/disable IAP for the cloud-run service."]
    pub fn iap_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iap_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ingress` after provisioning.\nProvides the ingress settings for this Service. On output, returns the currently observed ingress settings, or INGRESS_TRAFFIC_UNSPECIFIED if no revision is active. Possible values: [\"INGRESS_TRAFFIC_ALL\", \"INGRESS_TRAFFIC_INTERNAL_ONLY\", \"INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER\"]"]
    pub fn ingress(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingress", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `invoker_iam_disabled` after provisioning.\nDisables IAM permission check for run.routes.invoke for callers of this service. For more information, visit https://cloud.google.com/run/docs/securing/managing-access#invoker_check."]
    pub fn invoker_iam_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.invoker_iam_disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with  'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 Service.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modifier` after provisioning.\nEmail address of the last authenticated modifier."]
    pub fn last_modifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `latest_created_revision` after provisioning.\nName of the last created revision. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn latest_created_revision(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_created_revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `latest_ready_revision` after provisioning.\nName of the latest revision that is serving traffic. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn latest_ready_revision(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_ready_revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `launch_stage` after provisioning.\nThe launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA.\nIf no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features.\n\nFor example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values: [\"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn launch_stage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_stage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the cloud run service"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Service."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `observed_generation` after provisioning.\nThe generation of this Service currently serving traffic. See comments in reconciling for additional information on reconciliation process in Cloud Run. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a string instead of an integer."]
    pub fn observed_generation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.observed_generation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nReturns true if the Service is currently being acted upon by the system to bring it into the desired state.\n\nWhen a new Service is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Service to the desired serving state. This process is called reconciliation. While reconciliation is in process, observedGeneration, latest_ready_revison, trafficStatuses, and uri will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the serving state matches the Service, or there was an error, and reconciliation failed. This state can be found in terminalCondition.state.\n\nIf reconciliation succeeded, the following fields will match: traffic and trafficStatuses, observedGeneration and generation, latestReadyRevision and latestCreatedRevision.\n\nIf reconciliation failed, trafficStatuses, observedGeneration, and latestReadyRevision will have the state of the last serving revision, or empty for newly created Services. Additional information on the failure can be found in terminalCondition and conditions."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this Service, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn terminal_condition(&self) -> ListRef<CloudRunV2ServiceTerminalConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.terminal_condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traffic_statuses` after provisioning.\nDetailed status information for corresponding traffic targets. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn traffic_statuses(&self) -> ListRef<CloudRunV2ServiceTrafficStatusesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.traffic_statuses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nServer assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe last-modified time."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe main URI in which this Service is serving traffic."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `urls` after provisioning.\nAll URLs serving traffic for this Service."]
    pub fn urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.urls", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> ListRef<CloudRunV2ServiceBinaryAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.binary_authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build_config` after provisioning.\n"]
    pub fn build_config(&self) -> ListRef<CloudRunV2ServiceBuildConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.build_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multi_region_settings` after provisioning.\n"]
    pub fn multi_region_settings(&self) -> ListRef<CloudRunV2ServiceMultiRegionSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multi_region_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scaling` after provisioning.\n"]
    pub fn scaling(&self) -> ListRef<CloudRunV2ServiceScalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> ListRef<CloudRunV2ServiceTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudRunV2ServiceTimeoutsElRef {
        CloudRunV2ServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traffic` after provisioning.\n"]
    pub fn traffic(&self) -> ListRef<CloudRunV2ServiceTrafficElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.traffic", self.extract_ref()),
        )
    }
}
impl Referable for CloudRunV2Service {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudRunV2Service {}
impl ToListMappable for CloudRunV2Service {
    type O = ListRef<CloudRunV2ServiceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudRunV2Service_ {
    fn extract_resource_type(&self) -> String {
        "google_cloud_run_v2_service".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudRunV2Service {
    pub tf_id: String,
    #[doc = "The location of the cloud run service"]
    pub location: PrimField<String>,
    #[doc = "Name of the Service."]
    pub name: PrimField<String>,
}
impl BuildCloudRunV2Service {
    pub fn build(self, stack: &mut Stack) -> CloudRunV2Service {
        let out = CloudRunV2Service(Rc::new(CloudRunV2Service_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudRunV2ServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                client: core::default::Default::default(),
                client_version: core::default::Default::default(),
                custom_audiences: core::default::Default::default(),
                default_uri_disabled: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                description: core::default::Default::default(),
                iap_enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                ingress: core::default::Default::default(),
                invoker_iam_disabled: core::default::Default::default(),
                labels: core::default::Default::default(),
                launch_stage: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                binary_authorization: core::default::Default::default(),
                build_config: core::default::Default::default(),
                multi_region_settings: core::default::Default::default(),
                scaling: core::default::Default::default(),
                template: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                traffic: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudRunV2ServiceRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudRunV2ServiceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected in new resources.\nAll system annotations in v1 now have a corresponding field in v2 Service.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client` after provisioning.\nArbitrary identifier for the API client."]
    pub fn client(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_version` after provisioning.\nArbitrary version identifier for the API client."]
    pub fn client_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\nThe Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Service does not reach its Serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn conditions(&self) -> ListRef<CloudRunV2ServiceConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe creation time."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creator` after provisioning.\nEmail address of the authenticated creator."]
    pub fn creator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creator", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_audiences` after provisioning.\nOne or more custom audiences that you want this service to support. Specify each custom audience as the full URL in a string. The custom audiences are encoded in the token and used to authenticate requests.\nFor more information, see https://cloud.google.com/run/docs/configuring/custom-audiences."]
    pub fn custom_audiences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_audiences", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_uri_disabled` after provisioning.\nDisables public resolution of the default URI of this service."]
    pub fn default_uri_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_uri_disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nThe deletion time."]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the service. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the service,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the service will fail.\nWhen the field is set to false, deleting the service is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description of the Service. This field currently has a 512-character limit."]
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nA system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nFor a deleted resource, the time after which it will be permanently deleted."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nA number that monotonically increases every time the user modifies the desired state. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a string instead of an integer."]
    pub fn generation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iap_enabled` after provisioning.\nUsed to enable/disable IAP for the cloud-run service."]
    pub fn iap_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iap_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ingress` after provisioning.\nProvides the ingress settings for this Service. On output, returns the currently observed ingress settings, or INGRESS_TRAFFIC_UNSPECIFIED if no revision is active. Possible values: [\"INGRESS_TRAFFIC_ALL\", \"INGRESS_TRAFFIC_INTERNAL_ONLY\", \"INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER\"]"]
    pub fn ingress(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingress", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `invoker_iam_disabled` after provisioning.\nDisables IAM permission check for run.routes.invoke for callers of this service. For more information, visit https://cloud.google.com/run/docs/securing/managing-access#invoker_check."]
    pub fn invoker_iam_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.invoker_iam_disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component,\nenvironment, state, etc. For more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with  'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 Service.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modifier` after provisioning.\nEmail address of the last authenticated modifier."]
    pub fn last_modifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `latest_created_revision` after provisioning.\nName of the last created revision. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn latest_created_revision(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_created_revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `latest_ready_revision` after provisioning.\nName of the latest revision that is serving traffic. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn latest_ready_revision(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_ready_revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `launch_stage` after provisioning.\nThe launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA.\nIf no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features.\n\nFor example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values: [\"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn launch_stage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_stage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the cloud run service"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Service."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `observed_generation` after provisioning.\nThe generation of this Service currently serving traffic. See comments in reconciling for additional information on reconciliation process in Cloud Run. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a string instead of an integer."]
    pub fn observed_generation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.observed_generation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nReturns true if the Service is currently being acted upon by the system to bring it into the desired state.\n\nWhen a new Service is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Service to the desired serving state. This process is called reconciliation. While reconciliation is in process, observedGeneration, latest_ready_revison, trafficStatuses, and uri will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the serving state matches the Service, or there was an error, and reconciliation failed. This state can be found in terminalCondition.state.\n\nIf reconciliation succeeded, the following fields will match: traffic and trafficStatuses, observedGeneration and generation, latestReadyRevision and latestCreatedRevision.\n\nIf reconciliation failed, trafficStatuses, observedGeneration, and latestReadyRevision will have the state of the last serving revision, or empty for newly created Services. Additional information on the failure can be found in terminalCondition and conditions."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terminal_condition` after provisioning.\nThe Condition of this Service, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn terminal_condition(&self) -> ListRef<CloudRunV2ServiceTerminalConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.terminal_condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traffic_statuses` after provisioning.\nDetailed status information for corresponding traffic targets. See comments in reconciling for additional information on reconciliation process in Cloud Run."]
    pub fn traffic_statuses(&self) -> ListRef<CloudRunV2ServiceTrafficStatusesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.traffic_statuses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nServer assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe last-modified time."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe main URI in which this Service is serving traffic."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `urls` after provisioning.\nAll URLs serving traffic for this Service."]
    pub fn urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.urls", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> ListRef<CloudRunV2ServiceBinaryAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.binary_authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build_config` after provisioning.\n"]
    pub fn build_config(&self) -> ListRef<CloudRunV2ServiceBuildConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.build_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multi_region_settings` after provisioning.\n"]
    pub fn multi_region_settings(&self) -> ListRef<CloudRunV2ServiceMultiRegionSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multi_region_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scaling` after provisioning.\n"]
    pub fn scaling(&self) -> ListRef<CloudRunV2ServiceScalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> ListRef<CloudRunV2ServiceTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudRunV2ServiceTimeoutsElRef {
        CloudRunV2ServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traffic` after provisioning.\n"]
    pub fn traffic(&self) -> ListRef<CloudRunV2ServiceTrafficElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.traffic", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_transition_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl CloudRunV2ServiceConditionsEl {
    #[doc = "Set the field `execution_reason`.\n"]
    pub fn set_execution_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_reason = Some(v.into());
        self
    }
    #[doc = "Set the field `last_transition_time`.\n"]
    pub fn set_last_transition_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_transition_time = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reason = Some(v.into());
        self
    }
    #[doc = "Set the field `revision_reason`.\n"]
    pub fn set_revision_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revision_reason = Some(v.into());
        self
    }
    #[doc = "Set the field `severity`.\n"]
    pub fn set_severity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.severity = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceConditionsEl {
    type O = BlockAssignable<CloudRunV2ServiceConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceConditionsEl {}
impl BuildCloudRunV2ServiceConditionsEl {
    pub fn build(self) -> CloudRunV2ServiceConditionsEl {
        CloudRunV2ServiceConditionsEl {
            execution_reason: core::default::Default::default(),
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            revision_reason: core::default::Default::default(),
            severity: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceConditionsElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceConditionsElRef {
        CloudRunV2ServiceConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `execution_reason` after provisioning.\n"]
    pub fn execution_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_reason", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_transition_time` after provisioning.\n"]
    pub fn last_transition_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_transition_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
    #[doc = "Get a reference to the value of field `revision_reason` after provisioning.\n"]
    pub fn revision_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision_reason", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `severity` after provisioning.\n"]
    pub fn severity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTerminalConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_transition_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl CloudRunV2ServiceTerminalConditionEl {
    #[doc = "Set the field `execution_reason`.\n"]
    pub fn set_execution_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_reason = Some(v.into());
        self
    }
    #[doc = "Set the field `last_transition_time`.\n"]
    pub fn set_last_transition_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_transition_time = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reason = Some(v.into());
        self
    }
    #[doc = "Set the field `revision_reason`.\n"]
    pub fn set_revision_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revision_reason = Some(v.into());
        self
    }
    #[doc = "Set the field `severity`.\n"]
    pub fn set_severity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.severity = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTerminalConditionEl {
    type O = BlockAssignable<CloudRunV2ServiceTerminalConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTerminalConditionEl {}
impl BuildCloudRunV2ServiceTerminalConditionEl {
    pub fn build(self) -> CloudRunV2ServiceTerminalConditionEl {
        CloudRunV2ServiceTerminalConditionEl {
            execution_reason: core::default::Default::default(),
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            revision_reason: core::default::Default::default(),
            severity: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTerminalConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTerminalConditionElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceTerminalConditionElRef {
        CloudRunV2ServiceTerminalConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTerminalConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `execution_reason` after provisioning.\n"]
    pub fn execution_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_reason", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_transition_time` after provisioning.\n"]
    pub fn last_transition_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_transition_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
    #[doc = "Get a reference to the value of field `revision_reason` after provisioning.\n"]
    pub fn revision_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision_reason", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `severity` after provisioning.\n"]
    pub fn severity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTrafficStatusesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl CloudRunV2ServiceTrafficStatusesEl {
    #[doc = "Set the field `percent`.\n"]
    pub fn set_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percent = Some(v.into());
        self
    }
    #[doc = "Set the field `revision`.\n"]
    pub fn set_revision(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revision = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTrafficStatusesEl {
    type O = BlockAssignable<CloudRunV2ServiceTrafficStatusesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTrafficStatusesEl {}
impl BuildCloudRunV2ServiceTrafficStatusesEl {
    pub fn build(self) -> CloudRunV2ServiceTrafficStatusesEl {
        CloudRunV2ServiceTrafficStatusesEl {
            percent: core::default::Default::default(),
            revision: core::default::Default::default(),
            tag: core::default::Default::default(),
            type_: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTrafficStatusesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTrafficStatusesElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceTrafficStatusesElRef {
        CloudRunV2ServiceTrafficStatusesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTrafficStatusesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `percent` after provisioning.\n"]
    pub fn percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceBinaryAuthorizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    breakglass_justification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_default: Option<PrimField<bool>>,
}
impl CloudRunV2ServiceBinaryAuthorizationEl {
    #[doc = "Set the field `breakglass_justification`.\nIf present, indicates to use Breakglass using this justification. If useDefault is False, then it must be empty. For more information on breakglass, see https://cloud.google.com/binary-authorization/docs/using-breakglass"]
    pub fn set_breakglass_justification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.breakglass_justification = Some(v.into());
        self
    }
    #[doc = "Set the field `policy`.\nThe path to a binary authorization policy. Format: projects/{project}/platforms/cloudRun/{policy-name}"]
    pub fn set_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy = Some(v.into());
        self
    }
    #[doc = "Set the field `use_default`.\nIf True, indicates to use the default project's binary authorization policy. If False, binary authorization will be disabled."]
    pub fn set_use_default(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_default = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceBinaryAuthorizationEl {
    type O = BlockAssignable<CloudRunV2ServiceBinaryAuthorizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceBinaryAuthorizationEl {}
impl BuildCloudRunV2ServiceBinaryAuthorizationEl {
    pub fn build(self) -> CloudRunV2ServiceBinaryAuthorizationEl {
        CloudRunV2ServiceBinaryAuthorizationEl {
            breakglass_justification: core::default::Default::default(),
            policy: core::default::Default::default(),
            use_default: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceBinaryAuthorizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceBinaryAuthorizationElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceBinaryAuthorizationElRef {
        CloudRunV2ServiceBinaryAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceBinaryAuthorizationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `breakglass_justification` after provisioning.\nIf present, indicates to use Breakglass using this justification. If useDefault is False, then it must be empty. For more information on breakglass, see https://cloud.google.com/binary-authorization/docs/using-breakglass"]
    pub fn breakglass_justification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.breakglass_justification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\nThe path to a binary authorization policy. Format: projects/{project}/platforms/cloudRun/{policy-name}"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
    #[doc = "Get a reference to the value of field `use_default` after provisioning.\nIf True, indicates to use the default project's binary authorization policy. If False, binary authorization will be disabled."]
    pub fn use_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_default", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceBuildConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base_image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_automatic_updates: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_target: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_pool: Option<PrimField<String>>,
}
impl CloudRunV2ServiceBuildConfigEl {
    #[doc = "Set the field `base_image`.\nThe base image used to build the function."]
    pub fn set_base_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.base_image = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_automatic_updates`.\nSets whether the function will receive automatic base image updates."]
    pub fn set_enable_automatic_updates(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_automatic_updates = Some(v.into());
        self
    }
    #[doc = "Set the field `environment_variables`.\nUser-provided build-time environment variables for the function."]
    pub fn set_environment_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.environment_variables = Some(v.into());
        self
    }
    #[doc = "Set the field `function_target`.\nThe name of the function (as defined in source code) that will be executed. Defaults to the resource name suffix, if not specified. For backward compatibility, if function with given name is not found, then the system will try to use function named \"function\"."]
    pub fn set_function_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.function_target = Some(v.into());
        self
    }
    #[doc = "Set the field `image_uri`.\nArtifact Registry URI to store the built image."]
    pub fn set_image_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nService account to be used for building the container. The format of this field is 'projects/{projectId}/serviceAccounts/{serviceAccountEmail}'."]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `source_location`.\nThe Cloud Storage bucket URI where the function source code is located."]
    pub fn set_source_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_location = Some(v.into());
        self
    }
    #[doc = "Set the field `worker_pool`.\nName of the Cloud Build Custom Worker Pool that should be used to build the Cloud Run function. The format of this field is 'projects/{project}/locations/{region}/workerPools/{workerPool}' where {project} and {region} are the project id and region respectively where the worker pool is defined and {workerPool} is the short name of the worker pool."]
    pub fn set_worker_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.worker_pool = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceBuildConfigEl {
    type O = BlockAssignable<CloudRunV2ServiceBuildConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceBuildConfigEl {}
impl BuildCloudRunV2ServiceBuildConfigEl {
    pub fn build(self) -> CloudRunV2ServiceBuildConfigEl {
        CloudRunV2ServiceBuildConfigEl {
            base_image: core::default::Default::default(),
            enable_automatic_updates: core::default::Default::default(),
            environment_variables: core::default::Default::default(),
            function_target: core::default::Default::default(),
            image_uri: core::default::Default::default(),
            service_account: core::default::Default::default(),
            source_location: core::default::Default::default(),
            worker_pool: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceBuildConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceBuildConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceBuildConfigElRef {
        CloudRunV2ServiceBuildConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceBuildConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `base_image` after provisioning.\nThe base image used to build the function."]
    pub fn base_image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_image", self.base))
    }
    #[doc = "Get a reference to the value of field `enable_automatic_updates` after provisioning.\nSets whether the function will receive automatic base image updates."]
    pub fn enable_automatic_updates(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_automatic_updates", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\nUser-provided build-time environment variables for the function."]
    pub fn environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.environment_variables", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `function_target` after provisioning.\nThe name of the function (as defined in source code) that will be executed. Defaults to the resource name suffix, if not specified. For backward compatibility, if function with given name is not found, then the system will try to use function named \"function\"."]
    pub fn function_target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.function_target", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\nArtifact Registry URI to store the built image."]
    pub fn image_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe Cloud Build name of the latest successful deployment of the function."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nService account to be used for building the container. The format of this field is 'projects/{projectId}/serviceAccounts/{serviceAccountEmail}'."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_location` after provisioning.\nThe Cloud Storage bucket URI where the function source code is located."]
    pub fn source_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_location", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `worker_pool` after provisioning.\nName of the Cloud Build Custom Worker Pool that should be used to build the Cloud Run function. The format of this field is 'projects/{project}/locations/{region}/workerPools/{workerPool}' where {project} and {region} are the project id and region respectively where the worker pool is defined and {workerPool} is the short name of the worker pool."]
    pub fn worker_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_pool", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceMultiRegionSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    regions: Option<ListField<PrimField<String>>>,
}
impl CloudRunV2ServiceMultiRegionSettingsEl {
    #[doc = "Set the field `regions`.\nThe list of regions to deploy the multi-region Service."]
    pub fn set_regions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.regions = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceMultiRegionSettingsEl {
    type O = BlockAssignable<CloudRunV2ServiceMultiRegionSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceMultiRegionSettingsEl {}
impl BuildCloudRunV2ServiceMultiRegionSettingsEl {
    pub fn build(self) -> CloudRunV2ServiceMultiRegionSettingsEl {
        CloudRunV2ServiceMultiRegionSettingsEl {
            regions: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceMultiRegionSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceMultiRegionSettingsElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceMultiRegionSettingsElRef {
        CloudRunV2ServiceMultiRegionSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceMultiRegionSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `multi_region_id` after provisioning.\nSystem-generated unique id for the multi-region Service."]
    pub fn multi_region_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multi_region_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `regions` after provisioning.\nThe list of regions to deploy the multi-region Service."]
    pub fn regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceScalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_mode: Option<PrimField<String>>,
}
impl CloudRunV2ServiceScalingEl {
    #[doc = "Set the field `manual_instance_count`.\nTotal instance count for the service in manual scaling mode. This number of instances is divided among all revisions with specified traffic based on the percent of traffic they are receiving."]
    pub fn set_manual_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.manual_instance_count = Some(v.into());
        self
    }
    #[doc = "Set the field `max_instance_count`.\nCombined maximum number of instances for all revisions receiving traffic."]
    pub fn set_max_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_instance_count = Some(v.into());
        self
    }
    #[doc = "Set the field `min_instance_count`.\nMinimum number of instances for the service, to be divided among all revisions receiving traffic."]
    pub fn set_min_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_instance_count = Some(v.into());
        self
    }
    #[doc = "Set the field `scaling_mode`.\nThe [scaling mode](https://cloud.google.com/run/docs/reference/rest/v2/projects.locations.services#scalingmode) for the service. Possible values: [\"AUTOMATIC\", \"MANUAL\"]"]
    pub fn set_scaling_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scaling_mode = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceScalingEl {
    type O = BlockAssignable<CloudRunV2ServiceScalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceScalingEl {}
impl BuildCloudRunV2ServiceScalingEl {
    pub fn build(self) -> CloudRunV2ServiceScalingEl {
        CloudRunV2ServiceScalingEl {
            manual_instance_count: core::default::Default::default(),
            max_instance_count: core::default::Default::default(),
            min_instance_count: core::default::Default::default(),
            scaling_mode: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceScalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceScalingElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceScalingElRef {
        CloudRunV2ServiceScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceScalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `manual_instance_count` after provisioning.\nTotal instance count for the service in manual scaling mode. This number of instances is divided among all revisions with specified traffic based on the percent of traffic they are receiving."]
    pub fn manual_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manual_instance_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_instance_count` after provisioning.\nCombined maximum number of instances for all revisions receiving traffic."]
    pub fn max_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instance_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_instance_count` after provisioning.\nMinimum number of instances for the service, to be divided among all revisions receiving traffic."]
    pub fn min_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_instance_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scaling_mode` after provisioning.\nThe [scaling mode](https://cloud.google.com/run/docs/reference/rest/v2/projects.locations.services#scalingmode) for the service. Possible values: [\"AUTOMATIC\", \"MANUAL\"]"]
    pub fn scaling_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scaling_mode", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElBuildInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    function_target: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_location: Option<PrimField<String>>,
}
impl CloudRunV2ServiceTemplateElContainersElBuildInfoEl {
    #[doc = "Set the field `function_target`.\n"]
    pub fn set_function_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.function_target = Some(v.into());
        self
    }
    #[doc = "Set the field `source_location`.\n"]
    pub fn set_source_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_location = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersElBuildInfoEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElContainersElBuildInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElBuildInfoEl {}
impl BuildCloudRunV2ServiceTemplateElContainersElBuildInfoEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElBuildInfoEl {
        CloudRunV2ServiceTemplateElContainersElBuildInfoEl {
            function_target: core::default::Default::default(),
            source_location: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElBuildInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElBuildInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElContainersElBuildInfoElRef {
        CloudRunV2ServiceTemplateElContainersElBuildInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElBuildInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `function_target` after provisioning.\n"]
    pub fn function_target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.function_target", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_location` after provisioning.\n"]
    pub fn source_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_location", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    secret: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    #[doc = "Set the field `version`.\nThe Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    type O =
        BlockAssignable<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    #[doc = "The name of the secret in Cloud Secret Manager. Format: {secretName} if the secret is in the same project. projects/{project}/secrets/{secretName} if the secret is in a different project."]
    pub secret: PrimField<String>,
}
impl BuildCloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
        CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl {
            secret: self.secret,
            version: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
        CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\nThe name of the secret in Cloud Secret Manager. Format: {secretName} if the secret is in the same project. projects/{project}/secrets/{secretName} if the secret is in a different project."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElDynamic {
    secret_key_ref: Option<
        DynamicBlock<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>,
    >,
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_key_ref:
        Option<Vec<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl>>,
    dynamic: CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElDynamic,
}
impl CloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
    #[doc = "Set the field `secret_key_ref`.\n"]
    pub fn set_secret_key_ref(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secret_key_ref = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_key_ref = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {}
impl BuildCloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
        CloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl {
            secret_key_ref: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
        CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_key_ref` after provisioning.\n"]
    pub fn secret_key_ref(
        &self,
    ) -> ListRef<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElSecretKeyRefElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_key_ref", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElContainersElEnvElDynamic {
    value_source: Option<DynamicBlock<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElEnvEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_source: Option<Vec<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl>>,
    dynamic: CloudRunV2ServiceTemplateElContainersElEnvElDynamic,
}
impl CloudRunV2ServiceTemplateElContainersElEnvEl {
    #[doc = "Set the field `value`.\nLiteral value of the environment variable. Defaults to \"\" and the maximum allowed length is 32768 characters. Variable references are not supported in Cloud Run."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
    #[doc = "Set the field `value_source`.\n"]
    pub fn set_value_source(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.value_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.value_source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersElEnvEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElContainersElEnvEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElEnvEl {
    #[doc = "Name of the environment variable. Must be a C_IDENTIFIER, and may not exceed 32768 characters."]
    pub name: PrimField<String>,
}
impl BuildCloudRunV2ServiceTemplateElContainersElEnvEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElEnvEl {
        CloudRunV2ServiceTemplateElContainersElEnvEl {
            name: self.name,
            value: core::default::Default::default(),
            value_source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElEnvElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElEnvElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceTemplateElContainersElEnvElRef {
        CloudRunV2ServiceTemplateElContainersElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the environment variable. Must be a C_IDENTIFIER, and may not exceed 32768 characters."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nLiteral value of the environment variable. Defaults to \"\" and the maximum allowed length is 32768 characters. Variable references are not supported in Cloud Run."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
    #[doc = "Get a reference to the value of field `value_source` after provisioning.\n"]
    pub fn value_source(
        &self,
    ) -> ListRef<CloudRunV2ServiceTemplateElContainersElEnvElValueSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.value_source", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
    #[doc = "Set the field `port`.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\nThe name of the service to place in the gRPC HealthCheckRequest\n(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {}
impl BuildCloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
        CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
        CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe name of the service to place in the gRPC HealthCheckRequest\n(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    #[doc = "Set the field `value`.\nThe header field value"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl
{
    type O = BlockAssignable<
        CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    #[doc = "The header field name"]
    pub name: PrimField<String>,
}
impl BuildCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
    pub fn build(
        self,
    ) -> CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
        CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl {
            name: self.name,
            value: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
        CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe header field name"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe header field value"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElDynamic {
    http_headers: Option<
        DynamicBlock<CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl>,
    >,
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_headers:
        Option<Vec<CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl>>,
    dynamic: CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElDynamic,
}
impl CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
    #[doc = "Set the field `path`.\nPath to access on the HTTP server. Defaults to '/'."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `http_headers`.\n"]
    pub fn set_http_headers(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_headers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_headers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {}
impl BuildCloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
        CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl {
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            http_headers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
        CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath to access on the HTTP server. Defaults to '/'."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]
    pub fn http_headers(
        &self,
    ) -> ListRef<CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElHttpHeadersElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl {
    port: PrimField<f64>,
}
impl CloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl {}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl {
    #[doc = "Port number to access on the container. Must be in the range 1 to 65535.\nIf not specified, defaults to the exposed port of the container, which\nis the value of container.ports[0].containerPort."]
    pub port: PrimField<f64>,
}
impl BuildCloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl {
        CloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl { port: self.port }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketElRef {
        CloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Must be in the range 1 to 65535.\nIf not specified, defaults to the exposed port of the container, which\nis the value of container.ports[0].containerPort."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElDynamic {
    grpc: Option<DynamicBlock<CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl>>,
    http_get: Option<DynamicBlock<CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl>>,
    tcp_socket:
        Option<DynamicBlock<CloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_delay_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc: Option<Vec<CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_get: Option<Vec<CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_socket: Option<Vec<CloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl>>,
    dynamic: CloudRunV2ServiceTemplateElContainersElLivenessProbeElDynamic,
}
impl CloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
    #[doc = "Set the field `failure_threshold`.\nMinimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1."]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_delay_seconds`.\nNumber of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub fn set_initial_delay_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_delay_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `period_seconds`.\nHow often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeoutSeconds"]
    pub fn set_period_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.period_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_seconds`.\nNumber of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than periodSeconds. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grpc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_get = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_get = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tcp_socket`.\n"]
    pub fn set_tcp_socket(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tcp_socket = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tcp_socket = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElContainersElLivenessProbeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElLivenessProbeEl {}
impl BuildCloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
        CloudRunV2ServiceTemplateElContainersElLivenessProbeEl {
            failure_threshold: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            tcp_socket: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
        CloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElLivenessProbeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\nMinimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1."]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nNumber of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub fn initial_delay_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_delay_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `period_seconds` after provisioning.\nHow often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeoutSeconds"]
    pub fn period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.period_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\nNumber of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than periodSeconds. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> ListRef<CloudRunV2ServiceTemplateElContainersElLivenessProbeElGrpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }
    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(
        &self,
    ) -> ListRef<CloudRunV2ServiceTemplateElContainersElLivenessProbeElHttpGetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }
    #[doc = "Get a reference to the value of field `tcp_socket` after provisioning.\n"]
    pub fn tcp_socket(
        &self,
    ) -> ListRef<CloudRunV2ServiceTemplateElContainersElLivenessProbeElTcpSocketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tcp_socket", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElPortsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl CloudRunV2ServiceTemplateElContainersElPortsEl {
    #[doc = "Set the field `container_port`.\nPort number the container listens on. This must be a valid TCP port number, 0 < containerPort < 65536."]
    pub fn set_container_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.container_port = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nIf specified, used to specify which protocol to use. Allowed values are \"http1\" and \"h2c\"."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersElPortsEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElContainersElPortsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElPortsEl {}
impl BuildCloudRunV2ServiceTemplateElContainersElPortsEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElPortsEl {
        CloudRunV2ServiceTemplateElContainersElPortsEl {
            container_port: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElPortsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElPortsElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceTemplateElContainersElPortsElRef {
        CloudRunV2ServiceTemplateElContainersElPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElPortsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_port` after provisioning.\nPort number the container listens on. This must be a valid TCP port number, 0 < containerPort < 65536."]
    pub fn container_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_port", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIf specified, used to specify which protocol to use. Allowed values are \"http1\" and \"h2c\"."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl CloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl {
    #[doc = "Set the field `port`.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\nThe name of the service to place in the gRPC HealthCheckRequest\n(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl {}
impl BuildCloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl {
        CloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcElRef {
        CloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe name of the service to place in the gRPC HealthCheckRequest\n(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl CloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl {
    #[doc = "Set the field `path`.\nPath to access on the HTTP server. If set, it should not be empty string."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl {}
impl BuildCloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl {
        CloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl {
            path: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetElRef {
        CloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath to access on the HTTP server. If set, it should not be empty string."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElContainersElReadinessProbeElDynamic {
    grpc: Option<DynamicBlock<CloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl>>,
    http_get:
        Option<DynamicBlock<CloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElReadinessProbeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc: Option<Vec<CloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_get: Option<Vec<CloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl>>,
    dynamic: CloudRunV2ServiceTemplateElContainersElReadinessProbeElDynamic,
}
impl CloudRunV2ServiceTemplateElContainersElReadinessProbeEl {
    #[doc = "Set the field `failure_threshold`.\nMinimum consecutive failures for the probe to be considered failed after\nhaving succeeded. Defaults to 3."]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `period_seconds`.\nHow often (in seconds) to perform the probe.\nDefault to 10 seconds."]
    pub fn set_period_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.period_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `success_threshold`.\nMinimum consecutive successes for the probe to be considered successful after having failed.\nDefaults to 2."]
    pub fn set_success_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.success_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_seconds`.\nNumber of seconds after which the probe times out.\nDefaults to 1 second. Must be smaller than period_seconds."]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grpc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_get = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_get = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersElReadinessProbeEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElContainersElReadinessProbeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElReadinessProbeEl {}
impl BuildCloudRunV2ServiceTemplateElContainersElReadinessProbeEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElReadinessProbeEl {
        CloudRunV2ServiceTemplateElContainersElReadinessProbeEl {
            failure_threshold: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            success_threshold: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElReadinessProbeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElReadinessProbeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElContainersElReadinessProbeElRef {
        CloudRunV2ServiceTemplateElContainersElReadinessProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElReadinessProbeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\nMinimum consecutive failures for the probe to be considered failed after\nhaving succeeded. Defaults to 3."]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `period_seconds` after provisioning.\nHow often (in seconds) to perform the probe.\nDefault to 10 seconds."]
    pub fn period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.period_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `success_threshold` after provisioning.\nMinimum consecutive successes for the probe to be considered successful after having failed.\nDefaults to 2."]
    pub fn success_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.success_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\nNumber of seconds after which the probe times out.\nDefaults to 1 second. Must be smaller than period_seconds."]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(
        &self,
    ) -> ListRef<CloudRunV2ServiceTemplateElContainersElReadinessProbeElGrpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }
    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(
        &self,
    ) -> ListRef<CloudRunV2ServiceTemplateElContainersElReadinessProbeElHttpGetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_idle: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limits: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    startup_cpu_boost: Option<PrimField<bool>>,
}
impl CloudRunV2ServiceTemplateElContainersElResourcesEl {
    #[doc = "Set the field `cpu_idle`.\nDetermines whether CPU is only allocated during requests. True by default if the parent 'resources' field is not set. However, if\n'resources' is set, this field must be explicitly set to true to preserve the default behavior."]
    pub fn set_cpu_idle(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cpu_idle = Some(v.into());
        self
    }
    #[doc = "Set the field `limits`.\nOnly memory, CPU, and nvidia.com/gpu are supported. Use key 'cpu' for CPU limit, 'memory' for memory limit, 'nvidia.com/gpu' for gpu limit. Note: The only supported values for CPU are '1', '2', '4', '6' and '8'. Setting 4 CPU requires at least 2Gi of memory, setting 6 or more CPU requires at least 4Gi of memory. The values of the map is string form of the 'quantity' k8s type: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go"]
    pub fn set_limits(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.limits = Some(v.into());
        self
    }
    #[doc = "Set the field `startup_cpu_boost`.\nDetermines whether CPU should be boosted on startup of a new container instance above the requested CPU threshold, this can help reduce cold-start latency."]
    pub fn set_startup_cpu_boost(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.startup_cpu_boost = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersElResourcesEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElContainersElResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElResourcesEl {}
impl BuildCloudRunV2ServiceTemplateElContainersElResourcesEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElResourcesEl {
        CloudRunV2ServiceTemplateElContainersElResourcesEl {
            cpu_idle: core::default::Default::default(),
            limits: core::default::Default::default(),
            startup_cpu_boost: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElContainersElResourcesElRef {
        CloudRunV2ServiceTemplateElContainersElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpu_idle` after provisioning.\nDetermines whether CPU is only allocated during requests. True by default if the parent 'resources' field is not set. However, if\n'resources' is set, this field must be explicitly set to true to preserve the default behavior."]
    pub fn cpu_idle(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_idle", self.base))
    }
    #[doc = "Get a reference to the value of field `limits` after provisioning.\nOnly memory, CPU, and nvidia.com/gpu are supported. Use key 'cpu' for CPU limit, 'memory' for memory limit, 'nvidia.com/gpu' for gpu limit. Note: The only supported values for CPU are '1', '2', '4', '6' and '8'. Setting 4 CPU requires at least 2Gi of memory, setting 6 or more CPU requires at least 4Gi of memory. The values of the map is string form of the 'quantity' k8s type: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go"]
    pub fn limits(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }
    #[doc = "Get a reference to the value of field `startup_cpu_boost` after provisioning.\nDetermines whether CPU should be boosted on startup of a new container instance above the requested CPU threshold, this can help reduce cold-start latency."]
    pub fn startup_cpu_boost(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.startup_cpu_boost", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
    #[doc = "Set the field `port`.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\nThe name of the service to place in the gRPC HealthCheckRequest\n(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {}
impl BuildCloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
        CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
        CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe name of the service to place in the gRPC HealthCheckRequest\n(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    #[doc = "Set the field `value`.\nThe header field value"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl
{
    type O = BlockAssignable<
        CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    #[doc = "The header field name"]
    pub name: PrimField<String>,
}
impl BuildCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
    pub fn build(
        self,
    ) -> CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
        CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl {
            name: self.name,
            value: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
        CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe header field name"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe header field value"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElDynamic {
    http_headers: Option<
        DynamicBlock<CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl>,
    >,
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_headers:
        Option<Vec<CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl>>,
    dynamic: CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElDynamic,
}
impl CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
    #[doc = "Set the field `path`.\nPath to access on the HTTP server. Defaults to '/'."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nPort number to access on the container. Must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `http_headers`.\n"]
    pub fn set_http_headers(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_headers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_headers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {}
impl BuildCloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
        CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl {
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            http_headers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
        CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath to access on the HTTP server. Defaults to '/'."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]
    pub fn http_headers(
        &self,
    ) -> ListRef<CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElHttpHeadersElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
    #[doc = "Set the field `port`.\nPort number to access on the container. Must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {}
impl BuildCloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
        CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl {
            port: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
        CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElContainersElStartupProbeElDynamic {
    grpc: Option<DynamicBlock<CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl>>,
    http_get: Option<DynamicBlock<CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl>>,
    tcp_socket:
        Option<DynamicBlock<CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_delay_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc: Option<Vec<CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_get: Option<Vec<CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_socket: Option<Vec<CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl>>,
    dynamic: CloudRunV2ServiceTemplateElContainersElStartupProbeElDynamic,
}
impl CloudRunV2ServiceTemplateElContainersElStartupProbeEl {
    #[doc = "Set the field `failure_threshold`.\nMinimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1."]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_delay_seconds`.\nNumber of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub fn set_initial_delay_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_delay_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `period_seconds`.\nHow often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeoutSeconds"]
    pub fn set_period_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.period_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_seconds`.\nNumber of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than periodSeconds. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grpc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_get = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_get = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tcp_socket`.\n"]
    pub fn set_tcp_socket(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tcp_socket = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tcp_socket = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersElStartupProbeEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElContainersElStartupProbeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElStartupProbeEl {}
impl BuildCloudRunV2ServiceTemplateElContainersElStartupProbeEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElStartupProbeEl {
        CloudRunV2ServiceTemplateElContainersElStartupProbeEl {
            failure_threshold: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            tcp_socket: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
        CloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElStartupProbeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\nMinimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1."]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nNumber of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub fn initial_delay_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_delay_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `period_seconds` after provisioning.\nHow often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeoutSeconds"]
    pub fn period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.period_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\nNumber of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than periodSeconds. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> ListRef<CloudRunV2ServiceTemplateElContainersElStartupProbeElGrpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }
    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(
        &self,
    ) -> ListRef<CloudRunV2ServiceTemplateElContainersElStartupProbeElHttpGetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }
    #[doc = "Get a reference to the value of field `tcp_socket` after provisioning.\n"]
    pub fn tcp_socket(
        &self,
    ) -> ListRef<CloudRunV2ServiceTemplateElContainersElStartupProbeElTcpSocketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tcp_socket", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
    mount_path: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_path: Option<PrimField<String>>,
}
impl CloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
    #[doc = "Set the field `sub_path`.\nPath within the volume from which the container's volume should be mounted."]
    pub fn set_sub_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sub_path = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElContainersElVolumeMountsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
    #[doc = "Path within the container at which the volume should be mounted. Must not contain ':'. For Cloud SQL volumes, it can be left empty, or must otherwise be /cloudsql. All instances defined in the Volume will be available as /cloudsql/[instance]. For more information on Cloud SQL volumes, visit https://cloud.google.com/sql/docs/mysql/connect-run"]
    pub mount_path: PrimField<String>,
    #[doc = "This must match the Name of a Volume."]
    pub name: PrimField<String>,
}
impl BuildCloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
        CloudRunV2ServiceTemplateElContainersElVolumeMountsEl {
            mount_path: self.mount_path,
            name: self.name,
            sub_path: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
        CloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElVolumeMountsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\nPath within the container at which the volume should be mounted. Must not contain ':'. For Cloud SQL volumes, it can be left empty, or must otherwise be /cloudsql. All instances defined in the Volume will be available as /cloudsql/[instance]. For more information on Cloud SQL volumes, visit https://cloud.google.com/sql/docs/mysql/connect-run"]
    pub fn mount_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThis must match the Name of a Volume."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `sub_path` after provisioning.\nPath within the volume from which the container's volume should be mounted."]
    pub fn sub_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sub_path", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElContainersElDynamic {
    env: Option<DynamicBlock<CloudRunV2ServiceTemplateElContainersElEnvEl>>,
    liveness_probe: Option<DynamicBlock<CloudRunV2ServiceTemplateElContainersElLivenessProbeEl>>,
    ports: Option<DynamicBlock<CloudRunV2ServiceTemplateElContainersElPortsEl>>,
    readiness_probe: Option<DynamicBlock<CloudRunV2ServiceTemplateElContainersElReadinessProbeEl>>,
    resources: Option<DynamicBlock<CloudRunV2ServiceTemplateElContainersElResourcesEl>>,
    startup_probe: Option<DynamicBlock<CloudRunV2ServiceTemplateElContainersElStartupProbeEl>>,
    volume_mounts: Option<DynamicBlock<CloudRunV2ServiceTemplateElContainersElVolumeMountsEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElContainersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_image_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    depends_on: Option<ListField<PrimField<String>>>,
    image: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Vec<CloudRunV2ServiceTemplateElContainersElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    liveness_probe: Option<Vec<CloudRunV2ServiceTemplateElContainersElLivenessProbeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<Vec<CloudRunV2ServiceTemplateElContainersElPortsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    readiness_probe: Option<Vec<CloudRunV2ServiceTemplateElContainersElReadinessProbeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<Vec<CloudRunV2ServiceTemplateElContainersElResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    startup_probe: Option<Vec<CloudRunV2ServiceTemplateElContainersElStartupProbeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_mounts: Option<Vec<CloudRunV2ServiceTemplateElContainersElVolumeMountsEl>>,
    dynamic: CloudRunV2ServiceTemplateElContainersElDynamic,
}
impl CloudRunV2ServiceTemplateElContainersEl {
    #[doc = "Set the field `args`.\nArguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references are not supported in Cloud Run."]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `base_image_uri`.\nBase image for this container. If set, it indicates that the service is enrolled into automatic base image update."]
    pub fn set_base_image_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.base_image_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `command`.\nEntrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell"]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }
    #[doc = "Set the field `depends_on`.\nContainers which should be started before this container. If specified the container will wait to start until all containers with the listed names are healthy."]
    pub fn set_depends_on(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.depends_on = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nName of the container specified as a DNS_LABEL."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `working_dir`.\nContainer's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image."]
    pub fn set_working_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.working_dir = Some(v.into());
        self
    }
    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElContainersElEnvEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.env = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.env = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `liveness_probe`.\n"]
    pub fn set_liveness_probe(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElContainersElLivenessProbeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.liveness_probe = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.liveness_probe = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElContainersElPortsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ports = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ports = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `readiness_probe`.\n"]
    pub fn set_readiness_probe(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElContainersElReadinessProbeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.readiness_probe = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.readiness_probe = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElContainersElResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `startup_probe`.\n"]
    pub fn set_startup_probe(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElContainersElStartupProbeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.startup_probe = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.startup_probe = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `volume_mounts`.\n"]
    pub fn set_volume_mounts(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElContainersElVolumeMountsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.volume_mounts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.volume_mounts = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElContainersEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElContainersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElContainersEl {
    #[doc = "URL of the Container image in Google Container Registry or Google Artifact Registry. More info: https://kubernetes.io/docs/concepts/containers/images"]
    pub image: PrimField<String>,
}
impl BuildCloudRunV2ServiceTemplateElContainersEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElContainersEl {
        CloudRunV2ServiceTemplateElContainersEl {
            args: core::default::Default::default(),
            base_image_uri: core::default::Default::default(),
            command: core::default::Default::default(),
            depends_on: core::default::Default::default(),
            image: self.image,
            name: core::default::Default::default(),
            working_dir: core::default::Default::default(),
            env: core::default::Default::default(),
            liveness_probe: core::default::Default::default(),
            ports: core::default::Default::default(),
            readiness_probe: core::default::Default::default(),
            resources: core::default::Default::default(),
            startup_probe: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElContainersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElContainersElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceTemplateElContainersElRef {
        CloudRunV2ServiceTemplateElContainersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElContainersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nArguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references are not supported in Cloud Run."]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `base_image_uri` after provisioning.\nBase image for this container. If set, it indicates that the service is enrolled into automatic base image update."]
    pub fn base_image_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_image_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `build_info` after provisioning.\nThe build info of the container image."]
    pub fn build_info(&self) -> ListRef<CloudRunV2ServiceTemplateElContainersElBuildInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.build_info", self.base))
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\nEntrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell"]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }
    #[doc = "Get a reference to the value of field `depends_on` after provisioning.\nContainers which should be started before this container. If specified the container will wait to start until all containers with the listed names are healthy."]
    pub fn depends_on(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.depends_on", self.base))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\nURL of the Container image in Google Container Registry or Google Artifact Registry. More info: https://kubernetes.io/docs/concepts/containers/images"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the container specified as a DNS_LABEL."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `working_dir` after provisioning.\nContainer's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image."]
    pub fn working_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.working_dir", self.base))
    }
    #[doc = "Get a reference to the value of field `liveness_probe` after provisioning.\n"]
    pub fn liveness_probe(
        &self,
    ) -> ListRef<CloudRunV2ServiceTemplateElContainersElLivenessProbeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.liveness_probe", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> ListRef<CloudRunV2ServiceTemplateElContainersElPortsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
    #[doc = "Get a reference to the value of field `readiness_probe` after provisioning.\n"]
    pub fn readiness_probe(
        &self,
    ) -> ListRef<CloudRunV2ServiceTemplateElContainersElReadinessProbeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.readiness_probe", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> ListRef<CloudRunV2ServiceTemplateElContainersElResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `startup_probe` after provisioning.\n"]
    pub fn startup_probe(
        &self,
    ) -> ListRef<CloudRunV2ServiceTemplateElContainersElStartupProbeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.startup_probe", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(
        &self,
    ) -> ListRef<CloudRunV2ServiceTemplateElContainersElVolumeMountsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.volume_mounts", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElNodeSelectorEl {
    accelerator: PrimField<String>,
}
impl CloudRunV2ServiceTemplateElNodeSelectorEl {}
impl ToListMappable for CloudRunV2ServiceTemplateElNodeSelectorEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElNodeSelectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElNodeSelectorEl {
    #[doc = "The GPU to attach to an instance. See https://cloud.google.com/run/docs/configuring/services/gpu for configuring GPU."]
    pub accelerator: PrimField<String>,
}
impl BuildCloudRunV2ServiceTemplateElNodeSelectorEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElNodeSelectorEl {
        CloudRunV2ServiceTemplateElNodeSelectorEl {
            accelerator: self.accelerator,
        }
    }
}
pub struct CloudRunV2ServiceTemplateElNodeSelectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElNodeSelectorElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceTemplateElNodeSelectorElRef {
        CloudRunV2ServiceTemplateElNodeSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElNodeSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator` after provisioning.\nThe GPU to attach to an instance. See https://cloud.google.com/run/docs/configuring/services/gpu for configuring GPU."]
    pub fn accelerator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accelerator", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElScalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_instance_count: Option<PrimField<f64>>,
}
impl CloudRunV2ServiceTemplateElScalingEl {
    #[doc = "Set the field `max_instance_count`.\nMaximum number of serving instances that this resource should have. Must not be less than minimum instance count. If absent, Cloud Run will calculate\na default value based on the project's available container instances quota in the region and specified instance size."]
    pub fn set_max_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_instance_count = Some(v.into());
        self
    }
    #[doc = "Set the field `min_instance_count`.\nMinimum number of serving instances that this resource should have. Defaults to 0. Must not be greater than maximum instance count."]
    pub fn set_min_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_instance_count = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElScalingEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElScalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElScalingEl {}
impl BuildCloudRunV2ServiceTemplateElScalingEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElScalingEl {
        CloudRunV2ServiceTemplateElScalingEl {
            max_instance_count: core::default::Default::default(),
            min_instance_count: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElScalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElScalingElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceTemplateElScalingElRef {
        CloudRunV2ServiceTemplateElScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElScalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_instance_count` after provisioning.\nMaximum number of serving instances that this resource should have. Must not be less than minimum instance count. If absent, Cloud Run will calculate\na default value based on the project's available container instances quota in the region and specified instance size."]
    pub fn max_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instance_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_instance_count` after provisioning.\nMinimum number of serving instances that this resource should have. Defaults to 0. Must not be greater than maximum instance count."]
    pub fn min_instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_instance_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instances: Option<SetField<PrimField<String>>>,
}
impl CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
    #[doc = "Set the field `instances`.\nThe Cloud SQL instance connection names, as can be found in https://console.cloud.google.com/sql/instances. Visit https://cloud.google.com/sql/docs/mysql/connect-run for more information on how to connect Cloud SQL and Cloud Run. Format: {project}:{location}:{instance}"]
    pub fn set_instances(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.instances = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {}
impl BuildCloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
        CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl {
            instances: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
        CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instances` after provisioning.\nThe Cloud SQL instance connection names, as can be found in https://console.cloud.google.com/sql/instances. Visit https://cloud.google.com/sql/docs/mysql/connect-run for more information on how to connect Cloud SQL and Cloud Run. Format: {project}:{location}:{instance}"]
    pub fn instances(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElVolumesElEmptyDirEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    medium: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_limit: Option<PrimField<String>>,
}
impl CloudRunV2ServiceTemplateElVolumesElEmptyDirEl {
    #[doc = "Set the field `medium`.\nThe different types of medium supported for EmptyDir. Default value: \"MEMORY\" Possible values: [\"MEMORY\", \"DISK\"]"]
    pub fn set_medium(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.medium = Some(v.into());
        self
    }
    #[doc = "Set the field `size_limit`.\nLimit on the storage usable by this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. This field's values are of the 'Quantity' k8s type: https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/. The default is nil which means that the limit is undefined. More info: https://kubernetes.io/docs/concepts/storage/volumes/#emptydir."]
    pub fn set_size_limit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.size_limit = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElVolumesElEmptyDirEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElVolumesElEmptyDirEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElVolumesElEmptyDirEl {}
impl BuildCloudRunV2ServiceTemplateElVolumesElEmptyDirEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElVolumesElEmptyDirEl {
        CloudRunV2ServiceTemplateElVolumesElEmptyDirEl {
            medium: core::default::Default::default(),
            size_limit: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElVolumesElEmptyDirElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElVolumesElEmptyDirElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceTemplateElVolumesElEmptyDirElRef {
        CloudRunV2ServiceTemplateElVolumesElEmptyDirElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElVolumesElEmptyDirElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `medium` after provisioning.\nThe different types of medium supported for EmptyDir. Default value: \"MEMORY\" Possible values: [\"MEMORY\", \"DISK\"]"]
    pub fn medium(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.medium", self.base))
    }
    #[doc = "Get a reference to the value of field `size_limit` after provisioning.\nLimit on the storage usable by this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. This field's values are of the 'Quantity' k8s type: https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/. The default is nil which means that the limit is undefined. More info: https://kubernetes.io/docs/concepts/storage/volumes/#emptydir."]
    pub fn size_limit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_limit", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElVolumesElGcsEl {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_options: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
}
impl CloudRunV2ServiceTemplateElVolumesElGcsEl {
    #[doc = "Set the field `mount_options`.\nA list of flags to pass to the gcsfuse command for configuring this volume.\nFlags should be passed without leading dashes."]
    pub fn set_mount_options(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.mount_options = Some(v.into());
        self
    }
    #[doc = "Set the field `read_only`.\nIf true, mount the GCS bucket as read-only"]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElVolumesElGcsEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElVolumesElGcsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElVolumesElGcsEl {
    #[doc = "GCS Bucket name"]
    pub bucket: PrimField<String>,
}
impl BuildCloudRunV2ServiceTemplateElVolumesElGcsEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElVolumesElGcsEl {
        CloudRunV2ServiceTemplateElVolumesElGcsEl {
            bucket: self.bucket,
            mount_options: core::default::Default::default(),
            read_only: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElVolumesElGcsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElVolumesElGcsElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceTemplateElVolumesElGcsElRef {
        CloudRunV2ServiceTemplateElVolumesElGcsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElVolumesElGcsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nGCS Bucket name"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `mount_options` after provisioning.\nA list of flags to pass to the gcsfuse command for configuring this volume.\nFlags should be passed without leading dashes."]
    pub fn mount_options(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mount_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `read_only` after provisioning.\nIf true, mount the GCS bucket as read-only"]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElVolumesElNfsEl {
    path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
    server: PrimField<String>,
}
impl CloudRunV2ServiceTemplateElVolumesElNfsEl {
    #[doc = "Set the field `read_only`.\nIf true, mount the NFS volume as read only"]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElVolumesElNfsEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElVolumesElNfsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElVolumesElNfsEl {
    #[doc = "Path that is exported by the NFS server."]
    pub path: PrimField<String>,
    #[doc = "Hostname or IP address of the NFS server"]
    pub server: PrimField<String>,
}
impl BuildCloudRunV2ServiceTemplateElVolumesElNfsEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElVolumesElNfsEl {
        CloudRunV2ServiceTemplateElVolumesElNfsEl {
            path: self.path,
            read_only: core::default::Default::default(),
            server: self.server,
        }
    }
}
pub struct CloudRunV2ServiceTemplateElVolumesElNfsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElVolumesElNfsElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceTemplateElVolumesElNfsElRef {
        CloudRunV2ServiceTemplateElVolumesElNfsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElVolumesElNfsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath that is exported by the NFS server."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `read_only` after provisioning.\nIf true, mount the NFS volume as read only"]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }
    #[doc = "Get a reference to the value of field `server` after provisioning.\nHostname or IP address of the NFS server"]
    pub fn server(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<f64>>,
    path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl CloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
    #[doc = "Set the field `mode`.\nInteger octal mode bits to use on this file, must be a value between 01 and 0777 (octal). If 0 or not set, the Volume's default mode will be used."]
    pub fn set_mode(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nThe Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElVolumesElSecretElItemsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
    #[doc = "The relative path of the secret in the container."]
    pub path: PrimField<String>,
}
impl BuildCloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
        CloudRunV2ServiceTemplateElVolumesElSecretElItemsEl {
            mode: core::default::Default::default(),
            path: self.path,
            version: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
        CloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nInteger octal mode bits to use on this file, must be a value between 01 and 0777 (octal). If 0 or not set, the Volume's default mode will be used."]
    pub fn mode(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nThe relative path of the secret in the container."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElVolumesElSecretElDynamic {
    items: Option<DynamicBlock<CloudRunV2ServiceTemplateElVolumesElSecretElItemsEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElVolumesElSecretEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_mode: Option<PrimField<f64>>,
    secret: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<CloudRunV2ServiceTemplateElVolumesElSecretElItemsEl>>,
    dynamic: CloudRunV2ServiceTemplateElVolumesElSecretElDynamic,
}
impl CloudRunV2ServiceTemplateElVolumesElSecretEl {
    #[doc = "Set the field `default_mode`.\nInteger representation of mode bits to use on created files by default. Must be a value between 0000 and 0777 (octal), defaulting to 0444. Directories within the path are not affected by this setting."]
    pub fn set_default_mode(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElVolumesElSecretElItemsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.items = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.items = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElVolumesElSecretEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElVolumesElSecretEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElVolumesElSecretEl {
    #[doc = "The name of the secret in Cloud Secret Manager. Format: {secret} if the secret is in the same project. projects/{project}/secrets/{secret} if the secret is in a different project."]
    pub secret: PrimField<String>,
}
impl BuildCloudRunV2ServiceTemplateElVolumesElSecretEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElVolumesElSecretEl {
        CloudRunV2ServiceTemplateElVolumesElSecretEl {
            default_mode: core::default::Default::default(),
            secret: self.secret,
            items: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElVolumesElSecretElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElVolumesElSecretElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceTemplateElVolumesElSecretElRef {
        CloudRunV2ServiceTemplateElVolumesElSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElVolumesElSecretElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_mode` after provisioning.\nInteger representation of mode bits to use on created files by default. Must be a value between 0000 and 0777 (octal), defaulting to 0444. Directories within the path are not affected by this setting."]
    pub fn default_mode(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\nThe name of the secret in Cloud Secret Manager. Format: {secret} if the secret is in the same project. projects/{project}/secrets/{secret} if the secret is in a different project."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> ListRef<CloudRunV2ServiceTemplateElVolumesElSecretElItemsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElVolumesElDynamic {
    cloud_sql_instance:
        Option<DynamicBlock<CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl>>,
    empty_dir: Option<DynamicBlock<CloudRunV2ServiceTemplateElVolumesElEmptyDirEl>>,
    gcs: Option<DynamicBlock<CloudRunV2ServiceTemplateElVolumesElGcsEl>>,
    nfs: Option<DynamicBlock<CloudRunV2ServiceTemplateElVolumesElNfsEl>>,
    secret: Option<DynamicBlock<CloudRunV2ServiceTemplateElVolumesElSecretEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElVolumesEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_sql_instance: Option<Vec<CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    empty_dir: Option<Vec<CloudRunV2ServiceTemplateElVolumesElEmptyDirEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs: Option<Vec<CloudRunV2ServiceTemplateElVolumesElGcsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs: Option<Vec<CloudRunV2ServiceTemplateElVolumesElNfsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<Vec<CloudRunV2ServiceTemplateElVolumesElSecretEl>>,
    dynamic: CloudRunV2ServiceTemplateElVolumesElDynamic,
}
impl CloudRunV2ServiceTemplateElVolumesEl {
    #[doc = "Set the field `cloud_sql_instance`.\n"]
    pub fn set_cloud_sql_instance(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_sql_instance = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_sql_instance = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `empty_dir`.\n"]
    pub fn set_empty_dir(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElVolumesElEmptyDirEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.empty_dir = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.empty_dir = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElVolumesElGcsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `nfs`.\n"]
    pub fn set_nfs(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElVolumesElNfsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.nfs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.nfs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElVolumesElSecretEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secret = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secret = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElVolumesEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElVolumesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElVolumesEl {
    #[doc = "Volume's name."]
    pub name: PrimField<String>,
}
impl BuildCloudRunV2ServiceTemplateElVolumesEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElVolumesEl {
        CloudRunV2ServiceTemplateElVolumesEl {
            name: self.name,
            cloud_sql_instance: core::default::Default::default(),
            empty_dir: core::default::Default::default(),
            gcs: core::default::Default::default(),
            nfs: core::default::Default::default(),
            secret: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElVolumesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElVolumesElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceTemplateElVolumesElRef {
        CloudRunV2ServiceTemplateElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElVolumesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nVolume's name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `cloud_sql_instance` after provisioning.\n"]
    pub fn cloud_sql_instance(
        &self,
    ) -> ListRef<CloudRunV2ServiceTemplateElVolumesElCloudSqlInstanceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_sql_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `empty_dir` after provisioning.\n"]
    pub fn empty_dir(&self) -> ListRef<CloudRunV2ServiceTemplateElVolumesElEmptyDirElRef> {
        ListRef::new(self.shared().clone(), format!("{}.empty_dir", self.base))
    }
    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(&self) -> ListRef<CloudRunV2ServiceTemplateElVolumesElGcsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }
    #[doc = "Get a reference to the value of field `nfs` after provisioning.\n"]
    pub fn nfs(&self) -> ListRef<CloudRunV2ServiceTemplateElVolumesElNfsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nfs", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> ListRef<CloudRunV2ServiceTemplateElVolumesElSecretElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
}
impl CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
    #[doc = "Set the field `network`.\nThe VPC network that the Cloud Run resource will be able to send traffic to. At least one of network or subnetwork must be specified. If both\nnetwork and subnetwork are specified, the given VPC subnetwork must belong to the given VPC network. If network is not specified, it will be\nlooked up from the subnetwork."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nThe VPC subnetwork that the Cloud Run resource will get IPs from. At least one of network or subnetwork must be specified. If both\nnetwork and subnetwork are specified, the given VPC subnetwork must belong to the given VPC network. If subnetwork is not specified, the\nsubnetwork with the same name with the network will be used."]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nNetwork tags applied to this Cloud Run service."]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {}
impl BuildCloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
        CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
        CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe VPC network that the Cloud Run resource will be able to send traffic to. At least one of network or subnetwork must be specified. If both\nnetwork and subnetwork are specified, the given VPC subnetwork must belong to the given VPC network. If network is not specified, it will be\nlooked up from the subnetwork."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThe VPC subnetwork that the Cloud Run resource will get IPs from. At least one of network or subnetwork must be specified. If both\nnetwork and subnetwork are specified, the given VPC subnetwork must belong to the given VPC network. If subnetwork is not specified, the\nsubnetwork with the same name with the network will be used."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nNetwork tags applied to this Cloud Run service."]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElVpcAccessElDynamic {
    network_interfaces:
        Option<DynamicBlock<CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateElVpcAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connector: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interfaces: Option<Vec<CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl>>,
    dynamic: CloudRunV2ServiceTemplateElVpcAccessElDynamic,
}
impl CloudRunV2ServiceTemplateElVpcAccessEl {
    #[doc = "Set the field `connector`.\nVPC Access connector name. Format: projects/{project}/locations/{location}/connectors/{connector}, where {project} can be project id or number."]
    pub fn set_connector(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connector = Some(v.into());
        self
    }
    #[doc = "Set the field `egress`.\nTraffic VPC egress settings. Possible values: [\"ALL_TRAFFIC\", \"PRIVATE_RANGES_ONLY\"]"]
    pub fn set_egress(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.egress = Some(v.into());
        self
    }
    #[doc = "Set the field `network_interfaces`.\n"]
    pub fn set_network_interfaces(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_interfaces = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_interfaces = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateElVpcAccessEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateElVpcAccessEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateElVpcAccessEl {}
impl BuildCloudRunV2ServiceTemplateElVpcAccessEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateElVpcAccessEl {
        CloudRunV2ServiceTemplateElVpcAccessEl {
            connector: core::default::Default::default(),
            egress: core::default::Default::default(),
            network_interfaces: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElVpcAccessElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElVpcAccessElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceTemplateElVpcAccessElRef {
        CloudRunV2ServiceTemplateElVpcAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElVpcAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connector` after provisioning.\nVPC Access connector name. Format: projects/{project}/locations/{location}/connectors/{connector}, where {project} can be project id or number."]
    pub fn connector(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connector", self.base))
    }
    #[doc = "Get a reference to the value of field `egress` after provisioning.\nTraffic VPC egress settings. Possible values: [\"ALL_TRAFFIC\", \"PRIVATE_RANGES_ONLY\"]"]
    pub fn egress(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.egress", self.base))
    }
    #[doc = "Get a reference to the value of field `network_interfaces` after provisioning.\n"]
    pub fn network_interfaces(
        &self,
    ) -> ListRef<CloudRunV2ServiceTemplateElVpcAccessElNetworkInterfacesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interfaces", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2ServiceTemplateElDynamic {
    containers: Option<DynamicBlock<CloudRunV2ServiceTemplateElContainersEl>>,
    node_selector: Option<DynamicBlock<CloudRunV2ServiceTemplateElNodeSelectorEl>>,
    scaling: Option<DynamicBlock<CloudRunV2ServiceTemplateElScalingEl>>,
    volumes: Option<DynamicBlock<CloudRunV2ServiceTemplateElVolumesEl>>,
    vpc_access: Option<DynamicBlock<CloudRunV2ServiceTemplateElVpcAccessEl>>,
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_environment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_zonal_redundancy_disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instance_request_concurrency: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_affinity: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    containers: Option<Vec<CloudRunV2ServiceTemplateElContainersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_selector: Option<Vec<CloudRunV2ServiceTemplateElNodeSelectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling: Option<Vec<CloudRunV2ServiceTemplateElScalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes: Option<Vec<CloudRunV2ServiceTemplateElVolumesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_access: Option<Vec<CloudRunV2ServiceTemplateElVpcAccessEl>>,
    dynamic: CloudRunV2ServiceTemplateElDynamic,
}
impl CloudRunV2ServiceTemplateEl {
    #[doc = "Set the field `annotations`.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system annotations in v1 now have a corresponding field in v2 RevisionTemplate.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules."]
    pub fn set_annotations(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_key`.\nA reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek"]
    pub fn set_encryption_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_key = Some(v.into());
        self
    }
    #[doc = "Set the field `execution_environment`.\nThe sandbox environment to host this Revision. Possible values: [\"EXECUTION_ENVIRONMENT_GEN1\", \"EXECUTION_ENVIRONMENT_GEN2\"]"]
    pub fn set_execution_environment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_environment = Some(v.into());
        self
    }
    #[doc = "Set the field `gpu_zonal_redundancy_disabled`.\nTrue if GPU zonal redundancy is disabled on this revision."]
    pub fn set_gpu_zonal_redundancy_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.gpu_zonal_redundancy_disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `health_check_disabled`.\nDisables health checking containers during deployment."]
    pub fn set_health_check_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.health_check_disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment, state, etc.\nFor more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 RevisionTemplate."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `max_instance_request_concurrency`.\nSets the maximum number of requests that each serving instance can receive.\nIf not specified or 0, defaults to 80 when requested CPU >= 1 and defaults to 1 when requested CPU < 1."]
    pub fn set_max_instance_request_concurrency(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_instance_request_concurrency = Some(v.into());
        self
    }
    #[doc = "Set the field `revision`.\nThe unique name for the revision. If this field is omitted, it will be automatically generated based on the Service name."]
    pub fn set_revision(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revision = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nEmail address of the IAM service account associated with the revision of the service. The service account represents the identity of the running revision, and determines what permissions the revision has. If not provided, the revision will use the project's default service account."]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `session_affinity`.\nEnables session affinity. For more information, go to https://cloud.google.com/run/docs/configuring/session-affinity"]
    pub fn set_session_affinity(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.session_affinity = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout`.\nMax allowed time for an instance to respond to a request.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElContainersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.containers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.containers = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_selector`.\n"]
    pub fn set_node_selector(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElNodeSelectorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_selector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_selector = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scaling`.\n"]
    pub fn set_scaling(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElScalingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scaling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scaling = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElVolumesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.volumes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.volumes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vpc_access`.\n"]
    pub fn set_vpc_access(
        mut self,
        v: impl Into<BlockAssignable<CloudRunV2ServiceTemplateElVpcAccessEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_access = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_access = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTemplateEl {
    type O = BlockAssignable<CloudRunV2ServiceTemplateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTemplateEl {}
impl BuildCloudRunV2ServiceTemplateEl {
    pub fn build(self) -> CloudRunV2ServiceTemplateEl {
        CloudRunV2ServiceTemplateEl {
            annotations: core::default::Default::default(),
            encryption_key: core::default::Default::default(),
            execution_environment: core::default::Default::default(),
            gpu_zonal_redundancy_disabled: core::default::Default::default(),
            health_check_disabled: core::default::Default::default(),
            labels: core::default::Default::default(),
            max_instance_request_concurrency: core::default::Default::default(),
            revision: core::default::Default::default(),
            service_account: core::default::Default::default(),
            session_affinity: core::default::Default::default(),
            timeout: core::default::Default::default(),
            containers: core::default::Default::default(),
            node_selector: core::default::Default::default(),
            scaling: core::default::Default::default(),
            volumes: core::default::Default::default(),
            vpc_access: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTemplateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTemplateElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceTemplateElRef {
        CloudRunV2ServiceTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.\n\nCloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system annotations in v1 now have a corresponding field in v2 RevisionTemplate.\n\nThis field follows Kubernetes annotations' namespacing, limits, and rules."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }
    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\nA reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek"]
    pub fn encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `execution_environment` after provisioning.\nThe sandbox environment to host this Revision. Possible values: [\"EXECUTION_ENVIRONMENT_GEN1\", \"EXECUTION_ENVIRONMENT_GEN2\"]"]
    pub fn execution_environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_environment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gpu_zonal_redundancy_disabled` after provisioning.\nTrue if GPU zonal redundancy is disabled on this revision."]
    pub fn gpu_zonal_redundancy_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gpu_zonal_redundancy_disabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `health_check_disabled` after provisioning.\nDisables health checking containers during deployment."]
    pub fn health_check_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_disabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment, state, etc.\nFor more information, visit https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels.\n\nCloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.\nAll system labels in v1 now have a corresponding field in v2 RevisionTemplate."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `max_instance_request_concurrency` after provisioning.\nSets the maximum number of requests that each serving instance can receive.\nIf not specified or 0, defaults to 80 when requested CPU >= 1 and defaults to 1 when requested CPU < 1."]
    pub fn max_instance_request_concurrency(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instance_request_concurrency", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\nThe unique name for the revision. If this field is omitted, it will be automatically generated based on the Service name."]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nEmail address of the IAM service account associated with the revision of the service. The service account represents the identity of the running revision, and determines what permissions the revision has. If not provided, the revision will use the project's default service account."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `session_affinity` after provisioning.\nEnables session affinity. For more information, go to https://cloud.google.com/run/docs/configuring/session-affinity"]
    pub fn session_affinity(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_affinity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nMax allowed time for an instance to respond to a request.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> ListRef<CloudRunV2ServiceTemplateElContainersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.containers", self.base))
    }
    #[doc = "Get a reference to the value of field `node_selector` after provisioning.\n"]
    pub fn node_selector(&self) -> ListRef<CloudRunV2ServiceTemplateElNodeSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_selector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scaling` after provisioning.\n"]
    pub fn scaling(&self) -> ListRef<CloudRunV2ServiceTemplateElScalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scaling", self.base))
    }
    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> ListRef<CloudRunV2ServiceTemplateElVolumesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_access` after provisioning.\n"]
    pub fn vpc_access(&self) -> ListRef<CloudRunV2ServiceTemplateElVpcAccessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_access", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunV2ServiceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CloudRunV2ServiceTimeoutsEl {
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
impl ToListMappable for CloudRunV2ServiceTimeoutsEl {
    type O = BlockAssignable<CloudRunV2ServiceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTimeoutsEl {}
impl BuildCloudRunV2ServiceTimeoutsEl {
    pub fn build(self) -> CloudRunV2ServiceTimeoutsEl {
        CloudRunV2ServiceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceTimeoutsElRef {
        CloudRunV2ServiceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTimeoutsElRef {
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
pub struct CloudRunV2ServiceTrafficEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl CloudRunV2ServiceTrafficEl {
    #[doc = "Set the field `percent`.\nSpecifies percent of the traffic to this Revision. This defaults to zero if unspecified."]
    pub fn set_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percent = Some(v.into());
        self
    }
    #[doc = "Set the field `revision`.\nRevision to which to send this portion of traffic, if traffic allocation is by revision."]
    pub fn set_revision(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revision = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\nIndicates a string to be part of the URI to exclusively reference this target."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nThe allocation type for this traffic target. Possible values: [\"TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST\", \"TRAFFIC_TARGET_ALLOCATION_TYPE_REVISION\"]"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunV2ServiceTrafficEl {
    type O = BlockAssignable<CloudRunV2ServiceTrafficEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunV2ServiceTrafficEl {}
impl BuildCloudRunV2ServiceTrafficEl {
    pub fn build(self) -> CloudRunV2ServiceTrafficEl {
        CloudRunV2ServiceTrafficEl {
            percent: core::default::Default::default(),
            revision: core::default::Default::default(),
            tag: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct CloudRunV2ServiceTrafficElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunV2ServiceTrafficElRef {
    fn new(shared: StackShared, base: String) -> CloudRunV2ServiceTrafficElRef {
        CloudRunV2ServiceTrafficElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunV2ServiceTrafficElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `percent` after provisioning.\nSpecifies percent of the traffic to this Revision. This defaults to zero if unspecified."]
    pub fn percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\nRevision to which to send this portion of traffic, if traffic allocation is by revision."]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\nIndicates a string to be part of the URI to exclusively reference this target."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe allocation type for this traffic target. Possible values: [\"TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST\", \"TRAFFIC_TARGET_ALLOCATION_TYPE_REVISION\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudRunV2ServiceDynamic {
    binary_authorization: Option<DynamicBlock<CloudRunV2ServiceBinaryAuthorizationEl>>,
    build_config: Option<DynamicBlock<CloudRunV2ServiceBuildConfigEl>>,
    multi_region_settings: Option<DynamicBlock<CloudRunV2ServiceMultiRegionSettingsEl>>,
    scaling: Option<DynamicBlock<CloudRunV2ServiceScalingEl>>,
    template: Option<DynamicBlock<CloudRunV2ServiceTemplateEl>>,
    traffic: Option<DynamicBlock<CloudRunV2ServiceTrafficEl>>,
}
