use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ClouddeployTargetData {
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
    deploy_parameters: Option<RecField<PrimField<String>>>,
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
    require_approval: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anthos_cluster: Option<Vec<ClouddeployTargetAnthosClusterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    associated_entities: Option<Vec<ClouddeployTargetAssociatedEntitiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_target: Option<Vec<ClouddeployTargetCustomTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_configs: Option<Vec<ClouddeployTargetExecutionConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gke: Option<Vec<ClouddeployTargetGkeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_target: Option<Vec<ClouddeployTargetMultiTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run: Option<Vec<ClouddeployTargetRunEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ClouddeployTargetTimeoutsEl>,
    dynamic: ClouddeployTargetDynamic,
}
struct ClouddeployTarget_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ClouddeployTargetData>,
}
#[derive(Clone)]
pub struct ClouddeployTarget(Rc<ClouddeployTarget_>);
impl ClouddeployTarget {
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
    #[doc = "Set the field `annotations`.\nOptional. User annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deploy_parameters`.\nOptional. The deploy parameters to use for this target."]
    pub fn set_deploy_parameters(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().deploy_parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nOptional. Description of the `Target`. Max length is 255 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nOptional. Labels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `require_approval`.\nOptional. Whether or not the `Target` requires approval."]
    pub fn set_require_approval(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_approval = Some(v.into());
        self
    }
    #[doc = "Set the field `anthos_cluster`.\n"]
    pub fn set_anthos_cluster(
        self,
        v: impl Into<BlockAssignable<ClouddeployTargetAnthosClusterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().anthos_cluster = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.anthos_cluster = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `associated_entities`.\n"]
    pub fn set_associated_entities(
        self,
        v: impl Into<BlockAssignable<ClouddeployTargetAssociatedEntitiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().associated_entities = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.associated_entities = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `custom_target`.\n"]
    pub fn set_custom_target(
        self,
        v: impl Into<BlockAssignable<ClouddeployTargetCustomTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `execution_configs`.\n"]
    pub fn set_execution_configs(
        self,
        v: impl Into<BlockAssignable<ClouddeployTargetExecutionConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().execution_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.execution_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gke`.\n"]
    pub fn set_gke(self, v: impl Into<BlockAssignable<ClouddeployTargetGkeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gke = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gke = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `multi_target`.\n"]
    pub fn set_multi_target(
        self,
        v: impl Into<BlockAssignable<ClouddeployTargetMultiTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().multi_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.multi_target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `run`.\n"]
    pub fn set_run(self, v: impl Into<BlockAssignable<ClouddeployTargetRunEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().run = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.run = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ClouddeployTargetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nOptional. User annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the `Target` was created."]
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
    #[doc = "Get a reference to the value of field `deploy_parameters` after provisioning.\nOptional. The deploy parameters to use for this target."]
    pub fn deploy_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.deploy_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the `Target`. Max length is 255 characters."]
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nOptional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the `Target`. Format is `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`."]
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
    #[doc = "Get a reference to the value of field `require_approval` after provisioning.\nOptional. Whether or not the `Target` requires approval."]
    pub fn require_approval(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_approval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_id` after provisioning.\nOutput only. Resource id of the `Target`."]
    pub fn target_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique identifier of the `Target`."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Most recent time at which the `Target` was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `anthos_cluster` after provisioning.\n"]
    pub fn anthos_cluster(&self) -> ListRef<ClouddeployTargetAnthosClusterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.anthos_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_target` after provisioning.\n"]
    pub fn custom_target(&self) -> ListRef<ClouddeployTargetCustomTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_configs` after provisioning.\n"]
    pub fn execution_configs(&self) -> ListRef<ClouddeployTargetExecutionConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gke` after provisioning.\n"]
    pub fn gke(&self) -> ListRef<ClouddeployTargetGkeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gke", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `multi_target` after provisioning.\n"]
    pub fn multi_target(&self) -> ListRef<ClouddeployTargetMultiTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multi_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `run` after provisioning.\n"]
    pub fn run(&self) -> ListRef<ClouddeployTargetRunElRef> {
        ListRef::new(self.shared().clone(), format!("{}.run", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployTargetTimeoutsElRef {
        ClouddeployTargetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ClouddeployTarget {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ClouddeployTarget {}
impl ToListMappable for ClouddeployTarget {
    type O = ListRef<ClouddeployTargetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ClouddeployTarget_ {
    fn extract_resource_type(&self) -> String {
        "google_clouddeploy_target".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildClouddeployTarget {
    pub tf_id: String,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
    #[doc = "Name of the `Target`. Format is `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`."]
    pub name: PrimField<String>,
}
impl BuildClouddeployTarget {
    pub fn build(self, stack: &mut Stack) -> ClouddeployTarget {
        let out = ClouddeployTarget(Rc::new(ClouddeployTarget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ClouddeployTargetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deploy_parameters: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                require_approval: core::default::Default::default(),
                anthos_cluster: core::default::Default::default(),
                associated_entities: core::default::Default::default(),
                custom_target: core::default::Default::default(),
                execution_configs: core::default::Default::default(),
                gke: core::default::Default::default(),
                multi_target: core::default::Default::default(),
                run: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ClouddeployTargetRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployTargetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ClouddeployTargetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nOptional. User annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the `Target` was created."]
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
    #[doc = "Get a reference to the value of field `deploy_parameters` after provisioning.\nOptional. The deploy parameters to use for this target."]
    pub fn deploy_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.deploy_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the `Target`. Max length is 255 characters."]
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nOptional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the `Target`. Format is `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`."]
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
    #[doc = "Get a reference to the value of field `require_approval` after provisioning.\nOptional. Whether or not the `Target` requires approval."]
    pub fn require_approval(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_approval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_id` after provisioning.\nOutput only. Resource id of the `Target`."]
    pub fn target_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique identifier of the `Target`."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Most recent time at which the `Target` was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `anthos_cluster` after provisioning.\n"]
    pub fn anthos_cluster(&self) -> ListRef<ClouddeployTargetAnthosClusterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.anthos_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_target` after provisioning.\n"]
    pub fn custom_target(&self) -> ListRef<ClouddeployTargetCustomTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_configs` after provisioning.\n"]
    pub fn execution_configs(&self) -> ListRef<ClouddeployTargetExecutionConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gke` after provisioning.\n"]
    pub fn gke(&self) -> ListRef<ClouddeployTargetGkeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gke", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `multi_target` after provisioning.\n"]
    pub fn multi_target(&self) -> ListRef<ClouddeployTargetMultiTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multi_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `run` after provisioning.\n"]
    pub fn run(&self) -> ListRef<ClouddeployTargetRunElRef> {
        ListRef::new(self.shared().clone(), format!("{}.run", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployTargetTimeoutsElRef {
        ClouddeployTargetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddeployTargetAnthosClusterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    membership: Option<PrimField<String>>,
}
impl ClouddeployTargetAnthosClusterEl {
    #[doc = "Set the field `membership`.\nMembership of the GKE Hub-registered cluster to which to apply the Skaffold configuration. Format is `projects/{project}/locations/{location}/memberships/{membership_name}`."]
    pub fn set_membership(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.membership = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployTargetAnthosClusterEl {
    type O = BlockAssignable<ClouddeployTargetAnthosClusterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployTargetAnthosClusterEl {}
impl BuildClouddeployTargetAnthosClusterEl {
    pub fn build(self) -> ClouddeployTargetAnthosClusterEl {
        ClouddeployTargetAnthosClusterEl {
            membership: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployTargetAnthosClusterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployTargetAnthosClusterElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployTargetAnthosClusterElRef {
        ClouddeployTargetAnthosClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployTargetAnthosClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `membership` after provisioning.\nMembership of the GKE Hub-registered cluster to which to apply the Skaffold configuration. Format is `projects/{project}/locations/{location}/memberships/{membership_name}`."]
    pub fn membership(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployTargetAssociatedEntitiesElAnthosClustersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    membership: Option<PrimField<String>>,
}
impl ClouddeployTargetAssociatedEntitiesElAnthosClustersEl {
    #[doc = "Set the field `membership`.\nOptional. Membership of the GKE Hub-registered cluster to which to apply the Skaffold configuration. Format is `projects/{project}/locations/{location}/memberships/{membership_name}`."]
    pub fn set_membership(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.membership = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployTargetAssociatedEntitiesElAnthosClustersEl {
    type O = BlockAssignable<ClouddeployTargetAssociatedEntitiesElAnthosClustersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployTargetAssociatedEntitiesElAnthosClustersEl {}
impl BuildClouddeployTargetAssociatedEntitiesElAnthosClustersEl {
    pub fn build(self) -> ClouddeployTargetAssociatedEntitiesElAnthosClustersEl {
        ClouddeployTargetAssociatedEntitiesElAnthosClustersEl {
            membership: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployTargetAssociatedEntitiesElAnthosClustersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployTargetAssociatedEntitiesElAnthosClustersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployTargetAssociatedEntitiesElAnthosClustersElRef {
        ClouddeployTargetAssociatedEntitiesElAnthosClustersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployTargetAssociatedEntitiesElAnthosClustersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `membership` after provisioning.\nOptional. Membership of the GKE Hub-registered cluster to which to apply the Skaffold configuration. Format is `projects/{project}/locations/{location}/memberships/{membership_name}`."]
    pub fn membership(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployTargetAssociatedEntitiesElGkeClustersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_ip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_url: Option<PrimField<String>>,
}
impl ClouddeployTargetAssociatedEntitiesElGkeClustersEl {
    #[doc = "Set the field `cluster`.\nOptional. Information specifying a GKE Cluster. Format is `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}`."]
    pub fn set_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `internal_ip`.\nOptional. If true, `cluster` is accessed using the private IP address of the control plane endpoint. Otherwise, the default IP address of the control plane endpoint is used. The default IP address is the private IP address for clusters with private control-plane endpoints and the public IP address otherwise. Only specify this option when `cluster` is a [private GKE cluster](https://cloud.google.com/kubernetes-engine/docs/concepts/private-cluster-concept)."]
    pub fn set_internal_ip(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.internal_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `proxy_url`.\nOptional. If set, used to configure a [proxy](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#proxy) to the Kubernetes server."]
    pub fn set_proxy_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.proxy_url = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployTargetAssociatedEntitiesElGkeClustersEl {
    type O = BlockAssignable<ClouddeployTargetAssociatedEntitiesElGkeClustersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployTargetAssociatedEntitiesElGkeClustersEl {}
impl BuildClouddeployTargetAssociatedEntitiesElGkeClustersEl {
    pub fn build(self) -> ClouddeployTargetAssociatedEntitiesElGkeClustersEl {
        ClouddeployTargetAssociatedEntitiesElGkeClustersEl {
            cluster: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
            proxy_url: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployTargetAssociatedEntitiesElGkeClustersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployTargetAssociatedEntitiesElGkeClustersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployTargetAssociatedEntitiesElGkeClustersElRef {
        ClouddeployTargetAssociatedEntitiesElGkeClustersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployTargetAssociatedEntitiesElGkeClustersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nOptional. Information specifying a GKE Cluster. Format is `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}`."]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `internal_ip` after provisioning.\nOptional. If true, `cluster` is accessed using the private IP address of the control plane endpoint. Otherwise, the default IP address of the control plane endpoint is used. The default IP address is the private IP address for clusters with private control-plane endpoints and the public IP address otherwise. Only specify this option when `cluster` is a [private GKE cluster](https://cloud.google.com/kubernetes-engine/docs/concepts/private-cluster-concept)."]
    pub fn internal_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal_ip", self.base))
    }
    #[doc = "Get a reference to the value of field `proxy_url` after provisioning.\nOptional. If set, used to configure a [proxy](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#proxy) to the Kubernetes server."]
    pub fn proxy_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_url", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployTargetAssociatedEntitiesElDynamic {
    anthos_clusters: Option<DynamicBlock<ClouddeployTargetAssociatedEntitiesElAnthosClustersEl>>,
    gke_clusters: Option<DynamicBlock<ClouddeployTargetAssociatedEntitiesElGkeClustersEl>>,
}
#[derive(Serialize)]
pub struct ClouddeployTargetAssociatedEntitiesEl {
    entity_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anthos_clusters: Option<Vec<ClouddeployTargetAssociatedEntitiesElAnthosClustersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_clusters: Option<Vec<ClouddeployTargetAssociatedEntitiesElGkeClustersEl>>,
    dynamic: ClouddeployTargetAssociatedEntitiesElDynamic,
}
impl ClouddeployTargetAssociatedEntitiesEl {
    #[doc = "Set the field `anthos_clusters`.\n"]
    pub fn set_anthos_clusters(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployTargetAssociatedEntitiesElAnthosClustersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.anthos_clusters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.anthos_clusters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gke_clusters`.\n"]
    pub fn set_gke_clusters(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployTargetAssociatedEntitiesElGkeClustersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gke_clusters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gke_clusters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployTargetAssociatedEntitiesEl {
    type O = BlockAssignable<ClouddeployTargetAssociatedEntitiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployTargetAssociatedEntitiesEl {
    #[doc = "The name for the key in the map for which this object is mapped to in the API"]
    pub entity_id: PrimField<String>,
}
impl BuildClouddeployTargetAssociatedEntitiesEl {
    pub fn build(self) -> ClouddeployTargetAssociatedEntitiesEl {
        ClouddeployTargetAssociatedEntitiesEl {
            entity_id: self.entity_id,
            anthos_clusters: core::default::Default::default(),
            gke_clusters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployTargetAssociatedEntitiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployTargetAssociatedEntitiesElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployTargetAssociatedEntitiesElRef {
        ClouddeployTargetAssociatedEntitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployTargetAssociatedEntitiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `entity_id` after provisioning.\nThe name for the key in the map for which this object is mapped to in the API"]
    pub fn entity_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entity_id", self.base))
    }
    #[doc = "Get a reference to the value of field `anthos_clusters` after provisioning.\n"]
    pub fn anthos_clusters(
        &self,
    ) -> ListRef<ClouddeployTargetAssociatedEntitiesElAnthosClustersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.anthos_clusters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gke_clusters` after provisioning.\n"]
    pub fn gke_clusters(&self) -> ListRef<ClouddeployTargetAssociatedEntitiesElGkeClustersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gke_clusters", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployTargetCustomTargetEl {
    custom_target_type: PrimField<String>,
}
impl ClouddeployTargetCustomTargetEl {}
impl ToListMappable for ClouddeployTargetCustomTargetEl {
    type O = BlockAssignable<ClouddeployTargetCustomTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployTargetCustomTargetEl {
    #[doc = "Required. The name of the CustomTargetType. Format must be `projects/{project}/locations/{location}/customTargetTypes/{custom_target_type}`."]
    pub custom_target_type: PrimField<String>,
}
impl BuildClouddeployTargetCustomTargetEl {
    pub fn build(self) -> ClouddeployTargetCustomTargetEl {
        ClouddeployTargetCustomTargetEl {
            custom_target_type: self.custom_target_type,
        }
    }
}
pub struct ClouddeployTargetCustomTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployTargetCustomTargetElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployTargetCustomTargetElRef {
        ClouddeployTargetCustomTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployTargetCustomTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_target_type` after provisioning.\nRequired. The name of the CustomTargetType. Format must be `projects/{project}/locations/{location}/customTargetTypes/{custom_target_type}`."]
    pub fn custom_target_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_target_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddeployTargetExecutionConfigsElDefaultPoolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    artifact_storage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
}
impl ClouddeployTargetExecutionConfigsElDefaultPoolEl {
    #[doc = "Set the field `artifact_storage`.\nOptional. Cloud Storage location where execution outputs should be stored. This can either be a bucket (\"gs://my-bucket\") or a path within a bucket (\"gs://my-bucket/my-dir\"). If unspecified, a default bucket located in the same region will be used."]
    pub fn set_artifact_storage(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.artifact_storage = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nOptional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) will be used."]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployTargetExecutionConfigsElDefaultPoolEl {
    type O = BlockAssignable<ClouddeployTargetExecutionConfigsElDefaultPoolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployTargetExecutionConfigsElDefaultPoolEl {}
impl BuildClouddeployTargetExecutionConfigsElDefaultPoolEl {
    pub fn build(self) -> ClouddeployTargetExecutionConfigsElDefaultPoolEl {
        ClouddeployTargetExecutionConfigsElDefaultPoolEl {
            artifact_storage: core::default::Default::default(),
            service_account: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployTargetExecutionConfigsElDefaultPoolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployTargetExecutionConfigsElDefaultPoolElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployTargetExecutionConfigsElDefaultPoolElRef {
        ClouddeployTargetExecutionConfigsElDefaultPoolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployTargetExecutionConfigsElDefaultPoolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `artifact_storage` after provisioning.\nOptional. Cloud Storage location where execution outputs should be stored. This can either be a bucket (\"gs://my-bucket\") or a path within a bucket (\"gs://my-bucket/my-dir\"). If unspecified, a default bucket located in the same region will be used."]
    pub fn artifact_storage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.artifact_storage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nOptional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) will be used."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddeployTargetExecutionConfigsElPrivatePoolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    artifact_storage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    worker_pool: PrimField<String>,
}
impl ClouddeployTargetExecutionConfigsElPrivatePoolEl {
    #[doc = "Set the field `artifact_storage`.\nOptional. Cloud Storage location where execution outputs should be stored. This can either be a bucket (\"gs://my-bucket\") or a path within a bucket (\"gs://my-bucket/my-dir\"). If unspecified, a default bucket located in the same region will be used."]
    pub fn set_artifact_storage(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.artifact_storage = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nOptional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) will be used."]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployTargetExecutionConfigsElPrivatePoolEl {
    type O = BlockAssignable<ClouddeployTargetExecutionConfigsElPrivatePoolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployTargetExecutionConfigsElPrivatePoolEl {
    #[doc = "Required. Resource name of the Cloud Build worker pool to use. The format is `projects/{project}/locations/{location}/workerPools/{pool}`."]
    pub worker_pool: PrimField<String>,
}
impl BuildClouddeployTargetExecutionConfigsElPrivatePoolEl {
    pub fn build(self) -> ClouddeployTargetExecutionConfigsElPrivatePoolEl {
        ClouddeployTargetExecutionConfigsElPrivatePoolEl {
            artifact_storage: core::default::Default::default(),
            service_account: core::default::Default::default(),
            worker_pool: self.worker_pool,
        }
    }
}
pub struct ClouddeployTargetExecutionConfigsElPrivatePoolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployTargetExecutionConfigsElPrivatePoolElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddeployTargetExecutionConfigsElPrivatePoolElRef {
        ClouddeployTargetExecutionConfigsElPrivatePoolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployTargetExecutionConfigsElPrivatePoolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `artifact_storage` after provisioning.\nOptional. Cloud Storage location where execution outputs should be stored. This can either be a bucket (\"gs://my-bucket\") or a path within a bucket (\"gs://my-bucket/my-dir\"). If unspecified, a default bucket located in the same region will be used."]
    pub fn artifact_storage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.artifact_storage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nOptional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) will be used."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `worker_pool` after provisioning.\nRequired. Resource name of the Cloud Build worker pool to use. The format is `projects/{project}/locations/{location}/workerPools/{pool}`."]
    pub fn worker_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_pool", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddeployTargetExecutionConfigsElDynamic {
    default_pool: Option<DynamicBlock<ClouddeployTargetExecutionConfigsElDefaultPoolEl>>,
    private_pool: Option<DynamicBlock<ClouddeployTargetExecutionConfigsElPrivatePoolEl>>,
}
#[derive(Serialize)]
pub struct ClouddeployTargetExecutionConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    artifact_storage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    usages: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verbose: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_pool: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_pool: Option<Vec<ClouddeployTargetExecutionConfigsElDefaultPoolEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_pool: Option<Vec<ClouddeployTargetExecutionConfigsElPrivatePoolEl>>,
    dynamic: ClouddeployTargetExecutionConfigsElDynamic,
}
impl ClouddeployTargetExecutionConfigsEl {
    #[doc = "Set the field `artifact_storage`.\nOptional. Cloud Storage location in which to store execution outputs. This can either be a bucket (\"gs://my-bucket\") or a path within a bucket (\"gs://my-bucket/my-dir\"). If unspecified, a default bucket located in the same region will be used."]
    pub fn set_artifact_storage(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.artifact_storage = Some(v.into());
        self
    }
    #[doc = "Set the field `execution_timeout`.\nOptional. Execution timeout for a Cloud Build Execution. This must be between 10m and 24h in seconds format. If unspecified, a default timeout of 1h is used."]
    pub fn set_execution_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nOptional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) is used."]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `verbose`.\nOptional. If true, additional logging will be enabled when running builds in this execution environment."]
    pub fn set_verbose(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.verbose = Some(v.into());
        self
    }
    #[doc = "Set the field `worker_pool`.\nOptional. The resource name of the `WorkerPool`, with the format `projects/{project}/locations/{location}/workerPools/{worker_pool}`. If this optional field is unspecified, the default Cloud Build pool will be used."]
    pub fn set_worker_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.worker_pool = Some(v.into());
        self
    }
    #[doc = "Set the field `default_pool`.\n"]
    pub fn set_default_pool(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployTargetExecutionConfigsElDefaultPoolEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_pool = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_pool = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `private_pool`.\n"]
    pub fn set_private_pool(
        mut self,
        v: impl Into<BlockAssignable<ClouddeployTargetExecutionConfigsElPrivatePoolEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.private_pool = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.private_pool = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddeployTargetExecutionConfigsEl {
    type O = BlockAssignable<ClouddeployTargetExecutionConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployTargetExecutionConfigsEl {
    #[doc = "Required. Usages when this configuration should be applied."]
    pub usages: ListField<PrimField<String>>,
}
impl BuildClouddeployTargetExecutionConfigsEl {
    pub fn build(self) -> ClouddeployTargetExecutionConfigsEl {
        ClouddeployTargetExecutionConfigsEl {
            artifact_storage: core::default::Default::default(),
            execution_timeout: core::default::Default::default(),
            service_account: core::default::Default::default(),
            usages: self.usages,
            verbose: core::default::Default::default(),
            worker_pool: core::default::Default::default(),
            default_pool: core::default::Default::default(),
            private_pool: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddeployTargetExecutionConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployTargetExecutionConfigsElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployTargetExecutionConfigsElRef {
        ClouddeployTargetExecutionConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployTargetExecutionConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `artifact_storage` after provisioning.\nOptional. Cloud Storage location in which to store execution outputs. This can either be a bucket (\"gs://my-bucket\") or a path within a bucket (\"gs://my-bucket/my-dir\"). If unspecified, a default bucket located in the same region will be used."]
    pub fn artifact_storage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.artifact_storage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `execution_timeout` after provisioning.\nOptional. Execution timeout for a Cloud Build Execution. This must be between 10m and 24h in seconds format. If unspecified, a default timeout of 1h is used."]
    pub fn execution_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nOptional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) is used."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `usages` after provisioning.\nRequired. Usages when this configuration should be applied."]
    pub fn usages(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.usages", self.base))
    }
    #[doc = "Get a reference to the value of field `verbose` after provisioning.\nOptional. If true, additional logging will be enabled when running builds in this execution environment."]
    pub fn verbose(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.verbose", self.base))
    }
    #[doc = "Get a reference to the value of field `worker_pool` after provisioning.\nOptional. The resource name of the `WorkerPool`, with the format `projects/{project}/locations/{location}/workerPools/{worker_pool}`. If this optional field is unspecified, the default Cloud Build pool will be used."]
    pub fn worker_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_pool", self.base))
    }
    #[doc = "Get a reference to the value of field `default_pool` after provisioning.\n"]
    pub fn default_pool(&self) -> ListRef<ClouddeployTargetExecutionConfigsElDefaultPoolElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_pool", self.base))
    }
    #[doc = "Get a reference to the value of field `private_pool` after provisioning.\n"]
    pub fn private_pool(&self) -> ListRef<ClouddeployTargetExecutionConfigsElPrivatePoolElRef> {
        ListRef::new(self.shared().clone(), format!("{}.private_pool", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployTargetGkeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_endpoint: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_ip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_url: Option<PrimField<String>>,
}
impl ClouddeployTargetGkeEl {
    #[doc = "Set the field `cluster`.\nInformation specifying a GKE Cluster. Format is `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}."]
    pub fn set_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `dns_endpoint`.\nOptional. If set, the cluster will be accessed using the DNS endpoint. Note that both `dns_endpoint` and `internal_ip` cannot be set to true."]
    pub fn set_dns_endpoint(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.dns_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `internal_ip`.\nOptional. If true, `cluster` is accessed using the private IP address of the control plane endpoint. Otherwise, the default IP address of the control plane endpoint is used. The default IP address is the private IP address for clusters with private control-plane endpoints and the public IP address otherwise. Only specify this option when `cluster` is a [private GKE cluster](https://cloud.google.com/kubernetes-engine/docs/concepts/private-cluster-concept)."]
    pub fn set_internal_ip(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.internal_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `proxy_url`.\nOptional. If set, used to configure a [proxy](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#proxy) to the Kubernetes server."]
    pub fn set_proxy_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.proxy_url = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddeployTargetGkeEl {
    type O = BlockAssignable<ClouddeployTargetGkeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployTargetGkeEl {}
impl BuildClouddeployTargetGkeEl {
    pub fn build(self) -> ClouddeployTargetGkeEl {
        ClouddeployTargetGkeEl {
            cluster: core::default::Default::default(),
            dns_endpoint: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
            proxy_url: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployTargetGkeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployTargetGkeElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployTargetGkeElRef {
        ClouddeployTargetGkeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployTargetGkeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nInformation specifying a GKE Cluster. Format is `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}."]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `dns_endpoint` after provisioning.\nOptional. If set, the cluster will be accessed using the DNS endpoint. Note that both `dns_endpoint` and `internal_ip` cannot be set to true."]
    pub fn dns_endpoint(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_endpoint", self.base))
    }
    #[doc = "Get a reference to the value of field `internal_ip` after provisioning.\nOptional. If true, `cluster` is accessed using the private IP address of the control plane endpoint. Otherwise, the default IP address of the control plane endpoint is used. The default IP address is the private IP address for clusters with private control-plane endpoints and the public IP address otherwise. Only specify this option when `cluster` is a [private GKE cluster](https://cloud.google.com/kubernetes-engine/docs/concepts/private-cluster-concept)."]
    pub fn internal_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal_ip", self.base))
    }
    #[doc = "Get a reference to the value of field `proxy_url` after provisioning.\nOptional. If set, used to configure a [proxy](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#proxy) to the Kubernetes server."]
    pub fn proxy_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_url", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployTargetMultiTargetEl {
    target_ids: ListField<PrimField<String>>,
}
impl ClouddeployTargetMultiTargetEl {}
impl ToListMappable for ClouddeployTargetMultiTargetEl {
    type O = BlockAssignable<ClouddeployTargetMultiTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployTargetMultiTargetEl {
    #[doc = "Required. The target_ids of this multiTarget."]
    pub target_ids: ListField<PrimField<String>>,
}
impl BuildClouddeployTargetMultiTargetEl {
    pub fn build(self) -> ClouddeployTargetMultiTargetEl {
        ClouddeployTargetMultiTargetEl {
            target_ids: self.target_ids,
        }
    }
}
pub struct ClouddeployTargetMultiTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployTargetMultiTargetElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployTargetMultiTargetElRef {
        ClouddeployTargetMultiTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployTargetMultiTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_ids` after provisioning.\nRequired. The target_ids of this multiTarget."]
    pub fn target_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.target_ids", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployTargetRunEl {
    location: PrimField<String>,
}
impl ClouddeployTargetRunEl {}
impl ToListMappable for ClouddeployTargetRunEl {
    type O = BlockAssignable<ClouddeployTargetRunEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployTargetRunEl {
    #[doc = "Required. The location where the Cloud Run Service should be located. Format is `projects/{project}/locations/{location}`."]
    pub location: PrimField<String>,
}
impl BuildClouddeployTargetRunEl {
    pub fn build(self) -> ClouddeployTargetRunEl {
        ClouddeployTargetRunEl {
            location: self.location,
        }
    }
}
pub struct ClouddeployTargetRunElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployTargetRunElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployTargetRunElRef {
        ClouddeployTargetRunElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployTargetRunElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nRequired. The location where the Cloud Run Service should be located. Format is `projects/{project}/locations/{location}`."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddeployTargetTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ClouddeployTargetTimeoutsEl {
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
impl ToListMappable for ClouddeployTargetTimeoutsEl {
    type O = BlockAssignable<ClouddeployTargetTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddeployTargetTimeoutsEl {}
impl BuildClouddeployTargetTimeoutsEl {
    pub fn build(self) -> ClouddeployTargetTimeoutsEl {
        ClouddeployTargetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ClouddeployTargetTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddeployTargetTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ClouddeployTargetTimeoutsElRef {
        ClouddeployTargetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddeployTargetTimeoutsElRef {
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
struct ClouddeployTargetDynamic {
    anthos_cluster: Option<DynamicBlock<ClouddeployTargetAnthosClusterEl>>,
    associated_entities: Option<DynamicBlock<ClouddeployTargetAssociatedEntitiesEl>>,
    custom_target: Option<DynamicBlock<ClouddeployTargetCustomTargetEl>>,
    execution_configs: Option<DynamicBlock<ClouddeployTargetExecutionConfigsEl>>,
    gke: Option<DynamicBlock<ClouddeployTargetGkeEl>>,
    multi_target: Option<DynamicBlock<ClouddeployTargetMultiTargetEl>>,
    run: Option<DynamicBlock<ClouddeployTargetRunEl>>,
}
