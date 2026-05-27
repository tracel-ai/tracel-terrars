use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ClouddeployAutomationData {
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
    delivery_pipeline: PrimField<String>,
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
    service_account: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suspended: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<ClouddeployAutomationRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selector: Option<Vec<ClouddeployAutomationSelectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ClouddeployAutomationTimeoutsEl>,
    dynamic: ClouddeployAutomationDynamic,
}
struct ClouddeployAutomation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ClouddeployAutomationData>,
}
#[derive(Clone)]
pub struct ClouddeployAutomation(Rc<ClouddeployAutomation_>);
impl ClouddeployAutomation {
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
    #[doc = "Set the field `annotations`.\nOptional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an optional prefix and name, separated by a slash ('/'). * The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ('[a-z0-9A-Z]') with dashes ('-'), underscores ('_'), dots ('.'), and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots('.'), not longer than 253 characters in total, followed by a slash ('/'). See https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more details.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nOptional. Description of the 'Automation'. Max length is 255 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nOptional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 63 characters.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `suspended`.\nOptional. When Suspended, automation is deactivated from execution."]
    pub fn set_suspended(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().suspended = Some(v.into());
        self
    }
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl Into<BlockAssignable<ClouddeployAutomationRulesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `selector`.\n"]
    pub fn set_selector(
        self,
        v: impl Into<BlockAssignable<ClouddeployAutomationSelectorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().selector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.selector = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ClouddeployAutomationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nOptional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an optional prefix and name, separated by a slash ('/'). * The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ('[a-z0-9A-Z]') with dashes ('-'), underscores ('_'), dots ('.'), and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots('.'), not longer than 253 characters in total, followed by a slash ('/'). See https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more details.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the automation was created."]
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
    #[doc = "Get a reference to the value of field `delivery_pipeline` after provisioning.\nThe delivery_pipeline for the resource"]
    pub fn delivery_pipeline(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_pipeline", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the 'Automation'. Max length is 255 characters."]
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nOptional. The weak etag of the 'Automation' resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 63 characters.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the 'Automation'."]
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
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nRequired. Email address of the user-managed IAM service account that creates Cloud Deploy release and rollout resources."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `suspended` after provisioning.\nOptional. When Suspended, automation is deactivated from execution."]
    pub fn suspended(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.suspended", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique identifier of the 'Automation'."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Time at which the automation was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<ClouddeployAutomationRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `selector` after provisioning.\n"]
    pub fn selector(&self) -> ListRef<ClouddeployAutomationSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.selector", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployAutomationTimeoutsElRef {
        ClouddeployAutomationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ClouddeployAutomation {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ClouddeployAutomation {}
impl ToListMappable for ClouddeployAutomation {
    type O = ListRef<ClouddeployAutomationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ClouddeployAutomation_ {
    fn extract_resource_type(&self) -> String {
        "google_clouddeploy_automation".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildClouddeployAutomation {
    pub tf_id: String,
    #[doc = "The delivery_pipeline for the resource"]
    pub delivery_pipeline: PrimField<String>,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
    #[doc = "Name of the 'Automation'."]
    pub name: PrimField<String>,
    #[doc = "Required. Email address of the user-managed IAM service account that creates Cloud Deploy release and rollout resources."]
    pub service_account: PrimField<String>,
}
impl BuildClouddeployAutomation {
    pub fn build(self, stack: &mut Stack) -> ClouddeployAutomation {
        let out = ClouddeployAutomation(Rc::new(ClouddeployAutomation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ClouddeployAutomationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                delivery_pipeline: self.delivery_pipeline,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                service_account: self.service_account,
                suspended: core::default::Default::default(),
                rules: core::default::Default::default(),
                selector: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ClouddeployAutomationRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployAutomationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ClouddeployAutomationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nOptional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an optional prefix and name, separated by a slash ('/'). * The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ('[a-z0-9A-Z]') with dashes ('-'), underscores ('_'), dots ('.'), and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots('.'), not longer than 253 characters in total, followed by a slash ('/'). See https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more details.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the automation was created."]
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
    #[doc = "Get a reference to the value of field `delivery_pipeline` after provisioning.\nThe delivery_pipeline for the resource"]
    pub fn delivery_pipeline(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_pipeline", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the 'Automation'. Max length is 255 characters."]
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nOptional. The weak etag of the 'Automation' resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 63 characters.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the 'Automation'."]
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
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nRequired. Email address of the user-managed IAM service account that creates Cloud Deploy release and rollout resources."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `suspended` after provisioning.\nOptional. When Suspended, automation is deactivated from execution."]
    pub fn suspended(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.suspended", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique identifier of the 'Automation'."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Time at which the automation was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<ClouddeployAutomationRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `selector` after provisioning.\n"]
    pub fn selector(&self) -> ListRef<ClouddeployAutomationSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.selector", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployAutomationTimeoutsElRef {
        ClouddeployAutomationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddeployAutomationRulesElAdvanceRolloutRuleEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_phases: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait: Option<PrimField<String>>,
}
impl ClouddeployAutomationRulesElAdvanceRolloutRuleEl {
    #[doc = "Set the field `source_phases`.\nOptional. Proceeds only after phase name matched any one in the list. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: '^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$'."]
    pub fn set_source_phases(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.source_phases = Some(v.into());
        self
    }
    #[doc = "Set the field `wait`.\nOptional. How long to wait after a rollout is finished."]
    pub fn set_wait(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.wait = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployAutomationRulesElAdvanceRolloutRuleEl {
    type O = BlockAssignable<ClouddeployAutomationRulesElAdvanceRolloutRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployAutomationRulesElAdvanceRolloutRuleEl {
    #[doc = "Required. ID of the rule. This id must be unique in the 'Automation' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub id: PrimField<String>,
}
impl BuildClouddeployAutomationRulesElAdvanceRolloutRuleEl {
    pub fn build(self) -> ClouddeployAutomationRulesElAdvanceRolloutRuleEl {
        ClouddeployAutomationRulesElAdvanceRolloutRuleEl {
            id: self.id,
            source_phases: core::default::Default::default(),
            wait: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployAutomationRulesElAdvanceRolloutRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployAutomationRulesElAdvanceRolloutRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployAutomationRulesElAdvanceRolloutRuleElRef {
        ClouddeployAutomationRulesElAdvanceRolloutRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployAutomationRulesElAdvanceRolloutRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nRequired. ID of the rule. This id must be unique in the 'Automation' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `source_phases` after provisioning.\nOptional. Proceeds only after phase name matched any one in the list. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: '^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$'."]
    pub fn source_phases(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_phases", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `wait` after provisioning.\nOptional. How long to wait after a rollout is finished."]
    pub fn wait(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployAutomationRulesElPromoteReleaseRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_phase: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_target_id: Option<PrimField<String>>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait: Option<PrimField<String>>,
}
impl ClouddeployAutomationRulesElPromoteReleaseRuleEl {
    #[doc = "Set the field `destination_phase`.\nOptional. The starting phase of the rollout created by this operation. Default to the first phase."]
    pub fn set_destination_phase(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_phase = Some(v.into());
        self
    }
    #[doc = "Set the field `destination_target_id`.\nOptional. The ID of the stage in the pipeline to which this 'Release' is deploying. If unspecified, default it to the next stage in the promotion flow. The value of this field could be one of the following: * The last segment of a target name. It only needs the ID to determine if the target is one of the stages in the promotion sequence defined in the pipeline. * \"@next\", the next target in the promotion sequence."]
    pub fn set_destination_target_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_target_id = Some(v.into());
        self
    }
    #[doc = "Set the field `wait`.\nOptional. How long the release need to be paused until being promoted to the next target."]
    pub fn set_wait(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.wait = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployAutomationRulesElPromoteReleaseRuleEl {
    type O = BlockAssignable<ClouddeployAutomationRulesElPromoteReleaseRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployAutomationRulesElPromoteReleaseRuleEl {
    #[doc = "Required. ID of the rule. This id must be unique in the 'Automation' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub id: PrimField<String>,
}
impl BuildClouddeployAutomationRulesElPromoteReleaseRuleEl {
    pub fn build(self) -> ClouddeployAutomationRulesElPromoteReleaseRuleEl {
        ClouddeployAutomationRulesElPromoteReleaseRuleEl {
            destination_phase: core::default::Default::default(),
            destination_target_id: core::default::Default::default(),
            id: self.id,
            wait: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployAutomationRulesElPromoteReleaseRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployAutomationRulesElPromoteReleaseRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployAutomationRulesElPromoteReleaseRuleElRef {
        ClouddeployAutomationRulesElPromoteReleaseRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployAutomationRulesElPromoteReleaseRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `destination_phase` after provisioning.\nOptional. The starting phase of the rollout created by this operation. Default to the first phase."]
    pub fn destination_phase(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_phase", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destination_target_id` after provisioning.\nOptional. The ID of the stage in the pipeline to which this 'Release' is deploying. If unspecified, default it to the next stage in the promotion flow. The value of this field could be one of the following: * The last segment of a target name. It only needs the ID to determine if the target is one of the stages in the promotion sequence defined in the pipeline. * \"@next\", the next target in the promotion sequence."]
    pub fn destination_target_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_target_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nRequired. ID of the rule. This id must be unique in the 'Automation' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `wait` after provisioning.\nOptional. How long the release need to be paused until being promoted to the next target."]
    pub fn wait(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRetryEl {
    attempts: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backoff_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait: Option<PrimField<String>>,
}
impl ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRetryEl {
    #[doc = "Set the field `backoff_mode`.\nOptional. The pattern of how wait time will be increased. Default is linear. Backoff mode will be ignored if wait is 0. Possible values: [\"BACKOFF_MODE_UNSPECIFIED\", \"BACKOFF_MODE_LINEAR\", \"BACKOFF_MODE_EXPONENTIAL\"]"]
    pub fn set_backoff_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backoff_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `wait`.\nOptional. How long to wait for the first retry. Default is 0, and the maximum value is 14d. A duration in seconds with up to nine fractional digits, ending with 's'. Example: '3.5s'."]
    pub fn set_wait(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.wait = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRetryEl {
    type O = BlockAssignable<ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRetryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRetryEl {
    #[doc = "Required. Total number of retries. Retry is skipped if set to 0; The minimum value is 1, and the maximum value is 10."]
    pub attempts: PrimField<String>,
}
impl BuildClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRetryEl {
    pub fn build(self) -> ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRetryEl {
        ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRetryEl {
            attempts: self.attempts,
            backoff_mode: core::default::Default::default(),
            wait: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRetryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRetryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRetryElRef {
        ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRetryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRetryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attempts` after provisioning.\nRequired. Total number of retries. Retry is skipped if set to 0; The minimum value is 1, and the maximum value is 10."]
    pub fn attempts(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attempts", self.base))
    }
    #[doc = "Get a reference to the value of field `backoff_mode` after provisioning.\nOptional. The pattern of how wait time will be increased. Default is linear. Backoff mode will be ignored if wait is 0. Possible values: [\"BACKOFF_MODE_UNSPECIFIED\", \"BACKOFF_MODE_LINEAR\", \"BACKOFF_MODE_EXPONENTIAL\"]"]
    pub fn backoff_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backoff_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `wait` after provisioning.\nOptional. How long to wait for the first retry. Default is 0, and the maximum value is 14d. A duration in seconds with up to nine fractional digits, ending with 's'. Example: '3.5s'."]
    pub fn wait(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRollbackEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_phase: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_rollback_if_rollout_pending: Option<PrimField<bool>>,
}
impl ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRollbackEl {
    #[doc = "Set the field `destination_phase`.\nOptional. The starting phase ID for the Rollout. If unspecified, the Rollout will start in the stable phase."]
    pub fn set_destination_phase(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_phase = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_rollback_if_rollout_pending`.\nOptional. If pending rollout exists on the target, the rollback operation will be aborted."]
    pub fn set_disable_rollback_if_rollout_pending(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.disable_rollback_if_rollout_pending = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRollbackEl {
    type O =
        BlockAssignable<ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRollbackEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRollbackEl {}
impl BuildClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRollbackEl {
    pub fn build(self) -> ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRollbackEl {
        ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRollbackEl {
            destination_phase: core::default::Default::default(),
            disable_rollback_if_rollout_pending: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRollbackElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRollbackElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRollbackElRef {
        ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRollbackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRollbackElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `destination_phase` after provisioning.\nOptional. The starting phase ID for the Rollout. If unspecified, the Rollout will start in the stable phase."]
    pub fn destination_phase(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_phase", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_rollback_if_rollout_pending` after provisioning.\nOptional. If pending rollout exists on the target, the rollback operation will be aborted."]
    pub fn disable_rollback_if_rollout_pending(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_rollback_if_rollout_pending", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElDynamic {
    retry:
        Option<DynamicBlock<ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRetryEl>>,
    rollback: Option<
        DynamicBlock<ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRollbackEl>,
    >,
}
#[derive(Serialize)]
pub struct ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    retry: Option<Vec<ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRetryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rollback: Option<Vec<ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRollbackEl>>,
    dynamic: ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElDynamic,
}
impl ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesEl {
    #[doc = "Set the field `retry`.\n"]
    pub fn set_retry(
        mut self,
        v: impl Into<
            BlockAssignable<ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRetryEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retry = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retry = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rollback`.\n"]
    pub fn set_rollback(
        mut self,
        v: impl Into<
            BlockAssignable<
                ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRollbackEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rollback = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rollback = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesEl {
    type O = BlockAssignable<ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesEl {}
impl BuildClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesEl {
    pub fn build(self) -> ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesEl {
        ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesEl {
            retry: core::default::Default::default(),
            rollback: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRef {
        ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `retry` after provisioning.\n"]
    pub fn retry(
        &self,
    ) -> ListRef<ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRetryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry", self.base))
    }
    #[doc = "Get a reference to the value of field `rollback` after provisioning.\n"]
    pub fn rollback(
        &self,
    ) -> ListRef<ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRollbackElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rollback", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployAutomationRulesElRepairRolloutRuleElDynamic {
    repair_phases:
        Option<DynamicBlock<ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesEl>>,
}
#[derive(Serialize)]
pub struct ClouddeployAutomationRulesElRepairRolloutRuleEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jobs: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phases: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repair_phases: Option<Vec<ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesEl>>,
    dynamic: ClouddeployAutomationRulesElRepairRolloutRuleElDynamic,
}
impl ClouddeployAutomationRulesElRepairRolloutRuleEl {
    #[doc = "Set the field `jobs`.\nOptional. Jobs to repair. Proceeds only after job name matched any one in the list, or for all jobs if unspecified or empty. The phase that includes the job must match the phase ID specified in sourcePhase. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: ^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$."]
    pub fn set_jobs(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.jobs = Some(v.into());
        self
    }
    #[doc = "Set the field `phases`.\nOptional. Phases within which jobs are subject to automatic repair actions on failure. Proceeds only after phase name matched any one in the list, or for all phases if unspecified. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: ^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$."]
    pub fn set_phases(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.phases = Some(v.into());
        self
    }
    #[doc = "Set the field `repair_phases`.\n"]
    pub fn set_repair_phases(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.repair_phases = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.repair_phases = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployAutomationRulesElRepairRolloutRuleEl {
    type O = BlockAssignable<ClouddeployAutomationRulesElRepairRolloutRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployAutomationRulesElRepairRolloutRuleEl {
    #[doc = "Required. ID of the rule. This id must be unique in the 'Automation' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub id: PrimField<String>,
}
impl BuildClouddeployAutomationRulesElRepairRolloutRuleEl {
    pub fn build(self) -> ClouddeployAutomationRulesElRepairRolloutRuleEl {
        ClouddeployAutomationRulesElRepairRolloutRuleEl {
            id: self.id,
            jobs: core::default::Default::default(),
            phases: core::default::Default::default(),
            repair_phases: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployAutomationRulesElRepairRolloutRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployAutomationRulesElRepairRolloutRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployAutomationRulesElRepairRolloutRuleElRef {
        ClouddeployAutomationRulesElRepairRolloutRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployAutomationRulesElRepairRolloutRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nRequired. ID of the rule. This id must be unique in the 'Automation' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `jobs` after provisioning.\nOptional. Jobs to repair. Proceeds only after job name matched any one in the list, or for all jobs if unspecified or empty. The phase that includes the job must match the phase ID specified in sourcePhase. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: ^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$."]
    pub fn jobs(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.jobs", self.base))
    }
    #[doc = "Get a reference to the value of field `phases` after provisioning.\nOptional. Phases within which jobs are subject to automatic repair actions on failure. Proceeds only after phase name matched any one in the list, or for all phases if unspecified. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: ^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$."]
    pub fn phases(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.phases", self.base))
    }
    #[doc = "Get a reference to the value of field `repair_phases` after provisioning.\n"]
    pub fn repair_phases(
        &self,
    ) -> ListRef<ClouddeployAutomationRulesElRepairRolloutRuleElRepairPhasesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.repair_phases", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddeployAutomationRulesElTimedPromoteReleaseRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_phase: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_target_id: Option<PrimField<String>>,
    id: PrimField<String>,
    schedule: PrimField<String>,
    time_zone: PrimField<String>,
}
impl ClouddeployAutomationRulesElTimedPromoteReleaseRuleEl {
    #[doc = "Set the field `destination_phase`.\nOptional. The starting phase of the rollout created by this rule. Default to the first phase."]
    pub fn set_destination_phase(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_phase = Some(v.into());
        self
    }
    #[doc = "Set the field `destination_target_id`.\nOptional. The ID of the stage in the pipeline to which this Release is deploying. If unspecified, default it to the next stage in the promotion flow. The value of this field could be one of the following:\n  - The last segment of a target name\n  - \"@next\", the next target in the promotion sequence\""]
    pub fn set_destination_target_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_target_id = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployAutomationRulesElTimedPromoteReleaseRuleEl {
    type O = BlockAssignable<ClouddeployAutomationRulesElTimedPromoteReleaseRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployAutomationRulesElTimedPromoteReleaseRuleEl {
    #[doc = "Required. ID of the rule. This id must be unique in the 'Automation' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub id: PrimField<String>,
    #[doc = "Required. Schedule in crontab format. e.g. '0 9 * * 1' for every Monday at 9am."]
    pub schedule: PrimField<String>,
    #[doc = "Required. The time zone in IANA format IANA Time Zone Database (e.g. America/New_York)."]
    pub time_zone: PrimField<String>,
}
impl BuildClouddeployAutomationRulesElTimedPromoteReleaseRuleEl {
    pub fn build(self) -> ClouddeployAutomationRulesElTimedPromoteReleaseRuleEl {
        ClouddeployAutomationRulesElTimedPromoteReleaseRuleEl {
            destination_phase: core::default::Default::default(),
            destination_target_id: core::default::Default::default(),
            id: self.id,
            schedule: self.schedule,
            time_zone: self.time_zone,
        }
    }
}
pub struct ClouddeployAutomationRulesElTimedPromoteReleaseRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployAutomationRulesElTimedPromoteReleaseRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployAutomationRulesElTimedPromoteReleaseRuleElRef {
        ClouddeployAutomationRulesElTimedPromoteReleaseRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployAutomationRulesElTimedPromoteReleaseRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `destination_phase` after provisioning.\nOptional. The starting phase of the rollout created by this rule. Default to the first phase."]
    pub fn destination_phase(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_phase", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destination_target_id` after provisioning.\nOptional. The ID of the stage in the pipeline to which this Release is deploying. If unspecified, default it to the next stage in the promotion flow. The value of this field could be one of the following:\n  - The last segment of a target name\n  - \"@next\", the next target in the promotion sequence\""]
    pub fn destination_target_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_target_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nRequired. ID of the rule. This id must be unique in the 'Automation' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\nRequired. Schedule in crontab format. e.g. '0 9 * * 1' for every Monday at 9am."]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\nRequired. The time zone in IANA format IANA Time Zone Database (e.g. America/New_York)."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployAutomationRulesElDynamic {
    advance_rollout_rule: Option<DynamicBlock<ClouddeployAutomationRulesElAdvanceRolloutRuleEl>>,
    promote_release_rule: Option<DynamicBlock<ClouddeployAutomationRulesElPromoteReleaseRuleEl>>,
    repair_rollout_rule: Option<DynamicBlock<ClouddeployAutomationRulesElRepairRolloutRuleEl>>,
    timed_promote_release_rule:
        Option<DynamicBlock<ClouddeployAutomationRulesElTimedPromoteReleaseRuleEl>>,
}
#[derive(Serialize)]
pub struct ClouddeployAutomationRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advance_rollout_rule: Option<Vec<ClouddeployAutomationRulesElAdvanceRolloutRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    promote_release_rule: Option<Vec<ClouddeployAutomationRulesElPromoteReleaseRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repair_rollout_rule: Option<Vec<ClouddeployAutomationRulesElRepairRolloutRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timed_promote_release_rule: Option<Vec<ClouddeployAutomationRulesElTimedPromoteReleaseRuleEl>>,
    dynamic: ClouddeployAutomationRulesElDynamic,
}
impl ClouddeployAutomationRulesEl {
    #[doc = "Set the field `advance_rollout_rule`.\n"]
    pub fn set_advance_rollout_rule(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployAutomationRulesElAdvanceRolloutRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.advance_rollout_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.advance_rollout_rule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `promote_release_rule`.\n"]
    pub fn set_promote_release_rule(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployAutomationRulesElPromoteReleaseRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.promote_release_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.promote_release_rule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `repair_rollout_rule`.\n"]
    pub fn set_repair_rollout_rule(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployAutomationRulesElRepairRolloutRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.repair_rollout_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.repair_rollout_rule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timed_promote_release_rule`.\n"]
    pub fn set_timed_promote_release_rule(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployAutomationRulesElTimedPromoteReleaseRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timed_promote_release_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timed_promote_release_rule = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployAutomationRulesEl {
    type O = BlockAssignable<ClouddeployAutomationRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployAutomationRulesEl {}
impl BuildClouddeployAutomationRulesEl {
    pub fn build(self) -> ClouddeployAutomationRulesEl {
        ClouddeployAutomationRulesEl {
            advance_rollout_rule: core::default::Default::default(),
            promote_release_rule: core::default::Default::default(),
            repair_rollout_rule: core::default::Default::default(),
            timed_promote_release_rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployAutomationRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployAutomationRulesElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployAutomationRulesElRef {
        ClouddeployAutomationRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployAutomationRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `advance_rollout_rule` after provisioning.\n"]
    pub fn advance_rollout_rule(
        &self,
    ) -> ListRef<ClouddeployAutomationRulesElAdvanceRolloutRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advance_rollout_rule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `promote_release_rule` after provisioning.\n"]
    pub fn promote_release_rule(
        &self,
    ) -> ListRef<ClouddeployAutomationRulesElPromoteReleaseRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.promote_release_rule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `repair_rollout_rule` after provisioning.\n"]
    pub fn repair_rollout_rule(
        &self,
    ) -> ListRef<ClouddeployAutomationRulesElRepairRolloutRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.repair_rollout_rule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timed_promote_release_rule` after provisioning.\n"]
    pub fn timed_promote_release_rule(
        &self,
    ) -> ListRef<ClouddeployAutomationRulesElTimedPromoteReleaseRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.timed_promote_release_rule", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddeployAutomationSelectorElTargetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
}
impl ClouddeployAutomationSelectorElTargetsEl {
    #[doc = "Set the field `id`.\nID of the 'Target'. The value of this field could be one of the following: * The last segment of a target name. It only needs the ID to determine which target is being referred to * \"*\", all targets in a location."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nTarget labels."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployAutomationSelectorElTargetsEl {
    type O = BlockAssignable<ClouddeployAutomationSelectorElTargetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployAutomationSelectorElTargetsEl {}
impl BuildClouddeployAutomationSelectorElTargetsEl {
    pub fn build(self) -> ClouddeployAutomationSelectorElTargetsEl {
        ClouddeployAutomationSelectorElTargetsEl {
            id: core::default::Default::default(),
            labels: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployAutomationSelectorElTargetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployAutomationSelectorElTargetsElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployAutomationSelectorElTargetsElRef {
        ClouddeployAutomationSelectorElTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployAutomationSelectorElTargetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the 'Target'. The value of this field could be one of the following: * The last segment of a target name. It only needs the ID to determine which target is being referred to * \"*\", all targets in a location."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nTarget labels."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployAutomationSelectorElDynamic {
    targets: Option<DynamicBlock<ClouddeployAutomationSelectorElTargetsEl>>,
}
#[derive(Serialize)]
pub struct ClouddeployAutomationSelectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    targets: Option<Vec<ClouddeployAutomationSelectorElTargetsEl>>,
    dynamic: ClouddeployAutomationSelectorElDynamic,
}
impl ClouddeployAutomationSelectorEl {
    #[doc = "Set the field `targets`.\n"]
    pub fn set_targets(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployAutomationSelectorElTargetsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.targets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.targets = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployAutomationSelectorEl {
    type O = BlockAssignable<ClouddeployAutomationSelectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployAutomationSelectorEl {}
impl BuildClouddeployAutomationSelectorEl {
    pub fn build(self) -> ClouddeployAutomationSelectorEl {
        ClouddeployAutomationSelectorEl {
            targets: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployAutomationSelectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployAutomationSelectorElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployAutomationSelectorElRef {
        ClouddeployAutomationSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployAutomationSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> ListRef<ClouddeployAutomationSelectorElTargetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.targets", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployAutomationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ClouddeployAutomationTimeoutsEl {
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
impl ToListMappable for ClouddeployAutomationTimeoutsEl {
    type O = BlockAssignable<ClouddeployAutomationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployAutomationTimeoutsEl {}
impl BuildClouddeployAutomationTimeoutsEl {
    pub fn build(self) -> ClouddeployAutomationTimeoutsEl {
        ClouddeployAutomationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployAutomationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployAutomationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployAutomationTimeoutsElRef {
        ClouddeployAutomationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployAutomationTimeoutsElRef {
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
struct ClouddeployAutomationDynamic {
    rules: Option<DynamicBlock<ClouddeployAutomationRulesEl>>,
    selector: Option<DynamicBlock<ClouddeployAutomationSelectorEl>>,
}
