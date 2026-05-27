use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FirebaseAppHostingBuildData {
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
    backend: PrimField<String>,
    build_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<FirebaseAppHostingBuildSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FirebaseAppHostingBuildTimeoutsEl>,
    dynamic: FirebaseAppHostingBuildDynamic,
}
struct FirebaseAppHostingBuild_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FirebaseAppHostingBuildData>,
}
#[derive(Clone)]
pub struct FirebaseAppHostingBuild(Rc<FirebaseAppHostingBuild_>);
impl FirebaseAppHostingBuild {
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
    #[doc = "Set the field `annotations`.\nUnstructured key value map that may be set by external tools to\nstore and arbitrary metadata. They are not queryable and should be\npreserved when modifying objects.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nHuman-readable name. 63 character limit."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUnstructured key value map that can be used to organize and categorize\nobjects.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        self,
        v: impl Into<BlockAssignable<FirebaseAppHostingBuildSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FirebaseAppHostingBuildTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to\nstore and arbitrary metadata. They are not queryable and should be\npreserved when modifying objects.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backend` after provisioning.\nThe ID of the Backend that this Build applies to"]
    pub fn backend(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build_id` after provisioning.\nThe user-specified ID of the build being created."]
    pub fn build_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.build_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build_logs_uri` after provisioning.\nThe location of the [Cloud Build\nlogs](https://cloud.google.com/build/docs/view-build-results) for the build\nprocess."]
    pub fn build_logs_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.build_logs_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime at which the build was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable name. 63 character limit."]
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
    #[doc = "Get a reference to the value of field `environment` after provisioning.\nThe environment name of the backend when this build was created."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `error` after provisioning.\nThe 'Status' type defines a logical error model that is suitable for\ndifferent programming environments, including REST APIs and RPC APIs. It is\nused by [gRPC](https://github.com/grpc). Each 'Status' message contains\nthree pieces of data: error code, error message, and error details.\n\nYou can find out more about this error model and how to work with it in the\n[API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub fn error(&self) -> ListRef<FirebaseAppHostingBuildErrorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.error", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `error_source` after provisioning.\nThe source of the error for the build, if in a 'FAILED' state.\nPossible values:\nCLOUD_BUILD\nCLOUD_RUN"]
    pub fn error_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.error_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nServer-computed checksum based on other values; may be sent\non update or delete to ensure operation is done on expected resource."]
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
    #[doc = "Get a reference to the value of field `image` after provisioning.\nThe Artifact Registry\n[container\nimage](https://cloud.google.com/artifact-registry/docs/reference/rest/v1/projects.locations.repositories.dockerImages)\nURI, used by the Cloud Run\n['revision'](https://cloud.google.com/run/docs/reference/rest/v2/projects.locations.services.revisions)\nfor this build."]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize\nobjects.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the Backend that this Build applies to"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the build.\n\nFormat:\n\n'projects/{project}/locations/{locationId}/backends/{backendId}/builds/{buildId}'."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the build.\nPossible values:\nBUILDING\nBUILT\nDEPLOYING\nREADY\nFAILED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-assigned, unique identifier."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime at which the build was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<FirebaseAppHostingBuildSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppHostingBuildTimeoutsElRef {
        FirebaseAppHostingBuildTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for FirebaseAppHostingBuild {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FirebaseAppHostingBuild {}
impl ToListMappable for FirebaseAppHostingBuild {
    type O = ListRef<FirebaseAppHostingBuildRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FirebaseAppHostingBuild_ {
    fn extract_resource_type(&self) -> String {
        "google_firebase_app_hosting_build".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFirebaseAppHostingBuild {
    pub tf_id: String,
    #[doc = "The ID of the Backend that this Build applies to"]
    pub backend: PrimField<String>,
    #[doc = "The user-specified ID of the build being created."]
    pub build_id: PrimField<String>,
    #[doc = "The location of the Backend that this Build applies to"]
    pub location: PrimField<String>,
}
impl BuildFirebaseAppHostingBuild {
    pub fn build(self, stack: &mut Stack) -> FirebaseAppHostingBuild {
        let out = FirebaseAppHostingBuild(Rc::new(FirebaseAppHostingBuild_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FirebaseAppHostingBuildData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                backend: self.backend,
                build_id: self.build_id,
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                source: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FirebaseAppHostingBuildRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingBuildRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FirebaseAppHostingBuildRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to\nstore and arbitrary metadata. They are not queryable and should be\npreserved when modifying objects.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backend` after provisioning.\nThe ID of the Backend that this Build applies to"]
    pub fn backend(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build_id` after provisioning.\nThe user-specified ID of the build being created."]
    pub fn build_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.build_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build_logs_uri` after provisioning.\nThe location of the [Cloud Build\nlogs](https://cloud.google.com/build/docs/view-build-results) for the build\nprocess."]
    pub fn build_logs_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.build_logs_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime at which the build was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable name. 63 character limit."]
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
    #[doc = "Get a reference to the value of field `environment` after provisioning.\nThe environment name of the backend when this build was created."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `error` after provisioning.\nThe 'Status' type defines a logical error model that is suitable for\ndifferent programming environments, including REST APIs and RPC APIs. It is\nused by [gRPC](https://github.com/grpc). Each 'Status' message contains\nthree pieces of data: error code, error message, and error details.\n\nYou can find out more about this error model and how to work with it in the\n[API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub fn error(&self) -> ListRef<FirebaseAppHostingBuildErrorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.error", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `error_source` after provisioning.\nThe source of the error for the build, if in a 'FAILED' state.\nPossible values:\nCLOUD_BUILD\nCLOUD_RUN"]
    pub fn error_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.error_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nServer-computed checksum based on other values; may be sent\non update or delete to ensure operation is done on expected resource."]
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
    #[doc = "Get a reference to the value of field `image` after provisioning.\nThe Artifact Registry\n[container\nimage](https://cloud.google.com/artifact-registry/docs/reference/rest/v1/projects.locations.repositories.dockerImages)\nURI, used by the Cloud Run\n['revision'](https://cloud.google.com/run/docs/reference/rest/v2/projects.locations.services.revisions)\nfor this build."]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize\nobjects.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the Backend that this Build applies to"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the build.\n\nFormat:\n\n'projects/{project}/locations/{locationId}/backends/{backendId}/builds/{buildId}'."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the build.\nPossible values:\nBUILDING\nBUILT\nDEPLOYING\nREADY\nFAILED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-assigned, unique identifier."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime at which the build was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<FirebaseAppHostingBuildSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppHostingBuildTimeoutsElRef {
        FirebaseAppHostingBuildTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingBuildErrorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<ListField<RecField<PrimField<String>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl FirebaseAppHostingBuildErrorEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl Into<ListField<RecField<PrimField<String>>>>) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseAppHostingBuildErrorEl {
    type O = BlockAssignable<FirebaseAppHostingBuildErrorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingBuildErrorEl {}
impl BuildFirebaseAppHostingBuildErrorEl {
    pub fn build(self) -> FirebaseAppHostingBuildErrorEl {
        FirebaseAppHostingBuildErrorEl {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingBuildErrorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingBuildErrorElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingBuildErrorElRef {
        FirebaseAppHostingBuildErrorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingBuildErrorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> ListRef<RecRef<PrimExpr<String>>> {
        ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingBuildSourceElCodebaseElAuthorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_uri: Option<PrimField<String>>,
}
impl FirebaseAppHostingBuildSourceElCodebaseElAuthorEl {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
    #[doc = "Set the field `image_uri`.\n"]
    pub fn set_image_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_uri = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseAppHostingBuildSourceElCodebaseElAuthorEl {
    type O = BlockAssignable<FirebaseAppHostingBuildSourceElCodebaseElAuthorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingBuildSourceElCodebaseElAuthorEl {}
impl BuildFirebaseAppHostingBuildSourceElCodebaseElAuthorEl {
    pub fn build(self) -> FirebaseAppHostingBuildSourceElCodebaseElAuthorEl {
        FirebaseAppHostingBuildSourceElCodebaseElAuthorEl {
            display_name: core::default::Default::default(),
            email: core::default::Default::default(),
            image_uri: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingBuildSourceElCodebaseElAuthorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingBuildSourceElCodebaseElAuthorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FirebaseAppHostingBuildSourceElCodebaseElAuthorElRef {
        FirebaseAppHostingBuildSourceElCodebaseElAuthorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingBuildSourceElCodebaseElAuthorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\n"]
    pub fn image_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingBuildSourceElCodebaseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit: Option<PrimField<String>>,
}
impl FirebaseAppHostingBuildSourceElCodebaseEl {
    #[doc = "Set the field `branch`.\nThe branch in the codebase to build from, using the latest commit."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }
    #[doc = "Set the field `commit`.\nThe commit in the codebase to build from."]
    pub fn set_commit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.commit = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseAppHostingBuildSourceElCodebaseEl {
    type O = BlockAssignable<FirebaseAppHostingBuildSourceElCodebaseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingBuildSourceElCodebaseEl {}
impl BuildFirebaseAppHostingBuildSourceElCodebaseEl {
    pub fn build(self) -> FirebaseAppHostingBuildSourceElCodebaseEl {
        FirebaseAppHostingBuildSourceElCodebaseEl {
            branch: core::default::Default::default(),
            commit: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingBuildSourceElCodebaseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingBuildSourceElCodebaseElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingBuildSourceElCodebaseElRef {
        FirebaseAppHostingBuildSourceElCodebaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingBuildSourceElCodebaseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `author` after provisioning.\nVersion control metadata for a user associated with a resolved codebase.\nCurrently assumes a Git user."]
    pub fn author(&self) -> ListRef<FirebaseAppHostingBuildSourceElCodebaseElAuthorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.author", self.base))
    }
    #[doc = "Get a reference to the value of field `branch` after provisioning.\nThe branch in the codebase to build from, using the latest commit."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }
    #[doc = "Get a reference to the value of field `commit` after provisioning.\nThe commit in the codebase to build from."]
    pub fn commit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit", self.base))
    }
    #[doc = "Get a reference to the value of field `commit_message` after provisioning.\nThe message of a codebase change."]
    pub fn commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.commit_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `commit_time` after provisioning.\nThe time the change was made."]
    pub fn commit_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_time", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-friendly name to use for this Codebase when displaying a build.\nWe use the first eight characters of the SHA-1 hash for GitHub.com."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `hash` after provisioning.\nThe full SHA-1 hash of a Git commit, if available."]
    pub fn hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hash", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nA URI linking to the codebase on an hosting provider's website. May\nnot be valid if the commit has been rebased or force-pushed out of\nexistence in the linked repository."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingBuildSourceElContainerEl {
    image: PrimField<String>,
}
impl FirebaseAppHostingBuildSourceElContainerEl {}
impl ToListMappable for FirebaseAppHostingBuildSourceElContainerEl {
    type O = BlockAssignable<FirebaseAppHostingBuildSourceElContainerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingBuildSourceElContainerEl {
    #[doc = "A URI representing a container for the backend to use."]
    pub image: PrimField<String>,
}
impl BuildFirebaseAppHostingBuildSourceElContainerEl {
    pub fn build(self) -> FirebaseAppHostingBuildSourceElContainerEl {
        FirebaseAppHostingBuildSourceElContainerEl { image: self.image }
    }
}
pub struct FirebaseAppHostingBuildSourceElContainerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingBuildSourceElContainerElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingBuildSourceElContainerElRef {
        FirebaseAppHostingBuildSourceElContainerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingBuildSourceElContainerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\nA URI representing a container for the backend to use."]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
}
#[derive(Serialize, Default)]
struct FirebaseAppHostingBuildSourceElDynamic {
    codebase: Option<DynamicBlock<FirebaseAppHostingBuildSourceElCodebaseEl>>,
    container: Option<DynamicBlock<FirebaseAppHostingBuildSourceElContainerEl>>,
}
#[derive(Serialize)]
pub struct FirebaseAppHostingBuildSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    codebase: Option<Vec<FirebaseAppHostingBuildSourceElCodebaseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container: Option<Vec<FirebaseAppHostingBuildSourceElContainerEl>>,
    dynamic: FirebaseAppHostingBuildSourceElDynamic,
}
impl FirebaseAppHostingBuildSourceEl {
    #[doc = "Set the field `codebase`.\n"]
    pub fn set_codebase(
        mut self,
        v: impl Into<BlockAssignable<FirebaseAppHostingBuildSourceElCodebaseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.codebase = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.codebase = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        mut self,
        v: impl Into<BlockAssignable<FirebaseAppHostingBuildSourceElContainerEl>>,
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
impl ToListMappable for FirebaseAppHostingBuildSourceEl {
    type O = BlockAssignable<FirebaseAppHostingBuildSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingBuildSourceEl {}
impl BuildFirebaseAppHostingBuildSourceEl {
    pub fn build(self) -> FirebaseAppHostingBuildSourceEl {
        FirebaseAppHostingBuildSourceEl {
            codebase: core::default::Default::default(),
            container: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FirebaseAppHostingBuildSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingBuildSourceElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingBuildSourceElRef {
        FirebaseAppHostingBuildSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingBuildSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `codebase` after provisioning.\n"]
    pub fn codebase(&self) -> ListRef<FirebaseAppHostingBuildSourceElCodebaseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.codebase", self.base))
    }
    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> ListRef<FirebaseAppHostingBuildSourceElContainerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.container", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingBuildTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl FirebaseAppHostingBuildTimeoutsEl {
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
impl ToListMappable for FirebaseAppHostingBuildTimeoutsEl {
    type O = BlockAssignable<FirebaseAppHostingBuildTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingBuildTimeoutsEl {}
impl BuildFirebaseAppHostingBuildTimeoutsEl {
    pub fn build(self) -> FirebaseAppHostingBuildTimeoutsEl {
        FirebaseAppHostingBuildTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingBuildTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingBuildTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingBuildTimeoutsElRef {
        FirebaseAppHostingBuildTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingBuildTimeoutsElRef {
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
struct FirebaseAppHostingBuildDynamic {
    source: Option<DynamicBlock<FirebaseAppHostingBuildSourceEl>>,
}
