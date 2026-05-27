use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ClouddeployDeployPolicyData {
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
    rules: Option<Vec<ClouddeployDeployPolicyRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selectors: Option<Vec<ClouddeployDeployPolicySelectorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ClouddeployDeployPolicyTimeoutsEl>,
    dynamic: ClouddeployDeployPolicyDynamic,
}
struct ClouddeployDeployPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ClouddeployDeployPolicyData>,
}
#[derive(Clone)]
pub struct ClouddeployDeployPolicy(Rc<ClouddeployDeployPolicy_>);
impl ClouddeployDeployPolicy {
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
    #[doc = "Set the field `annotations`.\nUser annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an optional prefix and name, separated by a slash ('/'). * The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ('[a-z0-9A-Z]') with dashes ('-'), underscores ('_'), dots ('.'), and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots('.'), not longer than 253 characters in total, followed by a slash ('/'). See https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more details.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nDescription of the 'DeployPolicy'. Max length is 255 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 63 characters.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `suspended`.\nWhen suspended, the policy will not prevent actions from occurring, even if the action violates the policy."]
    pub fn set_suspended(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().suspended = Some(v.into());
        self
    }
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl Into<BlockAssignable<ClouddeployDeployPolicyRulesEl>>) -> Self {
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
    #[doc = "Set the field `selectors`.\n"]
    pub fn set_selectors(
        self,
        v: impl Into<BlockAssignable<ClouddeployDeployPolicySelectorsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().selectors = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.selectors = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ClouddeployDeployPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUser annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an optional prefix and name, separated by a slash ('/'). * The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ('[a-z0-9A-Z]') with dashes ('-'), underscores ('_'), dots ('.'), and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots('.'), not longer than 253 characters in total, followed by a slash ('/'). See https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more details.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the DeployPolicy was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the 'DeployPolicy'. Max length is 255 characters."]
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe weak etag of the 'DeployPolicy' resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 63 characters.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the 'DeployPolicy'."]
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
    #[doc = "Get a reference to the value of field `suspended` after provisioning.\nWhen suspended, the policy will not prevent actions from occurring, even if the action violates the policy."]
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
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique identifier of the 'DeployPolicy'."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Time at which the DeployPolicy was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<ClouddeployDeployPolicyRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `selectors` after provisioning.\n"]
    pub fn selectors(&self) -> ListRef<ClouddeployDeployPolicySelectorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.selectors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployDeployPolicyTimeoutsElRef {
        ClouddeployDeployPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ClouddeployDeployPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ClouddeployDeployPolicy {}
impl ToListMappable for ClouddeployDeployPolicy {
    type O = ListRef<ClouddeployDeployPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ClouddeployDeployPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_clouddeploy_deploy_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildClouddeployDeployPolicy {
    pub tf_id: String,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
    #[doc = "Name of the 'DeployPolicy'."]
    pub name: PrimField<String>,
}
impl BuildClouddeployDeployPolicy {
    pub fn build(self, stack: &mut Stack) -> ClouddeployDeployPolicy {
        let out = ClouddeployDeployPolicy(Rc::new(ClouddeployDeployPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ClouddeployDeployPolicyData {
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
                rules: core::default::Default::default(),
                selectors: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ClouddeployDeployPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeployPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ClouddeployDeployPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUser annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an optional prefix and name, separated by a slash ('/'). * The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ('[a-z0-9A-Z]') with dashes ('-'), underscores ('_'), dots ('.'), and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots('.'), not longer than 253 characters in total, followed by a slash ('/'). See https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more details.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the DeployPolicy was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the 'DeployPolicy'. Max length is 255 characters."]
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe weak etag of the 'DeployPolicy' resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 63 characters.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the 'DeployPolicy'."]
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
    #[doc = "Get a reference to the value of field `suspended` after provisioning.\nWhen suspended, the policy will not prevent actions from occurring, even if the action violates the policy."]
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
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique identifier of the 'DeployPolicy'."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Time at which the DeployPolicy was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<ClouddeployDeployPolicyRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `selectors` after provisioning.\n"]
    pub fn selectors(&self) -> ListRef<ClouddeployDeployPolicySelectorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.selectors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployDeployPolicyTimeoutsElRef {
        ClouddeployDeployPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndDateEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<PrimField<f64>>,
}
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndDateEl {
    #[doc = "Set the field `day`.\nDay of a month. Must be from 1 to 31 and valid for the year and month."]
    pub fn set_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.day = Some(v.into());
        self
    }
    #[doc = "Set the field `month`.\nMonth of a year. Must be from 1 to 12."]
    pub fn set_month(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.month = Some(v.into());
        self
    }
    #[doc = "Set the field `year`.\nYear of the date. Must be from 1 to 9999."]
    pub fn set_year(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.year = Some(v.into());
        self
    }
}
impl ToListMappable
    for ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndDateEl
{
    type O = BlockAssignable<
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndDateEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndDateEl
{}
impl BuildClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndDateEl {
    pub fn build(
        self,
    ) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndDateEl
    {
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndDateEl {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndDateElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndDateElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndDateElRef
    {
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndDateElRef { shared : shared , base : base . to_string () , }
    }
}
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndDateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month."]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12."]
    pub fn month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }
    #[doc = "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999."]
    pub fn year(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndTimeEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndTimeEl {
    #[doc = "Set the field `hours`.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\nMinutes of an hour. Must be greater than or equal to 0 and less than or equal to 59."]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
    #[doc = "Set the field `nanos`.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0 and less than or equal to 999,999,999."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\nSeconds of a minute. Must be greater than or equal to 0 and typically must be less than or equal to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable
    for ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndTimeEl
{
    type O = BlockAssignable<
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndTimeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndTimeEl
{}
impl BuildClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndTimeEl {
    pub fn build(
        self,
    ) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndTimeEl
    {
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndTimeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndTimeElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndTimeElRef
    {
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndTimeElRef { shared : shared , base : base . to_string () , }
    }
}
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\nMinutes of an hour. Must be greater than or equal to 0 and less than or equal to 59."]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0 and less than or equal to 999,999,999."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSeconds of a minute. Must be greater than or equal to 0 and typically must be less than or equal to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartDateEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<PrimField<f64>>,
}
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartDateEl {
    #[doc = "Set the field `day`.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub fn set_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.day = Some(v.into());
        self
    }
    #[doc = "Set the field `month`.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub fn set_month(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.month = Some(v.into());
        self
    }
    #[doc = "Set the field `year`.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub fn set_year(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.year = Some(v.into());
        self
    }
}
impl ToListMappable
    for ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartDateEl
{
    type O = BlockAssignable<
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartDateEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartDateEl
{}
impl
    BuildClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartDateEl
{
    pub fn build(
        self,
    ) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartDateEl
    {
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartDateEl {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartDateElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartDateElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartDateElRef { ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartDateElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartDateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub fn month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }
    #[doc = "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub fn year(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartTimeEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartTimeEl {
    #[doc = "Set the field `hours`.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\nMinutes of an hour. Must be greater than or equal to 0 and less than or equal to 59."]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
    #[doc = "Set the field `nanos`.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0 and less than or equal to 999,999,999."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\nSeconds of a minute. Must be greater than or equal to 0 and typically must be less than or equal to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable
    for ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartTimeEl
{
    type O = BlockAssignable<
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartTimeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartTimeEl
{}
impl
    BuildClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartTimeEl
{
    pub fn build(
        self,
    ) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartTimeEl
    {
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartTimeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartTimeElRef { fn new (shared : StackShared , base : String) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartTimeElRef { ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartTimeElRef { shared : shared , base : base . to_string () , } } }
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\nMinutes of an hour. Must be greater than or equal to 0 and less than or equal to 59."]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0 and less than or equal to 999,999,999."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSeconds of a minute. Must be greater than or equal to 0 and typically must be less than or equal to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElDynamic { end_date : Option < DynamicBlock < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndDateEl >> , end_time : Option < DynamicBlock < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndTimeEl >> , start_date : Option < DynamicBlock < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartDateEl >> , start_time : Option < DynamicBlock < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartTimeEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsEl { # [serde (skip_serializing_if = "Option::is_none")] end_date : Option < Vec < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndDateEl > > , # [serde (skip_serializing_if = "Option::is_none")] end_time : Option < Vec < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndTimeEl > > , # [serde (skip_serializing_if = "Option::is_none")] start_date : Option < Vec < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartDateEl > > , # [serde (skip_serializing_if = "Option::is_none")] start_time : Option < Vec < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartTimeEl > > , dynamic : ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElDynamic , }
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsEl {
    #[doc = "Set the field `end_date`.\n"]
    pub fn set_end_date(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndDateEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.end_date = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.end_date = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndTimeEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.end_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.end_time = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `start_date`.\n"]
    pub fn set_start_date(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartDateEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.start_date = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.start_date = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartTimeEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.start_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsEl
{
    type O = BlockAssignable<
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsEl {}
impl BuildClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsEl {
    pub fn build(
        self,
    ) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsEl {
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsEl {
            end_date: core::default::Default::default(),
            end_time: core::default::Default::default(),
            start_date: core::default::Default::default(),
            start_time: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElRef {
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(
        &self,
    ) -> ListRef<
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndDateElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.end_date", self.base))
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(
        &self,
    ) -> ListRef<
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElEndTimeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]    pub fn start_date (& self) -> ListRef < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartDateElRef >{
        ListRef::new(self.shared().clone(), format!("{}.start_date", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]    pub fn start_time (& self) -> ListRef < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElStartTimeElRef >{
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElEndTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElEndTimeEl {
    #[doc = "Set the field `hours`.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\nMinutes of an hour. Must be greater than or equal to 0 and less than or equal to 59."]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
    #[doc = "Set the field `nanos`.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0 and less than or equal to 999,999,999."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\nSeconds of a minute. Must be greater than or equal to 0 and typically must be less than or equal to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable
    for ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElEndTimeEl
{
    type O = BlockAssignable<
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElEndTimeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElEndTimeEl
{}
impl BuildClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElEndTimeEl {
    pub fn build(
        self,
    ) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElEndTimeEl
    {
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElEndTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElEndTimeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElEndTimeElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElEndTimeElRef
    {
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElEndTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElEndTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\nMinutes of an hour. Must be greater than or equal to 0 and less than or equal to 59."]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0 and less than or equal to 999,999,999."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSeconds of a minute. Must be greater than or equal to 0 and typically must be less than or equal to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElStartTimeEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElStartTimeEl {
    #[doc = "Set the field `hours`.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\nMinutes of an hour. Must be greater than or equal to 0 and less than or equal to 59."]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
    #[doc = "Set the field `nanos`.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0 and less than or equal to 999,999,999."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\nSeconds of a minute. Must be greater than or equal to 0 and typically must be less than or equal to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable
    for ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElStartTimeEl
{
    type O = BlockAssignable<
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElStartTimeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElStartTimeEl
{}
impl
    BuildClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElStartTimeEl
{
    pub fn build(
        self,
    ) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElStartTimeEl
    {
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElStartTimeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElStartTimeElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElStartTimeElRef
    {
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElStartTimeElRef { shared : shared , base : base . to_string () , }
    }
}
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\nMinutes of an hour. Must be greater than or equal to 0 and less than or equal to 59."]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0 and less than or equal to 999,999,999."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSeconds of a minute. Must be greater than or equal to 0 and typically must be less than or equal to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElDynamic { end_time : Option < DynamicBlock < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElEndTimeEl >> , start_time : Option < DynamicBlock < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElStartTimeEl >> , }
#[derive(Serialize)]
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsEl { # [serde (skip_serializing_if = "Option::is_none")] days_of_week : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] end_time : Option < Vec < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElEndTimeEl > > , # [serde (skip_serializing_if = "Option::is_none")] start_time : Option < Vec < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElStartTimeEl > > , dynamic : ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElDynamic , }
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsEl {
    #[doc = "Set the field `days_of_week`.\nDays of week. If left empty, all days of the week will be included. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn set_days_of_week(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.days_of_week = Some(v.into());
        self
    }
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElEndTimeEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.end_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.end_time = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElStartTimeEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.start_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsEl
{
    type O = BlockAssignable<
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsEl {}
impl BuildClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsEl {
    pub fn build(
        self,
    ) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsEl {
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsEl {
            days_of_week: core::default::Default::default(),
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElRef {
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `days_of_week` after provisioning.\nDays of week. If left empty, all days of the week will be included. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn days_of_week(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.days_of_week", self.base))
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(
        &self,
    ) -> ListRef<
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElEndTimeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]    pub fn start_time (& self) -> ListRef < ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElStartTimeElRef >{
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElDynamic {
    one_time_windows: Option<
        DynamicBlock<
            ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsEl,
        >,
    >,
    weekly_windows: Option<
        DynamicBlock<
            ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsEl {
    time_zone: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    one_time_windows: Option<
        Vec<ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_windows:
        Option<Vec<ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsEl>>,
    dynamic: ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElDynamic,
}
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsEl {
    #[doc = "Set the field `one_time_windows`.\n"]
    pub fn set_one_time_windows(
        mut self,
        v: impl Into<
            BlockAssignable<
                ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.one_time_windows = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.one_time_windows = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `weekly_windows`.\n"]
    pub fn set_weekly_windows(
        mut self,
        v: impl Into<
            BlockAssignable<
                ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weekly_windows = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weekly_windows = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsEl {
    type O = BlockAssignable<ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsEl {
    #[doc = "The time zone in IANA format IANA Time Zone Database (e.g. America/New_York)."]
    pub time_zone: PrimField<String>,
}
impl BuildClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsEl {
    pub fn build(self) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsEl {
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsEl {
            time_zone: self.time_zone,
            one_time_windows: core::default::Default::default(),
            weekly_windows: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElRef {
        ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone in IANA format IANA Time Zone Database (e.g. America/New_York)."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
    #[doc = "Get a reference to the value of field `one_time_windows` after provisioning.\n"]
    pub fn one_time_windows(
        &self,
    ) -> ListRef<ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElOneTimeWindowsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.one_time_windows", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weekly_windows` after provisioning.\n"]
    pub fn weekly_windows(
        &self,
    ) -> ListRef<ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElWeeklyWindowsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_windows", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ClouddeployDeployPolicyRulesElRolloutRestrictionElDynamic {
    time_windows:
        Option<DynamicBlock<ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsEl>>,
}
#[derive(Serialize)]
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<ListField<PrimField<String>>>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invokers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_windows: Option<Vec<ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsEl>>,
    dynamic: ClouddeployDeployPolicyRulesElRolloutRestrictionElDynamic,
}
impl ClouddeployDeployPolicyRulesElRolloutRestrictionEl {
    #[doc = "Set the field `actions`.\nRollout actions to be restricted as part of the policy. If left empty, all actions will be restricted. Possible values: [\"ADVANCE\", \"APPROVE\", \"CANCEL\", \"CREATE\", \"IGNORE_JOB\", \"RETRY_JOB\", \"ROLLBACK\", \"TERMINATE_JOBRUN\"]"]
    pub fn set_actions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.actions = Some(v.into());
        self
    }
    #[doc = "Set the field `invokers`.\nWhat invoked the action. If left empty, all invoker types will be restricted. Possible values: [\"USER\", \"DEPLOY_AUTOMATION\"]"]
    pub fn set_invokers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.invokers = Some(v.into());
        self
    }
    #[doc = "Set the field `time_windows`.\n"]
    pub fn set_time_windows(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.time_windows = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.time_windows = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployDeployPolicyRulesElRolloutRestrictionEl {
    type O = BlockAssignable<ClouddeployDeployPolicyRulesElRolloutRestrictionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeployPolicyRulesElRolloutRestrictionEl {
    #[doc = "ID of the rule. This id must be unique in the 'DeployPolicy' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub id: PrimField<String>,
}
impl BuildClouddeployDeployPolicyRulesElRolloutRestrictionEl {
    pub fn build(self) -> ClouddeployDeployPolicyRulesElRolloutRestrictionEl {
        ClouddeployDeployPolicyRulesElRolloutRestrictionEl {
            actions: core::default::Default::default(),
            id: self.id,
            invokers: core::default::Default::default(),
            time_windows: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeployPolicyRulesElRolloutRestrictionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeployPolicyRulesElRolloutRestrictionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeployPolicyRulesElRolloutRestrictionElRef {
        ClouddeployDeployPolicyRulesElRolloutRestrictionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeployPolicyRulesElRolloutRestrictionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `actions` after provisioning.\nRollout actions to be restricted as part of the policy. If left empty, all actions will be restricted. Possible values: [\"ADVANCE\", \"APPROVE\", \"CANCEL\", \"CREATE\", \"IGNORE_JOB\", \"RETRY_JOB\", \"ROLLBACK\", \"TERMINATE_JOBRUN\"]"]
    pub fn actions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the rule. This id must be unique in the 'DeployPolicy' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `invokers` after provisioning.\nWhat invoked the action. If left empty, all invoker types will be restricted. Possible values: [\"USER\", \"DEPLOY_AUTOMATION\"]"]
    pub fn invokers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.invokers", self.base))
    }
    #[doc = "Get a reference to the value of field `time_windows` after provisioning.\n"]
    pub fn time_windows(
        &self,
    ) -> ListRef<ClouddeployDeployPolicyRulesElRolloutRestrictionElTimeWindowsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_windows", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployDeployPolicyRulesElDynamic {
    rollout_restriction: Option<DynamicBlock<ClouddeployDeployPolicyRulesElRolloutRestrictionEl>>,
}
#[derive(Serialize)]
pub struct ClouddeployDeployPolicyRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rollout_restriction: Option<Vec<ClouddeployDeployPolicyRulesElRolloutRestrictionEl>>,
    dynamic: ClouddeployDeployPolicyRulesElDynamic,
}
impl ClouddeployDeployPolicyRulesEl {
    #[doc = "Set the field `rollout_restriction`.\n"]
    pub fn set_rollout_restriction(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployDeployPolicyRulesElRolloutRestrictionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rollout_restriction = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rollout_restriction = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployDeployPolicyRulesEl {
    type O = BlockAssignable<ClouddeployDeployPolicyRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeployPolicyRulesEl {}
impl BuildClouddeployDeployPolicyRulesEl {
    pub fn build(self) -> ClouddeployDeployPolicyRulesEl {
        ClouddeployDeployPolicyRulesEl {
            rollout_restriction: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeployPolicyRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeployPolicyRulesElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployDeployPolicyRulesElRef {
        ClouddeployDeployPolicyRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeployPolicyRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rollout_restriction` after provisioning.\n"]
    pub fn rollout_restriction(
        &self,
    ) -> ListRef<ClouddeployDeployPolicyRulesElRolloutRestrictionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rollout_restriction", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeployPolicySelectorsElDeliveryPipelineEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
}
impl ClouddeployDeployPolicySelectorsElDeliveryPipelineEl {
    #[doc = "Set the field `id`.\nID of the DeliveryPipeline. The value of this field could be one of the following:\n- The last segment of a pipeline name\n- \"*\", all delivery pipelines in a location"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nDeliveryPipeline labels."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployDeployPolicySelectorsElDeliveryPipelineEl {
    type O = BlockAssignable<ClouddeployDeployPolicySelectorsElDeliveryPipelineEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeployPolicySelectorsElDeliveryPipelineEl {}
impl BuildClouddeployDeployPolicySelectorsElDeliveryPipelineEl {
    pub fn build(self) -> ClouddeployDeployPolicySelectorsElDeliveryPipelineEl {
        ClouddeployDeployPolicySelectorsElDeliveryPipelineEl {
            id: core::default::Default::default(),
            labels: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployDeployPolicySelectorsElDeliveryPipelineElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeployPolicySelectorsElDeliveryPipelineElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployDeployPolicySelectorsElDeliveryPipelineElRef {
        ClouddeployDeployPolicySelectorsElDeliveryPipelineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeployPolicySelectorsElDeliveryPipelineElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the DeliveryPipeline. The value of this field could be one of the following:\n- The last segment of a pipeline name\n- \"*\", all delivery pipelines in a location"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nDeliveryPipeline labels."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeployPolicySelectorsElTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
}
impl ClouddeployDeployPolicySelectorsElTargetEl {
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
impl ToListMappable for ClouddeployDeployPolicySelectorsElTargetEl {
    type O = BlockAssignable<ClouddeployDeployPolicySelectorsElTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeployPolicySelectorsElTargetEl {}
impl BuildClouddeployDeployPolicySelectorsElTargetEl {
    pub fn build(self) -> ClouddeployDeployPolicySelectorsElTargetEl {
        ClouddeployDeployPolicySelectorsElTargetEl {
            id: core::default::Default::default(),
            labels: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployDeployPolicySelectorsElTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeployPolicySelectorsElTargetElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployDeployPolicySelectorsElTargetElRef {
        ClouddeployDeployPolicySelectorsElTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeployPolicySelectorsElTargetElRef {
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
struct ClouddeployDeployPolicySelectorsElDynamic {
    delivery_pipeline: Option<DynamicBlock<ClouddeployDeployPolicySelectorsElDeliveryPipelineEl>>,
    target: Option<DynamicBlock<ClouddeployDeployPolicySelectorsElTargetEl>>,
}
#[derive(Serialize)]
pub struct ClouddeployDeployPolicySelectorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_pipeline: Option<Vec<ClouddeployDeployPolicySelectorsElDeliveryPipelineEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Vec<ClouddeployDeployPolicySelectorsElTargetEl>>,
    dynamic: ClouddeployDeployPolicySelectorsElDynamic,
}
impl ClouddeployDeployPolicySelectorsEl {
    #[doc = "Set the field `delivery_pipeline`.\n"]
    pub fn set_delivery_pipeline(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployDeployPolicySelectorsElDeliveryPipelineEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.delivery_pipeline = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.delivery_pipeline = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployDeployPolicySelectorsElTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployDeployPolicySelectorsEl {
    type O = BlockAssignable<ClouddeployDeployPolicySelectorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeployPolicySelectorsEl {}
impl BuildClouddeployDeployPolicySelectorsEl {
    pub fn build(self) -> ClouddeployDeployPolicySelectorsEl {
        ClouddeployDeployPolicySelectorsEl {
            delivery_pipeline: core::default::Default::default(),
            target: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployDeployPolicySelectorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeployPolicySelectorsElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployDeployPolicySelectorsElRef {
        ClouddeployDeployPolicySelectorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeployPolicySelectorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delivery_pipeline` after provisioning.\n"]
    pub fn delivery_pipeline(
        &self,
    ) -> ListRef<ClouddeployDeployPolicySelectorsElDeliveryPipelineElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delivery_pipeline", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<ClouddeployDeployPolicySelectorsElTargetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployDeployPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ClouddeployDeployPolicyTimeoutsEl {
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
impl ToListMappable for ClouddeployDeployPolicyTimeoutsEl {
    type O = BlockAssignable<ClouddeployDeployPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployDeployPolicyTimeoutsEl {}
impl BuildClouddeployDeployPolicyTimeoutsEl {
    pub fn build(self) -> ClouddeployDeployPolicyTimeoutsEl {
        ClouddeployDeployPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployDeployPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployDeployPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployDeployPolicyTimeoutsElRef {
        ClouddeployDeployPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployDeployPolicyTimeoutsElRef {
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
struct ClouddeployDeployPolicyDynamic {
    rules: Option<DynamicBlock<ClouddeployDeployPolicyRulesEl>>,
    selectors: Option<DynamicBlock<ClouddeployDeployPolicySelectorsEl>>,
}
