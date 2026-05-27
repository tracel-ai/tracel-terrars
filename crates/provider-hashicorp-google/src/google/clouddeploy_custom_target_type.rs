use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ClouddeployCustomTargetTypeData {
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
    custom_actions: Option<Vec<ClouddeployCustomTargetTypeCustomActionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tasks: Option<Vec<ClouddeployCustomTargetTypeTasksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ClouddeployCustomTargetTypeTimeoutsEl>,
    dynamic: ClouddeployCustomTargetTypeDynamic,
}
struct ClouddeployCustomTargetType_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ClouddeployCustomTargetTypeData>,
}
#[derive(Clone)]
pub struct ClouddeployCustomTargetType(Rc<ClouddeployCustomTargetType_>);
impl ClouddeployCustomTargetType {
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
    #[doc = "Set the field `annotations`.\nUser annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nDescription of the 'CustomTargetType'. Max length is 255 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_actions`.\n"]
    pub fn set_custom_actions(
        self,
        v: impl Into<BlockAssignable<ClouddeployCustomTargetTypeCustomActionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_actions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tasks`.\n"]
    pub fn set_tasks(
        self,
        v: impl Into<BlockAssignable<ClouddeployCustomTargetTypeTasksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tasks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tasks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ClouddeployCustomTargetTypeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUser annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime at which the 'CustomTargetType' was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_target_type_id` after provisioning.\nResource id of the 'CustomTargetType'."]
    pub fn custom_target_type_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_target_type_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the 'CustomTargetType'. Max length is 255 characters."]
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe weak etag of the 'CustomTargetType' resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the source."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the 'CustomTargetType'."]
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique identifier of the 'CustomTargetType'."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime at which the 'CustomTargetType' was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_actions` after provisioning.\n"]
    pub fn custom_actions(&self) -> ListRef<ClouddeployCustomTargetTypeCustomActionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tasks` after provisioning.\n"]
    pub fn tasks(&self) -> ListRef<ClouddeployCustomTargetTypeTasksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tasks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployCustomTargetTypeTimeoutsElRef {
        ClouddeployCustomTargetTypeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ClouddeployCustomTargetType {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ClouddeployCustomTargetType {}
impl ToListMappable for ClouddeployCustomTargetType {
    type O = ListRef<ClouddeployCustomTargetTypeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ClouddeployCustomTargetType_ {
    fn extract_resource_type(&self) -> String {
        "google_clouddeploy_custom_target_type".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildClouddeployCustomTargetType {
    pub tf_id: String,
    #[doc = "The location of the source."]
    pub location: PrimField<String>,
    #[doc = "Name of the 'CustomTargetType'."]
    pub name: PrimField<String>,
}
impl BuildClouddeployCustomTargetType {
    pub fn build(self, stack: &mut Stack) -> ClouddeployCustomTargetType {
        let out = ClouddeployCustomTargetType(Rc::new(ClouddeployCustomTargetType_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ClouddeployCustomTargetTypeData {
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
                custom_actions: core::default::Default::default(),
                tasks: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ClouddeployCustomTargetTypeRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployCustomTargetTypeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ClouddeployCustomTargetTypeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUser annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime at which the 'CustomTargetType' was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_target_type_id` after provisioning.\nResource id of the 'CustomTargetType'."]
    pub fn custom_target_type_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_target_type_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the 'CustomTargetType'. Max length is 255 characters."]
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe weak etag of the 'CustomTargetType' resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the source."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the 'CustomTargetType'."]
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique identifier of the 'CustomTargetType'."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime at which the 'CustomTargetType' was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_actions` after provisioning.\n"]
    pub fn custom_actions(&self) -> ListRef<ClouddeployCustomTargetTypeCustomActionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tasks` after provisioning.\n"]
    pub fn tasks(&self) -> ListRef<ClouddeployCustomTargetTypeTasksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tasks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployCustomTargetTypeTimeoutsElRef {
        ClouddeployCustomTargetTypeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    ref_: Option<PrimField<String>>,
    repo: PrimField<String>,
}
impl ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGitEl {
    #[doc = "Set the field `path`.\nRelative path from the repository root to the Skaffold file."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `ref_`.\nGit ref the package should be cloned from."]
    pub fn set_ref(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ref_ = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGitEl {
    type O =
        BlockAssignable<ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGitEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGitEl {
    #[doc = "Git repository the package should be cloned from."]
    pub repo: PrimField<String>,
}
impl BuildClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGitEl {
    pub fn build(self) -> ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGitEl {
        ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGitEl {
            path: core::default::Default::default(),
            ref_: core::default::Default::default(),
            repo: self.repo,
        }
    }
}
pub struct ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGitElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGitElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGitElRef {
        ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nRelative path from the repository root to the Skaffold file."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `ref_` after provisioning.\nGit ref the package should be cloned from."]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }
    #[doc = "Get a reference to the value of field `repo` after provisioning.\nGit repository the package should be cloned from."]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudBuildRepoEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    ref_: Option<PrimField<String>>,
    repository: PrimField<String>,
}
impl ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudBuildRepoEl {
    #[doc = "Set the field `path`.\nRelative path from the repository root to the Skaffold file."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `ref_`.\nBranch or tag to use when cloning the repository."]
    pub fn set_ref(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ref_ = Some(v.into());
        self
    }
}
impl ToListMappable
    for ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudBuildRepoEl
{
    type O = BlockAssignable<
        ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudBuildRepoEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudBuildRepoEl
{
    #[doc = "Cloud Build 2nd gen repository in the format of 'projects/<project>/locations/<location>/connections/<connection>/repositories/<repository>'."]
    pub repository: PrimField<String>,
}
impl BuildClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudBuildRepoEl {
    pub fn build(
        self,
    ) -> ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudBuildRepoEl
    {
        ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudBuildRepoEl {
            path: core::default::Default::default(),
            ref_: core::default::Default::default(),
            repository: self.repository,
        }
    }
}
pub struct ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudBuildRepoElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudBuildRepoElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudBuildRepoElRef
    {
        ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudBuildRepoElRef { shared : shared , base : base . to_string () , }
    }
}
impl ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudBuildRepoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nRelative path from the repository root to the Skaffold file."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `ref_` after provisioning.\nBranch or tag to use when cloning the repository."]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\nCloud Build 2nd gen repository in the format of 'projects/<project>/locations/<location>/connections/<connection>/repositories/<repository>'."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudStorageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    source: PrimField<String>,
}
impl ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudStorageEl {
    #[doc = "Set the field `path`.\nRelative path from the source to the Skaffold file."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}
impl ToListMappable
    for ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudStorageEl
{
    type O = BlockAssignable<
        ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudStorageEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudStorageEl
{
    #[doc = "Cloud Storage source paths to copy recursively. For example, providing 'gs://my-bucket/dir/configs/*' will result in Skaffold copying all files within the 'dir/configs' directory in the bucket 'my-bucket'."]
    pub source: PrimField<String>,
}
impl BuildClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudStorageEl {
    pub fn build(
        self,
    ) -> ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudStorageEl
    {
        ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudStorageEl {
            path: core::default::Default::default(),
            source: self.source,
        }
    }
}
pub struct ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudStorageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudStorageElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudStorageElRef
    {
        ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nRelative path from the source to the Skaffold file."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\nCloud Storage source paths to copy recursively. For example, providing 'gs://my-bucket/dir/configs/*' will result in Skaffold copying all files within the 'dir/configs' directory in the bucket 'my-bucket'."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElDynamic { git : Option < DynamicBlock < ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGitEl >> , google_cloud_build_repo : Option < DynamicBlock < ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudBuildRepoEl >> , google_cloud_storage : Option < DynamicBlock < ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudStorageEl >> , }
#[derive(Serialize)]
pub struct ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesEl { # [serde (skip_serializing_if = "Option::is_none")] configs : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] git : Option < Vec < ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGitEl > > , # [serde (skip_serializing_if = "Option::is_none")] google_cloud_build_repo : Option < Vec < ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudBuildRepoEl > > , # [serde (skip_serializing_if = "Option::is_none")] google_cloud_storage : Option < Vec < ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudStorageEl > > , dynamic : ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElDynamic , }
impl ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesEl {
    #[doc = "Set the field `configs`.\nThe Skaffold Config modules to use from the specified source."]
    pub fn set_configs(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.configs = Some(v.into());
        self
    }
    #[doc = "Set the field `git`.\n"]
    pub fn set_git(
        mut self,
        v: impl Into<
            BlockAssignable<
                ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGitEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.git = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.git = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `google_cloud_build_repo`.\n"]
    pub fn set_google_cloud_build_repo(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudBuildRepoEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.google_cloud_build_repo = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.google_cloud_build_repo = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `google_cloud_storage`.\n"]
    pub fn set_google_cloud_storage(
        mut self,
        v : impl Into < BlockAssignable < ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudStorageEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.google_cloud_storage = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.google_cloud_storage = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesEl {
    type O = BlockAssignable<ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesEl {}
impl BuildClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesEl {
    pub fn build(self) -> ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesEl {
        ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesEl {
            configs: core::default::Default::default(),
            git: core::default::Default::default(),
            google_cloud_build_repo: core::default::Default::default(),
            google_cloud_storage: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElRef {
        ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `configs` after provisioning.\nThe Skaffold Config modules to use from the specified source."]
    pub fn configs(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.configs", self.base))
    }
    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(
        &self,
    ) -> ListRef<ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }
    #[doc = "Get a reference to the value of field `google_cloud_build_repo` after provisioning.\n"]
    pub fn google_cloud_build_repo(
        &self,
    ) -> ListRef<
        ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudBuildRepoElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_cloud_build_repo", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `google_cloud_storage` after provisioning.\n"]
    pub fn google_cloud_storage(
        &self,
    ) -> ListRef<
        ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElGoogleCloudStorageElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_cloud_storage", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ClouddeployCustomTargetTypeCustomActionsElDynamic {
    include_skaffold_modules:
        Option<DynamicBlock<ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesEl>>,
}
#[derive(Serialize)]
pub struct ClouddeployCustomTargetTypeCustomActionsEl {
    deploy_action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    render_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_skaffold_modules:
        Option<Vec<ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesEl>>,
    dynamic: ClouddeployCustomTargetTypeCustomActionsElDynamic,
}
impl ClouddeployCustomTargetTypeCustomActionsEl {
    #[doc = "Set the field `render_action`.\nThe Skaffold custom action responsible for render operations. If not provided then Cloud Deploy will perform the render operations via 'skaffold render'."]
    pub fn set_render_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.render_action = Some(v.into());
        self
    }
    #[doc = "Set the field `include_skaffold_modules`.\n"]
    pub fn set_include_skaffold_modules(
        mut self,
        v: impl Into<
            BlockAssignable<ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.include_skaffold_modules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.include_skaffold_modules = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployCustomTargetTypeCustomActionsEl {
    type O = BlockAssignable<ClouddeployCustomTargetTypeCustomActionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployCustomTargetTypeCustomActionsEl {
    #[doc = "The Skaffold custom action responsible for deploy operations."]
    pub deploy_action: PrimField<String>,
}
impl BuildClouddeployCustomTargetTypeCustomActionsEl {
    pub fn build(self) -> ClouddeployCustomTargetTypeCustomActionsEl {
        ClouddeployCustomTargetTypeCustomActionsEl {
            deploy_action: self.deploy_action,
            render_action: core::default::Default::default(),
            include_skaffold_modules: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployCustomTargetTypeCustomActionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployCustomTargetTypeCustomActionsElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployCustomTargetTypeCustomActionsElRef {
        ClouddeployCustomTargetTypeCustomActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployCustomTargetTypeCustomActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deploy_action` after provisioning.\nThe Skaffold custom action responsible for deploy operations."]
    pub fn deploy_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deploy_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `render_action` after provisioning.\nThe Skaffold custom action responsible for render operations. If not provided then Cloud Deploy will perform the render operations via 'skaffold render'."]
    pub fn render_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.render_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_skaffold_modules` after provisioning.\n"]
    pub fn include_skaffold_modules(
        &self,
    ) -> ListRef<ClouddeployCustomTargetTypeCustomActionsElIncludeSkaffoldModulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_skaffold_modules", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddeployCustomTargetTypeTasksElDeployElContainerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<RecField<PrimField<String>>>,
    image: PrimField<String>,
}
impl ClouddeployCustomTargetTypeTasksElDeployElContainerEl {
    #[doc = "Set the field `args`.\nArgs is the container arguments to use. This overrides the default arguments defined in the container image."]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `command`.\nCommand is the container entrypoint to use. This overrides the default entrypoint defined in the container image."]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }
    #[doc = "Set the field `env`.\nEnvironment variables that are set in the container."]
    pub fn set_env(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.env = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployCustomTargetTypeTasksElDeployElContainerEl {
    type O = BlockAssignable<ClouddeployCustomTargetTypeTasksElDeployElContainerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployCustomTargetTypeTasksElDeployElContainerEl {
    #[doc = "Image is the container image to use."]
    pub image: PrimField<String>,
}
impl BuildClouddeployCustomTargetTypeTasksElDeployElContainerEl {
    pub fn build(self) -> ClouddeployCustomTargetTypeTasksElDeployElContainerEl {
        ClouddeployCustomTargetTypeTasksElDeployElContainerEl {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            env: core::default::Default::default(),
            image: self.image,
        }
    }
}
pub struct ClouddeployCustomTargetTypeTasksElDeployElContainerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployCustomTargetTypeTasksElDeployElContainerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployCustomTargetTypeTasksElDeployElContainerElRef {
        ClouddeployCustomTargetTypeTasksElDeployElContainerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployCustomTargetTypeTasksElDeployElContainerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nArgs is the container arguments to use. This overrides the default arguments defined in the container image."]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\nCommand is the container entrypoint to use. This overrides the default entrypoint defined in the container image."]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }
    #[doc = "Get a reference to the value of field `env` after provisioning.\nEnvironment variables that are set in the container."]
    pub fn env(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.env", self.base))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\nImage is the container image to use."]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployCustomTargetTypeTasksElDeployElDynamic {
    container: Option<DynamicBlock<ClouddeployCustomTargetTypeTasksElDeployElContainerEl>>,
}
#[derive(Serialize)]
pub struct ClouddeployCustomTargetTypeTasksElDeployEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container: Option<Vec<ClouddeployCustomTargetTypeTasksElDeployElContainerEl>>,
    dynamic: ClouddeployCustomTargetTypeTasksElDeployElDynamic,
}
impl ClouddeployCustomTargetTypeTasksElDeployEl {
    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployCustomTargetTypeTasksElDeployElContainerEl>>,
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
impl ToListMappable for ClouddeployCustomTargetTypeTasksElDeployEl {
    type O = BlockAssignable<ClouddeployCustomTargetTypeTasksElDeployEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployCustomTargetTypeTasksElDeployEl {}
impl BuildClouddeployCustomTargetTypeTasksElDeployEl {
    pub fn build(self) -> ClouddeployCustomTargetTypeTasksElDeployEl {
        ClouddeployCustomTargetTypeTasksElDeployEl {
            container: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployCustomTargetTypeTasksElDeployElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployCustomTargetTypeTasksElDeployElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployCustomTargetTypeTasksElDeployElRef {
        ClouddeployCustomTargetTypeTasksElDeployElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployCustomTargetTypeTasksElDeployElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> ListRef<ClouddeployCustomTargetTypeTasksElDeployElContainerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.container", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployCustomTargetTypeTasksElRenderElContainerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<RecField<PrimField<String>>>,
    image: PrimField<String>,
}
impl ClouddeployCustomTargetTypeTasksElRenderElContainerEl {
    #[doc = "Set the field `args`.\nArgs is the container arguments to use. This overrides the default arguments defined in the container image."]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `command`.\nCommand is the container entrypoint to use. This overrides the default entrypoint defined in the container image."]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }
    #[doc = "Set the field `env`.\nEnvironment variables that are set in the container."]
    pub fn set_env(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.env = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployCustomTargetTypeTasksElRenderElContainerEl {
    type O = BlockAssignable<ClouddeployCustomTargetTypeTasksElRenderElContainerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployCustomTargetTypeTasksElRenderElContainerEl {
    #[doc = "Image is the container image to use."]
    pub image: PrimField<String>,
}
impl BuildClouddeployCustomTargetTypeTasksElRenderElContainerEl {
    pub fn build(self) -> ClouddeployCustomTargetTypeTasksElRenderElContainerEl {
        ClouddeployCustomTargetTypeTasksElRenderElContainerEl {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            env: core::default::Default::default(),
            image: self.image,
        }
    }
}
pub struct ClouddeployCustomTargetTypeTasksElRenderElContainerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployCustomTargetTypeTasksElRenderElContainerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployCustomTargetTypeTasksElRenderElContainerElRef {
        ClouddeployCustomTargetTypeTasksElRenderElContainerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployCustomTargetTypeTasksElRenderElContainerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nArgs is the container arguments to use. This overrides the default arguments defined in the container image."]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\nCommand is the container entrypoint to use. This overrides the default entrypoint defined in the container image."]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }
    #[doc = "Get a reference to the value of field `env` after provisioning.\nEnvironment variables that are set in the container."]
    pub fn env(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.env", self.base))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\nImage is the container image to use."]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployCustomTargetTypeTasksElRenderElDynamic {
    container: Option<DynamicBlock<ClouddeployCustomTargetTypeTasksElRenderElContainerEl>>,
}
#[derive(Serialize)]
pub struct ClouddeployCustomTargetTypeTasksElRenderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container: Option<Vec<ClouddeployCustomTargetTypeTasksElRenderElContainerEl>>,
    dynamic: ClouddeployCustomTargetTypeTasksElRenderElDynamic,
}
impl ClouddeployCustomTargetTypeTasksElRenderEl {
    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployCustomTargetTypeTasksElRenderElContainerEl>>,
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
impl ToListMappable for ClouddeployCustomTargetTypeTasksElRenderEl {
    type O = BlockAssignable<ClouddeployCustomTargetTypeTasksElRenderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployCustomTargetTypeTasksElRenderEl {}
impl BuildClouddeployCustomTargetTypeTasksElRenderEl {
    pub fn build(self) -> ClouddeployCustomTargetTypeTasksElRenderEl {
        ClouddeployCustomTargetTypeTasksElRenderEl {
            container: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployCustomTargetTypeTasksElRenderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployCustomTargetTypeTasksElRenderElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployCustomTargetTypeTasksElRenderElRef {
        ClouddeployCustomTargetTypeTasksElRenderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployCustomTargetTypeTasksElRenderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> ListRef<ClouddeployCustomTargetTypeTasksElRenderElContainerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.container", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployCustomTargetTypeTasksElDynamic {
    deploy: Option<DynamicBlock<ClouddeployCustomTargetTypeTasksElDeployEl>>,
    render: Option<DynamicBlock<ClouddeployCustomTargetTypeTasksElRenderEl>>,
}
#[derive(Serialize)]
pub struct ClouddeployCustomTargetTypeTasksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy: Option<Vec<ClouddeployCustomTargetTypeTasksElDeployEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    render: Option<Vec<ClouddeployCustomTargetTypeTasksElRenderEl>>,
    dynamic: ClouddeployCustomTargetTypeTasksElDynamic,
}
impl ClouddeployCustomTargetTypeTasksEl {
    #[doc = "Set the field `deploy`.\n"]
    pub fn set_deploy(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployCustomTargetTypeTasksElDeployEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.deploy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.deploy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `render`.\n"]
    pub fn set_render(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployCustomTargetTypeTasksElRenderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.render = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.render = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployCustomTargetTypeTasksEl {
    type O = BlockAssignable<ClouddeployCustomTargetTypeTasksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployCustomTargetTypeTasksEl {}
impl BuildClouddeployCustomTargetTypeTasksEl {
    pub fn build(self) -> ClouddeployCustomTargetTypeTasksEl {
        ClouddeployCustomTargetTypeTasksEl {
            deploy: core::default::Default::default(),
            render: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployCustomTargetTypeTasksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployCustomTargetTypeTasksElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployCustomTargetTypeTasksElRef {
        ClouddeployCustomTargetTypeTasksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployCustomTargetTypeTasksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deploy` after provisioning.\n"]
    pub fn deploy(&self) -> ListRef<ClouddeployCustomTargetTypeTasksElDeployElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deploy", self.base))
    }
    #[doc = "Get a reference to the value of field `render` after provisioning.\n"]
    pub fn render(&self) -> ListRef<ClouddeployCustomTargetTypeTasksElRenderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.render", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployCustomTargetTypeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ClouddeployCustomTargetTypeTimeoutsEl {
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
impl ToListMappable for ClouddeployCustomTargetTypeTimeoutsEl {
    type O = BlockAssignable<ClouddeployCustomTargetTypeTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployCustomTargetTypeTimeoutsEl {}
impl BuildClouddeployCustomTargetTypeTimeoutsEl {
    pub fn build(self) -> ClouddeployCustomTargetTypeTimeoutsEl {
        ClouddeployCustomTargetTypeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployCustomTargetTypeTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployCustomTargetTypeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployCustomTargetTypeTimeoutsElRef {
        ClouddeployCustomTargetTypeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployCustomTargetTypeTimeoutsElRef {
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
struct ClouddeployCustomTargetTypeDynamic {
    custom_actions: Option<DynamicBlock<ClouddeployCustomTargetTypeCustomActionsEl>>,
    tasks: Option<DynamicBlock<ClouddeployCustomTargetTypeTasksEl>>,
}
