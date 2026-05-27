use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct Cloudbuildv2ConnectionData {
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
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bitbucket_cloud_config: Option<Vec<Cloudbuildv2ConnectionBitbucketCloudConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bitbucket_data_center_config: Option<Vec<Cloudbuildv2ConnectionBitbucketDataCenterConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github_config: Option<Vec<Cloudbuildv2ConnectionGithubConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github_enterprise_config: Option<Vec<Cloudbuildv2ConnectionGithubEnterpriseConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab_config: Option<Vec<Cloudbuildv2ConnectionGitlabConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Cloudbuildv2ConnectionTimeoutsEl>,
    dynamic: Cloudbuildv2ConnectionDynamic,
}
struct Cloudbuildv2Connection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Cloudbuildv2ConnectionData>,
}
#[derive(Clone)]
pub struct Cloudbuildv2Connection(Rc<Cloudbuildv2Connection_>);
impl Cloudbuildv2Connection {
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
    #[doc = "Set the field `annotations`.\nAllows clients to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nIf disabled is set to true, functionality is disabled for this connection. Repository based API methods and webhooks processing for repositories in this connection will be disabled."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `bitbucket_cloud_config`.\n"]
    pub fn set_bitbucket_cloud_config(
        self,
        v: impl Into<BlockAssignable<Cloudbuildv2ConnectionBitbucketCloudConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bitbucket_cloud_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bitbucket_cloud_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `bitbucket_data_center_config`.\n"]
    pub fn set_bitbucket_data_center_config(
        self,
        v: impl Into<BlockAssignable<Cloudbuildv2ConnectionBitbucketDataCenterConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bitbucket_data_center_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .bitbucket_data_center_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `github_config`.\n"]
    pub fn set_github_config(
        self,
        v: impl Into<BlockAssignable<Cloudbuildv2ConnectionGithubConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().github_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.github_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `github_enterprise_config`.\n"]
    pub fn set_github_enterprise_config(
        self,
        v: impl Into<BlockAssignable<Cloudbuildv2ConnectionGithubEnterpriseConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().github_enterprise_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.github_enterprise_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gitlab_config`.\n"]
    pub fn set_gitlab_config(
        self,
        v: impl Into<BlockAssignable<Cloudbuildv2ConnectionGitlabConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gitlab_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gitlab_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Cloudbuildv2ConnectionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAllows clients to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Server assigned timestamp for when the connection was created."]
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
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nIf disabled is set to true, functionality is disabled for this connection. Repository based API methods and webhooks processing for repositories in this connection will be disabled."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `installation_state` after provisioning.\nOutput only. Installation state of the Connection."]
    pub fn installation_state(&self) -> ListRef<Cloudbuildv2ConnectionInstallationStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.installation_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nImmutable. The resource name of the connection."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Set to true when the connection is being set up or updated in the background."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Server assigned timestamp for when the connection was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bitbucket_cloud_config` after provisioning.\n"]
    pub fn bitbucket_cloud_config(
        &self,
    ) -> ListRef<Cloudbuildv2ConnectionBitbucketCloudConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bitbucket_cloud_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bitbucket_data_center_config` after provisioning.\n"]
    pub fn bitbucket_data_center_config(
        &self,
    ) -> ListRef<Cloudbuildv2ConnectionBitbucketDataCenterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bitbucket_data_center_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `github_config` after provisioning.\n"]
    pub fn github_config(&self) -> ListRef<Cloudbuildv2ConnectionGithubConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.github_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `github_enterprise_config` after provisioning.\n"]
    pub fn github_enterprise_config(
        &self,
    ) -> ListRef<Cloudbuildv2ConnectionGithubEnterpriseConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.github_enterprise_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gitlab_config` after provisioning.\n"]
    pub fn gitlab_config(&self) -> ListRef<Cloudbuildv2ConnectionGitlabConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gitlab_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Cloudbuildv2ConnectionTimeoutsElRef {
        Cloudbuildv2ConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for Cloudbuildv2Connection {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Cloudbuildv2Connection {}
impl ToListMappable for Cloudbuildv2Connection {
    type O = ListRef<Cloudbuildv2ConnectionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Cloudbuildv2Connection_ {
    fn extract_resource_type(&self) -> String {
        "google_cloudbuildv2_connection".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudbuildv2Connection {
    pub tf_id: String,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
    #[doc = "Immutable. The resource name of the connection."]
    pub name: PrimField<String>,
}
impl BuildCloudbuildv2Connection {
    pub fn build(self, stack: &mut Stack) -> Cloudbuildv2Connection {
        let out = Cloudbuildv2Connection(Rc::new(Cloudbuildv2Connection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Cloudbuildv2ConnectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                disabled: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                bitbucket_cloud_config: core::default::Default::default(),
                bitbucket_data_center_config: core::default::Default::default(),
                github_config: core::default::Default::default(),
                github_enterprise_config: core::default::Default::default(),
                gitlab_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct Cloudbuildv2ConnectionRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudbuildv2ConnectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl Cloudbuildv2ConnectionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAllows clients to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Server assigned timestamp for when the connection was created."]
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
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nIf disabled is set to true, functionality is disabled for this connection. Repository based API methods and webhooks processing for repositories in this connection will be disabled."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `installation_state` after provisioning.\nOutput only. Installation state of the Connection."]
    pub fn installation_state(&self) -> ListRef<Cloudbuildv2ConnectionInstallationStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.installation_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nImmutable. The resource name of the connection."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Set to true when the connection is being set up or updated in the background."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Server assigned timestamp for when the connection was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bitbucket_cloud_config` after provisioning.\n"]
    pub fn bitbucket_cloud_config(
        &self,
    ) -> ListRef<Cloudbuildv2ConnectionBitbucketCloudConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bitbucket_cloud_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bitbucket_data_center_config` after provisioning.\n"]
    pub fn bitbucket_data_center_config(
        &self,
    ) -> ListRef<Cloudbuildv2ConnectionBitbucketDataCenterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bitbucket_data_center_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `github_config` after provisioning.\n"]
    pub fn github_config(&self) -> ListRef<Cloudbuildv2ConnectionGithubConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.github_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `github_enterprise_config` after provisioning.\n"]
    pub fn github_enterprise_config(
        &self,
    ) -> ListRef<Cloudbuildv2ConnectionGithubEnterpriseConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.github_enterprise_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gitlab_config` after provisioning.\n"]
    pub fn gitlab_config(&self) -> ListRef<Cloudbuildv2ConnectionGitlabConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gitlab_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Cloudbuildv2ConnectionTimeoutsElRef {
        Cloudbuildv2ConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionInstallationStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage: Option<PrimField<String>>,
}
impl Cloudbuildv2ConnectionInstallationStateEl {
    #[doc = "Set the field `action_uri`.\n"]
    pub fn set_action_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `stage`.\n"]
    pub fn set_stage(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stage = Some(v.into());
        self
    }
}
impl ToListMappable for Cloudbuildv2ConnectionInstallationStateEl {
    type O = BlockAssignable<Cloudbuildv2ConnectionInstallationStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildv2ConnectionInstallationStateEl {}
impl BuildCloudbuildv2ConnectionInstallationStateEl {
    pub fn build(self) -> Cloudbuildv2ConnectionInstallationStateEl {
        Cloudbuildv2ConnectionInstallationStateEl {
            action_uri: core::default::Default::default(),
            message: core::default::Default::default(),
            stage: core::default::Default::default(),
        }
    }
}
pub struct Cloudbuildv2ConnectionInstallationStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudbuildv2ConnectionInstallationStateElRef {
    fn new(shared: StackShared, base: String) -> Cloudbuildv2ConnectionInstallationStateElRef {
        Cloudbuildv2ConnectionInstallationStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudbuildv2ConnectionInstallationStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action_uri` after provisioning.\n"]
    pub fn action_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stage", self.base))
    }
}
#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionBitbucketCloudConfigElAuthorizerCredentialEl {
    user_token_secret_version: PrimField<String>,
}
impl Cloudbuildv2ConnectionBitbucketCloudConfigElAuthorizerCredentialEl {}
impl ToListMappable for Cloudbuildv2ConnectionBitbucketCloudConfigElAuthorizerCredentialEl {
    type O = BlockAssignable<Cloudbuildv2ConnectionBitbucketCloudConfigElAuthorizerCredentialEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildv2ConnectionBitbucketCloudConfigElAuthorizerCredentialEl {
    #[doc = "Required. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub user_token_secret_version: PrimField<String>,
}
impl BuildCloudbuildv2ConnectionBitbucketCloudConfigElAuthorizerCredentialEl {
    pub fn build(self) -> Cloudbuildv2ConnectionBitbucketCloudConfigElAuthorizerCredentialEl {
        Cloudbuildv2ConnectionBitbucketCloudConfigElAuthorizerCredentialEl {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}
pub struct Cloudbuildv2ConnectionBitbucketCloudConfigElAuthorizerCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudbuildv2ConnectionBitbucketCloudConfigElAuthorizerCredentialElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudbuildv2ConnectionBitbucketCloudConfigElAuthorizerCredentialElRef {
        Cloudbuildv2ConnectionBitbucketCloudConfigElAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudbuildv2ConnectionBitbucketCloudConfigElAuthorizerCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub fn user_token_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_token_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated to this token."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl {
    user_token_secret_version: PrimField<String>,
}
impl Cloudbuildv2ConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl {}
impl ToListMappable for Cloudbuildv2ConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl {
    type O =
        BlockAssignable<Cloudbuildv2ConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildv2ConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl {
    #[doc = "Required. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub user_token_secret_version: PrimField<String>,
}
impl BuildCloudbuildv2ConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl {
    pub fn build(self) -> Cloudbuildv2ConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl {
        Cloudbuildv2ConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}
pub struct Cloudbuildv2ConnectionBitbucketCloudConfigElReadAuthorizerCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudbuildv2ConnectionBitbucketCloudConfigElReadAuthorizerCredentialElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudbuildv2ConnectionBitbucketCloudConfigElReadAuthorizerCredentialElRef {
        Cloudbuildv2ConnectionBitbucketCloudConfigElReadAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudbuildv2ConnectionBitbucketCloudConfigElReadAuthorizerCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub fn user_token_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_token_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated to this token."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize, Default)]
struct Cloudbuildv2ConnectionBitbucketCloudConfigElDynamic {
    authorizer_credential:
        Option<DynamicBlock<Cloudbuildv2ConnectionBitbucketCloudConfigElAuthorizerCredentialEl>>,
    read_authorizer_credential: Option<
        DynamicBlock<Cloudbuildv2ConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl>,
    >,
}
#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionBitbucketCloudConfigEl {
    webhook_secret_secret_version: PrimField<String>,
    workspace: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_credential:
        Option<Vec<Cloudbuildv2ConnectionBitbucketCloudConfigElAuthorizerCredentialEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_authorizer_credential:
        Option<Vec<Cloudbuildv2ConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl>>,
    dynamic: Cloudbuildv2ConnectionBitbucketCloudConfigElDynamic,
}
impl Cloudbuildv2ConnectionBitbucketCloudConfigEl {
    #[doc = "Set the field `authorizer_credential`.\n"]
    pub fn set_authorizer_credential(
        mut self,
        v: impl Into<
            BlockAssignable<Cloudbuildv2ConnectionBitbucketCloudConfigElAuthorizerCredentialEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authorizer_credential = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authorizer_credential = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `read_authorizer_credential`.\n"]
    pub fn set_read_authorizer_credential(
        mut self,
        v: impl Into<
            BlockAssignable<Cloudbuildv2ConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.read_authorizer_credential = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.read_authorizer_credential = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Cloudbuildv2ConnectionBitbucketCloudConfigEl {
    type O = BlockAssignable<Cloudbuildv2ConnectionBitbucketCloudConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildv2ConnectionBitbucketCloudConfigEl {
    #[doc = "Required. Immutable. SecretManager resource containing the webhook secret used to verify webhook events, formatted as 'projects/*/secrets/*/versions/*'."]
    pub webhook_secret_secret_version: PrimField<String>,
    #[doc = "The Bitbucket Cloud Workspace ID to be connected to Google Cloud Platform."]
    pub workspace: PrimField<String>,
}
impl BuildCloudbuildv2ConnectionBitbucketCloudConfigEl {
    pub fn build(self) -> Cloudbuildv2ConnectionBitbucketCloudConfigEl {
        Cloudbuildv2ConnectionBitbucketCloudConfigEl {
            webhook_secret_secret_version: self.webhook_secret_secret_version,
            workspace: self.workspace,
            authorizer_credential: core::default::Default::default(),
            read_authorizer_credential: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Cloudbuildv2ConnectionBitbucketCloudConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudbuildv2ConnectionBitbucketCloudConfigElRef {
    fn new(shared: StackShared, base: String) -> Cloudbuildv2ConnectionBitbucketCloudConfigElRef {
        Cloudbuildv2ConnectionBitbucketCloudConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudbuildv2ConnectionBitbucketCloudConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nRequired. Immutable. SecretManager resource containing the webhook secret used to verify webhook events, formatted as 'projects/*/secrets/*/versions/*'."]
    pub fn webhook_secret_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.webhook_secret_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `workspace` after provisioning.\nThe Bitbucket Cloud Workspace ID to be connected to Google Cloud Platform."]
    pub fn workspace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workspace", self.base))
    }
    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(
        &self,
    ) -> ListRef<Cloudbuildv2ConnectionBitbucketCloudConfigElAuthorizerCredentialElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorizer_credential", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `read_authorizer_credential` after provisioning.\n"]
    pub fn read_authorizer_credential(
        &self,
    ) -> ListRef<Cloudbuildv2ConnectionBitbucketCloudConfigElReadAuthorizerCredentialElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.read_authorizer_credential", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl {
    user_token_secret_version: PrimField<String>,
}
impl Cloudbuildv2ConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl {}
impl ToListMappable for Cloudbuildv2ConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl {
    type O =
        BlockAssignable<Cloudbuildv2ConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildv2ConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl {
    #[doc = "Required. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub user_token_secret_version: PrimField<String>,
}
impl BuildCloudbuildv2ConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl {
    pub fn build(self) -> Cloudbuildv2ConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl {
        Cloudbuildv2ConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}
pub struct Cloudbuildv2ConnectionBitbucketDataCenterConfigElAuthorizerCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudbuildv2ConnectionBitbucketDataCenterConfigElAuthorizerCredentialElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudbuildv2ConnectionBitbucketDataCenterConfigElAuthorizerCredentialElRef {
        Cloudbuildv2ConnectionBitbucketDataCenterConfigElAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudbuildv2ConnectionBitbucketDataCenterConfigElAuthorizerCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub fn user_token_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_token_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated to this token."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl {
    user_token_secret_version: PrimField<String>,
}
impl Cloudbuildv2ConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl {}
impl ToListMappable
    for Cloudbuildv2ConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl
{
    type O = BlockAssignable<
        Cloudbuildv2ConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildv2ConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl {
    #[doc = "Required. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub user_token_secret_version: PrimField<String>,
}
impl BuildCloudbuildv2ConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl {
    pub fn build(
        self,
    ) -> Cloudbuildv2ConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl {
        Cloudbuildv2ConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}
pub struct Cloudbuildv2ConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudbuildv2ConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudbuildv2ConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialElRef {
        Cloudbuildv2ConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudbuildv2ConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub fn user_token_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_token_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated to this token."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl {
    service: PrimField<String>,
}
impl Cloudbuildv2ConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl {}
impl ToListMappable for Cloudbuildv2ConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl {
    type O =
        BlockAssignable<Cloudbuildv2ConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildv2ConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl {
    #[doc = "Required. The Service Directory service name. Format: projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub service: PrimField<String>,
}
impl BuildCloudbuildv2ConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl {
    pub fn build(
        self,
    ) -> Cloudbuildv2ConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl {
        Cloudbuildv2ConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl {
            service: self.service,
        }
    }
}
pub struct Cloudbuildv2ConnectionBitbucketDataCenterConfigElServiceDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudbuildv2ConnectionBitbucketDataCenterConfigElServiceDirectoryConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudbuildv2ConnectionBitbucketDataCenterConfigElServiceDirectoryConfigElRef {
        Cloudbuildv2ConnectionBitbucketDataCenterConfigElServiceDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudbuildv2ConnectionBitbucketDataCenterConfigElServiceDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nRequired. The Service Directory service name. Format: projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize, Default)]
struct Cloudbuildv2ConnectionBitbucketDataCenterConfigElDynamic {
    authorizer_credential: Option<
        DynamicBlock<Cloudbuildv2ConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl>,
    >,
    read_authorizer_credential: Option<
        DynamicBlock<Cloudbuildv2ConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl>,
    >,
    service_directory_config: Option<
        DynamicBlock<Cloudbuildv2ConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionBitbucketDataCenterConfigEl {
    host_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_ca: Option<PrimField<String>>,
    webhook_secret_secret_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_credential:
        Option<Vec<Cloudbuildv2ConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_authorizer_credential:
        Option<Vec<Cloudbuildv2ConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory_config:
        Option<Vec<Cloudbuildv2ConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl>>,
    dynamic: Cloudbuildv2ConnectionBitbucketDataCenterConfigElDynamic,
}
impl Cloudbuildv2ConnectionBitbucketDataCenterConfigEl {
    #[doc = "Set the field `ssl_ca`.\nSSL certificate to use for requests to the Bitbucket Data Center."]
    pub fn set_ssl_ca(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_ca = Some(v.into());
        self
    }
    #[doc = "Set the field `authorizer_credential`.\n"]
    pub fn set_authorizer_credential(
        mut self,
        v: impl Into<
            BlockAssignable<
                Cloudbuildv2ConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authorizer_credential = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authorizer_credential = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `read_authorizer_credential`.\n"]
    pub fn set_read_authorizer_credential(
        mut self,
        v: impl Into<
            BlockAssignable<
                Cloudbuildv2ConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.read_authorizer_credential = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.read_authorizer_credential = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                Cloudbuildv2ConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_directory_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_directory_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Cloudbuildv2ConnectionBitbucketDataCenterConfigEl {
    type O = BlockAssignable<Cloudbuildv2ConnectionBitbucketDataCenterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildv2ConnectionBitbucketDataCenterConfigEl {
    #[doc = "The URI of the Bitbucket Data Center host this connection is for."]
    pub host_uri: PrimField<String>,
    #[doc = "Required. Immutable. SecretManager resource containing the webhook secret used to verify webhook events, formatted as 'projects/*/secrets/*/versions/*'."]
    pub webhook_secret_secret_version: PrimField<String>,
}
impl BuildCloudbuildv2ConnectionBitbucketDataCenterConfigEl {
    pub fn build(self) -> Cloudbuildv2ConnectionBitbucketDataCenterConfigEl {
        Cloudbuildv2ConnectionBitbucketDataCenterConfigEl {
            host_uri: self.host_uri,
            ssl_ca: core::default::Default::default(),
            webhook_secret_secret_version: self.webhook_secret_secret_version,
            authorizer_credential: core::default::Default::default(),
            read_authorizer_credential: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Cloudbuildv2ConnectionBitbucketDataCenterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudbuildv2ConnectionBitbucketDataCenterConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudbuildv2ConnectionBitbucketDataCenterConfigElRef {
        Cloudbuildv2ConnectionBitbucketDataCenterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudbuildv2ConnectionBitbucketDataCenterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_uri` after provisioning.\nThe URI of the Bitbucket Data Center host this connection is for."]
    pub fn host_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `server_version` after provisioning.\nOutput only. Version of the Bitbucket Data Center running on the 'host_uri'."]
    pub fn server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_ca` after provisioning.\nSSL certificate to use for requests to the Bitbucket Data Center."]
    pub fn ssl_ca(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca", self.base))
    }
    #[doc = "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nRequired. Immutable. SecretManager resource containing the webhook secret used to verify webhook events, formatted as 'projects/*/secrets/*/versions/*'."]
    pub fn webhook_secret_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.webhook_secret_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(
        &self,
    ) -> ListRef<Cloudbuildv2ConnectionBitbucketDataCenterConfigElAuthorizerCredentialElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorizer_credential", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `read_authorizer_credential` after provisioning.\n"]
    pub fn read_authorizer_credential(
        &self,
    ) -> ListRef<Cloudbuildv2ConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.read_authorizer_credential", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> ListRef<Cloudbuildv2ConnectionBitbucketDataCenterConfigElServiceDirectoryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_token_secret_version: Option<PrimField<String>>,
}
impl Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl {
    #[doc = "Set the field `oauth_token_secret_version`.\nA SecretManager resource containing the OAuth token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub fn set_oauth_token_secret_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oauth_token_secret_version = Some(v.into());
        self
    }
}
impl ToListMappable for Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl {
    type O = BlockAssignable<Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl {}
impl BuildCloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl {
    pub fn build(self) -> Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl {
        Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl {
            oauth_token_secret_version: core::default::Default::default(),
        }
    }
}
pub struct Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialElRef {
        Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `oauth_token_secret_version` after provisioning.\nA SecretManager resource containing the OAuth token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub fn oauth_token_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth_token_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated to this token."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize, Default)]
struct Cloudbuildv2ConnectionGithubConfigElDynamic {
    authorizer_credential:
        Option<DynamicBlock<Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl>>,
}
#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionGithubConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    app_installation_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_credential: Option<Vec<Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl>>,
    dynamic: Cloudbuildv2ConnectionGithubConfigElDynamic,
}
impl Cloudbuildv2ConnectionGithubConfigEl {
    #[doc = "Set the field `app_installation_id`.\nGitHub App installation id."]
    pub fn set_app_installation_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.app_installation_id = Some(v.into());
        self
    }
    #[doc = "Set the field `authorizer_credential`.\n"]
    pub fn set_authorizer_credential(
        mut self,
        v: impl Into<BlockAssignable<Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authorizer_credential = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authorizer_credential = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Cloudbuildv2ConnectionGithubConfigEl {
    type O = BlockAssignable<Cloudbuildv2ConnectionGithubConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildv2ConnectionGithubConfigEl {}
impl BuildCloudbuildv2ConnectionGithubConfigEl {
    pub fn build(self) -> Cloudbuildv2ConnectionGithubConfigEl {
        Cloudbuildv2ConnectionGithubConfigEl {
            app_installation_id: core::default::Default::default(),
            authorizer_credential: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Cloudbuildv2ConnectionGithubConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudbuildv2ConnectionGithubConfigElRef {
    fn new(shared: StackShared, base: String) -> Cloudbuildv2ConnectionGithubConfigElRef {
        Cloudbuildv2ConnectionGithubConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudbuildv2ConnectionGithubConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_installation_id` after provisioning.\nGitHub App installation id."]
    pub fn app_installation_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_installation_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(
        &self,
    ) -> ListRef<Cloudbuildv2ConnectionGithubConfigElAuthorizerCredentialElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorizer_credential", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {
    service: PrimField<String>,
}
impl Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {}
impl ToListMappable for Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {
    type O =
        BlockAssignable<Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {
    #[doc = "Required. The Service Directory service name. Format: projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub service: PrimField<String>,
}
impl BuildCloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {
    pub fn build(self) -> Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {
        Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {
            service: self.service,
        }
    }
}
pub struct Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigElRef {
        Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nRequired. The Service Directory service name. Format: projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize, Default)]
struct Cloudbuildv2ConnectionGithubEnterpriseConfigElDynamic {
    service_directory_config: Option<
        DynamicBlock<Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionGithubEnterpriseConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    app_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_installation_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_slug: Option<PrimField<String>>,
    host_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_key_secret_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_ca: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook_secret_secret_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory_config:
        Option<Vec<Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl>>,
    dynamic: Cloudbuildv2ConnectionGithubEnterpriseConfigElDynamic,
}
impl Cloudbuildv2ConnectionGithubEnterpriseConfigEl {
    #[doc = "Set the field `app_id`.\nId of the GitHub App created from the manifest."]
    pub fn set_app_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.app_id = Some(v.into());
        self
    }
    #[doc = "Set the field `app_installation_id`.\nID of the installation of the GitHub App."]
    pub fn set_app_installation_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.app_installation_id = Some(v.into());
        self
    }
    #[doc = "Set the field `app_slug`.\nThe URL-friendly name of the GitHub App."]
    pub fn set_app_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.app_slug = Some(v.into());
        self
    }
    #[doc = "Set the field `private_key_secret_version`.\nSecretManager resource containing the private key of the GitHub App, formatted as 'projects/*/secrets/*/versions/*'."]
    pub fn set_private_key_secret_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_key_secret_version = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_ca`.\nSSL certificate to use for requests to GitHub Enterprise."]
    pub fn set_ssl_ca(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_ca = Some(v.into());
        self
    }
    #[doc = "Set the field `webhook_secret_secret_version`.\nSecretManager resource containing the webhook secret of the GitHub App, formatted as 'projects/*/secrets/*/versions/*'."]
    pub fn set_webhook_secret_secret_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.webhook_secret_secret_version = Some(v.into());
        self
    }
    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v: impl Into<
            BlockAssignable<Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_directory_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_directory_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Cloudbuildv2ConnectionGithubEnterpriseConfigEl {
    type O = BlockAssignable<Cloudbuildv2ConnectionGithubEnterpriseConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildv2ConnectionGithubEnterpriseConfigEl {
    #[doc = "Required. The URI of the GitHub Enterprise host this connection is for."]
    pub host_uri: PrimField<String>,
}
impl BuildCloudbuildv2ConnectionGithubEnterpriseConfigEl {
    pub fn build(self) -> Cloudbuildv2ConnectionGithubEnterpriseConfigEl {
        Cloudbuildv2ConnectionGithubEnterpriseConfigEl {
            app_id: core::default::Default::default(),
            app_installation_id: core::default::Default::default(),
            app_slug: core::default::Default::default(),
            host_uri: self.host_uri,
            private_key_secret_version: core::default::Default::default(),
            ssl_ca: core::default::Default::default(),
            webhook_secret_secret_version: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Cloudbuildv2ConnectionGithubEnterpriseConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudbuildv2ConnectionGithubEnterpriseConfigElRef {
    fn new(shared: StackShared, base: String) -> Cloudbuildv2ConnectionGithubEnterpriseConfigElRef {
        Cloudbuildv2ConnectionGithubEnterpriseConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudbuildv2ConnectionGithubEnterpriseConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_id` after provisioning.\nId of the GitHub App created from the manifest."]
    pub fn app_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.base))
    }
    #[doc = "Get a reference to the value of field `app_installation_id` after provisioning.\nID of the installation of the GitHub App."]
    pub fn app_installation_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_installation_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `app_slug` after provisioning.\nThe URL-friendly name of the GitHub App."]
    pub fn app_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_slug", self.base))
    }
    #[doc = "Get a reference to the value of field `host_uri` after provisioning.\nRequired. The URI of the GitHub Enterprise host this connection is for."]
    pub fn host_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `private_key_secret_version` after provisioning.\nSecretManager resource containing the private key of the GitHub App, formatted as 'projects/*/secrets/*/versions/*'."]
    pub fn private_key_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_key_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_ca` after provisioning.\nSSL certificate to use for requests to GitHub Enterprise."]
    pub fn ssl_ca(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca", self.base))
    }
    #[doc = "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nSecretManager resource containing the webhook secret of the GitHub App, formatted as 'projects/*/secrets/*/versions/*'."]
    pub fn webhook_secret_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.webhook_secret_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> ListRef<Cloudbuildv2ConnectionGithubEnterpriseConfigElServiceDirectoryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl {
    user_token_secret_version: PrimField<String>,
}
impl Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl {}
impl ToListMappable for Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl {
    type O = BlockAssignable<Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl {
    #[doc = "Required. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub user_token_secret_version: PrimField<String>,
}
impl BuildCloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl {
    pub fn build(self) -> Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl {
        Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}
pub struct Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialElRef {
        Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub fn user_token_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_token_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated to this token."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl {
    user_token_secret_version: PrimField<String>,
}
impl Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl {}
impl ToListMappable for Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl {
    type O = BlockAssignable<Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl {
    #[doc = "Required. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub user_token_secret_version: PrimField<String>,
}
impl BuildCloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl {
    pub fn build(self) -> Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl {
        Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}
pub struct Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialElRef {
        Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes the Cloud Build connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub fn user_token_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_token_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated to this token."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl {
    service: PrimField<String>,
}
impl Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl {}
impl ToListMappable for Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl {
    type O = BlockAssignable<Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl {
    #[doc = "Required. The Service Directory service name. Format: projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub service: PrimField<String>,
}
impl BuildCloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl {
    pub fn build(self) -> Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl {
        Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl {
            service: self.service,
        }
    }
}
pub struct Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigElRef {
        Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nRequired. The Service Directory service name. Format: projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize, Default)]
struct Cloudbuildv2ConnectionGitlabConfigElDynamic {
    authorizer_credential:
        Option<DynamicBlock<Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl>>,
    read_authorizer_credential:
        Option<DynamicBlock<Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl>>,
    service_directory_config:
        Option<DynamicBlock<Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl>>,
}
#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionGitlabConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_ca: Option<PrimField<String>>,
    webhook_secret_secret_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_credential: Option<Vec<Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_authorizer_credential:
        Option<Vec<Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory_config:
        Option<Vec<Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl>>,
    dynamic: Cloudbuildv2ConnectionGitlabConfigElDynamic,
}
impl Cloudbuildv2ConnectionGitlabConfigEl {
    #[doc = "Set the field `host_uri`.\nThe URI of the GitLab Enterprise host this connection is for. If not specified, the default value is https://gitlab.com."]
    pub fn set_host_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_ca`.\nSSL certificate to use for requests to GitLab Enterprise."]
    pub fn set_ssl_ca(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_ca = Some(v.into());
        self
    }
    #[doc = "Set the field `authorizer_credential`.\n"]
    pub fn set_authorizer_credential(
        mut self,
        v: impl Into<BlockAssignable<Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authorizer_credential = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authorizer_credential = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `read_authorizer_credential`.\n"]
    pub fn set_read_authorizer_credential(
        mut self,
        v: impl Into<BlockAssignable<Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.read_authorizer_credential = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.read_authorizer_credential = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v: impl Into<BlockAssignable<Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_directory_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_directory_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Cloudbuildv2ConnectionGitlabConfigEl {
    type O = BlockAssignable<Cloudbuildv2ConnectionGitlabConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildv2ConnectionGitlabConfigEl {
    #[doc = "Required. Immutable. SecretManager resource containing the webhook secret of a GitLab Enterprise project, formatted as 'projects/*/secrets/*/versions/*'."]
    pub webhook_secret_secret_version: PrimField<String>,
}
impl BuildCloudbuildv2ConnectionGitlabConfigEl {
    pub fn build(self) -> Cloudbuildv2ConnectionGitlabConfigEl {
        Cloudbuildv2ConnectionGitlabConfigEl {
            host_uri: core::default::Default::default(),
            ssl_ca: core::default::Default::default(),
            webhook_secret_secret_version: self.webhook_secret_secret_version,
            authorizer_credential: core::default::Default::default(),
            read_authorizer_credential: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Cloudbuildv2ConnectionGitlabConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudbuildv2ConnectionGitlabConfigElRef {
    fn new(shared: StackShared, base: String) -> Cloudbuildv2ConnectionGitlabConfigElRef {
        Cloudbuildv2ConnectionGitlabConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudbuildv2ConnectionGitlabConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_uri` after provisioning.\nThe URI of the GitLab Enterprise host this connection is for. If not specified, the default value is https://gitlab.com."]
    pub fn host_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `server_version` after provisioning.\nOutput only. Version of the GitLab Enterprise server running on the 'host_uri'."]
    pub fn server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_ca` after provisioning.\nSSL certificate to use for requests to GitLab Enterprise."]
    pub fn ssl_ca(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca", self.base))
    }
    #[doc = "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nRequired. Immutable. SecretManager resource containing the webhook secret of a GitLab Enterprise project, formatted as 'projects/*/secrets/*/versions/*'."]
    pub fn webhook_secret_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.webhook_secret_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(
        &self,
    ) -> ListRef<Cloudbuildv2ConnectionGitlabConfigElAuthorizerCredentialElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorizer_credential", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `read_authorizer_credential` after provisioning.\n"]
    pub fn read_authorizer_credential(
        &self,
    ) -> ListRef<Cloudbuildv2ConnectionGitlabConfigElReadAuthorizerCredentialElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.read_authorizer_credential", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> ListRef<Cloudbuildv2ConnectionGitlabConfigElServiceDirectoryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Cloudbuildv2ConnectionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl Cloudbuildv2ConnectionTimeoutsEl {
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
impl ToListMappable for Cloudbuildv2ConnectionTimeoutsEl {
    type O = BlockAssignable<Cloudbuildv2ConnectionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildv2ConnectionTimeoutsEl {}
impl BuildCloudbuildv2ConnectionTimeoutsEl {
    pub fn build(self) -> Cloudbuildv2ConnectionTimeoutsEl {
        Cloudbuildv2ConnectionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct Cloudbuildv2ConnectionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Cloudbuildv2ConnectionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Cloudbuildv2ConnectionTimeoutsElRef {
        Cloudbuildv2ConnectionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Cloudbuildv2ConnectionTimeoutsElRef {
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
struct Cloudbuildv2ConnectionDynamic {
    bitbucket_cloud_config: Option<DynamicBlock<Cloudbuildv2ConnectionBitbucketCloudConfigEl>>,
    bitbucket_data_center_config:
        Option<DynamicBlock<Cloudbuildv2ConnectionBitbucketDataCenterConfigEl>>,
    github_config: Option<DynamicBlock<Cloudbuildv2ConnectionGithubConfigEl>>,
    github_enterprise_config: Option<DynamicBlock<Cloudbuildv2ConnectionGithubEnterpriseConfigEl>>,
    gitlab_config: Option<DynamicBlock<Cloudbuildv2ConnectionGitlabConfigEl>>,
}
