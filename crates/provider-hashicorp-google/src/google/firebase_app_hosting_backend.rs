use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FirebaseAppHostingBackendData {
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
    app_id: PrimField<String>,
    backend_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    service_account: PrimField<String>,
    serving_locality: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    codebase: Option<Vec<FirebaseAppHostingBackendCodebaseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FirebaseAppHostingBackendTimeoutsEl>,
    dynamic: FirebaseAppHostingBackendDynamic,
}
struct FirebaseAppHostingBackend_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FirebaseAppHostingBackendData>,
}
#[derive(Clone)]
pub struct FirebaseAppHostingBackend(Rc<FirebaseAppHostingBackend_>);
impl FirebaseAppHostingBackend {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nHuman-readable name. 63 character limit."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `environment`.\nThe environment name of the backend, used to load environment variables\nfrom environment specific configuration."]
    pub fn set_environment(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().environment = Some(v.into());
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
    #[doc = "Set the field `codebase`.\n"]
    pub fn set_codebase(
        self,
        v: impl Into<BlockAssignable<FirebaseAppHostingBackendCodebaseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().codebase = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.codebase = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FirebaseAppHostingBackendTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `app_id` after provisioning.\nThe [ID of a Web\nApp](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id)\nassociated with the backend."]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backend_id` after provisioning.\nId of the backend. Also used as the service ID for Cloud Run, and as part\nof the default domain name."]
    pub fn backend_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime at which the backend was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nTime at which the backend was deleted."]
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
    #[doc = "Get a reference to the value of field `environment` after provisioning.\nThe environment name of the backend, used to load environment variables\nfrom environment specific configuration."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize\nobjects.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe canonical IDs of a Google Cloud location such as \"us-east1\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_resources` after provisioning.\nA list of the resources managed by this backend."]
    pub fn managed_resources(&self) -> ListRef<FirebaseAppHostingBackendManagedResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the backend.\n\nFormat:\n\n'projects/{project}/locations/{locationId}/backends/{backendId}'."]
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
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe name of the service account used for Cloud Build and Cloud Run.\nShould have the role roles/firebaseapphosting.computeRunner\nor equivalent permissions."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `serving_locality` after provisioning.\nImmutable. Specifies how App Hosting will serve the content for this backend. It will\neither be contained to a single region (REGIONAL_STRICT) or allowed to use\nApp Hosting's global-replicated serving infrastructure (GLOBAL_ACCESS). Possible values: [\"REGIONAL_STRICT\", \"GLOBAL_ACCESS\"]"]
    pub fn serving_locality(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serving_locality", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime at which the backend was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe primary URI to communicate with the backend."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `codebase` after provisioning.\n"]
    pub fn codebase(&self) -> ListRef<FirebaseAppHostingBackendCodebaseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.codebase", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppHostingBackendTimeoutsElRef {
        FirebaseAppHostingBackendTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for FirebaseAppHostingBackend {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FirebaseAppHostingBackend {}
impl ToListMappable for FirebaseAppHostingBackend {
    type O = ListRef<FirebaseAppHostingBackendRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FirebaseAppHostingBackend_ {
    fn extract_resource_type(&self) -> String {
        "google_firebase_app_hosting_backend".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFirebaseAppHostingBackend {
    pub tf_id: String,
    #[doc = "The [ID of a Web\nApp](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id)\nassociated with the backend."]
    pub app_id: PrimField<String>,
    #[doc = "Id of the backend. Also used as the service ID for Cloud Run, and as part\nof the default domain name."]
    pub backend_id: PrimField<String>,
    #[doc = "The canonical IDs of a Google Cloud location such as \"us-east1\"."]
    pub location: PrimField<String>,
    #[doc = "The name of the service account used for Cloud Build and Cloud Run.\nShould have the role roles/firebaseapphosting.computeRunner\nor equivalent permissions."]
    pub service_account: PrimField<String>,
    #[doc = "Immutable. Specifies how App Hosting will serve the content for this backend. It will\neither be contained to a single region (REGIONAL_STRICT) or allowed to use\nApp Hosting's global-replicated serving infrastructure (GLOBAL_ACCESS). Possible values: [\"REGIONAL_STRICT\", \"GLOBAL_ACCESS\"]"]
    pub serving_locality: PrimField<String>,
}
impl BuildFirebaseAppHostingBackend {
    pub fn build(self, stack: &mut Stack) -> FirebaseAppHostingBackend {
        let out = FirebaseAppHostingBackend(Rc::new(FirebaseAppHostingBackend_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FirebaseAppHostingBackendData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                app_id: self.app_id,
                backend_id: self.backend_id,
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                environment: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                service_account: self.service_account,
                serving_locality: self.serving_locality,
                codebase: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FirebaseAppHostingBackendRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingBackendRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FirebaseAppHostingBackendRef {
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
    #[doc = "Get a reference to the value of field `app_id` after provisioning.\nThe [ID of a Web\nApp](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id)\nassociated with the backend."]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backend_id` after provisioning.\nId of the backend. Also used as the service ID for Cloud Run, and as part\nof the default domain name."]
    pub fn backend_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime at which the backend was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nTime at which the backend was deleted."]
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
    #[doc = "Get a reference to the value of field `environment` after provisioning.\nThe environment name of the backend, used to load environment variables\nfrom environment specific configuration."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize\nobjects.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe canonical IDs of a Google Cloud location such as \"us-east1\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_resources` after provisioning.\nA list of the resources managed by this backend."]
    pub fn managed_resources(&self) -> ListRef<FirebaseAppHostingBackendManagedResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the backend.\n\nFormat:\n\n'projects/{project}/locations/{locationId}/backends/{backendId}'."]
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
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe name of the service account used for Cloud Build and Cloud Run.\nShould have the role roles/firebaseapphosting.computeRunner\nor equivalent permissions."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `serving_locality` after provisioning.\nImmutable. Specifies how App Hosting will serve the content for this backend. It will\neither be contained to a single region (REGIONAL_STRICT) or allowed to use\nApp Hosting's global-replicated serving infrastructure (GLOBAL_ACCESS). Possible values: [\"REGIONAL_STRICT\", \"GLOBAL_ACCESS\"]"]
    pub fn serving_locality(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serving_locality", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime at which the backend was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe primary URI to communicate with the backend."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `codebase` after provisioning.\n"]
    pub fn codebase(&self) -> ListRef<FirebaseAppHostingBackendCodebaseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.codebase", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppHostingBackendTimeoutsElRef {
        FirebaseAppHostingBackendTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingBackendManagedResourcesElRunServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl FirebaseAppHostingBackendManagedResourcesElRunServiceEl {
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseAppHostingBackendManagedResourcesElRunServiceEl {
    type O = BlockAssignable<FirebaseAppHostingBackendManagedResourcesElRunServiceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingBackendManagedResourcesElRunServiceEl {}
impl BuildFirebaseAppHostingBackendManagedResourcesElRunServiceEl {
    pub fn build(self) -> FirebaseAppHostingBackendManagedResourcesElRunServiceEl {
        FirebaseAppHostingBackendManagedResourcesElRunServiceEl {
            service: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingBackendManagedResourcesElRunServiceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingBackendManagedResourcesElRunServiceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FirebaseAppHostingBackendManagedResourcesElRunServiceElRef {
        FirebaseAppHostingBackendManagedResourcesElRunServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingBackendManagedResourcesElRunServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingBackendManagedResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    run_service: Option<ListField<FirebaseAppHostingBackendManagedResourcesElRunServiceEl>>,
}
impl FirebaseAppHostingBackendManagedResourcesEl {
    #[doc = "Set the field `run_service`.\n"]
    pub fn set_run_service(
        mut self,
        v: impl Into<ListField<FirebaseAppHostingBackendManagedResourcesElRunServiceEl>>,
    ) -> Self {
        self.run_service = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseAppHostingBackendManagedResourcesEl {
    type O = BlockAssignable<FirebaseAppHostingBackendManagedResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingBackendManagedResourcesEl {}
impl BuildFirebaseAppHostingBackendManagedResourcesEl {
    pub fn build(self) -> FirebaseAppHostingBackendManagedResourcesEl {
        FirebaseAppHostingBackendManagedResourcesEl {
            run_service: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingBackendManagedResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingBackendManagedResourcesElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingBackendManagedResourcesElRef {
        FirebaseAppHostingBackendManagedResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingBackendManagedResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `run_service` after provisioning.\n"]
    pub fn run_service(
        &self,
    ) -> ListRef<FirebaseAppHostingBackendManagedResourcesElRunServiceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.run_service", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingBackendCodebaseEl {
    repository: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_directory: Option<PrimField<String>>,
}
impl FirebaseAppHostingBackendCodebaseEl {
    #[doc = "Set the field `root_directory`.\nIf 'repository' is provided, the directory relative to the root of the\nrepository to use as the root for the deployed web app."]
    pub fn set_root_directory(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.root_directory = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseAppHostingBackendCodebaseEl {
    type O = BlockAssignable<FirebaseAppHostingBackendCodebaseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingBackendCodebaseEl {
    #[doc = "The resource name for the Developer Connect\n['gitRepositoryLink'](https://cloud.google.com/developer-connect/docs/api/reference/rest/v1/projects.locations.connections.gitRepositoryLinks)\nconnected to this backend, in the format:\n\nprojects/{project}/locations/{location}/connections/{connection}/gitRepositoryLinks/{repositoryLink}"]
    pub repository: PrimField<String>,
}
impl BuildFirebaseAppHostingBackendCodebaseEl {
    pub fn build(self) -> FirebaseAppHostingBackendCodebaseEl {
        FirebaseAppHostingBackendCodebaseEl {
            repository: self.repository,
            root_directory: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingBackendCodebaseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingBackendCodebaseElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingBackendCodebaseElRef {
        FirebaseAppHostingBackendCodebaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingBackendCodebaseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\nThe resource name for the Developer Connect\n['gitRepositoryLink'](https://cloud.google.com/developer-connect/docs/api/reference/rest/v1/projects.locations.connections.gitRepositoryLinks)\nconnected to this backend, in the format:\n\nprojects/{project}/locations/{location}/connections/{connection}/gitRepositoryLinks/{repositoryLink}"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\nIf 'repository' is provided, the directory relative to the root of the\nrepository to use as the root for the deployed web app."]
    pub fn root_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_directory", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingBackendTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl FirebaseAppHostingBackendTimeoutsEl {
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
impl ToListMappable for FirebaseAppHostingBackendTimeoutsEl {
    type O = BlockAssignable<FirebaseAppHostingBackendTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingBackendTimeoutsEl {}
impl BuildFirebaseAppHostingBackendTimeoutsEl {
    pub fn build(self) -> FirebaseAppHostingBackendTimeoutsEl {
        FirebaseAppHostingBackendTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingBackendTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingBackendTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingBackendTimeoutsElRef {
        FirebaseAppHostingBackendTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingBackendTimeoutsElRef {
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
struct FirebaseAppHostingBackendDynamic {
    codebase: Option<DynamicBlock<FirebaseAppHostingBackendCodebaseEl>>,
}
